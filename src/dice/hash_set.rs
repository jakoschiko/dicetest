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

impl HashSetBuilder<Prng> {
    fn die() -> impl Die<Self> {
        dice::prng_fork().map(Self::with_hasher)
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
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(1337.into());
/// let fate = &mut Fate::new(&mut prng, 100.into());
/// let elem_die = dice::u8(..);
///
/// let set = dice::hash_set(&elem_die, ..).roll(fate);
/// assert!(set.len() <= 100);
///
/// let set = dice::hash_set(&elem_die, ..=73).roll(fate);
/// assert!(set.len() <= 73);
///
/// let set = dice::hash_set(&elem_die, 17..).roll(fate);
/// assert!(set.len() >= 17);
///
/// let set = dice::hash_set(&elem_die, 42).roll(fate);
/// assert!(set.len() <= 42);
/// ```
pub fn hash_set<T>(elem_die: impl Die<T>, tries_range: impl SizeRange) -> impl Die<HashSet<T, Prng>>
where
    T: Eq + Hash,
{
    dice::collection(HashSetBuilder::die(), elem_die, tries_range)
}
