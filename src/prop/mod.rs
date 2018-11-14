//! Provides the property trait `Prop` for defining test expectations.

mod eval;
pub use self::eval::Eval;

mod sample;
pub use self::sample::Sample;

mod prop;
pub use self::prop::Prop;

pub mod adapters;

mod show;
pub use self::show::{Show, DebugShow};

mod arg;
pub use self::arg::{Arg, IntoArg, GenArgExt};
