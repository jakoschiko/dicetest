#![allow(clippy::needless_doctest_main)]
//! Contains preludes for different use cases.
//!
//! # Examples
//!
//! ```
//! #[derive(Debug, Clone)]
//! pub struct Point<T>(T, T);
//!
//! impl<T: std::ops::Add<Output=T>> std::ops::Add for Point<T> {
//!     type Output = Self;
//!
//!     fn add(self, rhs: Self) -> Self::Output {
//!         Self(self.0 + rhs.0, self.1 + rhs.1)
//!     }
//! }
//!
//! impl<T: std::ops::Mul<Output=T>> std::ops::Mul for Point<T> {
//!     type Output = Self;
//!
//!     fn mul(self, rhs: Self) -> Self::Output {
//!         Self(self.0 * rhs.0, self.1 * rhs.1)
//!     }
//! }
//!
//! pub mod dice {
//!     use dicetest::prelude::dice::*;
//!
//!     pub fn point<T>(elem_die: impl Die<T>) -> impl Die<crate::Point<T>> {
//!         dice::array_2(elem_die).map(|[x, y]| crate::Point(x, y))
//!     }
//! }
//!
//! pub mod asserts {
//!     use dicetest::prelude::asserts::*;
//!
//!     pub fn commutative<T: PartialEq + Clone + std::fmt::Debug>(
//!         fate: &mut Fate,
//!         elem_die: impl Die<T>,
//!         binop: impl Fn(T, T) -> T,
//!     ) {
//!         let [a, b] = dice::array_2(elem_die).roll(fate);
//!         hint_debug!(a);
//!         hint_debug!(b);
//!         let ab = binop(a.clone(), b.clone());
//!         let ba = binop(b, a);
//!         assert_eq!(ab, ba);
//!     }
//! }
//!
//! mod tests {
//!     use dicetest::prelude::tests::*;
//!
//!     fn point_die() -> impl Die<crate::Point<u8>> {
//!         let elem_die = dice::u8(..);
//!         crate::dice::point(elem_die)
//!     }
//!
//!     #[test]
//!     fn add_is_commutative() {
//!         dicetest!(|fate| {
//!             crate::asserts::commutative(fate, point_die(), Point::add);
//!         })
//!     }
//!
//!     #[test]
//!     fn mul_is_commutative() {
//!         dicetest!(|fate| {
//!             crate::asserts::commutative(fate, point_die(), Point::mul);
//!         })
//!     }
//! }
//!
//! # fn main() {}
//! ```

/// Contains the most useful import for writing `Codie`s.
pub mod codice {
    pub use crate::codice;
    pub use crate::codie::Codie;
}

/// Contains the most useful import for writing `DieOnce`s and `Die`s.
pub mod dice {
    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate, Limit};
    pub use crate::prand::Prng;
}

/// Contains the most useful import for writing assertions that are using `DieOnce`s and `Die`s.
pub mod asserts {
    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate};
    pub use crate::hints;
    pub use crate::stats;
    pub use crate::{hint, hint_debug, stat, stat_debug};
}

/// Contains the most useful imports for writing tests that are using `DieOnce`s and `Die`s.
pub mod tests {
    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate};
    pub use crate::hints;
    pub use crate::runner::Config;
    pub use crate::stats;
    pub use crate::{dicetest, hint, hint_debug, stat, stat_debug};
}
