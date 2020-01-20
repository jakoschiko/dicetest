use crate::prelude::dice::*;

/// Generates `true` or `false` with the same probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let true_or_false = dice::bool().sample();
/// assert!(true_or_false == true || true_or_false == false);
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
/// let more_often_true_than_false = dice::weighted_bool(10, 1).sample();
/// assert!(more_often_true_than_false == true || more_often_true_than_false == false);
/// ```
pub fn weighted_bool(false_weight: u32, true_weight: u32) -> impl Die<bool> {
    dice::weighted_one_of_2((false_weight, false), (true_weight, true))
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;

    #[test]
    fn bool_calc_stats() {
        dicetest!(Config::default().with_passes(0), |mut fate| {
            stat!("bool()", "{}", fate.roll(dice::bool()));
        })
    }

    #[test]
    fn weighted_bool_calc_stats() {
        dicetest!(Config::default().with_passes(0), |mut fate| {
            stat!(
                "weighted_bool(1, 2)",
                "{}",
                fate.roll(dice::weighted_bool(1, 2))
            );
            stat!(
                "weighted_bool(10, 1)",
                "{}",
                fate.roll(dice::weighted_bool(9, 1))
            );
        })
    }
}
