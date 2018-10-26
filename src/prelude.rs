//! Contains preludes for different use cases.

pub mod gens {
    //! Contains the most useful import for defining generators.

    pub use ::rng::Rng;
    pub use ::gen::{Limit, GenOnce, Gen};
    pub use ::gens;
}

pub mod props {
    //! Contains the most useful import for defining properties.

    pub use ::rng::Rng;
    pub use ::gen::{Limit, GenOnce, Gen};
    pub use ::gens;
    pub use ::prop::{LazyString, Log, Eval, Prop, GenArgExt};
    pub use ::props;
}

pub mod tests {
    //! Contains the most useful imports for using properties in unit tests.

    pub use ::gen::{GenOnce, Gen};
    pub use ::gens;
    pub use ::prop::{Prop, GenArgExt};
    pub use ::props;
    pub use ::checker::{
        assert_prop, assert_prop_with_seed, assert_prop_with_params,
        debug_prop, debug_prop_with_params,
    };
}