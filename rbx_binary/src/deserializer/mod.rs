mod chunks;
mod error;
mod header;
mod state;

use std::io::Read;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use rbx_dom_weak::WeakDom;
use rbx_reflection::ReflectionDatabase;

use self::state::*;

use crate::chunk::parse_chunks;
use chunks::Chunks;

pub(crate) use self::header::FileHeader;

pub use self::error::Error;

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// A configurable deserializer for Roblox binary models and places.
///
/// ## Example
/// ```no_run
/// use std::fs::File;
/// use std::io::BufReader;
///
/// use rbx_binary::Deserializer;
///
/// let input = BufReader::new(File::open("File.rbxm")?);
///
/// let deserializer = Deserializer::new();
/// let dom = deserializer.deserialize(input)?;
///
/// // rbx_binary always returns a DOM with a DataModel at the top level.
/// // To get to the instances from our file, we need to go one level deeper.
///
/// println!("Root instances in file:");
/// for &referent in dom.root().children() {
///     let instance = dom.get_by_ref(referent).unwrap();
///     println!("- {}", instance.name);
/// }
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Configuration
///
/// A custom [`ReflectionDatabase`][ReflectionDatabase] can be specified via
/// [`reflection_database`][reflection_database].
///
/// [ReflectionDatabase]: rbx_reflection::ReflectionDatabase
/// [reflection_database]: Deserializer#method.reflection_database
pub struct Deserializer<'db> {
    database: &'db ReflectionDatabase<'db>,
}

impl<'db> Deserializer<'db> {
    /// Create a new `Deserializer` with the default settings.
    pub fn new() -> Self {
        Self {
            database: rbx_reflection_database::get().unwrap(),
        }
    }

    /// Sets what reflection database for the deserializer to use.
    #[inline]
    pub fn reflection_database(self, database: &'db ReflectionDatabase<'db>) -> Self {
        Self { database }
    }

    /// Deserialize a Roblox binary model or place from the given stream using
    /// this deserializer.
    pub fn deserialize<R: Read>(&self, mut reader: R) -> Result<WeakDom, Error> {
        profiling::scope!("rbx_binary::deserialize");
        let header = FileHeader::decode(&mut reader)?;
        let chunks = parse_chunks(reader).map_err(error::InnerError::from)?;

        #[cfg(not(feature = "rayon"))]
        let chunks = chunks.into_iter().map(|c| c.decode());

        // decompress all chunks in parallel
        #[cfg(feature = "rayon")]
        let chunks = chunks
            .into_par_iter()
            .map(|c| c.decode())
            .collect::<Vec<_>>();

        let chunks = Chunks::new(&header, chunks)?;

        // Do the rest of the preparation work for parallel decoding
        let mut type_infos = HashMap::with_capacity(header.num_types as usize);
        let root_instance_refs = Vec::new();
        let unknown_type_ids = HashSet::new();

        // Metadata is not used by rbx_binary.
        // if let Some(chunk) = chunks.meta {
        //     decode_meta_chunk(&chunk)?;
        // }
        let shared_strings = if let Some(chunk) = chunks.sstr {
            decode_sstr_chunk(&chunk)?
        } else {
            Vec::new()
        };
        let mut instances = decode_prnt_chunk(&chunks.prnt)?;

        for chunk in chunks.inst {
            decode_inst_chunk(&chunk, self.database, &mut instances, &mut type_infos)?;
        }

        let database = self.database;
        let prop_chunks: Vec<_> = chunks
            .prop
            .into_par_iter()
            .map(|chunk| {
                decode_prop_chunk(&chunk, database, &type_infos, &shared_strings, &instances)
            })
            .collect();

        let mut unknown_type_ids = HashSet::new();

        for prop_chunk in prop_chunks {
            match prop_chunk? {
                PropChunkResult::PropChunk(PropChunk {
                    type_id,
                    canonical_property,
                    values,
                }) => type_infos[&type_id].add_properties(canonical_property, values),
                PropChunkResult::UnknownBinaryType {
                    type_name,
                    prop_name,
                    binary_type_byte,
                } => {
                    if unknown_type_ids.insert(binary_type_byte) {
                        log::warn!(
                            "Unknown value type ID {byte:#04x} ({byte}) in Roblox \
                             binary model file. Found in property {class}.{prop}.",
                            byte = binary_type_byte,
                            class = type_name,
                            prop = prop_name,
                        );
                    }
                }
                PropChunkResult::MissingTypeByte | PropChunkResult::UnknownProperty => {}
            }
        }

        Ok(finish(type_infos))
    }
}

impl Default for Deserializer<'_> {
    fn default() -> Self {
        Self::new()
    }
}
