use std::collections::VecDeque;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`VecDeque`] builder for [`dice::collection`].
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
/// [`dice::collection`]: fn.collection.html
pub struct VecDequeBuilder;

impl<T> CollectionBuilder<T, VecDeque<T>> for VecDequeBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> VecDeque<T> {
        let mut vec = VecDeque::with_capacity(elems.len());
        vec.extend(elems);
        vec
    }
}

/// Generates a [`VecDeque`] that contains elements of type `T`.
///
/// The range specifies the length of the [`VecDeque`].
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn vec_deque<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<VecDeque<T>> {
    let builder_die = dice::from_fn(|_fate| VecDequeBuilder);
    dice::collection(builder_die, elem_die, len_range)
}
