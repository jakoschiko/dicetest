use ::rng::Rng;
use ::prop::{IntoLabel, Params, Result, Prop};

/// Default implementation for `Prop::label`.
pub struct PropLabel<P, L>
where
    P: Prop,
    L: IntoLabel,
{
    prop: P,
    label: L,
}

impl<P, L> PropLabel<P, L>
where
    P: Prop,
    L: IntoLabel,
{
    pub fn new(prop: P, label: L) -> Self {
        PropLabel { prop, label }
    }
}

impl<P, L> Prop for PropLabel<P, L>
where
    P: Prop,
    L: IntoLabel,
{
    fn eval(self, rng: &mut Rng, params: &Params) -> Result {
        let prop = self.prop;
        let label = self.label;

        let mut result = prop.eval(rng, params);

        if params.create_labels {
            result.labels.push(label);
        }

        result
    }
}
