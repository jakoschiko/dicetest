use crate::hints::Hints;
use crate::runner::{Config, Error, Run};
use crate::seed::Seed;
use crate::stats::Stats;

/// A counterexample is a failed test run. This struct contains details about such a counterexample.
pub struct Counterexample {
    /// The counterexample can be rerun using this parameters.
    pub run: Run,
    /// If hints are enabled, the runner tries to rerun the counterexample and collect these hints.
    /// Rerunning the counterexample can fail if the test is not deterministic.
    pub hints: Option<Hints>,
    /// The error occurred during the counterexample run.
    pub error: Error,
}

// The result of repeated test runs.
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
