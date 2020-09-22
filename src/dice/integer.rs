use std::fmt::Debug;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::prelude::*;

/// Non-empty range for integer generators like `dice::u8`, `dice::i32`, etc.
///
/// The integer type is represented by `I`.
pub trait IntegerRange<I> {
    /// Returns the inclusive lower bound and the inclusive upper bound that represent the range.
    ///
    /// # Panics
    ///
    /// Panics if the range is empty.
    fn bounds(self) -> (I, I);
}

struct UncheckedRange<I> {
    lower: I,
    upper: I,
}

fn empty_integer_range<I>(bounds: &(impl IntegerRange<I> + Debug)) -> ! {
    panic!(
        "IntegerRange is invalid because it contains no values: {:?}",
        bounds
    )
}

macro_rules! impl_integer_range {
    ($integer:ident) => {
        impl IntegerRange<$integer> for $integer {
            fn bounds(self) -> ($integer, $integer) {
                (self, self)
            }
        }

        impl IntegerRange<$integer> for Range<$integer> {
            fn bounds(self) -> ($integer, $integer) {
                if self.start < self.end {
                    let lower = self.start;
                    let upper = self.end - 1;
                    (lower, upper)
                } else {
                    empty_integer_range(&self)
                }
            }
        }

        impl IntegerRange<$integer> for RangeFrom<$integer> {
            fn bounds(self) -> ($integer, $integer) {
                (self.start, $integer::max_value())
            }
        }

        impl IntegerRange<$integer> for RangeFull {
            fn bounds(self) -> ($integer, $integer) {
                ($integer::min_value(), $integer::max_value())
            }
        }

        impl IntegerRange<$integer> for RangeInclusive<$integer> {
            fn bounds(self) -> ($integer, $integer) {
                if self.start() <= self.end() {
                    self.into_inner()
                } else {
                    empty_integer_range(&self)
                }
            }
        }

        impl IntegerRange<$integer> for RangeTo<$integer> {
            fn bounds(self) -> ($integer, $integer) {
                let lower = $integer::min_value();
                if self.end > lower {
                    let upper = self.end - 1;
                    (lower, upper)
                } else {
                    empty_integer_range(&self)
                }
            }
        }

        impl IntegerRange<$integer> for RangeToInclusive<$integer> {
            fn bounds(self) -> ($integer, $integer) {
                ($integer::min_value(), self.end)
            }
        }

        impl IntegerRange<$integer> for UncheckedRange<$integer> {
            fn bounds(self) -> ($integer, $integer) {
                (self.lower, self.upper)
            }
        }
    };
}

impl_integer_range! { u8 }
impl_integer_range! { i8 }
impl_integer_range! { u16 }
impl_integer_range! { i16 }
impl_integer_range! { u32 }
impl_integer_range! { i32 }
impl_integer_range! { u64 }
impl_integer_range! { i64 }
impl_integer_range! { u128 }
impl_integer_range! { i128 }
impl_integer_range! { usize }
impl_integer_range! { isize }

