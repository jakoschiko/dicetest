/// Macro for adding a hint. Supports the `format!` syntax.
#[macro_export]
macro_rules! hint {
    ($($arg:tt)*) => ({
        $crate::hints::add(|| format!($($arg)*))
    })
}

/// Similar to dbg from rust stdlib. Adds a hint with the expression value using `Debug` formatting
/// and returns it.
#[macro_export]
macro_rules! hint_dbg {
    ($val:expr) => {
        // Takes ownership.
        match $val {
            tmp => {
                $crate::hints::add(|| format!(concat!(stringify!($val), " = {:?}"), &tmp));
                tmp
            }
        }
    }
}

/// Macro for creating a stat with the given key and value. Supports the `format!` syntax for
/// the value.
#[macro_export]
macro_rules! stat {
    ($key:tt, $($arg:tt)*) => ({
        $crate::stats::inc($key, || format!($($arg)*))
    })
}

/// Similar to dbg from rust stdlib. Creates a stat with the expression as key and the
/// expression value as value using `Debug` formatting. Returns the expression value.
#[macro_export]
macro_rules! stat_dbg {
    ($val:expr) => {
        // Takes ownership.
        match $val {
            tmp => {
                $crate::stats::inc(stringify!($val), || format!("{:?}", &tmp));
                tmp
            }
        }
    }
}

/// Macro for checking the test with `checker::check`. If the `Config` parameter is omitted,
/// the default `Config` will be used.
#[macro_export]
macro_rules! dicetest {
    ($config:expr, $test:expr) => ({
        $crate::checker::check($config, $test);
    });
    ($test:expr) => ({
        let config = $crate::runner::Config::default();
        $crate::checker::check(config, $test);
    })
}
