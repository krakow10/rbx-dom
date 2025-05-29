use std::{
    collections::{btree_map, BTreeMap},
    convert::TryInto,
    io::Write,
};

use ahash::{HashMap, HashMapExt, HashSetExt};
use rbx_dom_weak::{
    types::{
        Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
        ColorSequenceKeypoint, Content, ContentId, ContentType, Enum, EnumItem, Faces, Font,
        MaterialColors, Matrix3, NumberRange, NumberSequence, NumberSequenceKeypoint,
        PhysicalProperties, Ray, Rect, Ref, SecurityCapabilities, SharedString, Tags, UDim, UDim2,
        UniqueId, Variant, VariantType, Vector2, Vector3, Vector3int16,
    },
    Instance, Ustr, UstrSet, WeakDom,
};

use rbx_reflection::{
    ClassDescriptor, ClassTag, PropertyDescriptor, PropertyKind, PropertySerialization,
    ReflectionDatabase,
};

use crate::{
    chunk::ChunkBuilder,
    core::{
        find_property_descriptors, PropertyDescriptors, RbxWriteExt, FILE_MAGIC_HEADER,
        FILE_SIGNATURE, FILE_VERSION,
    },
    types::Type,
    Serializer,
};

use super::prop::BorrowedVariantVec;
use super::error::InnerError;
use super::CompressionType;

static FILE_FOOTER: &[u8] = b"</roblox>";

/// Represents all of the state during a single serialization session. A new
/// `BinarySerializer` object should be created every time we want to serialize
/// a binary model file.
pub(super) struct SerializerState<'dom, 'db, W> {
    serializer: &'db Serializer<'db>,

    /// The dom containing all of the instances that we're serializing.
    dom: &'dom WeakDom,

    /// Where the binary output should be written.
    output: W,

    /// All of the instances, in a deterministic order, that we're going to be
    /// serializing.
    relevant_instances: Vec<Ref>,

    /// A map from rbx-dom's unique instance ID (Ref) to the ID space used in
    /// the binary model format, signed integers.
    id_to_referent: HashMap<Ref, i32>,

    /// All of the types of instance discovered by our serializer that we'll be
    /// writing into the output.
    type_infos: TypeInfos<'dom, 'db>,

    /// All of the SharedStrings in the DOM, in the order they'll be written
    // in.
    shared_strings: Vec<SharedString>,

    /// A map of SharedStrings to where it is in the SSTR chunk. This is used
    /// for writing PROP chunks.
    shared_string_ids: HashMap<SharedString, u32>,
}

/// An instance class that our serializer knows about. We should have one struct
/// per unique ClassName.
#[derive(Debug)]
struct TypeInfo<'dom, 'db> {
    /// The ID that this serializer will use to refer to this type of instance.
    type_id: u32,

    /// Whether this type is considered a service. Only one copy of a given
    /// service can exist for a given ServiceProvider. DataModel is the only
    /// ServiceProvider in most projects.
    is_service: bool,

    /// All of the instance referents referenced by this type.
    referents: Vec<Ref>,

    /// All of the defined properties for this type found on any instance of
    /// this type. Properties are keyed by their serialized name, multiple
    /// entries may be present for each logical property.  This is to avoid
    /// multiple database lookups.
    ///
    /// Stored in a sorted map to try to ensure that we write out properties in
    /// a deterministic order.
    properties: BTreeMap<Ustr, PropInfo<'dom, 'db>>,

    /// A reference to the type's class descriptor from rbx_reflection, if this
    /// is a known class.
    class_descriptor: Option<&'db ClassDescriptor<'db>>,
}

