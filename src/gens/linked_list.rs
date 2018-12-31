use std::collections::LinkedList;

use crate::gens::{CollectionBuilder, SizeRange};
use crate::prelude::gens::*;

/// `LinkedList` builder for `gens::collection`.
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
pub fn linked_list<T>(elem_gen: impl Gen<T>, len_range: impl SizeRange) -> impl Gen<LinkedList<T>> {
    let builder_gen = gens::just(LinkedListBuilder);
    gens::collection(builder_gen, elem_gen, len_range)
}
