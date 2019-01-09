use crate::prelude::dice::*;

/// Generates the given value.
pub fn just_once<T>(value: T) -> impl DieOnce<T> {
    dice::from_fn_once(|_| value)
}

/// Generates a clone of the given value.
pub fn just<T>(value: T) -> impl Die<T>
where
    T: Clone,
{
    dice::from_fn(move |_| value.clone())
}
