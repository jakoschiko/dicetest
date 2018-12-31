/// Adds a hint that contains the expression result converted with `Debug`.
#[macro_export]
macro_rules! hint {
    ($val:expr) => {
        $crate::hints::add(|| format!(concat!(stringify!($val), " = {:?}"), $val));
    };
}

/// Adds a hint that contains the result of the arguments applied to the `format` macro.
#[macro_export]
macro_rules! hint_format {
    ($($arg:tt)*) => ({
        $crate::hints::add(|| format!($($arg)*))
    })
}

/// Creates a stat with the expression as key and the expression result converted with `Debug` as
/// value.
#[macro_export]
macro_rules! stat {
    ($expr:expr) => {
        $crate::stats::inc(stringify!($expr), || format!("{:?}", $expr));
    };
}

/// Creates a stat with the first argument as key and the remaining arguments applied to the
/// `format` macro as value.
#[macro_export]
macro_rules! stat_format {
    ($key:tt, $($arg:tt)*) => ({
        $crate::stats::inc($key, || format!($($arg)*))
    })
}

/// Macro for checking the test with `checker::check`. If the `Config` parameter is omitted,
/// the default `Config` will be used.
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
