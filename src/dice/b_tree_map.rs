use std::collections::BTreeMap;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`BTreeMap`] builder for [`dice::collection`].
///
/// [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
/// [`dice::collection`]: fn.collection.html
pub struct BTreeMapBuilder;

impl<K: Ord, V> CollectionBuilder<(K, V), BTreeMap<K, V>> for BTreeMapBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = (K, V)>) -> BTreeMap<K, V> {
        let mut map = BTreeMap::new();
        map.extend(elems);
        map
    }
}

/// Generates a [`BTreeMap`] that contains keys of type `K` with values of type `V`.
///
/// The range specifies the number of tries to generate key-value entries with distinct keys.
///
/// [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn b_tree_map<K: Ord, V>(
    elem_die: impl Die<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Die<BTreeMap<K, V>> {
    let builder_die = dice::from_fn(|_fate| BTreeMapBuilder);
    dice::collection(builder_die, elem_die, tries_range)
}
