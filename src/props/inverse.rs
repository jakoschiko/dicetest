use std::fmt::Debug;

use ::prelude::props::*;

/// Tests if `g` is a left inverse for `f`.
pub fn left_inverse<X, Y>(
    gen_x: impl GenOnce<X>,
    f: impl FnOnce(X) -> Y,
    g: impl FnOnce(Y) -> X,
) -> impl Prop
where
    X: Debug + Clone + PartialEq,
{
    props::forall_1(
        gen_x.name("x"),
        move |log, x| {
            let y = f(x.clone());
            let other_x = g(y);
            log.print(|| format!("g(f(x)): {:?}", other_x));
            props::equal(x, other_x)
        }
    )
}

/// Tests if `h` is a right inverse for `f`.
pub fn right_inverse<X, Y>(
    gen_y: impl GenOnce<Y>,
    f: impl FnOnce(X) -> Y,
    h: impl FnOnce(Y) -> X,
) -> impl Prop
where
    Y: Debug + Clone + PartialEq,
{
    props::forall_1(
        gen_y.name("y"),
        move |log, y| {
            let x = h(y.clone());
            let other_y = f(x);
            log.print(|| format!("f(g(y)): {:?}", other_y));
            props::equal(y, other_y)
        }
    )
}
