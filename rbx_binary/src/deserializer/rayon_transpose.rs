use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::{iter, mem, ptr, slice};

use rayon::iter::plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

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

impl<K, V, S> ParallelIterator for TransposeIntoIter<K, V, S>
where
    K: Send + Clone + Eq + Hash,
    V: Send,
    S: Send + BuildHasher + Default,
{
    type Item = HashMap<K, V, S>;

    fn drive_unindexed<C: UnindexedConsumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<K, V, S> IndexedParallelIterator for TransposeIntoIter<K, V, S>
where
    K: Send + Clone + Eq + Hash,
    V: Send,
    S: Send + BuildHasher + Default,
{
    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn with_producer<CB: ProducerCallback<Self::Item>>(mut self, callback: CB) -> CB::Output {
        TransposeDrain::new(&mut self.map, self.len).with_producer(callback)
    }
}

/// Draining parallel iterator that moves a range out of a vector, but keeps the total capacity.
struct TransposeDrain<'data, K, V: Send, S> {
    map: HashMap<K, &'data mut Vec<V>, S>,
    len: usize,
}

impl<'data, K, V, S> TransposeDrain<'data, K, V, S>
where
    K: Clone + Eq + Hash,
    V: Send,
    S: BuildHasher + Default,
{
    fn new(map: &'data mut HashMap<K, Vec<V>, S>, len: usize) -> Self {
        let map = map
            .iter_mut()
            .map(|(key, vec)| (key.clone(), vec))
            .collect();
        Self { map, len }
    }
}
impl<'data, K, V, S> ParallelIterator for TransposeDrain<'data, K, V, S>
where
    K: 'data + Send + Clone + Eq + Hash,
    V: 'data + Send,
    S: 'data + Send + BuildHasher + Default,
{
    type Item = HashMap<K, V, S>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'data, K, V, S> IndexedParallelIterator for TransposeDrain<'data, K, V, S>
where
    K: 'data + Send + Clone + Eq + Hash,
    V: 'data + Send,
    S: 'data + Send + BuildHasher + Default,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        // Create the producer as the exclusive "owner" of the slice.
        let producer = TransposeDrainProducer::from_transpose(self);

        // The producer will move or drop each item from the drained range.
        callback.callback(producer)
    }
}

impl<'data, K, V: Send, S> Drop for TransposeDrain<'data, K, V, S> {
    fn drop(&mut self) {
        for vec in self.map.values_mut() {
            if vec.len() == self.len {
                // We must not have produced, so just call a normal drain to remove the items.
                vec.clear();
            }
        }
    }
}

// ////////////////////////////////////////////////////////////////////////
struct TransposeDrainProducer<'data, K, V: Send, S> {
    map: HashMap<K, DrainProducer<'data, V>, S>,
    len: usize,
}

impl<'data, K, V, S> TransposeDrainProducer<'data, K, V, S>
where
    K: Clone + Eq + Hash,
    V: Send,
    S: BuildHasher + Default,
{
    fn new(map: HashMap<K, DrainProducer<'data, V>, S>, len: usize) -> Self {
        Self { map, len }
    }

    fn from_transpose(mut transpose: TransposeDrain<'data, K, V, S>) -> Self {
        let len = transpose.len;
        let map = mem::take(&mut transpose.map)
            .into_iter()
            .map(|(key, vec)| (key.clone(), unsafe { DrainProducer::from_vec(vec) }))
            .collect();
        Self::new(map, len)
    }
}