impl<'dom, 'db> TypeInfo<'dom, 'db> {
    fn get_or_create_prop_info(
        &mut self,
        shared_strings: &mut Vec<SharedString>,
        shared_string_ids: &mut HashMap<SharedString, u32>,
        database: &'db ReflectionDatabase<'db>,
        type_name: &'dom str,
        prop_name: Ustr,
        sample_value: &'dom Variant,
    ) -> Result<&mut PropInfo<'dom, 'db>, InnerError> {
        // idea:
        // TypeInfo.properties is BTreeMap<Ustr, PropInfo>
        // PropInfo contains usize pointing to TypeInfo.values: Vec<BorrowedVariantVec<'dom>>
        // so they may be shared among multiple properties and use the same prop chunk.
        // indexmap?
        Ok(match self.properties.entry(prop_name) {
            btree_map::Entry::Occupied(entry) => entry.into_mut(),
            btree_map::Entry::Vacant(entry) => {
                let property_descriptor;
                let default_value;
                let canonical_name;
                let serialized_name;
                let serialized_ty;
                if let Some(class) = self.class_descriptor {
                    let class_prop = database.superclasses_iter(class).find_map(|class| {
                        class
                            .properties
                            .get(prop_name.as_str())
                            .map(|prop| (class, prop))
                    });

                    if let Some(descriptors) =
                        class_prop.and_then(|(class, prop)| PropertyDescriptors::new(class, prop))
                    {
                        canonical_name = descriptors.canonical.name.as_ref().into();
                        if let Some(serialized) = descriptors.serialized {
                            serialized_name = serialized.name.as_ref().into();
                            serialized_ty = serialized.data_type.ty();
                        } else {
                            serialized_name = prop_name;
                            serialized_ty = sample_value.ty();
                        }
                    } else {
                        canonical_name = prop_name;
                        serialized_name = prop_name;
                        serialized_ty = sample_value.ty();
                    };

                    property_descriptor = class_prop.map(|(_, prop)| prop);
                    default_value = database.find_default_property(class, &canonical_name);
                } else {
                    property_descriptor = None;
                    default_value = None;
                    canonical_name = prop_name;
                    serialized_name = prop_name;
                    serialized_ty = sample_value.ty();
                };

                let default_value = default_value
                    .or_else(|| fallback_default_value(serialized_ty))
                    .ok_or_else(|| {
                        // Since we don't know how to generate the default value
                        // for this property, we consider it unsupported.
                        InnerError::UnsupportedPropType {
                            type_name: type_name.to_string(),
                            prop_name: canonical_name.to_string(),
                            prop_type: format!("{:?}", serialized_ty),
                        }
                    })?;

                // There's no assurance that the default SharedString value
                // will actually get serialized inside of the SSTR chunk, so we
                // check here just to make sure.
                if let Variant::SharedString(sstr) = default_value {
                    if !shared_string_ids.contains_key(sstr) {
                        shared_string_ids.insert(sstr.clone(), 0);
                        shared_strings.push(sstr.clone());
                    }
                }

                let ser_type = Type::from_rbx_type(serialized_ty).ok_or_else(|| {
                    // This is a known value type, but rbx_binary doesn't have a
                    // binary type value for it. rbx_binary might be out of
                    // date?
                    InnerError::UnsupportedPropType {
                        type_name: type_name.to_string(),
                        prop_name: serialized_name.to_string(),
                        prop_type: format!("{:?}", serialized_ty),
                    }
                })?;

                // TODO: insert type_info.instances.len() references to the default value into values

                entry.insert(PropInfo {
                    prop_type: ser_type,
                    serialized_name,
                    values: BorrowedVariantVec::new(serialized_ty),
                    default_value,
                    property_descriptor,
                })
            }
        })
    }
}

/// A property on a specific class that our serializer knows about.
///
/// We should have one `PropInfo` per logical property per class that is used in
/// the document we are serializing. This means that even if `BasePart.Size` and
/// `BasePart.size` are present in the same document, they should share a
/// `PropInfo` as they are the same logical property.
#[derive(Debug)]
struct PropInfo<'dom, 'db> {
    /// The binary format type ID that will be use to serialize this property.
    /// This type is related to the type of the serialized form of the logical
    /// property, but is not 1:1.
    ///
    /// For example, a property marked to serialize as a
    /// `VariantType::BinaryString` will serialize as `Type::String`, the same
    /// as the `Content` and `String` variants do.
    prop_type: Type,

    /// An array of references to the values of this type.
    /// BorrowedVariantVec is a serialization gadget that also contains the Variant type.
    ///
    /// Contains the same type information as prop_type, duplicating its purpose.
    // TODO: deduplicate the purpose
    values: BorrowedVariantVec<'dom>,

    /// The serialized name for this property. This is the name that is actually
    /// written as part of the PROP chunk and may not line up with the canonical
    /// name for the property.
    serialized_name: Ustr,

    /// The default value for this property that should be used if any instances
    /// are missing this property.
    ///
    /// With the exception of newly-added properties, Roblox Studio will create
    /// files with instances that contain every property. When mixing old and
    /// newly-saved instances, or mixing instances generated from other tools,
    /// some properties may be missing. They will be populated from this value.
    ///
    /// Default values are first populated from the reflection database, if
    /// present, followed by an educated guess based on the type of the value.
    default_value: &'db Variant,

    // TODO: doc
    property_descriptor: Option<&'db PropertyDescriptor<'db>>,
}

/// Contains all of the `TypeInfo` objects known to the serializer so far. This
/// struct was broken out to help encapsulate the behavior here and to ease
/// self-borrowing issues from BinarySerializer getting too large.
#[derive(Debug)]
struct TypeInfos<'dom, 'db> {
    /// A map containing one entry for each unique ClassName discovered in the
    /// DOM.
    ///
    /// These are stored sorted so that we naturally iterate over them in order
    /// and improve our chances of being deterministic.
    values: BTreeMap<Ustr, TypeInfo<'dom, 'db>>,

    /// The next type ID that should be assigned if a type is discovered and
    /// added to the serializer.
    next_type_id: u32,
}

