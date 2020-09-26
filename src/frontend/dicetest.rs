use std::panic::{self, RefUnwindSafe, UnwindSafe};

use crate::frontend::env::{self, EnvValue};
use crate::frontend::formatter::*;
use crate::frontend::{Mode, RunCode};
use crate::{runner, Fate, Limit, Prng, Seed};

#[derive(Debug, Clone)]
struct Params {
    seed: Option<Seed>,
    once_limit: Limit,
    start_limit: Limit,
    end_limit: Limit,
    limit_multiplier: Option<f64>,
    passes: u64,
    passes_multiplier: Option<f64>,
    env_enabled: bool,
    hints_enabled: bool,
    stats_enabled: bool,
    formatting: Formatting,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            seed: None,
            once_limit: Limit::default(),
            start_limit: 0.into(),
            end_limit: Limit::default(),
            limit_multiplier: None,
            passes: 200,
            passes_multiplier: None,
            env_enabled: true,
            hints_enabled: true,
            stats_enabled: false,
            formatting: Formatting::default(),
        }
    }
}

/// Front end for configuring and running a test with pseudorandomly generated test data.
///
/// You can set the test parameters via source code or environment variables.
///
/// # Examples
///
/// Runs the test repeatedly with default config:
/// ```
/// use dicetest::Dicetest;
///
/// Dicetest::repeatedly().run(|_fate| {
///     // Put your test here.
/// });
/// ```
///
/// Runs the test repeatedly with custom config:
/// ```
/// use dicetest::Dicetest;
///
/// Dicetest::repeatedly().passes(42).run(|_fate| {
///     // Put your test here.
/// });
/// ```
///
/// Runs the test once with the given run code (printed when a test had failed):
/// ```
/// use dicetest::Dicetest;
///
/// Dicetest::debug("ABIDje/+CYVkmmCVTwKJ2go6VrzZWMjO2Bqc9m3b3h0DAAAAAAAAAA==").run(|_fate| {
///     // Put your test here.
/// });
/// ```
#[derive(Debug, Clone)]
#[must_use]
pub struct Dicetest {
    mode: Mode,
    params: Params,
}

impl Dicetest {
    /// Configuration for running the test in debug mode.
    ///
    /// In this mode the test will be run once. The parameters for pseudorandom value generation
    /// will be extracted from the given run code.
    ///
    /// # Panics
    ///
    /// Panics if the run code is invalid.
    ///
    /// # Environment
    ///
    /// You can set this mode with `DICETEST_DEBUG=<run code>`. The value `<run code>` must be
    /// a valid run code.
    pub fn debug(run_code: &str) -> Self {
        let run_code = RunCode::from_base64(&run_code).unwrap();
        Dicetest {
            mode: Mode::Debug(run_code),
            params: Params::default(),
        }
    }

    /// Configuration for running the test in run-once mode.
    ///
    /// In this mode the test will be run once. In contrast to debug mode, the `Seed` and `Limit`
    /// can be set separately via `Dicetest::seed` and `Dicetest::limit`.
    ///
    /// # Environment
    ///
    /// You can set this mode via `DICETEST_MODE=once`.
    pub fn once() -> Self {
        Dicetest {
            mode: Mode::Once,
            params: Params::default(),
        }
    }

    /// Configuration for running the test in run-repeatedly mode.
    ///
    /// In this mode the test will be run repeatedly until the configured number of passes has been
    /// reached or the test has panicked. If the test has panicked, a counterexample has been
    /// found. The counterexample can be debugged using the debug mode, see `Dicetest::debug`.
    ///
    /// # Environment
    ///
    /// You can set this mode via `DICETEST_MODE=repeatedly`.
    pub fn repeatedly() -> Self {
        Dicetest {
            mode: Mode::Repeatedly,
            params: Params::default(),
        }
    }

    /// Sets the initial `Seed` for the pseudorandom value generation.
    ///
    /// It's only used in run-once and run-repeatedly mode and is `None` by default.
    /// In case of `None` the `Seed` will be randomly generated.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_SEED=<seed>`. The value `<seed>` must be either
    /// `none` or `<u64>`.
    pub fn seed(mut self, seed: Option<Seed>) -> Self {
        self.params.seed = seed;
        self
    }

    /// Sets the upper limit for the size of generated dynamic data structures.
    ///
    /// It's only used in run-once mode and is `100` by default.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_ONCE_LIMIT=<u64>`.
    pub fn once_limit(mut self, once_limit: Limit) -> Self {
        self.params.once_limit = once_limit;
        self
    }

