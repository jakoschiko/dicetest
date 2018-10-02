//! Provides the random value generator traits `GenOnce` and `Gen` for generating different kind
//! of data.

mod size;
pub use self::size::Size;

mod gen_once;
pub use self::gen_once::GenOnce;

mod gen;
pub use self::gen::Gen;

mod wrapper;
pub use self::wrapper::{Wrapper, GenOnceWrapper, GenWrapper};

pub mod adapters;
