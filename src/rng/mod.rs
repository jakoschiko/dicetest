//! Provides the random number generator `Rng` for integer numbers.
//! This generator is the base for more complex generators that generate different kind of data.

mod rng;
pub use self::rng::Rng;
