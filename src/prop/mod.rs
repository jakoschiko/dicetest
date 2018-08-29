//! Provides a property trait for defining test expectations.

mod label;
pub use self::label::{Label, IntoLabel};

mod params;
pub use self::params::Params;

mod status;
pub use self::status::Status;

mod result;
pub use self::result::Result;

mod prop;
pub use self::prop::Prop;
