//! Dicetest is a framework for writing tests with pseudorandom generated test data.
//!
//! The [README] shows how to write a test with Dicetest. For more information, see the [GUIDE].
//!
//! [README]: https://github.com/jakoschiko/dicetest/blob/master/README.md
//! [GUIDE]: https://github.com/jakoschiko/dicetest/blob/master/GUIDE.md

// This crate makes assumptions regarding the pointer width. The following conditional error
// prevents the compilation for unsupported pointer widths.
//
// See https://github.com/rust-lang/rfcs/issues/1748
#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Only targets with pointer width 32 and 64 are currently supported");

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
