use std::hash::BuildHasher;
#[allow(deprecated)]
use std::hash::SipHasher;
use std::mem;
use std::num::Wrapping;

use rand::{self, Rng};

use crate::util::conversion;

/// This pseudo random number generator is the base for more complex random value generators.
///
/// The algorithms are based on [this article] by Bob Jenkins.
///
/// [this article]: http://burtleburtle.net/bob/rand/smallprng.html
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Prng {
    seed: (u64, u64, u64, u64),
}

impl Prng {
    /// Creates an `Prng` using a `u64` as seed.
    ///
    /// The result has a satisfying cycle length.
    pub fn init(seed_u64: u64) -> Prng {
        let seed = (0xf1ea_5eed, seed_u64, seed_u64, seed_u64);
        let mut prng = Prng { seed };
        for _ in 0..20 {
            prng.next_number();
        }
        prng
    }

    /// Creates an `Prng` using a random seed.
    ///
    /// The result has a satisfying cycle length.
    pub fn random() -> Prng {
        let seed = rand::thread_rng().gen();
        Prng::init(seed)
    }

    /// Creates an `Prng` using a byte array as seed.
    ///
    /// This function is a left and right inverse for `Prng::seed_as_bytes`.
    ///
    /// A satisfying cycle length is only guaranteed for bytes from `Prng::seed_as_bytes` called
    /// with an `Prng` that has a satisfying cycle length. Other bytes should not be passed to this
    /// function. For initializing an `Prng` with an arbitrary seed, use `Prng::init` instead.
    pub fn init_with_bytes(seed_bytes: [u8; 32]) -> Prng {
        let arrays: [[u8; 8]; 4] = unsafe { mem::transmute(seed_bytes) };

        let a = conversion::bytes_to_u64(arrays[0]);
        let b = conversion::bytes_to_u64(arrays[1]);
        let c = conversion::bytes_to_u64(arrays[2]);
        let d = conversion::bytes_to_u64(arrays[3]);

        let seed = (a, b, c, d);
        Prng { seed }
    }

    /// Returns the seed as a byte array.
    ///
    /// This function is a left and right inverse for `Prng::init_with_bytes`.
    pub fn seed_as_bytes(&self) -> [u8; 32] {
        let (a, b, c, d) = self.seed;

        let arrays = [
            conversion::u64_to_bytes(a),
            conversion::u64_to_bytes(b),
            conversion::u64_to_bytes(c),
            conversion::u64_to_bytes(d),
        ];

        unsafe { mem::transmute(arrays) }
    }

    #[allow(clippy::many_single_char_names)]
    /// Returns the next pseudo random number.
    pub fn next_number(&mut self) -> u64 {
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

    #[allow(clippy::many_single_char_names)]
    /// Replaces the seed of self with a new seed. The new seed is generated using the old seed
    /// and the given `u64` value.
    ///
    /// The implementation is inspired by [ScalaCheck](https://github.com/rickynils/scalacheck).
    pub fn reseed(&mut self, n: u64) {
        let (a, b, c, d) = self.seed;

        let n0 = (n >> 32) & 0xffff_ffff;
        let n1 = n & 0xffff_ffff;

        self.seed = (a ^ n0, b ^ n1, c, d);

        for _ in 0..16 {
            self.next_number();
        }
    }

    /// Splits off a new `Prng` from self. The seed of the new `Prng` is generated with self.
    pub fn fork(&mut self) -> Prng {
        let random_number = self.next_number();
        let mut reseeded_prng = self.clone();
        reseeded_prng.reseed(random_number);
        reseeded_prng
    }
}

impl BuildHasher for Prng {
    #[allow(deprecated)]
    type Hasher = SipHasher;

    fn build_hasher(&self) -> Self::Hasher {
        let mut prng = self.clone();
        let (key0, key1) = (prng.next_number(), prng.next_number());
        #[allow(deprecated)]
        SipHasher::new_with_keys(key0, key1)
    }
}

#[cfg(test)]
mod tests {
    use crate::asserts;
    use crate::die::Prng;
    use crate::prelude::tests::*;

    #[test]
    fn init_must_not_have_cycle_length_zero() {
        dicetest!(|fate| {
            let seed = dice::u64(..).roll(fate);

            let prng_init = Prng::init(seed);
            let mut prng_next = prng_init.clone();
            let _ = prng_next.next_number();
            let cycle_length_is_zero = prng_init == prng_next;

            hint!(seed);
            hint!(prng_init);
            hint!(prng_next);

            assert!(!cycle_length_is_zero);
        })
    }

    #[test]
    fn init_with_bytes_is_left_inverse() {
        dicetest!(|fate| {
            asserts::left_inverse(
                fate,
                dice::prng_fork(),
                |prng| prng.seed_as_bytes(),
                Prng::init_with_bytes,
            );
        })
    }

    #[test]
    fn seed_as_bytes_is_left_inverse() {
        dicetest!(|fate| {
            asserts::left_inverse(
                fate,
                dice::array_32(dice::u8(..)),
                Prng::init_with_bytes,
                |prng| prng.seed_as_bytes(),
            );
        })
    }

    #[test]
    fn reseed_changes_prng() {
        dicetest!(|fate| {
            let prng = dice::prng_fork().roll(fate);
            let seed = dice::u64(..).roll(fate);

            let mut prng_reseeded = prng.clone();
            prng_reseeded.reseed(seed);

            hint!(prng);
            hint!(seed);
            hint!(prng_reseeded);

            assert_ne!(prng, prng_reseeded);
        })
    }
}
