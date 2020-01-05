use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`HashSet`] builder for [`dice::collection`].
///
/// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
/// [`dice::collection`]: fn.collection.html
pub struct HashSetBuilder<S>
where
    S: BuildHasher,
{
    build_hasher: S,
}

impl<S> HashSetBuilder<S>
where
    S: BuildHasher,
{
    /// Creates a builder that uses the given [`BuildHasher`] for constructing a [`HashSet`].
    ///
    /// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
    /// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
    pub fn with_hasher(build_hasher: S) -> Self {
        Self { build_hasher }
    }
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

/// Generates a [`HashSet`] that uses a default pseudorandom [`BuildHasher`] and contains elements
/// of type `T`.
///
/// The range specifies the number of tries to generate distinct elements.
///
/// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
/// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn hash_set<T>(elem_die: impl Die<T>, tries_range: impl SizeRange) -> impl Die<HashSet<T, Prng>>
where
    T: Eq + Hash,
{
    let builder_die = dice::prng_fork().map(HashSetBuilder::with_hasher);
    dice::collection(builder_die, elem_die, tries_range)
}
