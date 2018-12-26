//! Contains preludes for different use cases.

pub mod gens {
    //! Contains the most useful import for writing generators.

    pub use crate::gen::{Prng, Limit, Dice, GenOnce, Gen};
    pub use crate::gens;
}

pub mod asserts {
    //! Contains the most useful import for writing assertions.

    pub use crate::{hint, hint_dbg, stat, stat_dbg};
    pub use crate::hints;
    pub use crate::stats;
    pub use crate::gen::{Dice, GenOnce, Gen};
    pub use crate::gens;
}

pub mod tests {
    //! Contains the most useful imports for writing tests.

    pub use crate::{hint, hint_dbg, stat, stat_dbg, dicetest};
    pub use crate::hints;
    pub use crate::stats;
    pub use crate::gen::{Dice, GenOnce, Gen};
    pub use crate::gens;
    pub use crate::runner::Config;
}