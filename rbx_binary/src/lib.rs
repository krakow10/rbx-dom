/*!
TODO: feature-dependent doctests
*/

#![deny(missing_docs)]

mod chunk;
mod core;
mod header;
mod types;

pub use chunk::CompressionType;

#[cfg(feature = "weak")]
mod weak;
#[cfg(feature = "weak")]
pub use weak::*;

#[cfg(feature = "strong")]
mod strong;
#[cfg(feature = "strong")]
pub use strong::*;

#[cfg(any(test, feature = "unstable_text_format"))]
mod text_deserializer;
/// An unstable textual format that can be used to debug binary models.
#[cfg(feature = "unstable_text_format")]
pub mod text_format {
    pub use crate::text_deserializer::*;
}
