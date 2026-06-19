use std::{borrow::Cow, collections::VecDeque, convert::TryInto};

use ahash::{HashMap, HashMapExt};
use rbx_dom_weak::{
    types::{
        Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
        ColorSequenceKeypoint, Content, ContentId, CustomPhysicalProperties, Enum, Faces, Font,
        FontStyle, FontWeight, MaterialColors, Matrix3, NetAssetRef, NumberRange, NumberSequence,
        NumberSequenceKeypoint, PhysicalProperties, Ray, Rect, Ref, SecurityCapabilities,
        SharedString, Tags, UDim, UDim2, UniqueId, Variant, VariantType, Vector2, Vector3,
        Vector3int16,
    },
    Ustr, UstrMap, WeakDom,
};
use rbx_reflection::{ClassDescriptor, PropertyKind, PropertySerialization, ReflectionDatabase};

use crate::{
    core::{find_property_descriptors, RbxReadExt, RbxReadInterleaved, ReadSlice},
    types::Type,
};

use super::error::InnerError;

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};
#[cfg(feature = "rayon")]
type VecIntoIter<T> = rayon::vec::IntoIter<T>;
#[cfg(not(feature = "rayon"))]
type VecIntoIter<T> = std::vec::IntoIter<T>;

/// The metadata contained in the file, which affects how some constructs
/// are interpreted by Roblox.
type Metadata<'a> = HashMap<&'a str, &'a str>;

/// The SharedStrings contained in the file, if any, in the order that they
/// appear in the file.
type SharedStrings = Vec<SharedString>;

/// All of the instance types described by the file so far.
/// The index is the type_id.
type TypeInfos<'db> = HashMap<u32, TypeInfo<'db>>;

/// referent id -> (referent, parent)
type Instances = HashMap<i32, Instance>;

/// Represents a unique instance class. Binary models define all their instance
/// types up front and give them a short u32 identifier.
struct TypeInfo<'dom> {
    /// The common name for this type like `Folder` or `UserInputService`.
    type_name: Ustr,

    /// The referent, children, and parent for each instance in order.
    instances: Vec<Instance>,

    /// A collection of property chunks for this type, with the value for each instance in order.
    properties: UstrMap<Vec<Variant>>,

    /// A reference to the type's class descriptor from rbx_reflection, if this
    /// is a known class.
    class_descriptor: Option<&'dom ClassDescriptor<'dom>>,
}

struct Instance {
    referent: Ref,
    children: Vec<Ref>,
    parent: Ref,
    name: String,
}

// TODO: rethink this, really only MissingTypeByte, UnknownProperty are necessary.  They are eventually ignored and become None
#[derive(Debug, thiserror::Error)]
enum PropChunkError {
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Missing Type Byte")]
    MissingTypeByte,
    #[error("Invalid Type Id {type_id}")]
    InvalidTypeId { type_id: u32 },
    #[error("Unknown Binary Type {binary_type_byte}")]
    UnknownBinaryType { binary_type_byte: u8 },
    #[error("Unknown Property")]
    UnknownProperty,
    #[error("Invalid property data: Property {type_name}.{prop_name} was expected to be {valid_value}, but it was {actual_value}")]
    InvalidPropData {
        type_name: String,
        prop_name: String,
        valid_value: &'static str,
        actual_value: String,
    },
    #[error(
        "Type mismatch: Property {type_name}.{prop_name} should be {valid_type_names}, but it was {actual_type_name}",
    )]
    PropTypeMismatch {
        type_name: String,
        prop_name: String,
        valid_type_names: &'static str,
        actual_type_name: String,
    },
    #[error("Invalid property data: CFrame property {type_name}.{prop_name} had an invalid rotation ID {id:02x}")]
    BadRotationId {
        type_name: String,
        prop_name: String,
        id: u8,
    },
    #[error("'PhysicalProperties' discriminator {0:b} is not supported")]
    BadPhysicalPropertiesType(u8),
    #[error("Expected type id for {expected_type_name} ({expected_type_id:02x}) when reading OptionalCFrame; got {actual_type_id:02x}")]
    BadOptionalCFrameFormat {
        expected_type_name: String,
        expected_type_id: u8,
        actual_type_id: u8,
    },

    #[error("'Content' type {0} is not implemented")]
    BadContentType(i32),
}

struct PropChunk<'db> {
    type_id: u32,
    canonical_property: CanonicalProperty<'db>,
    values: Vec<Variant>,
}

