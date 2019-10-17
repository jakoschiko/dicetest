use std::collections::BTreeSet;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`BTreeSet`] builder for [`dice::collection`].
///
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
/// [`dice::collection`]: fn.collection.html
pub struct BTreeSetBuilder;

impl<T: Ord> CollectionBuilder<T, BTreeSet<T>> for BTreeSetBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> BTreeSet<T> {
        let mut set = BTreeSet::new();
        set.extend(elems);
        set
    }
}

/// Generates a [`BTreeSet`] that contains elements of type `T`.
///
/// The range specifies the number of tries to generate distinct elements.
///
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn b_tree_set<T: Ord>(
    elem_die: impl Die<T>,
    tries_range: impl SizeRange,
) -> impl Die<BTreeSet<T>> {
    let builder_die = dice::from_fn(|_fate| BTreeSetBuilder);
    dice::collection(builder_die, elem_die, tries_range)
}
