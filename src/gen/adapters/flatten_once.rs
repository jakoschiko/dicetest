use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Params, GenOnce};

/// Default implementation for `GenOnce::flatten_once`.
pub struct FlattenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    ggt: GGT,
    _t: PhantomData<T>,
    _gt: PhantomData<GT>,
}

impl<T, GT, GGT> FlattenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    pub fn new(ggt: GGT) -> Self {
        FlattenOnce {
            ggt,
            _t: PhantomData,
            _gt: PhantomData,
        }
    }
}

impl<T, GT, GGT> GenOnce<T> for FlattenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    fn gen_once(self, rng: &mut Rng, params: &Params) -> T {
        let ggt = self.ggt;

        let gt = ggt.gen_once(rng, params);
        let t = gt.gen_once(rng, params);

        t
    }
}
