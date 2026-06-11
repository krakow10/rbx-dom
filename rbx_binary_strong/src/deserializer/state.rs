use std::collections::HashMap;
use std::io;

use rbx_dom_strong::{StrongDom, classes, enums};
use rbx_types::{CFrame, Matrix3, SharedString, Vector3};

use rbx_binary_core::{
    chunk::ChunkSlices,
    core::{RbxReadExt, RbxReadInterleaved},
    header::FileHeader,
};

use super::error::InnerError;

#[cfg(not(feature = "rayon"))]
use core::iter::IntoIterator as IntoIter;
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator as IntoIter;
#[cfg(feature = "rayon")]
use rayon::iter::plumbing::{Consumer, ProducerCallback, UnindexedConsumer, bridge};
#[cfg(feature = "rayon")]
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

struct DuplicateProp;
impl From<DuplicateProp> for InnerError {
    fn from(_value: DuplicateProp) -> Self {
        InnerError::DuplicatePropChunk {
            prop_name: "TODO".to_owned(),
        }
    }
}
fn err_if_some<T>(value: Option<T>) -> Result<(), DuplicateProp> {
    match value {
        Some(_) => Err(DuplicateProp),
        None => Ok(()),
    }
}

// === GENERATED ===

#[derive(Debug, Clone, Copy)]
pub(super) enum ClassType {
    Part,
    WedgePart,
}

// A parallelizable chunk of property values which knows which class and property it is for.
enum ClassPropertyChunk {
    PartPropertyChunk(PartPropertyChunk),
    WedgePartPropertyChunk(WedgePartPropertyChunk),
}
impl ClassPropertyChunk {
    fn new(
        data: Vec<u8>,
        len: usize,
        prop_name: &str,
        class_type: ClassType,
    ) -> Result<Self, InnerError> {
        Ok(match class_type {
            ClassType::Part => {
                Self::PartPropertyChunk(PartPropertyChunk::new(data, len, prop_name, class_type)?)
            }
            ClassType::WedgePart => Self::WedgePartPropertyChunk(WedgePartPropertyChunk::new(
                data, len, prop_name, class_type,
            )?),
        })
    }
}

// This will be populated with chunk decoders by decode_prop_chunk
#[derive(Default)]
struct ClassPropertyChunks {
    Part: PartPropertyChunks,
    WedgePart: WedgePartPropertyChunks,
}

impl ClassPropertyChunks {
    fn try_push(&mut self, prop_chunk: ClassPropertyChunk) -> Result<(), DuplicateProp> {
        match prop_chunk {
            ClassPropertyChunk::PartPropertyChunk(part_property_chunk) => {
                self.Part.try_push(part_property_chunk)
            }
            ClassPropertyChunk::WedgePartPropertyChunk(wedge_part_property_chunk) => {
                self.WedgePart.try_push(wedge_part_property_chunk)
            }
        }
    }
}

struct InstanceIter {
    Name: <<String as IntoPropertyChunkState>::State as IntoIter>::Iter,
}
impl ParallelIterator for InstanceIter {
    type Item = classes::Instance;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}
impl IndexedParallelIterator for InstanceIter {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.Name.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        // Drain every item, and then the vector only needs to free its buffer.
        self.Name
            .map(|name| classes::Instance {
                Name: name,
                ..Default::default()
            })
            .with_producer(callback)
    }
}

#[derive(Default)]
struct InstancePropertyChunks {
    Name: Option<PropertyChunk<String>>,
}
impl InstancePropertyChunks {
    fn try_push(&mut self, prop_chunk: InstancePropertyChunk) -> Result<(), DuplicateProp> {
        match prop_chunk {
            InstancePropertyChunk::Name(property_chunk) => {
                err_if_some(self.Name.replace(property_chunk))
            }
        }
    }
}
impl IntoIter for InstancePropertyChunks {
    type Item = classes::Instance;
    type Iter = InstanceIter;
    fn into_par_iter(self) -> Self::Iter {
        Self::Iter {
            Name: self.Name.unwrap().state.into_par_iter(),
        }
    }
}

struct BasePartIter {
    superclass: InstanceIter,
    CFrame: <<CFrame as IntoPropertyChunkState>::State as IntoIter>::Iter,
}
impl ParallelIterator for BasePartIter {
    type Item = classes::BasePart;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}
impl IndexedParallelIterator for BasePartIter {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.superclass.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        // Drain every item, and then the vector only needs to free its buffer.
        self.superclass
            .zip_eq(self.CFrame)
            .map(|(superclass, cframe)| classes::BasePart {
                superclass: classes::PVInstance {
                    superclass,
                    ..Default::default()
                },
                CFrame: cframe,
                ..Default::default()
            })
            .with_producer(callback)
    }
}

