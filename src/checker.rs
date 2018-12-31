//! A unit-test friendly interface for the `runner`. Provides configuration by environment variables
//! and panics on test failure. Logs a human-readable summary of the test runs to stdout.

mod log_condition;
pub use self::log_condition::LogCondition;

mod mode;
use self::mode::Mode;

mod env;

mod functions;
pub use self::functions::{check, check_repeatedly, check_once};
