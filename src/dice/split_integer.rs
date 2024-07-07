use std::fmt::Debug;

use crate::prelude::*;

macro_rules! fn_split_integer {
    (
        $integer:ident,
        $split_integer:ident,
        $split_integer_n:ident
    ) => {
        /// Generates `N` integers that sum up to `sum`.
        ///
        /// # Panics
        ///
        /// Panics if `sum != 0 && N == 0` is true.
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
        /// let mut fate = Fate::new(&mut prng, limit);
        ///
        /// let sum = 20;
        /// const N: usize = 10;
        /// let parts = fate.roll(dice::split_u8::<N>(sum));
        ///
        /// assert_eq!(N, parts.len());
        /// assert_eq!(sum, parts.into_iter().sum());
        /// ```
        ///
        /// This example panics:
        ///
        /// ```should_panic
        /// use dicetest::prelude::*;
        ///
        /// // Oh no, panic!
        /// let _parts_die = dice::split_u8::<0>(1);
        /// ```
        #[track_caller]
        pub fn $split_integer<const N: usize>(sum: $integer) -> impl Die<[$integer; N]> {
            assert_solution_exists(sum, N);

            dice::from_fn(move |mut fate| {
                let mut parts = [0; N];
                if !parts.is_empty() {
                    split_integer_recur(&mut parts, sum, &mut fate);
                }
                parts
            })
        }

        /// Generates `n` integers that sum up to `sum`.
        ///
        /// # Panics
        ///
        /// Panics if `sum != 0 && n == 0` is true.
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
        /// let mut fate = Fate::new(&mut prng, limit);
        ///
        /// let sum = 20;
        /// let n = 10;
        /// let parts = fate.roll(dice::split_u8_n(sum, n));
        ///
        /// assert_eq!(n, parts.len());
        /// assert_eq!(sum, parts.into_iter().sum());
        /// ```
        ///
        /// This example panics:
        ///
        /// ```should_panic
        /// use dicetest::prelude::*;
        ///
        /// // Oh no, panic!
        /// let _parts_die = dice::split_u8_n(1, 0);
        /// ```
        #[track_caller]
        pub fn $split_integer_n(sum: $integer, n: usize) -> impl Die<Vec<$integer>> {
            assert_solution_exists(sum, n);

            dice::from_fn(move |mut fate| {
                let mut parts = vec![0; n];
                if !parts.is_empty() {
                    split_integer_recur(&mut parts, sum, &mut fate);
                }
                parts
            })
        }
    };
}

fn_split_integer! { u8, split_u8, split_u8_n }
fn_split_integer! { u16, split_u16, split_u16_n }
fn_split_integer! { u32, split_u32, split_u32_n }
fn_split_integer! { u64, split_u64, split_u64_n }
fn_split_integer! { u128, split_u128, split_u128_n }
fn_split_integer! { usize, split_usize, split_usize_n }

#[track_caller]
fn assert_solution_exists<I: Integer + Debug>(sum: I, n: usize) {
    let no_solution_exists = sum != I::ZERO && n == 0;
    assert!(
        !no_solution_exists,
        "Cannot generate {n} parts that sum up to {sum:?}"
    );
}

// We use divide and conquer!
fn split_integer_recur<I: Integer>(parts: &mut [I], sum: I, fate: &mut Fate) {
    if sum != I::ZERO {
        if parts.len() == 1 {
            parts[0] = sum;
        } else {
            let (left_sum, right_sum) = sum.split(fate);

            if parts.len() == 2 {
                parts[0] = left_sum;
                parts[1] = right_sum;
            } else {
                let middle_index = if parts.len() % 2 == 0 || fate.roll(dice::bool()) {
                    parts.len() / 2
                } else {
                    parts.len() / 2 + 1
                };

                split_integer_recur(&mut parts[0..middle_index], left_sum, fate);
                split_integer_recur(&mut parts[middle_index..], right_sum, fate);
            }
        }
    }
}

trait Integer: Copy + Eq + Sized {
    const ZERO: Self;

    fn split(self, fate: &mut Fate) -> (Self, Self);

    #[cfg(test)]
    fn add(self, rhs: Self) -> Self;

    #[cfg(test)]
    fn roll(fate: &mut Fate) -> Self;
}

macro_rules! impl_integer {
    (
        $integer:ident,
        $die:ident,
        $uni_die:ident
    ) => {
        impl Integer for $integer {
            const ZERO: Self = 0;

            fn split(self, fate: &mut Fate) -> (Self, Self) {
                let left = fate.roll(dice::$uni_die(0..=self));
                (left, self - left)
            }

            #[cfg(test)]
            fn add(self, rhs: Self) -> Self {
                self + rhs
            }

            #[cfg(test)]
            fn roll(fate: &mut Fate) -> Self {
                fate.roll(dice::$die(..))
            }
        }
    };
}

