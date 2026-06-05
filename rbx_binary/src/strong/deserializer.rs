use std::io;

use rbx_dom_strong::enums;
use rbx_types::CFrame;

use crate::core::RbxReadExt;

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    type State: IntoIterator<Item = Result<Self, Self::PropertyError>>;
    #[cfg(feature = "rayon")]
    type State: IntoParallelIterator<Item = Result<Self, Self::PropertyError>>;
    type StateError;
    type PropertyError;
    fn decode_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::StateError>;
}

struct PropertyChunk<T: DeserializeState> {
    state: T::State,
}

struct StringState {
    strings: Vec<String>,
}

impl DeserializeState for String {
    type State = StringState;
    type PropertyError = ();
    type StateError = io::Error;

    fn decode_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::StateError> {
        // The length of every string must be determined.  May as well store slice windows.
        let mut strings = Vec::with_capacity(len);

        let mut slice = chunk.as_slice();
        // TODO: use PR #592 RbxReadInterleaved Trait
        for _ in 0..len {
            strings.push(slice.read_string()?);
        }

        Ok(StringState { strings })
    }
}

impl IntoIterator for StringState {
    type Item = Result<String, ()>;
    type IntoIter =
        core::iter::Map<std::vec::IntoIter<String>, fn(std::string::String) -> Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.strings.into_iter().map(Ok)
    }
}

#[cfg(feature = "rayon")]
impl IntoParallelIterator for StringState {
    type Item = Result<String, ()>;
    type Iter =
        rayon::iter::Map<rayon::vec::IntoIter<String>, fn(std::string::String) -> Self::Item>;

    fn into_par_iter(self) -> Self::Iter {
        self.strings.into_par_iter().map(Ok)
    }
}
