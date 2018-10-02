//! The standard collection of `Prop` implementations.

mod from_fn_once;
pub use self::from_fn_once::*;

mod forall;
pub use self::forall::*;

mod eq;
pub use self::eq::*;
