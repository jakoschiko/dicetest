use crate::gen::{Dice, Gen, GenOnce};

/// Adapter for `Gen::boxed`.
pub struct BoxedGen<'a, T> {
    r#dyn: Box<dyn Gen<T> + 'a>,
}

impl<'a, T> BoxedGen<'a, T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'a,
    {
        let r#dyn = Box::new(gen);
        BoxedGen { r#dyn }
    }
}

impl<'a, T> Gen<T> for BoxedGen<'a, T> {
    fn gen(&self, dice: &mut Dice) -> T {
        self.r#dyn.gen(dice)
    }
}

impl<'a, T> GenOnce<T> for BoxedGen<'a, T> {
    fn gen_once(self, dice: &mut Dice) -> T {
        self.gen(dice)
    }
}
