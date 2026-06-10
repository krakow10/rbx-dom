mod error;
mod state;

use std::str;

use rbx_dom_strong::StrongDom;

use state::DeserializerState;

pub use error::Error;

pub struct Deserializer {}

impl Deserializer {
    /// Create a new `Deserializer` with the default settings.
    pub fn new() -> Self {
        Self {}
    }

    /// Deserialize a Roblox binary model or place from the given stream using
    /// this deserializer.
    pub fn deserialize(&self, data: &[u8]) -> Result<StrongDom, Error> {
        let mut deserializer = DeserializerState::new(data)?;

        loop {
            let chunk = deserializer.next_chunk()?;

            match &chunk.name {
                b"META" => deserializer.decode_meta_chunk(&chunk.data)?,
                b"SSTR" => deserializer.decode_sstr_chunk(&chunk.data)?,
                b"INST" => deserializer.decode_inst_chunk(&chunk.data)?,
                b"PROP" => deserializer.decode_prop_chunk(&chunk.data)?,
                b"PRNT" => deserializer.decode_prnt_chunk(&chunk.data)?,
                b"END\0" => {
                    deserializer.decode_end_chunk(&chunk.data)?;
                    break;
                }
                _ => match str::from_utf8(&chunk.name) {
                    Ok(name) => log::info!("Unknown binary chunk name {name}"),
                    Err(_) => log::info!("Unknown binary chunk name {:?}", chunk.name),
                },
            }
        }

        Ok(deserializer.finish())
    }
}
