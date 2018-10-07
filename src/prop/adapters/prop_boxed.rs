use ::rng::Rng;
use ::gen::Size;
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
    fn eval(mut self, rng: &mut Rng, size: Size, log: &mut Log) -> Eval {
        self.boxed.eval(rng, size, log)
    }

    fn boxed(self) -> PropBoxed
    where
        Self: Sized + 'static,
    {
        self
    }
}

trait Wrapper {
    fn eval(&mut self, &mut Rng, Size, log: &mut Log) -> Eval;
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
    fn eval(&mut self, rng: &mut Rng, size: Size, log: &mut Log) -> Eval {
        let prop = self.prop.take().expect("PropWrapper::eval should not be called twice");
        prop.eval(rng, size, log)
    }
}
