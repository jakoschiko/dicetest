use ::std::fmt::Debug;

use ::prop::{Eval, Prop};
use ::props;

/// This property holds iff both values are equal.
pub fn eq<A>(left: A, right: A) -> impl Prop
where
    A: Debug + PartialEq
{
    props::from_fn_once(move |_, _, log| {
        if left == right {
            log.print("Values are equal");
            Eval::True
        } else {
            log.print("Values are not equal:", );
            log.indent_print();
            log.print(|| format!(" Left: {:?}", left));
            log.print(|| format!("Right: {:?}", right));
            log.unindent_print();
            Eval::False
        }
    })
}
