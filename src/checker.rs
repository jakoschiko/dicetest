//! A unit-test friendly interface for the `runner`. Provides configuration by environment variables
//! and panics on test failure with a human-readable summary of the test runs.

mod panic;
pub use self::panic::Panic;

mod mode;
use self::mode::Mode;

mod env;

mod functions;
pub use self::functions::{check, check_repeatedly, check_once};
