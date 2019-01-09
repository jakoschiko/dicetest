use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// `HashMap` builder for `dice::collection`.
pub struct HashMapBuilder<S>
where
    S: BuildHasher,
{
    build_hasher: S,
}

impl<S> HashMapBuilder<S>
where
    S: BuildHasher,
{
    pub fn new(build_hasher: S) -> Self {
        HashMapBuilder { build_hasher }
    }
}

impl<K, V, S> CollectionBuilder<(K, V), HashMap<K, V, S>> for HashMapBuilder<S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn build(self, elems: impl ExactSizeIterator<Item = (K, V)>) -> HashMap<K, V, S> {
        let build_hasher = self.build_hasher;
        let mut map = HashMap::with_capacity_and_hasher(elems.len(), build_hasher);
        map.extend(elems);
        map
    }
}

/// Generates a `HashMap` that uses a custom `BuildHasher` and contains keys of type `K` with
/// values of type `V`.
///
/// The range specifies the number of tries to generate key-value entries with distinct keys.
pub fn hash_map_with_hasher<K, V, S>(
    build_hasher_die: impl Die<S>,
    elem_die: impl Die<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Die<HashMap<K, V, S>>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    let builder_die = build_hasher_die.map(HashMapBuilder::new);
    dice::collection(builder_die, elem_die, tries_range)
}

/// Generates a `HashMap` that uses a default pseudorando `BuildHasher` and contains keys of type
/// `K` with values of type `V`.
///
/// The range specifies the number of tries to generate key-value entries with distinct keys.
pub fn hash_map<K, V>(
    elem_die: impl Die<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Die<HashMap<K, V, Prng>>
where
    K: Eq + Hash,
{
    hash_map_with_hasher(dice::prng_fork(), elem_die, tries_range)
}
