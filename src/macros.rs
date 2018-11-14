/// Macro for logging a message with `logger`. Supports the `format!` syntax.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        $crate::logger::log(|| format!($($arg)*))
    })
}

/// Macro for logging a variable with `logger`. Uses `Debug` formatting.
#[macro_export]
macro_rules! log_var {
    ($arg:tt) => ({
        $crate::logger::log(|| format!(concat!(stringify!($arg), ": {:?}"), $arg))
    })
}
