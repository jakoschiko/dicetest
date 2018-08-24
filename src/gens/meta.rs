use ::rng::Rng;
use ::gen::{Params, Gen};
use ::gens;

/// Generates a clone of the `Rng` passed to the generator method.
///
/// This generator should only be used for meta-purposes because it does not alter the original
/// `Rng`. If you need a `Rng` as a source of randomness, you should use `gens::rng_fork` instead.
pub fn reveal_rng() -> impl Gen<Rng> {
    gens::from_fn(|rng, _| rng.clone())
}

/// Generates a clone of the `Params` passed to the generator method.
pub fn reveal_params() -> impl Gen<Params> {
    gens::from_fn(|_, params| params.clone())
}

/// Generates the `Params::max_size` passed to the generator method.
pub fn reveal_max_size() -> impl Gen<usize> {
    gens::from_fn(|_, params| params.max_size)
}
