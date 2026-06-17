use std::{iter, mem, ptr, slice};

use rayon::iter::{
    plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer},
    IndexedParallelIterator, IntoParallelIterator, ParallelDrainRange, ParallelIterator,
};

use rbx_dom_weak::types::*;
use rbx_dom_weak::UstrMap;

type Properties = UstrMap<Variant>;

#[derive(Debug, Clone)]
pub enum TypeChunk<'dom> {
    // These chunks are decoded in stage 1
    String(Vec<&'dom str>),
    BinaryString(Vec<&'dom [u8]>),
    ContentId(Vec<ContentId>),
    Tags(Vec<Tags>),
    MaterialColors(Vec<MaterialColors>),
    SharedString(Vec<SharedString>),
    NetAssetRef(Vec<NetAssetRef>),
    BrickColor(Vec<BrickColor>),
    CFrame(Vec<CFrame>),
    NumberSequence(Vec<NumberSequence>),
    ColorSequence(Vec<ColorSequence>),
    NumberRange(Vec<NumberRange>),
    PhysicalProperties(Vec<PhysicalProperties>),
    OptionalCFrame(Vec<Option<CFrame>>),
    Font(Vec<Font>),
    Content(Vec<Content>),
    // These chunks are decoded in stage 2
    // TODO: write custom parallel iterators for each one e.g. BoolIter<'dom>
    Bool(Vec<bool>),
    Int32(Vec<i32>),
    Float32(Vec<f32>),
    Float64(Vec<f64>),
    UDim(Vec<UDim>),
    UDim2(Vec<UDim2>),
    Ray(Vec<Ray>),
    Faces(Vec<Faces>),
    Axes(Vec<Axes>),
    Color3(Vec<Color3>),
    Vector2(Vec<Vector2>),
    Vector3(Vec<Vector3>),
    Enum(Vec<Enum>),
    Ref(Vec<Ref>),
    Vector3int16(Vec<Vector3int16>),
    Rect(Vec<Rect>),
    Color3uint8(Vec<Color3uint8>),
    Int64(Vec<i64>),
    UniqueId(Vec<UniqueId>),
    SecurityCapabilities(Vec<SecurityCapabilities>),
}

/// Parallel iterator that moves out of a vector.
#[derive(Debug, Clone)]
pub struct TypeChunkIntoIter<'dom> {
    inner: TypeChunk<'dom>,
}

impl<'dom> IntoParallelIterator for TypeChunk<'dom> {
    type Item = Variant;
    type Iter = TypeChunkIntoIter<'dom>;

    fn into_par_iter(self) -> Self::Iter {
        TypeChunkIntoIter { inner: self }
    }
}

impl<'dom> ParallelIterator for TypeChunkIntoIter<'dom> {
    type Item = Variant;

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

impl<'dom> IndexedParallelIterator for TypeChunkIntoIter<'dom> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        match self.inner {
            TypeChunk::String(mut items) => items
                .par_drain(..)
                .map(Variant::from)
                .with_producer(callback),
            TypeChunk::BinaryString(items) => todo!(),
            TypeChunk::ContentId(content_ids) => todo!(),
            TypeChunk::Tags(items) => todo!(),
            TypeChunk::MaterialColors(items) => todo!(),
            TypeChunk::SharedString(shared_strings) => todo!(),
            TypeChunk::NetAssetRef(net_asset_refs) => todo!(),
            TypeChunk::BrickColor(brick_colors) => todo!(),
            TypeChunk::CFrame(cframes) => todo!(),
            TypeChunk::NumberSequence(number_sequences) => todo!(),
            TypeChunk::ColorSequence(color_sequences) => todo!(),
            TypeChunk::NumberRange(number_ranges) => todo!(),
            TypeChunk::PhysicalProperties(items) => todo!(),
            TypeChunk::OptionalCFrame(cframes) => todo!(),
            TypeChunk::Font(fonts) => todo!(),
            TypeChunk::Content(contents) => todo!(),
            TypeChunk::Bool(items) => todo!(),
            TypeChunk::Int32(items) => todo!(),
            TypeChunk::Float32(items) => todo!(),
            TypeChunk::Float64(items) => todo!(),
            TypeChunk::UDim(udims) => todo!(),
            TypeChunk::UDim2(udim2s) => todo!(),
            TypeChunk::Ray(items) => todo!(),
            TypeChunk::Faces(items) => todo!(),
            TypeChunk::Axes(items) => todo!(),
            TypeChunk::Color3(color3s) => todo!(),
            TypeChunk::Vector2(vector2s) => todo!(),
            TypeChunk::Vector3(vector3s) => todo!(),
            TypeChunk::Enum(items) => todo!(),
            TypeChunk::Ref(items) => todo!(),
            TypeChunk::Vector3int16(vector3int16s) => todo!(),
            TypeChunk::Rect(rects) => todo!(),
            TypeChunk::Color3uint8(color3uint8s) => todo!(),
            TypeChunk::Int64(items) => todo!(),
            TypeChunk::UniqueId(unique_ids) => todo!(),
            TypeChunk::SecurityCapabilities(items) => todo!(),
        }
    }
}

// We need access to DrainProducer, but it's a private type
// === PASTED RAYON INTERNALS === //
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
    unsafe fn from_vec(vec: &mut Vec<T>, len: usize) -> DrainProducer<'_, T> {
        let start = vec.len();
        assert!(vec.capacity() - start >= len);

        // The pointer is derived from `Vec` directly, not through a `Deref`,
        // so it has provenance over the whole allocation.
        unsafe {
            let ptr = vec.as_mut_ptr().add(start);
            DrainProducer::new(slice::from_raw_parts_mut(ptr, len))
        }
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
// === END OF PASTED RAYON INTERNALS === //

/// Iterator which yields items of Instance.
// In rayon this is literally not an iterator.
// It's a thing that can generate a producer, and a producer can generate a (sequential) iterator.
struct TypeInfoIter<'dom> {
    type_info: TypeInfo<'dom>,
    num_instances: usize,
}

impl<'dom> ParallelIterator for TypeInfoIter<'dom> {
    type Item = Instance;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.num_instances)
    }
}
impl<'dom> IndexedParallelIterator for TypeInfoIter<'dom> {
    fn len(&self) -> usize {
        self.num_instances
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        // TODO: roll referent, children, parent, properties together with MultiZip or something
        callback.callback(PropertiesProducer {
            properties: self.type_info.properties,
            num_instances: self.num_instances,
        })
    }
}

struct PropertiesProducerIter<'dom> {
    /// A collection of property chunks for this type, with the value for each instance in order.
    properties: UstrMap<TypeChunk<'dom>>,

    num_instances: usize,
}

/// Iterator Producer which can by split
struct PropertiesProducer<'dom> {
    /// A collection of property chunks for this type, with the value for each instance in order.
    properties: UstrMap<TypeChunk<'dom>>,
    num_instances: usize,
}

impl<'dom> Producer for PropertiesProducer<'dom> {
    type Item = Instance;

    type IntoIter = PropertiesProducerIter<'dom>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        todo!()
    }
}