impl<'dom, 'db> TypeInfos<'dom, 'db> {
    fn new() -> Self {
        Self {
            values: BTreeMap::new(),
            next_type_id: 0,
        }
    }

    /// Finds the type info from the given ClassName if it exists, or creates
    /// one and returns a reference to it if not.
    fn get_or_create(
        &mut self,
        database: &'db ReflectionDatabase<'db>,
        class: Ustr,
    ) -> &mut TypeInfo<'dom, 'db> {
        // Split self into independent mutable references.
        let TypeInfos {
            values,
            next_type_id,
        } = self;
        values.entry(class).or_insert_with(|| {
            let type_id = *next_type_id;
            *next_type_id += 1;

            let class_descriptor = database.classes.get(class.as_str());

            let is_service = if let Some(descriptor) = &class_descriptor {
                descriptor.tags.contains(&ClassTag::Service)
            } else {
                log::info!("The class {} is not known to rbx_binary", class);
                false
            };

            let mut properties = BTreeMap::new();

            // Every instance has a property named Name. Even though
            // rbx_dom_weak encodes the name property specially, we still insert
            // this property into the type info and handle it like a regular
            // property during encoding.
            //
            // We can use a dummy default_value here because instances from
            // rbx_dom_weak always have a name set.
            properties.insert(
                "Name".into(),
                PropInfo {
                    prop_type: Type::String,
                    serialized_name: "Name".into(),
                    values: BorrowedVariantVec::String(Vec::new()),
                    default_value: &DEFAULT_STRING,
                    property_descriptor: class_descriptor
                        .and_then(|descriptor| descriptor.properties.get("Name")),
                },
            );

            TypeInfo {
                type_id,
                is_service,
                referents: Vec::new(),
                properties,
                class_descriptor,
            }
        })
    }
}

impl<'dom, 'db, W: Write> SerializerState<'dom, 'db, W> {
    pub fn new(serializer: &'db Serializer<'db>, dom: &'dom WeakDom, output: W) -> Self {
        SerializerState {
            serializer,
            dom,
            output,
            relevant_instances: Vec::new(),
            id_to_referent: HashMap::new(),
            type_infos: TypeInfos::new(),
            shared_strings: Vec::new(),
            shared_string_ids: HashMap::new(),
        }
    }

    /// Mark the given instance IDs and all of their descendants as intended for
    /// serialization with this serializer.
    #[profiling::function]
    pub fn add_instances(&mut self, referents: &[Ref]) -> Result<(), InnerError> {
        // Populate relevant_instances with a depth-first post-order traversal over the
        // tree(s). This is important to ensure that the order of the PRNT chunk (later
        // written by SerializerState::serialize_parents) is correct.

        // The implementation here slightly deviates from Roblox. Roblox writes the PRNT
        // in depth-first post-order, numbers referents in depth-first pre-order, and
        // generates type infos in lexical order by class name. See
        // https://github.com/rojo-rbx/rbx-dom/pull/411#issuecomment-2103713517

        // Since it seems only the PRNT chunk has important semantics related to its
        // ordering, we do one tree traversal in this function, thereby numbering
        // referents, generating type infos, and writing the PRNT chunk all in depth-first
        // post-order.
        let mut to_visit = Vec::new();
        let mut last_visited_child = None;

        to_visit.extend(referents.iter().rev());

        while let Some(referent) = to_visit.last() {
            let instance = self
                .dom
                .get_by_ref(*referent)
                .ok_or(InnerError::InvalidInstanceId {
                    referent: *referent,
                })?;

            to_visit.extend(instance.children().iter().rev());

            while let Some(referent) = to_visit.last() {
                let instance =
                    self.dom
                        .get_by_ref(*referent)
                        .ok_or(InnerError::InvalidInstanceId {
                            referent: *referent,
                        })?;

                if !instance.children().is_empty()
                    && instance.children().last() != last_visited_child.as_ref()
                {
                    break;
                }

                self.relevant_instances.push(*referent);
                self.collect_type_info(instance)?;
                last_visited_child = to_visit.pop();
            }
        }

        // Sort shared_strings by their hash, to ensure they are deterministically added
        // into the SSTR chunk, then assign them corresponding ids
        self.shared_strings.sort_by_key(SharedString::hash);
        for (id, shared_string) in self.shared_strings.iter().cloned().enumerate() {
            self.shared_string_ids.insert(shared_string, id as u32);
        }

        log::debug!(
            "Discovered {} unique TypeInfos",
            self.type_infos.values.len()
        );

        Ok(())
    }

