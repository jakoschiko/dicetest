use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::*;

/// [`Vec`] builder for [`dice::collection`].
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`dice::collection`]: fn.collection.html
pub struct VecBuilder;

impl VecBuilder {
    fn die() -> impl Die<Self> {
        dice::from_fn(|_fate| Self)
    }
}

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
/// let vec = fate.with_limit(100.into()).roll(dice::vec(&elem_die, ..));
/// assert!(vec.len() <= 100);
///
/// let vec = fate.roll(dice::vec(&elem_die, ..=73));
/// assert!(vec.len() <= 73);
///
/// let vec = fate.roll(dice::vec(&elem_die, 17..));
/// assert!(vec.len() >= 17);
///
/// let vec = fate.roll(dice::vec(&elem_die, 42));
/// assert!(vec.len() == 42);
/// ```
pub fn vec<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<Vec<T>> {
    dice::collection(VecBuilder::die(), elem_die, len_range)
}

/// Similar to `dice::vec` but each element is generated using only a random part of
/// `Limit`.
///
/// If you want to generate a `Vec` that contains other collections, then you should
/// consider using this generator for the outer `Vec`. That way the overall size is
/// bounded by `Limit` (and not the square of `Limit`).
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
/// let vec_of_vecs_die = dice::outer_vec(vec_die, ..);
///
/// let vec_of_vecs = fate.roll(vec_of_vecs_die);
/// assert!(vec_of_vecs.iter().flatten().count() <= 100);
/// ```
pub fn outer_vec<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<Vec<T>> {
    dice::outer_collection(VecBuilder::die(), elem_die, len_range)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn vec_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                stat!(
                    "vec(dice::bool(), ..=3)",
                    "{:?}",
                    fate.roll(dice::vec(dice::bool(), ..=3)),
                );
            })
    }
}
