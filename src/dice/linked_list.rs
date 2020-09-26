use std::collections::LinkedList;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::*;

/// [`LinkedList`] builder for [`dice::collection`].
///
/// [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
/// [`dice::collection`]: fn.collection.html
pub struct LinkedListBuilder;

impl LinkedListBuilder {
    fn die() -> impl Die<Self> {
        dice::from_fn(|_fate| Self)
    }
}

impl<T> CollectionBuilder<T, LinkedList<T>> for LinkedListBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> LinkedList<T> {
        elems.collect()
    }
}

/// Generates a [`LinkedList`] that contains elements of type `T`.
///
/// The range specifies the length of the [`LinkedList`].
///
/// [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
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
/// let list = fate.with_limit(100.into()).roll(dice::linked_list(&elem_die, ..));
/// assert!(list.len() <= 100);
///
/// let list = fate.roll(dice::linked_list(&elem_die, ..=73));
/// assert!(list.len() <= 73);
///
/// let list = fate.roll(dice::linked_list(&elem_die, 17..));
/// assert!(list.len() >= 17);
///
/// let list = fate.roll(dice::linked_list(&elem_die, 42));
/// assert!(list.len() == 42);
/// ```
pub fn linked_list<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<LinkedList<T>> {
    dice::collection(LinkedListBuilder::die(), elem_die, len_range)
}

/// Similar to `dice::linked_list` but each element is generated using only a random part of
/// `Limit`.
///
/// If you want to generate a `LinkedList` that contains other collections, then you should
/// consider using this generator for the outer `LinkedList`. That way the overall size is
/// bounded by `Limit` (and not the square of `Limit`).
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
/// let list_die = dice::linked_list(elem_die, ..);
/// let list_of_lists_die = dice::outer_linked_list(list_die, ..);
///
/// let list_of_lists = fate.roll(list_of_lists_die);
/// assert!(list_of_lists.iter().flatten().count() <= 100);
/// ```
pub fn outer_linked_list<T>(
    elem_die: impl Die<T>,
    len_range: impl SizeRange,
) -> impl Die<LinkedList<T>> {
    dice::outer_collection(LinkedListBuilder::die(), elem_die, len_range)
}
