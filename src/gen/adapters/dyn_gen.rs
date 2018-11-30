use crate::gen::{Dice, GenOnce, Gen};

/// Adapter for `Gen::dyn`.
pub struct DynGen<'a, T> {
    dyn: Box<dyn Gen<T> + 'a>,
}

impl<'a, T> DynGen<'a, T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'a,
    {
        let dyn = Box::new(gen);
        DynGen { dyn }
    }
}

impl<'a, T> Gen<T> for DynGen<'a, T> {
    fn gen(&self, dice: &mut Dice) -> T {
        self.dyn.gen(dice)
    }
}

impl<'a, T> GenOnce<T> for DynGen<'a, T> {
    fn gen_once(self, dice: &mut Dice) -> T {
        self.gen(dice)
    }
}
