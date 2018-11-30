use crate::prelude::gens::*;

// Generates the next number of the `Rng` passed to the generator method.
pub fn rng_next() -> impl Gen<u64> {
    gens::from_fn(|dice| dice.rng.next())
}

// Generates a fork of the `Rng` passed to the generator method.
pub fn rng_fork() -> impl Gen<Rng> {
    gens::from_fn(|dice| dice.rng.fork())
}