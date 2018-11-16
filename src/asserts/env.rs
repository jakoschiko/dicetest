use std::env;
use std::str::FromStr;
use std::time::Duration;

use crate::gen::Limit;
use crate::brooder::EvalParams;
use crate::asserts::{Panic, Mode};

const KEY_PANIC: &'static str = "RUSTCHECK_PANIC";
const KEY_MODE: &'static str = "RUSTCHECK_MODE";
const KEY_SEED: &'static str = "RUSTCHECK_SEED";
const KEY_START_LIMIT: &'static str = "RUSTCHECK_START_LIMIT";
const KEY_END_LIMIT: &'static str = "RUSTCHECK_END_LIMIT";
const KEY_MIN_PASSED: &'static str = "RUSTCHECK_MIN_PASSED";
const KEY_WORKER_COUNT: &'static str = "RUSTCHECK_WORKER_COUNT";
const KEY_TIMEOUT: &'static str = "RUSTCHECK_TIMEOUT";
const KEY_LIMIT: &'static str = "RUSTCHECK_LIMIT";
const KEY_CODE: &'static str = "RUSTCHECK_CODE";
const KEY_DEBUG: &'static str = "RUSTCHECK_DEBUG";

const VALUE_NONE: &'static str = "never";
const VALUE_ALWAYS: &'static str = "always";
const VALUE_NOT_PASSED: &'static str = "not_passed";
const VALUE_NEVER: &'static str = "never";
const VALUE_BROODER: &'static str = "brooder";
const VALUE_SAMPLE: &'static str = "sample";

fn read_value<T, E>(
    key: &str,
    typ: &str,
    default: T,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> Result<T, String> {
    let var = env::var(key);
    let parsed = var.map(|s| {
        let result = parse(&s);
        result.map_err(|_|format!("Value for '{}' must be {}", key, typ))
    });
    parsed.unwrap_or(Ok(default))
}

fn read_option_value<T, E>(
    key: &str,
    typ: &str,
    default: Option<T>,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> Result<Option<T>, String> {
    let var = env::var(key);
    let parsed = var.map(|s| {
        let result = if &s == VALUE_NONE { Ok(None) } else { parse(&s).map(|v| Some(v)) };
        result.map_err(|_| {
            format!("Value for '{}' must be either '{}' or {}", key, VALUE_NONE, typ)
        })
    });
    parsed.unwrap_or(Ok(default))
}

pub fn read_seed(default: Option<u64>) -> Result<Option<u64>, String> {
    read_option_value(KEY_SEED, "an integer", default, u64::from_str)
}

pub fn read_start_limit(default: u64) -> Result<u64, String> {
    read_value(KEY_START_LIMIT, "an u64", default, u64::from_str)
}

pub fn read_end_limit(default: u64) -> Result<u64, String> {
    read_value(KEY_END_LIMIT, "an u64", default, u64::from_str)
}

pub fn read_min_passed(default: u64) -> Result<u64, String> {
    read_value(KEY_MIN_PASSED, "an u64", default, u64::from_str)
}

pub fn read_worker_count(default: u64) -> Result<u64, String> {
    read_value(KEY_WORKER_COUNT, "an u64", default, u64::from_str)
}

pub fn read_timeout(default: Option<Duration>) -> Result<Option<Duration>, String> {
    read_option_value(KEY_TIMEOUT, "a f64", default,
        |s| f32::from_str(s).map(|secs| Duration::from_nanos((secs * 1e-9) as u64)))
}

pub fn read_panic(default: Panic) -> Result<Panic, String> {
    match env::var(KEY_PANIC) {
        Err(_) => Ok(default),
        Ok(var) => {
            let str = var.as_str();
            if str == VALUE_ALWAYS { Ok(Panic::Always) }
            else if str == VALUE_NOT_PASSED { Ok(Panic::NotPassed) }
            else if str == VALUE_NEVER { Ok(Panic::Never) }
            else {
                let error = format!(
                    "Value for '{}' must be either '{}', '{}' or '{}'",
                    KEY_PANIC, VALUE_ALWAYS, VALUE_NOT_PASSED, VALUE_NEVER
                );
                Err(error)
            }
        }
    }
}

pub fn read_mode(default: Mode) -> Result<Mode, String> {
    match env::var(KEY_MODE) {
        Err(_) => Ok(default),
        Ok(var) => {
            let str = var.as_str();
            if str == VALUE_BROODER { Ok(Mode::Brooder) }
            else if str == VALUE_SAMPLE { Ok(Mode::Sample) }
            else {
                let error = format!(
                    "Value for '{}' must be either '{}', or '{}'",
                    KEY_MODE, VALUE_BROODER, VALUE_SAMPLE
                );
                Err(error)
            }
        }
    }
}

pub fn read_limit(default: Limit) -> Result<Limit, String> {
    read_value(KEY_LIMIT, "an u64", default, |s| u64::from_str(s).map(|lim| Limit(lim)))
}

fn read_code_with_key(
    key: &'static str,
    default: Option<EvalParams>
) -> Result<Option<EvalParams>, String> {
    read_option_value(key, "a valid evaluation code", default, EvalParams::from_eval_code)
}

pub fn read_code(default: Option<EvalParams>) -> Result<Option<EvalParams>, String> {
    read_code_with_key(KEY_CODE, default)
}

pub fn read_debug(default: Option<EvalParams>) -> Result<Option<EvalParams>, String> {
    read_code_with_key(KEY_DEBUG, default)
}
