use std::mem;

/// Converts the `u32` to bytes using little endian.
pub fn u64_to_bytes(target_endian: u64) -> [u8; 8] {
    let little_endian = target_endian.to_le();
    unsafe { mem::transmute(little_endian) }
}

/// Converts the bytes to an `u64` using little endian.
pub fn bytes_to_u64(bytes: [u8; 8]) -> u64 {
    let little_endian: u64 = unsafe { mem::transmute(bytes) };
    let target_endian = u64::from_le(little_endian);
    target_endian
}

#[cfg(test)]
mod tests {
    #[test]
    fn bytes_to_u64_is_left_inverse() {
        // TODO: impl test
    }

    #[test]
    fn u64_to_bytes_is_left_inverse() {
        // TODO: impl test
    }
}
