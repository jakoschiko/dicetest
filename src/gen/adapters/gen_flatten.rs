use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Params, GenOnce, Gen};

/// Default implementation for `Gen::flatten`.
pub struct GenFlatten<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    ggt: GGT,
    _t: PhantomData<T>,
    _gt: PhantomData<GT>,
}

impl<T, GT, GGT> GenFlatten<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    pub fn new(ggt: GGT) -> Self {
        GenFlatten {
            ggt,
            _t: PhantomData,
            _gt: PhantomData,
        }
    }
}

impl<T, GT, GGT> Gen<T> for GenFlatten<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    fn gen(&self, rng: &mut Rng, params: &Params) -> T {
        let ggt = &self.ggt;

        let gt = ggt.gen(rng, params);
        let t = gt.gen_once(rng, params);

        t
    }
}

impl<T, GT, GGT> GenOnce<T> for GenFlatten<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    fn gen_once(self, rng: &mut Rng, params: &Params) -> T {
        self.gen(rng, params)
    }
}
