use std::mem::size_of;
use std::fmt::Debug;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::prelude::gens::*;

// Non-empty range for integer generators like `gens::u8`, `gens::i32`, etc.
// The integer type is represented by `I`.
pub trait IntRange<I>: Clone + Debug {
    /// Returns the inclusive bounds `(lower, upper)` that represent the range. They must hold
    /// `lower <= upper`.
    ///
    /// # Panics
    /// Panics if the range cannot be represented as `(lower, upper)` with `lower <= upper`.
    fn bounds(&self) -> (I, I);
}

#[derive(Clone, Debug)]
struct UncheckedRange<I> {
    lower: I,
    upper: I,
}

fn empty_int_range<I>(bounds: &(impl IntRange<I> + Debug)) -> ! {
    panic!("IntRange is invalid because it contains no values: {:?}", bounds)
}

macro_rules! impl_int_range {
    ($int:ident) => (
        impl IntRange<$int> for $int {
            fn bounds(&self) -> ($int, $int) {
                (*self, *self)
            }
        }

        impl IntRange<$int> for Range<$int> {
            fn bounds(&self) -> ($int, $int) {
                if self.start < self.end {
                    let lower = self.start;
                    let upper = self.end - 1;
                    (lower, upper)
                } else {
                    empty_int_range(self)
                }
            }
        }

        impl IntRange<$int> for RangeFrom<$int> {
            fn bounds(&self) -> ($int, $int) {
                let lower = self.start;
                let upper = $int::max_value();
                (lower, upper)
            }
        }

        impl IntRange<$int> for RangeFull {
            fn bounds(&self) -> ($int, $int) {
                ($int::min_value(), $int::max_value())
            }
        }

        impl IntRange<$int> for RangeInclusive<$int> {
            fn bounds(&self) -> ($int, $int) {
                let lower = *self.start();
                let upper = *self.end();
                if lower <= upper {
                    (lower, upper)
                } else {
                    empty_int_range(self)
                }
            }
        }

        impl IntRange<$int> for RangeTo<$int> {
            fn bounds(&self) -> ($int, $int) {
                let lower = $int::min_value();
                if self.end > lower {
                    let upper = self.end - 1;
                    (lower, upper)
                } else {
                    empty_int_range(self)
                }
            }
        }

        impl IntRange<$int> for RangeToInclusive<$int> {
            fn bounds(&self) -> ($int, $int) {
                ($int::min_value(), self.end)
            }
        }

        impl IntRange<$int> for UncheckedRange<$int> {
            fn bounds(&self) -> ($int, $int) {
                (self.lower, self.upper)
            }
        }
    )
}

impl_int_range! { u8 }
impl_int_range! { i8 }
impl_int_range! { u16 }
impl_int_range! { i16 }
impl_int_range! { u32 }
impl_int_range! { i32 }
impl_int_range! { u64 }
impl_int_range! { i64 }
impl_int_range! { u128 }
impl_int_range! { i128 }
impl_int_range! { usize }
impl_int_range! { isize }

macro_rules! fn_int {
    ($int:ident, $uni_int:ident, $uint:ident, $random_int:ident, $special:expr) => (
        /// Generates an integer inside the given range. All integers are uniformly distributed.
        ///
        /// # Panics
        /// Panics if the range is invalid, see `IntRange::bounds`.
        pub fn $uni_int(range: impl IntRange<$int>) -> impl Gen<$int> {
            fn to_shifted_unsigned(i: $int) -> $uint {
                let uoffset = $int::min_value() as $uint;
                let x = i as $uint;
                x.wrapping_add(uoffset)
            }

            fn from_shifted_unsigned(u: $uint) -> $int {
                let uoffset = $int::min_value() as $uint;
                let x = u.wrapping_add(uoffset);
                x as $int
            }

            // `IntRange::bounds` guarantees that `lower <= upper`
            let (lower, upper) = range.bounds();

            gens::from_fn(move |dice| {
                if lower == upper {
                    // The range contains exactly one value
                    lower
                } else {
                    let random_int = $random_int(dice.prng);

                    if lower == $int::min_value() && upper == $int::max_value() {
                        // Full integer range, hence the randomly chosen integer is already inside
                        // the range
                        random_int
                    } else {
                        let random_unsigned_inside_range = {
                            // We shift the integer into the unsigned integer range
                            let random_unsigned = to_shifted_unsigned(random_int);
                            let lower_unsigned = to_shifted_unsigned(lower);
                            let upper_unsigned = to_shifted_unsigned(upper);

                            // There will be no underflow because `upper > lower` (see above)
                            let range_minus_one = upper_unsigned - lower_unsigned;
                            // There will be no overflow because it's not the full integer range
                            // (see above)
                            let range = range_minus_one + 1;
                            let offset = random_unsigned % range;
                            lower_unsigned + offset
                        };

                        from_shifted_unsigned(random_unsigned_inside_range)
                    }
                }
            })
        }

        /// Generates an integer inside the given range. Some special integers have a higher
        /// probability of being generated.
        ///
        /// # Panics
        /// Panics if the range is invalid, see `IntRange::bounds`.
        pub fn $int(range: impl IntRange<$int>) -> impl Gen<$int> {
            let (lower, upper) = range.bounds();
            // `uni_int` does not need to check the range again
            let unchecked_range = UncheckedRange { lower, upper };
            let all_gen = $uni_int(unchecked_range);

            let special_gen = {
                let extremum_gen = move || gens::one_of_2(lower, upper);
                let special_fallback_gen = extremum_gen();
                let special_gen = {
                    gens::from_fn(move |dice| {
                        let special_values = $special;
                        let special_value = gens::one_of_array(&special_values).gen(dice);
                        if lower <= special_value && special_value <= upper {
                            special_value
                        } else {
                            // `special_value` is outside the range, fallback to other generator
                            special_fallback_gen.gen(dice)
                        }
                    })
                };

                gens::one_of_gen_2(extremum_gen(), special_gen)
            };

            gens::one_of_gen_2(all_gen, special_gen)
        }
    )
}

