//! Provides the traits `DieOnce` and `Die` for generating pseudorandom values.

mod seed;
pub use self::seed::Seed;

mod prng;
pub use self::prng::Prng;

mod limit;
pub use self::limit::Limit;

mod fate;
pub use self::fate::Fate;

mod die_once;
pub use self::die_once::DieOnce;

#[allow(clippy::module_inception)]
mod die;
pub use self::die::Die;

pub mod adapters;
