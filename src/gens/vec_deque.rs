use std::collections::VecDeque;

use crate::gens::{CollectionBuilder, SizeRange};
use crate::prelude::gens::*;

/// `VecDeque` builder for `gens::collection`.
#[derive(Clone)]
pub struct VecDequeBuilder;

impl<T> CollectionBuilder<T, VecDeque<T>> for VecDequeBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> VecDeque<T> {
        let mut vec = VecDeque::with_capacity(elems.len());
        vec.extend(elems);
        vec
    }
}

/// Generates a `VecDeque` that contains elements of type `T`.
///
/// The range specifies the length of the `VecDeque`.
pub fn vec_deque<T>(elem_gen: impl Gen<T>, len_range: impl SizeRange) -> impl Gen<VecDeque<T>> {
    let builder_gen = gens::just(VecDequeBuilder);
    gens::collection(builder_gen, elem_gen, len_range)
}
