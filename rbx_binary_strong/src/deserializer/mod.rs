mod error;
mod state;

#[cfg(feature = "rayon")]
use rayon::iter::{ParallelBridge, ParallelIterator};
use rbx_dom_strong::StrongDom;

use state::{Chunk, DeserializerState};

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

        #[cfg(feature = "rayon")]
        let chunks = chunks.par_bridge();

        // Parallelize per chunk.
        // This decodes non-parallelizable properties.
        let chunks = chunks.map(Chunk::decode);

        // rayon cannot fold single threaded.
        #[cfg(feature = "rayon")]
        let chunks: Vec<_> = chunks.collect();

        for chunk in chunks {
            let chunk = chunk?;
            deserializer.try_push(chunk)?;
        }

        // Parallelize per instance
        // This decodes parallelizable properties.
        Ok(deserializer.finish()?)
    }
}
