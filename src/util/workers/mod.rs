//! Helper for running computations on multiple threads.

mod result;
pub use self::result::{SingleResult, JoinedResult};

mod without_timeout;

mod with_timeout;

mod run;
pub use self::run::run;
