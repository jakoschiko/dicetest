//! Provides a random value generator trait for generating different kind of data.

mod params;
pub use self::params::Params;

mod gen_once;
pub use self::gen_once::GenOnce;

mod gen;
pub use self::gen::Gen;

mod wrapper;
pub use self::wrapper::{Wrapper, GenOnceWrapper, GenWrapper};

pub mod adapters;
