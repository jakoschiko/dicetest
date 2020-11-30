use std::collections::BinaryHeap;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::*;

/// [`BinaryHeap`] builder for [`dice::collection`].
///
/// [`dice::collection`]: dice::collection()
pub struct BinaryHeapBuilder;

impl BinaryHeapBuilder {
    fn die() -> impl Die<Self> {
        dice::from_fn(|_fate| Self)
    }
}

impl<T> CollectionBuilder<T, BinaryHeap<T>> for BinaryHeapBuilder
where
    T: Ord,
{
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> BinaryHeap<T> {
        let mut heap = BinaryHeap::with_capacity(elems.len());
        heap.extend(elems);
        heap
    }
}

/// Generates a [`BinaryHeap`] that contains elements of type `T`.
///
/// The range specifies the length of the [`BinaryHeap`].
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
/// let heap = fate.with_limit(100.into()).roll(dice::binary_heap(&elem_die, ..));
/// assert!(heap.len() <= 100);
///
/// let heap = fate.roll(dice::binary_heap(&elem_die, ..=73));
/// assert!(heap.len() <= 73);
///
/// let heap = fate.roll(dice::binary_heap(&elem_die, 17..));
/// assert!(heap.len() >= 17);
///
/// let heap = fate.roll(dice::binary_heap(&elem_die, 42));
/// assert!(heap.len() == 42);
/// ```
pub fn binary_heap<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<BinaryHeap<T>>
where
    T: Ord,
{
    dice::collection(BinaryHeapBuilder::die(), elem_die, len_range)
}

/// Similar to [`dice::binary_heap`] but each element is generated using only a random part of
/// [`Limit`].
///
/// If you want to generate a [`BinaryHeap`] that contains other collections, then you should
/// consider using this generator for the outer [`BinaryHeap`]. That way the overall size is
/// bounded by [`Limit`] (and not the square of [`Limit`]).
///
/// [`Limit`]: crate::Limit
/// [`dice::binary_heap`]: dice::binary_heap()
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
/// let heap_of_vecs_die = dice::outer_binary_heap(vec_die, ..);
///
/// let heap_of_vecs = fate.roll(heap_of_vecs_die);
/// assert!(heap_of_vecs.iter().flatten().count() <= 100);
/// ```
pub fn outer_binary_heap<T>(
    elem_die: impl Die<T>,
    tries_range: impl SizeRange,
) -> impl Die<BinaryHeap<T>>
where
    T: Ord,
{
    dice::outer_collection(BinaryHeapBuilder::die(), elem_die, tries_range)
}
