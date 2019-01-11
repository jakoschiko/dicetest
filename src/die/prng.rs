use std::hash::BuildHasher;
#[allow(deprecated)]
use std::hash::SipHasher;
use std::mem;
use std::num::Wrapping;

use crate::seed::Seed;
use crate::util::conversion;

/// A pseudorandom number generator. Provides the randomness for `DieOnce` and `Die`.
///
/// The algorithms are based on [this article] by Bob Jenkins.
///
/// [this article]: http://burtleburtle.net/bob/rand/smallprng.html
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Prng {
    state: (u64, u64, u64, u64),
}

impl Prng {
    /// Creates a new instance whose internal state is initialized with the given seed.
    ///
    /// The result has a satisfying cycle length.
    pub fn from_seed(seed: Seed) -> Prng {
        let state = (0xf1ea_5eed, seed.0, seed.0, seed.0);
        let mut prng = Prng { state };
        for _ in 0..20 {
            prng.next_number();
        }
        prng
    }

    /// Creates an `Prng` using the given byte array as internal state.
    ///
    /// This function is a left and right inverse for `Prng::to_bytes`.
    ///
    /// A satisfying cycle length is only guaranteed for bytes from `Prng::to_bytes` called
    /// with an `Prng` that has a satisfying cycle length. Other bytes should not be passed to this
    /// function. For initializing an `Prng` with an arbitrary seed, use `Prng::from_seed` instead.
    pub fn from_bytes(state_bytes: [u8; 32]) -> Prng {
        let arrays: [[u8; 8]; 4] = unsafe { mem::transmute(state_bytes) };

        let a = conversion::bytes_to_u64(arrays[0]);
        let b = conversion::bytes_to_u64(arrays[1]);
        let c = conversion::bytes_to_u64(arrays[2]);
        let d = conversion::bytes_to_u64(arrays[3]);

        let state = (a, b, c, d);
        Prng { state }
    }

    /// Returns the internal state as a byte array.
    ///
    /// This function is a left and right inverse for `Prng::from_bytes`.
    pub fn to_bytes(&self) -> [u8; 32] {
        let (a, b, c, d) = self.state;

        let arrays = [
            conversion::u64_to_bytes(a),
            conversion::u64_to_bytes(b),
            conversion::u64_to_bytes(c),
            conversion::u64_to_bytes(d),
        ];

        unsafe { mem::transmute(arrays) }
    }

    #[allow(clippy::many_single_char_names)]
    /// Returns the next pseudorandom number.
    pub fn next_number(&mut self) -> u64 {
        let (a, b, c, d) = self.state;

        // We use `Wrapping` because overflow and underflow is intended
        let Wrapping(e) = Wrapping(a) - Wrapping(b.rotate_left(7));
        let Wrapping(f) = Wrapping(b) ^ Wrapping(c.rotate_left(13));
        let Wrapping(g) = Wrapping(c) + Wrapping(d.rotate_left(37));
        let Wrapping(h) = Wrapping(d) + Wrapping(e);
        let Wrapping(i) = Wrapping(e) + Wrapping(a);

        self.state = (f, g, h, i);
        i
    }

    #[allow(clippy::many_single_char_names)]
    /// Reinitialze the internal state of self using the current internal state and the given seed.
    ///
    /// The implementation is inspired by [ScalaCheck](https://github.com/rickynils/scalacheck).
    pub fn reseed(&mut self, seed: Seed) {
        let (a, b, c, d) = self.state;

        let n0 = (seed.0 >> 32) & 0xffff_ffff;
        let n1 = seed.0 & 0xffff_ffff;

        self.state = (a ^ n0, b ^ n1, c, d);

        for _ in 0..16 {
            self.next_number();
        }
    }

    /// Splits off a new `Prng` from self. The internal state of the new `Prng` is generated with
    /// self.
    pub fn fork(&mut self) -> Prng {
        let random_number = self.next_number();
        let mut reseeded_prng = self.clone();
        reseeded_prng.reseed(random_number.into());
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
    fn from_seed_must_not_have_cycle_length_zero() {
        dicetest!(|fate| {
            let seed = dice::u64(..).roll(fate);

            let prng_init = Prng::from_seed(seed.into());
            let mut prng_next = prng_init.clone();
            let _ = prng_next.next_number();
            let cycle_length_is_zero = prng_init == prng_next;

            hint_debug!(seed);
            hint_debug!(prng_init);
            hint_debug!(prng_next);

            assert!(!cycle_length_is_zero);
        })
    }

    #[test]
    fn from_bytes_is_left_inverse() {
        dicetest!(|fate| {
            asserts::left_inverse(
                fate,
                dice::prng_fork(),
                |prng| prng.to_bytes(),
                Prng::from_bytes,
            );
        })
    }

    #[test]
    fn to_bytes_is_left_inverse() {
        dicetest!(|fate| {
            asserts::left_inverse(
                fate,
                dice::array_32(dice::u8(..)),
                Prng::from_bytes,
                |prng| prng.to_bytes(),
            );
        })
    }

    #[test]
    fn reseed_changes_prng() {
        dicetest!(|fate| {
            let prng = dice::prng_fork().roll(fate);
            let seed = dice::u64(..).roll(fate);

            let mut prng_reseeded = prng.clone();
            prng_reseeded.reseed(seed.into());

            hint_debug!(prng);
            hint_debug!(seed);
            hint_debug!(prng_reseeded);

            assert_ne!(prng, prng_reseeded);
        })
    }
}
