use std::{
    fmt,
    io::{self, Read, Write},
    str,
};

use crate::{
    core::RbxReadExt, core::RbxWriteExt, decompression_host::DecompressionHost, serializer::CompressionType,
};

const ZSTD_MAGIC_NUMBER: &[u8] = &[0x28, 0xb5, 0x2f, 0xfd];
const LZ4_MAGIC_NUMBER: &[u8] = &[0x04, 0x22, 0x4d, 0x18];

/// Represents one chunk from a binary model file.
#[derive(Debug)]
pub struct Chunk<'file> {
    pub name: [u8; 4],
    pub data: &'file [u8],
}

impl<'file> Chunk<'file> {
    /// Reads and decodes a `Chunk` from the given reader.
    pub fn decode_with(host: &'file DecompressionHost, slice: &mut &'file [u8]) -> io::Result<Self> {
        let header = decode_chunk_header(slice)?;

        log::trace!("{}", header);

        let data = if header.compressed_len == 0 {
            log::trace!("No compression");
            &slice[..header.len as usize]
        } else {
            let compressed_data = &slice[..header.compressed_len as usize];

            match compressed_data.get(0..4) {
                Some(ZSTD_MAGIC_NUMBER) => {
                    log::trace!("ZSTD compression");
                    let buffer = host.alloc_buffer(header.len as usize);
                    zstd::bulk::decompress_to_buffer(&compressed_data, buffer)?;
                    buffer
                }
                Some(LZ4_MAGIC_NUMBER) => {
                    log::trace!("LZ4 compression");
                    let buffer = host.alloc_buffer(header.len as usize);
                    lz4::block::decompress_to_buffer(
                        &compressed_data,
                        Some(header.len as i32),
                        buffer,
                    )?;
                    buffer
                }
                _ => {
                    panic!("unknown compression format");
                }
            }
        };

        assert_eq!(data.len(), header.len as usize);

        Ok(Chunk {
            name: header.name,
            data,
        })
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

    /// Consume the chunk and write it to the given writer.
    pub fn dump<W: Write>(self, mut writer: W) -> io::Result<()> {
        writer.write_all(self.chunk_name)?;

        match self.compression {
            CompressionType::Lz4 => {
                let compressed = lz4::block::compress(&self.buffer, None, false)?;

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

#[derive(Debug)]
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

fn decode_chunk_header(source: &mut &[u8]) -> io::Result<ChunkHeader> {
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
