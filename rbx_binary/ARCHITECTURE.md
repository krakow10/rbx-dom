# Parallel decoding architecture

Decoding is split into two stages: per-chunk parallel, and per-instance parallel.  The chunk layout of some property types can be known without requiring sequential decoding.  Complete decoding of these chunks is deferred to the per-instance parallel stage.  The property chunks which must be decoded sequentially are decoded in the per-chunk parallel stage.

## Stage 1: Per-chunk parallelism: DeserializerState::new
### Parse header
### Parse chunk headers sequentially
  - Call parse_chunks which returns `Vec<CompressedChunk>`
  - This sets us up for parallel chunk decompression
### Decompress all chunks in parallel
### Categorize chunks
  - Chunks are categorized into META, SSTR, INST, PROP, PRNT, END and expected to appear in that order
### Construct type_infos from INST chunks in parallel
  - INST chunks are decoded in parallel and assembled into type_infos HashMap
### Populate type_info.properties HashMap from PROP chunks in parallel
  - PROP chunk header is parsed, property type is determined
  - The data for each chunk is placed into type_info.properties which has the type `UStrMap<TypeChunk>`
  - `TypeChunk` enumerates the allowed binary type and canonical type pairs, and each variant contains an iterator that produces the canonical type from that binary type.
  - Many property types such as `CFrame` must be decoded sequentially because the position of each value is not known before decoding the previous value.  These are decoded with per-chunk parallelism within this stage.  These are converted into their respective TypeChunk variant.
  - Property types for which the chunk layout can be known ahead of time require very little processing in this stage.  These are converted into their respective TypeChunk variant.

### DeserializerState is ready!

## Stage 2: Per-instance parallelism: DeserializerState.finish()
A massive nested structure of parallel iterators is constructed, and run with per-instance parallelism.

Each variant of `TypeChunk` has its own implementation of IndexedParallelIterator which yields items of that Type.  For example, the variant `CFrame(VecIntoIter<CFrame>)` simply contains `Vec<CFrame>.into_par_iter()`, which is the CFrame value for each Part instance.

The property iterators are driven together to yield an iterator of complete instances of that class.  The class instance iterators from type_infos are ultimately converted into rbx_dom_weak::Instance, and then flattened together into a fantastically large iterator, which is collected into WeakDom in parallel.

### Decoding is complete!
