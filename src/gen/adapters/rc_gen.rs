use std::rc::Rc;

use crate::gen::{Fate, Gen, GenOnce};

/// Adapter for `GenOnce::rc`.
#[derive(Clone)]
pub struct RcGen<'a, T> {
    gen: Rc<dyn Gen<T> + 'a>,
}

impl<'a, T> RcGen<'a, T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'a,
    {
        let gen = Rc::new(gen);
        RcGen { gen }
    }
}

impl<'a, T> Gen<T> for RcGen<'a, T> {
    fn gen(&self, fate: &mut Fate) -> T {
        self.gen.gen(fate)
    }
}

impl<'a, T> GenOnce<T> for RcGen<'a, T> {
    fn gen_once(self, fate: &mut Fate) -> T {
        self.gen(fate)
    }
}