fn random_u8(prng: &mut Prng) -> u8 {
    prng.next_number() as u8
}

fn random_i8(prng: &mut Prng) -> i8 {
    prng.next_number() as i8
}

fn random_u16(prng: &mut Prng) -> u16 {
    prng.next_number() as u16
}

fn random_i16(prng: &mut Prng) -> i16 {
    prng.next_number() as i16
}

fn random_u32(prng: &mut Prng) -> u32 {
    prng.next_number() as u32
}

fn random_i32(prng: &mut Prng) -> i32 {
    prng.next_number() as i32
}

fn random_u64(prng: &mut Prng) -> u64 {
    prng.next_number() as u64
}

fn random_i64(prng: &mut Prng) -> i64 {
    prng.next_number() as i64
}

fn random_u128(prng: &mut Prng) -> u128 {
    (u128::from(prng.next_number()) << 64) | u128::from(prng.next_number())
}

fn random_i128(prng: &mut Prng) -> i128 {
    random_u128(prng) as i128
}

fn random_usize(prng: &mut Prng) -> usize {
    if size_of::<usize>() <= size_of::<u64>() {
        random_u64(prng) as usize
    } else if size_of::<usize>() == size_of::<u128>() {
        random_u128(prng) as usize
    } else {
        panic!("Can't generate `usize` because it has an unsupported size");
    }
}

fn random_isize(prng: &mut Prng) -> isize {
    if size_of::<isize>() <= size_of::<i64>() {
        random_i64(prng) as isize
    } else if size_of::<isize>() == size_of::<i128>() {
        random_i128(prng) as isize
    } else {
        panic!("Can't generate `isize` because it has an unsupported size");
    }
}

