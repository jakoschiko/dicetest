use std::marker::PhantomData;

use crate::rng::Rng;
use crate::gen::{Limit, GenOnce};

/// Adapter for `GenOnce::dyn_once`.
pub struct DynGenOnce<'a, T>
where
    T: 'a,
{
    dyn: Box<dyn Wrapper<T> + 'a>,
}

impl<'a, T> DynGenOnce<'a, T>
where
    T: 'a,
{
    pub fn new<G>(gen: G) -> Self
    where
        G: GenOnce<T> + 'a,
    {
        let wrapper = GenOnceWrapper {
            gen: Some(gen),
            _t: PhantomData,
        };
        let dyn = Box::new(wrapper);
        DynGenOnce { dyn }
    }
}

impl<'a, T> GenOnce<T> for DynGenOnce<'a, T>
where
    T: 'a,
{
    fn gen_once(mut self, rng: &mut Rng, lim: Limit) -> T {
        self.dyn.gen_once(rng, lim)
    }
}

trait Wrapper<T> {
    fn gen_once(&mut self, &mut Rng, Limit) -> T;
}

struct GenOnceWrapper<T, G>
where
    G: GenOnce<T>,
{
    gen: Option<G>,
    _t: PhantomData<T>,
}

impl<T, G> Wrapper<T> for GenOnceWrapper<T, G>
where
    G: GenOnce<T>,
{
    fn gen_once(&mut self, rng: &mut Rng, lim: Limit) -> T {
        let gen = self.gen.take().expect("GenOnceWrapper::gen should not be called twice");
        gen.gen_once(rng, lim)
    }
}
