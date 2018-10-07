use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Size, GenOnce};

/// Adapter for `GenOnce::flat_map_once`.
pub struct GenFlatMapOnce<T, U, GT, GU, F>
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

impl<T, U, GT, GU, F> GenFlatMapOnce<T, U, GT, GU, F>
where
    GT: GenOnce<T>,
    GU: GenOnce<U>,
    F: FnOnce(T) -> GU,
{
    pub fn new(gt: GT, f: F) -> Self {
        GenFlatMapOnce {
            gt,
            f,
            _t: PhantomData,
            _u: PhantomData,
            _gu: PhantomData,
        }
    }
}

impl<T, U, GT, GU, F> GenOnce<U> for GenFlatMapOnce<T, U, GT, GU, F>
where
    GT: GenOnce<T>,
    GU: GenOnce<U>,
    F: FnOnce(T) -> GU,
{
    fn gen_once(self, rng: &mut Rng, size: Size) -> U {
        let gt = self.gt;
        let f = self.f;

        let t = gt.gen_once(rng, size);
        let gu = f(t);
        let u = gu.gen_once(rng, size);

        u
    }
}
