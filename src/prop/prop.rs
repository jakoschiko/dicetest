use ::rng::Rng;
use ::gen::Limit;
use ::prop::{Log, Eval, Sample};
use ::prop::adapters::PropBoxed;

/// Trait for implementing properties. A property represents a logic expression and can be evaluated
/// to an extended truth value.
///
/// An implementation is allowed to use randomness for its evaluation. The most important use case
/// for randomness is the evaluation of universal quantifiers with random arguments.
pub trait Prop {
    /// Consumes the property and evalutes it.
    ///
    /// The parameters `Rng` and `Limit` corresponds to parameters needed for using `GenOnce` and
    /// `Gen`. The `Rng` is the only source of the randomness. Besides that, the evaluation is
    /// derterministic.
    fn eval(self, &mut Log, &mut Rng, Limit) -> Eval;

    /// Wraps `self` into a `Box`.
    fn boxed(self) -> PropBoxed
    where
        Self: Sized + 'static,
    {
        PropBoxed::new(self)
    }

    /// Calls `Prop::eval` with random seed, default limit and enabled `Log`. Useful for debugging
    /// the property.
    fn sample(self) -> Sample
    where
        Self: Sized,
    {
        let mut log = Log::with_print_enabled();
        let mut rng = Rng::random();
        let lim = Limit::default();

        let eval = self.eval(&mut log, &mut rng, lim);
        let log_data = log.data();
        let prints = log_data.prints;

        Sample { eval, prints }
    }
}

impl<F> Prop for F
where
    F: FnOnce(&mut Log, &mut Rng, Limit) -> Eval,
{
    fn eval(self, log: &mut Log, rng: &mut Rng, lim: Limit) -> Eval {
        self(log, rng, lim)
    }
}

impl Prop for Eval {
    fn eval(self, log: &mut Log, _rng: &mut Rng, _lim: Limit) -> Eval {
        log.print(|| {
            match self {
                Eval::True => "True",
                Eval::Passed => "Passed",
                Eval::False => "False",
            }
        });

        self
    }
}

impl Prop for bool {
    fn eval(self, log: &mut Log, _rng: &mut Rng, _lim: Limit) -> Eval {
        log.print(|| {
            if self {
                "True from bool"
            } else {
                "False from bool"
            }
        });

        if self {
            Eval::True
        } else {
            Eval::False
        }
    }
}
