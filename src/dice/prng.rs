use crate::prelude::dice::*;

// Generates the next number of the `Prng` that will be passed to the generator.
pub fn prng_next_number() -> impl Die<u64> {
    dice::from_fn(|fate| fate.prng.next_number())
}

// Generates a fork of the `Prng` that will be passed to the generator.
pub fn prng_fork() -> impl Die<Prng> {
    dice::from_fn(|fate| fate.prng.fork())
}
