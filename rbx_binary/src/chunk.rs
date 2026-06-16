use std::{
    fmt,
    io::{self, Read, Write},
    str,
};

use crate::{
    core::{unexpected_eof, RbxReadExt, RbxWriteExt},
    serializer::CompressionType,
};

const ZSTD_MAGIC_NUMBER: &[u8] = &[0x28, 0xb5, 0x2f, 0xfd];

pub struct UnexpectedChunk {
    pub expected: &'static str,
    pub actual: String,
}

/// Represents one chunk from a binary model file.
#[derive(Debug)]
pub struct Chunk {
    pub name: [u8; 4],
    pub data: Vec<u8>,
}

impl Chunk {
    // returns expected_chunk
    pub fn once(self, expected: &'static str) -> Result<Box<[u8]>, UnexpectedChunk> {
        if self.name != expected.as_bytes() {
            return Err(UnexpectedChunk {
                expected,
                actual: core::str::from_utf8(&self.name)
                    .unwrap_or("UTF8Error")
                    .to_owned(),
            });
        }
        Ok(self.data.into_boxed_slice())
    }
}

/// Holds a chunk that is currently being written.
///
/// This type intended to be written into via io::Write and then dumped into the
/// output stream all at once. It handles compression and chunk header output
/// automatically.
#[must_use]
pub struct ChunkBuilder {
    chunk_name: &'static [u8],
    compression: CompressionType,
    buffer: Vec<u8>,
}

impl ChunkBuilder {
    /// Creates a new `ChunkBuilder` with the given name and compression
    /// setting.
    pub fn new(chunk_name: &'static [u8], compression: CompressionType) -> Self {
        ChunkBuilder {
            chunk_name,
            compression,
            buffer: Vec::new(),
        }
    }

    /// Reserve bytes and use a closure to initialize them.
    pub fn initialize_bytes_with(&mut self, len: usize, initialize_bytes: impl FnOnce(&mut [u8])) {
        let current_len = self.buffer.len();
        self.buffer.extend(core::iter::repeat_n(0, len));
        initialize_bytes(&mut self.buffer[current_len..]);
    }

    /// Consume the chunk and write it to the given writer.
    pub fn dump<W: Write>(self, mut writer: W) -> io::Result<()> {
        writer.write_all(self.chunk_name)?;

        match self.compression {
            CompressionType::Lz4 => {
                let compressed = lz4_flex::block::compress(&self.buffer);

                writer.write_le_u32(compressed.len() as u32)?;
                writer.write_le_u32(self.buffer.len() as u32)?;
                writer.write_le_u32(0)?;

                writer.write_all(&compressed)?;
            }
            CompressionType::None => {
                writer.write_le_u32(0)?;
                writer.write_le_u32(self.buffer.len() as u32)?;
                writer.write_le_u32(0)?;

                writer.write_all(&self.buffer)?;
            }
            CompressionType::Zstd => {
                let compressed = zstd::bulk::compress(&self.buffer, 0)?;

                writer.write_le_u32(compressed.len() as u32)?;
                writer.write_le_u32(self.buffer.len() as u32)?;
                writer.write_le_u32(0)?;

                // ZSTD includes the magic number when compressing so we don't
                // have to write it manually
                writer.write_all(&compressed)?;
            }
        }

        Ok(())
    }
}

impl Write for ChunkBuilder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ChunkHeader {
    /// 4-byte short name for the chunk, like "INST" or "PRNT"
    name: [u8; 4],

    /// The length of the chunk's compressed data. For uncompressed chunks, this
    /// is always zero.
    compressed_len: u32,

    /// The length that the chunk's data will have when decompressed. For
    /// uncompressed chunks, this is their length as-is.
    len: u32,

    /// Always zero.
    reserved: u32,
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        let name = if let Ok(name) = str::from_utf8(&self.name) {
            name.to_owned()
        } else {
            format!("{:?}", self.name)
        };

        write!(
            output,
            "Chunk \"{}\" (compressed: {}, len: {}, reserved: {})",
            name, self.compressed_len, self.len, self.reserved
        )
    }
}