    /// Sets the initial upper limit for the size of generated dynamic data structures.
    ///
    /// It will be used for the first test run. The following test runs use an interpolated limit,
    /// see `Dicetest::end_limit`. It's only used in run-repeatedly mode and is `0` by default.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_START_LIMIT=<u64>`.
    pub fn start_limit(mut self, start_limit: Limit) -> Self {
        self.params.start_limit = start_limit;
        self
    }

    /// Sets the final upper limit for the size of generated dynamic data structures.
    ///
    /// It will be used for the last test run. The previous test runs use an interpolated limit,
    /// see `Dicetest::start_limit`. It's only used in run-repeatedly mode and is `100` by default.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_END_LIMIT=<u64>`.
    pub fn end_limit(mut self, end_limit: Limit) -> Self {
        self.params.end_limit = end_limit;
        self
    }

    /// Sets the multiplier for the `Limit`.
    ///
    /// It will be applied to the values that can be set via `Dicetest::once_limit`,
    /// `Dicetest::start_limit` and `Dicetest::end_limit` before running the test.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_LIMIT_MULTIPLIER=<factor>`. The value `<factor>`
    /// must be either `none` or `<f64>`.
    pub fn limit_multiplier(mut self, limit_multiplier: Option<f64>) -> Self {
        self.params.limit_multiplier = limit_multiplier;
        self
    }

    /// Sets how many times the test needs to be run without failing.
    ///
    /// It's only used in run-repeatedly mode and is `200` by default.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_PASSES=<u64>`.
    pub fn passes(mut self, passes: u64) -> Self {
        self.params.passes = passes;
        self
    }

    /// Sets the multiplier for the number of passes.
    ///
    /// It will be applied to the value that can be set via `Dicetest::passes` before running
    /// the test.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_PASSES_MULTIPLIER=<factor>`. The value `<factor>`
    /// must be either `none` or `<f64>`.
    pub fn passes_multiplier(mut self, passes_multiplier: Option<f64>) -> Self {
        self.params.passes_multiplier = passes_multiplier;
        self
    }

    /// Sets whether hints are collected during the test run.
    ///
    /// In run-once and debug mode hints are collected during the single test run.
    /// In run-repeatedly mode hints are only collected if a counterexample has been found.
    /// The counterexample will be rerun to collect the hints. Rerunning the counterexample
    /// can fail if the test is not deterministic.
    ///
    /// This parameter is `true` by default. It works only work if the feature `hints` is present.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_HINTS_ENABLED=<bool>`.
    pub fn hints_enabled(mut self, hints_enabled: bool) -> Self {
        self.params.hints_enabled = hints_enabled;
        self
    }

    /// Sets whether stats are collected during the test runs.
    ///
    /// In run-once and debug mode stats are collected during the single test run.
    /// In run-repeatedly mode stats are collected during all test runs (except the rerun of
    /// the counterexample).
    ///
    /// This parameter is `false` by default. It works only work if the feature `stats` is present.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_STATS_ENABLED=<bool>`.
    pub fn stats_enabled(mut self, stats_enabled: bool) -> Self {
        self.params.stats_enabled = stats_enabled;
        self
    }

    /// Sets the maximum numbers of values per key that will be used when formatting the stats.
    ///
    /// If `None` all values will be present in the result. This parameter is `Some(20)` by default.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_STATS_MAX_VALUE_COUNT=<max_value_count>`.
    /// The value `<max_value_count>` must be either `none` or `<usize>`.
    pub fn stats_max_value_count(mut self, stats_max_value_count: Option<usize>) -> Self {
        self.params.formatting.stats_max_value_count = stats_max_value_count;
        self
    }

    /// Sets the number of decimal places for percent values that will be used when formatting the
    /// stats.
    ///
    /// This parameter is `2` by default.
    ///
    /// # Environment
    ///
    /// You can set this parameter via `DICETEST_STATS_PERCENT_PRECISION=<usize>`.
    pub fn stats_percent_precision(mut self, stats_percent_precision: usize) -> Self {
        self.params.formatting.stats_percent_precision = stats_percent_precision;
        self
    }

