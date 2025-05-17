use std::io::Read;

use crate::chunk::Chunks;
use crate::header::{FileHeader, HeaderError};

pub struct DecompressedFile {
    pub(crate) header: FileHeader,
    pub(crate) chunks: Chunks,
}

impl DecompressedFile {
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, HeaderError> {
        let header = FileHeader::decode(&mut reader)?;
        let chunks = Chunks::decode(reader)?;
        Ok(DecompressedFile { header, chunks })
    }
}
