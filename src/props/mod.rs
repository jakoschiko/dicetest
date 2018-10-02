//! The standard collection of `Prop` implementations.

mod from_fn_once;
pub use self::from_fn_once::*;

mod forall;
pub use self::forall::*;

mod eq;
pub use self::eq::*;

mod assert;
pub use self::assert::*;

mod inverse;
pub use self::inverse::*;
