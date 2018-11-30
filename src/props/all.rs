use crate::prelude::props::*;

macro_rules! fn_all_n {
    ($all_n:ident: $($prop_i:ident)+) => (
        /// This property holds iff all given properties hold. It represents a logical conjunction.
        pub fn $all_n(
            $($prop_i: impl Prop,)*
        ) -> impl Prop {
            props::from_fn(|dice| {
                let mut index = 0;
                let mut acc = Eval::True;

                $(
                    index += 1;

                    let logger_enabled = logger::enabled();
                    if logger_enabled {
                        log!("all term {}:", index);
                        logger::indent();
                    }

                    acc = acc.and($prop_i.eval(dice));

                    if logger_enabled {
                        logger::unindent()
                    }

                    if acc == Eval::False {
                        return acc;
                    }
                )*

                acc
            })
        }
    )
}

fn_all_n! { all_2:
    prop_1 prop_2
}

fn_all_n! { all_3:
    prop_1 prop_2 prop_3
}

fn_all_n! { all_4:
    prop_1 prop_2 prop_3 prop_4
}

fn_all_n! { all_5:
    prop_1 prop_2 prop_3 prop_4 prop_5
}

fn_all_n! { all_6:
    prop_1 prop_2 prop_3 prop_4 prop_5 prop_6
}

fn_all_n! { all_7:
    prop_1 prop_2 prop_3 prop_4 prop_5 prop_6 prop_7
}

fn_all_n! { all_8:
    prop_1 prop_2 prop_3 prop_4 prop_5 prop_6 prop_7 prop_8
}

fn_all_n! { all_9:
    prop_1 prop_2 prop_3 prop_4 prop_5 prop_6 prop_7 prop_8 prop_9
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use crate::prelude::tests::*;
    use crate::prop::Eval;

    #[test]
    fn all_is_conjunction() {
        fn assert_input_output(term_1: Eval, term_2: Eval, eval_expected: Eval) {
            let eval = props::all_2(term_1, term_2).sample().eval;
            assert_eq!(eval, eval_expected);
        }

        assert_input_output(Eval::True, Eval::True, Eval::True);
        assert_input_output(Eval::True, Eval::False, Eval::False);
        assert_input_output(Eval::False, Eval::True, Eval::False);
        assert_input_output(Eval::False, Eval::False, Eval::False);
    }

    #[test]
    fn all_is_short_circuit() {
        let prop_2_was_evalutated = Rc::new(Cell::new(false));
        let prop_2 = props::from_fn(|_| {
            prop_2_was_evalutated.set(true);
            Eval::False
        });
        let _ = props::all_2(Eval::False, prop_2).sample();
        assert!(!prop_2_was_evalutated.get());
    }
}