#[derive(Default)]
struct BasePartPropertyChunks {
    superclass: InstancePropertyChunks,
    CFrame: Option<PropertyChunk<CFrame>>,
}
impl BasePartPropertyChunks {
    fn try_push(&mut self, prop_chunk: BasePartPropertyChunk) -> Result<(), DuplicateProp> {
        match prop_chunk {
            BasePartPropertyChunk::Superclass(instance_property_chunk) => {
                self.superclass.try_push(instance_property_chunk)
            }
            BasePartPropertyChunk::CFrame(property_chunk) => {
                err_if_some(self.CFrame.replace(property_chunk))
            }
        }
    }
}
impl IntoIter for BasePartPropertyChunks {
    type Item = classes::BasePart;
    type Iter = BasePartIter;
    fn into_par_iter(self) -> Self::Iter {
        let superclass = InstanceIter {
            Name: self.superclass.Name.unwrap().state.into_par_iter(),
        };
        Self::Iter {
            superclass,
            // TODO: use serialize code to generate a PropertyChunk<CFrame> full of default values for this type
            CFrame: self.CFrame.unwrap().state.into_par_iter(),
        }
    }
}

#[derive(Default)]
struct PartPropertyChunks {
    superclass: BasePartPropertyChunks,
    Shape: Option<PropertyChunk<enums::FormFactor>>,
}
impl PartPropertyChunks {
    fn try_push(&mut self, prop_chunk: PartPropertyChunk) -> Result<(), DuplicateProp> {
        match prop_chunk {
            PartPropertyChunk::Superclass(base_part_property_chunk) => {
                self.superclass.try_push(base_part_property_chunk)
            }
            PartPropertyChunk::Shape(property_chunk) => {
                err_if_some(self.Shape.replace(property_chunk))
            }
        }
    }
}

#[derive(Default)]
struct WedgePartPropertyChunks {
    superclass: BasePartPropertyChunks,
}
impl WedgePartPropertyChunks {
    fn try_push(&mut self, prop_chunk: WedgePartPropertyChunk) -> Result<(), DuplicateProp> {
        match prop_chunk {
            WedgePartPropertyChunk::Superclass(base_part_property_chunk) => {
                self.superclass.try_push(base_part_property_chunk)
            }
        }
    }
}

enum InstancePropertyChunk {
    Name(PropertyChunk<String>),
}
impl InstancePropertyChunk {
    fn new(
        data: Vec<u8>,
        len: usize,
        prop_name: &str,
        class_type: ClassType,
    ) -> Result<Self, InnerError> {
        Ok(match prop_name {
            "Name" => Self::Name(PropertyChunk::new(data, len)?),
            other => {
                return Err(InnerError::UnknownProperty {
                    class_type,
                    property_name: other.to_owned(),
                });
            }
        })
    }
}

enum BasePartPropertyChunk {
    Superclass(InstancePropertyChunk),
    CFrame(PropertyChunk<CFrame>),
}
impl BasePartPropertyChunk {
    fn new(
        data: Vec<u8>,
        len: usize,
        prop_name: &str,
        class_type: ClassType,
    ) -> Result<Self, InnerError> {
        Ok(match prop_name {
            "CFrame" => Self::CFrame(PropertyChunk::new(data, len)?),
            other => Self::Superclass(InstancePropertyChunk::new(data, len, other, class_type)?),
        })
    }
}

enum PartPropertyChunk {
    Superclass(BasePartPropertyChunk),
    Shape(PropertyChunk<enums::FormFactor>),
}
impl PartPropertyChunk {
    fn new(
        data: Vec<u8>,
        len: usize,
        prop_name: &str,
        class_type: ClassType,
    ) -> Result<Self, InnerError> {
        Ok(match prop_name {
            "shape" => Self::Shape(PropertyChunk::new(data, len)?),
            other => Self::Superclass(BasePartPropertyChunk::new(data, len, other, class_type)?),
        })
    }
}
enum WedgePartPropertyChunk {
    Superclass(BasePartPropertyChunk),
}
impl WedgePartPropertyChunk {
    fn new(
        data: Vec<u8>,
        len: usize,
        prop_name: &str,
        class_type: ClassType,
    ) -> Result<Self, InnerError> {
        Ok(match prop_name {
            other => Self::Superclass(BasePartPropertyChunk::new(data, len, other, class_type)?),
        })
    }
}

// === HAND WRITTEN ===

struct TypeInfo {
    class_type: ClassType,
    referents: Vec<i32>,
}

