use crate::prelude::gens::*;

fn char_in_range(lower: u32, upper: u32) -> impl Gen<char> {
    gens::uni_u32(lower..=upper)
        .map(std::char::from_u32)
        .map(Option::unwrap)
}

/// Generator for lowercase letters from the ASCII alphabet ('a' to 'z').
pub fn char_ascii_alphabetic_lowercase() -> impl Gen<char> {
    char_in_range('a' as u32, 'z' as u32)
}

/// Generator for uppercase letters from the ASCII alphabet ('A' to 'Z').
pub fn char_ascii_alphabetic_uppercase() -> impl Gen<char> {
    char_in_range('A' as u32, 'Z' as u32)
}

/// Generator for letters from the ASCII alphabet ('a' to 'z' and 'A' to 'Z').
///
/// It always succeeds and has no `Shrink`.
pub fn char_ascii_alphabetic() -> impl Gen<char> {
    gens::one_of_gen_2(
        char_ascii_alphabetic_lowercase(),
        char_ascii_alphabetic_uppercase(),
    )
}

/// Generator for ASCII digits ('0' to '9').
pub fn char_ascii_digit() -> impl Gen<char> {
    char_in_range('0' as u32, '9' as u32)
}

/// Generator for lowercase and uppercase letters from
/// the ASCII alphabet ('a' to 'z' and 'A' to 'Z') and
/// ASCII digits ('0' to '9').
pub fn char_ascii_alphanumeric() -> impl Gen<char> {
    gens::one_of_gen_3(
        char_ascii_alphabetic_lowercase(),
        char_ascii_alphabetic_uppercase(),
        char_ascii_digit(),
    )
}

/// Generator for printable ASCII characters (see [wikipedia]).
///
/// [wikipedia]: https://en.wikipedia.org/wiki/ASCII#Printable_characters
pub fn char_ascii_printable() -> impl Gen<char> {
    char_in_range(0x20, 0x7E)
}

/// Generator for ASCII characters (see [wikipedia]).
/// Note that not all characters are printable.
///
/// [wikipedia]: https://en.wikipedia.org/wiki/ASCII
pub fn char_ascii() -> impl Gen<char> {
    char_in_range(0x0, 0x7F)
}

/// Generator for valid [`char`]s. A [`char`] represents an
/// unicode scalar value (see [here for definition][1]).
///
/// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
/// [1]: http://www.unicode.org/glossary/#unicode_scalar_value
pub fn char() -> impl Gen<char> {
    gens::one_of_gen_2(
        char_in_range(0x0, 0xD7FF),
        char_in_range(0xE000, 0x0010_FFFF),
    )
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;

    #[test]
    fn char_ascii_alphabetic_lowercase_generates_only_valid_values() {
        dicetest!(|fate| {
            let char = gens::char_ascii_alphabetic_lowercase().gen(fate);

            hint!(char);

            assert!(char.is_ascii_alphabetic());
            assert!(char.is_ascii_lowercase());
        })
    }

    #[test]
    fn char_ascii_alphabetic_uppercase_generates_only_valid_values() {
        dicetest!(|fate| {
            let char = gens::char_ascii_alphabetic_uppercase().gen(fate);

            hint!(char);

            assert!(char.is_ascii_alphabetic());
            assert!(char.is_ascii_uppercase());
        })
    }

    #[test]
    fn char_ascii_alphabetic_generates_only_valid_values() {
        dicetest!(|fate| {
            let char = gens::char_ascii_alphabetic().gen(fate);

            hint!(char);

            assert!(char.is_ascii_alphabetic());
        })
    }

    #[test]
    fn char_ascii_digit_generates_only_valid_values() {
        dicetest!(|fate| {
            let char = gens::char_ascii_digit().gen(fate);

            hint!(char);

            assert!(char.is_ascii_digit());
        })
    }

    #[test]
    fn char_ascii_alphanumeric_generates_only_valid_values() {
        dicetest!(|fate| {
            let char = gens::char_ascii_alphanumeric().gen(fate);

            hint!(char);

            assert!(char.is_ascii_alphanumeric());
        })
    }

    #[test]
    fn char_ascii_printable_generates_only_valid_values() {
        dicetest!(|fate| {
            let char = gens::char_ascii_printable().gen(fate);

            hint!(char);

            assert!(char.is_ascii());
            assert!(!char.is_ascii_control());
        })
    }

    #[test]
    fn char_ascii_generates_only_valid_values() {
        dicetest!(|fate| {
            let char = gens::char_ascii().gen(fate);

            hint!(char);

            assert!(char.is_ascii());
        })
    }

    #[test]
    fn char_generates_only_valid_values() {
        dicetest!(|fate| {
            let char = gens::char().gen(fate);

            hint!(char);

            assert!(std::char::from_u32(char as u32).is_some());
        })
    }
}