impl<'data, K, V, S> Producer for TransposeDrainProducer<'data, K, V, S>
where
    K: 'data + Send + Clone + Eq + Hash,
    V: 'data + Send,
    S: 'data + Send + BuildHasher + Default,
{
    type Item = HashMap<K, V, S>;
    type IntoIter = TransposeSliceDrain<'data, K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        TransposeSliceDrain {
            map: self
                .map
                .into_iter()
                // replace the slice so we don't drop it twice
                .map(|(key, mut slice)| (key, SliceDrain::new(mem::take(&mut slice.slice))))
                .collect(),
            len: self.len,
        }
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        // TODO: figure out if there is a way to reuse the allocation of self
        let (left, right) = self
            .map
            .into_iter()
            .map(|(key, mut drain_producer)| {
                let slice = mem::take(&mut drain_producer.slice);
                let (left, right) = slice.split_at_mut(index);
                unsafe {
                    (
                        (key.clone(), DrainProducer::new(left)),
                        (key, DrainProducer::new(right)),
                    )
                }
            })
            .collect();
        (Self::new(left, self.len), Self::new(right, self.len))
    }
}

pub(crate) struct DrainProducer<'data, T: Send> {
    slice: &'data mut [T],
}

impl<'data, T: Send> DrainProducer<'data, T> {
    /// Creates a draining producer, which *moves* items from the slice.
    ///
    /// Unsafe because `!Copy` data must not be read after the borrow is released.
    unsafe fn new(slice: &'data mut [T]) -> Self {
        Self { slice }
    }
    /// Creates a draining producer, which *moves* items from the tail of the vector.
    ///
    /// Unsafe because we're moving from beyond `vec.len()`, so the caller must ensure
    /// that data is initialized and not read after the borrow is released.
    unsafe fn from_vec(vec: &'data mut Vec<T>) -> Self {
        let len = vec.len();

        // Make the vector forget about the drained items.
        unsafe { vec.set_len(0) };

        // The pointer is derived from `Vec` directly, not through a `Deref`,
        // so it has provenance over the whole allocation.
        let slice = unsafe {
            let ptr = vec.as_mut_ptr();
            slice::from_raw_parts_mut(ptr, len)
        };

        Self { slice }
    }
}

impl<'data, T: Send> Drop for DrainProducer<'data, T> {
    fn drop(&mut self) {
        // extract the slice so we can use `Drop for [T]`
        let slice_ptr: *mut [T] = mem::take::<&'data mut [T]>(&mut self.slice);
        unsafe { ptr::drop_in_place::<[T]>(slice_ptr) };
    }
}

// ////////////////////////////////////////////////////////////////////////

// like std::vec::Drain, without updating a source Vec
struct TransposeSliceDrain<'data, K, V, S> {
    map: HashMap<K, SliceDrain<'data, V>, S>,
    len: usize,
}

impl<'data, K, V, S> Iterator for TransposeSliceDrain<'data, K, V, S>
where
    K: Clone + Eq + Hash,
    V: 'data,
    S: BuildHasher + Default,
{
    type Item = HashMap<K, V, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.map
            .iter_mut()
            .map(|(key, iter)| {
                // Coerce the pointer early, so we don't keep the
                // reference that's about to be invalidated.
                let ptr: *const V = iter.iter.next()?;
                Some((key.clone(), unsafe { ptr::read(ptr) }))
            })
            .collect()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn count(self) -> usize {
        self.len
    }
}

impl<'data, K, V, S> DoubleEndedIterator for TransposeSliceDrain<'data, K, V, S>
where
    K: Clone + Eq + Hash,
    V: 'data,
    S: BuildHasher + Default,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.map
            .iter_mut()
            .map(|(key, iter)| {
                // Coerce the pointer early, so we don't keep the
                // reference that's about to be invalidated.
                let ptr: *const V = iter.iter.next_back()?;
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

pub(crate) struct SliceDrain<'data, T> {
    iter: slice::IterMut<'data, T>,
}

impl<'data, T> SliceDrain<'data, T> {
    fn new(slice: &'data mut [T]) -> Self {
        let iter = slice.iter_mut();
        Self { iter }
    }
}

impl<'data, T: 'data> Drop for SliceDrain<'data, T> {
    fn drop(&mut self) {
        // extract the iterator so we can use `Drop for [T]`
        let slice_ptr: *mut [T] = mem::replace(&mut self.iter, [].iter_mut()).into_slice();
        unsafe { ptr::drop_in_place::<[T]>(slice_ptr) };
    }
}
