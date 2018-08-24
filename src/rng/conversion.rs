use std::mem;

/// Converts the `u64`s to `u8`s using little endian.
pub fn u64s_to_u8s(u64s: &[u64]) -> Vec<u8> {
    let iter = u64s
        .into_iter()
        .flat_map(|u64| u64_to_u8s(*u64).to_vec().into_iter());

    iter.collect()
}

/// Converts the `u8`s to `u64`s using little endian. If there are not enough `u8`s for the last
/// `u64`, additional zeros will be used.
pub fn u8s_to_u64s(bytes: &[u8]) -> Vec<u64> {
    let chunks = bytes.chunks(8);

    let iter = chunks.map(|chunk| {
        let get = |index| chunk.get(index).map_or(0, |int| *int);
        let array = [get(0), get(1), get(2), get(3), get(4), get(5), get(6), get(7)];
        u8s_to_u64(array)
    });

    iter.collect()
}

/// Converts the single `u64` to eight `u8`s using little endian.
fn u64_to_u8s(native: u64) -> [u8; 8] {
    let little_endian = native.to_le();
    unsafe { mem::transmute(little_endian) }
}

/// Converts the eight `u8`s to a single `u64` using little endian.
fn u8s_to_u64(bytes: [u8; 8]) -> u64 {
    let little_endian: u64 = unsafe { mem::transmute(bytes) };
    let native = u64::from_le(little_endian);
    native
}

#[cfg(test)]
mod tests {
    #[test]
    fn u8s_to_u64_is_left_inverse() {
        // TODO: impl test
    }

    #[test]
    fn u64_to_u8s_is_left_inverse() {
        // TODO: impl test
    }

    #[test]
    fn u8s_to_u64s_is_left_inverse() {
        // TODO: impl test
    }

    #[test]
    fn u64s_to_u8s_is_left_inverse_with_additional_zeros() {
        // TODO: impl test
    }
}
