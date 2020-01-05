use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`HashMap`] builder for [`dice::collection`].
///
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [`dice::collection`]: fn.collection.html
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

/// Generates a [`HashMap`] that uses a default pseudorandom [`BuildHasher`] and contains keys of
/// type `K` with values of type `V`.
///
/// The range specifies the number of tries to generate key-value entries with distinct keys.
///
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn hash_map<K, V>(
    elem_die: impl Die<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Die<HashMap<K, V, Prng>>
where
    K: Eq + Hash,
{
    let builder_die = dice::prng_fork().map(HashMapBuilder::new);
    dice::collection(builder_die, elem_die, tries_range)
}
