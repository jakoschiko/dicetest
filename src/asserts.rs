//! A collection of useful assertions.

use std::fmt::Debug;

use crate::prelude::asserts::*;

/// Asserts that `g` is a left inverse for `f`.
pub fn left_inverse<X, Y>(
    fate: &mut Fate,
    gen_x: impl GenOnce<X>,
    f: impl FnOnce(X) -> Y,
    g: impl FnOnce(Y) -> X,
) where
    X: Debug + Clone + PartialEq,
{
    let x = gen_x.gen_once(fate);
    let y = f(x.clone());
    let other_x = g(y);

    assert_eq!(x, other_x)
}

/// Asserts that `h` is a right inverse for `f`.
pub fn right_inverse<X, Y>(
    fate: &mut Fate,
    gen_y: impl GenOnce<Y>,
    f: impl FnOnce(X) -> Y,
    h: impl FnOnce(Y) -> X,
) where
    Y: Debug + Clone + PartialEq,
{
    let y = gen_y.gen_once(fate);
    let x = h(y.clone());
    let other_y = f(x);

    assert_eq!(y, other_y)
}