    /// Collect information about all the different types of instance and their
    /// properties.
    // Using the entry API here, as Clippy suggests, would require us to
    // clone canonical_name in a cold branch. We don't want to do that.
    #[allow(clippy::map_entry)]
    #[profiling::function]
    pub fn collect_type_info(&mut self, instance: &'dom Instance) -> Result<(), InnerError> {
        let type_info = self
            .type_infos
            .get_or_create(self.serializer.database, instance.class);
        type_info.referents.push(instance.referent());

        for (prop_name, prop_value) in &instance.properties {
            // Discover and track any shared strings we come across.
            if let Variant::SharedString(shared_string) = prop_value {
                if !self.shared_string_ids.contains_key(shared_string) {
                    // We insert it with a dummy id of 0 so that we can check for contains_key.
                    // The actual id is set in `add_instances`
                    self.shared_string_ids.insert(shared_string.clone(), 0);
                    self.shared_strings.push(shared_string.clone())
                }
            }

            let prop_info = type_info.get_or_create_prop_info(
                &mut self.shared_strings,
                &mut self.shared_string_ids,
                self.serializer.database,
                instance.class.as_str(),
                *prop_name,
                prop_value,
            )?;

            // Append value to prop_info values.  This avoids
            // iterating over the instances and properties twice.
            if let Err(e) = prop_info.values.push(prop_value) {
                panic!("Failed to add property: class={} prop={prop_name}\n{e:?}",instance.class);
            }
        }

        // Note that default values must be filled for properties that were not visited.

        Ok(())
    }

    /// Populate the map from rbx-dom's instance ID space to the IDs that we'll
    /// be serializing to the model.
    #[profiling::function]
    pub fn generate_referents(&mut self) {
        self.id_to_referent.reserve(self.relevant_instances.len());

        for (next_referent, id) in self.relevant_instances.iter().enumerate() {
            self.id_to_referent
                .insert(*id, next_referent.try_into().unwrap());
        }

        log::debug!("Collected {} referents", self.id_to_referent.len());
    }

    pub fn write_header(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing header");

        self.output.write_all(FILE_MAGIC_HEADER)?;
        self.output.write_all(FILE_SIGNATURE)?;
        self.output.write_le_u16(FILE_VERSION)?;

        self.output
            .write_le_u32(self.type_infos.values.len() as u32)?;
        self.output
            .write_le_u32(self.relevant_instances.len() as u32)?;
        self.output.write_all(&[0; 8])?;

        Ok(())
    }

    /// Write out any metadata about this file, stored in a chunk named META.
    pub fn serialize_metadata(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing metadata (currently no-op)");
        // TODO: There is no concept of metadata in a dom yet.
        Ok(())
    }

    /// Write out all of the SharedStrings in this file, if any exist,
    /// stored in a chunk named SSTR.
    #[profiling::function]
    pub fn serialize_shared_strings(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing shared string chunk");

        if self.shared_strings.is_empty() {
            return Ok(());
        }

        let mut chunk = ChunkBuilder::new(b"SSTR", self.serializer.compression);

        chunk.write_le_u32(0)?; // SSTR version number
        chunk.write_le_u32(self.shared_strings.len() as u32)?;

        for shared_string in &self.shared_strings {
            // Better to write nothing than write half a hash
            chunk.write_all(&[0; 16])?;
            chunk.write_binary_string(shared_string.data())?;
        }

        chunk.dump(&mut self.output)?;

        Ok(())
    }

    /// Write out the declarations of all instances, stored in a series of
    /// chunks named INST.
    #[profiling::function]
    pub fn serialize_instances(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing instance chunks");

        for (type_name, type_info) in &self.type_infos.values {
            log::trace!(
                "Writing chunk for {} ({} instances)",
                type_name,
                type_info.referents.len()
            );

            let mut chunk = ChunkBuilder::new(b"INST", self.serializer.compression);

            chunk.write_le_u32(type_info.type_id)?;
            chunk.write_string(type_name)?;

            // It's possible that this integer will be expanded in the future to
            // be a general version/format field instead of just service vs
            // non-service.
            //
            // At that point, we'll start thinking about it like it's a u8
            // instead of a bool.
            chunk.write_bool(type_info.is_service)?;

            chunk.write_le_u32(type_info.referents.len() as u32)?;

            chunk.write_referent_array(
                type_info
                    .referents
                    .iter()
                    .map(|referent| self.id_to_referent[referent]),
            )?;

            if type_info.is_service {
                // It's unclear what this byte is used for, but when the type is
                // a service (like Workspace, Lighting, etc), we need to write
                // the value `1` for every instance in our file of that type.
                //
                // In 99.9% of cases, there's only going to be one copy of a
                // given service, so we're not worried about doing this super
                // efficiently.
                for _ in 0..type_info.referents.len() {
                    chunk.write_u8(1)?;
                }
            }

            chunk.dump(&mut self.output)?;
        }

        Ok(())
    }

