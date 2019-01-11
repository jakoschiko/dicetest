use crate::dice::SizeRange;
use crate::prelude::dice::*;

/// A collection builder for `dice::collection`.
pub trait CollectionBuilder<T, C> {
    /// Build a collection from the given elements.
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> C;
}

/// Generates a collection of type `C` that contains elements of type `T`.
///
/// The collection is created as follows:
/// * The `CollectionBuilder` is generated.
/// * The element count is generated using the given range.
/// * The elements are generated and passed to the `CollectionBuilder`.
pub fn collection<T, C, B>(
    builder_die: impl Die<B>,
    elem_die: impl Die<T>,
    elem_count_range: impl SizeRange,
) -> impl Die<C>
where
    B: CollectionBuilder<T, C>,
{
    let elem_count_die = dice::size(elem_count_range);
    dice::from_fn(move |fate| {
        let builder = builder_die.roll(fate);
        let elem_count = elem_count_die.roll(fate);
        let elems = (0..elem_count).map(|_| elem_die.roll(fate));
        builder.build(elems)
    })
}
