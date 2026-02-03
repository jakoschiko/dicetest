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
//! fn result_of_sort_is_sorted() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let mut vec: Vec<u8> = fate.roll(die());
//!         hint!("unsorted: {:?}", vec);
//!
//!         vec.sort();
//!         hint!("  sorted: {:?}", vec);
//!
//!         let is_sorted = vec.windows(2).all(|w| w[0] <= w[1]);
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
//! #### `derive` (disabled by default)
//!
//! If enabled, a derive macro for [`Dice`] is available.
//!
//! #### `hints` (enabled by default)
//!
//! Enables or disables the hints feature at compile time. If disabled,
//! all hints operations are no-ops.
//!
//! #### `stats` (enabled by default)
//!
//! Enables or disables the stats feature at compile time. If disabled,
//! all stats operations are no-ops.
//!
//! #### `rand_core` (disabled by default)
//!
//! If enabled, [`Prng`] and [`Fate`] implements the [`rand_core::RngCore`]
//! trait.
//!
//! #### `rand` (disabled by default)
//!
//! If enabled, [`dice::from_distribution`] is available. It allows to create a [`Die`]
//! based on a [`rand::distributions::Distribution`].

// This allows us to add annotations to feature-gated items.
#![cfg_attr(docsrs, feature(doc_cfg))]

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

mod dice_trait;
pub use dice_trait::{Dice, die};

#[cfg(feature = "derive")]
mod dice_derive;
#[cfg(feature = "derive")]
pub use dice_derive::Dice;

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
