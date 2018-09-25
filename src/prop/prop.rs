use ::rng::Rng;
use ::prop::{IntoLabel, Params, Status, Result};
use ::prop::adapters::{PropLabel, PropBoxed};

/// Trait for implementing properties. A property represents a logic expression and can be evaluated
/// to an extended truth value.
///
/// An implementation is allowed to use randomness for its evaluation. The most important use case
/// for randomness is the approximatively evaluation of universal quantifiers.
pub trait Prop {
    /// Consumes the property and evalutes it using the given parameters.
    ///
    /// The `Rng` is the only source of the randomness. Besides that, the generation is
    /// derterministic.
    fn eval(self, &mut Rng, &Params) -> Result;

    /// Converts this property into another property, that appends the given label to the result.
    /// Besides that the evalation is identical. The label will be evaluated lazily.
    fn label<L>(self, label: L) -> PropLabel<Self, L>
    where
        Self: Sized,
        L: IntoLabel,
    {
        PropLabel::new(self, label)
    }

    /// Wraps `self` into a `Box`.
    fn boxed(self) -> PropBoxed
    where
        Self: Sized + 'static,
    {
        PropBoxed::new(self)
    }

    /// Calls `Prop::eval` with random seed and default parameters. Useful for debugging the
    /// property.
    fn sample(self) -> Result
    where
        Self: Sized,
    {
        let mut rng = Rng::random();
        let params = Params::default();

        self.eval(&mut rng, &params)
    }
}

impl<F> Prop for F
where
    F: FnOnce(&mut Rng, &Params) -> Result,
{
    fn eval(self, rng: &mut Rng, params: &Params) -> Result {
        self(rng, params)
    }
}

impl Prop for Status {
    fn eval(self, _rng: &mut Rng, params: &Params) -> Result {
        let mut result = Result::new(self);

        if params.create_labels {
            let label = match self {
                Status::True => "True",
                Status::Passed => "Passed",
                Status::False => "False",
            };
            result.labels.push(label);
        }

        result
    }
}

impl Prop for bool {
    fn eval(self, _rng: &mut Rng, params: &Params) -> Result {
        let status = if self {
            Status::True
        } else {
            Status::False
        };

        let mut result = Result::new(status);

        if params.create_labels {
            let label = if self {
                "True from bool"
            } else {
                "False from bool"
            };
            result.labels.push(label);
        }

        result
    }
}
