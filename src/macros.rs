/// Adds a hint that contains the arguments applied to the [`format`] macro.
///
/// # Examples
///
/// ```
/// use dicetest::hint;
///
/// let unknown_value = 42;
/// hint!("Revealing the unknown value: {}", unknown_value);
/// ```
#[macro_export]
macro_rules! hint {
    ($($arg:tt)*) => ({
        $crate::hints::add(|| format!($($arg)*))
    })
}

/// Adds a hint that contains the stringified argument and the argument value converted with
/// [`Debug`].
///
/// [`Debug`]: std::fmt::Debug
///
/// # Examples
///
/// ```
/// use dicetest::hint_debug;
///
/// let unknown_value = 42;
/// hint_debug!(unknown_value);
/// ```
#[macro_export]
macro_rules! hint_debug {
    ($arg:tt) => {
        $crate::hints::add(|| format!(concat!("{} = {:?}"), stringify!($arg), $arg));
    };
}

/// Creates a stat with the first argument as stat key and the remaining arguments applied to the
/// [`format`] macro as stat value.
///
/// # Examples
///
/// ```
/// use dicetest::stat;
///
/// let random_number = 4;
/// stat!("Is random number even?", "{}", random_number % 2 == 0);
/// ```
#[macro_export]
macro_rules! stat {
    ($key:tt, $($arg:tt)*) => ({
        $crate::stats::inc($key, || format!($($arg)*))
    })
}

/// Creates a stat with the stringified argument as stat key and the argument value converted with
/// [`Debug`] as stat value.
///
/// [`Debug`]: std::fmt::Debug
///
/// # Examples
///
/// ```
/// use dicetest::stat_debug;
///
/// let random_number = 4;
/// stat_debug!({ random_number % 2 == 0 });
/// ```
#[macro_export]
macro_rules! stat_debug {
    ($arg:tt) => {
        $crate::stats::inc(stringify!($arg), || format!("{:?}", $arg));
    };
}

#[cfg(test)]
mod tests {
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
}
