//! The runner tries to falsify the assertions of a test by running it repeatedly with different
//! seeds and aborts once a counterexample has been found.

mod limit_series;
use limit_series::LimitSeries;

mod config;
pub use config::Config;

mod run;
pub use run::Run;

mod error;
pub use error::Error;

mod sample;
pub use sample::Sample;

mod summary;
pub use summary::{Counterexample, Summary};

mod functions;
pub use functions::{run_once, run_repeatedly};
