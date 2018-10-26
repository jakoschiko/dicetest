//! The standard collection of `Prop` implementations.

mod from_fn;
pub use self::from_fn::*;

mod forall;
pub use self::forall::*;

mod all;
pub use self::all::*;

mod compare;
pub use self::compare::*;

mod assert;
pub use self::assert::*;

mod inverse;
pub use self::inverse::*;
