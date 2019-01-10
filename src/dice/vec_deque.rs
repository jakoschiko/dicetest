use std::collections::VecDeque;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// `VecDeque` builder for `dice::collection`.
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
pub fn vec_deque<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<VecDeque<T>> {
    let builder_die = dice::just(VecDequeBuilder);
    dice::collection(builder_die, elem_die, len_range)
}