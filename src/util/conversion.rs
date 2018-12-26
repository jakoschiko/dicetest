use std::mem;

/// Converts the `u32` to bytes using little endian.
///
/// This function is a left and right inverse for `bytes_to_u64`.
pub fn u64_to_bytes(target_endian: u64) -> [u8; 8] {
    let little_endian = target_endian.to_le();
    unsafe { mem::transmute(little_endian) }
}

/// Converts the bytes to an `u64` using little endian.
///
/// This function is a left and right inverse for `u64_to_bytes`.
pub fn bytes_to_u64(bytes: [u8; 8]) -> u64 {
    let little_endian: u64 = unsafe { mem::transmute(bytes) };
    u64::from_le(little_endian)
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;
    use crate::util::conversion;
    use crate::asserts;

    #[test]
    fn bytes_to_u64_is_left_inverse() {
        dicetest!(|dice| {
            asserts::left_inverse(
                dice,
                gens::array_8(gens::u8(..)),
                conversion::bytes_to_u64,
                conversion::u64_to_bytes,
            )
        })
    }

    #[test]
    fn u64_to_bytes_is_left_inverse() {
        dicetest!(|dice| {
            asserts::left_inverse(
                dice,
                gens::u64(..),
                conversion::u64_to_bytes,
                conversion::bytes_to_u64,
            )
        })
    }

}
