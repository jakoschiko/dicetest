use std::collections::BinaryHeap;

use crate::gens::{CollectionBuilder, SizeRange};
use crate::prelude::gens::*;

/// `BinaryHeap` builder for `gens::collection`.
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
    elem_gen: impl Gen<T>,
    len_range: impl SizeRange,
) -> impl Gen<BinaryHeap<T>> {
    let builder_gen = gens::just(BinaryHeapBuilder);
    gens::collection(builder_gen, elem_gen, len_range)
}
