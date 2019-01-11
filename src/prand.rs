//! Provides the types `Seed` and `Prng` for using pseudorandomness.

mod seed;
pub use self::seed::Seed;

mod prng;
pub use self::prng::Prng;
