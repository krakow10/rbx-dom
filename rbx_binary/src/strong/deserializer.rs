use std::io;

use rbx_dom_strong::enums;
use rbx_types::CFrame;

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
        let mut strings = Vec::with_capacity(len);

        let mut slice = chunk.as_slice();
        // TODO: use PR #592 RbxReadInterleaved Trait
        for _ in 0..len {
            strings.push(slice.read_string()?);
        }

        Ok(strings)
    }
}
