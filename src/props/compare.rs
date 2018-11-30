use crate::std::fmt::Debug;

use crate::prelude::props::*;

/// This property holds iff `left == right` is true.
pub fn equal<A>(left: A, right: A) -> impl Prop
where
    A: Debug + PartialEq
{
   compare(left, right, |l, r| l == r, "==")
}

/// This property holds iff `left != right` is true.
pub fn not_equal<A>(left: A, right: A) -> impl Prop
where
    A: Debug + PartialEq
{
   compare(left, right, |l, r| l != r, "!=")
}

/// This property holds iff `left < right` is true.
pub fn less<A>(left: A, right: A) -> impl Prop
where
    A: Debug + PartialOrd
{
   compare(left, right, |l, r| l < r, "<")
}

/// This property holds iff `left <= right` is true.
pub fn less_than<A>(left: A, right: A) -> impl Prop
where
    A: Debug + PartialOrd
{
   compare(left, right, |l, r| l <= r, "<=")
}


/// This property holds iff `left > right` is true.
pub fn greater<A>(left: A, right: A) -> impl Prop
where
    A: Debug + PartialOrd
{
   compare(left, right, |l, r| l > r, ">")
}

/// This property holds iff `left >= right` is true.
pub fn greater_than<A>(left: A, right: A) -> impl Prop
where
    A: Debug + PartialOrd
{
   compare(left, right, |l, r| l >= r, ">=")
}

fn compare<A>(
    left: A,
    right: A,
    compare_op: impl FnOnce(&A, &A) -> bool,
    compare_str: &'static str,
) -> impl Prop
where
    A: Debug
{
    props::from_fn(move |_| {
        let expectation = compare_op(&left, &right);

        if logger::enabled() {
          log!("Assertion `left {} right` is {}", compare_str, expectation);
          logger::indent();
          log!(" left: {:?}", left);
          log!("right: {:?}", right);
          logger::unindent();
        }

        if expectation { Eval::True } else { Eval::False }
    })
}
