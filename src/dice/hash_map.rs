use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

use crate::dice::{CollectionBuilder, LengthRange};
use crate::prelude::*;
use crate::Prng;

/// [`HashMap`] builder for [`dice::collection`].
///
/// [`dice::collection`]: dice::collection()
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
    /// Creates a builder that uses the given [`BuildHasher`] for constructing a [`HashMap`].
    pub fn with_hasher(build_hasher: S) -> Self {
        HashMapBuilder { build_hasher }
    }
}

impl HashMapBuilder<Prng> {
    fn die() -> impl Die<Self> {
        dice::from_fn(|mut fate| Self::with_hasher(fate.fork_prng()))
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
/// let elem_die = dice::zip().two(dice::u8(..), dice::char());
///
/// let map = fate.with_limit(100.into()).roll(dice::hash_map(&elem_die, ..));
/// assert!(map.len() <= 100);
///
/// let map = fate.roll(dice::hash_map(&elem_die, ..=73));
/// assert!(map.len() <= 73);
///
/// let map = fate.roll(dice::hash_map(&elem_die, 17..));
/// assert!(map.len() >= 17);
///
/// let map = fate.roll(dice::hash_map(&elem_die, 42));
/// assert!(map.len() <= 42);
/// ```
pub fn hash_map<K, V>(
    elem_die: impl Die<(K, V)>,
    tries_range: impl LengthRange,
) -> impl Die<HashMap<K, V, Prng>>
where
    K: Eq + Hash,
{
    dice::collection(HashMapBuilder::die(), elem_die, tries_range)
}

/// Similar to [`dice::hash_map`] but each element is generated using only a random part of
/// [`Limit`].
///
/// If you want to generate a [`HashMap`] that contains other collections, then you should
/// consider using this generator for the outer [`HashMap`]. That way the overall length is
/// bounded by [`Limit`] (and not the square of [`Limit`]).
///
/// [`Limit`]: crate::Limit
/// [`dice::hash_map`]: dice::hash_map()
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
/// let elem_die = dice::char();
/// let vec_die = dice::zip().two(dice::u8(..), dice::vec(elem_die, ..));
/// let map_of_vecs_die = dice::outer_hash_map(vec_die, ..);
///
/// let map_of_vecs = fate.roll(map_of_vecs_die);
/// assert!(map_of_vecs.values().flatten().count() <= 100);
/// ```
pub fn outer_hash_map<K, V>(
    elem_die: impl Die<(K, V)>,
    tries_range: impl LengthRange,
) -> impl Die<HashMap<K, V, Prng>>
where
    K: Eq + Hash,
{
    dice::outer_collection(HashMapBuilder::die(), elem_die, tries_range)
}
