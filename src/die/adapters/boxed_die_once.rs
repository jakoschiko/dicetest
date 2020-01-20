use std::marker::PhantomData;

use crate::die::{DieOnce, Limit};
use crate::prand::Prng;

/// Adapter for `DieOnce::boxed_once`.
pub struct BoxedDieOnce<'a, T>
where
    T: 'a,
{
    die: Box<dyn Wrapper<T> + 'a>,
}

impl<'a, T> BoxedDieOnce<'a, T>
where
    T: 'a,
{
    pub fn new<D>(die: D) -> Self
    where
        D: DieOnce<T> + 'a,
    {
        let wrapper = DieOnceWrapper {
            die: Some(die),
            _t: PhantomData,
        };
        let die = Box::new(wrapper);
        BoxedDieOnce { die }
    }
}

impl<'a, T> DieOnce<T> for BoxedDieOnce<'a, T>
where
    T: 'a,
{
    fn roll_once(mut self, prng: &mut Prng, limit: Limit) -> T {
        self.die.roll_once(prng, limit)
    }
}

trait Wrapper<T> {
    fn roll_once(&mut self, prng: &mut Prng, limit: Limit) -> T;
}

struct DieOnceWrapper<T, D>
where
    D: DieOnce<T>,
{
    die: Option<D>,
    _t: PhantomData<T>,
}

impl<T, D> Wrapper<T> for DieOnceWrapper<T, D>
where
    D: DieOnce<T>,
{
    fn roll_once(&mut self, prng: &mut Prng, limit: Limit) -> T {
        let die = self.die.take().unwrap();
        die.roll_once(prng, limit)
    }
}
