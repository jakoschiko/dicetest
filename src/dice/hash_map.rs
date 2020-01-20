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
    /// Creates a builder that uses the given [`BuildHasher`] for constructing a [`HashMap`].
    ///
    /// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
    /// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    pub fn with_hasher(build_hasher: S) -> Self {
        HashMapBuilder { build_hasher }
    }
}

impl HashMapBuilder<Prng> {
    fn die() -> impl Die<Self> {
        dice::prng_fork().map(Self::with_hasher)
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
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut fate = Fate {
///     prng: &mut Prng::from_seed(1337.into()),
///     limit: 100.into(),
/// };
/// let elem_die = dice::zip_2(dice::u8(..), dice::char());
///
/// let map = fate.roll(dice::hash_map(&elem_die, ..));
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
    tries_range: impl SizeRange,
) -> impl Die<HashMap<K, V, Prng>>
where
    K: Eq + Hash,
{
    dice::collection(HashMapBuilder::die(), elem_die, tries_range)
}

/// Similar to `dice::hash_map` but each element is generated using only a random part of
/// `Limit`.
///
/// If you want to generate a `HashMap` that contains other collections, then you should
/// consider using this generator for the outer `HashMap`. That way the overall size is
/// bounded by `Limit` (and not the square of `Limit`).
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
/// let mut fate = Fate {
///     prng: &mut Prng::from_seed(1337.into()),
///     limit: 100.into(),
/// };
/// let elem_die = dice::char();
/// let vec_die = dice::zip_2(dice::u8(..), dice::vec(elem_die, ..));
/// let map_of_vecs_die = dice::outer_hash_map(vec_die, ..);
///
/// let map_of_vecs = fate.roll(map_of_vecs_die);
/// assert!(map_of_vecs.values().flatten().count() <= 100);
/// ```
pub fn outer_hash_map<K, V>(
    elem_die: impl Die<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Die<HashMap<K, V, Prng>>
where
    K: Eq + Hash,
{
    dice::outer_collection(HashMapBuilder::die(), elem_die, tries_range)
}
