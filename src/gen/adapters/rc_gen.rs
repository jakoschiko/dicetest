use std::rc::Rc;

use crate::gen::{Dice, GenOnce, Gen};

/// Adapter for `GenOnce::rc`.
#[derive(Clone)]
pub struct RcGen<'a, T> {
    r#dyn: Rc<dyn Gen<T> + 'a>,
}

impl<'a, T> RcGen<'a, T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'a,
    {
        let r#dyn = Rc::new(gen);
        RcGen { r#dyn }
    }
}

impl<'a, T> Gen<T> for RcGen<'a, T> {
    fn gen(&self, dice: &mut Dice) -> T {
        self.r#dyn.gen(dice)
    }
}

impl<'a, T> GenOnce<T> for RcGen<'a, T> {
    fn gen_once(self, dice: &mut Dice) -> T {
        self.gen(dice)
    }
}