fn decode_chunk_header<R: Read>(source: &mut R) -> io::Result<ChunkHeader> {
    let mut name = [0; 4];
    source.read_exact(&mut name)?;

    let compressed_len = source.read_le_u32()?;
    let len = source.read_le_u32()?;
    let reserved = source.read_le_u32()?;

    if reserved != 0 {
        panic!(
            "Chunk reserved space was not zero, it was {}. This chunk may be malformed.",
            reserved
        );
    }

    Ok(ChunkHeader {
        name,
        compressed_len,
        len,
        reserved,
    })
}

#[derive(Clone, Debug)]
pub struct CompressedChunk {
    header: ChunkHeader,
    /// The chunk payload data
    data: Vec<u8>,
}
impl CompressedChunk {
    const fn name(&self) -> [u8; 4] {
        self.header.name
    }
    pub fn decode(self) -> io::Result<Chunk> {
        let data = if self.header.compressed_len == 0 {
            log::trace!("No compression");
            self.data
        } else {
            if self.data.get(0..4) == Some(ZSTD_MAGIC_NUMBER) {
                log::trace!("ZSTD compression");
                zstd::bulk::decompress(&self.data, self.header.len as usize)?
            } else {
                log::trace!("LZ4 compression");
                lz4_flex::block::decompress(&self.data, self.header.len as usize)
                    .map_err(io::Error::other)?
            }
        };

        assert_eq!(data.len(), self.header.len as usize);

        Ok(Chunk {
            name: self.header.name,
            data,
        })
    }
}

struct ChunkParser<R> {
    reader: R,
}
impl<R> ChunkParser<R> {
    const fn new(reader: R) -> Self {
        Self { reader }
    }
    fn next_chunk(&mut self) -> io::Result<CompressedChunk>
    where
        R: Read,
    {
        let header = decode_chunk_header(&mut self.reader)?;

        let len = if header.compressed_len == 0 {
            header.len
        } else {
            header.compressed_len
        };

        let mut data = Vec::with_capacity(len as usize);
        (&mut self.reader).take(len as u64).read_to_end(&mut data)?;

        Ok(CompressedChunk { header, data })
    }
}

pub fn parse_chunks<R: Read>(reader: R) -> io::Result<Vec<CompressedChunk>> {
    let mut chunk_parser = ChunkParser::new(reader);

    let mut chunks = Vec::new();
    loop {
        let chunk = chunk_parser.next_chunk()?;
        let is_end = chunk.name() == *b"END\0";
        chunks.push(chunk);
        if is_end {
            break;
        }
    }

    Ok(chunks)
}

pub struct ChunkCategorizer<I> {
    chunks: I,
}

impl<I: Iterator<Item = io::Result<Chunk>>> ChunkCategorizer<I> {
    pub fn new<C: IntoIterator<IntoIter = I, Item = io::Result<Chunk>>>(chunks: C) -> Self {
        Self {
            chunks: chunks.into_iter(),
        }
    }
    pub fn try_next(&mut self) -> io::Result<Chunk> {
        self.chunks.next().ok_or_else(unexpected_eof)?
    }
    // returns (Option<expected_chunk>, next_chunk)
    pub fn optional(
        &mut self,
        chunk: Chunk,
        expected: [u8; 4],
    ) -> io::Result<(Option<Box<[u8]>>, Chunk)> {
        if chunk.name == expected {
            let next_chunk = self.try_next()?;
            Ok((Some(chunk.data.into_boxed_slice()), next_chunk))
        } else {
            Ok((None, chunk))
        }
    }
    // populates `chunks` and returns next_chunk
    pub fn repeated(
        &mut self,
        mut chunk: Chunk,
        expected: [u8; 4],
        chunks_out: &mut Vec<Box<[u8]>>,
    ) -> io::Result<Chunk> {
        while chunk.name == expected {
            chunks_out.push(chunk.data.into_boxed_slice());
            chunk = self.try_next()?;
        }
        Ok(chunk)
    }
}
