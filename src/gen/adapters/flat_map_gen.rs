use std::marker::PhantomData;

use crate::rng::Rng;
use crate::gen::{Limit, GenOnce, Gen};

/// Adapter for `Gen::flat_map`.
pub struct FlatMapGen<T, U, GT, GU, F>
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

impl<T, U, GT, GU, F> FlatMapGen<T, U, GT, GU, F>
where
    GT: Gen<T>,
    GU: GenOnce<U>,
    F: Fn(T) -> GU,
{
    pub fn new(gt: GT, f: F) -> Self {
        FlatMapGen {
            gt,
            f,
            _t: PhantomData,
            _u: PhantomData,
            _gu: PhantomData,
        }
    }
}

impl<T, U, GT, GU, F> Gen<U> for FlatMapGen<T, U, GT, GU, F>
where
    GT: Gen<T>,
    GU: GenOnce<U>,
    F: Fn(T) -> GU,
{
    fn gen(&self, rng: &mut Rng, lim: Limit) -> U {
        let gt = &self.gt;
        let f = &self.f;

        let t = gt.gen(rng, lim);
        let gu = f(t);
        let u = gu.gen_once(rng, lim);

        u
    }
}

impl<T, U, GT, GU, F> GenOnce<U> for FlatMapGen<T, U, GT, GU, F>
where
    GT: Gen<T>,
    GU: GenOnce<U>,
    F: Fn(T) -> GU,
{
    fn gen_once(self, rng: &mut Rng, lim: Limit) -> U {
        self.gen(rng, lim)
    }
}