pub(super) struct ParallelState {
    header: FileHeader,
    /// The SharedStrings contained in the file, if any, in the order that they
    /// appear in the file.
    shared_strings: Vec<SharedString>,
    /// All of the property data is collected here.
    prop: ClassPropertyChunks,
    prnt: Option<Vec<u8>>,
    end: Option<Vec<u8>>,
}
impl ParallelState {
    pub(super) fn new(mut input: &[u8]) -> Result<Self, InnerError> {
        let header = FileHeader::decode(&mut input)?;
        let chunks = ChunkSlices::new(&header, input)?;

        chunks.meta;
        chunks.sstr;

        // All of the instance types described by the file so far.
        // The index is the type_id.
        let type_infos = HashMap::with_capacity(header.num_types() as usize);

        chunks.inst;
        for prop_chunk in chunks.prop {
            let data = prop_chunk.decode()?;
            decode_prop_chunk(data, &type_infos)?;
        }

        chunks.prnt;
        chunks.end;

        Ok(ParallelState {
            header,
            shared_strings: Vec::new(),
            prop: ClassPropertyChunks::default(),
            prnt: None,
            end: None,
        })
    }
    pub(super) fn finish(self) -> Result<StrongDom, InnerError> {
        todo!()
    }
}

fn decode_prop_chunk(
    data: Vec<u8>,
    type_infos: &HashMap<u32, TypeInfo>,
) -> Result<ClassPropertyChunk, InnerError> {
    let mut chunk = data.as_slice();
    let type_id = chunk.read_le_u32()?;
    let prop_name = chunk.read_string()?;

    let type_info = type_infos
        .get(&type_id)
        .ok_or(InnerError::InvalidTypeId { type_id })?;

    let class_property = ClassPropertyChunk::new(
        data,
        type_info.referents.len(),
        prop_name,
        type_info.class_type,
    )?;

    Ok(class_property)
}

/// Transform a prop chunk into a form which can be deserialized in parallel
trait IntoPropertyChunkState: Sized {
    /// Many properties cannot be decoded in parallel, such as Name.
    /// This is intended to contain the data that must be decoded sequentially required to reach a parallel decoding implementation.
    type State: IntoIter<Item = Self, Iter: IndexedParallelIterator>;
    type Error;
    /// All fallible operations must happen ahead of iteration
    fn into_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::Error>;
}

struct PropertyChunk<T: IntoPropertyChunkState> {
    state: T::State,
}
impl<T: IntoPropertyChunkState> PropertyChunk<T> {
    fn new(chunk: Vec<u8>, len: usize) -> Result<Self, T::Error> {
        let state = T::into_state(chunk, len)?;
        Ok(Self { state })
    }
}

impl IntoPropertyChunkState for String {
    type State = Vec<String>;
    type Error = io::Error;

    fn into_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::Error> {
        // The length of every string must be determined.  May as well store slice windows.
        let mut values = Vec::with_capacity(len);

        let mut slice = chunk.as_slice();
        // TODO: use PR #592 RbxReadInterleaved Trait
        for _ in 0..len {
            values.push(slice.read_string()?.to_owned());
        }

        Ok(values)
    }
}

impl IntoPropertyChunkState for CFrame {
    type State = Vec<CFrame>;
    type Error = io::Error;

    fn into_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::Error> {
        let mut rotations = Vec::with_capacity(len);

        let mut slice = chunk.as_slice();

        for _ in 0..len {
            let id = slice.read_u8()?;
            if id == 0 {
                rotations.push(Matrix3::new(
                    Vector3::new(
                        slice.read_le_f32()?,
                        slice.read_le_f32()?,
                        slice.read_le_f32()?,
                    ),
                    Vector3::new(
                        slice.read_le_f32()?,
                        slice.read_le_f32()?,
                        slice.read_le_f32()?,
                    ),
                    Vector3::new(
                        slice.read_le_f32()?,
                        slice.read_le_f32()?,
                        slice.read_le_f32()?,
                    ),
                ));
            } else if let Ok(basic_rotation) = Matrix3::from_basic_rotation_id(id) {
                rotations.push(basic_rotation);
            } else {
                todo!("Errors");
            }
        }

        let x = slice.read_interleaved_f32_array(len)?;
        let y = slice.read_interleaved_f32_array(len)?;
        let z = slice.read_interleaved_f32_array(len)?;

        let values = x
            .zip(y)
            .zip(z)
            .map(|((x, y), z)| Vector3::new(x, y, z))
            .zip(rotations)
            .map(|(position, rotation)| CFrame::new(position, rotation));

        Ok(values.collect())
    }
}

impl IntoPropertyChunkState for enums::FormFactor {
    type State = Vec<enums::FormFactor>;
    type Error = io::Error;

    fn into_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::Error> {
        // TODO: use PR #592 RbxReadInterleaved Trait
        Ok(chunk
            .as_slice()
            .read_interleaved_u32_array(len)?
            .map(|value| {
                // TODO: TryFrom<u32>
                Some(match value {
                    0 => enums::FormFactor::Symmetric,
                    1 => enums::FormFactor::Brick,
                    2 => enums::FormFactor::Plate,
                    3 => enums::FormFactor::Custom,
                    _ => return None,
                })
            })
            .collect::<Option<Vec<_>>>()
            .expect("Invalid Enum"))
    }
}
