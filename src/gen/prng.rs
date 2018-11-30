use std::num::Wrapping;
use std::mem;
use std::hash::BuildHasher;
#[allow(deprecated)]
use std::hash::SipHasher;

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
        let seed = (0xf1ea5eed, seed_u64, seed_u64, seed_u64);
        let mut prng = Prng { seed };
        for _ in 0..20 {
            prng.next();
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

        let seed_bytes = unsafe { mem::transmute(arrays) };
        seed_bytes
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

    /// Replaces the seed of self with a new seed. The new seed is generated using the old seed
    /// and the given `u64` value.
    ///
    /// The implementation is inspired by [ScalaCheck](https://github.com/rickynils/scalacheck).
    pub fn reseed(&mut self, n: u64) {
        let (a, b, c, d) = self.seed;

        let n0 = (n >> 32) & 0xffffffff;
        let n1 = n & 0xffffffff;

        self.seed = (a ^ n0, b ^ n1, c, d);

        for _ in 0..16 {
            self.next();
        }
    }

    /// Splits off a new `Prng` from self. The seed of the new `Prng` is generated with self.
    pub fn fork(&mut self) -> Prng {
        let random_number = self.next();
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
        let (key0, key1) = (prng.next(), prng.next());
        #[allow(deprecated)]
        let hasher = SipHasher::new_with_keys(key0, key1);
        hasher
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;
    use crate::gen::Prng;

    #[test]
    fn init_must_not_have_cycle_length_zero() {
        assert_prop!(
            props::forall_1(
                gens::u64(..).name("seed"),
                |seed| {
                    let prng_init = Prng::init(seed);
                    log!("Prng after init: {:?}", prng_init);
                    let mut prng_next = prng_init.clone();
                    let _ = prng_next.next();
                    log!("Prng after next: {:?}", prng_next);
                    let cycle_length_is_zero = prng_init == prng_next;
                    props::assert(!cycle_length_is_zero, "Cycle length is not zero")
                }
            )
        )
    }

    #[test]
    fn init_with_bytes_is_left_inverse() {
        assert_prop!(
            props::left_inverse(
                gens::prng_fork(),
                |prng| prng.seed_as_bytes(),
                Prng::init_with_bytes,
            )
        )
    }

    #[test]
    fn seed_as_bytes_is_left_inverse() {
        assert_prop!(
            props::left_inverse(
                gens::array_32(gens::u8(..)),
                Prng::init_with_bytes,
                |prng| prng.seed_as_bytes(),
            )
        )
    }

    #[test]
    fn reseed_changes_prng() {
        assert_prop!(
            props::forall_2(
                gens::prng_fork().name("prng"),
                gens::u64(..).name("seed"),
                |prng, seed| {
                    let mut prng_reseeded = prng.clone();
                    prng_reseeded.reseed(seed);
                    log_var!(prng_reseeded);
                    let prngs_are_equal = prng == prng_reseeded;
                    props::assert(!prngs_are_equal, "Reseeded Prng is not equal")
                }
            )
        )
    }
}
