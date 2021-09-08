//! Framework for writing tests with randomly generated test data.
//!
//! # Status of this crate
//!
//! The author does not consider this crate as stable yet. Changes will be documented in the
//! [changelog](https://github.com/jakoschiko/dicetest/blob/master/CHANGELOG.md).
//!
//! # Example
//!
//! Here's an example of an incorrect sort function tested with dicetest:
//!
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
//!     use dicetest::prelude::*;
//!
//!     #[test]
//!     fn result_of_bubble_sort_is_sorted() {
//!         Dicetest::repeatedly().run(|mut fate| {
//!             let mut v = fate.roll(dice::vec(dice::u8(..), ..));
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
//!
//! ```text
//! The test failed after 31 passes.
//!
//! # Config
//! - seed: 3713861809241954222
//! - start limit: 0
//! - end limit: 100
//! - passes: 200
//!
//! # Counterexample
//! - run code: "/yiA1sab3S4UnCf4ozyMpxMxzg1NtFybCuYLHy0/oscDAAAAAAAAAA=="
//! - limit: 3
//! - hints:
//!     - unsorted: [201, 209, 2]
//!     -   sorted: [201, 2, 209]
//! - error: assertion failed: is_sorted
//! ```
//!
//! You can rerun the counterexample by setting an environment variable:
//!
//! ```text
//! DICETEST_DEBUG=/yiA1sab3S4UnCf4ozyMpxMxzg1NtFybCuYLHy0/oscDAAAAAAAAAA== cargo test
//! ```
//!
//! Or you can modify the test:
//!
//! ```
//! # use dicetest::Dicetest;
//! Dicetest::debug("/yiA1sab3S4UnCf4ozyMpxMxzg1NtFybCuYLHy0/oscDAAAAAAAAAA==").run(|mut fate| {
//!     // ...
//! })
//! ```
//!
//! # Features
//!
//! These features are **available**:
//!
//! - Generators for many libstd types (`u8`, `String`, `Vec`, etc.).
//! - Generators for functions (`FnMut`, `FnOnce`, `Fn`).
//! - Generator combinators (`map`, `flat_map`, `zip`, etc.).
//! - Integration of `rand::distributions::Distribution`.
//! - Integration of `quickcheck::Arbitrary` (without shrinking).
//! - Configurable test runner.
//! - Utilities for debugging tests (`hints` and `stats`).
//!
//! These features are **missing**:
//!
//! - Shrinking of counterexamples.
//! - Custom pseudorandom number generators.
//! - Own type class for arbitrary types.
//!
//! # Alternatives
//!
//! * Write down your test data and use a loop.
//! * Use the crate [quickcheck].
//! * Use the crate [proptest].
//!
//! [quickcheck]: https://crates.io/crates/quickcheck
//! [proptest]: https://crates.io/crates/proptest
//!
//! # Guide
//!
//! This section will guide you through the most important concepts and features of dicetest.
//!
//! ## Pseudorandomness
//!
//! The type `Seed` allows to determine the [pseudorandomness]. You can either use a fixed
//! `Seed` or a random `Seed`:
//!
//! ```
//! use dicetest::Seed;
//!
//! println!("{:?}", Seed(42));
//! // Output: Seed(42)
//!
//! println!("{:?}", Seed::random());
//! // Output: Seed(8019292413750407764)
//! ```
//!
//! The `Seed` can be used to initialize the [pseudorandom number generator] `Prng`. For each
//! `Seed` the `Prng` provides a different infinite pseudorandom sequence of `u64`s
//!
//! ```
//! use dicetest::{Prng, Seed};
//!
//! fn print_random_values(mut prng: Prng) {
//!     for _ in 0..3 {
//!         print!("{:?}, ", prng.next_number());
//!     }
//!     println!("...");
//! }
//!
//! print_random_values(Prng::from_seed(Seed(42)));
//! // Output: 16628028624323922065, 3476588890713931039, 59688652182557721, ...
//! print_random_values(Prng::from_seed(Seed(42)));
//! // Output: 16628028624323922065, 3476588890713931039, 59688652182557721, ...
//! print_random_values(Prng::from_seed(Seed::random()));
//! // Output: 4221507577048064061, 15374206214556255352, 4977687432463843847, ...
//! print_random_values(Prng::from_seed(Seed::random()));
//! // Output: 11086225885938422405, 9312304973013875005, 1036200222843160301, ...
//! ```
//!
//! [pseudorandomness]: https://en.wikipedia.org/wiki/Pseudorandomness
//! [pseudorandom number generator]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator
//!
//! ## Dice
//!
//! Although `Prng` can only generate pseudorandom `u64`s, the `u64`s can be used for constructing
//! more complex values. The traits `DieOnce` and `Die` represents `Prng`-based generators for
//! values of any type.
//!
//! An implementor of `DieOnce` is a generator that can be used a single time
//! (similar to [`FnOnce`]).
//! ```
//! use dicetest::prelude::*;
//!
//! let xx = "xx".to_string();
//! let yy = "yy".to_string();
//!
//! // This generator implements `DieOnce`.
//! // It chooses one of the `String`s without cloning them.
//! let xx_or_yy_die = dice::one_of_once().two(xx, yy);
//! ```
//!
//! An implementor of `Die` is a generator that can be used infinite times (similar to [`Fn`]).
//! ```
//! use dicetest::prelude::*;
//!
//! let xx = "xx".to_string();
//! let yy = "yy".to_string();
//!
//! // This generator implements `Die`.
//! // It chooses one of the `String`s by cloning them.
//! let xx_or_yy_die = dice::one_of().two(xx, yy);
//!
//! // This generator uses `xx_or_yy_die` to generate three `String`s at once.
//! let three_xx_or_yy_die = dice::array::<_, _, 3>(xx_or_yy_die);
//! ```
//!
//! Generators can be easily implemented and composed:
//! ```
//! use dicetest::prelude::*;
//!
//! // A classic die that generates a number between 1 and 6 with uniform distribution.
//! let classic_die = dice::one_of().six::<u8>(1, 2, 3, 4, 5, 6);
//!
//! // A loaded die that generates the number 6 more frequently.
//! let loaded_die =
//!     dice::weighted_one_of().six::<u8>((1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (2, 6));
//!
//! // This die generates the result of the function.
//! let die_from_fn = dice::from_fn(|_| 42);
//!
//! // This die generates always the same `String` by cloning the original one.
//! let foo_die = dice::just("foo".to_string());
//!
//! // This die generates an arbitrary byte.
//! let byte_die = dice::u8(..);
//!
//! // This die generates a non-zero byte.
//! let non_zero_byte_die = dice::u8(1..);
//!
//! // This die generates a `Vec` that contains an arbitrary number of arbitrary bytes.
//! let bytes_die = dice::vec(dice::u8(..), ..);
//!
//! // This die generates a `Vec` that contains up to 10 arbitrary bytes.
//! let up_to_ten_bytes_die = dice::vec(dice::u8(..), ..=10);
//!
//! // This die generates an arbitrary wrapped byte.
//! struct WrappedByte(u8);
//! let wrapped_byte_die = dice::u8(..).map(WrappedByte);
//!
//! // This die generates a permutation of `(0..=n)` for an arbitrary `n`.
//! let permutation_die = dice::length(0..).flat_map(|n| {
//!     let vec = (0..=n).collect::<Vec<_>>();
//!     dice::shuffled_vec(vec)
//! });
//! ```
//!
//! The struct `Fate` is necessary for using `DieOnce` or `Die`. It contains two parameters:
//!
//! * `Prng`: Provides the pseudorandom `u64`s that the implementor of `DieOnce` or `Die` can use
//! for constructing more complex values. The implementor should only use this as its source of
//! randomness.
//! * `Limit`: The upper limit for the length of dynamic data structures generated by the
//! implementor of `DieOnce` or `Die`. The implementor is allowed to freely interpret or even
//! ignore this value.
//!
//! ```
//! use dicetest::prelude::*;
//! use dicetest::{Limit, Prng};
//!
//! // Provides the randomness for the generator and will be mutated when used.
//! let mut prng = Prng::from_seed(0x5EED.into());
//! // Limits the length of dynamic data structures. The generator has only read access.
//! let limit = Limit(5);
//!
//! // Contains all parameters necessary for using `DieOnce` or `Die`.
//! let mut fate = Fate::new(&mut prng, limit);
//!
//! // Generator for a `Vec` with an arbitrary length.
//! let vec_die = dice::vec(dice::u8(..), ..);
//!
//! // Generates a `Vec`. Although `vec_die` can generate a `Vec` with an arbitrary length,
//! // the length of the actual `Vec` is limited by `limit`.
//! let vec = fate.roll(vec_die);
//! assert!(vec.len() <= 5);
//!
//! println!("{:?}", vec);
//! // Output: [252, 231, 153, 0]
//! ```
//!
//! [`FnOnce`]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
//! [`Fn`]: https://doc.rust-lang.org/std/ops/trait.Fn.html
//!
//! ## Tests
//!
//! If you want to write a test with randomly generated test data you can use the test
//! builder`Dicetest`:
//! * It can be configured via source code or environment variables.
//! * It runs your test repeatedly with different seeds.
//! * It logs useful information that helps you to debug your test.
//!
//! ```
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn test_foo() {
//!     // Runs your test with default configuration.
//!     Dicetest::repeatedly().run(|fate| {
//!         // Write your test here.
//!     });
//! }
//!
//! #[test]
//! fn test_bar() {
//!     // Runs your test with custom configuration.
//!     Dicetest::repeatedly().passes(10000).run(|fate| {
//!         // Write your test here.
//!     });
//! }
//! ```
//!
//! The closure contains your test. With the passed `fate` you can generate test data and make
//! assertions. If the closure panics, `Dicetest` catches the panic, logs the test result to
//! stdout and resumes the panic.
//!
//! ## Hints
//!
//! Hints can be used to analyze a single test run. In most cases you want to analyze the
//! counterexample. Use it to reveal what test data were generated or which branches were taken:
//!
//! ```
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn test_foo() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let x = fate.roll(dice::u8(1..=5));
//!         hint_debug!(x);
//!
//!         let y = fate.roll(dice::u8(1..=3));
//!         if y != x {
//!             hint!("took branch if with y = {}", y);
//!
//!             assert_eq!(3, y);
//!         } else {
//!             hint!("took branch else");
//!         }
//!     })
//! }
//! ```
//!
//! Running the test produces the following output:
//!
//! ```text
//! The test failed after 0 passes.
//!
//! # Config
//! - seed: 10929669535587280453
//! - start limit: 0
//! - end limit: 100
//! - passes: 200
//!
//! # Counterexample
//! - run code: "JfXG0LRXjKUMu+YmdrF38/GstRdeLAeMRTKskCQcgNoAAAAAAAAAAA=="
//! - limit: 0
//! - hints:
//!     - x = 5
//!     - took branch if with y = 1
//! - error: assertion failed: `(left == right)`
//!   left: `3`,
//!  right: `1`
//! ```
//!
//! ## Stats
//!
//! Stats can be used to analyze multiple test runs. Use it to reveal the distribution of
//! generated test data or the probability of branches:
//!
//! ```
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn test_foo() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let x = fate.roll(dice::u8(1..=5));
//!         stat_debug!(x);
//!
//!         let y = fate.roll(dice::u8(1..=3));
//!         if y != x {
//!             stat!("branch", "if with y = {}", y)
//!         } else {
//!             stat!("branch", "else");
//!         }
//!     })
//! }
//! ```
//!
//! Running the test with the environment variable `DICETEST_STATS_ENABLED=true` produces
//! the following output:
//!
//! ```text
//! The test withstood 200 passes.
//!
//! # Config
//! - seed: 5043079553183914912
//! - start limit: 0
//! - end limit: 100
//! - passes: 200
//!
//! # Stats
//! - branch:
//!     - 29.50% (59): if with y = 1
//!     - 27.50% (55): if with y = 3
//!     - 22.50% (45): if with y = 2
//!     - 20.50% (41): else
//! - x:
//!     - 31.50% (63): 1
//!     - 22.00% (44): 5
//!     - 17.00% (34): 2
//!     - 15.50% (31): 4
//!     - 14.00% (28): 3
//! ```
//!
//! ## Environment variables
//!
//! You can use environment variables to configure your tests without changing the source code.
//! See the documentation of `Dicetest` for a full list of supported environment variables.
//! Here are some examples:
//!
//! * You want to debug the counterexample of `mytest` with its run code (copied from the test result):
//! ```text
//! DICETEST_DEBUG=ABIDje/+CYVkmmCVTwKJ2go6VrzZWMjO2Bqc9m3b3h0DAAAAAAAAAA== cargo test mytest
//! ```
//! * You want to reproduce the result of `mytest` with its seed (copied from the test result):
//! ```text
//! DICETEST_SEED=795359663177100823 cargo test mytest
//! ```
//! * You want to see the stats of `mytest`:
//! ```text
//! DICETEST_STATS_ENABLED=true cargo test -- --show-output mytest
//! ```
//! * You want to run `mytest` with more passes and bigger test data:
//! ```text
//! DICETEST_PASSES_MULTIPLIER=10 DICETEST_LIMIT_MULTIPLIER=2 cargo test mytest
//! ```
//! * You want to run `mytest` with a single test run and see the test result:
//! ```text
//! DICETEST_MODE=once cargo test -- --show-output mytest
//! ```
//!
//! ## Feature flags
//!
//! There are several feature flags for disabling runtime overhead or enabling additional
//! features at compile time.
//!
//! ### `hints` (enabled by default)
//! Enables or disables the hints feature at compile time. If disabled,
//! all hints operations are no-ops.
//!
//! ### `stats` (enabled by default)
//! Enables or disables the stats feature at compile time. If disabled,
//! all stats operations are no-ops.
//!
//! ### `rand_core` (disabled by default)
//! If enabled, `dicetest::Prng` and `dicetest::Fate` implements the `rand_core::RngCore`
//! trait.
//!
//! ### `rand_full` (disabled by default, alias for `rand_core,rand`)
//! If enabled, `Fate::roll_distribution` and `dice::from_distribution` are available.
//! This allows to generate values and create `Die`s from implementations
//! of `rand::distributions::Distribution`.
//!
//! ```
//! # #[cfg(feature = "rand_full")] {
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
//!
//! ### `quickcheck_full` (disabled by default, alias for `rand_core,quickcheck`)
//! If enabled, `Fate` implements the `quickcheck::Gen` trait and `Fate::roll_arbitrary` and
//! `dice::arbitrary` are available. This allows to generate values and create `Die`s for types
//! that implements `quickcheck::Arbitrary`.
//!
//! ```
//! # #[cfg(feature = "quickcheck_full")] {
//! use dicetest::prelude::*;
//! use dicetest::{Limit, Prng};
//!
//! let mut prng = Prng::from_seed(0x5EED.into());
//! let limit = Limit(5);
//! let mut fate = Fate::new(&mut prng, limit);
//!
//! // Generate a value of a type that implements `quickcheck::Arbitrary`
//! let byte: u8 = fate.roll_arbitrary();
//! println!("{:?}", byte);
//! // Output: 0
//!
//! // Create a `Die` for a type that implements `quickcheck::Arbitrary`
//! let byte_die = dice::arbitrary();
//! let bytes_die = dice::vec(byte_die, 1..);
//! let bytes: Vec<u8> = fate.roll(bytes_die);
//! println!("{:?}", bytes);
//! // Output: [1, 4, 4, 2]
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