impl_integer! { u8, u8, uni_u8 }
impl_integer! { u16, u16, uni_u16 }
impl_integer! { u32, u32, uni_u32 }
impl_integer! { u64, u64, uni_u64 }
impl_integer! { u128, u128, uni_u128 }
impl_integer! { usize, usize, uni_usize }

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::Integer;

    fn sum<I: Integer>(parts: &[I]) -> I {
        parts.iter().copied().fold(I::ZERO, I::add)
    }

    macro_rules! split_integer_tests {
        (
            $integer:ident,
            $split_integer:ident,
            $split_integer_n:ident:
            $split_integer_returns_the_expected_sum_and_count:ident
            $split_integer_n_returns_the_expected_sum_and_count:ident
            $split_integer_with_zero:ident
        ) => {
            #[test]
            fn $split_integer_with_zero() {
                Dicetest::repeatedly().run(|mut fate| {
                    let parts = fate.roll(dice::$split_integer::<0>($integer::ZERO));
                    assert_eq!(parts, []);
                })
            }

            #[test]
            fn $split_integer_returns_the_expected_sum_and_count() {
                Dicetest::repeatedly().run(|mut fate| {
                    let expected_sum = $integer::roll(&mut fate);

                    let parts = fate.roll(dice::$split_integer::<1>(expected_sum));
                    let actual_sum = sum(&parts);
                    assert_eq!(expected_sum, actual_sum);

                    let parts = fate.roll(dice::$split_integer::<2>(expected_sum));
                    let actual_sum = sum(&parts);
                    assert_eq!(expected_sum, actual_sum);

                    let parts = fate.roll(dice::$split_integer::<3>(expected_sum));
                    let actual_sum = sum(&parts);
                    assert_eq!(expected_sum, actual_sum);

                    let parts = fate.roll(dice::$split_integer::<4>(expected_sum));
                    let actual_sum = sum(&parts);
                    assert_eq!(expected_sum, actual_sum);

                    let parts = fate.roll(dice::$split_integer::<5>(expected_sum));
                    let actual_sum = sum(&parts);
                    assert_eq!(expected_sum, actual_sum);
                })
            }

            #[test]
            fn $split_integer_n_returns_the_expected_sum_and_count() {
                Dicetest::repeatedly().run(|mut fate| {
                    let expected_count = fate.roll(dice::length(..));
                    let expected_sum = if expected_count == 0 {
                        $integer::ZERO
                    } else {
                        $integer::roll(&mut fate)
                    };

                    let parts = fate.roll(dice::$split_integer_n(expected_sum, expected_count));

                    let actual_sum = sum(&parts);
                    let actual_count = parts.len();

                    assert_eq!(expected_sum, actual_sum);
                    assert_eq!(expected_count, actual_count);
                })
            }
        };
    }

    split_integer_tests! { u8, split_u8, split_u8_n:
        split_u8_returns_the_expected_sum_and_count
        split_u8_n_returns_the_expected_sum_and_count
        split_u8_with_zero
    }

    split_integer_tests! { u16, split_u16, split_u16_n:
        split_u16_returns_the_expected_sum_and_count
        split_u16_n_returns_the_expected_sum_and_count
        split_u16_with_zero
    }

    split_integer_tests! { u32, split_u32, split_u32_n:
        split_u32_returns_the_expected_sum_and_count
        split_u32_n_returns_the_expected_sum_and_count
        split_u32_with_zero
    }

    split_integer_tests! { u64, split_u64, split_u64_n:
        split_u64_returns_the_expected_sum_and_count
        split_u64_n_returns_the_expected_sum_and_count
        split_u64_with_zero
    }

    split_integer_tests! { u128, split_u128, split_u128_n:
        split_u128_returns_the_expected_sum_and_count
        split_u128_n_returns_the_expected_sum_and_count
        split_u128_with_zero
    }

    split_integer_tests! { usize, split_usize, split_usize_n:
        split_usize_returns_the_expected_sum_and_count
        split_usize_n_returns_the_expected_sum_and_count
        split_usize_with_zero
    }

    #[test]
    fn split_u64_n_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                let sum = 8;
                let n = 4;
                let parts = fate.roll(dice::split_u64_n(sum, n));
                stat_debug!(parts);
                stat!(
                    "for all part p: p > 0",
                    "{}",
                    parts.iter().all(|&part| part > 0),
                );
                stat!(
                    "for all part p: p >= 0.10 * sum / part_count",
                    "{}",
                    parts.iter().all(|&part| part * 10 * n as u64 >= sum),
                );
                stat!(
                    "for all part p: p >= 0.33 * sum / part_count",
                    "{}",
                    parts.iter().all(|&part| part * 3 * n as u64 >= sum),
                );
                stat!(
                    "for all part p: p >= 0.50 * sum / part_count",
                    "{}",
                    parts.iter().all(|&part| part * 2 * n as u64 >= sum),
                );
                stat!(
                    "for all part p: p >= 0.66 * sum / part_count",
                    "{}",
                    parts.iter().all(|&part| part * 3 * n as u64 >= sum * 2),
                );
                stat!(
                    "for all part p: p >= 0.90 * sum / part_count",
                    "{}",
                    parts.iter().all(|&part| part * 10 * n as u64 >= sum * 9),
                );
                stat!(
                    "for any part p: p >= 0.10 * sum",
                    "{}",
                    parts.iter().any(|&part| part * 10_u64 >= sum),
                );
                stat!(
                    "for any part p: p >= 0.33 * sum",
                    "{}",
                    parts.iter().any(|&part| part * 3_u64 >= sum),
                );
                stat!(
                    "for any part p: p >= 0.50 * sum",
                    "{}",
                    parts.iter().any(|&part| part * 2_u64 >= sum),
                );
                stat!(
                    "for any part p: p >= 0.66 * sum",
                    "{}",
                    parts.iter().any(|&part| part * 3_u64 >= sum * 2),
                );
                stat!(
                    "for any part p: p >= 0.90 * sum",
                    "{}",
                    parts.iter().any(|&part| part * 10_u64 >= sum * 9),
                );
                stat!("greatest parts", "{:?}", {
                    let max = parts.iter().max();
                    parts
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(index, part)| {
                                if Some(part) == max {
                                    Some(index)
                                } else {
                                    None
                                }
                            },
                        )
                        .collect::<Vec<_>>()
                })
            })
    }
}
