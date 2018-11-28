//! A simple counter for keys and values. Counts the occurrences of a value per key.
//!
//! Can be completely disabled with the feature `disabled_counter`.

mod stat;
pub use self::stat::{Occurence, Stat, Stats};

mod counter;
pub use self::counter::*;
