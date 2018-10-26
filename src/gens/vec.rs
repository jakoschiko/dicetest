use ::prelude::gens::*;
use ::gens::{SizeRange, CollectionBuilder};

#[derive(Clone)]
struct VecBuilder;

impl<T> CollectionBuilder<T, Vec<T>> for VecBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(elems.len());
        vec.extend(elems);
        vec
    }
}

/// Generates a `Vec` that contains elements of type `T`.
///
/// The range specifies the length of the `Vec`.
pub fn vec<T>(
    elem_gen: impl Gen<T>,
    len_range: impl SizeRange,
) -> impl Gen<Vec<T>> {
    let builder_gen = gens::just(VecBuilder);
    gens::collection(builder_gen, elem_gen, len_range)
}
