use crate::prelude::gens::*;

// Generates the next number of the `Prng` that will be passed to the generator.
pub fn prng_next_number() -> impl Gen<u64> {
    gens::from_fn(|dice| dice.prng.next_number())
}

// Generates a fork of the `Prng` that will be passed to the generator.
pub fn prng_fork() -> impl Gen<Prng> {
    gens::from_fn(|dice| dice.prng.fork())
}