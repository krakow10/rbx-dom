/// Host storage for decompressed data.  The WeakDom may
/// contain references to this data depending on the DecodeOptions.
pub struct DecompressionHost {
    alloc: bumpalo::Bump,
}

impl DecompressionHost {
    /// TODO: doc
    pub fn new() -> Self {
        Self {
            alloc: bumpalo::Bump::new(),
        }
    }
    /// TODO: doc
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            alloc: bumpalo::Bump::with_capacity(capacity),
        }
    }
    /// Allocate a buffer of a specific size.  Used internally
    /// to create a buffer to decompress into. The buffer
    /// contains uninitialized memory and should not be read from.
    pub fn alloc_buffer(&self, size: usize) -> &mut [u8] {
        let layout = core::alloc::Layout::array::<u8>(size).unwrap();
        let ptr = self.alloc.alloc_layout(layout).as_ptr();

        // SAFETY: bumpalo panics if allocation fails
        // meaning ptr is always non-null
        unsafe { core::slice::from_raw_parts_mut(ptr, size) }
    }
    /// Used for benchmarking only
    #[doc(hidden)]
    pub unsafe fn clear(&mut self) {
        self.alloc.reset();
    }
}
