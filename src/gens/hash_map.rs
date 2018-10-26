use std::collections::HashMap;
use std::hash::{Hash, BuildHasher};

use ::rng::Rng;
use ::gen::Gen;
use ::gens::{self, SizeRange, CollectionBuilder};

struct HashMapBuilder<S>
where
    S: BuildHasher,
{
    build_hasher: S,
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
/// The range specifies the number of tries to generate key-value-entries with distinct keys.
pub fn hash_map_with_hasher<K, V, S>(
    build_hasher_gen: impl Gen<S>,
    elem_gen: impl Gen<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Gen<HashMap<K, V, S>>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    let builder_gen = build_hasher_gen.map(|build_hasher| HashMapBuilder { build_hasher });
    gens::collection(builder_gen, elem_gen, tries_range)
}

/// Generates a `HashMap` that uses a default pseudorando `BuildHasher` and contains keys of type
/// `K` with values of type `V`.
///
/// The range specifies the number of tries to generate key-value-entries with distinct keys.
pub fn hash_map<K, V>(
    elem_gen: impl Gen<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Gen<HashMap<K, V, Rng>>
where
    K: Eq + Hash,
{
    hash_map_with_hasher(gens::rng_fork(), elem_gen, tries_range)
}
