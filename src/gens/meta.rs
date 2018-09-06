use ::rng::Rng;
use ::gen::{Params, Gen};
use ::gens;

/// Generates a clone of the `Rng` passed to the generator method.
///
/// This generator should only be used for meta-purposes because it does not alter the original
/// `Rng`. If you need an `Rng` as a source of randomness, you should use `gens::rng_fork` instead.
pub fn reveal_rng() -> impl Gen<Rng> {
    gens::from_fn(|rng, _| rng.clone())
}

/// Generates a clone of the `Params` passed to the generator method.
pub fn reveal_params() -> impl Gen<Params> {
    gens::from_fn(|_, params| params.clone())
}

/// Generates the `Params::size` passed to the generator method.
pub fn reveal_size() -> impl Gen<u64> {
    gens::from_fn(|_, params| params.size)
}
