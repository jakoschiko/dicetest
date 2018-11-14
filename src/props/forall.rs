use crate::prelude::props::*;
use crate::prop::{Show, IntoArg};

macro_rules! fn_forall_n {
    (
        $forall_n:ident:
        $($Ti:ident, $Gi:ident, $Si:ident, $Ai:ident, $arg_i:ident, $value_i:ident)+
    ) => (
        /// A property that represents an universal quantifier.
        ///
        /// This property will be evaluated as follows:
        ///     * Random values will be generated using the given `Arg`s
        ///     * The generated values will be passed to the given predicate for creating the
        ///     predicate property.
        ///     * The predicate property will be evaluated
        ///     * If the predicate property evaluates to `Status::False`, this property evaluates to
        ///     `Status::False`
        ///     * Else this property evalutes to `Status::Passed`
        ///
        /// In general universal quantifier cannot be proven in finite time. Hence this property
        /// never evaluates to `Status::True`.
        pub fn $forall_n<$($Ti,)* $($Gi,)* $($Si,)* $($Ai,)* P, F>(
            $($arg_i: $Ai,)*
            predicate: F,
        ) -> impl Prop
        where
            $($Gi: GenOnce<$Ti>,)*
            $($Si: Show<$Ti>,)*
            $($Ai: IntoArg<$Ti, $Gi, $Si>,)*
            P: Prop,
            F: FnOnce($($Ti,)*) -> P,
        {
            props::from_fn(move |rng, lim| {
                $(let $arg_i = $arg_i.into_arg();)*
                $(let $value_i = $arg_i.gen.gen_once(rng, lim);)*

                let mut arg_infos = Vec::new();

                let logger_enabled = logger::enabled();

                if logger_enabled {
                    let mut index = 0;

                    $({
                        index += 1;
                        let arg_info = arg_info(index, $arg_i.name_opt, &$value_i, $arg_i.show);
                        arg_infos.push(arg_info)
                    })*
                }

                eval_predicate(
                    rng,
                    lim,
                    predicate($($value_i,)*),
                    logger_enabled,
                    arg_infos,
                )
            })
        }
    )
}

fn_forall_n! { forall_1:
    T1, G1, S1, A1, arg_1, value_1
}

fn_forall_n! { forall_2:
    T1, G1, S1, A1, arg_1, value_1
    T2, G2, S2, A2, arg_2, value_2
}

fn_forall_n! { forall_3:
    T1, G1, S1, A1, arg_1, value_1
    T2, G2, S2, A2, arg_2, value_2
    T3, G3, S3, A3, arg_3, value_3
}

fn_forall_n! { forall_4:
    T1, G1, S1, A1, arg_1, value_1
    T2, G2, S2, A2, arg_2, value_2
    T3, G3, S3, A3, arg_3, value_3
    T4, G4, S4, A4, arg_4, value_4
}

fn_forall_n! { forall_5:
    T1, G1, S1, A1, arg_1, value_1
    T2, G2, S2, A2, arg_2, value_2
    T3, G3, S3, A3, arg_3, value_3
    T4, G4, S4, A4, arg_4, value_4
    T5, G5, S5, A5, arg_5, value_5
}

fn_forall_n! { forall_6:
    T1, G1, S1, A1, arg_1, value_1
    T2, G2, S2, A2, arg_2, value_2
    T3, G3, S3, A3, arg_3, value_3
    T4, G4, S4, A4, arg_4, value_4
    T5, G5, S5, A5, arg_5, value_5
    T6, G6, S6, A6, arg_6, value_6
}

fn_forall_n! { forall_7:
    T1, G1, S1, A1, arg_1, value_1
    T2, G2, S2, A2, arg_2, value_2
    T3, G3, S3, A3, arg_3, value_3
    T4, G4, S4, A4, arg_4, value_4
    T5, G5, S5, A5, arg_5, value_5
    T6, G6, S6, A6, arg_6, value_6
    T7, G7, S7, A7, arg_7, value_7
}

fn_forall_n! { forall_8:
    T1, G1, S1, A1, arg_1, value_1
    T2, G2, S2, A2, arg_2, value_2
    T3, G3, S3, A3, arg_3, value_3
    T4, G4, S4, A4, arg_4, value_4
    T5, G5, S5, A5, arg_5, value_5
    T6, G6, S6, A6, arg_6, value_6
    T7, G7, S7, A7, arg_7, value_7
    T8, G8, S8, A8, arg_8, value_8
}

fn_forall_n! { forall_9:
    T1, G1, S1, A1, arg_1, value_1
    T2, G2, S2, A2, arg_2, value_2
    T3, G3, S3, A3, arg_3, value_3
    T4, G4, S4, A4, arg_4, value_4
    T5, G5, S5, A5, arg_5, value_5
    T6, G6, S6, A6, arg_6, value_6
    T7, G7, S7, A7, arg_7, value_7
    T8, G8, S8, A8, arg_8, value_8
    T9, G9, S9, A9, arg_9, value_9
}

fn arg_info<T, S>(index: u32, name_opt: Option<&'static str>, value: &T, show: S) -> String
where
    S: Show<T>,
{
    let name_string = match name_opt {
        None => String::new(),
        Some(name) => {
            format!("{}: ", name)
        }
    };

    let value_string = show.show(&value);

    format!("{}.) {}{}", index, name_string, value_string)
}

fn eval_predicate(
    rng: &mut Rng,
    lim: Limit,
    predicate: impl Prop,
    logger_enabled: bool,
    arg_infos: Vec<String>,
) -> Eval {
    if logger_enabled {
        log!("forall args:");
        logger::indent();
        for arg_info in arg_infos.into_iter() {
            log!("{}", arg_info);
        }
        logger::unindent();
        log!("forall predicate:");
        logger::indent();
    }

    let eval = predicate.eval(rng, lim);

    if logger_enabled {
        logger::unindent();
    }

    match eval {
        Eval::True => Eval::Passed,
        Eval::Passed => Eval::Passed,
        Eval::False => Eval::False,
    }
}
