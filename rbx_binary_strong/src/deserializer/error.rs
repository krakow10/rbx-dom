use std::io;

use thiserror::Error;

use rbx_binary_core::header::HeaderError;

use super::state::ClassType;

/// Represents an error that occurred during deserialization.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    source: Box<InnerError>,
}

impl From<InnerError> for Error {
    fn from(inner: InnerError) -> Self {
        Self {
            source: Box::new(inner),
        }
    }
}
impl From<HeaderError> for InnerError {
    fn from(error: HeaderError) -> Self {
        match error {
            HeaderError::Io { source } => InnerError::Io { source },
            HeaderError::BadHeader => InnerError::BadHeader,
            HeaderError::UnknownFileVersion { version } => {
                InnerError::UnknownFileVersion { version }
            }
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum InnerError {
    #[error(transparent)]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("Invalid file header")]
    BadHeader,

    #[error("Unknown file version {version}. Known versions are: 0")]
    UnknownFileVersion { version: u16 },

    #[error("Unknown version {version} for chunk {chunk_name}")]
    UnknownChunkVersion {
        chunk_name: &'static str,
        version: u32,
    },

    #[error("Unexpected chunk {actual}, expected {expected}")]
    UnexpectedChunk {
        expected: &'static str,
        actual: String,
    },

    #[error("Property {class_type:?}.{property_name} is unknown")]
    UnknownProperty {
        class_type: ClassType,
        property_name: String,
    },

    #[error("File referred to type ID {type_id}, which was not declared")]
    InvalidTypeId { type_id: u32 },

    #[error(
        "Invalid property data: CFrame property {type_name}.{prop_name} had an invalid rotation ID {id:02x}"
    )]
    BadRotationId {
        type_name: String,
        prop_name: String,
        id: u8,
    },

    #[error(
        "Expected type id for {expected_type_name} ({expected_type_id:02x}) when reading OptionalCFrame; got {actual_type_id:02x}"
    )]
    BadOptionalCFrameFormat {
        expected_type_name: String,
        expected_type_id: u8,
        actual_type_id: u8,
    },

    #[error("'Content' type {0} is not implemented")]
    BadContentType(i32),

    #[error("'PhysicalProperties' discriminator {0:b} is not supported")]
    BadPhysicalPropertiesType(u8),

    #[error("Malformed file: Referent {referent} has already appeared in the file")]
    DuplicateReferent { referent: i32 },

    #[error("Malformed file: INST Chunk with type_id {type_id} has already appeared in the file")]
    DuplicateInstChunk { type_id: u32 },

    #[error(
        "Malformed file: PROP Chunk with prop_name {prop_name} has already appeared in the file"
    )]
    DuplicatePropChunk { prop_name: String },
}
