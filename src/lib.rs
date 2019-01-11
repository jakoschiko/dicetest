//! Dicetest is a framework for writing tests with pseudorandom generated test data.
//!
//! # Current state
//!
//! The author does not consider this crate as stable yet.
//!
//! # Simple example
//!
//! Here's an example of a broken sort function tested with dicetest:
//! ```
//! fn bubble_sort<T: Ord>(slice: &mut [T]) {
//!     let len = slice.len();
//!
//!     for _ in 0..len {
//!         for j in 1..len - 1 {
//!             let jpp = j + 1;
//!             if slice[j] > slice[jpp] {
//!                 slice.swap(j, jpp);
//!             }
//!         }
//!     }
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!     use dicetest::prelude::tests::*;
//!
//!     #[test]
//!     fn result_of_bubble_sort_is_sorted() {
//!         dicetest!(|fate| {
//!             let mut v = dice::vec(dice::u8(..), ..).roll(fate);
//!             hint!("unsorted: {:?}", v);
//!
//!             bubble_sort(&mut v);
//!             hint!("  sorted: {:?}", v);
//!
//!             let is_sorted = v.windows(2).all(|w| w[0] <= w[1]);
//!             assert!(is_sorted);
//!         })
//!     }
//! }
//! ```
//!
//! Running `cargo test` produces the following output:
//! ```text
//! The test failed after 36 passes.
//!
//! # Config
//! - seed: 795359663177100823
//! - start limit: 0
//! - end limit: 100
//! - passes: 1000
//!
//! # Counterexample
//! - run code: "ABIDje/+CYVkmmCVTwKJ2go6VrzZWMjO2Bqc9m3b3h0DAAAAAAAAAA=="
//! - limit: 3
//! - hints:
//!     - unsorted: [255, 252, 131]
//!     -   sorted: [255, 131, 252]
//! - error: assertion failed: is_sorted
//! ```
//!
//! You can rerun the counterexample by setting a environment variable:
//! ```text
//! DICETEST_DEBUG=ABIDje/+CYVkmmCVTwKJ2go6VrzZWMjO2Bqc9m3b3h0DAAAAAAAAAA== cargo test
//! ```
//!
//! # Alternatives
//!
//! * Write down your test data and use a loop.
//! * Use the crate `quickqueck`.
//! * Use the crate `proptest`.

mod macros;

mod util;

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
