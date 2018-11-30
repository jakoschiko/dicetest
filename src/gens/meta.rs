use crate::prelude::gens::*;

/// Generates a clone of the `Rng` passed to the generator method.
///
/// This generator should only be used for meta-purposes because it does not alter the original
/// `Rng`. If you need an `Rng` as a source of randomness, you should use `gens::rng_fork` instead.
pub fn reveal_rng() -> impl Gen<Rng> {
    gens::from_fn(|dice| dice.rng.clone())
}

/// Generates the `Limit` passed to the generator method.
pub fn reveal_limit() -> impl Gen<Limit> {
    gens::from_fn(|dice| dice.limit())
}
