use std::marker::PhantomData;

use crate::rng::Rng;
use crate::gen::{Limit, GenOnce};

/// Adapter for `GenOnce::flatten_once`.
pub struct FlattenGenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    ggt: GGT,
    _t: PhantomData<T>,
    _gt: PhantomData<GT>,
}

impl<T, GT, GGT> FlattenGenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    pub fn new(ggt: GGT) -> Self {
        FlattenGenOnce {
            ggt,
            _t: PhantomData,
            _gt: PhantomData,
        }
    }
}

impl<T, GT, GGT> GenOnce<T> for FlattenGenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    fn gen_once(self, rng: &mut Rng, lim: Limit) -> T {
        let ggt = self.ggt;

        let gt = ggt.gen_once(rng, lim);
        let t = gt.gen_once(rng, lim);

        t
    }
}
