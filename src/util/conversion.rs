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
    let target_endian = u64::from_le(little_endian);
    target_endian
}

#[cfg(test)]
mod tests {
    use ::prelude::*;

    #[test]
    fn bytes_to_u64_is_left_inverse() {
        assert_prop(|| {
            props::left_inverse(
                gens::array_8(gens::u8(..)),
                ::util::conversion::bytes_to_u64,
                ::util::conversion::u64_to_bytes,
            )
        })
    }

    #[test]
    fn u64_to_bytes_is_left_inverse() {
        assert_prop(|| {
            props::left_inverse(
                gens::u64(..),
                ::util::conversion::u64_to_bytes,
                ::util::conversion::bytes_to_u64,
            )
        })
    }

}