macro_rules! fn_integer {
    (
        $integer:ident,
        $uni_integer:ident,
        $uinteger:ident,
        $random_uinteger:ident,
        $special_values:expr
    ) => {
        /// Generates an integer inside the given range. All integers are uniformly distributed.
        ///
        /// # Panics
        ///
        /// Panics if the range is empty.
        ///
        /// # Examples
        ///
        /// This example generates integers without panicking:
        ///
        /// ```
        /// use dicetest::prelude::*;
        /// use dicetest::{Prng, Limit};
        ///
        /// let mut prng = Prng::from_seed(0x5EED.into());
        /// let limit = Limit::default();
        ///
        /// Fate::run(&mut prng, limit, |fate| {
        ///     assert!(dice::uni_u8(42).roll(fate) == 42);
        ///
        ///     assert!(dice::uni_u8(42..).roll(fate) >= 42);
        ///
        ///     assert!(dice::uni_u8(..=71).roll(fate) <= 71);
        ///
        ///     assert!(dice::uni_u8(..71).roll(fate) < 71);
        ///
        ///     let integer = dice::uni_u8(42..=71).roll(fate);
        ///     assert!(integer >= 42 && integer <= 71);
        ///
        ///     let integer = dice::uni_u8(42..71).roll(fate);
        ///     assert!(integer >= 42 && integer < 71);
        ///
        ///     let integer = dice::uni_u8(..).roll(fate);
        ///     assert!(integer >= 0 && integer <= u8::max_value());
        /// });
        /// ```
        ///
        /// This example panics:
        ///
        /// ```should_panic
        /// use dicetest::prelude::*;
        ///
        /// // Oh no, panic!
        /// let _integer_die = dice::u8(71..42);
        /// ```
        pub fn $uni_integer(range: impl IntegerRange<$integer>) -> impl Die<$integer> {
            fn to_shifted_unsigned(i: $integer) -> $uinteger {
                let uoffset = $integer::min_value() as $uinteger;
                let x = i as $uinteger;
                x.wrapping_add(uoffset)
            }

            fn from_shifted_unsigned(u: $uinteger) -> $integer {
                let uoffset = $integer::min_value() as $uinteger;
                let x = u.wrapping_add(uoffset);
                x as $integer
            }

            // `IntegerRange::bounds` guarantees that `lower <= upper`
            let (lower, upper) = range.bounds();

            dice::from_fn(move |fate| {
                if lower == upper {
                    // The range contains exactly one value
                    lower
                } else {
                    let random_unsigned = $random_uinteger(fate);

                    if lower == $integer::min_value() && upper == $integer::max_value() {
                        // Full integer range, hence the randomly chosen integer is already inside
                        // the range
                        random_unsigned as $integer
                    } else {
                        let random_unsigned_inside_range = {
                            // We shift the integer into the unsigned integer range
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
        ///
        /// Panics if the range is empty.
        ///
        /// # Examples
        ///
        /// This example generates integers without panicking:
        ///
        /// ```
        /// use dicetest::prelude::*;
        /// use dicetest::{Prng, Limit};
        ///
        /// let mut prng = Prng::from_seed(0x5EED.into());
        /// let limit = Limit::default();
        ///
        /// Fate::run(&mut prng, limit, |fate| {
        ///     assert!(dice::u8(42).roll(fate) == 42);
        ///
        ///     assert!(dice::u8(42..).roll(fate) >= 42);
        ///
        ///     assert!(dice::u8(..=71).roll(fate) <= 71);
        ///
        ///     assert!(dice::u8(..71).roll(fate) < 71);
        ///
        ///     let integer = dice::u8(42..=71).roll(fate);
        ///     assert!(integer >= 42 && integer <= 71);
        ///
        ///     let integer = dice::u8(42..71).roll(fate);
        ///     assert!(integer >= 42 && integer < 71);
        ///
        ///     let integer = dice::u8(..).roll(fate);
        ///     assert!(integer >= 0 && integer <= u8::max_value());
        /// });
        /// ```
        ///
        /// This example panics:
        ///
        /// ```should_panic
        /// use dicetest::prelude::*;
        ///
        /// // Oh no, panic!
        /// let _integer_die = dice::u8(71..42);
        /// ```
        pub fn $integer(range: impl IntegerRange<$integer>) -> impl Die<$integer> {
            let (lower, upper) = range.bounds();

            // `uni_integer` does not need to check the range again
            let unchecked_range = UncheckedRange { lower, upper };
            let regular_value_die = $uni_integer(unchecked_range);

            // Generates once in a while a special value that is inside the range
            let maybe_special_value_die = {
                let border_value_die = dice::one_of_2(Some(lower), Some(upper));
                let const_value_die = {
                    let special_values = &$special_values;
                    dice::one_of_slice(special_values).map(move |special_value| {
                        if lower <= special_value && special_value <= upper {
                            Some(special_value)
                        } else {
                            None
                        }
                    })
                };

                dice::weighted_one_of_die_3(
                    (6, dice::just(None)),
                    (1, border_value_die),
                    (1, const_value_die),
                )
            };

            dice::from_fn(move |fate| match maybe_special_value_die.roll(fate) {
                Some(special_value) => special_value,
                None => regular_value_die.roll(fate),
            })
        }
    };
}

fn random_u64(prng: &mut Fate) -> u64 {
    prng.next_number()
}

fn random_u128(prng: &mut Fate) -> u128 {
    (u128::from(prng.next_number()) << 64) | u128::from(prng.next_number())
}

// Some of the integer types use random generators for bigger integer types.
// This improves the uniform destribution.
fn_integer! { u8, uni_u8, u64, random_u64, [1, 2] }
fn_integer! { i8, uni_i8, u64, random_u64, [-2, -1, 0, 1, 2] }
fn_integer! { u16, uni_u16, u64, random_u64, [1, 2] }
fn_integer! { i16, uni_i16, u64, random_u64, [-2, -1, 0, 1, 2] }
fn_integer! { u32, uni_u32, u64, random_u64, [1, 2] }
fn_integer! { i32, uni_i32, u64, random_u64, [-2, -1, 0, 1, 2] }
fn_integer! { u64, uni_u64, u64, random_u64, [1, 2] }
fn_integer! { i64, uni_i64, u64, random_u64, [-2, -1, 0, 1, 2] }
fn_integer! { u128, uni_u128, u128, random_u128, [1, 2] }
fn_integer! { i128, uni_i128, u128, random_u128, [-2, -1, 0, 1, 2] }
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
fn_integer! { usize, uni_usize, u64, random_u64, [1, 2] }
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
fn_integer! { isize, uni_isize, u64, random_u64, [-2, -1, 0, 1, 2] }

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::prelude::*;

    fn range_contains_integer<I, ID, B, BD, R>(
        fate: &mut Fate,
        range_data_die: BD,
        create_range: fn(B) -> R,
        integer_die: fn(R) -> ID,
        is_in_range: fn(B, I) -> bool,
    ) where
        I: Debug,
        ID: DieOnce<I>,
        B: Copy + Debug,
        BD: DieOnce<B>,
        R: dice::IntegerRange<I> + Debug,
    {
        let range_data = range_data_die.roll_once(fate);
        hint_debug!(range_data);

        let range = create_range(range_data);
        hint_debug!(range);

        let integer = integer_die(range).roll_once(fate);
        hint_debug!(integer);

        assert!(is_in_range(range_data, integer));
    }

    macro_rules! range_tests {
        (
            $integer:ident:
            $integer_is_in_single_value_range:ident
            $integer_is_in_range:ident
            $integer_is_in_range_from:ident
            $integer_is_in_range_inclusive:ident
            $integer_is_in_range_to:ident
            $integer_is_in_range_to_inclusive:ident
        ) => {
            #[test]
            fn $integer_is_in_single_value_range() {
                Dicetest::repeatedly().run(|fate| {
                    range_contains_integer(
                        fate,
                        dice::$integer(..),
                        |single| single,
                        dice::$integer,
                        |single, integer| single == integer,
                    );
                })
            }

            #[test]
            fn $integer_is_in_range() {
                Dicetest::repeatedly().run(|fate| {
                    range_contains_integer(
                        fate,
                        dice::array_2(dice::$integer(..$integer::max_value() - 1))
                            .map(|[a, b]| (a.min(b), a.max(b) + 1)),
                        |(lower, upper)| lower..upper,
                        dice::$integer,
                        |(lower, upper), integer| lower <= integer && integer < upper,
                    );
                })
            }

            #[test]
            fn $integer_is_in_range_from() {
                Dicetest::repeatedly().run(|fate| {
                    range_contains_integer(
                        fate,
                        dice::$integer(..),
                        |lower| lower..,
                        dice::$integer,
                        |lower, integer| lower <= integer,
                    );
                })
            }

            #[test]
            fn $integer_is_in_range_inclusive() {
                Dicetest::repeatedly().run(|fate| {
                    range_contains_integer(
                        fate,
                        dice::array_2(dice::$integer(..)).map(|[a, b]| (a.min(b), a.max(b))),
                        |(lower, upper)| lower..=upper,
                        dice::$integer,
                        |(lower, upper), integer| lower <= integer && integer <= upper,
                    );
                })
            }

            #[test]
            fn $integer_is_in_range_to() {
                Dicetest::repeatedly().run(|fate| {
                    range_contains_integer(
                        fate,
                        dice::$integer($integer::min_value() + 1..),
                        |upper| ..upper,
                        dice::$integer,
                        |upper, integer| integer < upper,
                    );
                })
            }

            #[test]
            fn $integer_is_in_range_to_inclusive() {
                Dicetest::repeatedly().run(|fate| {
                    range_contains_integer(
                        fate,
                        dice::$integer(..),
                        |upper| ..=upper,
                        dice::$integer,
                        |upper, integer| integer <= upper,
                    );
                })
            }
        };
    }

    range_tests! { u8:
        u8_is_in_single_value_range
        u8_is_in_range
        u8_is_in_range_from
        u8_is_in_range_inclusive
        u8_is_in_range_to
        u8_is_in_range_to_inclusive
    }

    range_tests! { i8:
        i8_is_in_single_value_range
        i8_is_in_range
        i8_is_in_range_from
        i8_is_in_range_inclusive
        i8_is_in_range_to
        i8_is_in_range_to_inclusive
    }

    range_tests! { u16:
        u16_is_in_single_value_range
        u16_is_in_range
        u16_is_in_range_from
        u16_is_in_range_inclusive
        u16_is_in_range_to
        u16_is_in_range_to_inclusive
    }

    range_tests! { i16:
        i16_is_in_single_value_range
        i16_is_in_range
        i16_is_in_range_from
        i16_is_in_range_inclusive
        i16_is_in_range_to
        i16_is_in_range_to_inclusive
    }

    range_tests! { u32:
        u32_is_in_single_value_range
        u32_is_in_range
        u32_is_in_range_from
        u32_is_in_range_inclusive
        u32_is_in_range_to
        u32_is_in_range_to_inclusive
    }

    range_tests! { i32:
        i32_is_in_single_value_range
        i32_is_in_range
        i32_is_in_range_from
        i32_is_in_range_inclusive
        i32_is_in_range_to
        i32_is_in_range_to_inclusive
    }

    range_tests! { u64:
        u64_is_in_single_value_range
        u64_is_in_range
        u64_is_in_range_from
        u64_is_in_range_inclusive
        u64_is_in_range_to
        u64_is_in_range_to_inclusive
    }

    range_tests! { i64:
        i64_is_in_single_value_range
        i64_is_in_range
        i64_is_in_range_from
        i64_is_in_range_inclusive
        i64_is_in_range_to
        i64_is_in_range_to_inclusive
    }

    range_tests! { u128:
        u128_is_in_single_value_range
        u128_is_in_range
        u128_is_in_range_from
        u128_is_in_range_inclusive
        u128_is_in_range_to
        u128_is_in_range_to_inclusive
    }

    range_tests! { i128:
        i128_is_in_single_value_range
        i128_is_in_range
        i128_is_in_range_from
        i128_is_in_range_inclusive
        i128_is_in_range_to
        i128_is_in_range_to_inclusive
    }

    range_tests! { usize:
        usize_is_in_single_value_range
        usize_is_in_range
        usize_is_in_range_from
        usize_is_in_range_inclusive
        usize_is_in_range_to
        usize_is_in_range_to_inclusive
    }

    range_tests! { isize:
        isize_is_in_single_value_range
        isize_is_in_range
        isize_is_in_range_from
        isize_is_in_range_inclusive
        isize_is_in_range_to
        isize_is_in_range_to_inclusive
    }

    #[test]
    fn u8_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|fate| {
                stat!("u8(..)", "{}", dice::u8(..).roll(fate));
                stat!("u8(..=9)", "{}", dice::u8(..=9).roll(fate));
                stat!("u8(100..=199)", "{}", dice::u8(100..=199).roll(fate));
            })
    }

    #[test]
    fn uni_u8_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|fate| {
                stat!("uni_u8(..)", "{}", dice::uni_u8(..).roll(fate));
                stat!("uni_u8(..=9)", "{}", dice::uni_u8(..=9).roll(fate));
                stat!(
                    "uni_u8(100..=199)",
                    "{}",
                    dice::uni_u8(100..=199).roll(fate),
                );
            })
    }

    #[test]
    fn i8_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|fate| {
                stat!("i8(..)", "{}", dice::i8(..).roll(fate));
                stat!("i8(-4..=5)", "{}", dice::i8(-4..=5).roll(fate));
            })
    }

    #[test]
    fn uni_i8_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|fate| {
                stat!("uni_i8(..)", "{}", dice::uni_i8(..).roll(fate));
                stat!("uni_i8(-4..=5)", "{}", dice::uni_i8(-4..=5).roll(fate));
            })
    }
}
