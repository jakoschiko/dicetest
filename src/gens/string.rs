use crate::gens::{CollectionBuilder, SizeRange};
use crate::prelude::gens::*;

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
pub fn string(char_gen: impl Gen<char>, len_range: impl SizeRange) -> impl Gen<String> {
    let builder_gen = gens::just(StringBuilder);
    gens::collection(builder_gen, char_gen, len_range)
}
