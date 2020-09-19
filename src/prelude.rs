//! Contains preludes for different use cases.

/// Contains the most useful import for writing `Codie`s.
pub mod codice {
    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::prand::Seed;
}

/// Contains the most useful import for writing `DieOnce`s and `Die`s.
pub mod dice {
    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate, Limit};
    pub use crate::prand::Prng;
}

/// Contains the most useful import for writing assertions that are using `DieOnce`s and `Die`s.
pub mod asserts {
    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate};
    pub use crate::hints;
    pub use crate::stats;
    pub use crate::{hint, hint_debug, stat, stat_debug};
}

/// Contains the most useful imports for writing tests that are using `DieOnce`s and `Die`s.
pub mod tests {
    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate};
    pub use crate::hints;
    pub use crate::stats;
    pub use crate::Dicetest;
    pub use crate::{hint, hint_debug, stat, stat_debug};
}
