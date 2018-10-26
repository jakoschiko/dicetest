use std;

use ::gen::Gen;
use ::gens;

fn char_in_range(lower: u32, upper: u32) -> impl Gen<char> {
    gens::u32_uniform(lower..=upper)
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
        char_in_range(0xE000, 0x10FFFF),
    )
}

#[cfg(test)]
mod tests {
    use std;

    use ::prelude::*;

    #[test]
    fn char_ascii_alphabetic_lowercase_generates_only_valid_values() {
        assert_prop(|| {
            props::forall_1(
                gens::char_ascii_alphabetic_lowercase(),
                |_, char| char.is_ascii_alphabetic() && char.is_ascii_lowercase()
            ).dyn()
        })
    }

    #[test]
    fn char_ascii_alphabetic_uppercase_generates_only_valid_values() {
        assert_prop(|| {
            props::forall_1(
                gens::char_ascii_alphabetic_uppercase(),
                |_, char| char.is_ascii_alphabetic() && char.is_ascii_uppercase()
            ).dyn()
        })
    }

    #[test]
    fn char_ascii_alphabetic_generates_only_valid_values() {
        assert_prop(|| {
            props::forall_1(
                gens::char_ascii_alphabetic(),
                |_, char| char.is_ascii_alphabetic()
            ).dyn()
        })
    }


    #[test]
    fn char_ascii_digit_generates_only_valid_values() {
        assert_prop(|| {
            props::forall_1(
                gens::char_ascii_digit(),
                |_, char| char.is_ascii_digit()
            ).dyn()
        })
    }

    #[test]
    fn char_ascii_alphanumeric_generates_only_valid_values() {
        assert_prop(|| {
            props::forall_1(
                gens::char_ascii_alphanumeric(),
                |_, char| char.is_ascii_alphanumeric()
            ).dyn()
        })
    }

    #[test]
    fn char_ascii_printable_generates_only_valid_values() {
        assert_prop(|| {
            props::forall_1(
                gens::char_ascii_printable(),
                |_, char| char.is_ascii() && !char.is_ascii_control()
            ).dyn()
        })
    }

    #[test]
    fn char_ascii_generates_only_valid_values() {
        assert_prop(|| {
            props::forall_1(
                gens::char_ascii(),
                |_, char| char.is_ascii()
            ).dyn()
        })
    }

    #[test]
    fn char_generates_only_valid_values() {
        assert_prop(|| {
            props::forall_1(
                gens::char(),
                |_, char| std::char::from_u32(char as u32).is_some()
            ).dyn()
        })
    }
}
