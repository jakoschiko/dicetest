use std::marker::PhantomData;

use crate::gen::{Fate, Gen, GenOnce};

/// Adapter for `GenOnce::flatten_once` and `Gen::flatten`.
pub struct FlattenGen<T, GT, GGT> {
    ggt: GGT,
    _t: PhantomData<T>,
    _gt: PhantomData<GT>,
}

impl<T, GT, GGT> FlattenGen<T, GT, GGT> {
    pub fn new(ggt: GGT) -> Self {
        FlattenGen {
            ggt,
            _t: PhantomData,
            _gt: PhantomData,
        }
    }
}

impl<T, GT, GGT> GenOnce<T> for FlattenGen<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: GenOnce<GT>,
{
    fn gen_once(self, fate: &mut Fate) -> T {
        let ggt = self.ggt;

        let gt = ggt.gen_once(fate);
        gt.gen_once(fate)
    }
}

impl<T, GT, GGT> Gen<T> for FlattenGen<T, GT, GGT>
where
    GT: GenOnce<T>,
    GGT: Gen<GT>,
{
    fn gen(&self, fate: &mut Fate) -> T {
        let ggt = &self.ggt;

        let gt = ggt.gen(fate);
        gt.gen_once(fate)
    }
}
