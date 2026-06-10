mod error;
mod state;

use std::str;

#[cfg(feature = "rayon")]
use rayon::iter::{ParallelBridge, ParallelIterator};

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
        let (mut deserializer, chunks) = DeserializerState::new(data)?;

        let it = chunks;
        #[cfg(feature = "rayon")]
        let it = it.par_bridge();

        // Parallelize per chunk.
        // This decodes non-parallelizable properties.
        for chunk in it.map(decode_chunk) {
            deserializer.receive_chunk(chunk)?;
        }

        // Parallelize per instance
        // This decodes parallelizable properties.
        Ok(deserializer.finish())
    }
}
