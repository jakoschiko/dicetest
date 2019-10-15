#![allow(clippy::float_cmp)]

use std::fmt::Debug;
use std::ops::{RangeFrom, RangeFull, RangeInclusive, RangeToInclusive};

use crate::prelude::dice::*;

/// Non-empty range for float generators `dice::f32` and `dice::f64`.
///
/// The float type is represented by `F`.
pub trait FloatRange<F> {
    /// Returns the inclusive lower bound and the inclusive upper bound that represent the range.
    ///
    /// # Panics
    ///
    /// Panics if the range is empty or it contains NaN.
    fn bounds(self) -> (F, F);
}

fn nan_range<I>(bounds: &(impl FloatRange<I> + Debug)) -> ! {
    panic!(
        "FloatRange is invalid because it contains NaN: {:?}",
        bounds
    )
}

fn empty_float_range<I>(bounds: &(impl FloatRange<I> + Debug)) -> ! {
    panic!(
        "FloatRange is invalid because it contains no values: {:?}",
        bounds
    )
}

macro_rules! impl_float_range {
    ($float:ident) => {
        impl FloatRange<$float> for $float {
            fn bounds(self) -> ($float, $float) {
                if self.is_nan() {
                    nan_range(&self)
                } else {
                    (self, self)
                }
            }
        }

        impl FloatRange<$float> for RangeFrom<$float> {
            fn bounds(self) -> ($float, $float) {
                if self.start.is_nan() {
                    nan_range(&self)
                } else {
                    (self.start, std::$float::INFINITY)
                }
            }
        }

        impl FloatRange<$float> for RangeFull {
            fn bounds(self) -> ($float, $float) {
                (std::$float::NEG_INFINITY, std::$float::INFINITY)
            }
        }

        impl FloatRange<$float> for RangeInclusive<$float> {
            fn bounds(self) -> ($float, $float) {
                if self.start().is_nan() || self.end().is_nan() {
                    nan_range(&self)
                } else if self.start() <= self.end() {
                    self.into_inner()
                } else {
                    empty_float_range(&self)
                }
            }
        }

        impl FloatRange<$float> for RangeToInclusive<$float> {
            fn bounds(self) -> ($float, $float) {
                if self.end.is_nan() {
                    nan_range(&self)
                } else {
                    (std::$float::NEG_INFINITY, self.end)
                }
            }
        }
    };
}

impl_float_range! { f32 }
impl_float_range! { f64 }

