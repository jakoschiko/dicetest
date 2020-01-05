use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

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
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(1337.into());
/// let fate = &mut Fate::new(&mut prng, 100.into());
/// let elem_die = dice::u8(..);
///
/// let vec = dice::vec(&elem_die, ..).roll(fate);
/// assert!(vec.len() <= 100);
///
/// let vec = dice::vec(&elem_die, ..=73).roll(fate);
/// assert!(vec.len() <= 73);
///
/// let vec = dice::vec(&elem_die, 17..).roll(fate);
/// assert!(vec.len() >= 17);
///
/// let vec = dice::vec(&elem_die, 42).roll(fate);
/// assert!(vec.len() == 42);
/// ```
pub fn vec<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<Vec<T>> {
    dice::collection(VecBuilder::die(), elem_die, len_range)
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
