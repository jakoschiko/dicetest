//! A runner for tests with pseudorandomly generated test data.
//!
//! The runner mainly exists for implementing [`Dicetest`]. You probably want to use [`Dicetest`]
//! instead of using the runner directly.
//!
//! The modules [`once`] and [`repeatedly`] contains runner functions with different strategies.
//!
//! [`Dicetest`]: crate::Dicetest

mod util;

mod limit_series;
use limit_series::LimitSeries;

mod error;
pub use error::Error;

pub mod once;

pub mod repeatedly;
