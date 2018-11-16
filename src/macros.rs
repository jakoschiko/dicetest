/// Macro for logging a message with `logger::log`. Supports the `format!` syntax.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        $crate::logger::log(|| format!($($arg)*))
    })
}

/// Macro for logging a variable with `logger::log`. Uses `Debug` formatting.
#[macro_export]
macro_rules! log_var {
    ($arg:tt) => ({
        $crate::logger::log(|| format!(concat!(stringify!($arg), ": {:?}"), $arg))
    })
}

/// Macro for checking the property with `asserts::assert_prop`. If the `Config` parameter is
/// omitted, it uses the default `Config`.
#[macro_export]
macro_rules! assert_prop {
    ($config:expr, $prop:expr) => ({
        $crate::asserts::assert_prop($config, || $prop);
    });
    ($prop:expr) => ({
        let config = $crate::brooder::Config::default();
        $crate::asserts::assert_prop(config, || $prop);
    })
}
