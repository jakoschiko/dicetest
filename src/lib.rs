//! Framework for writing tests with randomly generated test data.
//!
//! See [the readme] and the [the guide] for more information.
//!
//! [the readme]: https://github.com/jakoschiko/dicetest/blob/main/README.md
//! [the guide]: https://github.com/jakoschiko/dicetest/blob/main/GUIDE.md
//!
//! # Example
//!
//! ```no_run
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn result_of_bubble_sort_is_sorted() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let mut v = fate.roll(dice::vec(dice::u8(..), ..));
//!         hint!("unsorted: {:?}", v);
//!
//!         v.sort();
//!         hint!("  sorted: {:?}", v);
//!
//!         let is_sorted = v.windows(2).all(|w| w[0] <= w[1]);
//!         assert!(is_sorted);
//!     })
//! }
//! ```
//!
//! # Environment variables
//!
//! See the documentation of [`Dicetest`] for a full list of supported environment variables.
//!
//! # Feature flags
//!
//! There are several feature flags for disabling runtime overhead or enabling additional
//! features at compile time.
//!
//! ### `hints` (enabled by default)
//!
//! Enables or disables the hints feature at compile time. If disabled,
//! all hints operations are no-ops.
//!
//! ### `stats` (enabled by default)
//!
//! Enables or disables the stats feature at compile time. If disabled,
//! all stats operations are no-ops.
//!
//! ### `rand_core` (disabled by default)
//!
//! If enabled, `dicetest::Prng` and `dicetest::Fate` implements the `rand_core::RngCore`
//! trait.
//!
//! ### `rand` (disabled by default)
//!
//! If enabled, `Fate::roll_distribution` and `dice::from_distribution` are available.
//! This allows to generate values and create `Die`s from implementations
//! of `rand::distributions::Distribution`.
//!
//! ```
//! # #[cfg(feature = "rand")] {
//! use dicetest::prelude::*;
//! use dicetest::{Limit, Prng};
//!
//! let mut prng = Prng::from_seed(0x5EED.into());
//! let limit = Limit(5);
//! let mut fate = Fate::new(&mut prng, limit);
//!
//! // Generate a value from a `rand::distributions::Distribution`
//! let byte: u8 = fate.roll_distribution(rand::distributions::Standard);
//! println!("{:?}", byte);
//! // Output: 28
//!
//! // Create a `Die` from a `rand::distributions::Distribution`
//! let byte_die = dice::from_distribution(rand::distributions::Standard);
//! let bytes_die = dice::vec(byte_die, 1..);
//! let bytes: Vec<u8> = fate.roll(bytes_die);
//! println!("{:?}", bytes);
//! // Output: [236, 205, 151, 229]
//! # }
//! ```

// This crate makes assumptions regarding the pointer width. The following conditional error
// prevents the compilation for unsupported pointer widths.
//
// See https://github.com/rust-lang/rfcs/issues/1748
#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Only targets with pointer width 32 and 64 are currently supported");

mod macros;

mod util;

mod seed;
pub use seed::Seed;

mod prng;
pub use prng::Prng;

mod codie;
pub use codie::Codie;

mod limit;
pub use limit::Limit;

mod fate;
pub use fate::Fate;

mod die_once;
pub use die_once::DieOnce;

mod die;
pub use die::Die;

pub mod adapters;

pub mod codice;

pub mod dice;

pub mod hints;

pub mod stats;

pub mod runner;

mod frontend;
pub use frontend::Dicetest;

pub mod prelude;

#[cfg(test)]
mod asserts;

// Test examples from the README.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

// Test examples from the GUIDE.
#[doc = include_str!("../GUIDE.md")]
#[cfg(doctest)]
pub struct GuideDoctests;
