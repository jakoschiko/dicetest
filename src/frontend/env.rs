use std::env::{self, VarError};
use std::str::FromStr;

use crate::frontend::{Mode, RunCode};
use crate::{Limit, Seed};

const KEY_MODE: &str = "DICETEST_MODE";
const KEY_DEBUG: &str = "DICETEST_DEBUG";
const KEY_SEED: &str = "DICETEST_SEED";
const KEY_ONCE_LIMIT: &str = "DICETEST_ONCE_LIMIT";
const KEY_START_LIMIT: &str = "DICETEST_START_LIMIT";
const KEY_END_LIMIT: &str = "DICETEST_END_LIMIT";
const KEY_LIMIT_MULTIPLIER: &str = "DICETEST_LIMIT_MULTIPLIER";
const KEY_PASSES: &str = "DICETEST_PASSES";
const KEY_PASSES_MULTIPLIER: &str = "DICETEST_PASSES_MULTIPLIER";
const KEY_HINTS_ENABLED: &str = "DICETEST_HINTS_ENABLED";
const KEY_STATS_ENABLED: &str = "DICETEST_STATS_ENABLED";
const KEY_STATS_MAX_VALUE_COUNT: &str = "DICETEST_STATS_MAX_VALUE_COUNT";
const KEY_STATS_PERCENT_PRECISION: &str = "DICETEST_STATS_PERCENT_PRECISION";

const VALUE_NONE: &str = "none";
const VALUE_REPEATEDLY: &str = "repeatedly";
const VALUE_ONCE: &str = "once";

pub enum EnvValue<T> {
    NotPresent,
    Present(T),
}

pub fn read_mode() -> Result<EnvValue<Mode>, String> {
    let key = KEY_DEBUG;
    match env::var(key) {
        Err(VarError::NotPresent) => read_non_debug_mode(),
        Err(err) => handle_var_error(key, err),
        Ok(s) => match RunCode::from_base64(&s) {
            Ok(run_code) => Ok(EnvValue::Present(Mode::Debug(run_code))),
            Err(err) => Err(format!("Value for '{}' is not valid: {}", key, err)),
        },
    }
}

fn read_non_debug_mode() -> Result<EnvValue<Mode>, String> {
    match env::var(KEY_MODE) {
        Err(err) => handle_var_error(KEY_MODE, err),
        Ok(var) => {
            let str = var.as_str();
            if str == VALUE_REPEATEDLY {
                Ok(EnvValue::Present(Mode::Repeatedly))
            } else if str == VALUE_ONCE {
                Ok(EnvValue::Present(Mode::Once))
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

pub fn read_seed() -> Result<EnvValue<Option<Seed>>, String> {
    read_option_value(KEY_SEED, "an u64", |s| u64::from_str(s).map(Seed))
}

pub fn read_once_limit() -> Result<EnvValue<Limit>, String> {
    read_value(KEY_ONCE_LIMIT, "an u64", |s| u64::from_str(s).map(Limit))
}

pub fn read_start_limit() -> Result<EnvValue<Limit>, String> {
    read_value(KEY_START_LIMIT, "an u64", |s| u64::from_str(s).map(Limit))
}

pub fn read_end_limit() -> Result<EnvValue<Limit>, String> {
    read_value(KEY_END_LIMIT, "an u64", |s| u64::from_str(s).map(Limit))
}

pub fn read_limit_multiplier() -> Result<EnvValue<Option<f64>>, String> {
    read_option_value(KEY_LIMIT_MULTIPLIER, "a f64", |s| f64::from_str(s))
}

pub fn read_passes() -> Result<EnvValue<u64>, String> {
    read_value(KEY_PASSES, "an u64", u64::from_str)
}

pub fn read_passes_multiplier() -> Result<EnvValue<Option<f64>>, String> {
    read_option_value(KEY_PASSES_MULTIPLIER, "a f64", |s| f64::from_str(s))
}

pub fn read_hints_enabled() -> Result<EnvValue<bool>, String> {
    read_value(KEY_HINTS_ENABLED, "a bool", bool::from_str)
}

pub fn read_stats_enabled() -> Result<EnvValue<bool>, String> {
    read_value(KEY_STATS_ENABLED, "a bool", bool::from_str)
}

pub fn read_stats_max_value_count() -> Result<EnvValue<Option<usize>>, String> {
    read_option_value(KEY_STATS_MAX_VALUE_COUNT, "an usize", usize::from_str)
}

pub fn read_stats_percent_precision() -> Result<EnvValue<usize>, String> {
    read_value(KEY_STATS_PERCENT_PRECISION, "an usize", usize::from_str)
}

fn read_value<T, E>(
    key: &str,
    typ: &str,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> Result<EnvValue<T>, String> {
    match env::var(key) {
        Err(err) => handle_var_error(key, err),
        Ok(s) => match parse(&s) {
            Ok(value) => Ok(EnvValue::Present(value)),
            Err(_) => Err(format!("Value for '{}' must be {}", key, typ)),
        },
    }
}

fn read_option_value<T, E>(
    key: &str,
    typ: &str,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> Result<EnvValue<Option<T>>, String> {
    match env::var(key) {
        Err(err) => handle_var_error(key, err),
        Ok(s) if s == VALUE_NONE => Ok(EnvValue::Present(None)),
        Ok(s) => match parse(&s) {
            Ok(value) => Ok(EnvValue::Present(Some(value))),
            Err(_) => Err(format!(
                "Value for '{}' must be either '{}' or {}",
                key, VALUE_NONE, typ
            )),
        },
    }
}

fn handle_var_error<T>(key: &str, err: VarError) -> Result<EnvValue<T>, String> {
    match err {
        VarError::NotPresent => Ok(EnvValue::NotPresent),
        VarError::NotUnicode(_) => Err(format!("Value for '{}' is not valid unicode", key)),
    }
}
