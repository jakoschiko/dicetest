use crate::prelude::dice::*;

/// Generates a `None` or a `Some` that contains a value from the given generator. `None` and `Some`
/// have the same probalility.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let foo_or_bar_die = dice::one_of_2_once("foo", "bar");
/// let optional_foo_or_bar_die = dice::option_once(foo_or_bar_die);
///
/// let foo_or_bar_or_none = optional_foo_or_bar_die.sample_once();
/// ```
pub fn option_once<T>(some_die: impl DieOnce<T>) -> impl DieOnce<Option<T>> {
    dice::one_of_die_2_once(dice::just_once(None), some_die.map_once(Some))
}

/// Generates a `None` or a `Some` that contains a value from the given generator. `None` and `Some`
/// have the same probalility.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let byte_die = dice::u8(..);
/// let optional_byte_die = dice::option(byte_die);
///
/// let byte_or_none = optional_byte_die.sample();
/// ```
pub fn option<T>(some_die: impl Die<T>) -> impl Die<Option<T>> {
    dice::one_of_die_2(dice::from_fn(|_| None), some_die.map(Some))
}

/// Generates a `None` or a `Some` that contains a value from the given generator. The probabilities
/// of `None` and `Some` depend on the given weights.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let foo_or_bar_die = dice::one_of_2_once("foo", "bar");
/// let optional_foo_or_bar_die = dice::weighted_option_once(10, (1, foo_or_bar_die));
///
/// let probably_none = optional_foo_or_bar_die.sample_once();
/// ```
pub fn weighted_option_once<T>(
    none_weight: u32,
    (some_weight, some_die): (u32, impl DieOnce<T>),
) -> impl DieOnce<Option<T>> {
    dice::weighted_one_of_die_2_once(
        (none_weight, dice::just_once(None)),
        (some_weight, some_die.map_once(Some)),
    )
}

/// Generates a `None` or a `Some` that contains a value from the given generator. The probabilities
/// of `None` and `Some` depend on the given weights.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let byte_die = dice::u8(..);
/// let optional_byte_die = dice::weighted_option(10, (1, byte_die));
///
/// let probably_none = optional_byte_die.sample();
/// ```
pub fn weighted_option<T>(
    none_weight: u32,
    (some_weight, some_die): (u32, impl Die<T>),
) -> impl Die<Option<T>> {
    dice::weighted_one_of_die_2(
        (none_weight, dice::from_fn(|_| None)),
        (some_weight, some_die.map(Some)),
    )
}
