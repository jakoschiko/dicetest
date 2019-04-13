//! Dicetest is a framework for writing tests with pseudorandom generated test data.
//!
//! The [README] shows how to write a test with Dicetest. For more information, see the [GUIDE].
//!
//! [README]: https://github.com/jakoschiko/dicetest/blob/master/README.md
//! [GUIDE]: https://github.com/jakoschiko/dicetest/blob/master/GUIDE.md

mod macros;

mod util;

pub mod prand;

pub mod hints;

pub mod stats;

pub mod codie;

pub mod codice;

pub mod die;

pub mod dice;

pub mod runner;

pub mod formatter;

pub mod checker;

pub mod prelude;

#[cfg(test)]
mod asserts;
