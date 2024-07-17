use crate::dice::LengthRange;
use crate::prelude::*;

/// A collection builder for [`dice::collection`].
///
/// The collection has the type `C` and contains elements of type `T`.
///
/// [`dice::collection`]: dice::collection()
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
/// # Panics
///
/// Panics if the range is empty.
pub fn collection<T, C, B>(
    builder_die: impl Die<B>,
    elem_die: impl Die<T>,
    length_range: impl LengthRange,
) -> impl Die<C>
where
    B: CollectionBuilder<T, C>,
{
    let length_die = dice::length(length_range);
    dice::from_fn(move |mut fate| {
        let builder = fate.roll(&builder_die);
        let length = fate.roll(&length_die);
        let elems = (0..length).map(|_| fate.roll(&elem_die));
        builder.build(elems)
    })
}

/// Similar to [`dice::collection`] but each element is generated using only a random part of
/// [`Limit`].
///
/// If you want to generate a collection that contains other collections, then you should
/// consider using this generator for the outer collection. That way the overall length is
/// bounded by [`Limit`] (and not the square of [`Limit`]).
///
/// [`Limit`]: crate::Limit
/// [`dice::collection`]: dice::collection()
///
/// # Panics
///
/// Panics if the range is empty.
pub fn outer_collection<T, C, B>(
    builder_die: impl Die<B>,
    elem_die: impl Die<T>,
    length_range: impl LengthRange,
) -> impl Die<C>
where
    B: CollectionBuilder<T, C>,
{
    let length_die = dice::length(length_range);
    dice::from_fn(move |mut fate| {
        let builder = fate.roll(&builder_die);
        let length = fate.roll(&length_die);
        let elem_limits = if length == 0 {
            Vec::new()
        } else {
            fate.roll(dice::split_limit_n(fate.limit(), length))
        };
        let elems = elem_limits
            .into_iter()
            .map(|limit| fate.with_limit(limit).roll(&elem_die));
        builder.build(elems)
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::Limit;

    #[test]
    fn outer_collection_overall_length_is_bounded_by_limit() {
        Dicetest::repeatedly().run(|mut fate| {
            let length = fate.roll(dice::length(..));

            let limit = Limit::saturating_from_usize(length);

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
            let vec_of_vecs = fate.with_limit(limit).roll(vec_of_vecs_die);

            let overall_length = vec_of_vecs.iter().flatten().count();
            assert!(overall_length <= length);
        })
    }
}
