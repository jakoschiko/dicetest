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
    ($($arg:tt)*) => {
        $crate::hints::add(|| format!($($arg)*));
    }
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
        $crate::hints::add(|| format!("{} = {:?}", stringify!($arg), $arg));
    };
}

/// Indents all hints in the caller's code block after this macro is called.
///
/// If arguments are specified, a (not indented) hint will be added with the arguments applied
/// to the [`format`] macro. This hint is meant as the title of the section.
///
/// # Examples
///
/// ```
/// use dicetest::{hint, hint_section};
///
/// hint!("Start test"); // This hint is not indented
///
/// {
///     hint_section!("Test foo"); // This hint is not indented
///     hint!("foo"); // This hint is indented
/// }
///
/// {
///     hint_section!("Test bar"); // This hint is not indented
///     hint!("bar"); // This hint is indented
///     
///     hint_section!(); // No hint
///     hint!("bar"); // This hint is indented twice
/// }
///
/// hint!("Test finished"); // This hint is not indented
/// ```
#[macro_export]
macro_rules! hint_section {
    () => {
        let _block_ident = $crate::hints::Section::start();
    };
    ($($arg:tt)*) => {
        $crate::hints::add(|| format!($($arg)*));
        let _block_ident = $crate::hints::Section::start();
    }
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
    ($key:tt, $($arg:tt)*) => {
        $crate::stats::inc($key, || format!($($arg)*))
    }
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
        $crate::stats::inc(stringify!($arg), || format!("{:?}", $arg))
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
    fn macro_hint_section_produces_valid_code() {
        if false {
            hint_section!();
            hint_section!("foo");
            hint_section!("bar {}", 42);
        }
    }

    #[test]
    #[cfg(feature = "hints")]
    fn macro_hint_section_produces_correct_indent() {
        let (_, actual_hints) = crate::hints::collect(|| {
            {
                hint!("foo1");
                hint_section!();
                hint!("foo2");
                hint_section!("bar");
                hint!("foo3");
            };
            hint!("foo4");
        });
        let expected_hints = crate::hints::Hints(vec![
            crate::hints::Hint {
                indent: 0,
                text: "foo1".to_owned(),
            },
            crate::hints::Hint {
                indent: 1,
                text: "foo2".to_owned(),
            },
            crate::hints::Hint {
                indent: 1,
                text: "bar".to_owned(),
            },
            crate::hints::Hint {
                indent: 2,
                text: "foo3".to_owned(),
            },
            crate::hints::Hint {
                indent: 0,
                text: "foo4".to_owned(),
            },
        ]);
        assert_eq!(expected_hints, actual_hints)
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
