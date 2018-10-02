pub use ::prop::{LazyString, Eval, Prop};
pub use ::props;

/// This property holds if `assertion` is true.
pub fn assert(assertion: bool, message: impl LazyString) -> impl Prop {
    props::from_fn_once(move |_, _, log| {
        if assertion {
            log.print("Assertion holds");
            Eval::True
        } else {
            log.print("Assertion does not hold:");
            log.indent_print();
            log.print(message);
            log.unindent_print();
            Eval::False
        }
    })
}
