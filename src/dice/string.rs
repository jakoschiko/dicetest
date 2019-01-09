use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

#[derive(Clone)]
struct StringBuilder;

impl CollectionBuilder<char, String> for StringBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = char>) -> String {
        let mut string = String::with_capacity(elems.len());
        string.extend(elems);
        string
    }
}

/// Generates a `String` that contains the specified `chars`.
///
/// The range specifies the length of the `String`.
pub fn string(char_die: impl Die<char>, len_range: impl SizeRange) -> impl Die<String> {
    let builder_die = dice::just(StringBuilder);
    dice::collection(builder_die, char_die, len_range)
}
