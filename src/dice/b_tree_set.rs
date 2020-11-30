use std::collections::BTreeSet;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::*;

/// [`BTreeSet`] builder for [`dice::collection`].
///
/// [`dice::collection`]: dice::collection()
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
        elems.collect()
    }
}

/// Generates a [`BTreeSet`] that contains elements of type `T`.
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
/// let set = fate.with_limit(100.into()).roll(dice::b_tree_set(&elem_die, ..));
/// assert!(set.len() <= 100);
///
/// let set = fate.roll(dice::b_tree_set(&elem_die, ..=73));
/// assert!(set.len() <= 73);
///
/// let set = fate.roll(dice::b_tree_set(&elem_die, 17..));
/// assert!(set.len() >= 17);
///
/// let set = fate.roll(dice::b_tree_set(&elem_die, 42));
/// assert!(set.len() <= 42);
/// ```
pub fn b_tree_set<T>(elem_die: impl Die<T>, tries_range: impl SizeRange) -> impl Die<BTreeSet<T>>
where
    T: Ord,
{
    dice::collection(BTreeSetBuilder::die(), elem_die, tries_range)
}

/// Similar to [`dice::b_tree_set`] but each element is generated using only a random part of
/// [`Limit`].
///
/// If you want to generate a [`BTreeSet`] that contains other collections, then you should
/// consider using this generator for the outer [`BTreeSet`]. That way the overall size is
/// bounded by [`Limit`] (and not the square of [`Limit`]).
///
/// [`Limit`]: crate::Limit
/// [`dice::b_tree_set`]: dice::b_tree_set()
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
/// let set_of_vecs_die = dice::outer_b_tree_set(vec_die, ..);
///
/// let set_of_vecs = fate.roll(set_of_vecs_die);
/// assert!(set_of_vecs.iter().flatten().count() <= 100);
/// ```
pub fn outer_b_tree_set<T>(
    elem_die: impl Die<T>,
    tries_range: impl SizeRange,
) -> impl Die<BTreeSet<T>>
where
    T: Ord,
{
    dice::outer_collection(BTreeSetBuilder::die(), elem_die, tries_range)
}
