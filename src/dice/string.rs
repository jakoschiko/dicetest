use crate::dice::{CollectionBuilder, LengthRange};
use crate::prelude::*;

/// [`String`] builder for [`dice::collection`].
///
/// [`dice::collection`]: dice::collection()
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
/// let char_die = dice::char();
///
/// let string = fate.with_limit(100.into()).roll(dice::string(&char_die, ..));
/// assert!(string.chars().count() <= 100);
///
/// let string = fate.roll(dice::string(&char_die, ..=73));
/// assert!(string.chars().count() <= 73);
///
/// let string = fate.roll(dice::string(&char_die, 17..));
/// assert!(string.chars().count() >= 17);
///
/// let string = fate.roll(dice::string(&char_die, 42));
/// assert!(string.chars().count() == 42);
/// ```
pub fn string(char_die: impl Die<char>, length_range: impl LengthRange) -> impl Die<String> {
    dice::collection(StringBuilder::die(), char_die, length_range)
}
