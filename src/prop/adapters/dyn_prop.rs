use crate::gen::Dice;
use crate::prop::{Eval, Prop};

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
    fn eval(mut self, dice: &mut Dice) -> Eval {
        self.dyn.eval(dice)
    }
}

trait Wrapper {
    fn eval(&mut self, dice: &mut Dice) -> Eval;
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
    fn eval(&mut self, dice: &mut Dice) -> Eval {
        let prop = self.prop.take().unwrap();
        prop.eval(dice)
    }
}
