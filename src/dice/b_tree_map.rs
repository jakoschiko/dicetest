use std::collections::BTreeMap;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::*;

/// [`BTreeMap`] builder for [`dice::collection`].
///
/// [`dice::collection`]: dice::collection()
pub struct BTreeMapBuilder;

impl BTreeMapBuilder {
    fn die() -> impl Die<Self> {
        dice::from_fn(|_fate| Self)
    }
}

impl<K, V> CollectionBuilder<(K, V), BTreeMap<K, V>> for BTreeMapBuilder
where
    K: Ord,
{
    fn build(self, elems: impl ExactSizeIterator<Item = (K, V)>) -> BTreeMap<K, V> {
        elems.collect()
    }
}

/// Generates a [`BTreeMap`] that contains keys of type `K` with values of type `V`.
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
/// let elem_die = dice::zip_2(dice::u8(..), dice::char());
///
/// let map = fate.with_limit(100.into()).roll(dice::b_tree_map(&elem_die, ..));
/// assert!(map.len() <= 100);
///
/// let map = fate.roll(dice::b_tree_map(&elem_die, ..=73));
/// assert!(map.len() <= 73);
///
/// let map = fate.roll(dice::b_tree_map(&elem_die, 17..));
/// assert!(map.len() >= 17);
///
/// let map = fate.roll(dice::b_tree_map(&elem_die, 42));
/// assert!(map.len() <= 42);
/// ```
pub fn b_tree_map<K, V>(
    elem_die: impl Die<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Die<BTreeMap<K, V>>
where
    K: Ord,
{
    dice::collection(BTreeMapBuilder::die(), elem_die, tries_range)
}

/// Similar to [`dice::b_tree_map`] but each element is generated using only a random part of
/// [`Limit`].
///
/// If you want to generate a [`BTreeMap`] that contains other collections, then you should
/// consider using this generator for the outer [`BTreeMap`]. That way the overall size is
/// bounded by [`Limit`] (and not the square of [`Limit`]).
///
/// [`Limit`]: crate::Limit
/// [`dice::b_tree_map`]: dice::b_tree_map()
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
/// let vec_die = dice::zip_2(dice::u8(..), dice::vec(elem_die, ..));
/// let map_of_vecs_die = dice::outer_b_tree_map(vec_die, ..);
///
/// let map_of_vecs = fate.roll(map_of_vecs_die);
/// assert!(map_of_vecs.values().flatten().count() <= 100);
/// ```
pub fn outer_b_tree_map<K, V>(
    elem_die: impl Die<(K, V)>,
    tries_range: impl SizeRange,
) -> impl Die<BTreeMap<K, V>>
where
    K: Ord,
{
    dice::outer_collection(BTreeMapBuilder::die(), elem_die, tries_range)
}
