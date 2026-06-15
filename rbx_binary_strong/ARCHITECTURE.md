# Parallel decoding architecture

Decoding is split into two stages: per-chunk parallel, and per-instance parallel.  The chunk layout of some property types can be known without requiring sequential decoding.  Complete decoding of these chunks is deferred to the per-instance parallel stage.  The property chunks which must be decoded sequentially are decoded in the per-chunk parallel stage.

## Stage 1: Per-chunk parallelism: ParallelState::new
### Parse header
### Parse chunk headers sequentially
  - Call parse_chunks which returns `Vec<CompressedChunk<'a>>`
  - This sets us up for parallel chunk decompression
### Decompress all chunks in parallel
### Categorize chunks
  - Chunks are categorized into META, SSTR, INST, PROP, PRNT, END and placed into local variables
### Construct type_infos from INST chunks in parallel
  - INST chunks are decoded in parallel and assembled into type_infos HashMap
### Construct ClassPropertyChunks from PROP chunks in parallel
  - PROP chunk header is parsed, property type is determined
  - Most property types such as `CFrame` must be decoded serially because the position of each value is not known before decoding the previous value.  These are decoded with per-chunk parallelism right now.
  - ClassPropertyChunk is constructed using `ClassPropertyChunk::new` which matches the class and calls the appropriate class chunk constructor.
  - The class chunk constructor (such as `PartPropertyChunk::new`) matches the property name and the binary type provided by the PROP chunk, and infers which decoder should be used.
  - This is essentially multiplexing `PropertyChunk::<T>::new`, with T inferred by the destination enum variant of PartPropertyChunk
  - Each ClassPropertyChunk is inserted into ClassPropertyChunks with the `try_push` method

### ParallelState is ready!

## Stage 2: Per-instance parallelism: ParallelState.finish()
A massive nested structure of parallel iterators is constructed using generated code, and run with per-instance parallelism.

Each field of ClassPropertyChunks has its own implementation of IntoParallelIterator which yields items of that class.  For example, given the field `Part: PartPropertyChunks`, `class_property_chunks.Part.into_par_iter()` yields classes::Part.

Each field of PartPropertyChunks represents the property values for one property of each Part.  For example, the field `CFrame: PropertyChunk<CFrame>` simply contains `Vec<CFrame>`, which is the CFrame value for each Part instance.  Each `PropertyChunk::<T>` has its own implementation of IntoParallelIterator which yields values of that property.  For example, given the field `CFrame: PropertyChunk<CFrame>`, `part_property_chunks.CFrame.into_par_iter()` yields CFrame.

The property iterators are zipped together to yield an iterator of complete instances of that class.  The class instance iterators are ultimately converted into rbx_dom_strong::Instance, and then chained together into a fantastically large iterator, which is collected into StrongDom in parallel.

### Decoding is complete!
