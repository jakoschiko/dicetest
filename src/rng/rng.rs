use std::num::Wrapping;
use std::mem;
use std::hash::BuildHasher;
#[allow(deprecated)]
use std::hash::SipHasher;

use rand::{self, Rng as LibRng};

use crate::util::conversion;

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
    /// Creates an `Rng` using a `u64` as seed.
    ///
    /// The result has a satisfying cycle length.
    pub fn init(seed_u64: u64) -> Rng {
        let seed = (0xf1ea5eed, seed_u64, seed_u64, seed_u64);
        let mut rng = Rng { seed };
        for _ in 0..20 {
            rng.next();
        }
        rng
    }

    /// Creates an `Rng` using a random seed.
    ///
    /// The result has a satisfying cycle length.
    pub fn random() -> Rng {
        let seed = rand::thread_rng().gen();
        Rng::init(seed)
    }

    /// Creates an `Rng` using a byte array as seed.
    ///
    /// This function is a left and right inverse for `Rng::seed_as_bytes`.
    ///
    /// A satisfying cycle length is only guaranteed for bytes from `Rng::seed_as_bytes` called
    /// with an `Rng` that has a satisfying cycle length. Other bytes should not be passed to this
    /// function. For initializing an `Rng` with an arbitrary seed, use `Rng::init` instead.
    pub fn init_with_bytes(seed_bytes: [u8; 32]) -> Rng {
        let arrays: [[u8; 8]; 4] = unsafe { mem::transmute(seed_bytes) };

        let a = conversion::bytes_to_u64(arrays[0]);
        let b = conversion::bytes_to_u64(arrays[1]);
        let c = conversion::bytes_to_u64(arrays[2]);
        let d = conversion::bytes_to_u64(arrays[3]);

        let seed = (a, b, c, d);
        Rng { seed }
    }

    /// Returns the seed as a byte array.
    ///
    /// This function is a left and right inverse for `Rng::init_with_bytes`.
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

    /// Splits off a new `Rng` from self. The seed of the new `Rng` is generated with self.
    pub fn fork(&mut self) -> Rng {
        let random_number = self.next();
        let mut reseeded_rng = self.clone();
        reseeded_rng.reseed(random_number);
        reseeded_rng
    }
}

impl BuildHasher for Rng {
    #[allow(deprecated)]
    type Hasher = SipHasher;

    fn build_hasher(&self) -> Self::Hasher {
        let mut rng = self.clone();
        let (key0, key1) = (rng.next(), rng.next());
        #[allow(deprecated)]
        let hasher = SipHasher::new_with_keys(key0, key1);
        hasher
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;
    use crate::rng::Rng;

    #[test]
    fn init_must_not_have_cycle_length_zero() {
        assert_prop(|| {
            props::forall_1(
                gens::u64(..).name("seed"),
                |seed| {
                    let rng_init = Rng::init(seed);
                    log!("Rng after init: {:?}", rng_init);
                    let mut rng_next = rng_init.clone();
                    let _ = rng_next.next();
                    log!("Rng after next: {:?}", rng_next);
                    let cycle_length_is_zero = rng_init == rng_next;
                    props::assert(!cycle_length_is_zero, "Cycle length is not zero")
                }
            )
        })
    }

    #[test]
    fn init_with_bytes_is_left_inverse() {
        assert_prop(|| {
            props::left_inverse(
                gens::rng_fork(),
                |rng| rng.seed_as_bytes(),
                Rng::init_with_bytes,
            )
        })
    }

    #[test]
    fn seed_as_bytes_is_left_inverse() {
        assert_prop(|| {
            props::left_inverse(
                gens::array_32(gens::u8(..)),
                Rng::init_with_bytes,
                |rng| rng.seed_as_bytes(),
            )
        })
    }

    #[test]
    fn reseed_changes_rng() {
        assert_prop(|| {
            props::forall_2(
                gens::rng_fork().name("rng"),
                gens::u64(..).name("seed"),
                |rng, seed| {
                    let mut rng_reseeded = rng.clone();
                    rng_reseeded.reseed(seed);
                    log_var!(rng_reseeded);
                    let rngs_are_equal = rng == rng_reseeded;
                    props::assert(!rngs_are_equal, "Reseeded Rng is not equal")
                }
            )
        })
    }
}
