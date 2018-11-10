//! Contains preludes for different use cases.

pub mod gens {
    //! Contains the most useful import for defining generators.

    pub use crate::rng::Rng;
    pub use crate::gen::{Limit, GenOnce, Gen};
    pub use crate::gens;
}

pub mod props {
    //! Contains the most useful import for defining properties.

    pub use crate::rng::Rng;
    pub use crate::gen::{Limit, GenOnce, Gen};
    pub use crate::gens;
    pub use crate::prop::{LazyString, Log, Eval, Prop, GenArgExt};
    pub use crate::props;
}

pub mod tests {
    //! Contains the most useful imports for using properties in unit tests.

    pub use crate::gen::{GenOnce, Gen};
    pub use crate::gens;
    pub use crate::prop::{Prop, GenArgExt};
    pub use crate::props;
    pub use crate::asserts::assert_prop;
}