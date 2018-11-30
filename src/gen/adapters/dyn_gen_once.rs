use std::marker::PhantomData;

use crate::gen::{Dice, GenOnce};

/// Adapter for `GenOnce::dyn_once`.
pub struct DynGenOnce<'a, T>
where
    T: 'a,
{
    dyn: Box<dyn Wrapper<T> + 'a>,
}

impl<'a, T> DynGenOnce<'a, T>
where
    T: 'a,
{
    pub fn new<G>(gen: G) -> Self
    where
        G: GenOnce<T> + 'a,
    {
        let wrapper = GenOnceWrapper {
            gen: Some(gen),
            _t: PhantomData,
        };
        let dyn = Box::new(wrapper);
        DynGenOnce { dyn }
    }
}

impl<'a, T> GenOnce<T> for DynGenOnce<'a, T>
where
    T: 'a,
{
    fn gen_once(mut self, dice: &mut Dice) -> T {
        self.dyn.gen_once(dice)
    }
}

trait Wrapper<T> {
    fn gen_once(&mut self, dice: &mut Dice) -> T;
}

struct GenOnceWrapper<T, G>
where
    G: GenOnce<T>,
{
    gen: Option<G>,
    _t: PhantomData<T>,
}

impl<T, G> Wrapper<T> for GenOnceWrapper<T, G>
where
    G: GenOnce<T>,
{
    fn gen_once(&mut self, dice: &mut Dice) -> T {
        let gen = self.gen.take().unwrap();
        gen.gen_once(dice)
    }
}
