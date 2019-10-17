use std::collections::BinaryHeap;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`BinaryHeap`] builder for [`dice::collection`].
///
/// [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
/// [`dice::collection`]: fn.collection.html
pub struct BinaryHeapBuilder;

impl<T: Ord> CollectionBuilder<T, BinaryHeap<T>> for BinaryHeapBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> BinaryHeap<T> {
        let mut heap = BinaryHeap::with_capacity(elems.len());
        heap.extend(elems);
        heap
    }
}

/// Generates a [`BinaryHeap`] that contains elements of type `T`.
///
/// The range specifies the length of the [`BinaryHeap`].
///
/// [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn binary_heap<T: Ord>(
    elem_die: impl Die<T>,
    len_range: impl SizeRange,
) -> impl Die<BinaryHeap<T>> {
    let builder_die = dice::from_fn(|_fate| BinaryHeapBuilder);
    dice::collection(builder_die, elem_die, len_range)
}
