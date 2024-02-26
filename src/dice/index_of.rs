use crate::prelude::*;

/// Generates an valid index for the given slice.
///
/// # Panics
///
/// Panics if the slice empty.
///
/// # Examples
///
/// This example generates an index without panicking:
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let array = ['a', 'b', 'c'];
/// let index = fate.roll(dice::index_of(&array));
/// assert!(0 <= index && index < array.len());
/// ```
///
/// This example panics:
///
/// ```should_panic
/// use dicetest::prelude::*;
///
/// // Oh no, panic!
/// let _index_die = dice::index_of::<char>(&[]);
/// ```
pub fn index_of<'a, T>(slice: &[T]) -> impl Die<usize> + 'a {
    let len = slice.len();
    dice::uni_usize(0..len)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn index_of_generates_valid_index() {
        Dicetest::repeatedly().run(|mut fate| {
            let vec = fate.roll(dice::vec(dice::u8(..), 1..));
            let index = fate.roll(dice::index_of(&vec));

            assert!(index < vec.len());
        })
    }

    #[test]
    fn index_of_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                stat!(
                    "index_of(&[1, 2, 3, 4, 5])",
                    "{}",
                    fate.roll(dice::index_of(&[1, 2, 3, 4, 5])),
                );
            })
    }
}
