//! Contains preludes for different use cases.

pub mod cogens {
    //! Contains the most useful import for writing co-generators.

    pub use crate::cogen::Cogen;
    pub use crate::cogens;
}

pub mod gens {
    //! Contains the most useful import for writing generators.

    pub use crate::cogen::Cogen;
    pub use crate::cogens;
    pub use crate::gen::{Dice, Gen, GenOnce, Limit, Prng};
    pub use crate::gens;
}

pub mod asserts {
    //! Contains the most useful import for writing assertions.

    pub use crate::cogen::Cogen;
    pub use crate::cogens;
    pub use crate::gen::{Dice, Gen, GenOnce};
    pub use crate::gens;
    pub use crate::hints;
    pub use crate::stats;
    pub use crate::{hint, hint_format, stat, stat_format};
}

pub mod tests {
    //! Contains the most useful imports for writing tests.

    pub use crate::cogen::Cogen;
    pub use crate::cogens;
    pub use crate::gen::{Dice, Gen, GenOnce};
    pub use crate::gens;
    pub use crate::hints;
    pub use crate::runner::Config;
    pub use crate::stats;
    pub use crate::{dicetest, hint, hint_format, stat, stat_format};
}
