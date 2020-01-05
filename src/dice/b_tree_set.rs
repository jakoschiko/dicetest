use std::collections::BTreeSet;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`BTreeSet`] builder for [`dice::collection`].
///
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
/// [`dice::collection`]: fn.collection.html
pub struct BTreeSetBuilder;

impl BTreeSetBuilder {
    fn die() -> impl Die<Self> {
        dice::from_fn(|_fate| Self)
    }
}

impl<T> CollectionBuilder<T, BTreeSet<T>> for BTreeSetBuilder
where
    T: Ord,
{
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> BTreeSet<T> {
        let mut set = BTreeSet::new();
        set.extend(elems);
        set
    }
}

/// Generates a [`BTreeSet`] that contains elements of type `T`.
///
/// The range specifies the number of tries to generate distinct elements.
///
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
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
/// let set = dice::b_tree_set(&elem_die, ..).roll(fate);
/// assert!(set.len() <= 100);
///
/// let set = dice::b_tree_set(&elem_die, ..=73).roll(fate);
/// assert!(set.len() <= 73);
///
/// let set = dice::b_tree_set(&elem_die, 17..).roll(fate);
/// assert!(set.len() >= 17);
///
/// let set = dice::b_tree_set(&elem_die, 42).roll(fate);
/// assert!(set.len() <= 42);
/// ```
pub fn b_tree_set<T>(elem_die: impl Die<T>, tries_range: impl SizeRange) -> impl Die<BTreeSet<T>>
where
    T: Ord,
{
    dice::collection(BTreeSetBuilder::die(), elem_die, tries_range)
}
