use std::sync::Arc;

use crate::gen::{Dice, GenOnce, Gen};

/// Adapter for `Gen::dyn_arc`.
#[derive(Clone)]
pub struct DynArcGen<T> {
    dyn: Arc<dyn Gen<T>>,
}

impl<T> DynArcGen<T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'static,
    {
        let dyn = Arc::new(gen);
        DynArcGen { dyn }
    }
}

impl<T> Gen<T> for DynArcGen<T> {
    fn gen(&self, dice: &mut Dice) -> T {
        self.dyn.gen(dice)
    }
}

impl<T> GenOnce<T> for DynArcGen<T> {
    fn gen_once(self, dice: &mut Dice) -> T {
        self.gen(dice)
    }
}
