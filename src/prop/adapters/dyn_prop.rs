use ::rng::Rng;
use ::gen::Limit;
use ::prop::{Log, Eval, Prop};

/// Adapter for `Prop::dyn`.
pub struct DynProp<'a> {
    dyn: Box<dyn Wrapper + 'a>,
}

impl<'a> DynProp<'a> {
    pub fn new<P>(prop: P) -> Self
    where
        P: Prop + 'a,
    {
        let wrapper = PropWrapper {
            prop: Some(prop)
        };
        let dyn = Box::new(wrapper);
        DynProp { dyn }
    }
}

impl<'a> Prop for DynProp<'a> {
    fn eval(mut self, log: &mut Log, rng: &mut Rng, lim: Limit) -> Eval {
        self.dyn.eval(log, rng, lim)
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
