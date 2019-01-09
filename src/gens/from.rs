use crate::prelude::gens::*;

struct Fun<F>(F);

impl<T, F> GenOnce<T> for Fun<F>
where
    F: FnOnce(&mut Fate) -> T,
{
    fn gen_once(self, fate: &mut Fate) -> T {
        self.0(fate)
    }
}

impl<T, F> Gen<T> for Fun<F>
where
    F: Fn(&mut Fate) -> T,
{
    fn gen(&self, fate: &mut Fate) -> T {
        self.0(fate)
    }
}

/// Helper for implementing a `GenOnce` from a `FnOnce` that takes a `Fate`.
pub fn from_fn_once<T, F>(f: F) -> impl GenOnce<T>
where
    F: FnOnce(&mut Fate) -> T,
{
    Fun(f)
}

/// Helper for implementing a `Gen` from a `Fn` that takes a `Fate`.
pub fn from_fn<T, F>(f: F) -> impl Gen<T>
where
    F: Fn(&mut Fate) -> T,
{
    Fun(f)
}

/// Helper for implementing a `Gen` from a `Fn` that returns a `GenOnce`.
pub fn from_gen_once_fn<T, GT, F>(f: F) -> impl Gen<T>
where
    GT: GenOnce<T>,
    F: Fn() -> GT,
{
    from_fn(move |fate| f().gen_once(fate))
}
