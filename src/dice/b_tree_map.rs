use std::collections::BTreeMap;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`BTreeMap`] builder for [`dice::collection`].
///
/// [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
/// [`dice::collection`]: fn.collection.html
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
        let mut map = BTreeMap::new();
        map.extend(elems);
        map
    }
}

/// Generates a [`BTreeMap`] that contains keys of type `K` with values of type `V`.
///
/// The range specifies the number of tries to generate key-value entries with distinct keys.
///
/// [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
///
/// # Panics
///
/// Panics if the range is empty.
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(1337.into());
/// let fate = &mut Fate::new(&mut prng, 100.into());
/// let elem_die = dice::zip_2(dice::u8(..), dice::char());
///
/// let map = dice::b_tree_map(&elem_die, ..).roll(fate);
/// assert!(map.len() <= 100);
///
/// let map = dice::b_tree_map(&elem_die, ..=73).roll(fate);
/// assert!(map.len() <= 73);
///
/// let map = dice::b_tree_map(&elem_die, 17..).roll(fate);
/// assert!(map.len() >= 17);
///
/// let map = dice::b_tree_map(&elem_die, 42).roll(fate);
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
