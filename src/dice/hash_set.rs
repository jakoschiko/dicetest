use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

use crate::Prng;
use crate::dice::{CollectionBuilder, LengthRange};
use crate::prelude::*;

/// [`HashSet`] builder for [`dice::collection`].
///
/// [`dice::collection`]: dice::collection()
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
    pub fn with_hasher(build_hasher: S) -> Self {
        Self { build_hasher }
    }
}

impl HashSetBuilder<Prng> {
    fn die() -> impl Die<Self> {
        dice::from_fn(|mut fate| Self::with_hasher(fate.fork_prng()))
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
/// # Panics
///
/// Panics if the range is empty.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let elem_die = dice::u8(..);
///
/// let set = fate.with_limit(100.into()).roll(dice::hash_set(&elem_die, ..));
/// assert!(set.len() <= 100);
///
/// let set = fate.roll(dice::hash_set(&elem_die, ..=73));
/// assert!(set.len() <= 73);
///
/// let set = fate.roll(dice::hash_set(&elem_die, 17..));
/// assert!(set.len() >= 17);
///
/// let set = fate.roll(dice::hash_set(&elem_die, 42));
/// assert!(set.len() <= 42);
/// ```
pub fn hash_set<T>(
    elem_die: impl Die<T>,
    tries_range: impl LengthRange,
) -> impl Die<HashSet<T, Prng>>
where
    T: Eq + Hash,
{
    dice::collection(HashSetBuilder::die(), elem_die, tries_range)
}

/// Similar to [`dice::hash_set`] but each element is generated using only a random part of
/// [`Limit`].
///
/// If you want to generate a [`HashSet`] that contains other collections, then you should
/// consider using this generator for the outer [`HashSet`]. That way the overall length is
/// bounded by [`Limit`] (and not the square of [`Limit`]).
///
/// [`Limit`]: crate::Limit
/// [`dice::hash_set`]: dice::hash_set()
///
/// # Panics
///
/// Panics if the range is empty.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let elem_die = dice::u8(..);
/// let vec_die = dice::vec(elem_die, ..);
/// let set_of_vecs_die = dice::outer_hash_set(vec_die, ..);
///
/// let set_of_vecs = fate.roll(set_of_vecs_die);
/// assert!(set_of_vecs.iter().flatten().count() <= 100);
/// ```
pub fn outer_hash_set<T>(
    elem_die: impl Die<T>,
    tries_range: impl LengthRange,
) -> impl Die<HashSet<T, Prng>>
where
    T: Eq + Hash,
{
    dice::outer_collection(HashSetBuilder::die(), elem_die, tries_range)
}
