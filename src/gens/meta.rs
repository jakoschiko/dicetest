use ::rng::Rng;
use ::gen::{Size, Gen};
use ::gens;

/// Generates a clone of the `Rng` passed to the generator method.
///
/// This generator should only be used for meta-purposes because it does not alter the original
/// `Rng`. If you need an `Rng` as a source of randomness, you should use `gens::rng_fork` instead.
pub fn reveal_rng() -> impl Gen<Rng> {
    gens::from_fn(|rng, _| rng.clone())
}

/// Generates the `Size` passed to the generator method.
pub fn reveal_size() -> impl Gen<Size> {
    gens::from_fn(|_, size| size)
}
