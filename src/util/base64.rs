//! Provides the traditional (MIME) base64 encoding and decoding.
//!
//! The algorithms are based on the examples from [Wikipedia].
//!
//! [Wikipedia]: https://en.wikibooks.org/wiki/Algorithm_Implementation/Miscellaneous/Base64

use std::iter;

const BYTE_TO_CHAR: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '+', '/',
];

#[allow(clippy::zero_prefixed_literal)]
const CHAR_TO_BYTE: [u8; 256] = [
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 62, 64, 64, 64, 63,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 64, 64, 64, 64, 64, 64,
    64, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14,
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 64, 64, 64, 64, 64,
    64, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64
];

const PAD_CHAR: char = '=';

fn is_invalid_char(c: char) -> bool {
    (c as u32) >= 256 || CHAR_TO_BYTE[c as usize] >= 64
}

/// Converts the given bytes to a base64 string.
///
/// This function is a right inverse for `decode`.
pub fn encode(bytes: &[u8]) -> String {
    let pad_count = {
        let outrange = bytes.len() % 3;
        let pad_count = (3 - outrange) % 3;
        pad_count as usize
    };

    let bytes_padding = iter::repeat_with(|| &0).take(pad_count);

    let mut i = bytes
        .into_iter()
        .chain(bytes_padding);

    let mut result = String::new();

    while let (Some(b0), Some(b1), Some(b2)) = (i.next(), i.next(), i.next()) {
        let n =
            ((u32::from(*b0)) << 16) |
            ((u32::from(*b1)) << 8) |
            u32::from(*b2);

        let n0 = (n >> 18) & 63;
        let n1 = (n >> 12) & 63;
        let n2 = (n >> 6) & 63;
        let n3 = n & 63;

        result.push(BYTE_TO_CHAR[n0 as usize]);
        result.push(BYTE_TO_CHAR[n1 as usize]);
        result.push(BYTE_TO_CHAR[n2 as usize]);
        result.push(BYTE_TO_CHAR[n3 as usize]);
    }

    let result_len = result.len();
    result.truncate(result_len - pad_count);

    let result_padding = PAD_CHAR.to_string().repeat(pad_count);
    result.push_str(&result_padding);

    result
}

/// Tries to convert the given base64 string to bytes.
/// Fails if the string has no valid base64 encoding.
///
/// This function is a left inverse for `encode`.
pub fn decode(base64: &str) -> Result<Vec<u8>, String> {
    let suffix =
        if base64.ends_with("==") { "AA" }
        else if base64.ends_with('=') { "A" }
        else { "" };

    let prefix = &base64[0..base64.len() - suffix.len()];

    let invalid_char = prefix
        .chars()
        .find(|&c| is_invalid_char(c));

    if let Some(invalid_char) = invalid_char {
        return Err(format!("Base64 string contains invalid character: {:?}", invalid_char))
    }

    let has_invalid_length = (prefix.len() + suffix.len()) % 4 != 0;

    if has_invalid_length {
        return Err("Base64 string has invalid length".to_string());
    }

    let all = prefix.chars().chain(suffix.chars());
    let mut i = all.map(|c| CHAR_TO_BYTE[c as usize]);

    let mut result = Vec::new();

    while let (Some(n0), Some(n1), Some(n2), Some(n3)) = (i.next(), i.next(), i.next(), i.next()) {
        let n = (u32::from(n0) << 18)
            | (u32::from(n1) << 12)
            | (u32::from(n2) << 6)
            | u32::from(n3);

        let b1 = ((n >> 16) & 0xFF) as u8;
        let b2 = ((n >> 8) & 0xFF) as u8;
        let b3 = (n & 0xFF) as u8;

        result.extend_from_slice(&[b1, b2, b3]);
    }

    for _ in 0..suffix.len() {
        result.pop();
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;
    use crate::util::base64;

    #[test]
    fn encode_produces_empty_string_if_bytes_is_empty() {
        let bytes = Vec::new();
        let base64 = base64::encode(&bytes);
        assert_eq!(base64, "");
    }

    #[test]
    fn encode_produces_non_empty_string_if_bytes_is_non_empty() {
        dicetest!(|dice| {
            let bytes = gens::vec(gens::u8(..), 1..).gen(dice);
            let base64 = base64::encode(&bytes);

            hint_dbg!(bytes);
            hint_dbg!(&base64);

            assert!(!base64.is_empty())
        })
    }

    #[test]
    fn decode_is_left_inverse() {
        dicetest!(|dice| {
            let bytes = gens::vec(gens::u8(..), ..).gen(dice);
            let base64 = base64::encode(&bytes);

            hint_dbg!(&bytes);
            hint_dbg!(&base64);

            let decoded_bytes = base64::decode(&base64).unwrap();

            assert_eq!(bytes, decoded_bytes);
        })
    }

    #[test]
    fn decode_fails_if_string_contains_invalid_char() {
        dicetest!(|dice| {
            let valid_len_gen = gens::size(4..).map(|len| len - (len % 4));

            let len = valid_len_gen.gen(dice);
            let base64 = gens::string(gens::char(), len).gen(dice);

            let is_invalid = base64.chars().any(base64::is_invalid_char);

            if !is_invalid {
                return;
            }

            let bytes = base64::decode(&base64);

            hint_dbg!(&base64);
            hint_dbg!(&bytes);

            assert!(bytes.is_err());
        })
    }

    #[test]
    fn decode_fails_if_string_has_invalid_length() {
        dicetest!(|dice| {
            let base_64_char_gen = gens::one_of_array(&base64::BYTE_TO_CHAR);
            let invalid_len_gen = gens::size(1..)
                .map(|len| if len % 4 == 0 { len + 1 } else { len } );

            let len = invalid_len_gen.gen(dice);
            let invalid_base64 = gens::string(base_64_char_gen, len).gen(dice);

            let bytes = base64::decode(&invalid_base64);

            hint_dbg!(&invalid_base64);
            hint_dbg!(&bytes);

            assert!(bytes.is_err());
        })
    }
}
