use std::io;

use rbx_dom_strong::{classes, enums};
use rbx_types::{CFrame, Matrix3, Vector3};

use crate::core::RbxReadExt;

#[cfg(not(feature = "rayon"))]
use core::iter::IntoIterator as IntoIter;
use rayon::iter::plumbing::{bridge, Consumer, ProducerCallback, UnindexedConsumer};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator as IntoIter;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

// === GENERATED ===

// This will be populated with chunk decoders by decode_prop_chunk
struct DeserializerClassPropertyChunks {
    Part: Option<PartPropertyChunks>,
    WedgePart: Option<WedgePartPropertyChunks>,
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
struct InstancePropertyChunks {
    Name: Option<PropertyChunk<String>>,
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

struct BasePartPropertyChunks {
    superclass: InstancePropertyChunks,
    CFrame: Option<PropertyChunk<CFrame>>,
}
impl IntoIter for BasePartPropertyChunks {
    type Item = classes::BasePart;
    type Iter = BasePartIter;
    fn into_par_iter(self) -> Self::Iter {
        Self::Iter {
            superclass: self.superclass.into_par_iter(),
            CFrame: self.CFrame.unwrap().state.into_par_iter(),
        }
    }
}

struct PartPropertyChunks {
    superclass: BasePartPropertyChunks,
    Shape: Option<PropertyChunk<enums::FormFactor>>,
}

struct WedgePartPropertyChunks {
    superclass: BasePartPropertyChunks,
}

// === HAND WRITTEN ===

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

impl IntoPropertyChunkState for String {
    type State = Vec<String>;
    type Error = io::Error;

    fn into_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::Error> {
        // The length of every string must be determined.  May as well store slice windows.
        let mut values = Vec::with_capacity(len);

        let mut slice = chunk.as_slice();
        // TODO: use PR #592 RbxReadInterleaved Trait
        for _ in 0..len {
            values.push(slice.read_string()?);
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
