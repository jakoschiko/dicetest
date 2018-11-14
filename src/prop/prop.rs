use crate::log;
use crate::logger;
use crate::rng::Rng;
use crate::gen::Limit;
use crate::prop::{Eval, Sample};
use crate::prop::adapters::DynProp;

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
    fn eval(self, &mut Rng, Limit) -> Eval;

    /// Puts `self` behind a pointer.
    fn dyn<'a>(self) -> DynProp<'a>
    where
        Self: Sized + 'a,
    {
        DynProp::new(self)
    }

    /// Calls `Prop::eval` with a random seed, default limit and enabled `logger`. Useful for
    /// debugging the property.
    fn sample(self) -> Sample
    where
        Self: Sized,
    {
        let mut rng = Rng::random();
        let lim = Limit::default();

        self.sample_with_params(&mut rng, lim)
    }

    /// Calls `Prop::eval` with the given seed, the given limit and enabled `logger`. Useful for
    /// debugging the property.
    fn sample_with_params(self, rng: &mut Rng, lim: Limit) -> Sample
    where
        Self: Sized,
    {
        let (eval, messages) = logger::collect_messages(|| {
            self.eval(rng, lim)
        });

        Sample { eval, messages }
    }
}

impl<F> Prop for F
where
    F: FnOnce(&mut Rng, Limit) -> Eval,
{
    fn eval(self, rng: &mut Rng, lim: Limit) -> Eval {
        self(rng, lim)
    }
}

impl Prop for Eval {
    fn eval(self, _rng: &mut Rng, _lim: Limit) -> Eval {
        log!("{}", {
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
    fn eval(self, _rng: &mut Rng, _lim: Limit) -> Eval {
        log!("{}", {
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
