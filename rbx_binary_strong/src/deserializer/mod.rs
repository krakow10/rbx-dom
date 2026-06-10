mod error;
mod state;

use rbx_dom_strong::StrongDom;

pub use error::Error;

use state::ParallelState;

/// Describes the strategy that rbx_binary_strong should use when deserializing
/// properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DecodePropertyBehavior {
    /// Ignores properties that aren't known by rbx_binary_strong.
    ///
    /// The default and safest option. With this set, properties that are newer
    /// than the current version of the strong types will be ignored.
    IgnoreUnknown,

    /// Returns an error if any properties are found that aren't known by
    /// rbx_binary_strong.
    ErrorOnUnknown,
}

pub struct Deserializer {}

impl Deserializer {
    /// Create a new `Deserializer` with the default settings.
    pub fn new() -> Self {
        Self {}
    }

    /// Deserialize a Roblox binary model or place from the given stream using
    /// this deserializer.
    pub fn deserialize(&self, data: &[u8]) -> Result<StrongDom, Error> {
        // Parallelize per chunk.
        // This decodes non-parallelizable properties.
        let parallel = ParallelState::new(data)?;

        // Parallelize per instance
        // This decodes parallelizable properties.
        Ok(parallel.finish()?)
    }
}
