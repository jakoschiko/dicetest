//! Provides the random value generator traits `GenOnce` and `Gen` for generating different kind
//! of data.

mod prng;
pub use self::prng::Prng;

mod limit;
pub use self::limit::Limit;

mod dice;
pub use self::dice::Dice;

mod gen_once;
pub use self::gen_once::GenOnce;

#[allow(clippy::module_inception)]
mod gen;
pub use self::gen::Gen;

pub mod adapters;