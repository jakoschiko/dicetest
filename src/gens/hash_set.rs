use std::collections::HashSet;
use std::hash::{Hash, BuildHasher};

use crate::prelude::gens::*;
use crate::gens::{SizeRange, CollectionBuilder};

struct HashSetBuilder<S>
where
    S: BuildHasher,
{
    build_hasher: S,
}

impl<T, S> CollectionBuilder<T, HashSet<T, S>> for HashSetBuilder<S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> HashSet<T, S> {
        let build_hasher = self.build_hasher;
        let mut set = HashSet::with_capacity_and_hasher(elems.len(), build_hasher);
        set.extend(elems);
        set
    }
}

/// Generates a `HashSet` that uses a custom `BuildHasher` and contains elements of type `T`.
///
/// The range specifies the number of tries to generate distinct elements.
pub fn hash_set_with_hasher<T, S>(
    build_hasher_gen: impl Gen<S>,
    elem_gen: impl Gen<T>,
    tries_range: impl SizeRange,
) -> impl Gen<HashSet<T, S>>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    let builder_gen = build_hasher_gen.map(|build_hasher| HashSetBuilder { build_hasher });
    gens::collection(builder_gen, elem_gen, tries_range)
}

/// Generates a `HashSet` that uses a default pseudorandom `BuildHasher` and contains elements of
/// type `T`.
///
/// The range specifies the number of tries to generate distinct elements.
pub fn hash_set<T>(
    elem_gen: impl Gen<T>,
    tries_range: impl SizeRange,
) -> impl Gen<HashSet<T, Prng>>
where
    T: Eq + Hash,
{
    hash_set_with_hasher(gens::prng_fork(), elem_gen, tries_range)
}
