use crate::log;
use crate::logger;
use crate::rng::Rng;
use crate::gen::{Limit, Dice};
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
    /// The `Dice` is the only source of the randomness. Besides that, the evaluation is
    /// derterministic.
    fn eval(self, dice: &mut Dice) -> Eval;

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
        let rng = &mut Rng::random();
        let lim = Limit::default();

        self.sample_with_dice(&mut Dice::new(rng, lim))
    }

    /// Calls `Prop::eval` with the given dice and enabled `logger`. Useful for
    /// debugging the property.
    fn sample_with_dice(self, dice: &mut Dice) -> Sample
    where
        Self: Sized,
    {
        let (eval, messages) = logger::collect_messages(|| {
            self.eval(dice)
        });

        Sample { eval, messages }
    }
}

impl<F> Prop for F
where
    F: FnOnce(&mut Dice) -> Eval,
{
    fn eval(self, dice: &mut Dice) -> Eval {
        self(dice)
    }
}

impl Prop for Eval {
    fn eval(self, _dice: &mut Dice) -> Eval {
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
    fn eval(self, _dice: &mut Dice) -> Eval {
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
