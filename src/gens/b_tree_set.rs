use std::collections::BTreeSet;

use crate::prelude::gens::*;
use crate::gens::{SizeRange, CollectionBuilder};

#[derive(Clone)]
struct BTreeSetBuilder;

impl<T: Ord> CollectionBuilder<T, BTreeSet<T>> for BTreeSetBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> BTreeSet<T> {
        let mut set = BTreeSet::new();
        set.extend(elems);
        set
    }
}

/// Generates a `BTreeSet` that contains elements of type `T`.
///
/// The range specifies the number of tries to generate distinct elements.
pub fn b_tree_set<T: Ord>(
    elem_gen: impl Gen<T>,
    tries_range: impl SizeRange,
) -> impl Gen<BTreeSet<T>> {
    let builder_gen = gens::just(BTreeSetBuilder);
    gens::collection(builder_gen, elem_gen, tries_range)
}