use ::rng::Rng;
use ::gen::Size;
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
    /// The parameters `Rng` and `Size` corresponds to parameters needed for using `GenOnce` and
    /// `Gen`. The `Rng` is the only source of the randomness. Besides that, the evaluation is
    /// derterministic.
    fn eval(self, &mut Rng, Size, &mut Log) -> Eval;

    /// Wraps `self` into a `Box`.
    fn boxed(self) -> PropBoxed
    where
        Self: Sized + 'static,
    {
        PropBoxed::new(self)
    }

    /// Calls `Prop::eval` with random seed, default size and enabled `Log`. Useful for debugging
    /// the property.
    fn sample(self) -> Sample
    where
        Self: Sized,
    {
        let mut rng = Rng::random();
        let size = Size::default();
        let mut log = Log::with_print_enabled();

        let eval = self.eval(&mut rng, size, &mut log);
        let log_data = log.data();
        let prints = log_data.prints;

        Sample { eval, prints }
    }
}

impl<F> Prop for F
where
    F: FnOnce(&mut Rng, Size, &mut Log) -> Eval,
{
    fn eval(self, rng: &mut Rng, size: Size, log: &mut Log) -> Eval {
        self(rng, size, log)
    }
}

impl Prop for Eval {
    fn eval(self, _rng: &mut Rng, _size: Size, log: &mut Log) -> Eval {
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
    fn eval(self, _rng: &mut Rng, _size: Size, log: &mut Log) -> Eval {
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
