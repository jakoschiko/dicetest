use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Size, GenOnce, Gen};

/// Adapter for `Gen::flatten`.
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
    fn gen(&self, rng: &mut Rng, size: Size) -> T {
        let ggt = &self.ggt;

        let gt = ggt.gen(rng, size);
        let t = gt.gen_once(rng, size);

        t
    }
}

impl<T, GT, GGT> GenOnce<T> for GenFlatten<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    fn gen_once(self, rng: &mut Rng, size: Size) -> T {
        self.gen(rng, size)
    }
}
