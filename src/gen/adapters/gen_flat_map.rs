use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Params, GenOnce, Gen};

/// Default implementation for `Gen::flat_map`.
pub struct GenFlatMap<T, U, GT, GU, F>
where
    GT: Gen<T>,
    GU: GenOnce<U>,
    F: Fn(T) -> GU,
{
    gt: GT,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
    _gu: PhantomData<GU>,
}

impl<T, U, GT, GU, F> GenFlatMap<T, U, GT, GU, F>
where
    GT: Gen<T>,
    GU: GenOnce<U>,
    F: Fn(T) -> GU,
{
    pub fn new(gt: GT, f: F) -> Self {
        GenFlatMap {
            gt,
            f,
            _t: PhantomData,
            _u: PhantomData,
            _gu: PhantomData,
        }
    }
}

impl<T, U, GT, GU, F> Gen<U> for GenFlatMap<T, U, GT, GU, F>
where
    GT: Gen<T>,
    GU: GenOnce<U>,
    F: Fn(T) -> GU,
{
    fn gen(&self, rng: &mut Rng, params: &Params) -> U {
        let gt = &self.gt;
        let f = &self.f;

        let t = gt.gen(rng, params);
        let gu = f(t);
        let u = gu.gen_once(rng, params);

        u
    }
}

impl<T, U, GT, GU, F> GenOnce<U> for GenFlatMap<T, U, GT, GU, F>
where
    GT: Gen<T>,
    GU: GenOnce<U>,
    F: Fn(T) -> GU,
{
    fn gen_once(self, rng: &mut Rng, params: &Params) -> U {
        self.gen(rng, params)
    }
}