    /// Sets whether test parameters can be overridden via environment variables.
    ///
    /// If set to true and special environment variables are present, `Dicetest::run`
    /// will parse their values and override the corresponding test parameters
    /// before running the test. This parameter is `true` by default.
    pub fn env_enabled(mut self, env_enabled: bool) -> Self {
        self.params.env_enabled = env_enabled;
        self
    }

    /// Runs the test with the given configuration and prints the result to stdout.
    ///
    /// If special environment variables are present, this function will parse their values and
    /// override the corresponding test parameters before running the test. This can be disabled
    /// via `Dicetest::env_enabled`.
    ///
    /// # Panics
    ///
    /// Panics if parsing a present environment variable has failed or the test has panicked
    /// during a test run.
    pub fn run<T>(self, test: T)
    where
        T: Fn(Fate) + UnwindSafe + RefUnwindSafe,
    {
        let config = if self.params.env_enabled {
            self.override_by_env().unwrap()
        } else {
            self
        };

        let params = config.params;

        match config.mode {
            Mode::Debug(run_code) => {
                let prng = run_code.prng.clone();
                let config = runner::once::Config {
                    limit: run_code.limit,
                    hints_enabled: params.hints_enabled,
                    stats_enabled: params.stats_enabled,
                };
                let report = runner::once::run(prng, &config, test);

                let formatting = &params.formatting;
                println!(
                    "{}",
                    display_run_once_report(&run_code, None, &report, formatting)
                );

                if let Some(err) = report.error.map(|error| error.0) {
                    panic::resume_unwind(err);
                }
            }
            Mode::Once => {
                let seed = params.seed.unwrap_or_else(Seed::random);
                let prng = Prng::from_seed(seed);
                let mut limit = params.once_limit;

                if let Some(limit_multiplier) = params.limit_multiplier {
                    limit = multiply(limit.0, limit_multiplier).into();
                }

                let config = runner::once::Config {
                    limit,
                    hints_enabled: params.hints_enabled,
                    stats_enabled: params.stats_enabled,
                };
                let report = runner::once::run(prng.clone(), &config, test);

                let run_code = RunCode { prng, limit };
                let formatting = &params.formatting;
                println!(
                    "{}",
                    display_run_once_report(&run_code, Some(seed), &report, formatting)
                );

                if let Some(err) = report.error.map(|error| error.0) {
                    panic::resume_unwind(err);
                }
            }
            Mode::Repeatedly => {
                let seed = params.seed.unwrap_or_else(Seed::random);
                let prng = Prng::from_seed(seed);
                let mut start_limit = params.start_limit;
                let mut end_limit = params.end_limit;
                let mut passes = params.passes;

                if let Some(limit_multiplier) = params.limit_multiplier {
                    start_limit = multiply(start_limit.0, limit_multiplier).into();
                    end_limit = multiply(end_limit.0, limit_multiplier).into();
                }
                if let Some(passes_multiplier) = params.passes_multiplier {
                    passes = multiply(passes, passes_multiplier);
                }

                let config = runner::repeatedly::Config {
                    start_limit,
                    end_limit,
                    passes,
                    hints_enabled: params.hints_enabled,
                    stats_enabled: params.stats_enabled,
                };
                let report = runner::repeatedly::run(prng, &config, test);

                let formatting = &params.formatting;
                println!(
                    "{}",
                    display_run_repeatedly_report(seed, &config, &report, formatting)
                );

                if let Some(err) = report.counterexample.map(|c| c.error.0) {
                    panic::resume_unwind(err);
                }
            }
        }
    }

    fn override_by_env(mut self) -> Result<Self, String> {
        // Read values
        if let EnvValue::Present(mode) = env::read_mode()? {
            self.mode = mode;
        }
        if let EnvValue::Present(seed) = env::read_seed()? {
            self.params.seed = seed
        }
        if let EnvValue::Present(once_limit) = env::read_once_limit()? {
            self.params.once_limit = once_limit
        }
        if let EnvValue::Present(start_limit) = env::read_start_limit()? {
            self.params.start_limit = start_limit
        }
        if let EnvValue::Present(end_limit) = env::read_end_limit()? {
            self.params.end_limit = end_limit
        }
        if let EnvValue::Present(limit_multiplier) = env::read_limit_multiplier()? {
            self.params.limit_multiplier = limit_multiplier;
        }
        if let EnvValue::Present(passes) = env::read_passes()? {
            self.params.passes = passes
        }
        if let EnvValue::Present(passes_multiplier) = env::read_passes_multiplier()? {
            self.params.passes_multiplier = passes_multiplier
        }
        if let EnvValue::Present(hints_enabled) = env::read_hints_enabled()? {
            self.params.hints_enabled = hints_enabled
        }
        if let EnvValue::Present(stats_enabled) = env::read_stats_enabled()? {
            self.params.stats_enabled = stats_enabled
        }
        if let EnvValue::Present(stats_max_value_count) = env::read_stats_max_value_count()? {
            self.params.formatting.stats_max_value_count = stats_max_value_count
        }
        if let EnvValue::Present(stats_percent_precision) = env::read_stats_percent_precision()? {
            self.params.formatting.stats_percent_precision = stats_percent_precision
        }

        Ok(self)
    }
}

