use crate::prelude::*;

/// Generates `true` or `false` with the same probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let true_or_false = fate.roll(dice::bool());
/// assert!(true_or_false == true || true_or_false == false);
/// ```
pub fn bool() -> impl Die<bool> {
    dice::one_of().two(false, true)
}

/// Generates `true` or `false` with probabilities based on the given weights.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let more_often_true_than_false = fate.roll(dice::weighted_bool(10, 1));
/// assert!(more_often_true_than_false == true || more_often_true_than_false == false);
/// ```
pub fn weighted_bool(false_weight: u32, true_weight: u32) -> impl Die<bool> {
    dice::weighted_one_of().two((false_weight, false), (true_weight, true))
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn bool_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                stat!("bool()", "{}", fate.roll(dice::bool()));
            })
    }

    #[test]
    fn weighted_bool_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                stat!(
                    "weighted_bool(1, 2)",
                    "{}",
                    fate.roll(dice::weighted_bool(1, 2)),
                );
                stat!(
                    "weighted_bool(10, 1)",
                    "{}",
                    fate.roll(dice::weighted_bool(9, 1)),
                );
            })
    }
}
