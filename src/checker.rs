//! A unit-test friendly interface for the `runner` that panics once the test failed.

mod mode;
use mode::Mode;

mod env;

mod functions;
pub use functions::{check, check_once, check_repeatedly};
