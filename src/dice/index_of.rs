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
/// let array = ['a', 'b', 'c'];
/// let index = dice::index_of(&array).sample();
/// assert!(0 <= index && index < array.len());
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
        dicetest!(|fate| {
            let vec = dice::vec(dice::u8(..), 1..).roll(fate);
            let index = dice::index_of(&vec).roll(fate);

            assert!(index < vec.len());
        })
    }
}
