//! The checker evaluates a property several times to approximate its solution.
//!
//! There are three kind of checker functions:
//!
//! # `check_prop*`
//! These functions check a property and return a `Result`. The `Result` is a summary of all
//! evaluations.
//!
//! # `assert_prop*`
//! These functions are similiar to `check_prop*`, but does not return a `Result`. Instead they
//! panic if the property was falsified or the evaluation failed. They provide a convenient way
//! for checking a property inside a unit test.
//!
//! # `debug_prop*`
//! Use this functions to debug a property. The most important use case is rerunning a property's
//! counterproof.

mod eval_params;
pub use self::eval_params::EvalParams;


mod eval_series_params;
use self::eval_series_params::EvalSeriesParams;

mod eval_series_status;
pub use self::eval_series_status::EvalSeriesStatus;

mod eval_series_result;
pub use self::eval_series_result::EvalSeriesResult;

mod size_series;
use self::size_series::SizeSeries;

mod eval_series;


mod params;
pub use self::params::Params;

mod thread_err;
pub use self::thread_err::ThreadErr;

mod status;
pub use self::status::Status;

mod result;
pub use self::result::Result;

mod portions;
use self::portions::Portions;

mod check;
pub use self::check::*;

mod assert;
pub use self::assert::*;

mod debug;
pub use self::debug::*;
