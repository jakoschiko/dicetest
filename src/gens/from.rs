use crate::prelude::gens::*;

struct Fun<F>(F);

impl<T, F> GenOnce<T> for Fun<F>
where
    F: FnOnce(&mut Dice) -> T,
{
    fn gen_once(self, dice: &mut Dice) -> T {
        self.0(dice)
    }
}

impl<T, F> Gen<T> for Fun<F>
where
    F: Fn(&mut Dice) -> T,
{
    fn gen(&self, dice: &mut Dice) -> T {
        self.0(dice)
    }
}

/// Helper for implementing a `GenOnce` from a `FnOnce` that takes a `Dice`.
pub fn from_fn_once<T, F>(f: F) -> impl GenOnce<T>
where
    F: FnOnce(&mut Dice) -> T,
{
    Fun(f)
}

/// Helper for implementing a `Gen` from a `Fn` that takes a `Dice`.
pub fn from_fn<T, F>(f: F) -> impl Gen<T>
where
    F: Fn(&mut Dice) -> T,
{
    Fun(f)
}

/// Helper for implementing a `Gen` from a `Fn` that returns a `GenOnce`.
pub fn from_gen_once_fn<T, GT, F>(f: F) -> impl Gen<T>
where
    GT: GenOnce<T>,
    F: Fn() -> GT,
{
    from_fn(move |dice| f().gen_once(dice))
}
