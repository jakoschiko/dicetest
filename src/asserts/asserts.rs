use std::panic::UnwindSafe;

use crate::rng::Rng;
use crate::gen::Limit;
use crate::prop::Prop;
use crate::brooder::{EvalParams, Config, brood_prop};
use crate::asserts::{Panic, Mode, env};

/// Checks the property. How the property is checked can be configured with environment variables.
///
/// # Panics
///
/// You can configure in which cases this function should panic by using the following environment
/// variable:
///
/// - `RUSTCHECK_PANIC=<panic>`
/// Whether the function panics depends on `<panic>` with the following options:
///     - `always`
///     This function panics always.
///     - `not_passed`
///     The default value. This function panics iff the property was not proven or has not passed.
///     - `never`
///     This function does not panic.
///
/// The panic message contains a summary of the evaluation results.
///
/// # Modes
///
/// There are two modes for checking the property. You can configure the mode by using the following
/// environment variable:
///
/// - `RUSTCHECK_MODE=<mode>`
/// How the property will be checked depends on `<mode>` with the following options:
///     - `brooder`
///     The default value. The property will be evaluated several times and the result will be
///     summarized.
///     - `sample`
///     The property will be evaluated a single time.
///
/// Each mode can be configured with additional environment variables.
///
/// # Brooder mode configuration
///
/// By default the given `Config` will be used. However, you can override the `Config` by using the
/// following environment variables:
///
/// - `RUSTCHECK_SEED=<seed>`
/// The initial seed. See `Config::seed`. There are the following options for `<seed>`:
///     - `none`
///     The seed will be generated randomly.
///     - `<u64>`
///     This integer will be used as seed.
/// - `RUSTCHECK_START_LIMIT=<u64>`
/// The initial `Limit` value. See `Config::start_limit`.
/// - `RUSTCHECK_END_LIMIT=<u64>`
/// The final `Limit` value. See `Config::end_limit`.
/// - `RUSTCHECK_MIN_PASSED=<u64>`
/// The maximum number of passes. See `Config::min_passed`.
/// - `RUSTCHECK_WORKER_COUNT=<u64>`
/// The number of workers. See `Config::worker_count`.
/// - `RUSTCHECK_TIMEOUT=<timeout>`
/// The timeout for aborting the check. See `Config::timeout`. There are the following options
/// for `<timeout>`:
///     - `none`
///     There is no timeout.
///     - `<f64>`
///     The timeout in seconds.
///
///
/// # Sample mode configuration
///
/// By default a random seed and the default `Limit` will be used. However, you can override these
/// parameters by using the following environment variables:
///
/// - `RUSTCHECK_SEED=<seed>`
/// The initial seed. See `Rng::init`. Ignored if `RUSTCHECK_CODE` is present. There are the
/// following options for `<seed>`:
///     - `none`
///     The seed will be generated randomly.
///     - `<u64>`
///     This integer will be used as seed.
/// - `RUSTCHECK_LIMIT=<u64>`
/// This integer will be used as `Limit`. Ignored if `RUSTCHECK_CODE` is present.
/// - `RUSTCHECK_CODE=<evaluation code>`
/// Both seed and `Limit` will be decoded from the evaluation code.
///
/// # Debug
///
/// The following environment variable allows to debug a falsified property easily:
///
/// - `RUSTCHECK_DEBUG=<evaluation code>` Both seed and `Limit` will be decoded from the
/// evaluation code and the property will be evaluated a single time. The function will always panic
/// to present the evaluation result. It's a shortcut for
/// `RUSTCHECK_PANIC=always RUSTCHECK_MODE=sample RUSTCHECK_CODE=<evaluation code>`.
/// All other environment variables will be ignored.
pub fn assert_prop<P, F>(config: Config, prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Clone + Send + UnwindSafe + 'static,
{
    let debug_params = env::read_debug(None).unwrap();

    if let Some(params) = debug_params {
        let panic = Panic::Always;
        assert_prop_with_sample(panic, params, prop_fn());
    } else {
        let mode = env::read_mode(Mode::Brooder).unwrap();
        let panic = env::read_panic(Panic::default()).unwrap();

        match mode {
            Mode::Brooder => {
                let seed = env::read_seed(config.seed).unwrap();
                let start_limit = env::read_start_limit(config.start_limit).unwrap();
                let end_limit = env::read_end_limit(config.end_limit).unwrap();
                let min_passed = env::read_min_passed(config.min_passed).unwrap();
                let worker_count = env::read_worker_count(config.worker_count).unwrap();
                let timeout  = env::read_timeout(config.timeout).unwrap();

                let overriden_config = Config {
                    seed,
                    start_limit,
                    end_limit,
                    min_passed,
                    worker_count,
                    timeout,
                };

                assert_prop_with_brooder(panic, overriden_config, prop_fn);
            }
            Mode::Sample => {
                let code_params = env::read_code(None).unwrap();
                let params = code_params.unwrap_or_else(|| {
                    let seed = env::read_seed(None).unwrap();
                    let rng = seed.map_or_else(|| Rng::random(), Rng::init);
                    let limit = env::read_limit(Limit::default()).unwrap();
                    EvalParams { rng, limit }
                });

                assert_prop_with_sample(panic, params, prop_fn())
            }
        }
    }
}


/// Checks the property by evaluating it several times.
pub fn assert_prop_with_brooder<P, F>(panic: Panic, config: Config, prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Clone + Send + UnwindSafe + 'static,
{
    let report = brood_prop(config, prop_fn);

    let should_panic = panic.should_panic_with_status(&report.status);

    if should_panic {
        panic!(report.pretty());
    }
}

/// Checks the property by evaluating it a single time.
pub fn assert_prop_with_sample<P>(panic: Panic, params: EvalParams, prop: P)
where
    P: Prop,
{
    let eval_code = params.eval_code();

    let mut rng = params.rng;
    let lim = params.limit;
    let sample = prop.sample_with_params(&mut rng, lim);

    let should_panic = panic.should_panic_with_eval(sample.eval);

    if should_panic {
        panic!(
            "\
            Property evaluation sample\n\
            Evaluation code: \"{}\"\n\
            Limit: {}\n\
            {}\n\
            ",
            eval_code,
            lim.0,
            sample.pretty(),
        );
    }
}