impl<'db> PropChunk<'db> {
    /// Generate a property chunk given a function that generates property values.
    fn from_fn<V, F>(
        type_id: u32,
        canonical_property: CanonicalProperty<'db>,
        len: usize,
        f: F,
    ) -> Result<Self, PropChunkError>
    where
        V: Into<Variant>,
        F: FnMut() -> Result<V, PropChunkError>,
    {
        // helper to create value iterators
        struct SomeFromFn<F> {
            f: F,
            len: usize,
        }

        impl<T, F> Iterator for SomeFromFn<F>
        where
            F: FnMut() -> T,
        {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                Some((self.f)())
            }
        }
        impl<F, T> ExactSizeIterator for SomeFromFn<F>
        where
            F: FnMut() -> T,
        {
            fn len(&self) -> usize {
                self.len
            }
        }

        let values = SomeFromFn { f, len }
            .map(|result| result.map(Into::into))
            .collect::<Result<Vec<Variant>, PropChunkError>>()?;

        Ok(Self {
            type_id,
            canonical_property,
            values,
        })
    }
    /// Generate a property chunk from an infallible iterator
    fn from_iter<V, I>(type_id: u32, canonical_property: CanonicalProperty<'db>, values: I) -> Self
    where
        V: Into<Variant>,
        I: IntoIterator<Item = V, IntoIter: ExactSizeIterator>,
    {
        let values = values.into_iter().map(Into::into).collect();
        Self {
            type_id,
            canonical_property,
            values,
        }
    }
}

/// Properties may be serialized under different names or types than
/// they ultimately should have in the DOM. CanonicalProperty
/// represents the "proper" name and type of a property, and possibly
/// contains a migration for some properties Roblox has replaced with
/// others (like Font, which has been superceded by FontFace).
#[derive(Debug)]
struct CanonicalProperty<'db> {
    name: Ustr,
    ty: VariantType,
    migration: Option<&'db PropertySerialization<'db>>,
}

fn find_canonical_property<'de>(
    database: &'de ReflectionDatabase,
    binary_type: Type,
    class_descriptor: Option<&'de ClassDescriptor<'de>>,
    prop_name: &str,
) -> Option<CanonicalProperty<'de>> {
    match find_property_descriptors(database, class_descriptor, prop_name) {
        Some((_, descriptors)) => {
            // If this descriptor is known but wasn't supposed to be
            // serialized, we should skip it.
            //
            // On 2021-09-07 (v494), BasePart.MaterialVariant was added as a
            // serializable Referent property. It was removed soon after, on
            // 2021-10-12 (v499). Any models saved during that period have
            // BasePart.MaterialVariant present.
            //
            // On 2022-03-08 (v517), BasePart.MaterialVariant was
            // reintroduced as a string property that does not serialize. It
            // serializes as MaterialVariantSerialized.
            //
            // In case we run into a model serialized during that period, or
            // this happens again, we need to make sure that the name we
            // found is the one that's supposed to serialize.
            if let PropertyKind::Canonical { serialization } = &descriptors.canonical.kind {
                if matches!(serialization, PropertySerialization::DoesNotSerialize) {
                    log::debug!(
                        "Skipping property {} as it is canonical and should not serialize.",
                        descriptors.canonical.name
                    );
                    return None;
                }
            }

            // TODO: Do we need an additional fix here?
            let canonical_name = descriptors.canonical.name;
            let canonical_type = descriptors.canonical.data_type.ty();
            let migration = match &descriptors.canonical.kind {
                PropertyKind::Canonical {
                    serialization: migration @ PropertySerialization::Migrate(_),
                } => Some(migration),
                _ => None,
            };

            log::trace!(
                "Known prop, canonical name {canonical_name} and type {canonical_type:?}, with {migration:?} migration",
            );

            Some(CanonicalProperty {
                name: canonical_name.into(),
                ty: canonical_type,
                migration,
            })
        }
        None => {
            let canonical_type = match binary_type.to_default_rbx_type() {
                Some(rbx_type) => rbx_type,
                None => {
                    log::warn!("Unsupported prop type {binary_type:?}, skipping property");
                    return None;
                }
            };

            log::trace!("Unknown prop, using type {canonical_type:?}");

            Some(CanonicalProperty {
                name: prop_name.into(),
                ty: canonical_type,
                migration: None,
            })
        }
    }
}

// keep the tab depth the same
pub(super) use tab_saver::*;
mod tab_saver {
    use super::*;

