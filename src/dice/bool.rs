use crate::prelude::dice::*;

/// Generates `true` or `false` with the same probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
///
/// Fate::run(&mut prng, limit, |fate| {
///     let true_or_false = dice::bool().roll(fate);
///     assert!(true_or_false == true || true_or_false == false);
/// });
/// ```
pub fn bool() -> impl Die<bool> {
    dice::one_of_2(false, true)
}

/// Generates `true` or `false` with probabilities based on the given weights.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
///
/// Fate::run(&mut prng, limit, |fate| {
///     let more_often_true_than_false = dice::weighted_bool(10, 1).roll(fate);
///     assert!(more_often_true_than_false == true || more_often_true_than_false == false);
/// });
/// ```
pub fn weighted_bool(false_weight: u32, true_weight: u32) -> impl Die<bool> {
    dice::weighted_one_of_2((false_weight, false), (true_weight, true))
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;

    #[test]
    fn bool_calc_stats() {
        dicetest!(Config::default().with_passes(0), |fate| {
            stat!("bool()", "{}", dice::bool().roll(fate));
        })
    }

    #[test]
    fn weighted_bool_calc_stats() {
        dicetest!(Config::default().with_passes(0), |fate| {
            stat!(
                "weighted_bool(1, 2)",
                "{}",
                dice::weighted_bool(1, 2).roll(fate),
            );
            stat!(
                "weighted_bool(10, 1)",
                "{}",
                dice::weighted_bool(9, 1).roll(fate),
            );
        })
    }
}
