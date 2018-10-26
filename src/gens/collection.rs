use ::gen::Gen;
use ::gens::{self, SizeRange};

/// Builds collections for `gens::collection`.
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
    builder_gen: impl Gen<B>,
    elem_gen: impl Gen<T>,
    elem_count_range: impl SizeRange,
) -> impl Gen<C>
where
    B: CollectionBuilder<T, C>,
{
    let elem_count_gen = gens::size(elem_count_range);
    gens::from_fn(move |rng, lim| {
        let builder = builder_gen.gen(rng, lim);
        let elem_count = elem_count_gen.gen(rng, lim);
        let elems = (0..elem_count).map(|_| elem_gen.gen(rng, lim));
        builder.build(elems)
    })
}
