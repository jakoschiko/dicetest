/// Converts the [`u32`] to bytes using little endian.
///
/// This function is a left and right inverse for [`bytes_to_u64`].
pub fn u64_to_bytes(u64: u64) -> [u8; 8] {
    u64.to_le_bytes()
}

/// Converts the bytes to an [`u64`] using little endian.
///
/// This function is a left and right inverse for [`u64_to_bytes`].
pub fn bytes_to_u64(bytes: [u8; 8]) -> u64 {
    u64::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use crate::asserts;
    use crate::prelude::*;
    use crate::util::conversion;

    #[test]
    fn bytes_to_u64_is_left_inverse() {
        Dicetest::repeatedly().run(|fate| {
            asserts::left_inverse(
                fate,
                dice::array(dice::u8(..)),
                conversion::bytes_to_u64,
                conversion::u64_to_bytes,
            )
        })
    }

    #[test]
    fn u64_to_bytes_is_left_inverse() {
        Dicetest::repeatedly().run(|fate| {
            asserts::left_inverse(
                fate,
                dice::u64(..),
                conversion::u64_to_bytes,
                conversion::bytes_to_u64,
            )
        })
    }
}
