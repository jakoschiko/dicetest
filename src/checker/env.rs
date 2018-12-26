use std::env;
use std::str::FromStr;

use crate::gen::Limit;
use crate::runner::Run;
use crate::checker::{Panic, Mode};

const KEY_PANIC: &'static str = "DICETEST_PANIC";
const KEY_MODE: &'static str = "DICETEST_MODE";
const KEY_SEED: &'static str = "DICETEST_SEED";
const KEY_START_LIMIT: &'static str = "DICETEST_START_LIMIT";
const KEY_END_LIMIT: &'static str = "DICETEST_END_LIMIT";
const KEY_PASSES: &'static str = "DICETEST_PASSES";
const KEY_HINTS_ENABLED: &'static str = "DICETEST_HINTS_ENABLED";
const KEY_STATS_ENABLED: &'static str = "DICETEST_STATS_ENABLED";
const KEY_LIMIT: &'static str = "DICETEST_LIMIT";
const KEY_RUN_CODE: &'static str = "DICETEST_RUN_CODE";
const KEY_DEBUG: &'static str = "DICETEST_DEBUG";

const VALUE_NONE: &'static str = "none";
const VALUE_ALWAYS: &'static str = "always";
const VALUE_ON_FAILURE: &'static str = "on_failure";
const VALUE_REPEATEDLY: &'static str = "repeatedly";
const VALUE_ONCE: &'static str = "once";

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

pub fn read_passes(default: u64) -> Result<u64, String> {
    read_value(KEY_PASSES, "an u64", default, u64::from_str)
}

pub fn read_hints_enabled(default: bool) -> Result<bool, String> {
    read_value(KEY_HINTS_ENABLED, "a bool", default, bool::from_str)
}

pub fn read_stats_enabled(default: bool) -> Result<bool, String> {
    read_value(KEY_STATS_ENABLED, "a bool", default, bool::from_str)
}

pub fn read_panic(default: Panic) -> Result<Panic, String> {
    match env::var(KEY_PANIC) {
        Err(_) => Ok(default),
        Ok(var) => {
            let str = var.as_str();
            if str == VALUE_ALWAYS { Ok(Panic::Always) }
            else if str == VALUE_ON_FAILURE { Ok(Panic::OnFailure) }
            else {
                let error = format!(
                    "Value for '{}' must be either '{}' or '{}'",
                    KEY_PANIC, VALUE_ALWAYS, VALUE_ON_FAILURE
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
            if str == VALUE_REPEATEDLY { Ok(Mode::Repeatedly) }
            else if str == VALUE_ONCE { Ok(Mode::Once) }
            else {
                let error = format!(
                    "Value for '{}' must be either '{}', or '{}'",
                    KEY_MODE, VALUE_REPEATEDLY, VALUE_ONCE
                );
                Err(error)
            }
        }
    }
}

pub fn read_limit(default: Limit) -> Result<Limit, String> {
    read_value(KEY_LIMIT, "an u64", default, |s| u64::from_str(s).map(|lim| Limit(lim)))
}

fn read_run_code_with_key(
    key: &'static str,
    default: Option<Run>
) -> Result<Option<Run>, String> {
    read_option_value(key, "a valid run code", default, Run::from_run_code)
}

pub fn read_run_code(default: Option<Run>) -> Result<Option<Run>, String> {
    read_run_code_with_key(KEY_RUN_CODE, default)
}

pub fn read_debug(default: Option<Run>) -> Result<Option<Run>, String> {
    read_run_code_with_key(KEY_DEBUG, default)
}
