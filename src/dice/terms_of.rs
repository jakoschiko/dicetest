use crate::prelude::*;

macro_rules! fn_terms_of_integer {
    (
        $integer:ident,
        $terms_of_integer:ident,
        $uni_integer:ident
    ) => {
        /// Generates `count` integers that sum up to `sum`.
        ///
        /// # Panics
        ///
        /// Panics if `sum != 0 && count == 0` is true.
        ///
        /// # Examples
        ///
        /// This example generates terms without panicking:
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
        /// let count = 10;
        /// let terms = fate.roll(dice::terms_of_u8(sum, count));
        ///
        /// assert_eq!(count, terms.len());
        /// assert_eq!(sum, terms.into_iter().sum());
        /// ```
        ///
        /// This example panics:
        ///
        /// ```should_panic
        /// use dicetest::prelude::*;
        ///
        /// // Oh no, panic!
        /// let _terms_die = dice::terms_of_u8(1, 0);
        /// ```
        pub fn $terms_of_integer(sum: $integer, count: usize) -> impl Die<Vec<$integer>> {
            let no_solution_exists = sum != 0 && count == 0;
            assert!(
                !no_solution_exists,
                "Cannot generate {} terms that sum up to {}",
                count, sum,
            );

            // We use divide and conquer!
            fn set_terms(terms: &mut [$integer], sum: $integer, fate: &mut Fate) {
                if sum != 0 {
                    if terms.len() == 1 {
                        terms[0] = sum;
                    } else {
                        let left_sum = fate.roll(dice::$uni_integer(0..=sum));
                        let right_sum = sum - left_sum;

                        let middle_index = if terms.len() % 2 == 0 || fate.roll(dice::bool()) {
                            terms.len() / 2
                        } else {
                            terms.len() / 2 + 1
                        };

                        set_terms(&mut terms[0..middle_index], left_sum, fate);
                        set_terms(&mut terms[middle_index..], right_sum, fate);
                    }
                }
            }

            dice::from_fn(move |mut fate| {
                let mut terms = vec![0; count];

                if !terms.is_empty() {
                    set_terms(&mut terms, sum, &mut fate);
                }

                terms
            })
        }
    };
}

fn_terms_of_integer! { u8, terms_of_u8, uni_u8 }
fn_terms_of_integer! { u16, terms_of_u16, uni_u16 }
fn_terms_of_integer! { u32, terms_of_u32, uni_u32 }
fn_terms_of_integer! { u64, terms_of_u64, uni_u64 }
fn_terms_of_integer! { u128, terms_of_u128, uni_u128 }
fn_terms_of_integer! { usize, terms_of_usize, uni_usize }

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    macro_rules! terms_of_integer_tests {
        (
            $integer:ident,
            $terms_of_integer:ident:
            $terms_of_integer_returns_the_expected_sum_and_count:ident
        ) => {
            #[test]
            fn $terms_of_integer_returns_the_expected_sum_and_count() {
                Dicetest::repeatedly().run(|mut fate| {
                    let expected_count = fate.roll(dice::length(..));
                    let expected_sum = if expected_count == 0 {
                        0
                    } else {
                        fate.roll(dice::$integer(..))
                    };

                    let terms = fate.roll(dice::$terms_of_integer(expected_sum, expected_count));

                    let actual_sum = terms.iter().sum();
                    let actual_count = terms.len();

                    assert_eq!(expected_sum, actual_sum);
                    assert_eq!(expected_count, actual_count);
                })
            }
        };
    }

    terms_of_integer_tests! { u8, terms_of_u8:
        terms_of_u8_returns_the_expected_sum_and_count
    }

    terms_of_integer_tests! { u16, terms_of_u16:
        terms_of_u16_returns_the_expected_sum_and_count
    }

    terms_of_integer_tests! { u32, terms_of_u32:
        terms_of_u32_returns_the_expected_sum_and_count
    }

    terms_of_integer_tests! { u64, terms_of_u64:
        terms_of_u64_returns_the_expected_sum_and_count
    }

    terms_of_integer_tests! { u128, terms_of_u128:
        terms_of_u128_returns_the_expected_sum_and_count
    }

    terms_of_integer_tests! { usize, terms_of_usize:
        terms_of_usize_returns_the_expected_sum_and_count
    }

    #[test]
    fn terms_of_u64_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                let sum = 8;
                let count = 4;
                let terms = fate.roll(dice::terms_of_u64(sum, count));
                stat_debug!(terms);
                stat!(
                    "for all term t: t > 0",
                    "{}",
                    terms.iter().all(|&term| term > 0),
                );
                stat!(
                    "for all term t: t >= 0.10 * sum / term_count",
                    "{}",
                    terms.iter().all(|&term| term * 10 * count as u64 >= sum),
                );
                stat!(
                    "for all term t: t >= 0.33 * sum / term_count",
                    "{}",
                    terms.iter().all(|&term| term * 3 * count as u64 >= sum),
                );
                stat!(
                    "for all term t: t >= 0.50 * sum / term_count",
                    "{}",
                    terms.iter().all(|&term| term * 2 * count as u64 >= sum),
                );
                stat!(
                    "for all term t: t >= 0.66 * sum / term_count",
                    "{}",
                    terms.iter().all(|&term| term * 3 * count as u64 >= sum * 2),
                );
                stat!(
                    "for all term t: t >= 0.90 * sum / term_count",
                    "{}",
                    terms
                        .iter()
                        .all(|&term| term * 10 * count as u64 >= sum * 9),
                );
                stat!(
                    "for any term t: t >= 0.10 * sum",
                    "{}",
                    terms.iter().any(|&term| term * 10_u64 >= sum),
                );
                stat!(
                    "for any term t: t >= 0.33 * sum",
                    "{}",
                    terms.iter().any(|&term| term * 3_u64 >= sum),
                );
                stat!(
                    "for any term t: t >= 0.50 * sum",
                    "{}",
                    terms.iter().any(|&term| term * 2_u64 >= sum),
                );
                stat!(
                    "for any term t: t >= 0.66 * sum",
                    "{}",
                    terms.iter().any(|&term| term * 3_u64 >= sum * 2),
                );
                stat!(
                    "for any term t: t >= 0.90 * sum",
                    "{}",
                    terms.iter().any(|&term| term * 10_u64 >= sum * 9),
                );
                stat!("greatest terms", "{:?}", {
                    let max = terms.iter().max();
                    terms
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(index, term)| {
                                if Some(term) == max {
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
