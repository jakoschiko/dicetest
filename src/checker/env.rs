use std::env;
use std::str::FromStr;

use crate::checker::{LogCondition, Mode};
use crate::die::Limit;
use crate::runner::Run;

const KEY_LOG_CONDITION: &str = "DICETEST_LOG_CONDITION";
const KEY_MODE: &str = "DICETEST_MODE";
const KEY_SEED: &str = "DICETEST_SEED";
const KEY_START_LIMIT: &str = "DICETEST_START_LIMIT";
const KEY_END_LIMIT: &str = "DICETEST_END_LIMIT";
const KEY_LIMIT_MULTIPLIER: &str = "DICETEST_LIMIT_MULTIPLIER";
const KEY_PASSES: &str = "DICETEST_PASSES";
const KEY_PASSES_MULTIPLIER: &str = "DICETEST_PASSES_MULTIPLIER";
const KEY_HINTS_ENABLED: &str = "DICETEST_HINTS_ENABLED";
const KEY_STATS_ENABLED: &str = "DICETEST_STATS_ENABLED";
const KEY_LIMIT: &str = "DICETEST_LIMIT";
const KEY_RUN_CODE: &str = "DICETEST_RUN_CODE";
const KEY_DEBUG: &str = "DICETEST_DEBUG";

const VALUE_NONE: &str = "none";
const VALUE_ALWAYS: &str = "always";
const VALUE_ON_FAILURE: &str = "on_failure";
const VALUE_REPEATEDLY: &str = "repeatedly";
const VALUE_ONCE: &str = "once";

fn read_value<T, E>(
    key: &str,
    typ: &str,
    default: T,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> Result<T, String> {
    let var = env::var(key);
    let parsed = var.map(|s| {
        let result = parse(&s);
        result.map_err(|_| format!("Value for '{}' must be {}", key, typ))
    });
    parsed.unwrap_or_else(|_| Ok(default))
}

fn read_option_value<T, E>(
    key: &str,
    typ: &str,
    default: Option<T>,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> Result<Option<T>, String> {
    let var = env::var(key);
    let parsed = var.map(|s| {
        let result = if s == VALUE_NONE {
            Ok(None)
        } else {
            parse(&s).map(Some)
        };
        result.map_err(|_| {
            format!(
                "Value for '{}' must be either '{}' or {}",
                key, VALUE_NONE, typ
            )
        })
    });
    parsed.unwrap_or_else(|_| Ok(default))
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

pub fn read_limit_multiplier(default: Option<f64>) -> Result<Option<f64>, String> {
    read_value(KEY_LIMIT_MULTIPLIER, "a f64", default, |s| {
        f64::from_str(s).map(Some)
    })
}

pub fn read_passes(default: u64) -> Result<u64, String> {
    read_value(KEY_PASSES, "an u64", default, u64::from_str)
}

pub fn read_passes_multiplier(default: Option<f64>) -> Result<Option<f64>, String> {
    read_value(KEY_PASSES_MULTIPLIER, "a f64", default, |s| {
        f64::from_str(s).map(Some)
    })
}

pub fn read_hints_enabled(default: bool) -> Result<bool, String> {
    read_value(KEY_HINTS_ENABLED, "a bool", default, bool::from_str)
}

pub fn read_stats_enabled(default: bool) -> Result<bool, String> {
    read_value(KEY_STATS_ENABLED, "a bool", default, bool::from_str)
}

pub fn read_log_condition(default: LogCondition) -> Result<LogCondition, String> {
    match env::var(KEY_LOG_CONDITION) {
        Err(_) => Ok(default),
        Ok(var) => {
            let str = var.as_str();
            if str == VALUE_ALWAYS {
                Ok(LogCondition::Always)
            } else if str == VALUE_ON_FAILURE {
                Ok(LogCondition::OnFailure)
            } else {
                let error = format!(
                    "Value for '{}' must be either '{}' or '{}'",
                    KEY_LOG_CONDITION, VALUE_ALWAYS, VALUE_ON_FAILURE
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
            if str == VALUE_REPEATEDLY {
                Ok(Mode::Repeatedly)
            } else if str == VALUE_ONCE {
                Ok(Mode::Once)
            } else {
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
    read_value(KEY_LIMIT, "an u64", default, |s| {
        u64::from_str(s).map(Limit)
    })
}

fn read_run_code_with_key(key: &'static str, default: Option<Run>) -> Result<Option<Run>, String> {
    read_option_value(key, "a valid run code", default, Run::from_run_code)
}

pub fn read_run_code(default: Option<Run>) -> Result<Option<Run>, String> {
    read_run_code_with_key(KEY_RUN_CODE, default)
}

pub fn read_debug(default: Option<Run>) -> Result<Option<Run>, String> {
    read_run_code_with_key(KEY_DEBUG, default)
}