macro_rules! fn_float {
    (
        $float:ident,
        $int:ident,
        $float_util:ident,
        $arb_float:ident,
        $unit_float:ident,
        $open_unit_float:ident
    ) => {
        /// Generates a float inside the given range. Some special floats have a higher
        /// probability of being generated.
        ///
        /// # Panics
        ///
        /// Panics if the range or it contains NaN.
        pub fn $float(range: impl FloatRange<$float>) -> impl Die<$float> {
            // `FloatRange::bounds` guarantees that `lower <= upper` and both bounds are not NaN.
            let (lower, upper) = range.bounds();

            // Generates a value that is inside the range.
            // If `lower != upper` the generator will not
            // generate `INFINITY` or `NEG_INFINITY`. These
            // values may be generates by the generator
            // `maybe_special_value_die` instead.
            let regular_value_die = dice::from_fn(move |fate| {
                if lower == upper {
                    // Range contains only one value
                    lower
                } else {
                    // It holds `lower < upper`

                    let factor = $unit_float().roll(fate);

                    // Just ignore infinite values here. If the range contains infinite values,
                    // `maybe_special_value_die` is able to roll them.

                    let min = if lower == std::$float::NEG_INFINITY {
                        std::$float::MIN
                    } else {
                        // `lower` cannot be `INFINITY` because `lower < upper`
                        lower
                    };
                    let max = if upper == std::$float::INFINITY {
                        std::$float::MAX
                    } else {
                        // `upper` cannot be `INFINITY` because `lower < upper`
                        upper
                    };

                    $float_util::linear_ipol_float(factor, min, max)
                }
            });

            // Generates once in a while a special value that is inside the range.
            let maybe_special_value_die = {
                let border_value_die = dice::one_of_2(Some(lower), Some(upper));
                let const_value_die = {
                    let const_values = $float_util::SPECIAL_VALUES;
                    dice::one_of_slice(const_values).map(move |special_value| {
                        if lower <= special_value && special_value <= upper {
                            Some(special_value)
                        } else {
                            None
                        }
                    })
                };

                dice::weighted_one_of_die_3(
                    (10, dice::just(None)),
                    (1, border_value_die),
                    (1, const_value_die),
                )
            };

            dice::from_fn(move |fate| match maybe_special_value_die.roll(fate) {
                Some(special_value) => special_value,
                None => regular_value_die.roll(fate),
            })
        }

        /// Generates an arbitrary float (including NaN). Some special floats have a higher
        /// probability of being generated.
        pub fn $arb_float() -> impl Die<$float> {
            dice::weighted_one_of_die_2((10, $float(..)), (1, dice::just(std::$float::NAN)))
        }

        /// Generates a uniformly distributed float that lies inside the closed unit interval
        /// `[0, 1]`.
        pub fn $unit_float() -> impl Die<$float> {
            let numerator_die = dice::prng_next_number().map(|bits| bits as $int);

            dice::one_of_die_4(
                dice::from_fn(move |fate| {
                    const FACTOR: $float = 1.0 / std::$int::MAX as $float;
                    let numerator = numerator_die.roll(fate);
                    numerator as $float * FACTOR
                }),
                dice::just(0.0),
                dice::just(0.5),
                dice::just(1.0),
            )
        }

        /// Generates a uniformly distributed float that lies inside the open unit interval
        /// `[0, 1)`.
        pub fn $open_unit_float() -> impl Die<$float> {
            let numerator_die =
                dice::prng_next_number().map(|bits| (bits as $int) & $float_util::MAX_ONES);

            dice::one_of_die_3(
                dice::from_fn(move |fate| {
                    let numerator = numerator_die.roll(fate);
                    $float_util::open_unit_float(numerator)
                }),
                dice::just(0.0),
                dice::just($float_util::open_unit_float($float_util::MAX_ONES)),
            )
        }

        mod $float_util {
            // Some floats that could be considered as special (but not NaN).
            pub const SPECIAL_VALUES: &[$float] = &[
                0.0,
                -0.0,
                1.0,
                -1.0,
                std::$float::EPSILON,
                std::$float::INFINITY,
                std::$float::MAX,
                std::$float::MIN,
                std::$float::MIN_POSITIVE,
                std::$float::NEG_INFINITY,
                std::$float::consts::E,
                std::$float::consts::FRAC_1_PI,
                std::$float::consts::FRAC_2_PI,
                std::$float::consts::FRAC_2_SQRT_PI,
                std::$float::consts::FRAC_1_SQRT_2,
                std::$float::consts::FRAC_PI_2,
                std::$float::consts::FRAC_PI_3,
                std::$float::consts::FRAC_PI_4,
                std::$float::consts::FRAC_PI_6,
                std::$float::consts::FRAC_PI_8,
                std::$float::consts::LN_2,
                std::$float::consts::LN_10,
                std::$float::consts::LOG2_E,
                std::$float::consts::LOG10_E,
                std::$float::consts::PI,
                std::$float::consts::SQRT_2,
            ];

            // Represents the ones that the float type can maximally store.
            pub const MAX_ONES: $int = {
                let float_bits = std::mem::size_of::<$float>() * 8;
                let mantissa_bits = std::$float::MANTISSA_DIGITS as usize;
                std::$int::MAX >> (float_bits - mantissa_bits)
            };

            // Calculate a float that lies inside the range `[0, 1)` if
            // `numerator <= MAX_ONES` is true.
            pub fn open_unit_float(numerator: $int) -> $float {
                const DENOMINATOR: $int = MAX_ONES + 1;
                const FACTOR: $float = 1.0 / DENOMINATOR as $float;
                numerator as $float * FACTOR
            }

            // Calculates a linear interpolated float between `min` and `max` if
            //   - `factor`, `min`, `max` are finite numbers and
            //   - `min <= max` is true and
            //   - `0 <= factor && 0 factor <= 1` is true.
            pub fn linear_ipol_float(factor: $float, min: $float, max: $float) -> $float {
                // We want to calculate `min + factor * (max - min)`.
                // But because of the properties of float numbers, we need
                // some conditionals to guarantee that the result is between
                // `min` and `max`.
                if min.signum() == max.signum() {
                    // We tweak the rounding error by using different
                    // (but mathematically equivalent) expressions
                    // depending on where we want to interpolate.
                    if factor <= 0.5 {
                        // Hurray, we can use the orignal formula!
                        min + factor * (max - min)
                    } else {
                        // Similar to the orignal formula, but flipped.
                        max + (1.0 - factor) * (min - max)
                    }
                } else {
                    // It holds `min <= 0` and `0 <= max`.
                    // The result of `max - min` could be an infinite value.
                    // So we use a mathematically equivalent expression that
                    // doesn't need the difference.
                    min * (1.0 - factor) + factor * max
                }
            }
        }
    };
}

