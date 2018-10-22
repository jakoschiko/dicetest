use std::rc::Rc;

use ::rng::Rng;
use ::gen::{Limit, GenOnce, Gen};

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
    fn gen(&self, rng: &mut Rng, lim: Limit) -> T {
        self.dyn.gen(rng, lim)
    }
}

impl<'a, T> GenOnce<T> for DynRcGen<'a, T> {
    fn gen_once(self, rng: &mut Rng, lim: Limit) -> T {
        self.gen(rng, lim)
    }
}
