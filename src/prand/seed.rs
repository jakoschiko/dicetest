use rand::{self, Rng};

/// A seed for pseudorandomness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed(pub u64);

impl Seed {
    /// Creates a random seed.
    pub fn random() -> Self {
        let seed = rand::thread_rng().gen();
        Seed(seed)
    }
}

impl From<u64> for Seed {
    fn from(seed: u64) -> Self {
        Seed(seed)
    }
}
