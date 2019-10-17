use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`String`] builder for [`dice::collection`].
///
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`dice::collection`]: fn.collection.html
pub struct StringBuilder;

impl CollectionBuilder<char, String> for StringBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = char>) -> String {
        let mut string = String::with_capacity(elems.len());
        string.extend(elems);
        string
    }
}

/// Generates a [`String`] that contains the specified [`char`]s.
///
/// The range specifies the length of the [`String`].
///
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn string(char_die: impl Die<char>, len_range: impl SizeRange) -> impl Die<String> {
    let builder_die = dice::from_fn(|_fate| StringBuilder);
    dice::collection(builder_die, char_die, len_range)
}
