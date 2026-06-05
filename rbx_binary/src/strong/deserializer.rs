use std::io;

use rbx_dom_strong::enums;
use rbx_types::{CFrame, Matrix3, Vector3};

use crate::core::RbxReadExt;

#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;

// === GENERATED ===

struct DeserializerClassPropertyChunks {
    Part: Option<PartPropertyChunks>,
    WedgePart: Option<WedgePartPropertyChunks>,
}

struct InstancePropertyChunks {
    Name: Option<PropertyChunk<String>>,
}
struct BasePartPropertyChunks {
    superclass: InstancePropertyChunks,
    CFrame: Option<PropertyChunk<CFrame>>,
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
trait DeserializeState: Sized {
    /// Many properties cannot be decoded in parallel, such as Name.
    /// This is intended to contain the data that must be decoded sequentially required to reach a parallel decoding implementation.
    #[cfg(not(feature = "rayon"))]
    type State: IntoIterator<Item = Self>;
    #[cfg(feature = "rayon")]
    type State: IntoParallelIterator<Item = Self>;
    type Error;
    /// All fallible operations must happen ahead of iteration
    fn decode_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::Error>;
}

struct PropertyChunk<T: DeserializeState> {
    state: T::State,
}

impl DeserializeState for String {
    type State = Vec<String>;
    type Error = io::Error;

    fn decode_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::Error> {
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

impl DeserializeState for CFrame {
    type State = Vec<CFrame>;
    type Error = io::Error;

    fn decode_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::Error> {
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
