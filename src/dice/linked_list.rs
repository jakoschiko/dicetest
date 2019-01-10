use std::collections::LinkedList;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// `LinkedList` builder for `dice::collection`.
#[derive(Clone)]
pub struct LinkedListBuilder;

impl<T> CollectionBuilder<T, LinkedList<T>> for LinkedListBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> LinkedList<T> {
        let mut vec = LinkedList::new();
        vec.extend(elems);
        vec
    }
}

/// Generates a `LinkedList` that contains elements of type `T`.
///
/// The range specifies the length of the `LinkedList`.
pub fn linked_list<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<LinkedList<T>> {
    let builder_die = dice::just(LinkedListBuilder);
    dice::collection(builder_die, elem_die, len_range)
}