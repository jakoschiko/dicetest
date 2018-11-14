pub use crate::prelude::props::*;

/// This property holds if `assertion` is true.
pub fn assert(assertion: bool, message: &'static str) -> impl Prop {
    props::from_fn(move |_, _| {
        if assertion {
            log!("Assertion holds");
            Eval::True
        } else {
            if logger::enabled() {
                log!("Assertion does not hold:");
                logger::indent();
                log!("{}", message);
                logger::unindent();
            }
            Eval::False
        }
    })
}
