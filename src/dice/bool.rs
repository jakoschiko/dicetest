use crate::prelude::dice::*;

/// Generates `true` or `false` with the same probability.
pub fn bool() -> impl Die<bool> {
    dice::one_of_2(false, true)
}

/// Generates `true` or `false` with probabilities based on the given weights.
pub fn weighted_bool(false_weight: u32, true_weight: u32) -> impl Die<bool> {
    dice::weighted_one_of_2((false_weight, false), (true_weight, true))
}
