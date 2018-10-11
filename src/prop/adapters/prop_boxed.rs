use ::rng::Rng;
use ::gen::Limit;
use ::prop::{Log, Eval, Prop};

/// Adapter for `Prop::boxed`.
pub struct PropBoxed {
    boxed: Box<dyn Wrapper>,
}

impl PropBoxed {
    pub fn new<P>(prop: P) -> Self
    where
        P: Prop + 'static,
    {
        let wrapper = PropWrapper {
            prop: Some(prop)
        };
        let boxed = Box::new(wrapper);
        PropBoxed { boxed }
    }
}

impl Prop for PropBoxed {
    fn eval(mut self, log: &mut Log, rng: &mut Rng, lim: Limit) -> Eval {
        self.boxed.eval(log, rng, lim)
    }

    fn boxed(self) -> PropBoxed
    where
        Self: Sized + 'static,
    {
        self
    }
}

trait Wrapper {
    fn eval(&mut self, log: &mut Log, &mut Rng, Limit) -> Eval;
}

struct PropWrapper<P>
where
    P: Prop,
{
    prop: Option<P>,
}

impl<P> Wrapper for PropWrapper<P>
where
    P: Prop,
{
    fn eval(&mut self, log: &mut Log, rng: &mut Rng, lim: Limit) -> Eval {
        let prop = self.prop.take().expect("PropWrapper::eval should not be called twice");
        prop.eval(log, rng, lim)
    }
}
