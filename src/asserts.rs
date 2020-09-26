//! A collection of useful assertions.

use std::fmt::Debug;

use crate::prelude::*;

/// Asserts that `g` is a left inverse for `f`.
pub fn left_inverse<X, Y>(
    mut fate: Fate,
    x_die: impl DieOnce<X>,
    f: impl FnOnce(X) -> Y,
    g: impl FnOnce(Y) -> X,
) where
    X: Debug + Clone + PartialEq,
{
    let x = fate.roll(x_die);
    let y = f(x.clone());
    let other_x = g(y);

    assert_eq!(x, other_x)
}

/// Asserts that `h` is a right inverse for `f`.
pub fn right_inverse<X, Y>(
    mut fate: Fate,
    y_die: impl DieOnce<Y>,
    f: impl FnOnce(X) -> Y,
    h: impl FnOnce(Y) -> X,
) where
    Y: Debug + Clone + PartialEq,
{
    let y = fate.roll(y_die);
    let x = h(y.clone());
    let other_y = f(x);

    assert_eq!(y, other_y)
}