    /// Write out batch declarations of property values for the instances
    /// previously defined in the INST chunks. Property data is contained in
    /// chunks named PROP.
    #[profiling::function]
    pub fn serialize_properties(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing properties");

        for (type_name, type_info) in &self.type_infos.values {
            for (prop_name, prop_info) in &type_info.properties {
                profiling::scope!("serialize property", prop_name.borrow());
                log::trace!(
                    "Writing property {}.{} (type {:?})",
                    type_name,
                    prop_name,
                    prop_info.prop_type
                );

                // Do migration
                let mut migrated_properties = Vec::new();
                let migrated_values;
                let (serialized_name, property_values) =
                    if let Some(prop_descriptor) = prop_info.property_descriptor {
                        match &prop_descriptor.kind {
                            PropertyKind::Canonical { serialization } => match serialization {
                                PropertySerialization::Serializes => {
                                    (prop_name.as_str(), &prop_info.values)
                                }
                                // Skip serializing this property chunk entirely
                                PropertySerialization::DoesNotSerialize => continue,
                                PropertySerialization::SerializesAs(serialized_name) => {
                                    (serialized_name.as_ref(), &prop_info.values)
                                }
                                PropertySerialization::Migrate(property_migration) => {
                                    // This is hard to fit into the migration api and ends up being slow and wasteful
                                    let cloned_variants = prop_info.values.cloned_variant_vec();
                                    migrated_properties = cloned_variants
                                        .iter()
                                        .map(|value| property_migration.perform(value).unwrap())
                                        .collect();
                                    // Assume there is at least one value.
                                    // The prop_info would not have been created otherwise.
                                    let mut values = BorrowedVariantVec::new(
                                        migrated_properties.first().unwrap().ty(),
                                    );
                                    // Assume all migrations end up with the same type.
                                    for value in &migrated_properties {
                                        values.push(value).unwrap();
                                    }
                                    migrated_values = values;
                                    (
                                        property_migration.new_property_name.as_str(),
                                        &migrated_values,
                                    )
                                }
                                _ => panic!("Unknown PropertySerialization"),
                            },
                            PropertyKind::Alias { alias_for } => {
                                (alias_for.as_ref(), &prop_info.values)
                            }
                            _ => panic!("Unknown PropertyKind"),
                        }
                    } else {
                        (prop_name.as_str(), &prop_info.values)
                    };

                let mut chunk = ChunkBuilder::new(b"PROP", self.serializer.compression);

                chunk.write_le_u32(type_info.type_id)?;
                chunk.write_string(serialized_name)?;
                chunk.write_u8(prop_info.prop_type as u8)?;

                let invalid_value = |i: usize, bad_value: &Variant| InnerError::InvalidPropValue {
                    instance_full_name: self.full_name_for(type_info.referents[i]),
                    type_name: type_name.to_string(),
                    prop_name: prop_name.to_string(),
                    prop_type: format!("{:?}", bad_value.ty()),
                };

                match property_values {
                    BorrowedVariantVec::Tags(values) => {
                        for &value in values {
                            let buf = value.encode();
                            chunk.write_binary_string(&buf)?;
                        }
                    }
                    BorrowedVariantVec::Attributes(values) => {
                        for (i, &value) in values.into_iter().enumerate() {
                            let mut buf = Vec::new();

                            value.to_writer(&mut buf).map_err(|_| {
                                invalid_value(i, &Variant::Attributes(value.clone()))
                            })?;

                            chunk.write_binary_string(&buf)?;
                        }
                    }
                    BorrowedVariantVec::MaterialColors(values) => {
                        for &value in values {
                            chunk.write_binary_string(&value.encode())?;
                        }
                    }
                    BorrowedVariantVec::UDim(values) => {
                        let mut scale = Vec::with_capacity(values.len());
                        let mut offset = Vec::with_capacity(values.len());
                        for &value in values {
                            scale.push(value.scale);
                            offset.push(value.offset);
                        }
                        chunk.write_interleaved_f32_array(scale)?;
                        chunk.write_interleaved_i32_array(offset)?;
                    }
                    BorrowedVariantVec::UDim2(values) => {
                        let mut scale_x = Vec::with_capacity(values.len());
                        let mut scale_y = Vec::with_capacity(values.len());
                        let mut offset_x = Vec::with_capacity(values.len());
                        let mut offset_y = Vec::with_capacity(values.len());

                        for &value in values {
                            scale_x.push(value.x.scale);
                            scale_y.push(value.y.scale);
                            offset_x.push(value.x.offset);
                            offset_y.push(value.y.offset);
                        }

                        chunk.write_interleaved_f32_array(scale_x)?;
                        chunk.write_interleaved_f32_array(scale_y)?;
                        chunk.write_interleaved_i32_array(offset_x)?;
                        chunk.write_interleaved_i32_array(offset_y)?;
                    }
                    BorrowedVariantVec::Font(values) => {
                        for &value in values {
                            chunk.write_string(&value.family)?;
                            chunk.write_le_u16(value.weight.as_u16())?;
                            chunk.write_u8(value.style.as_u8())?;
                            chunk.write_string(
                                value.cached_face_id.as_deref().unwrap_or_default(),
                            )?;
                        }
                    }
                    BorrowedVariantVec::Vector2(values) => {
                        let mut x = Vec::with_capacity(values.len());
                        let mut y = Vec::with_capacity(values.len());
                        for &value in values {
                            x.push(value.x);
                            y.push(value.y);
                        }
                        chunk.write_interleaved_f32_array(x)?;
                        chunk.write_interleaved_f32_array(y)?;
                    }
                    BorrowedVariantVec::Vector3(values) => {
                        let mut x = Vec::with_capacity(values.len());
                        let mut y = Vec::with_capacity(values.len());
                        let mut z = Vec::with_capacity(values.len());
                        for &value in values {
                            x.push(value.x);
                            y.push(value.y);
                            z.push(value.z);
                        }
                        chunk.write_interleaved_f32_array(x)?;
                        chunk.write_interleaved_f32_array(y)?;
                        chunk.write_interleaved_f32_array(z)?;
                    }
                    BorrowedVariantVec::Vector3int16(values) => {
                        for &value in values {
                            chunk.write_le_i16(value.x)?;
                            chunk.write_le_i16(value.y)?;
                            chunk.write_le_i16(value.z)?;
                        }
                    }
                    BorrowedVariantVec::SharedString(values) => {
                        let it = values.into_iter().map(|&value| {
                            if let Some(id) = self.shared_string_ids.get(value) {
                                *id
                            } else {
                                panic!(
                                    "SharedString {} was not found during type collection",
                                    value.hash()
                                )
                            }
                        });
                        chunk.write_interleaved_u32_array(it)?;
                    }
                    BorrowedVariantVec::OptionalCFrame(values) => {
                        let mut bools = Vec::with_capacity(values.len());
                        let mut x = Vec::with_capacity(values.len());
                        let mut y = Vec::with_capacity(values.len());
                        let mut z = Vec::with_capacity(values.len());

                        chunk.write_u8(Type::CFrame as u8)?;
                        for &value in values {
                            let matrix;
                            if let Some(value) = value {
                                matrix = value.orientation;
                                x.push(value.position.x);
                                y.push(value.position.y);
                                z.push(value.position.z);
                                bools.push(0x01);
                            } else {
                                matrix = Matrix3::identity();
                                x.push(0.0);
                                y.push(0.0);
                                z.push(0.0);
                                bools.push(0x00);
                            };

                            // write matrix
                            if let Some(id) = matrix.to_basic_rotation_id() {
                                chunk.write_u8(id)?;
                            } else {
                                chunk.write_u8(0x00)?;

                                chunk.write_le_f32(matrix.x.x)?;
                                chunk.write_le_f32(matrix.x.y)?;
                                chunk.write_le_f32(matrix.x.z)?;

                                chunk.write_le_f32(matrix.y.x)?;
                                chunk.write_le_f32(matrix.y.y)?;
                                chunk.write_le_f32(matrix.y.z)?;

                                chunk.write_le_f32(matrix.z.x)?;
                                chunk.write_le_f32(matrix.z.y)?;
                                chunk.write_le_f32(matrix.z.z)?;
                            }
                        }

                        chunk.write_interleaved_f32_array(x)?;
                        chunk.write_interleaved_f32_array(y)?;
                        chunk.write_interleaved_f32_array(z)?;

                        chunk.write_u8(Type::Bool as u8)?;
                        chunk.write_all(bools.as_slice())?;
                    }
                    BorrowedVariantVec::UniqueId(values) => {
                        let mut blobs = Vec::with_capacity(values.len());
                        for &value in values {
                            let mut blob = [0; 16];
                            // This is maybe not the best solution to this
                            // but we can always change it.
                            blob[0..4].copy_from_slice(&value.index().to_be_bytes());
                            blob[4..8].copy_from_slice(&value.time().to_be_bytes());
                            blob[8..].copy_from_slice(&value.random().rotate_left(1).to_be_bytes());
                            blobs.push(blob);
                        }

                        chunk.write_interleaved_bytes::<16>(&blobs)?;
                    }
                    BorrowedVariantVec::SecurityCapabilities(values) => {
                        chunk.write_interleaved_i64_array(
                            values.into_iter().map(|&value| value.bits() as i64),
                        )?;
                    }
                    BorrowedVariantVec::Content(values) => {
                        let mut source_types = Vec::with_capacity(values.len());
                        let mut uris = Vec::with_capacity(values.len());
                        let mut objects = Vec::new();
                        for (i, &value) in values.into_iter().enumerate() {
                            source_types.push(match value.value() {
                                ContentType::None => 0,
                                ContentType::Uri(uri) => {
                                    uris.push(uri.clone());
                                    1
                                }
                                ContentType::Object(referent) => {
                                    if let Some(id) = self.id_to_referent.get(referent) {
                                        objects.push(*id);
                                    } else {
                                        objects.push(-1);
                                    }
                                    2
                                }
                                _ => {
                                    return Err(invalid_value(i, &Variant::Content(value.clone())))
                                }
                            });
                        }
                        chunk.write_interleaved_i32_array(source_types)?;

                        chunk.write_le_u32(uris.len() as u32)?;
                        for uri in uris {
                            chunk.write_string(&uri)?;
                        }
                        chunk.write_le_u32(objects.len() as u32)?;
                        chunk.write_referent_array(objects)?;

                        // If we ever need to support the external referents,
                        // we will need to add it here.
                        chunk.write_le_u32(0)?;
                    }
                }

                chunk.dump(&mut self.output)?;
            }
        }

        Ok(())
    }

