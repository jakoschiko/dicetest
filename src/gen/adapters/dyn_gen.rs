use ::rng::Rng;
use ::gen::{Limit, GenOnce, Gen};

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
    fn gen(&self, rng: &mut Rng, lim: Limit) -> T {
        self.dyn.gen(rng, lim)
    }
}

impl<'a, T> GenOnce<T> for DynGen<'a, T> {
    fn gen_once(self, rng: &mut Rng, lim: Limit) -> T {
        self.gen(rng, lim)
    }
}
