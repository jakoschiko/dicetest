use std::marker::PhantomData;

use crate::gen::{Dice, GenOnce};

/// Adapter for `GenOnce::boxed_once`.
pub struct BoxedGenOnce<'a, T>
where
    T: 'a,
{
    r#dyn: Box<dyn Wrapper<T> + 'a>,
}

impl<'a, T> BoxedGenOnce<'a, T>
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
        let r#dyn = Box::new(wrapper);
        BoxedGenOnce { r#dyn }
    }
}

impl<'a, T> GenOnce<T> for BoxedGenOnce<'a, T>
where
    T: 'a,
{
    fn gen_once(mut self, dice: &mut Dice) -> T {
        self.r#dyn.gen_once(dice)
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
