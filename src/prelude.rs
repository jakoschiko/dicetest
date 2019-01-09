//! Contains preludes for different use cases.

pub mod codice {
    //! Contains the most useful import for writing `Codie`s.

    pub use crate::codice;
    pub use crate::codie::Codie;
}

pub mod dice {
    //! Contains the most useful import for writing `DieOnce`s and `Die`s.

    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate, Limit, Prng};
}

pub mod asserts {
    //! Contains the most useful import for writing assertions that are using `DieOnce`s and `Die`s.

    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate};
    pub use crate::hints;
    pub use crate::stats;
    pub use crate::{hint, hint_format, stat, stat_format};
}

pub mod tests {
    //! Contains the most useful imports for writing tests.

    pub use crate::codice;
    pub use crate::codie::Codie;
    pub use crate::dice;
    pub use crate::die::{Die, DieOnce, Fate};
    pub use crate::hints;
    pub use crate::runner::Config;
    pub use crate::stats;
    pub use crate::{dicetest, hint, hint_format, stat, stat_format};
}
