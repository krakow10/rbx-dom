use std::{borrow::Cow, collections::BTreeMap, io::Write};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use rbx_dom_weak::{
    types::{Ref, SharedString, SharedStringHash, Tags, Variant, VariantType},
    ustr, Ustr, WeakDom,
};
use rbx_reflection::{PropertyKind, PropertySerialization, ReflectionDatabase};

use crate::{
    conversion::ConvertVariant,
    core::find_serialized_property_descriptor,
    error::{EncodeError as NewEncodeError, EncodeErrorKind},
    types::write_value_xml,
};

use crate::serializer_core::{XmlEventWriter, XmlWriteEvent};

pub fn encode_internal<W: Write>(
    output: W,
    tree: &WeakDom,
    ids: &[Ref],
    options: EncodeOptions,
) -> Result<(), NewEncodeError> {
    let mut writer = XmlEventWriter::from_output(output);
    let mut state = EmitState::new(options);

    writer.write(XmlWriteEvent::start_element("roblox").attr("version", "4"))?;

    let mut property_buffer = Vec::new();
    for id in ids {
        serialize_instance(&mut writer, &mut state, tree, *id, &mut property_buffer)?;
    }

    serialize_shared_strings(&mut writer, &mut state)?;

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

/// Describes the strategy that rbx_xml should use when serializing properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EncodePropertyBehavior {
    /// Ignores properties that aren't known by rbx_xml.
    ///
    /// This is the default.
    IgnoreUnknown,

    /// Write unrecognized properties.
    ///
    /// With this option set, properties that are newer than rbx_xml's
    /// reflection database will show up. It may be problematic to depend on
    /// these properties, since rbx_xml may start supporting them with
    /// non-reflection specific names at a future date.
    WriteUnknown,

    /// Returns an error if any properties are found that aren't known by
    /// rbx_xml.
    ErrorOnUnknown,

    /// Completely turns off rbx_xml's reflection database. Property names and
    /// types will appear exactly as they are in the tree.
    ///
    /// This setting is useful for debugging the model format. It leaves the
    /// user to deal with oddities like how `Part.FormFactor` is actually
    /// serialized as `Part.formFactorRaw`.
    NoReflection,
}

/// Options available for serializing an XML-format model or place.
#[derive(Debug, Clone)]
pub struct EncodeOptions<'db> {
    property_behavior: EncodePropertyBehavior,
    database: &'db ReflectionDatabase<'db>,
}

impl<'db> EncodeOptions<'db> {
    /// Constructs a `EncodeOptions` with all values set to their defaults.
    #[inline]
    pub fn new() -> Self {
        EncodeOptions {
            property_behavior: EncodePropertyBehavior::IgnoreUnknown,
            database: rbx_reflection_database::get().unwrap(),
        }
    }

    /// Determines how rbx_xml will serialize properties, especially unknown
    /// ones.
    #[inline]
    pub fn property_behavior(self, property_behavior: EncodePropertyBehavior) -> Self {
        EncodeOptions {
            property_behavior,
            ..self
        }
    }

    /// Determines what reflection database rbx_xml will use to serialize
    /// properties.
    #[inline]
    pub fn reflection_database(self, database: &'db ReflectionDatabase<'db>) -> Self {
        EncodeOptions { database, ..self }
    }

    pub(crate) fn use_reflection(&self) -> bool {
        self.property_behavior != EncodePropertyBehavior::NoReflection
    }
}

impl<'db> Default for EncodeOptions<'db> {
    fn default() -> EncodeOptions<'db> {
        EncodeOptions::new()
    }
}

pub struct EmitState<'db> {
    options: EncodeOptions<'db>,

    /// A map of IDs written so far to the generated referent that they use.
    /// This map is used to correctly emit Ref properties.
    referent_map: HashMap<Ref, u32>,

    /// The referent value that will be used for emitting the next instance.
    next_referent: u32,

    /// A map of all shared strings referenced so far while generating XML. This
    /// map will be written as the file's SharedString dictionary.
    shared_strings_to_emit: BTreeMap<SharedStringHash, SharedString>,

    /// The always-written properties for each class, with their default values,
    /// keyed by class name and computed lazily on the first instance of each
    /// class.
    ///
    /// The inner `Option` distinguishes "not computed yet" (`None`) from
    /// "computed" (`Some`), since a computed list can be legitimately empty
    /// and `get_always_written_properties` allocates for every call.
    injected_properties_by_class: HashMap<Ustr, Option<Vec<(&'db str, &'db Variant)>>>,
}

