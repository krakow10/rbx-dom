use std::{iter, mem, ptr, slice};

use rayon::iter::plumbing::Producer;

// vec.rs Rayon source code

// ////////////////////////////////////////////////////////////////////////

pub(crate) struct DrainProducer<'data, T: Send> {
    slice: &'data mut [T],
}

impl<T: Send> DrainProducer<'_, T> {
    /// Creates a draining producer, which *moves* items from the slice.
    ///
    /// Unsafe because `!Copy` data must not be read after the borrow is released.
    pub(crate) unsafe fn new(slice: &mut [T]) -> DrainProducer<'_, T> {
        DrainProducer { slice }
    }

    /// Creates a draining producer, which *moves* items from the tail of the vector.
    ///
    /// Unsafe because we're moving from beyond `vec.len()`, so the caller must ensure
    /// that data is initialized and not read after the borrow is released.
    pub(crate) unsafe fn from_vec(vec: &mut Vec<T>, len: usize) -> DrainProducer<'_, T> {
        let start = vec.len();
        assert!(vec.capacity() - start >= len);

        // The pointer is derived from `Vec` directly, not through a `Deref`,
        // so it has provenance over the whole allocation.
        let ptr = vec.as_mut_ptr().add(start);
        DrainProducer::new(slice::from_raw_parts_mut(ptr, len))
    }
}

impl<'data, T: 'data + Send> Producer for DrainProducer<'data, T> {
    type Item = T;
    type IntoIter = SliceDrain<'data, T>;

    fn into_iter(mut self) -> Self::IntoIter {
        // replace the slice so we don't drop it twice
        let slice = mem::take(&mut self.slice);
        SliceDrain {
            iter: slice.iter_mut(),
        }
    }

    fn split_at(mut self, index: usize) -> (Self, Self) {
        // replace the slice so we don't drop it twice
        let slice = mem::take(&mut self.slice);
        let (left, right) = slice.split_at_mut(index);
        unsafe { (DrainProducer::new(left), DrainProducer::new(right)) }
    }
}

impl<'data, T: 'data + Send> Drop for DrainProducer<'data, T> {
    fn drop(&mut self) {
        // extract the slice so we can use `Drop for [T]`
        let slice_ptr: *mut [T] = mem::take::<&'data mut [T]>(&mut self.slice);
        unsafe { ptr::drop_in_place::<[T]>(slice_ptr) };
    }
}

// ////////////////////////////////////////////////////////////////////////

// like std::vec::Drain, without updating a source Vec
pub(crate) struct SliceDrain<'data, T> {
    iter: slice::IterMut<'data, T>,
}

impl<'data, T: 'data> Iterator for SliceDrain<'data, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // Coerce the pointer early, so we don't keep the
        // reference that's about to be invalidated.
        let ptr: *const T = self.iter.next()?;
        Some(unsafe { ptr::read(ptr) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.len()
    }
}

impl<'data, T: 'data> DoubleEndedIterator for SliceDrain<'data, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Coerce the pointer early, so we don't keep the
        // reference that's about to be invalidated.
        let ptr: *const T = self.iter.next_back()?;
        Some(unsafe { ptr::read(ptr) })
    }
}

impl<'data, T: 'data> ExactSizeIterator for SliceDrain<'data, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'data, T: 'data> iter::FusedIterator for SliceDrain<'data, T> {}

impl<'data, T: 'data> Drop for SliceDrain<'data, T> {
    fn drop(&mut self) {
        // extract the iterator so we can use `Drop for [T]`
        let slice_ptr: *mut [T] = mem::replace(&mut self.iter, [].iter_mut()).into_slice();
        unsafe { ptr::drop_in_place::<[T]>(slice_ptr) };
    }
}
