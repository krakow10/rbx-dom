use std::io;

use crate::{
    chunk::{Chunk, ChunkCategorizer},
    deserializer::{error::InnerError, FileHeader},
};

pub struct Chunks {
    // optional chunks
    pub meta: Option<Box<[u8]>>,
    pub sstr: Option<Box<[u8]>>,
    // repeated chunks
    pub inst: Vec<Box<[u8]>>,
    pub prop: Vec<Box<[u8]>>,
    // once chunks
    pub prnt: Box<[u8]>,
    pub end: Box<[u8]>,
}

impl Chunks {
    pub fn new<
        C: IntoIterator<
            IntoIter: ExactSizeIterator<Item = io::Result<Chunk>>,
            Item = io::Result<Chunk>,
        >,
    >(
        header: &FileHeader,
        chunks: C,
    ) -> Result<Self, InnerError> {
        let chunks = chunks.into_iter();
        let chunks_len = chunks.len();
        let mut chunks = ChunkCategorizer::new(chunks);

        // Expect chunks to appear in a particular order
        let chunk = chunks.try_next()?;

        let (meta, chunk) = chunks.optional(chunk, *b"META")?;
        let (sstr, chunk) = chunks.optional(chunk, *b"SSTR")?;

        let mut inst = Vec::with_capacity(header.num_types as usize);
        let chunk = chunks.repeated(chunk, *b"INST", &mut inst)?;

        // calculate capacity by deduction
        let mut prop = Vec::with_capacity(chunks_len.saturating_sub(
            1 + 1
                + (meta.is_some() as usize)
                + (sstr.is_some() as usize)
                + header.num_types as usize,
        ));
        let chunk = chunks.repeated(chunk, *b"PROP", &mut prop)?;

        let prnt = chunk.once("PRNT")?;

        let chunk = chunks.try_next()?;
        let end = chunk.once("END\0")?;

        Ok(Chunks {
            meta,
            sstr,
            inst,
            prop,
            prnt,
            end,
        })
    }
}
