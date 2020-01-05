use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`Vec`] builder for [`dice::collection`].
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`dice::collection`]: fn.collection.html
pub struct VecBuilder;

impl<T> CollectionBuilder<T, Vec<T>> for VecBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(elems.len());
        vec.extend(elems);
        vec
    }
}

/// Generates a [`Vec`] that contains elements of type `T`.
///
/// The range specifies the length of the [`Vec`].
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn vec<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<Vec<T>> {
    let builder_die = dice::from_fn(|_fate| VecBuilder);
    dice::collection(builder_die, elem_die, len_range)
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;

    #[test]
    fn vec_calc_stats() {
        dicetest!(Config::default().with_passes(0), |fate| {
            stat!(
                "vec(dice::bool(), ..=3)",
                "{:?}",
                dice::vec(dice::bool(), ..=3).roll(fate),
            );
        })
    }
}
