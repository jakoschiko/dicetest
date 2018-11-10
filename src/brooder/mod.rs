//! The brooder checks a property by evaluating it several times to approximate its solution.

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

mod limit_series;
use self::limit_series::LimitSeries;

mod portions;
use self::portions::Portions;

mod brooder;
pub use self::brooder::*;
