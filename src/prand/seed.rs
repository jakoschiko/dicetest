use getrandom::getrandom;

/// A seed for pseudorandomness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed(pub u64);

impl Seed {
    /// Creates a random seed using the random number generator of the operating system.
    ///
    /// # Panics
    ///
    /// If the operation system fails to generate the needed amount of random bytes this function
    /// will panic. See the documetation of [getrandom] for more details.
    ///
    /// [getrandom]: https://docs.rs/getrandom
    pub fn random() -> Self {
        let mut buf = [0u8; 8];
        getrandom(&mut buf).expect("Random seed generation has failed");
        let seed = u64::from_le_bytes(buf);
        Seed(seed)
    }
}

impl From<u64> for Seed {
    fn from(seed: u64) -> Self {
        Seed(seed)
    }
}
