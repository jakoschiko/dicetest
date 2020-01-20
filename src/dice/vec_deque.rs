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
/// let mut fate = Fate {
///     prng: &mut Prng::from_seed(1337.into()),
///     limit: 100.into(),
/// };
/// let elem_die = dice::u8(..);
///
/// let vec = fate.roll(dice::vec_deque(&elem_die, ..));
/// assert!(vec.len() <= 100);
///
/// let vec = fate.roll(dice::vec_deque(&elem_die, ..=73));
/// assert!(vec.len() <= 73);
///
/// let vec = fate.roll(dice::vec_deque(&elem_die, 17..));
/// assert!(vec.len() >= 17);
///
/// let vec = fate.roll(dice::vec_deque(&elem_die, 42));
/// assert!(vec.len() == 42);
/// ```
pub fn vec_deque<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<VecDeque<T>> {
    dice::collection(VecDequeBuilder::die(), elem_die, len_range)
}

/// Similar to `dice::vec_deque` but each element is generated using only a random part of
/// `Limit`.
///
/// If you want to generate a `VecDeque` that contains other collections, then you should
/// consider using this generator for the outer `VecDeque`. That way the overall size is
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
/// let elem_die = dice::u8(..);
/// let vec_die = dice::vec_deque(elem_die, ..);
/// let vec_of_vecs_die = dice::outer_vec_deque(vec_die, ..);
///
/// let vec_of_vecs = fate.roll(vec_of_vecs_die);
/// assert!(vec_of_vecs.iter().flatten().count() <= 100);
/// ```
pub fn outer_vec_deque<T>(
    elem_die: impl Die<T>,
    len_range: impl SizeRange,
) -> impl Die<VecDeque<T>> {
    dice::outer_collection(VecDequeBuilder::die(), elem_die, len_range)
}
