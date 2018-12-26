use crate::prelude::gens::*;

// Generates the next number of the `Prng` passed to the generator method.
pub fn prng_next_number() -> impl Gen<u64> {
    gens::from_fn(|dice| dice.prng.next_number())
}

// Generates a fork of the `Prng` passed to the generator method.
pub fn prng_fork() -> impl Gen<Prng> {
    gens::from_fn(|dice| dice.prng.fork())
}