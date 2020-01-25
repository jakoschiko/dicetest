use std::collections::BinaryHeap;

use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`BinaryHeap`] builder for [`dice::collection`].
///
/// [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
/// [`dice::collection`]: fn.collection.html
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
/// [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
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
/// Fate::run(
///     &mut Prng::from_seed(0x5EED.into()),
///     Default::default(),
///     |fate| {
///         let elem_die = dice::u8(..);
///
///         let heap = dice::binary_heap(&elem_die, ..).roll(fate);
///         assert!(heap.len() <= 100);
///
///         let heap = dice::binary_heap(&elem_die, ..=73).roll(fate);
///         assert!(heap.len() <= 73);
///
///         let heap = dice::binary_heap(&elem_die, 17..).roll(fate);
///         assert!(heap.len() >= 17);
///
///         let heap = dice::binary_heap(&elem_die, 42).roll(fate);
///         assert!(heap.len() == 42);
///     }
/// );
/// ```
pub fn binary_heap<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<BinaryHeap<T>>
where
    T: Ord,
{
    dice::collection(BinaryHeapBuilder::die(), elem_die, len_range)
}

/// Similar to `dice::binary_heap` but each element is generated using only a random part of
/// `Limit`.
///
/// If you want to generate a `BinaryHeap` that contains other collections, then you should
/// consider using this generator for the outer `BinaryHeap`. That way the overall size is
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
/// Fate::run(
///     &mut Prng::from_seed(0x5EED.into()),
///     Default::default(),
///     |fate| {
///         let elem_die = dice::u8(..);
///         let vec_die = dice::vec(elem_die, ..);
///         let heap_of_vecs_die = dice::outer_binary_heap(vec_die, ..);
///
///         let heap_of_vecs = heap_of_vecs_die.roll(fate);
///         assert!(heap_of_vecs.iter().flatten().count() <= 100);
///     }
/// );
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