fn multiply(value: u64, factor: f64) -> u64 {
    (value as f64 * factor) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_debug() {
        let run_code = RunCode {
            prng: Prng::from_seed(42.into()),
            limit: Limit::default(),
        };
        let dicetest = Dicetest::debug(&run_code.to_base64());
        assert_eq!(Mode::Debug(run_code), dicetest.mode);
    }

    #[test]
    fn set_once() {
        let dicetest = Dicetest::once();
        assert_eq!(Mode::Once, dicetest.mode);
    }

    #[test]
    fn set_repeatedly() {
        let dicetest = Dicetest::repeatedly();
        assert_eq!(Mode::Repeatedly, dicetest.mode);
    }

    #[test]
    fn set_seed() {
        let seed = Some(42.into());
        let dicetest = Dicetest::repeatedly().seed(seed);
        assert_eq!(seed, dicetest.params.seed);
    }

    #[test]
    fn set_once_limit() {
        let once_limit = 42.into();
        let dicetest = Dicetest::repeatedly().once_limit(once_limit);
        assert_eq!(once_limit, dicetest.params.once_limit);
    }

    #[test]
    fn set_start_limit() {
        let start_limit = 42.into();
        let dicetest = Dicetest::repeatedly().start_limit(start_limit);
        assert_eq!(start_limit, dicetest.params.start_limit);
    }

    #[test]
    fn set_end_limit() {
        let end_limit = 42.into();
        let dicetest = Dicetest::repeatedly().end_limit(end_limit);
        assert_eq!(end_limit, dicetest.params.end_limit);
    }

    #[test]
    fn set_limit_multiplier() {
        let limit_multiplier = Some(42.);
        let dicetest = Dicetest::repeatedly().limit_multiplier(limit_multiplier);
        assert_eq!(limit_multiplier, dicetest.params.limit_multiplier);
    }

    #[test]
    fn set_passes() {
        let passes = 42;
        let dicetest = Dicetest::repeatedly().passes(passes);
        assert_eq!(passes, dicetest.params.passes);
    }

    #[test]
    fn set_passes_multiplier() {
        let passes_multiplier = Some(42.);
        let dicetest = Dicetest::repeatedly().passes_multiplier(passes_multiplier);
        assert_eq!(passes_multiplier, dicetest.params.passes_multiplier);
    }

    #[test]
    fn set_hints_enabled() {
        let hints_enabled = false;
        let dicetest = Dicetest::repeatedly().hints_enabled(hints_enabled);
        assert_eq!(hints_enabled, dicetest.params.hints_enabled);
    }

    #[test]
    fn set_stats_enabled() {
        let stats_enabled = true;
        let dicetest = Dicetest::repeatedly().stats_enabled(stats_enabled);
        assert_eq!(stats_enabled, dicetest.params.stats_enabled);
    }

    #[test]
    fn set_stats_max_value_count() {
        let stats_max_value_count = Some(42);
        let dicetest = Dicetest::repeatedly().stats_max_value_count(stats_max_value_count);
        assert_eq!(
            stats_max_value_count,
            dicetest.params.formatting.stats_max_value_count
        );
    }

    #[test]
    fn set_stats_percent_precision() {
        let stats_percent_precision = 42;
        let dicetest = Dicetest::repeatedly().stats_percent_precision(stats_percent_precision);
        assert_eq!(
            stats_percent_precision,
            dicetest.params.formatting.stats_percent_precision
        );
    }

    #[test]
    fn set_env_enabled() {
        let env_enabled = false;
        let dicetest = Dicetest::repeatedly().env_enabled(env_enabled);
        assert_eq!(env_enabled, dicetest.params.env_enabled);
    }
}
