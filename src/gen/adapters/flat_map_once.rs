use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Params, GenOnce};

/// Default implementation for `GenOnce::flat_map_once`.
pub struct FlatMapOnce<T, U, GT, GU, F>
where
    GT: GenOnce<T>,
    GU: GenOnce<U>,
    F: FnOnce(T) -> GU,
{
    gt: GT,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
    _gu: PhantomData<GU>,
}

impl<T, U, GT, GU, F> FlatMapOnce<T, U, GT, GU, F>
where
    GT: GenOnce<T>,
    GU: GenOnce<U>,
    F: FnOnce(T) -> GU,
{
    pub fn new(gt: GT, f: F) -> Self {
        FlatMapOnce {
            gt,
            f,
            _t: PhantomData,
            _u: PhantomData,
            _gu: PhantomData,
        }
    }
}

impl<T, U, GT, GU, F> GenOnce<U> for FlatMapOnce<T, U, GT, GU, F>
where
    GT: GenOnce<T>,
    GU: GenOnce<U>,
    F: FnOnce(T) -> GU,
{
    fn gen_once(self, rng: &mut Rng, params: &Params) -> U {
        let gt = self.gt;
        let f = self.f;

        let t = gt.gen_once(rng, params);
        let gu = f(t);
        let u = gu.gen_once(rng, params);

        u
    }
}
