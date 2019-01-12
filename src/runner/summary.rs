use crate::hints::Hints;
use crate::prand::Seed;
use crate::runner::{Config, Error, Run};
use crate::stats::Stats;

/// Contains details about a failed test run.
#[derive(Debug)]
pub struct Counterexample {
    /// The counterexample can be rerun using this parameters.
    pub run: Run,
    /// The hints collected during the counterexample run.
    ///
    /// If hints are enabled, the runner tries to rerun the counterexample to collect hints.
    /// Rerunning the counterexample can fail if the test is not deterministic.
    pub hints: Option<Hints>,
    /// The error occurred during the counterexample run.
    pub error: Error,
}

/// The result of repeated test runs.
#[derive(Debug)]
pub struct Summary {
    /// The configuration used to create this result.
    pub config: Config,
    /// Initial seed used by the runner. It was either passed via `Config` or randomly generated.
    pub seed: Seed,
    /// The number of test runs that did not fail.
    pub passes: u64,
    /// The stats collected during all test runs. It's defined if and only if stats are enabled.
    pub stats: Option<Stats>,
    /// If defined it contains the failed test run. Otherwise all test runs were successful.
    pub counterexample: Option<Counterexample>,
}