fn_float! { f32, u32, f32_util, any_f32, unit_f32, open_unit_f32 }
fn_float! { f64, u64, f64_util, any_f64, unit_f64, open_unit_f64 }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::tests::*;

    #[test]
    fn f32_util_open_unit_float_examples() {
        assert_eq!(0.0, f32_util::open_unit_float(0));

        // The binary number `0.111111...` (with 24 `1`s).
        let expected_max_result = {
            // Represents the exponent 2^-1.
            // That's the `0.1` of the binary number.
            let exponent_bits = 126 << 23;
            // All 23 bits of the mantisse are `1`.
            // These are the remaining 23 `1`s after the `0.1`.
            let max_mantisse = std::u32::MAX >> (32 - 23);
            f32::from_bits(exponent_bits | max_mantisse)
        };

        assert_eq!(
            expected_max_result,
            f32_util::open_unit_float(f32_util::MAX_ONES)
        );
    }

    #[test]
    fn f64_util_open_unit_float_examples() {
        assert_eq!(0.0, f64_util::open_unit_float(0));

        // The binary number `0.111111...` (with 53 `1`s).
        let expected_max_result = {
            // Represents the exponent 2^-1.
            // That's the `0.1` of the binary number.
            let exponent_bits = 1022 << 52;
            // All 52 bits of the mantisse are `1`.
            // These are the remaining 52 `1`s after the `0.1`.
            let max_mantisse = std::u64::MAX >> (64 - 52);
            f64::from_bits(exponent_bits | max_mantisse)
        };

        assert_eq!(
            expected_max_result,
            f64_util::open_unit_float(f64_util::MAX_ONES)
        );
    }

    fn range_contains_float<I, ID, B, BD, R>(
        fate: &mut Fate,
        range_data_die: BD,
        create_range: fn(B) -> R,
        float_die: fn(R) -> ID,
        is_in_range: fn(B, I) -> bool,
    ) where
        I: Debug,
        ID: DieOnce<I>,
        B: Copy + Debug,
        BD: DieOnce<B>,
        R: FloatRange<I> + Debug,
    {
        let range_data = range_data_die.roll_once(fate);
        hint_debug!(range_data);

        let range = create_range(range_data);
        hint_debug!(range);

        let float = float_die(range).roll_once(fate);
        hint_debug!(float);

        assert!(is_in_range(range_data, float));
    }

    macro_rules! range_tests {
        (
            $float:ident:
            $float_util:ident
            $unit_float:ident
            $open_unit_float:ident
            $unit_float_rolls_values_in_expected_range:ident
            $open_unit_float_rolls_values_in_expected_range:ident
            $float_util_linear_ipol_float_with_same_min_and_max:ident
            $float_util_linear_ipol_float_with_factor_limits:ident
            $float_util_linear_ipol_float_center_examples:ident
            $float_is_in_single_value_range:ident
            $float_is_in_range_from:ident
            $float_is_in_range_inclusive:ident
            $float_is_in_range_to_inclusive:ident
            $unit_float_calc_stats:ident
            $open_unit_float_calc_stats:ident
        ) => {
            #[test]
            fn $unit_float_rolls_values_in_expected_range() {
                dicetest!(|fate| {
                    let float = $unit_float().roll(fate);
                    hint_debug!(float);

                    assert!(float <= 1.0);
                    assert!(float >= 0.0);
                })
            }

            #[test]
            fn $open_unit_float_rolls_values_in_expected_range() {
                dicetest!(|fate| {
                    let float = $open_unit_float().roll(fate);
                    hint_debug!(float);

                    assert!(float < 1.0);
                    assert!(float >= 0.0);
                })
            }

            #[test]
            fn $float_util_linear_ipol_float_with_same_min_and_max() {
                dicetest!(|fate| {
                    let float = $float(std::$float::MIN..=std::$float::MAX).roll(fate);
                    let factor = $unit_float().roll(fate);

                    assert_eq!(float, $float_util::linear_ipol_float(factor, float, float));
                })
            }

            #[test]
            fn $float_util_linear_ipol_float_with_factor_limits() {
                dicetest!(|fate| {
                    let float1 = $float(std::$float::MIN..=std::$float::MAX).roll(fate);
                    let float2 = $float(std::$float::MIN..=std::$float::MAX).roll(fate);
                    let (min, max) = if float1 <= float2 {
                        (float1, float2)
                    } else {
                        (float2, float1)
                    };

                    assert_eq!(min, $float_util::linear_ipol_float(0.0, min, max));
                    assert_eq!(max, $float_util::linear_ipol_float(1.0, min, max));
                })
            }

            #[test]
            fn $float_util_linear_ipol_float_center_examples() {
                assert_eq!(
                    std::$float::MIN / 2.0,
                    $float_util::linear_ipol_float(0.5, std::$float::MIN, 0.0),
                );
                assert_eq!(
                    std::$float::MAX / 2.0,
                    $float_util::linear_ipol_float(0.5, 0.0, std::$float::MAX),
                );
                assert_eq!(
                    0.0,
                    $float_util::linear_ipol_float(0.5, std::$float::MIN, std::$float::MAX),
                );
            }

            #[test]
            fn $float_is_in_single_value_range() {
                dicetest!(|fate| {
                    range_contains_float(
                        fate,
                        $float(..),
                        |single| single,
                        $float,
                        |single, float| single == float,
                    );
                })
            }

            #[test]
            fn $float_is_in_range_from() {
                dicetest!(|fate| {
                    range_contains_float(
                        fate,
                        $float(..),
                        |lower| lower..,
                        $float,
                        |lower, float| lower <= float,
                    );
                })
            }

            #[test]
            fn $float_is_in_range_inclusive() {
                dicetest!(|fate| {
                    range_contains_float(
                        fate,
                        dice::array_2($float(..)).map(|[a, b]| (a.min(b), a.max(b))),
                        |(lower, upper)| lower..=upper,
                        $float,
                        |(lower, upper), float| lower <= float && float <= upper,
                    );
                })
            }

            #[test]
            fn $float_is_in_range_to_inclusive() {
                dicetest!(|fate| {
                    range_contains_float(
                        fate,
                        $float(..),
                        |upper| ..=upper,
                        $float,
                        |upper, float| float <= upper,
                    );
                })
            }

            #[test]
            fn $unit_float_calc_stats() {
                dicetest!(|fate| {
                    let float = $unit_float().roll(fate);

                    if float >= 0.5 {
                        stat!("float", "[0.5, 1]")
                    } else if float >= 0.25 {
                        stat!("float", "[0.25, 0.5)");
                    } else if float >= 0.125 {
                        stat!("float", "[0.125, 0.25)");
                    } else if float >= 0.0625 {
                        stat!("float", "[0.0625, 0.125)");
                    } else if float >= 0.03125 {
                        stat!("float", "[0.03125, 0.0625)");
                    } else {
                        stat!("float", "[0, 0.031255)");
                    }
                })
            }

            #[test]
            fn $open_unit_float_calc_stats() {
                dicetest!(|fate| {
                    let float = $open_unit_float().roll(fate);

                    if float >= 0.5 {
                        stat!("float", "[0.5, 1)")
                    } else if float >= 0.25 {
                        stat!("float", "[0.25, 0.5)");
                    } else if float >= 0.125 {
                        stat!("float", "[0.125, 0.25)");
                    } else if float >= 0.0625 {
                        stat!("float", "[0.0625, 0.125)");
                    } else if float >= 0.03125 {
                        stat!("float", "[0.03125, 0.0625)");
                    } else {
                        stat!("float", "[0, 0.031255)");
                    }
                })
            }
        };
    }

    range_tests! { f32:
        f32_util
        unit_f32
        open_unit_f32
        unit_f32_rolls_values_in_expected_range
        open_unit_f32_rolls_values_in_expected_range
        f32_util_linear_ipol_float_with_same_min_and_max
        f32_util_linear_ipol_float_with_factor_limits
        f32_util_linear_ipol_float_center_examples
        f32_is_in_single_value_range
        f32_is_in_range_from
        f32_is_in_range_inclusive
        f32_is_in_range_to_inclusive
        unit_f32_calc_stats
        open_unit_f32_calc_stats
    }

    range_tests! { f64:
        f64_util
        unit_f64
        open_unit_f64
        unit_f64_rolls_values_in_expected_range
        open_unit_f64_rolls_values_in_expected_range
        f64_util_linear_ipol_float_with_same_min_and_max
        f64_util_linear_ipol_float_with_factor_limits
        f64_util_linear_ipol_float_center_examples
        f64_is_in_single_value_range
        f64_is_in_range_from
        f64_is_in_range_inclusive
        f64_is_in_range_to_inclusive
        unit_f64_calc_stats
        open_unit_f64_calc_stats
    }
}
