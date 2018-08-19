use std::num::Wrapping;

use rand::{self, Rng as LibRng};

use ::rng::conversion;

/// Deterministic generator for pseudo random numbers.
///
/// The algorithms are based on [this article] by Bob Jenkins.
///
/// [this article]: http://burtleburtle.net/bob/rand/smallprng.html
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rng {
    seed: (u64, u64, u64, u64),
}

impl Rng {
    /// Creates a `Rng` using a `u64` as seed.
    /// The result has a satisfying cycle length.
    pub fn init(seed_u64: u64) -> Rng {
        let seed = (0xf1ea5eed, seed_u64, seed_u64, seed_u64);
        let mut rng = Rng { seed };
        for _ in 0..20 {
            rng.next();
        }
        rng
    }

    /// Creates a `Rng` using a random seed.
    /// The result has a satisfying cycle length.
    pub fn random() -> Rng {
        let seed = rand::thread_rng().gen();
        Rng::init(seed)
    }

    /// Creates a `Rng` using a byte slice as seed.
    /// There is no guarantee for a satisfying cycle length.
    ///
    /// This function is a right inverse for `Rng::seed_as_bytes`.
    pub fn init_with_bytes(seed_bytes: &[u8]) -> Rng {
        let mut iter = conversion::u8s_to_u64s(seed_bytes).into_iter();

        let mut rng = {
            let mut next = || iter.next().unwrap_or(0);
            let seed = (next(), next(), next(), next());
            Rng { seed }
        };

        while let Some(i) = iter.next() {
            rng.reseed(i);
        };

        rng
    }

    /// Returns the seed as a byte array.
    ///
    /// This function is a right inverse for `Rng::init_with_bytes`.
    pub fn seed_as_bytes(&self) -> Vec<u8> {
        let (a, b, c, d) = self.seed;
        conversion::u64s_to_u8s(&[a, b, c, d])
    }

    /// Returns the next pseudo random numver.
    pub fn next(&mut self) -> u64 {
        let (a, b, c, d) = self.seed;

        // We use `Wrapping` because overflow and underflow is intended
        let Wrapping(e) = Wrapping(a) - Wrapping(b.rotate_left(7));
        let Wrapping(f) = Wrapping(b) ^ Wrapping(c.rotate_left(13));
        let Wrapping(g) = Wrapping(c) + Wrapping(d.rotate_left(37));
        let Wrapping(h) = Wrapping(d) + Wrapping(e);
        let Wrapping(i) = Wrapping(e) + Wrapping(a);

        self.seed = (f, g, h, i);
        i
    }

    /// Creates a completely different seed from the current seed and the given `u64` value.
    ///
    /// The implementation is inspired by [ScalaCheck] and propably there is no statistical
    /// foundation for this.
    ///
    /// [ScalaCheck]: https://github.com/rickynils/scalacheck
    pub fn reseed(&mut self, n: u64) {
        let (a, b, c, d) = self.seed;

        let n0 = (n >> 32) & 0xffffffff;
        let n1 = n & 0xffffffff;

        self.seed = (a ^ n0, b ^ n1, c, d);

        for _ in 0..16 {
            self.next();
        }
    }

    // TODO: doc
    // TODO: test
    pub fn fork(&mut self) -> Rng {
        let random_number = self.next();
        let mut reseeded_rng = self.clone();
        reseeded_rng.reseed(random_number);
        reseeded_rng
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_must_no_have_cycle_lenght_zero() {
        // TODO: impl test
    }

    #[test]
    fn init_with_bytes_is_right_inverse() {
        // TODO: impl test
    }

    #[test]
    fn reseed_must_change_next_random_number() {
        // TODO: impl test
    }
}
