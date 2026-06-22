use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::ops::{Range, RangeBounds};
use std::{iter, mem, ptr, slice};

use rayon::iter::plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer};
use rayon::iter::{IndexedParallelIterator, ParallelDrainRange, ParallelIterator};

/// Parallel iterator that moves HashMap<K,V> out of a hashmap of vectors.
#[derive(Debug, Clone)]
pub struct TransposeIntoIter<K, V, S> {
    map: HashMap<K, Vec<V>, S>,
    len: usize,
}

impl<K, V, S> TransposeIntoIter<K, V, S> {
    pub fn new(map: HashMap<K, Vec<V>, S>, len: usize) -> Self {
        Self { map, len }
    }
}

impl<K: Send, V: Send, S: Send> ParallelIterator for TransposeIntoIter<K, V, S> {
    type Item = HashMap<K, V, S>;

    fn drive_unindexed<C: UnindexedConsumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<K: Send, V: Send, S: Send> IndexedParallelIterator for TransposeIntoIter<K, V, S> {
    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn with_producer<CB: ProducerCallback<Self::Item>>(mut self, callback: CB) -> CB::Output {
        let producer = TransposeProducer::new(self.map, self.len);

        callback.callback(producer)
    }
}

impl<'data, T: Send> ParallelDrainRange<usize> for &'data mut Vec<T> {
    type Iter = Drain<'data, T>;
    type Item = T;

    fn par_drain<R: RangeBounds<usize>>(self, range: R) -> Self::Iter {
        Drain {
            orig_len: self.len(),
            range: simplify_range(range, self.len()),
            vec: self,
        }
    }
}

/// Draining parallel iterator that moves a range out of a vector, but keeps the total capacity.
struct TransposeDrain<'data, K, V: Send, S> {
    map: HashMap<K, &'data mut Vec<V>, S>,
    len: usize,
}

impl<'data, K: Send, V: Send, S: Send> ParallelIterator for TransposeDrain<'data, K, V, S> {
    type Item = HashMap<K, V, S>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'data, K: Send, V: Send, S: Send> IndexedParallelIterator for TransposeDrain<'data, K, V, S> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.range.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        unsafe {
            // Make the vector forget about the drained items, and temporarily the tail too.
            self.vec.set_len(self.range.start);

            // Create the producer as the exclusive "owner" of the slice.
            let producer = TransposeDrainProducer::from_vec(self.vec, self.range.len());

            // The producer will move or drop each item from the drained range.
            callback.callback(producer)
        }
    }
}

