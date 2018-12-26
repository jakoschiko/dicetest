use std::marker::PhantomData;

use crate::gen::{Dice, GenOnce, Gen};

/// Adapter for `GenOnce::flat_map_once` and `Gen::flat_map`.
pub struct FlatMapGen<T, U, GT, GU, F> {
    gt: GT,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
    _gu: PhantomData<GU>,
}

impl<T, U, GT, GU, F> FlatMapGen<T, U, GT, GU, F> {
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

impl<T, U, GT, GU, F> GenOnce<U> for FlatMapGen<T, U, GT, GU, F>
where
    GT: GenOnce<T>,
    GU: GenOnce<U>,
    F: FnOnce(T) -> GU,
{
    fn gen_once(self, dice: &mut Dice) -> U {
        let gt = self.gt;
        let f = self.f;

        let t = gt.gen_once(dice);
        let gu = f(t);
        gu.gen_once(dice)
    }
}

impl<T, U, GT, GU, F> Gen<U> for FlatMapGen<T, U, GT, GU, F>
where
    GT: Gen<T>,
    GU: GenOnce<U>,
    F: Fn(T) -> GU,
{
    fn gen(&self, dice: &mut Dice) -> U {
        let gt = &self.gt;
        let f = &self.f;

        let t = gt.gen(dice);
        let gu = f(t);
        gu.gen_once(dice)
    }
}
