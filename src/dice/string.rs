use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`String`] builder for [`dice::collection`].
///
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`dice::collection`]: fn.collection.html
pub struct StringBuilder;

impl StringBuilder {
    fn die() -> impl Die<Self> {
        dice::from_fn(|_fate| Self)
    }
}

impl CollectionBuilder<char, String> for StringBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = char>) -> String {
        let mut string = String::with_capacity(elems.len());
        string.extend(elems);
        string
    }
}

/// Generates a [`String`] that contains the specified [`char`]s.
///
/// The range specifies the number of [`char`]s in the [`String`].
///
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
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
/// Fate::run(
///     &mut Prng::from_seed(0x5EED.into()),
///     Default::default(),
///     |fate| {
///         let char_die = dice::char();
///
///         let string = dice::string(&char_die, ..).roll(fate);
///         assert!(string.chars().count() <= 100);
///
///         let string = dice::string(&char_die, ..=73).roll(fate);
///         assert!(string.chars().count() <= 73);
///
///         let string = dice::string(&char_die, 17..).roll(fate);
///         assert!(string.chars().count() >= 17);
///
///         let string = dice::string(&char_die, 42).roll(fate);
///         assert!(string.chars().count() == 42);
///     }
/// );
/// ```
pub fn string(char_die: impl Die<char>, len_range: impl SizeRange) -> impl Die<String> {
    dice::collection(StringBuilder::die(), char_die, len_range)
}
