/// Adds a hint that contains the arguments applied to the `format` macro.
#[macro_export]
macro_rules! hint {
    ($($arg:tt)*) => ({
        $crate::hints::add(|| format!($($arg)*))
    })
}

/// Adds a hint that contains the stringified argument and the argument value converted with
/// `Debug`.
#[macro_export]
macro_rules! hint_debug {
    ($arg:tt) => {
        $crate::hints::add(|| format!(concat!("{} = {:?}"), stringify!($arg), $arg));
    };
}

/// Creates a stat with the first argument as stat key and the remaining arguments applied to the
/// `format` macro as stat value.
#[macro_export]
macro_rules! stat {
    ($key:tt, $($arg:tt)*) => ({
        $crate::stats::inc($key, || format!($($arg)*))
    })
}

/// Creates a stat with the stringified argument as stat key and the argument value converted with
/// `Debug` as stat value.
#[macro_export]
macro_rules! stat_debug {
    ($arg:tt) => {
        $crate::stats::inc(stringify!($arg), || format!("{:?}", $arg));
    };
}

/// Checks the test with `checker::check`. The config can be omitted.
///
/// If the `Config` parameter is omitted, the default `Config` will be used.
#[macro_export]
macro_rules! dicetest {
    ($config:expr, $test:expr) => {{
        $crate::checker::check($config, $test);
    }};
    ($test:expr) => {{
        let config = $crate::runner::Config::default();
        $crate::checker::check(config, $test);
    }};
}

#[cfg(test)]
mod tests {
    use crate::runner::Config;

    #[test]
    fn macro_hint_produces_valid_code() {
        if false {
            hint!("foo");
            hint!("bar {}", 42);
        }
    }

    #[test]
    fn macro_hint_debug_produces_valid_code() {
        if false {
            hint_debug!(42);
            hint_debug!((0 < 20));
            hint_debug!((if true { 1 } else { 2 }));
        }
    }

    #[test]
    fn macro_stat_produces_valid_code() {
        if false {
            stat!("A", "foo");
            stat!("B", "bar {}", 42);
        }
    }

    #[test]
    fn macro_stat_debug_produces_valid_code() {
        if false {
            stat_debug!(42);
            stat_debug!((0 < 20));
            stat_debug!((if true { 1 } else { 2 }));
        }
    }

    #[test]
    fn macro_dicetest_produces_valid_code() {
        if false {
            dicetest!(|_fate| assert_eq!(1, 2));
            dicetest!(Config::default(), |_fate| assert_eq!(1, 2));
        }
    }
}
