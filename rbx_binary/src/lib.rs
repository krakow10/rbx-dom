/*!
Implementation of Roblox's binary model (rbxm) and place (rbxl) file formats.

# Examples

## Read a model file

To read a model or place file using rbx_binary's default settings, use
[`from_reader`]. The [`Deserializer`] API exposes additional configuration
options.

```no_run
use std::fs::File;
use std::io::BufReader;

// Using buffered I/O is recommended with rbx_binary
let input = BufReader::new(File::open("MyModel.rbxm")?);

let dom = rbx_binary::from_reader(input)?;

// rbx_binary always returns a DOM with a DataModel at the top level.
// To get to the instances from our file, we need to go one level deeper.

println!("Root instances in file:");
for &referent in dom.root().children() {
    let instance = dom.get_by_ref(referent).unwrap();
    println!("- {}", instance.name);
}

# Ok::<(), Box<dyn std::error::Error>>(())
```

## Write a model file

To write a model or place file using rbx_binary's default settings, use
[`to_writer`]. The [`Serializer`] API exposes additional configuration options.

```no_run
use std::fs::File;
use std::io::BufWriter;

use rbx_dom_weak::{InstanceBuilder, WeakDom};

let dom = WeakDom::new(InstanceBuilder::new("Folder"));

// Using buffered I/O is recommended with rbx_binary
let output = BufWriter::new(File::create("PlainFolder.rbxm")?);
rbx_binary::to_writer(output, &dom, &[dom.root_ref()])?;

# Ok::<(), Box<dyn std::error::Error>>(())
```
*/

#![deny(missing_docs)]

mod chunk;
mod core;
mod deserializer;
mod serializer;
mod types;

#[cfg(any(test, feature = "unstable_text_format"))]
mod text_deserializer;

#[cfg(test)]
mod tests;

use std::io::{Read, Write};

pub use deserializer::DecompressedFile;
use rbx_dom_weak::{types::Ref, WeakDom};

/// An unstable textual format that can be used to debug binary models.
#[cfg(feature = "unstable_text_format")]
pub mod text_format {
    pub use crate::text_deserializer::*;
}

pub use crate::{
    deserializer::{DecodeOptions, Deserializer, Error as DecodeError, StringIntern},
    serializer::{CompressionType, Error as EncodeError, Serializer},
};

/// Deserialize a Roblox binary model or place from a stream.
pub fn from_reader<'dom, 'file, R: Read, S: StringIntern<'file, 'dom>>(
    reader: R,
    options: DecodeOptions<S>,
) -> Result<WeakDom<'dom>, DecodeError> {
    let file = DecompressedFile::from_reader(reader)?;
    // SAFETY: The string internment signature gurantees that
    // the output string slice lives for 'dom, so the input
    // 'file lifetime is irrelevant, and yet the rust compiler
    // forces the input lifetime to match an unspecifiable value
    // ('file only exists inside this scope).
    //
    // Note that there may be a way to do this without unsafe,
    // but I don't know how to do it.
    unsafe fn set_lifetime<'a, T>(value: &T) -> &'a T {
        let ptr = value as *const T;
        unsafe { &*ptr }
    }
    // Force the lifetime to be 'file.
    Deserializer::new().deserialize(unsafe { set_lifetime(&file) }, options)
}

/// Serializes a subset of the given DOM to a binary format model or place,
/// writing to something that implements the `std::io::Write` trait.
pub fn to_writer<W: Write>(writer: W, dom: &WeakDom, refs: &[Ref]) -> Result<(), EncodeError> {
    Serializer::new().serialize(writer, dom, refs)
}
