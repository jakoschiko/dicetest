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
/// let mut prng = Prng::from_seed(1337.into());
/// let fate = &mut Fate::new(&mut prng, 100.into());
/// let elem_die = dice::zip_2(dice::u8(..), dice::char());
///
/// let map = dice::hash_map(&elem_die, ..).roll(fate);
/// assert!(map.len() <= 100);
///
/// let map = dice::hash_map(&elem_die, ..=73).roll(fate);
/// assert!(map.len() <= 73);
///
/// let map = dice::hash_map(&elem_die, 17..).roll(fate);
/// assert!(map.len() >= 17);
///
/// let map = dice::hash_map(&elem_die, 42).roll(fate);
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