    #[profiling::function]
    pub fn decode_meta_chunk<'a>(mut chunk: &'a [u8]) -> Result<Metadata<'a>, InnerError> {
        let len = chunk.read_le_u32()?;
        let mut metadata = HashMap::with_capacity(len as usize);

        for _ in 0..len {
            let key = chunk.read_string()?;
            let value = chunk.read_string()?;

            metadata.insert(key, value);
        }

        Ok(metadata)
    }

    #[profiling::function]
    pub fn decode_sstr_chunk(mut chunk: &[u8]) -> Result<SharedStrings, InnerError> {
        let version = chunk.read_le_u32()?;

        if version != 0 {
            return Err(InnerError::UnknownChunkVersion {
                chunk_name: "SSTR",
                version,
            });
        }

        let num_entries = chunk.read_le_u32()?;
        let mut shared_strings = Vec::with_capacity(num_entries as usize);

        for _ in 0..num_entries {
            let _hash = chunk.read_slice(16)?; // We don't do anything with the hash.
            let data = chunk.read_binary_string()?.to_owned();
            shared_strings.push(SharedString::new(data));
        }

        Ok(shared_strings)
    }

    #[profiling::function]
    pub fn decode_inst_chunk<'db>(
        mut chunk: &[u8],
        database: &'db ReflectionDatabase<'_>,
        instances: &mut HashMap<i32, Instance>,
        type_infos: &mut TypeInfos<'db>,
    ) -> Result<(), InnerError> {
        let type_id = chunk.read_le_u32()?;
        let type_name = chunk.read_string()?;
        let object_format = chunk.read_u8()?;
        let number_instances = chunk.read_le_u32()?;

        log::trace!(
            "INST chunk (type ID {type_id}, type name {type_name}, format {object_format}, {number_instances} instances)",
        );

        let referents = chunk.read_referent_array(number_instances as usize)?;

        let (class_descriptor, prop_capacity) = if let Some(class) = database.classes.get(type_name)
        {
            (Some(class), class.default_properties.len())
        } else {
            (None, 0)
        };

        // TODO: Check object_format and check for service markers if it's 1?

        let instances = referents
            .map(|referent| instances.remove(&referent).ok_or(referent))
            .collect::<Result<Vec<_>, i32>>()
            // Every referent should be unique
            .map_err(|referent| InnerError::DuplicateReferent { referent })?;

        let replaced_type_info = type_infos.insert(
            type_id,
            TypeInfo {
                type_name: type_name.into(),
                instances,
                properties: UstrMap::with_capacity(prop_capacity),
                class_descriptor,
            },
        );

        // Every type_id should be unique
        if replaced_type_info.is_some() {
            return Err(InnerError::DuplicateInstChunk { type_id });
        };

        Ok(())
    }

    #[profiling::function]
    pub fn decode_prop_chunk<'a, 'de: 'a>(
        mut chunk: &'a [u8],
        database: &'de ReflectionDatabase<'de>,
        type_infos: &TypeInfos<'de>,
        shared_strings: &[SharedString],
        ref_by_id: &HashMap<i32, Ref>,
    ) -> Result<PropChunk<'a>, PropChunkError> {
        let type_id = chunk.read_le_u32()?;
        let prop_name = chunk.read_string()?;

        // TODO: collect prop chunk results first, then assemble into TypeInfos
        let type_info = type_infos
            .get(&type_id)
            .ok_or(PropChunkError::InvalidTypeId { type_id })?;

        let type_name = type_info.type_name;
        let class_descriptor = type_info.class_descriptor;
        let num_instances = type_info.instances.len();

        // PROP chunks that contain no type byte are ignored by Roblox. This can
        // happen when a new type is introduced.
        //
        // On 2021-04-08, OptionalCoordinateFrame was introduced, but its
        // serialized format was just a type ID followed by the prop name. This
        // leads us to believe that Roblox will silently ignore any PROP chunks
        // that end immediately after the prop name, so we do the same.
        let binary_type_byte = match chunk.read_u8() {
            Ok(byte) => byte,
            Err(_) => return Err(PropChunkError::MissingTypeByte),
        };

        let binary_type: Type = match binary_type_byte.try_into() {
            Ok(ty) => ty,
            Err(_) => return Err(PropChunkError::UnknownBinaryType { binary_type_byte }),
        };

        log::trace!(
            "PROP chunk ({}.{}, instance type {}",
            type_name,
            prop_name,
            type_id
        );

        // TODO: figure out how to skip this for Name property
        let property = if let Some(property) =
            find_canonical_property(database, binary_type, class_descriptor, prop_name)
        {
            property
        } else {
            return Err(PropChunkError::UnknownProperty);
        };

        // The `Name` prop is special and is routed to a different spot for
        // rbx_dom_weak, so we handle it specially here.
        if prop_name == "Name" {
            // TODO: If an instance is never assigned a name through this code
            // path, we should use the reflection database to figure out its
            // default name. This should be rare: effectively never!

            return PropChunk::from_fn(type_id, property, num_instances, || {
                let binary_string = chunk.read_binary_string()?;
                Ok(match std::str::from_utf8(binary_string) {
                    Ok(value) => value.to_owned(),
                    Err(_) => {
                        log::warn!(
                            "Performing lossy string conversion on property {}.{} because it did not contain UTF-8.
This may cause unexpected or broken behavior in your final results if you rely on this property being non UTF-8.",
                                type_name,
                                prop_name
                            );

                        String::from_utf8_lossy(binary_string).into_owned()
                    }
                })
            });
        }

        let canonical_type = property.ty;
        let canonical_name = property.name;

        match binary_type {
            Type::String => match canonical_type {
                VariantType::String => PropChunk::from_fn(type_id, property, num_instances, || {
                    let binary_string = chunk.read_binary_string()?;
                    Ok(match std::str::from_utf8(binary_string) {
                        Ok(value) => value.to_owned(),
                        Err(_) => {
                            log::warn!(
                            "Performing lossy string conversion on property {}.{} because it did not contain UTF-8.
This may cause unexpected or broken behavior in your final results if you rely on this property being non UTF-8.",
                                    type_name,
                                    canonical_name
                                );

                            String::from_utf8_lossy(binary_string).into_owned()
                        }
                    })
                }),
                VariantType::ContentId => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        Ok(ContentId::from(chunk.read_string()?.to_owned()))
                    })
                }
                VariantType::BinaryString => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        Ok(BinaryString::from(chunk.read_binary_string()?.to_owned()))
                    })
                }
                VariantType::Tags => PropChunk::from_fn(type_id, property, num_instances, || {
                    let buffer = chunk.read_binary_string()?;

                    let value =
                        Tags::decode(buffer).map_err(|_| PropChunkError::InvalidPropData {
                            type_name: type_name.to_string(),
                            prop_name: prop_name.to_owned(),
                            valid_value: "a list of valid null-delimited UTF-8 strings",
                            actual_value: "invalid UTF-8".to_string(),
                        })?;

                    Ok(value)
                }),
                VariantType::Attributes => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let buffer = chunk.read_binary_string()?;

                        Ok(match Attributes::from_reader(buffer) {
                            Ok(value) => Variant::from(value),
                            Err(err) => {
                                log::warn!(
                                    "Failed to parse Attributes on {} because {:?}; falling back to BinaryString.
rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning.",
                                    type_name,
                                    err
                                );

                                Variant::from(BinaryString::from(buffer))
                            }
                        })
                    })
                }
                VariantType::MaterialColors => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let buffer = chunk.read_binary_string()?;
                        Ok(match MaterialColors::decode(buffer) {
                            Ok(value) => Variant::from(value),
                            Err(err) => {
                                log::warn!(
                                    "Failed to parse MaterialColors on {} because {:?}; falling back to BinaryString.

rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning.",
                                    type_name,
                                    err
                                );

                                Variant::from(BinaryString::from(buffer))
                            }
                        })
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names:
                            "String, ContentId, Content, Tags, Attributes, or BinaryString",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Bool => match canonical_type {
                VariantType::Bool => {
                    PropChunk::from_fn(type_id, property, num_instances, || Ok(chunk.read_bool()?))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Bool",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Int32 => match canonical_type {
                VariantType::Int32 => Ok(PropChunk::from_iter(
                    type_id,
                    property,
                    chunk.read_interleaved_i32_array(num_instances)?,
                )),
                // This branch allows values serialized as Int32 to be converted to Int64 when we expect a Int64
                // Basically, we convert Int32 to Int64 when we expect a Int64 but read a Int32
                // See: #301
                VariantType::Int64 => Ok(PropChunk::from_iter(
                    type_id,
                    property,
                    chunk
                        .read_interleaved_i32_array(num_instances)?
                        .map(i64::from),
                )),
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Int32",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Float32 => match canonical_type {
                VariantType::Float32 => Ok(PropChunk::from_iter(
                    type_id,
                    property,
                    chunk.read_interleaved_f32_array(num_instances)?,
                )),
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Float32",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Float64 => match canonical_type {
                VariantType::Float64 => PropChunk::from_fn(
                    type_id,
                    property,
                    num_instances,
                    || Ok(chunk.read_le_f64()?),
                ),
                // This branch allows values serialized as Float32 to be converted to Float64 when we expect a Float64
                // Basically, we convert Float32 to Float64 when we expect a Float64 but read a Float32
                // See: #301
                VariantType::Float32 => Ok(PropChunk::from_iter(
                    type_id,
                    property,
                    chunk
                        .read_interleaved_f32_array(num_instances)?
                        .map(f64::from),
                )),
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Float64",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::UDim => match canonical_type {
                VariantType::UDim => {
                    let scales = chunk.read_interleaved_f32_array(num_instances)?;
                    let offsets = chunk.read_interleaved_i32_array(num_instances)?;

                    let values = scales
                        .zip(offsets)
                        .map(|(scale, offset)| UDim::new(scale, offset));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "UDim",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::UDim2 => match canonical_type {
                VariantType::UDim2 => {
                    let prop_count = num_instances;
                    let scale_x = chunk.read_interleaved_f32_array(prop_count)?;
                    let scale_y = chunk.read_interleaved_f32_array(prop_count)?;
                    let offset_x = chunk.read_interleaved_i32_array(prop_count)?;
                    let offset_y = chunk.read_interleaved_i32_array(prop_count)?;

                    let x = scale_x
                        .zip(offset_x)
                        .map(|(scale, offset)| UDim::new(scale, offset));

                    let y = scale_y
                        .zip(offset_y)
                        .map(|(scale, offset)| UDim::new(scale, offset));

                    let values = x.zip(y).map(|(x, y)| UDim2::new(x, y));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "UDim2",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Ray => match canonical_type {
                VariantType::Ray => PropChunk::from_fn(type_id, property, num_instances, || {
                    let origin_x = chunk.read_le_f32()?;
                    let origin_y = chunk.read_le_f32()?;
                    let origin_z = chunk.read_le_f32()?;
                    let direction_x = chunk.read_le_f32()?;
                    let direction_y = chunk.read_le_f32()?;
                    let direction_z = chunk.read_le_f32()?;

                    Ok(Ray::new(
                        Vector3::new(origin_x, origin_y, origin_z),
                        Vector3::new(direction_x, direction_y, direction_z),
                    ))
                }),
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Ray",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Faces => match canonical_type {
                VariantType::Faces => PropChunk::from_fn(type_id, property, num_instances, || {
                    let value = chunk.read_u8()?;
                    let faces =
                        Faces::from_bits(value).ok_or_else(|| PropChunkError::InvalidPropData {
                            type_name: type_name.to_string(),
                            prop_name: prop_name.to_owned(),
                            valid_value: "less than 63",
                            actual_value: value.to_string(),
                        })?;

                    Ok(faces)
                }),
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Faces",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Axes => match canonical_type {
                VariantType::Axes => PropChunk::from_fn(type_id, property, num_instances, || {
                    let value = chunk.read_u8()?;

                    let axes =
                        Axes::from_bits(value).ok_or_else(|| PropChunkError::InvalidPropData {
                            type_name: type_name.to_string(),
                            prop_name: prop_name.to_owned(),
                            valid_value: "less than 7",
                            actual_value: value.to_string(),
                        })?;

                    Ok(axes)
                }),
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Axes",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::BrickColor => match canonical_type {
                VariantType::BrickColor => {
                    let mut values = chunk.read_interleaved_u32_array(num_instances)?;

                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let value = values.next().unwrap();
                        let color = value
                            .try_into()
                            .ok()
                            .and_then(BrickColor::from_number)
                            .ok_or_else(|| PropChunkError::InvalidPropData {
                                type_name: type_name.to_string(),
                                prop_name: prop_name.to_owned(),
                                valid_value: "a valid BrickColor",
                                actual_value: value.to_string(),
                            })?;

                        Ok(color)
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "BrickColor",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Color3 => match canonical_type {
                VariantType::Color3 => {
                    let r = chunk.read_interleaved_f32_array(num_instances)?;
                    let g = chunk.read_interleaved_f32_array(num_instances)?;
                    let b = chunk.read_interleaved_f32_array(num_instances)?;

                    let values = r.zip(g).zip(b).map(|((r, g), b)| Color3::new(r, g, b));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Color3",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Vector2 => match canonical_type {
                VariantType::Vector2 => {
                    let x = chunk.read_interleaved_f32_array(num_instances)?;
                    let y = chunk.read_interleaved_f32_array(num_instances)?;

                    let values = x.zip(y).map(|(x, y)| Vector2::new(x, y));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Vector2",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Vector3 => match canonical_type {
                VariantType::Vector3 => {
                    let x = chunk.read_interleaved_f32_array(num_instances)?;
                    let y = chunk.read_interleaved_f32_array(num_instances)?;
                    let z = chunk.read_interleaved_f32_array(num_instances)?;

                    let values = x.zip(y).zip(z).map(|((x, y), z)| Vector3::new(x, y, z));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Vector3",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::CFrame => match canonical_type {
                VariantType::CFrame => {
                    let mut rotations = Vec::with_capacity(num_instances);

                    for _ in 0..num_instances {
                        let id = chunk.read_u8()?;
                        if id == 0 {
                            rotations.push(Matrix3::new(
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                            ));
                        } else if let Ok(basic_rotation) = Matrix3::from_basic_rotation_id(id) {
                            rotations.push(basic_rotation);
                        } else {
                            return Err(PropChunkError::BadRotationId {
                                type_name: type_name.to_string(),
                                prop_name: prop_name.to_owned(),
                                id,
                            });
                        }
                    }

                    let x = chunk.read_interleaved_f32_array(num_instances)?;
                    let y = chunk.read_interleaved_f32_array(num_instances)?;
                    let z = chunk.read_interleaved_f32_array(num_instances)?;

                    let values = x
                        .zip(y)
                        .zip(z)
                        .map(|((x, y), z)| Vector3::new(x, y, z))
                        .zip(rotations)
                        .map(|(position, rotation)| CFrame::new(position, rotation));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "CFrame",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Enum => match canonical_type {
                VariantType::Enum => {
                    let values = chunk
                        .read_interleaved_u32_array(num_instances)?
                        .map(Enum::from_u32);

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Enum",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Ref => match canonical_type {
                VariantType::Ref => {
                    let values = chunk
                        .read_referent_array(num_instances)?
                        .map(|value| ref_by_id.get(&value).copied().unwrap_or(Ref::none()));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Ref",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Vector3int16 => match canonical_type {
                VariantType::Vector3int16 => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        Ok(Vector3int16::new(
                            chunk.read_le_i16()?,
                            chunk.read_le_i16()?,
                            chunk.read_le_i16()?,
                        ))
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Vector3int16",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Font => match canonical_type {
                VariantType::Font => PropChunk::from_fn(type_id, property, num_instances, || {
                    let family = chunk.read_string()?.to_owned();
                    let weight = FontWeight::from_u16(chunk.read_le_u16()?).unwrap_or_default();
                    let style = FontStyle::from_u8(chunk.read_u8()?).unwrap_or_default();
                    let cached_face_id = chunk.read_string()?.to_owned();

                    let cached_face_id = if cached_face_id.is_empty() {
                        None
                    } else {
                        Some(cached_face_id)
                    };

                    Ok(Font {
                        family,
                        weight,
                        style,
                        cached_face_id,
                    })
                }),
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Font",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::NumberSequence => match canonical_type {
                VariantType::NumberSequence => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let keypoint_count = chunk.read_le_u32()?;
                        let mut keypoints = Vec::with_capacity(keypoint_count as usize);

                        for _ in 0..keypoint_count {
                            keypoints.push(NumberSequenceKeypoint::new(
                                chunk.read_le_f32()?,
                                chunk.read_le_f32()?,
                                chunk.read_le_f32()?,
                            ))
                        }

                        Ok(NumberSequence { keypoints })
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "NumberSequence",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::ColorSequence => match canonical_type {
                VariantType::ColorSequence => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let keypoint_count = chunk.read_le_u32()? as usize;
                        let mut keypoints = Vec::with_capacity(keypoint_count);

                        for _ in 0..keypoint_count {
                            keypoints.push(ColorSequenceKeypoint::new(
                                chunk.read_le_f32()?,
                                Color3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                            ));

                            // envelope is serialized but doesn't do anything; don't do anything with it
                            chunk.read_le_f32()?;
                        }

                        Ok(ColorSequence { keypoints })
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "ColorSequence",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::NumberRange => match canonical_type {
                VariantType::NumberRange => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        Ok(NumberRange::new(chunk.read_le_f32()?, chunk.read_le_f32()?))
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "NumberRange",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Rect => match canonical_type {
                VariantType::Rect => {
                    let len = num_instances;
                    let x_min = chunk.read_interleaved_f32_array(len)?;
                    let y_min = chunk.read_interleaved_f32_array(len)?;
                    let x_max = chunk.read_interleaved_f32_array(len)?;
                    let y_max = chunk.read_interleaved_f32_array(len)?;

                    let values = x_min.zip(y_min).zip(x_max).zip(y_max).map(
                        |(((x_min, y_min), x_max), y_max)| {
                            Rect::new(Vector2::new(x_min, y_min), Vector2::new(x_max, y_max))
                        },
                    );

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Rect",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::PhysicalProperties => match canonical_type {
                VariantType::PhysicalProperties => {
                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let discriminator = chunk.read_u8()?;
                        let value = match discriminator {
                            0b00 | 0b10 => Variant::PhysicalProperties(PhysicalProperties::Default),
                            0b01 => Variant::PhysicalProperties(PhysicalProperties::Custom(
                                CustomPhysicalProperties::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    1.0,
                                ),
                            )),
                            0b11 => Variant::PhysicalProperties(PhysicalProperties::Custom(
                                CustomPhysicalProperties::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                            )),
                            _ => {
                                return Err(PropChunkError::BadPhysicalPropertiesType(
                                    discriminator,
                                ))
                            }
                        };

                        Ok(value)
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "PhysicalProperties",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Color3uint8 => match canonical_type {
                VariantType::Color3 => {
                    let len = num_instances;
                    let r = chunk.read_slice(len)?;
                    let g = chunk.read_slice(len)?;
                    let b = chunk.read_slice(len)?;

                    let values = r
                        .iter()
                        .zip(g)
                        .zip(b)
                        .map(|((r, g), b)| Color3uint8::new(*r, *g, *b));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Color3",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Int64 => match canonical_type {
                VariantType::Int64 => {
                    let values = chunk.read_interleaved_i64_array(num_instances)?;

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Int64",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::SharedString => match canonical_type {
                VariantType::SharedString => {
                    let mut values = chunk.read_interleaved_u32_array(num_instances)?;

                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let value = values.next().unwrap();
                        let shared_string =
                            shared_strings.get(value as usize).ok_or_else(|| {
                                PropChunkError::InvalidPropData {
                                    type_name: type_name.to_string(),
                                    prop_name: prop_name.to_owned(),
                                    valid_value: "a valid SharedString",
                                    actual_value: format!("{value:?}"),
                                }
                            })?;

                        Ok(shared_string.clone())
                    })
                }
                VariantType::NetAssetRef => {
                    let mut values = chunk.read_interleaved_u32_array(num_instances)?;

                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let value = values.next().unwrap();
                        let net_asset = NetAssetRef::from(
                            shared_strings
                                .get(value as usize)
                                .ok_or_else(|| PropChunkError::InvalidPropData {
                                    type_name: type_name.to_string(),
                                    prop_name: prop_name.to_owned(),
                                    valid_value: "a valid NetAssetRef",
                                    actual_value: format!("{value:?}"),
                                })?
                                .clone(),
                        );

                        Ok(net_asset)
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "SharedString",
                        actual_type_name: format!("{invalid_type:?}"),
                    })
                }
            },
            Type::OptionalCFrame => match canonical_type {
                VariantType::OptionalCFrame => {
                    let mut rotations = Vec::with_capacity(num_instances);

                    // Roblox writes a type marker for CFrame here that we don't
                    // need to use. We explicitly check for this right now just
                    // in case we're wrong and we do need it!
                    let actual_type_id = chunk.read_u8()?;
                    if actual_type_id != Type::CFrame as u8 {
                        return Err(PropChunkError::BadOptionalCFrameFormat {
                            expected_type_name: String::from("CFrame"),
                            expected_type_id: Type::CFrame as u8,
                            actual_type_id,
                        });
                    }

                    for _ in 0..num_instances {
                        let id = chunk.read_u8()?;
                        if id == 0 {
                            rotations.push(Matrix3::new(
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                            ));
                        } else if let Ok(basic_rotation) = Matrix3::from_basic_rotation_id(id) {
                            rotations.push(basic_rotation);
                        } else {
                            return Err(PropChunkError::BadRotationId {
                                type_name: type_name.to_string(),
                                prop_name: prop_name.to_owned(),
                                id,
                            });
                        }
                    }

                    let x = chunk.read_interleaved_f32_array(num_instances)?;
                    let y = chunk.read_interleaved_f32_array(num_instances)?;
                    let z = chunk.read_interleaved_f32_array(num_instances)?;

                    // Roblox writes a type marker for Bool here that we don't
                    // need to use. We explicitly check for this right now just
                    // in case we're wrong and we do need it!
                    let actual_type_id = chunk.read_u8()?;
                    if actual_type_id != Type::Bool as u8 {
                        return Err(PropChunkError::BadOptionalCFrameFormat {
                            expected_type_name: String::from("Bool"),
                            expected_type_id: Type::Bool as u8,
                            actual_type_id,
                        });
                    }

                    let markers = chunk.read_slice(num_instances)?;

                    let values = x
                        .zip(y)
                        .zip(z)
                        .map(|((x, y), z)| Vector3::new(x, y, z))
                        .zip(rotations)
                        .zip(markers.iter().copied())
                        .map(|((position, rotation), marker)| {
                            if marker == 0 {
                                None
                            } else {
                                Some(CFrame::new(position, rotation))
                            }
                        });

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "OptionalCFrame",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::UniqueId => match canonical_type {
                VariantType::UniqueId => {
                    let n = num_instances;
                    let values = chunk.read_interleaved_bytes::<16>(n)?.map(|value| {
                        let mut value = value.as_slice();
                        UniqueId::new(
                            value.read_be_u32().unwrap(),
                            value.read_be_u32().unwrap(),
                            value.read_be_i64().unwrap().rotate_right(1),
                        )
                    });

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "UniqueId",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::SecurityCapabilities => match canonical_type {
                VariantType::SecurityCapabilities => {
                    let values = chunk.read_interleaved_i64_array(num_instances)?;

                    let values = values.map(|value| SecurityCapabilities::from_bits(value as u64));

                    Ok(PropChunk::from_iter(type_id, property, values))
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "SecurityCapabilities",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
            Type::Content => match canonical_type {
                VariantType::Content => {
                    let mut values = chunk.read_interleaved_i32_array(num_instances)?;

                    let uri_count = chunk.read_le_u32()? as usize;
                    let mut uris = VecDeque::with_capacity(uri_count);
                    for _ in 0..uri_count {
                        uris.push_front(chunk.read_string()?.to_owned());
                    }

                    let object_count = chunk.read_le_u32()? as usize;
                    let mut objects: VecDeque<i32> =
                        chunk.read_referent_array(object_count)?.collect();

                    let external_count = chunk.read_le_u32()? as usize;
                    // We are advised by Roblox to just ignore this, as it's
                    // meant for internal use. If we want to use it in the
                    // future, it's a referent array.
                    let _bytes = chunk.read_slice(external_count * 4)?;

                    PropChunk::from_fn(type_id, property, num_instances, || {
                        let value = match values.next().unwrap() {
                            0 => Content::none(),
                            1 => Content::from_uri(uris.pop_back().unwrap()),
                            2 => {
                                let read_value = objects.pop_back().unwrap();
                                if let Some(referent) = ref_by_id.get(&read_value) {
                                    Content::from_referent(*referent)
                                } else {
                                    Content::none()
                                }
                            }
                            n => return Err(PropChunkError::BadContentType(n)),
                        };
                        Ok(value)
                    })
                }
                invalid_type => {
                    return Err(PropChunkError::PropTypeMismatch {
                        type_name: type_name.to_string(),
                        prop_name: prop_name.to_owned(),
                        valid_type_names: "Content",
                        actual_type_name: format!("{invalid_type:?}"),
                    });
                }
            },
        }
    }

    #[profiling::function]
    pub fn decode_prnt_chunk(mut chunk: &[u8]) -> Result<Instances, InnerError> {
        let version = chunk.read_u8()?;

        if version != 0 {
            return Err(InnerError::UnknownChunkVersion {
                chunk_name: "PRNT",
                version: version as u32,
            });
        }

        let number_objects = chunk.read_le_u32()?;

        log::trace!("PRNT chunk ({number_objects} instances)");

        let subjects = chunk.read_referent_array(number_objects as usize)?;
        let parents = chunk.read_referent_array(number_objects as usize)?;

        let referents_map: HashMap<_, _> = subjects
            .zip(parents)
            .map(|(id, parent_id)| (id, (Ref::new(), parent_id)))
            .collect();
        let instances = referents_map
            .iter()
            .map(|(&id, &(referent, ref parent_id))| {
                (
                    id,
                    Instance {
                        referent,
                        parent: referents_map.get(parent_id).unwrap().0,
                        name: String::new(),
                        children: todo!(),
                    },
                )
            })
            .collect();

        Ok(instances)
    }

    #[profiling::function]
    pub fn decode_end_chunk(_chunk: &[u8]) -> Result<(), InnerError> {
        log::trace!("END chunk");

        // We don't do any validation on the END chunk. There's no useful
        // information for us here as it just signals that the file hasn't been
        // truncated.

        Ok(())
    }

    /// Combines together all the decoded information to build and emplace
    /// instances in our tree.
    #[profiling::function]
    pub fn finish(type_infos: TypeInfos<'_>) -> WeakDom {
        log::trace!("Constructing tree from deserialized data");

        let root_ref = Ref::new();

        // The necessary effort is put in to compute the required information ahead of time so that this operation can have maximum parallelism.
        // Each TypeInfo is populated with all the information needed to create Instances in parallel.
        #[cfg(feature = "rayon")]
        let instances = type_infos.into_par_iter();
        #[cfg(not(feature = "rayon"))]
        let instances = type_infos.into_iter();

        // TypeInfoContext implements IntoParallelIterator and yields Instances.
        // iterating over type_infos is parallel-flattened with the iterators for each class, giving per-instance parallelism.
        let instances = instances.flat_map(|(_, type_info)| type_info);

        // Collect into a vec in parallel and then convert into a HashMap.
        // This is done internally in rayon when parallel collecting a HashMap, but we can grab the referent from the instance if we do it manually.
        #[cfg(feature = "rayon")]
        let instances = instances.collect::<Vec<_>>().into_iter();

        let instances = instances
            // grab the referent from the instance
            .map(|instance| (instance.referent(), instance))
            .collect();

        WeakDom::from_raw(root_ref, instances)
    }
}