    /// Write out the hierarchical relations between instances, stored in a
    /// chunk named PRNT.
    #[profiling::function]
    pub fn serialize_parents(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing parent relationships");

        let mut chunk = ChunkBuilder::new(b"PRNT", self.serializer.compression);

        chunk.write_u8(0)?; // PRNT version 0
        chunk.write_le_u32(self.relevant_instances.len() as u32)?;

        let object_referents = self
            .relevant_instances
            .iter()
            .map(|id| self.id_to_referent[id]);

        let parent_referents = self.relevant_instances.iter().map(|id| {
            let instance = self.dom.get_by_ref(*id).unwrap();

            // If there's no parent set OR our parent is not one of the
            // instances we're serializing, we use -1 to represent a null
            // parent.
            if instance.parent().is_some() {
                self.id_to_referent
                    .get(&instance.parent())
                    .cloned()
                    .unwrap_or(-1)
            } else {
                -1
            }
        });

        chunk.write_referent_array(object_referents)?;
        chunk.write_referent_array(parent_referents)?;

        chunk.dump(&mut self.output)?;

        Ok(())
    }

    /// Write the fixed, uncompressed end chunk used to verify that the file
    /// hasn't been truncated mistakenly. This chunk is named END\0, with a zero
    /// byte at the end.
    #[profiling::function]
    pub fn serialize_end(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing file end");

        let mut end = ChunkBuilder::new(b"END\0", CompressionType::None);
        end.write_all(FILE_FOOTER)?;
        end.dump(&mut self.output)?;

        Ok(())
    }

