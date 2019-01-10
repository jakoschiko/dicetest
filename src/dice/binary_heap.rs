use std::collections::BinaryHeap;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// `BinaryHeap` builder for `dice::collection`.
#[derive(Clone)]
pub struct BinaryHeapBuilder;

impl<T: Ord> CollectionBuilder<T, BinaryHeap<T>> for BinaryHeapBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> BinaryHeap<T> {
        let mut heap = BinaryHeap::with_capacity(elems.len());
        heap.extend(elems);
        heap
    }
}

/// Generates a `BinaryHeap` that contains elements of type `T`.
///
/// The range specifies the length of the `BinaryHeap`.
pub fn binary_heap<T: Ord>(
    elem_die: impl Die<T>,
    len_range: impl SizeRange,
) -> impl Die<BinaryHeap<T>> {
    let builder_die = dice::just(BinaryHeapBuilder);
    dice::collection(builder_die, elem_die, len_range)
}