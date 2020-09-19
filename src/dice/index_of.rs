use crate::prelude::dice::*;

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
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
///
/// Fate::run(&mut prng, limit, |fate| {
///     let array = ['a', 'b', 'c'];
///     let index = dice::index_of(&array).roll(fate);
///     assert!(0 <= index && index < array.len());
/// });
/// ```
///
/// This example panics:
///
/// ```should_panic
/// use dicetest::prelude::dice::*;
///
/// // Oh no, panic!
/// let _index_die = dice::index_of::<char>(&[]);
/// ```
pub fn index_of<'a, 'b, T>(slice: &'a [T]) -> impl Die<usize> + 'b {
    let len = slice.len();
    dice::uni_usize(0..len)
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;

    #[test]
    fn index_of_generates_valid_index() {
        Dicetest::repeatedly().run(|fate| {
            let vec = dice::vec(dice::u8(..), 1..).roll(fate);
            let index = dice::index_of(&vec).roll(fate);

            assert!(index < vec.len());
        })
    }

    #[test]
    fn index_of_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|fate| {
                stat!(
                    "index_of(&[1, 2, 3, 4, 5])",
                    "{}",
                    dice::index_of(&[1, 2, 3, 4, 5]).roll(fate),
                );
            })
    }
}
