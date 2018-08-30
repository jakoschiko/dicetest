use ::rng::Rng;
use ::gen::GenOnce;
use ::prop::{Show, Label, IntoLabel, Arg, Params, Status, Result, Prop};
use ::props;

macro_rules! fn_forall_n {
    (
        $forall_n:ident:
        $($Ti:ident, $Gi:ident, $Li:ident, $Si:ident, $arg_i:ident, $value_i:ident)+
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
        pub fn $forall_n<$($Ti,)* $($Gi,)* $($Li,)* $($Si,)* P, F>(
            $($arg_i: Arg<$Ti, $Gi, $Li, $Si>,)*
            f: F,
        ) -> impl Prop
        where
            $($Gi: GenOnce<$Ti>,)*
            $($Li: IntoLabel,)*
            $($Si: Show<$Ti>,)*
            P: Prop,
            F: FnOnce($($Ti,)*) -> P,
        {
            props::from_fn_once(move |rng, params| {
                $(let $value_i = $arg_i.gen.gen_once(rng, &params.gen_params);)*

                let mut arg_labels = Vec::new();

                if params.create_labels {
                    let mut index = 0;

                    $({
                        index += 1;
                        let arg_label = arg_label(index, &$value_i, $arg_i.label_opt, $arg_i.show);
                        arg_labels.push(arg_label)
                    })*
                }

                let body = f($($value_i,)*);

                eval_body(rng, params, body, arg_labels)
            })
        }
    )
}

fn_forall_n! { forall_1:
    T1, G1, L1, S1, arg_1, value1
}

fn_forall_n! { forall_2:
    T1, G1, L1, S1, arg_1, value_1
    T2, G2, L2, S2, arg_2, value_2
}

fn_forall_n! { forall_3:
    T1, G1, L1, S1, arg_1, value_1
    T2, G2, L2, S2, arg_2, value_2
    T3, G3, L3, S3, arg_3, value_3
}

fn_forall_n! { forall_4:
    T1, G1, L1, S1, arg_1, value_1
    T2, G2, L2, S2, arg_2, value_2
    T3, G3, L3, S3, arg_3, value_3
    T4, G4, L4, S4, arg_4, value_4
}

fn_forall_n! { forall_5:
    T1, G1, L1, S1, arg_1, value_1
    T2, G2, L2, S2, arg_2, value_2
    T3, G3, L3, S3, arg_3, value_3
    T4, G4, L4, S4, arg_4, value_4
    T5, G5, L5, S5, arg_5, value_5
}

fn_forall_n! { forall_6:
    T1, G1, L1, S1, arg_1, value_1
    T2, G2, L2, S2, arg_2, value_2
    T3, G3, L3, S3, arg_3, value_3
    T4, G4, L4, S4, arg_4, value_4
    T5, G5, L5, S5, arg_5, value_5
    T6, G6, L6, S6, arg_6, value_6
}

fn_forall_n! { forall_7:
    T1, G1, L1, S1, arg_1, value_1
    T2, G2, L2, S2, arg_2, value_2
    T3, G3, L3, S3, arg_3, value_3
    T4, G4, L4, S4, arg_4, value_4
    T5, G5, L5, S5, arg_5, value_5
    T6, G6, L6, S6, arg_6, value_6
    T7, G7, L7, S7, arg_7, value_7
}

fn_forall_n! { forall_8:
    T1, G1, L1, S1, arg_1, value_1
    T2, G2, L2, S2, arg_2, value_2
    T3, G3, L3, S3, arg_3, value_3
    T4, G4, L4, S4, arg_4, value_4
    T5, G5, L5, S5, arg_5, value_5
    T6, G6, L6, S6, arg_6, value_6
    T7, G7, L7, S7, arg_7, value_7
    T8, G8, L8, S8, arg_8, value_8
}

fn_forall_n! { forall_9:
    T1, G1, L1, S1, arg_1, value_1
    T2, G2, L2, S2, arg_2, value_2
    T3, G3, L3, S3, arg_3, value_3
    T4, G4, L4, S4, arg_4, value_4
    T5, G5, L5, S5, arg_5, value_5
    T6, G6, L6, S6, arg_6, value_6
    T7, G7, L7, S7, arg_7, value_7
    T8, G8, L8, S8, arg_8, value_8
    T9, G9, L9, S9, arg_9, value_9
}

fn arg_label<T, L, S>(index: u32, value: &T, label_opt: Option<L>, show: S) -> Label
where
    L: IntoLabel,
    S: Show<T>,
{
    let label_string = match label_opt {
        None => String::new(),
        Some(label) => {
            let text  = label.into_label().text;
            format!(", {}", text)
        }
    };

    let value_string = show.show(&value);

    let arg_string = format!("arg_{}{}: {}", index, label_string, value_string);

    arg_string.into_label()
}

fn eval_body(
    rng: &mut Rng,
    params: &Params,
    body: impl Prop,
    arg_labels: Vec<Label>
) -> Result {
    let result = body.eval(rng, params);

    let forall_status = match result.status {
        Status::True => Status::Passed,
        status => status,
    };

    let mut forall_result = Result::new(forall_status);

    if params.create_labels {
        forall_result.append_label("forall args:");
        forall_result.append_labels(arg_labels);
        forall_result.append_label("forall labels:");
        forall_result.append_labels_indented(result.labels);
    }

    forall_result
}