fn_int! { u8, uni_u8, u8, random_u8, [1, 2] }
fn_int! { i8, uni_i8, u8, random_i8, [-2, -1, 0, 1, 2] }
fn_int! { u16, uni_u16, u16, random_u16, [1, 2] }
fn_int! { i16, uni_i16, u16, random_i16, [-2, -1, 0, 1, 2] }
fn_int! { u32, uni_u32, u32, random_u32, [1, 2] }
fn_int! { i32, uni_i32, u32, random_i32, [-2, -1, 0, 1, 2] }
fn_int! { u64, uni_u64, u64, random_u64, [1, 2] }
fn_int! { i64, uni_i64, u64, random_i64, [-2, -1, 0, 1, 2] }
fn_int! { u128, uni_u128, u128, random_u128, [1, 2] }
fn_int! { i128, uni_i128, u128, random_i128, [-2, -1, 0, 1, 2] }
fn_int! { usize, uni_usize, usize, random_usize, [1, 2] }
fn_int! { isize, uni_isize, usize, random_isize, [-2, -1, 0, 1, 2] }

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::prelude::tests::*;

    fn range_contains_int<I, GI, B, GB, R>(
        dice: &mut Dice,
        range_data_gen: GB,
        create_range: fn(B) -> R,
        int_gen: fn(R) -> GI,
        is_in_range: fn(B, I) -> bool,
    )
    where
        I: Debug,
        GI: GenOnce<I>,
        B: Copy + Debug,
        GB: GenOnce<B>,
        R: gens::IntRange<I> + Debug,
    {
        let range_data = range_data_gen.gen_once(dice);
        hint!(range_data);

        let range = create_range(range_data);
        hint!(range);

        let int = int_gen(range).gen_once(dice);
        hint!(int);

        assert!(is_in_range(range_data, int));
    }

    macro_rules! range_tests {
        (
            $int:ident:
            $int_is_in_range:ident
            $int_is_in_range_from:ident
            $int_is_in_range_inclusive:ident
            $int_is_in_range_to:ident
            $int_is_in_range_to_inclusive:ident
        ) => (
            #[test]
            fn $int_is_in_range() {
                dicetest!(|dice| {
                    range_contains_int(
                        dice,
                        gens::array_2(gens::$int(..$int::max_value() - 1))
                            .map(|[a, b]| (a.min(b), a.max(b) + 1)),
                        |(lower, upper)| lower..upper,
                        gens::$int,
                        |(lower, upper), int| lower <= int && int < upper,
                    );
                })
            }

            #[test]
            fn $int_is_in_range_from() {
                dicetest!(|dice| {
                    range_contains_int(
                        dice,
                        gens::$int(..),
                        |lower| lower..,
                        gens::$int,
                        |lower, size| lower <= size,
                    );
                })
            }

            #[test]
            fn $int_is_in_range_inclusive() {
                dicetest!(|dice| {
                    range_contains_int(
                        dice,
                        gens::array_2(gens::$int(..))
                            .map(|[a, b]| (a.min(b), a.max(b))),
                        |(lower, upper)| lower..=upper,
                        gens::$int,
                        |(lower, upper), size| lower <= size && size <= upper,
                    );
                })
            }

            #[test]
            fn $int_is_in_range_to() {
                dicetest!(|dice| {
                    range_contains_int(
                        dice,
                        gens::$int(1..),
                        |upper| ..upper,
                        gens::$int,
                        |upper, size| size < upper,
                    );
                })
            }

            #[test]
            fn $int_is_in_range_to_inclusive() {
                dicetest!(|dice| {
                    range_contains_int(
                        dice,
                        gens::$int(..),
                        |upper| ..=upper,
                        gens::$int,
                        |upper, size| size <= upper,
                    );
                })
            }
        )
    }

    range_tests! { u8:
        u8_is_in_range
        u8_is_in_range_from
        u8_is_in_range_inclusive
        u8_is_in_range_to
        u8_is_in_range_to_inclusive
    }

    range_tests! { i8:
        i8_is_in_range
        i8_is_in_range_from
        i8_is_in_range_inclusive
        i8_is_in_range_to
        i8_is_in_range_to_inclusive
    }

    range_tests! { u16:
        u16_is_in_range
        u16_is_in_range_from
        u16_is_in_range_inclusive
        u16_is_in_range_to
        u16_is_in_range_to_inclusive
    }

    range_tests! { i16:
        i16_is_in_range
        i16_is_in_range_from
        i16_is_in_range_inclusive
        i16_is_in_range_to
        i16_is_in_range_to_inclusive
    }

    range_tests! { u32:
        u32_is_in_range
        u32_is_in_range_from
        u32_is_in_range_inclusive
        u32_is_in_range_to
        u32_is_in_range_to_inclusive
    }

    range_tests! { i32:
        i32_is_in_range
        i32_is_in_range_from
        i32_is_in_range_inclusive
        i32_is_in_range_to
        i32_is_in_range_to_inclusive
    }

    range_tests! { u64:
        u64_is_in_range
        u64_is_in_range_from
        u64_is_in_range_inclusive
        u64_is_in_range_to
        u64_is_in_range_to_inclusive
    }

    range_tests! { i64:
        i64_is_in_range
        i64_is_in_range_from
        i64_is_in_range_inclusive
        i64_is_in_range_to
        i64_is_in_range_to_inclusive
    }

    range_tests! { u128:
        u128_is_in_range
        u128_is_in_range_from
        u128_is_in_range_inclusive
        u128_is_in_range_to
        u128_is_in_range_to_inclusive
    }

    range_tests! { i128:
        i128_is_in_range
        i128_is_in_range_from
        i128_is_in_range_inclusive
        i128_is_in_range_to
        i128_is_in_range_to_inclusive
    }

    range_tests! { usize:
        usize_is_in_range
        usize_is_in_range_from
        usize_is_in_range_inclusive
        usize_is_in_range_to
        usize_is_in_range_to_inclusive
    }

    range_tests! { isize:
        isize_is_in_range
        isize_is_in_range_from
        isize_is_in_range_inclusive
        isize_is_in_range_to
        isize_is_in_range_to_inclusive
    }

}
