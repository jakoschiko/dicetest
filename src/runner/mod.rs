//! The runner tries to falsify the assertions of a test by running it repeatedly with different
//! seeds and aborts once a counterexample has been found. It returns all necessary information to
//! reproduce the result.

mod limit_series;
use self::limit_series::LimitSeries;

mod config;
pub use self::config::Config;

mod run;
pub use self::run::Run;

mod error;
pub use self::error::Error;

mod sample;
pub use self::sample::Sample;

mod summary;
pub use self::summary::{Counterexample, Summary};

mod functions;
pub use self::functions::{run_repeatedly, run_once};
