use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Size, GenOnce};

/// Default implementation for `GenOnce::flatten_once`.
pub struct GenFlattenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    ggt: GGT,
    _t: PhantomData<T>,
    _gt: PhantomData<GT>,
}

impl<T, GT, GGT> GenFlattenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    pub fn new(ggt: GGT) -> Self {
        GenFlattenOnce {
            ggt,
            _t: PhantomData,
            _gt: PhantomData,
        }
    }
}

impl<T, GT, GGT> GenOnce<T> for GenFlattenOnce<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    fn gen_once(self, rng: &mut Rng, size: Size) -> T {
        let ggt = self.ggt;

        let gt = ggt.gen_once(rng, size);
        let t = gt.gen_once(rng, size);

        t
    }
}
