use std::panic::{resume_unwind, RefUnwindSafe, UnwindSafe};

use crate::checker::{env, LogCondition, Mode};
use crate::die::{Fate, Limit};
use crate::formatter::{self, Formatting};
use crate::prand::{Prng, Seed};
use crate::runner::{run_once, run_repeatedly, Config, Run};

/// Checks the test. How the test is checked can be configured with environment variables.
///
/// # Panics
///
/// Panics if and only if a test run has failed or an malformed environment variable is present.
///
/// # Stdout
///
/// You can configure when the test result will be logged to stdout by using the following
/// environment variable:
///
/// - `DICETEST_LOG_CONDITION=<log condition>`
/// Whether the test result will be logged depends on `<log condition>` with the following
/// options:
///     - `always`
///     The test result will be always logged.
///     - `on_failure`
///     The default value. The test result will be logged if and only if a test run has failed.
///
/// There are some environment variables that configures the output format:
///
/// - `DICETEST_STATS_MAX_VALUE_COUNT=<max_value_count>`
/// The maximum number of values per key used when formatting stats.
/// See `Formatting::stats_max_value_count`.
/// There are the following options for `<max_value_count>`:
///     - `none`
///     There is no maximum number.
///     - `<u64>`
///     This integer will be used as maximum number.
/// - `DICETEST_STATS_PERCENT_PRECISION=<usize>`
/// The number of decimal places for percent values used when formatting stats.
/// See `Formatting::stats_percent_precision`.
///
/// # Modes
///
/// There are two modes for checking the test. You can configure the mode by using the following
/// environment variable:
///
/// - `DICETEST_MODE=<mode>`
/// How the test will be checked depends on `<mode>` with the following options:
///     - `repeatedly`
///     The default value. The test will be run repeatedly and the result will be
///     summarized.
///     - `once`
///     The test will be run a single time.
///
/// Each mode can be configured with additional environment variables.
///
/// # `repeatedly` mode configuration
///
/// By default the given `Config` will be used. However, you can override the `Config` by using the
/// following environment variables:
///
/// - `DICETEST_SEED=<seed>`
/// The initial `Seed`. See `Config::seed`. There are the following options for `<seed>`:
///     - `none`
///     The `Seed` will be generated randomly.
///     - `<u64>`
///     This integer will be used as `Seed`.
/// - `DICETEST_START_LIMIT=<u64>`
/// The initial `Limit` value. See `Config::start_limit`.
/// - `DICETEST_END_LIMIT=<u64>`
/// The final `Limit` value. See `Config::end_limit`.
/// - `DICETEST_LIMIT_MULTIPLIER=<f64>`
/// Multiplies the initial and the final `Limit` values with the given factor.
/// See `Config::with_multiplied_limit`.
/// - `DICETEST_PASSES=<u64>`
/// The number of test runs. See `Config::passes`.
/// - `DICETEST_PASSES_MULTIPLIER=<f64>`
/// Multiplies the number of test runs with the given factor.
/// See `Config::with_multiplied_passes`.
/// - `DICETEST_HINTS_ENABLED=<bool>`
/// Enables the hints. See `Config::hints_enabled`.
/// - `DICETEST_STATS_ENABLED=<bool>`
/// Enables the stats. See `Config::stats_enabled`.
///
/// # `once` mode configuration
///
/// By default a random seed and the default `Limit` will be used. However, you can override these
/// parameters by using the following environment variables:
///
/// - `DICETEST_SEED=<seed>`
/// The initial `Seed`. Ignored if `DICETEST_RUN_CODE` is present. There are the
/// following options for `<seed>`:
///     - `none`
///     The default value. The `Seed` will be generated randomly.
///     - `<u64>`
///     This integer will be used as `Seed`.
/// - `DICETEST_LIMIT=<u64>`
/// This integer will be used as `Limit`. If not present, the default `Limit` will be used.
/// Ignored if `DICETEST_RUN_CODE` is present.
/// - `DICETEST_RUN_CODE=<run code>`
/// Both `Seed` and `Limit` will be decoded from the run code.
///
/// # Debug
///
/// The following environment variable allows to debug a falsified property easily:
///
/// - `DICETEST_DEBUG=<run code>` Both `Seed` and `Limit` will be decoded from the
/// run code and the test will be checked a single time. This function logs always the test result.
/// It's an alias for
/// `DICETEST_LOG_CONDITION=always DICETEST_MODE=once DICETEST_RUN_CODE=<run code>`.
/// All other environment variables will be ignored.
#[allow(clippy::needless_pass_by_value)]
pub fn check<T>(config: Config, test: T)
where
    T: Fn(Fate) + UnwindSafe + RefUnwindSafe,
{
    let debug_params = env::read_debug(None).unwrap();

    if let Some(params) = debug_params {
        let log_condition = LogCondition::Always;
        check_once(log_condition, params, |fate| test(fate));
    } else {
        let mode = env::read_mode(Mode::Repeatedly).unwrap();
        let log_condition = env::read_log_condition(LogCondition::default()).unwrap();

        match mode {
            Mode::Repeatedly => {
                let formatting = read_formatting_from_env().unwrap();
                let overridden_config = override_config_from_env(&config).unwrap();

                check_repeatedly(log_condition, formatting, overridden_config, test);
            }
            Mode::Once => {
                let code_params = env::read_run_code(None).unwrap();
                let run = code_params.unwrap_or_else(|| {
                    let seed = env::read_seed(None).unwrap();
                    let prng = Prng::from_seed(seed.unwrap_or_else(Seed::random));
                    let limit = env::read_limit(Limit::default()).unwrap();
                    Run { prng, limit }
                });

                check_once(log_condition, run, |fate| test(fate))
            }
        }
    }
}

