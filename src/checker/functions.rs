use std::panic::{UnwindSafe, RefUnwindSafe};

use crate::gen::{Prng, Limit, Dice};
use crate::runner::{Run, Config, run_once, run_repeatedly};
use crate::formatter;
use crate::checker::{Panic, Mode, env};

/// Checks the test. How the test is checked can be configured with environment variables.
///
/// # Panics
///
/// You can configure in which cases this function should panic by using the following environment
/// variable:
///
/// - `DICETEST_PANIC=<panic>`
/// Whether this function panics depends on `<panic>` with the following options:
///     - `always`
///     This function panics always.
///     - `on_failure`
///     The default value. This function panics once a test run has failed.
///
/// The panic message contains a summary of the test runs.
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
/// The initial seed. See `Config::seed`. There are the following options for `<seed>`:
///     - `none`
///     The seed will be generated randomly.
///     - `<u64>`
///     This integer will be used as seed.
/// - `DICETEST_START_LIMIT=<u64>`
/// The initial `Limit` value. See `Config::start_limit`.
/// - `DICETEST_END_LIMIT=<u64>`
/// The final `Limit` value. See `Config::end_limit`.
/// - `DICETEST_PASSES=<u64>`
/// The number of test runs. See `Config::passes`.
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
/// The initial seed. See `Prng::init`. Ignored if `DICETEST_RUN_CODE` is present. There are the
/// following options for `<seed>`:
///     - `none`
///     The seed will be generated randomly.
///     - `<u64>`
///     This integer will be used as seed.
/// - `DICETEST_LIMIT=<u64>`
/// This integer will be used as `Limit`. Ignored if `DICETEST_RUN_CODE` is present.
/// - `DICETEST_RUN_CODE=<run code>`
/// Both seed and `Limit` will be decoded from the run code.
///
/// # Debug
///
/// The following environment variable allows to debug a falsified property easily:
///
/// - `DICETEST_DEBUG=<run code>` Both seed and `Limit` will be decoded from the
/// run code and the test will be checked a single time. This function will always panic
/// to present details of the test run. It's a shortcut for
/// `DICETEST_PANIC=always DICETEST_MODE=once DICETEST_RUN_CODE=<run code>`.
/// All other environment variables will be ignored.
#[allow(clippy::needless_pass_by_value)]
pub fn check<T>(config: Config, test: T)
where
    T: Fn(&mut Dice) + UnwindSafe + RefUnwindSafe,
{
    let debug_params = env::read_debug(None).unwrap();

    if let Some(params) = debug_params {
        let panic = Panic::Always;
        check_once(panic, params, |dice| test(dice));
    } else {
        let mode = env::read_mode(Mode::Repeatedly).unwrap();
        let panic = env::read_panic(Panic::default()).unwrap();

        match mode {
            Mode::Repeatedly => {
                let seed = env::read_seed(config.seed).unwrap();
                let start_limit = env::read_start_limit(config.start_limit).unwrap();
                let end_limit = env::read_end_limit(config.end_limit).unwrap();
                let passes = env::read_passes(config.passes).unwrap();
                let hints_enabled  = env::read_hints_enabled(config.hints_enabled).unwrap();
                let stats_enabled  = env::read_stats_enabled(config.stats_enabled).unwrap();

                let overriden_config = Config {
                    seed,
                    start_limit,
                    end_limit,
                    passes,
                    hints_enabled,
                    stats_enabled,
                };

                check_repeatedly(panic, overriden_config, test);
            }
            Mode::Once => {
                let code_params = env::read_run_code(None).unwrap();
                let run = code_params.unwrap_or_else(|| {
                    let seed = env::read_seed(None).unwrap();
                    let prng = seed.map_or_else(|| Prng::random(), Prng::init);
                    let limit = env::read_limit(Limit::default()).unwrap();
                    Run { prng, limit }
                });

                check_once(panic, run, |dice| test(dice))
            }
        }
    }
}

/// Checks the test by running it once with the given configuration.
///
/// # Panics
///
/// Panics depending on the given `panic`.
pub fn check_once<T>(panic: Panic, run: Run, test: T)
where
    T: FnOnce(&mut Dice) + UnwindSafe + RefUnwindSafe,
{
    let sample = run_once(run, test);

    let should_panic = match panic {
        Panic::Always => true,
        Panic::OnFailure => sample.error.is_some(),
    };

    if should_panic {
        let message = formatter::pretty_sample(&sample);
        panic!(message);
    }
}

/// Checks the test by running it repeatedly with the given configuration and different seeds.
///
/// The test will be run until the configured number of passes has been reached or a test run
/// has failed.
///
/// # Panics
///
/// Panics depending on the given `panic`.
pub fn check_repeatedly<T>(panic: Panic, config: Config, test: T)
where
    T: Fn(&mut Dice) + UnwindSafe + RefUnwindSafe,
{
    let summary = run_repeatedly(config, test);

    let should_panic = match panic {
        Panic::Always => true,
        Panic::OnFailure => summary.counterexample.is_some(),
    };

    if should_panic {
        let message = formatter::pretty_summary(&summary);
        panic!(message);
    }
}