impl<'data, T: Send> Drop for Drain<'data, T> {
    fn drop(&mut self) {
        let Range { start, end } = self.range;
        if self.vec.len() == self.orig_len {
            // We must not have produced, so just call a normal drain to remove the items.
            self.vec.drain(start..end);
        } else if start == end {
            // Empty range, so just restore the length to its original state
            unsafe {
                self.vec.set_len(self.orig_len);
            }
        } else if end < self.orig_len {
            // The producer was responsible for consuming the drained items.
            // Move the tail items to their new place, then set the length to include them.
            unsafe {
                let ptr = self.vec.as_mut_ptr().add(start);
                let tail_ptr = self.vec.as_ptr().add(end);
                let tail_len = self.orig_len - end;
                ptr::copy(tail_ptr, ptr, tail_len);
                self.vec.set_len(start + tail_len);
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////
struct TransposeDrainProducer<'data, K, V: Send, S> {
    map: HashMap<K, &'data mut [V], S>,
    len: usize,
}

impl<'data, K: Clone + Eq + Hash, V: Send, S: BuildHasher + Default>
    TransposeDrainProducer<'data, K, V, S>
{
    /// Creates a draining producer, which *moves* items from the slice.
    ///
    /// Unsafe because `!Copy` data must not be read after the borrow is released.
    pub(crate) unsafe fn new(map: HashMap<K, &'data mut [V], S>, len: usize) -> Self {
        Self { map, len }
    }

    /// Creates a draining producer, which *moves* items from the tail of the vector.
    ///
    /// Unsafe because we're moving from beyond `vec.len()`, so the caller must ensure
    /// that data is initialized and not read after the borrow is released.
    unsafe fn from_transpose(transpose: &mut TransposeDrain<'data, K, V, S>) -> Self {
        let len = transpose.len;
        let map = transpose
            .map
            .iter_mut()
            .map(|(key, vec)| {
                // The pointer is derived from `Vec` directly, not through a `Deref`,
                // so it has provenance over the whole allocation.
                let slice = unsafe {
                    let ptr = vec.as_mut_ptr();
                    slice::from_raw_parts_mut(ptr, len)
                };
                (key.clone(), slice)
            })
            .collect();
        Self::new(map, len)
    }
}

impl<
        'data,
        K: 'data + Send + Clone + Eq + Hash,
        V: 'data + Send,
        S: 'data + Send + BuildHasher + Default,
    > Producer for TransposeDrainProducer<'data, K, V, S>
{
    type Item = HashMap<K, V, S>;
    type IntoIter = TransposeSliceDrain<'data, K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        TransposeSliceDrain {
            map: self
                .map
                .into_iter()
                // replace the slice so we don't drop it twice
                .map(|(key, mut slice)| (key, mem::take(&mut slice).iter_mut()))
                .collect(),
            len: self.len,
        }
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        // TODO: figure out if there is a way to reuse the allocation of self
        let (left, right) = self
            .map
            .into_iter()
            .map(|(key, slice)| {
                let (left, right) = slice.split_at_mut(index);
                ((key.clone(), left), (key, right))
            })
            .collect();
        unsafe { (Self::new(left, self.len), Self::new(right, self.len)) }
    }
}

impl<'data, K, V: 'data + Send, S> Drop for TransposeDrainProducer<'data, K, V, S> {
    fn drop(&mut self) {
        for slice in self.map.values_mut() {
            // extract the slice so we can use `Drop for [T]`
            let slice_ptr: *mut [V] = mem::take::<&'data mut [V]>(slice);
            unsafe { ptr::drop_in_place::<[V]>(slice_ptr) };
        }
    }
}

// ////////////////////////////////////////////////////////////////////////

// like std::vec::Drain, without updating a source Vec
struct TransposeSliceDrain<'data, K, V, S> {
    map: HashMap<K, slice::IterMut<'data, V>, S>,
    len: usize,
}

impl<'data, K: Clone + Eq + Hash, V: 'data, S: BuildHasher + Default> Iterator
    for TransposeSliceDrain<'data, K, V, S>
{
    type Item = HashMap<K, V, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.map
            .iter_mut()
            .map(|(key, iter)| {
                // Coerce the pointer early, so we don't keep the
                // reference that's about to be invalidated.
                let ptr: *const V = iter.next()?;
                Some((key.clone(), unsafe { ptr::read(ptr) }))
            })
            .collect::<Option<Self::Item>>()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn count(self) -> usize {
        self.len
    }
}

impl<'data, K: Clone + Eq + Hash, V: 'data, S: BuildHasher + Default> DoubleEndedIterator
    for TransposeSliceDrain<'data, K, V, S>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.map
            .iter_mut()
            .map(|(key, iter)| {
                // Coerce the pointer early, so we don't keep the
                // reference that's about to be invalidated.
                let ptr: *const V = iter.next_back()?;
                Some((key.clone(), unsafe { ptr::read(ptr) }))
            })
            .collect::<Option<Self::Item>>()
    }
}

impl<'data, K: Clone + Eq + Hash, V: 'data, S: BuildHasher + Default> ExactSizeIterator
    for TransposeSliceDrain<'data, K, V, S>
{
    fn len(&self) -> usize {
        self.len
    }
}

impl<'data, K: Clone + Eq + Hash, V: 'data, S: BuildHasher + Default> iter::FusedIterator
    for TransposeSliceDrain<'data, K, V, S>
{
}

impl<'data, K, V, S> Drop for TransposeSliceDrain<'data, K, V, S> {
    fn drop(&mut self) {
        for iter in self.map.values_mut() {
            // extract the iterator so we can use `Drop for [T]`
            let slice_ptr: *mut [V] = mem::replace(iter, [].iter_mut()).into_slice();
            unsafe { ptr::drop_in_place::<[V]>(slice_ptr) };
        }
    }
}
