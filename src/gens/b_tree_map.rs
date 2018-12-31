use std::collections::BTreeMap;

use crate::gens::{CollectionBuilder, SizeRange};
use crate::prelude::gens::*;

/// `BTreeMap` builder for `gens::collection`.
#[derive(Clone)]
pub struct BTreeMapBuilder;

impl<K: Ord, V> CollectionBuilder<(K, V), BTreeMap<K, V>> for BTreeMapBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = (K, V)>) -> BTreeMap<K, V> {
        let mut map = BTreeMap::new();
        map.extend(elems);
        map
    }
}

/// Generates a `BTreeMap` that contains keys of type `K` with values of type `V`.
///
/// The range specifies the number of tries to generate key-value entries with distinct keys.
pub fn b_tree_map<K: Ord, V>(
    elem_gen: impl Gen<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Gen<BTreeMap<K, V>> {
    let builder_gen = gens::just(BTreeMapBuilder);
    gens::collection(builder_gen, elem_gen, tries_range)
}