/// Checks the test by running it once with the given configuration.
///
/// # Panics
///
/// Panics if and only if the test run has failed.
///
/// # Stdout
///
/// Depending on the `LogCondition` the test result will be logged to stdout.
pub fn check_once<T>(log_condition: LogCondition, run: Run, test: T)
where
    T: FnOnce(Fate) + UnwindSafe + RefUnwindSafe,
{
    let sample = run_once(run, test);

    let should_print = match log_condition {
        LogCondition::Always => true,
        LogCondition::OnFailure => sample.error.is_some(),
    };

    if should_print {
        let message = formatter::pretty_sample(&sample);
        log(&message);
    }

    if let Some(err) = sample.error.map(|e| e.0) {
        resume_unwind(err);
    }
}

/// Checks the test by running it repeatedly with the given configuration and different seeds.
///
/// The test will be run until the configured number of passes has been reached or a test run
/// has failed.
///
/// # Panics
///
/// Panics if and only if a test run has failed.
///
/// # Stdout
///
/// Depending on the `LogCondition` the test result will be logged to stdout. The output format can
/// be configured with the `Formatting`.
pub fn check_repeatedly<T>(
    log_condition: LogCondition,
    formatting: Formatting,
    config: Config,
    test: T,
) where
    T: Fn(Fate) + UnwindSafe + RefUnwindSafe,
{
    let summary = run_repeatedly(config, test);

    let should_log = match log_condition {
        LogCondition::Always => true,
        LogCondition::OnFailure => summary.counterexample.is_some(),
    };

    if should_log {
        let message = formatter::pretty_summary(&summary, formatting);
        log(&message);
    }

    if let Some(err) = summary.counterexample.map(|c| c.error.0) {
        resume_unwind(err);
    }
}

fn log(text: &str) {
    print!("{}", text);
}

fn override_config_from_env(config: &Config) -> Result<Config, String> {
    let seed = env::read_seed(config.seed).unwrap();
    let start_limit = env::read_start_limit(config.start_limit).unwrap();
    let end_limit = env::read_end_limit(config.end_limit).unwrap();
    let passes = env::read_passes(config.passes).unwrap();
    let hints_enabled = env::read_hints_enabled(config.hints_enabled).unwrap();
    let stats_enabled = env::read_stats_enabled(config.stats_enabled).unwrap();

    let overriden_config = Config {
        seed,
        start_limit,
        end_limit,
        passes,
        hints_enabled,
        stats_enabled,
    };

    let overriden_config = match env::read_limit_multiplier(None)? {
        Some(factor) => overriden_config.with_multiplied_limit(factor),
        None => overriden_config,
    };

    let overriden_config = match env::read_passes_multiplier(None)? {
        Some(factor) => overriden_config.with_multiplied_passes(factor),
        None => overriden_config,
    };

    Ok(overriden_config)
}

fn read_formatting_from_env() -> Result<Formatting, String> {
    let default_formatting = Formatting::default();

    let stats_max_value_count =
        env::read_stats_max_value_count(default_formatting.stats_max_value_count)?;
    let stats_percent_precision =
        env::read_stats_percent_precision(default_formatting.stats_percent_precision)?;

    Ok(Formatting {
        stats_max_value_count,
        stats_percent_precision,
    })
}
