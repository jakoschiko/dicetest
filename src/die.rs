//! Provides the random value generator traits `DieOnce` and `Die` for generating different kind
//! of data.

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