    /// Equivalent to Instance:GetFullName() from Roblox.
    fn full_name_for(&self, subject_ref: Ref) -> String {
        let mut components = Vec::new();
        let mut current_id = subject_ref;

        while current_id.is_some() {
            let instance = self.dom.get_by_ref(current_id).unwrap();
            components.push(instance.name.as_str());
            current_id = instance.parent();
        }

        let mut name = String::new();
        for component in components.iter().rev() {
            name.push_str(component);
            name.push('.');
        }
        name.pop();

        name
    }
}
static DEFAULT_STRING: Variant = Variant::String(String::new());
fn fallback_default_value(rbx_type: VariantType) -> Option<&'static Variant> {
    static DEFAULT_BINARYSTRING: Variant = Variant::BinaryString(BinaryString::new());
    static DEFAULT_BOOL: Variant = Variant::Bool(false);
    static DEFAULT_INT32: Variant = Variant::Int32(0);
    static DEFAULT_FLOAT32: Variant = Variant::Float32(0.0);
    static DEFAULT_FLOAT64: Variant = Variant::Float64(0.0);
    static DEFAULT_UDIM: Variant = Variant::UDim(UDim::new(0.0, 0));
    static DEFAULT_UDIM2: Variant =
        Variant::UDim2(UDim2::new(UDim::new(0.0, 0), UDim::new(0.0, 0)));
    static DEFAULT_RAY: Variant = Variant::Ray(Ray::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ));
    static DEFAULT_FACES: Variant = Variant::Faces(Faces::from_bits(0).unwrap());
    static DEFAULT_AXES: Variant = Variant::Axes(Axes::from_bits(0).unwrap());
    static DEFAULT_BRICKCOLOR: Variant = Variant::BrickColor(BrickColor::MediumStoneGrey);
    static DEFAULT_CFRAME: Variant = Variant::CFrame(CFrame::new(
        Vector3::new(0.0, 0.0, 0.0),
        Matrix3::identity(),
    ));
    static DEFAULT_ENUM: Variant = Variant::Enum(Enum::from_u32(u32::MAX));
    static DEFAULT_COLOR3: Variant = Variant::Color3(Color3::new(0.0, 0.0, 0.0));
    static DEFAULT_VECTOR2: Variant = Variant::Vector2(Vector2::new(0.0, 0.0));
    static DEFAULT_VECTOR3: Variant = Variant::Vector3(Vector3::new(0.0, 0.0, 0.0));
    static DEFAULT_REF: Variant = Variant::Ref(Ref::none());
    static DEFAULT_VECTOR3INT16: Variant = Variant::Vector3int16(Vector3int16::new(0, 0, 0));
    lazy_static::lazy_static! {
        static ref DEFAULT_NUMBERSEQUENCE: Variant = Variant::NumberSequence(NumberSequence {
            keypoints: vec![
                NumberSequenceKeypoint::new(0.0, 0.0, 0.0),
                NumberSequenceKeypoint::new(0.0, 0.0, 0.0),
            ],
        });
    }
    lazy_static::lazy_static! {
        static ref DEFAULT_COLORSEQUENCE: Variant = Variant::ColorSequence(ColorSequence {
            keypoints: vec![
                ColorSequenceKeypoint::new(0.0, Color3::new(0.0, 0.0, 0.0)),
                ColorSequenceKeypoint::new(0.0, Color3::new(0.0, 0.0, 0.0)),
            ],
        });
    }
    static DEFAULT_NUMBERRANGE: Variant = Variant::NumberRange(NumberRange::new(0.0, 0.0));
    static DEFAULT_RECT: Variant =
        Variant::Rect(Rect::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)));
    static DEFAULT_PHYSICALPROPERTIES: Variant =
        Variant::PhysicalProperties(PhysicalProperties::Default);
    static DEFAULT_COLOR3UINT8: Variant = Variant::Color3uint8(Color3uint8::new(0, 0, 0));
    static DEFAULT_INT64: Variant = Variant::Int64(0);
    lazy_static::lazy_static! {
        static ref DEFAULT_SHAREDSTRING: Variant =
            Variant::SharedString(SharedString::new(Vec::new()));
    }
    static DEFAULT_OPTIONALCFRAME: Variant = Variant::OptionalCFrame(None);
    static DEFAULT_TAGS: Variant = Variant::Tags(Tags::new());
    static DEFAULT_CONTENTID: Variant = Variant::ContentId(ContentId::new());
    static DEFAULT_ATTRIBUTES: Variant = Variant::Attributes(Attributes::new());
    static DEFAULT_UNIQUEID: Variant = Variant::UniqueId(UniqueId::nil());
    lazy_static::lazy_static! {
        static ref DEFAULT_FONT: Variant = Variant::Font(Font::default());
    }
    static DEFAULT_MATERIALCOLORS: Variant = Variant::MaterialColors(MaterialColors::new());
    static DEFAULT_SECURITYCAPABILITIES: Variant =
        Variant::SecurityCapabilities(SecurityCapabilities::from_bits(0));
    static DEFAULT_CONTENT: Variant = Variant::Content(Content::none());
    Some(match rbx_type {
        VariantType::String => &DEFAULT_STRING,
        VariantType::BinaryString => &DEFAULT_BINARYSTRING,
        VariantType::Bool => &DEFAULT_BOOL,
        VariantType::Int32 => &DEFAULT_INT32,
        VariantType::Float32 => &DEFAULT_FLOAT32,
        VariantType::Float64 => &DEFAULT_FLOAT64,
        VariantType::UDim => &DEFAULT_UDIM,
        VariantType::UDim2 => &DEFAULT_UDIM2,
        VariantType::Ray => &DEFAULT_RAY,
        VariantType::Faces => &DEFAULT_FACES,
        VariantType::Axes => &DEFAULT_AXES,
        VariantType::BrickColor => &DEFAULT_BRICKCOLOR,
        VariantType::CFrame => &DEFAULT_CFRAME,
        VariantType::Enum => &DEFAULT_ENUM,
        VariantType::Color3 => &DEFAULT_COLOR3,
        VariantType::Vector2 => &DEFAULT_VECTOR2,
        VariantType::Vector3 => &DEFAULT_VECTOR3,
        VariantType::Ref => &DEFAULT_REF,
        VariantType::Vector3int16 => &DEFAULT_VECTOR3INT16,
        VariantType::NumberSequence => &DEFAULT_NUMBERSEQUENCE,
        VariantType::ColorSequence => &DEFAULT_COLORSEQUENCE,
        VariantType::NumberRange => &DEFAULT_NUMBERRANGE,
        VariantType::Rect => &DEFAULT_RECT,
        VariantType::PhysicalProperties => &DEFAULT_PHYSICALPROPERTIES,
        VariantType::Color3uint8 => &DEFAULT_COLOR3UINT8,
        VariantType::Int64 => &DEFAULT_INT64,
        VariantType::SharedString => &DEFAULT_SHAREDSTRING,
        VariantType::OptionalCFrame => &DEFAULT_OPTIONALCFRAME,
        VariantType::Tags => &DEFAULT_TAGS,
        VariantType::ContentId => &DEFAULT_CONTENTID,
        VariantType::Attributes => &DEFAULT_ATTRIBUTES,
        VariantType::UniqueId => &DEFAULT_UNIQUEID,
        VariantType::Font => &DEFAULT_FONT,
        VariantType::MaterialColors => &DEFAULT_MATERIALCOLORS,
        VariantType::SecurityCapabilities => &DEFAULT_SECURITYCAPABILITIES,
        VariantType::Content => &DEFAULT_CONTENT,
        _ => return None,
    })
}
