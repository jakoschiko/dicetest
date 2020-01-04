//! A unit-test friendly interface for the `runner` that panics once the test failed.

mod log_condition;
pub use log_condition::LogCondition;

mod mode;
use mode::Mode;

mod env;

mod functions;
pub use functions::{check, check_once, check_repeatedly};
