//! Provides [base62] encoding and decoding.
//!
//! We use base62 because its string representation is easy to select in many
//! terminals. Encodings like base64 contain symbols that might prevent the
//! user from selecting the whole string via double-click or other mechanisms.
//!
//! [base62]: https://en.wikipedia.org/wiki/Base62

use std::collections::VecDeque;

/// Converts the given bytes to a base62 string.
///
/// This function is a right inverse for [`decode`].
///
/// Warning: Performance is really bad, probably O(n*n), but it's good enough
/// for our use case.
pub fn encode(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let mut bytes_iter = bytes.iter().copied();
    let mut remaining_bytes = VecDeque::new();
    let mut base62 = String::new();
    let mut digits = Vec::new();

    while let Some(byte) = bytes_iter.next() {
        if byte == 0 {
            base62.push('0');
        } else {
            remaining_bytes.push_back(byte);
            remaining_bytes.extend(bytes_iter);
            break;
        }
    }

    while !remaining_bytes.is_empty() {
        let mut digit = 0;

        for byte in &mut remaining_bytes {
            let temp = ((digit as u16) << 8) + (*byte as u16);
            *byte = (temp / 62) as u8;
            digit = (temp % 62) as u8;
        }

        digits.push(digit);

        while let Some(&byte) = remaining_bytes.front() {
            if byte == 0 {
                remaining_bytes.pop_front();
            } else {
                break;
            }
        }
    }

    for &digit in digits.iter().rev() {
        let offset = match digit {
            0..=9 => b'0',
            10..=35 => b'A' - 10,
            36.. => b'a' - 36,
        };
        let char = (digit + offset) as char;

        base62.push(char);
    }

    base62
}

/// Tries to convert the given base62 string to bytes.
/// Fails if the string has no valid base62 encoding.
///
/// This function is a left inverse for [`encode`].
///
/// Warning: Performance is really bad, probably O(n*n), but it's good enough
/// for our use case.
pub fn decode(base62: &str) -> Result<Vec<u8>, String> {
    if base62.is_empty() {
        return Ok(Vec::new());
    }

    let mut bytes = Vec::new();
    let mut additional_bytes: Vec<u8> = Vec::new();

    for char in base62.chars() {
        if additional_bytes.is_empty() && char == '0' {
            bytes.push(0);
        } else {
            let offset = match char {
                '0'..='9' => b'0',
                'A'..='Z' => b'A' - 10,
                'a'..='z' => b'a' - 36,
                _ => {
                    return Err(format!(
                        "Base64 string contains invalid character: {char:?}"
                    ));
                }
            };

            let mut digit = char as u32 - offset as u32;

            for byte in additional_bytes.iter_mut().rev() {
                let temp = (*byte as u32) * 62 + digit;
                *byte = temp as u8;
                digit = temp >> 8;
            }

            while digit > 0 {
                additional_bytes.insert(0, digit as u8);
                digit >>= 8;
            }
        }
    }

    bytes.extend_from_slice(&additional_bytes);

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::util::base62;

    #[test]
    fn encode_examples() {
        assert_eq!(base62::encode(&[]), "");
        assert_eq!(base62::encode(&[0]), "0");
        assert_eq!(base62::encode(&[0, 0]), "00");
        assert_eq!(base62::encode(&[0, 0, 0, 0, 0]), "00000");
        assert_eq!(base62::encode(&[0, 0, 0, 0, 0, 0, 0, 0]), "00000000");
        assert_eq!(base62::encode(&[1]), "1");
        assert_eq!(base62::encode(&[1, 1]), "49");
        assert_eq!(base62::encode(&[1, 1, 1, 1, 1]), "4hnupV");
        assert_eq!(base62::encode(&[1, 1, 1, 1, 1, 1, 1,]), "1IExPTpE9");
        assert_eq!(base62::encode(&[2]), "2");
        assert_eq!(base62::encode(&[2, 2]), "8I");
        assert_eq!(base62::encode(&[61]), "z");
        assert_eq!(base62::encode(&[62]), "10");
        assert_eq!(base62::encode(&[63]), "11");
        assert_eq!(base62::encode(&[255]), "47");
        assert_eq!(base62::encode(&[255, 255]), "H31");
    }

    #[test]
    fn encode_produces_non_empty_string_if_bytes_is_non_empty() {
        Dicetest::repeatedly().run(|mut fate| {
            let bytes = fate.roll(dice::vec(dice::u8(..), 1..));
            let base62 = base62::encode(&bytes);

            hint_debug!(bytes);
            hint_debug!(base62);

            assert!(!base62.is_empty())
        })
    }

    #[test]
    fn decode_examples() {
        assert_eq!(base62::decode(""), Ok(vec![]));
        assert_eq!(base62::decode("0"), Ok(vec![0]));
        assert_eq!(base62::decode("00"), Ok(vec![0, 0]));
        assert_eq!(base62::decode("000"), Ok(vec![0, 0, 0]));
        assert_eq!(base62::decode("0000"), Ok(vec![0, 0, 0, 0]));
        assert_eq!(base62::decode("1"), Ok(vec![1]));
        assert_eq!(base62::decode("2"), Ok(vec![2]));
        assert_eq!(base62::decode("10"), Ok(vec![62]));
        assert_eq!(base62::decode("01"), Ok(vec![0, 1]));
        assert_eq!(base62::decode("FF"), Ok(vec![3, 177]));
        assert_eq!(
            base62::decode("dicetest"),
            Ok(vec![127, 55, 202, 128, 230, 115])
        );
    }

    #[test]
    fn decode_is_left_inverse() {
        Dicetest::repeatedly().run(|mut fate| {
            let bytes = fate.roll(dice::vec(dice::u8(..), ..));
            let base62 = base62::encode(&bytes);

            hint_debug!(bytes);
            hint_debug!(base62);

            let decoded_bytes = base62::decode(&base62).unwrap();

            hint_debug!(decoded_bytes);

            assert_eq!(bytes, decoded_bytes);
        })
    }

    #[test]
    fn decode_fails_if_string_contains_invalid_char() {
        Dicetest::repeatedly().run(|mut fate| {
            let base62 = fate.roll(dice::string(dice::char(), ..));

            let is_valid = base62
                .chars()
                .all(|char| matches!(char, '0'..='9' | 'A'..='Z' | 'a'..='z'));

            if is_valid {
                return;
            }

            let bytes = base62::decode(&base62);

            hint_debug!(base62);
            hint_debug!(bytes);

            assert!(bytes.is_err());
        })
    }
}
