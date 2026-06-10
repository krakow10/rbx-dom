use std::io;

use rbx_dom_strong::{classes, enums};
use rbx_types::{CFrame, Matrix3, SharedString, Vector3};

use rbx_binary_core::{
    chunk::{ChunkIter, CompressedChunk},
    core::{RbxReadExt, RbxReadInterleaved},
    header::FileHeader,
};

use super::error::{Error, InnerError};

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

// This will be populated with chunk decoders by decode_prop_chunk
#[derive(Default)]
struct DeserializerClassPropertyChunks {
    Part: PartPropertyChunks,
    WedgePart: WedgePartPropertyChunks,
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

enum BasePartPropertyChunk {
    Superclass(InstancePropertyChunk),
    CFrame(PropertyChunk<CFrame>),
}

enum PartPropertyChunk {
    Superclass(BasePartPropertyChunk),
    Shape(PropertyChunk<enums::FormFactor>),
}
enum WedgePartPropertyChunk {
    Superclass(BasePartPropertyChunk),
}

enum ClassPropertyChunk {
    PartPropertyChunk(PartPropertyChunk),
    WedgePartPropertyChunk(WedgePartPropertyChunk),
}

impl DeserializerClassPropertyChunks {
    fn try_push(&mut self, prop_chunk: PropChunk) -> Result<(), DuplicateProp> {
        match prop_chunk.class_property {
            ClassPropertyChunk::PartPropertyChunk(part_property_chunk) => {
                self.Part.try_push(part_property_chunk)
            }
            ClassPropertyChunk::WedgePartPropertyChunk(wedge_part_property_chunk) => {
                self.WedgePart.try_push(wedge_part_property_chunk)
            }
        }
    }
}

// === HAND WRITTEN ===

pub(super) struct DeserializerState {
    header: FileHeader,
    /// The SharedStrings contained in the file, if any, in the order that they
    /// appear in the file.
    shared_strings: Vec<SharedString>,
    inst: Option<Vec<u8>>,
    prop: DeserializerClassPropertyChunks,
    prnt: Option<Vec<u8>>,
    end: Option<Vec<u8>>,
}
impl DeserializerState {
    pub(super) fn new(mut input: &[u8]) -> Result<(Self, ChunkIter<'_>), InnerError> {
        let header = FileHeader::decode(&mut input)?;

        Ok((
            DeserializerState {
                header,
                shared_strings: Vec::new(),
                inst: None,
                prop: DeserializerClassPropertyChunks::default(),
                prnt: None,
                end: None,
            },
            ChunkIter::new(input),
        ))
    }
    pub(super) fn try_push(&mut self, chunk: Chunk) -> Result<(), InnerError> {
        Ok(match chunk {
            Chunk::Meta(meta_chunk) => todo!(),
            Chunk::Sstr(sstr_chunk) => todo!(),
            Chunk::Inst(inst_chunk) => todo!(),
            Chunk::Prop(prop_chunk) => self.prop.try_push(prop_chunk)?,
            Chunk::Prnt(prnt_chunk) => todo!(),
            Chunk::End(end_chunk) => todo!(),
        })
    }
}

struct MetaChunk {}
struct SstrChunk {}
struct InstChunk {}
struct PropChunk {
    class_property: ClassPropertyChunk,
}
struct PrntChunk {}
struct EndChunk {}

pub(super) enum Chunk {
    Meta(MetaChunk),
    Sstr(SstrChunk),
    Inst(InstChunk),
    Prop(PropChunk),
    Prnt(PrntChunk),
    End(EndChunk),
}
impl Chunk {
    pub(super) fn decode(compressed_chunk: io::Result<CompressedChunk<'_>>) -> Result<Self, Error> {
        let compressed_chunk = compressed_chunk.map_err(InnerError::from)?;
        let chunk = compressed_chunk.decode().map_err(InnerError::from)?;
        match &chunk.name {
            b"META" => MetaChunk::new(chunk.data)?,
            b"SSTR" => SstrChunk::new(chunk.data)?,
            b"INST" => InstChunk::new(chunk.data)?,
            b"PROP" => PropChunk::new(chunk.data)?,
            b"PRNT" => PrntChunk::new(chunk.data)?,
            b"END\0" => EndChunk::new(chunk.data)?,
        }
    }
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
