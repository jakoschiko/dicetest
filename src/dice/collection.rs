use crate::dice::SizeRange;
use crate::prelude::dice::*;

/// A collection builder for [`dice::collection`].
///
/// The collection has the type `C` and contains elements of type `T`.
///
/// [`dice::collection`]: fn.collection.html
pub trait CollectionBuilder<T, C> {
    /// Build a collection from the given elements.
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> C;
}

/// Generates a collection of type `C` that contains elements of type `T`.
///
/// The collection is created as follows:
/// * The [`CollectionBuilder`] is generated.
/// * The element count is generated using the given range.
/// * The elements are generated.
/// * The generated elements are passed to [`CollectionBuilder::build`].
///
/// [`CollectionBuilder`]: trait.CollectionBuilder.html
/// [`CollectionBuilder::build`]: trait.CollectionBuilder.html#tymethod.build
///
/// # Panics
///
/// Panics if the range is empty.
pub fn collection<T, C, B>(
    builder_die: impl Die<B>,
    elem_die: impl Die<T>,
    elem_count_range: impl SizeRange,
) -> impl Die<C>
where
    B: CollectionBuilder<T, C>,
{
    let elem_count_die = dice::size(elem_count_range);
    dice::from_fn(move |mut fate| {
        let builder = fate.roll(&builder_die);
        let elem_count = fate.roll(&elem_count_die);
        let elems = (0..elem_count).map(|_| fate.roll(&elem_die));
        builder.build(elems)
    })
}

/// Similar to `dice::collection` but each element is generated using only a random part of
/// `Limit`.
///
/// If you want to generate a collection that contains other collections, then you should
/// consider using this generator for the outer collection. That way the overall size is
/// bounded by `Limit` (and not the square of `Limit`).
///
/// # Panics
///
/// Panics if the range is empty.
pub fn outer_collection<T, C, B>(
    builder_die: impl Die<B>,
    elem_die: impl Die<T>,
    elem_count_range: impl SizeRange,
) -> impl Die<C>
where
    B: CollectionBuilder<T, C>,
{
    let elem_count_die = dice::size(elem_count_range);
    dice::from_fn(move |mut fate| {
        let builder = fate.roll(&builder_die);
        let elem_count = fate.roll(&elem_count_die);
        let elem_limits = if elem_count == 0 {
            Vec::new()
        } else {
            fate.roll(dice::terms_of_u64(fate.limit.0, elem_count))
        };
        let elems = elem_limits
            .into_iter()
            .map(|limit| elem_die.roll(fate.prng, limit.into()));
        builder.build(elems)
    })
}

#[cfg(test)]
mod tests {
    use crate::die::Limit;
    use crate::prelude::tests::*;

    #[test]
    fn outer_collection_overall_size_is_bounded_by_limit() {
        dicetest!(|mut fate| {
            let size = fate.roll(dice::size(..));

            let limit = Limit::saturating_from_usize(size);

            pub struct TestBuilder;

            impl<T> dice::CollectionBuilder<T, Vec<T>> for TestBuilder {
                fn build(self, elems: impl ExactSizeIterator<Item = T>) -> Vec<T> {
                    elems.collect()
                }
            }

            let builder_die = dice::from_fn(|_| TestBuilder);
            let elem_die = dice::u8(..);
            let vec_die = dice::collection(&builder_die, elem_die, ..);
            let vec_of_vecs_die = dice::outer_collection(&builder_die, vec_die, ..);
            let vec_of_vecs = vec_of_vecs_die.roll(fate.prng, limit);

            let overall_size = vec_of_vecs.iter().flatten().count();
            assert!(overall_size <= size);
        })
    }
}
