//! Provides the traits `DieOnce` and `Die` for generating pseudorandom values.

mod limit;
pub use limit::Limit;

mod fate;
pub use fate::Fate;

mod die_once;
pub use die_once::DieOnce;

#[allow(clippy::module_inception)]
mod die;
pub use die::Die;

pub mod adapters;
