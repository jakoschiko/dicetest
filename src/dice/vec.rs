use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// `Vec` builder for `dice::collection`.
#[derive(Clone)]
pub struct VecBuilder;

impl<T> CollectionBuilder<T, Vec<T>> for VecBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(elems.len());
        vec.extend(elems);
        vec
    }
}

/// Generates a `Vec` that contains elements of type `T`.
///
/// The range specifies the length of the `Vec`.
pub fn vec<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<Vec<T>> {
    let builder_die = dice::just(VecBuilder);
    dice::collection(builder_die, elem_die, len_range)
}
