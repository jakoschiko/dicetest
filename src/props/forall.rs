use ::rng::Rng;
use ::gen::{Size, GenOnce};
use ::prop::{Label, IntoLabel, Log, Eval, Prop, Show, IntoArg};
use ::props;

macro_rules! fn_forall_n {
    (
        $forall_n:ident:
        $($Ti:ident, $Gi:ident, $Li:ident, $Si:ident, $Ai:ident, $arg_i:ident, $value_i:ident)+
    ) => (
        /// Implements an universal quantifier.
        ///
        /// This property will be evaluated as follows:
        ///     * Random values will be generated using the given `Arg`s
        ///     * The generated values will be passed to the given function for creating the body
        ///     * The body will be evaluated
        ///     * If the body evaluates to `Status::False`, this property evaluates to
        ///     `Status::False`
        ///     * Else this property evalutes to `Status::Passed`
        ///
        /// In general universal quantifier cannot be proven in finite time. Hence this property
        /// never evaluates to `Status::True`.
        pub fn $forall_n<$($Ti,)* $($Gi,)* $($Li,)* $($Si,)* $($Ai,)* P, F>(
            $($arg_i: $Ai,)*
            f: F,
        ) -> impl Prop
        where
            $($Gi: GenOnce<$Ti>,)*
            $($Li: IntoLabel,)*
            $($Si: Show<$Ti>,)*
            $($Ai: IntoArg<$Ti, $Gi, $Li, $Si>,)*
            P: Prop,
            F: FnOnce($($Ti,)*) -> P,
        {
            props::from_fn_once(move |rng, size, log| {
                $(let $arg_i = $arg_i.into_arg();)*
                $(let $value_i = $arg_i.gen.gen_once(rng, size);)*

                let mut arg_infos = Vec::new();

                if log.print_enabled() {
                    let mut index = 0;

                    $({
                        index += 1;
                        let arg_info = arg_info(index, $arg_i.name_opt, &$value_i, $arg_i.show);
                        arg_infos.push(arg_info)
                    })*
                }

                let body = f($($value_i,)*);

                eval_body(rng, size, log, body, arg_infos)
            })
        }
    )
}

fn_forall_n! { forall_1:
    T1, G1, L1, S1, A1, arg_1, value_1
}

fn_forall_n! { forall_2:
    T1, G1, L1, S1, A1, arg_1, value_1
    T2, G2, L2, S2, A2, arg_2, value_2
}

fn_forall_n! { forall_3:
    T1, G1, L1, S1, A1, arg_1, value_1
    T2, G2, L2, S2, A2, arg_2, value_2
    T3, G3, L3, S3, A3, arg_3, value_3
}

fn_forall_n! { forall_4:
    T1, G1, L1, S1, A1, arg_1, value_1
    T2, G2, L2, S2, A2, arg_2, value_2
    T3, G3, L3, S3, A3, arg_3, value_3
    T4, G4, L4, S4, A4, arg_4, value_4
}

fn_forall_n! { forall_5:
    T1, G1, L1, S1, A1, arg_1, value_1
    T2, G2, L2, S2, A2, arg_2, value_2
    T3, G3, L3, S3, A3, arg_3, value_3
    T4, G4, L4, S4, A4, arg_4, value_4
    T5, G5, L5, S5, A5, arg_5, value_5
}

fn_forall_n! { forall_6:
    T1, G1, L1, S1, A1, arg_1, value_1
    T2, G2, L2, S2, A2, arg_2, value_2
    T3, G3, L3, S3, A3, arg_3, value_3
    T4, G4, L4, S4, A4, arg_4, value_4
    T5, G5, L5, S5, A5, arg_5, value_5
    T6, G6, L6, S6, A6, arg_6, value_6
}

fn_forall_n! { forall_7:
    T1, G1, L1, S1, A1, arg_1, value_1
    T2, G2, L2, S2, A2, arg_2, value_2
    T3, G3, L3, S3, A3, arg_3, value_3
    T4, G4, L4, S4, A4, arg_4, value_4
    T5, G5, L5, S5, A5, arg_5, value_5
    T6, G6, L6, S6, A6, arg_6, value_6
    T7, G7, L7, S7, A7, arg_7, value_7
}

fn_forall_n! { forall_8:
    T1, G1, L1, S1, A1, arg_1, value_1
    T2, G2, L2, S2, A2, arg_2, value_2
    T3, G3, L3, S3, A3, arg_3, value_3
    T4, G4, L4, S4, A4, arg_4, value_4
    T5, G5, L5, S5, A5, arg_5, value_5
    T6, G6, L6, S6, A6, arg_6, value_6
    T7, G7, L7, S7, A7, arg_7, value_7
    T8, G8, L8, S8, A8, arg_8, value_8
}

fn_forall_n! { forall_9:
    T1, G1, L1, S1, A1, arg_1, value_1
    T2, G2, L2, S2, A2, arg_2, value_2
    T3, G3, L3, S3, A3, arg_3, value_3
    T4, G4, L4, S4, A4, arg_4, value_4
    T5, G5, L5, S5, A5, arg_5, value_5
    T6, G6, L6, S6, A6, arg_6, value_6
    T7, G7, L7, S7, A7, arg_7, value_7
    T8, G8, L8, S8, A8, arg_8, value_8
    T9, G9, L9, S9, A9, arg_9, value_9
}

fn arg_info<T, L, S>(index: u32, name_opt: Option<L>, value: &T, show: S) -> Label
where
    L: IntoLabel,
    S: Show<T>,
{
    let name_string = match name_opt {
        None => String::new(),
        Some(name) => {
            let name  = name.into_label().text;
            format!("{}: ", name)
        }
    };

    let value_string = show.show(&value);

    let arg_string = format!("{}.) {}{}", index, name_string, value_string);

    arg_string.into_label()
}

fn eval_body(
    rng: &mut Rng,
    size: Size,
    log: &mut Log,
    body: impl Prop,
    arg_infos: Vec<Label>,
) -> Eval {
    if log.print_enabled() {
        log.print("forall args:");
        log.indent_print();
        for arg_info in arg_infos.into_iter() {
            log.print(arg_info);
        }
        log.unindent_print();
        log.print("forall labels:");
        log.indent_print();
    }

    let eval = body.eval(rng, size, log);

    log.unindent_print();

    match eval {
        Eval::True => Eval::Passed,
        Eval::Passed => Eval::Passed,
        Eval::False => Eval::False,
    }
}
