use ::rng::Rng;
use ::prop::{Params, Result, Prop};

/// Default implementation for `Prop::boxed`.
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
    fn eval(mut self, rng: &mut Rng, params: &Params) -> Result {
        self.boxed.eval(rng, params)
    }

    fn boxed(self) -> PropBoxed
    where
        Self: Sized + 'static,
    {
        self
    }
}

trait Wrapper {
    fn eval(&mut self, &mut Rng, &Params) -> Result;
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
    fn eval(&mut self, rng: &mut Rng, params: &Params) -> Result {
        let prop = self.prop.take().expect("PropWrapper::eval should not be called twice");
        prop.eval(rng, params)
    }
}
