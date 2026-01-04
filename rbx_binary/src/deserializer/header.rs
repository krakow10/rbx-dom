use std::io::Read;

use crate::core::{ReadSlice, FILE_MAGIC_HEADER, FILE_SIGNATURE, FILE_VERSION};

use super::error::InnerError;

/// All the information contained in the header before any chunks are read from
/// the file.
pub(crate) struct FileHeader {
    /// The number of instance types (represented for us as `TypeInfo`) that are
    /// in this file. Generally useful to pre-size some containers before
    /// reading the file.
    pub(crate) num_types: u32,

    /// The total number of instances described by this file.
    pub(crate) num_instances: u32,
}

/// The raw header as it appears in the file.
/// Used to calculate the length of data to read.
#[derive(zerocopy::FromBytes, zerocopy::Immutable, zerocopy::KnownLayout)]
#[repr(C, packed)]
struct RawFileHeader {
    magic_header: [u8; 8],
    signature: [u8; 6],
    version: u16,
    num_types: u32,
    num_instances: u32,
    reserved: [u8; 8],
}

impl FileHeader {
    pub(crate) fn decode<R: Read>(mut source: R) -> Result<Self, InnerError> {
        // Read a buffer the same length as the header
        let mut data = [0; size_of::<RawFileHeader>()];
        source.read_exact(&mut data)?;

        // Read the fields off of a slice
        let mut slice: &[u8] = &data;

        let header: &RawFileHeader = slice.read_ref()?;

        if header.magic_header != FILE_MAGIC_HEADER {
            return Err(InnerError::BadHeader);
        }

        if header.signature != FILE_SIGNATURE {
            return Err(InnerError::BadHeader);
        }

        if header.version != FILE_VERSION {
            return Err(InnerError::UnknownFileVersion {
                version: header.version,
            });
        }

        if header.reserved != [0; 8] {
            return Err(InnerError::BadHeader);
        }

        Ok(Self {
            num_types: header.num_types,
            num_instances: header.num_instances,
        })
    }
}
