use std::collections::LinkedList;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

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
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(1337.into());
/// let fate = &mut Fate::new(&mut prng, 100.into());
/// let elem_die = dice::u8(..);
///
/// let list = dice::linked_list(&elem_die, ..).roll(fate);
/// assert!(list.len() <= 100);
///
/// let list = dice::linked_list(&elem_die, ..=73).roll(fate);
/// assert!(list.len() <= 73);
///
/// let list = dice::linked_list(&elem_die, 17..).roll(fate);
/// assert!(list.len() >= 17);
///
/// let list = dice::linked_list(&elem_die, 42).roll(fate);
/// assert!(list.len() == 42);
/// ```
pub fn linked_list<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<LinkedList<T>> {
    dice::collection(LinkedListBuilder::die(), elem_die, len_range)
}