impl<'db> EmitState<'db> {
    pub fn new(options: EncodeOptions<'db>) -> EmitState<'db> {
        EmitState {
            options,
            referent_map: HashMap::new(),
            next_referent: 0,
            shared_strings_to_emit: BTreeMap::new(),
            injected_properties_by_class: HashMap::new(),
        }
    }

    pub fn map_id(&mut self, id: Ref) -> u32 {
        match self.referent_map.get(&id) {
            Some(&value) => value,
            None => {
                let referent = self.next_referent;
                self.referent_map.insert(id, referent);
                self.next_referent += 1;
                referent
            }
        }
    }

    pub fn add_shared_string(&mut self, value: SharedString) {
        self.shared_strings_to_emit.insert(value.hash(), value);
    }

    /// Get this class's always-written properties with their default values,
    /// computing them once per class so that instances of the same class
    /// don't repeat the database lookups and default value clones.
    fn injected_properties(&mut self, class: Ustr) -> &[(&'db str, &'db Variant)] {
        let database = self.options.database;
        let entry = self
            .injected_properties_by_class
            .entry(class)
            .or_insert_with(|| None);
        if entry.is_none() {
            let mut properties = Vec::new();
            // We don't want to fail when we encounter an unknown class!
            let class_descriptor = database.classes.get(class.as_str());
            if let Some(class_descriptor) = class_descriptor {
                for prop_name in database.get_always_written_properties(class_descriptor) {
                    let Some(default) =
                        database.find_default_property(class_descriptor, prop_name)
                    else {
                        continue;
                    };
                    properties.push((prop_name, default));
                }
            }
            *entry = Some(properties);
        }
        entry.as_ref().unwrap()
    }
}

/// Serialize a single instance.
///
/// `property_buffer` is a Vec that can be reused between calls to
/// serialize_instance to make sorting properties more efficient.
fn serialize_instance<'db: 'dom, 'dom, W: Write>(
    writer: &mut XmlEventWriter<W>,
    state: &mut EmitState<'db>,
    tree: &'dom WeakDom,
    id: Ref,
    property_buffer: &mut Vec<(&'dom str, Cow<'dom, Variant>)>,
) -> Result<(), NewEncodeError> {
    let instance = tree.get_by_ref(id).unwrap();
    let mapped_id = state.map_id(id);

    writer.write(
        XmlWriteEvent::start_element("Item")
            .attr("class", &instance.class)
            .attr("referent", &mapped_id.to_string()),
    )?;

    writer.write(XmlWriteEvent::start_element("Properties"))?;

    write_value_xml(
        writer,
        state,
        "Name",
        &Variant::String(instance.name.clone()),
    )?;

    // Some classes have properties that must be written for Roblox to read
    // them correctly. Their default values are generated once per class and
    // merged with any instance values here.
    let injected: &[(&'db str, &'db Variant)] = if state.options.use_reflection() {
        state.injected_properties(instance.class)
    } else {
        &[]
    };

    // Merge each of the instance's property values with the injected default,
    // if any.
    for (name, value) in instance.properties.iter() {
        let default = injected
            .iter()
            .find(|(injected_name, _)| *injected_name == name.as_str())
            .map(|(_, default)| *default);

        let property_value: Cow<'dom, Variant> = match (default, value) {
            (Some(Variant::Attributes(default)), Variant::Attributes(existing)) => {
                let mut new = default.clone();
                // We want user-defined attributes to win, so we don't
                // override them here.
                for (name, value) in existing {
                    new.insert(name.clone(), value.clone());
                }
                Cow::Owned(new.into())
            }
            (Some(default), value) if value.ty() == VariantType::Attributes => {
                return Err(NewEncodeError::new(EncodeErrorKind::UnableToMergeProperties {
                    class_name: instance.class.to_string(),
                    property_name: name.as_str().to_string(),
                    actual_type: default.ty(),
                    expected_type: VariantType::Attributes,
                }));
            }
            (Some(Variant::Tags(default)), Variant::Tags(existing)) => {
                // This is technically inefficient, but that's fine
                // because Tags being merged should be extremely rare.
                let mut tag_map: HashSet<&str> = HashSet::new();
                tag_map.extend(existing.iter());
                tag_map.extend(default.iter());
                let mut new_tags = Tags::new();
                for tag in tag_map {
                    new_tags.push(tag)
                }
                Cow::Owned(new_tags.into())
            }
            (Some(default), value) if value.ty() == VariantType::Tags => {
                return Err(NewEncodeError::new(EncodeErrorKind::UnableToMergeProperties {
                    class_name: instance.class.to_string(),
                    property_name: name.as_str().to_string(),
                    actual_type: default.ty(),
                    expected_type: VariantType::Tags,
                }));
            }
            _ => Cow::Borrowed(value),
        };

        property_buffer.push((name.as_str(), property_value));
    }

    // Inject any always-written property the instance didn't define, using
    // the class's default value.
    for &(name, default) in injected {
        if !instance.properties.contains_key(&ustr(name)) {
            property_buffer.push((name, Cow::Borrowed(default)));
        }
    }

    property_buffer.sort_unstable_by_key(|(key, _)| *key);

    for (property_name, value) in property_buffer.drain(..) {
        let maybe_serialized_descriptor = if state.options.use_reflection() {
            find_serialized_property_descriptor(
                &instance.class,
                property_name,
                state.options.database,
            )
        } else {
            None
        };

        if let Some(serialized_descriptor) = maybe_serialized_descriptor {
            let data_type = serialized_descriptor.data_type.ty();

            let serialized_name = serialized_descriptor.name;

            let converted_value = match value.try_convert_ref(instance.class, data_type) {
                Ok(value) => value,
                Err(message) => {
                    return Err(
                        writer.error(EncodeErrorKind::UnsupportedPropertyConversion {
                            class_name: instance.class.to_string(),
                            property_name: property_name.to_string(),
                            expected_type: data_type,
                            actual_type: value.ty(),
                            message,
                        }),
                    )
                }
            };

            // Perform migrations during serialization
            if let PropertyKind::Canonical {
                serialization: PropertySerialization::Migrate(migration),
            } = &serialized_descriptor.kind
            {
                // If the migration fails, there's no harm in us doing nothing
                // since old values will still load in Studio.
                if let Ok(new_value) = migration.perform(&converted_value) {
                    for &new_property_name in migration.new_property_names() {
                        write_value_xml(writer, state, new_property_name, &new_value)?;
                    }

                    continue;
                }
            }

            write_value_xml(writer, state, serialized_name, &converted_value)?;
        } else {
            match state.options.property_behavior {
                EncodePropertyBehavior::IgnoreUnknown => {}
                EncodePropertyBehavior::WriteUnknown | EncodePropertyBehavior::NoReflection => {
                    // We'll take this value as-is with no conversions on
                    // either the name or value.

                    write_value_xml(writer, state, property_name, value.as_ref())?;
                }
                EncodePropertyBehavior::ErrorOnUnknown => {
                    return Err(writer.error(EncodeErrorKind::UnknownProperty {
                        class_name: instance.class.to_string(),
                        property_name: property_name.to_string(),
                    }));
                }
            }
        }
    }

    writer.write(XmlWriteEvent::end_element())?;

    for child_id in instance.children() {
        serialize_instance(writer, state, tree, *child_id, property_buffer)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

fn serialize_shared_strings<W: Write>(
    writer: &mut XmlEventWriter<W>,
    state: &mut EmitState,
) -> Result<(), NewEncodeError> {
    if state.shared_strings_to_emit.is_empty() {
        return Ok(());
    }

    writer.write(XmlWriteEvent::start_element("SharedStrings"))?;

    for value in state.shared_strings_to_emit.values() {
        // Roblox expects SharedString hashes to be the same length as an MD5
        // hash: 16 bytes, so we truncate our larger hashes to fit.
        let full_hash = value.hash();
        let truncated_hash = &full_hash.as_bytes()[..16];

        writer.write(
            XmlWriteEvent::start_element("SharedString")
                .attr("md5", &base64::encode(truncated_hash)),
        )?;

        writer.write_string(&base64::encode(value.data()))?;
        writer.end_element()?;
    }

    writer.end_element()?;
    Ok(())
}
