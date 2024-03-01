use std::hash::{BuildHasher, Hasher, RandomState};

/// A seed for pseudorandomness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed(pub u64);

impl Seed {
    /// Creates a random seed using the random number generator of the operating system.
    ///
    /// # Panics
    ///
    /// If the operation system fails to generate the needed amount of random bytes this function
    /// will panic. See the documentation of [getrandom] for more details.
    ///
    /// [getrandom]: https://docs.rs/getrandom
    pub fn random() -> Self {
        // Hack for obtaining randomness from stdlib.
        // The idea was taken from the crate arbtest.
        let seed = RandomState::new().build_hasher().finish();
        Seed(seed)
    }
}

impl From<u64> for Seed {
    fn from(seed: u64) -> Self {
        Seed(seed)
    }
}
