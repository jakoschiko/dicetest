//! The checker evaluates a property several times to approximate its solution.
//!
//! There are three kind of checker functions:
//!
//! # `check_prop*`
//! These functions check a property and return a `Report`. The `Report` is a summary of all
//! evaluations.
//!
//! # `assert_prop*`
//! These functions are similiar to `check_prop*`, but does not return a `Report`. Instead they
//! panic if the property was falsified or the evaluation failed. They provide a convenient way
//! for checking a property inside a unit test.
//!
//! # `debug_prop*`
//! Use this functions to debug a property. The most important use case is rerunning a property's
//! counterproof.

mod eval_params;
pub use self::eval_params::EvalParams;

mod eval_summary;
pub use self::eval_summary::EvalSummary;

mod eval_series;
pub use self::eval_series::EvalSeries;


mod params;
pub use self::params::Params;

mod thread_err;
pub use self::thread_err::ThreadErr;

mod status;
pub use self::status::Status;

mod report;
pub use self::report::Report;


mod size_series;
use self::size_series::SizeSeries;

mod eval_runner;


mod portions;
use self::portions::Portions;

mod check;
pub use self::check::*;

mod assert;
pub use self::assert::*;

mod debug;
pub use self::debug::*;
