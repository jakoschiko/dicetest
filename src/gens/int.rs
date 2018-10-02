use std;
use std::fmt::Debug;
use std::ops::{Add, Sub, Rem};

use ::rng::Rng;
use ::gen::Gen;
use ::gens;

/// Abstraction for signed and unsigned binary integers.
///
/// # Generators
/// - `gens::int_uniform`
/// - `gens::int`
/// - `gens::choose_int_uniform`
/// - `gens::choose_int`
pub trait Int:
    Add<Output=Self> +
    Sub<Output=Self> +
    Rem<Output=Self> +
    Ord +
    Copy +
    Debug
where
    Self: Sized + 'static,
{
    /// The singed representation of this interger type.
    type S: Int<S = Self::S>;
    /// The unsinged representation of this interger type.
    type U: Int<U = Self::U>;

    /// The integer `1`.
    fn one() -> Self;
    /// The smallest integer.
    fn min_value() -> Self;
    /// The biggest integer.
    fn max_value() -> Self;
    /// These integers are special and should be generated with a higher propability.
    fn special_values() -> &'static [Self];

    /// Generates a random integer.
    fn from_rng(rng: &mut Rng) -> Self;

    /// Re-intepretes the bits of this integer as a unsigned interger. No-op if the own interger
    /// type is unsigned.
    fn to_unsigned(self) -> Self::U;
    /// Re-intepretes the bits of given unsigned integer as a the own integer type. No-op if the own
    /// integer type is unsigned.
    fn from_unsigned(u: Self::U) -> Self;
    /// Shifts this integer to the unsigned integer range. No-op if the own interger type is
    /// unsigned.
    ///
    /// # Examples
    /// - -128s8 will be shifted to 0u8
    /// - 0s8 will be shifted to 128u8
    /// - 127s8 will be shifted to 255u8
    fn to_shifted_unsigned(self) -> Self::U;
    /// Shifts the given unsigned integer to the own integer range. No-op if the own interger type
    /// is unsigned.
    ///
    /// # Examples
    /// - 0u8 will be shifted to 128s8
    /// - 128u8 will be shifted to 0s8
    /// - 255u8 will be shifted to 127s8
    fn from_shifted_unsigned(u: Self::U) -> Self;
}

macro_rules! impl_unsigned_and_signed_int {
    ($utype:ident, $stype:ident) => (
        impl Int for $utype {
            type S = $stype;
            type U = Self;

            fn one() -> Self { 1 }
            fn min_value() -> Self { 0 }
            fn max_value() -> Self { std::$utype::MAX }
            fn special_values() -> &'static [Self] {
                &[0, 1, 2, std::$utype::MAX]
            }

            fn from_rng(rng: &mut Rng) -> Self { rng.next() as $utype }

            fn to_unsigned(self) -> Self::U { self }
            fn from_unsigned(u: Self::U) -> Self { u }
            fn to_shifted_unsigned(self) -> Self::U { self }
            fn from_shifted_unsigned(u: Self::U) -> Self { u }


        }

        impl Int for $stype {
            type S = Self;
            type U = $utype;

            fn one() -> Self { 1 }
            fn min_value() -> Self { std::$stype::MIN }
            fn max_value() -> Self { std::$stype::MAX }
            fn special_values() -> &'static [Self] {
                &[std::$stype::MIN, -2, -1, 0, 1, 2, std::$stype::MAX]
            }

            fn from_rng(rng: &mut Rng) -> Self { rng.next() as $stype }

            fn to_unsigned(self) -> Self::U { self as $utype }
            fn from_unsigned(u: Self::U) -> Self { u as $stype }
            fn to_shifted_unsigned(self) -> Self::U {
                let uoffset = Self::min_value().to_unsigned();
                let x = self.to_unsigned();
                x.wrapping_add(uoffset)
            }
            fn from_shifted_unsigned(u: Self::U) -> Self {
                let uoffset = Self::min_value().to_unsigned();
                let x = u.wrapping_add(uoffset);
                Self::from_unsigned(x)
            }
        }
    )
}

impl_unsigned_and_signed_int! { usize, isize }
impl_unsigned_and_signed_int! { u8, i8 }
impl_unsigned_and_signed_int! { u16, i16 }
impl_unsigned_and_signed_int! { u32, i32 }
impl_unsigned_and_signed_int! { u64, i64 }

/// Generates an integer. All integers are uniformly distributed.
pub fn int_uniform<I>() -> impl Gen<I>
where
    I: Int,
{
    gens::from_fn(|rng, _| I::from_rng(rng))
}

/// Generates an integer. Some special intergers have a higher probability of being generated.
pub fn int<I: Int>() -> impl Gen<I> {
    let all_gen = int_uniform::<I>();

    let special_gen = gens::one_of_array(I::special_values());

    gens::one_of_gen_2(all_gen, special_gen)
}

/// Generates an integer that lies between the inclusive interval borders.
/// All integers in the interval are uniformly distributed.
///
/// # Panics
/// Panics if the interval is empty, i.e. `min > max`.
pub fn choose_int_uniform<I>(min: I, max: I) -> impl Gen<I>
where
    I: Int
{
    if min > max {
        // Empty interval, there are no values to generate
        panic!("choose_int_uniform: the interval [{:?},{:?}] is empty", min, max);
    }

    gens::from_fn(move |rng, _| {
        if min == max {
            // Interval has exactly one value
            min
        } else {
            let random_int = I::from_rng(rng);

            if min == I::min_value() && max == I::max_value() {
                // Full integer range, hence the randomly chosen int is already in the interval
                random_int
            } else {
                let random_unsigned_in_interval = {
                    // We shift the interger into the unsigned integer range.
                    let random_unsigned = I::to_shifted_unsigned(random_int);
                    let min_unsigned = I::to_shifted_unsigned(min);
                    let max_unsigned = I::to_shifted_unsigned(max);
                    let one = I::U::one();

                    // There will be no underflow because `max > min` (see above).
                    let range_minus_one = max_unsigned - min_unsigned;
                    // There will be no overflow because it's not the full range (see above).
                    let range = range_minus_one + one;
                    let offset = random_unsigned % range;
                    min_unsigned + offset
                };

                I::from_shifted_unsigned(random_unsigned_in_interval)
            }
        }
    })
}

/// Generates an integer that lies between the inclusive interval borders.
/// Some special intergers in the interval have a higher probability of being generated.
///
/// # Panics
/// Panics if the interval is empty, i.e. `min > max`.
pub fn choose_int<I>(min: I, max: I) -> impl Gen<I>
where
    I: Int
{
    let all_gen = choose_int_uniform::<I>(min, max);

    let special_gen = {
        let limit_gen = move || gens::one_of_2(min, max);
        let fallback_gen = limit_gen();
        let special_in_interval_gen = {
            gens::from_fn(move |rng, size| {
                let special_gen = gens::one_of_array(I::special_values());
                let special = special_gen.gen(rng, size);
                if min <= special && special <= max {
                    special
                } else {
                    // Significant value is not in range, fallback to other generator
                    fallback_gen.gen(rng, size)
                }
            })
        };

        gens::one_of_gen_2(limit_gen(), special_in_interval_gen)
    };

    gens::one_of_gen_2(all_gen, special_gen)
}

#[cfg(test)]
mod tests {
    #[test]
    fn choose_int_uniform_generates_only_ints_in_interval() {
        // TODO: impl test
    }

    #[test]
    fn choose_int_generates_only_ints_in_interval() {
        // TODO: impl test
    }
}
