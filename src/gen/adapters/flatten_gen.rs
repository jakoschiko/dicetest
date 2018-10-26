use std::marker::PhantomData;

use crate::rng::Rng;
use crate::gen::{Limit, GenOnce, Gen};

/// Adapter for `Gen::flatten`.
pub struct FlattenGen<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    ggt: GGT,
    _t: PhantomData<T>,
    _gt: PhantomData<GT>,
}

impl<T, GT, GGT> FlattenGen<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    pub fn new(ggt: GGT) -> Self {
        FlattenGen {
            ggt,
            _t: PhantomData,
            _gt: PhantomData,
        }
    }
}

impl<T, GT, GGT> Gen<T> for FlattenGen<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    fn gen(&self, rng: &mut Rng, lim: Limit) -> T {
        let ggt = &self.ggt;

        let gt = ggt.gen(rng, lim);
        let t = gt.gen_once(rng, lim);

        t
    }
}

impl<T, GT, GGT> GenOnce<T> for FlattenGen<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    fn gen_once(self, rng: &mut Rng, lim: Limit) -> T {
        self.gen(rng, lim)
    }
}
