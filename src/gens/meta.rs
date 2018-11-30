use crate::prelude::gens::*;

/// Generates a clone of the `Prng` passed to the generator method.
///
/// This generator should only be used for meta-purposes because it does not alter the original
/// `Prng`. If you need an `Prng` as a source of randomness, you should use `gens::rng_fork`
/// instead.
pub fn reveal_prng() -> impl Gen<Prng> {
    gens::from_fn(|dice| dice.prng.clone())
}

/// Generates the `Limit` passed to the generator method.
pub fn reveal_limit() -> impl Gen<Limit> {
    gens::from_fn(|dice| dice.limit())
}
