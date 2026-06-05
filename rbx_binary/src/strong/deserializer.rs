use std::io;

use rbx_dom_strong::enums;
use rbx_types::CFrame;

use crate::core::RbxReadExt;

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

trait DeserializeProperty {
    type Property;
    type PropertyError;
    fn decode_one_property(
        &self,
        instance_index: usize,
    ) -> Result<Self::Property, Self::PropertyError>;
}

trait DeserializeState {
    /// Many properties cannot be decoded in parallel, such as Name.
    /// This is intended to contain the data that must be decoded sequentially required to reach a parallel decoding implementation.
    type State: DeserializeProperty<Property = Self>;
    type StateError;
    fn decode_state(chunk: Vec<u8>, len: usize) -> Result<Self::State, Self::StateError>;
}

struct PropertyChunk<T: DeserializeState> {
    state: T::State,
}

/// The index is out of range.
struct IndexError;

struct StringState {
    strings: Vec<String>,
}

impl DeserializeState for String {
    type State = StringState;
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

impl DeserializeProperty for StringState {
    type Property = String;
    type PropertyError = IndexError;

    fn decode_one_property(
        &self,
        instance_index: usize,
    ) -> Result<Self::Property, Self::PropertyError> {
        self.strings.get(instance_index).cloned().ok_or(IndexError)
    }
}
