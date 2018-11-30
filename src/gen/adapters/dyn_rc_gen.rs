use std::rc::Rc;

use crate::gen::{Dice, GenOnce, Gen};

/// Adapter for `GenOnce::dyn_rc`.
#[derive(Clone)]
pub struct DynRcGen<'a, T> {
    dyn: Rc<dyn Gen<T> + 'a>,
}

impl<'a, T> DynRcGen<'a, T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'a,
    {
        let dyn = Rc::new(gen);
        DynRcGen { dyn }
    }
}

impl<'a, T> Gen<T> for DynRcGen<'a, T> {
    fn gen(&self, dice: &mut Dice) -> T {
        self.dyn.gen(dice)
    }
}

impl<'a, T> GenOnce<T> for DynRcGen<'a, T> {
    fn gen_once(self, dice: &mut Dice) -> T {
        self.gen(dice)
    }
}
