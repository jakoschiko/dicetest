use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;
use std::collections::VecDeque;

/// [`VecDeque`] builder for [`dice::collection`].
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
/// [`dice::collection`]: fn.collection.html
pub struct VecDequeBuilder;

impl VecDequeBuilder {
    fn die() -> impl Die<Self> {
        dice::from_fn(|_fate| Self)
    }
}

impl<T> CollectionBuilder<T, VecDeque<T>> for VecDequeBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> VecDeque<T> {
        let mut vec = VecDeque::with_capacity(elems.len());
        vec.extend(elems);
        vec
    }
}

/// Generates a [`VecDeque`] that contains elements of type `T`.
///
/// The range specifies the length of the [`VecDeque`].
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
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
/// let vec = dice::vec_deque(&elem_die, ..).roll(fate);
/// assert!(vec.len() <= 100);
///
/// let vec = dice::vec_deque(&elem_die, ..=73).roll(fate);
/// assert!(vec.len() <= 73);
///
/// let vec = dice::vec_deque(&elem_die, 17..).roll(fate);
/// assert!(vec.len() >= 17);
///
/// let vec = dice::vec_deque(&elem_die, 42).roll(fate);
/// assert!(vec.len() == 42);
/// ```
pub fn vec_deque<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<VecDeque<T>> {
    dice::collection(VecDequeBuilder::die(), elem_die, len_range)
}
