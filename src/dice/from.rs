use crate::prelude::dice::*;

struct Fun<F>(F);

impl<T, F> DieOnce<T> for Fun<F>
where
    F: FnOnce(&mut Fate) -> T,
{
    fn roll_once(self, fate: &mut Fate) -> T {
        self.0(fate)
    }
}

impl<T, F> Die<T> for Fun<F>
where
    F: Fn(&mut Fate) -> T,
{
    fn roll(&self, fate: &mut Fate) -> T {
        self.0(fate)
    }
}

/// Helper for implementing a `DieOnce` from a `FnOnce` that takes a `Fate`.
pub fn from_fn_once<T, F>(f: F) -> impl DieOnce<T>
where
    F: FnOnce(&mut Fate) -> T,
{
    Fun(f)
}

/// Helper for implementing a `Die` from a `Fn` that takes a `Fate`.
pub fn from_fn<T, F>(f: F) -> impl Die<T>
where
    F: Fn(&mut Fate) -> T,
{
    Fun(f)
}

/// Helper for implementing a `Die` from a `Fn` that returns a `DieOnce`.
pub fn from_die_once_fn<T, TD, F>(f: F) -> impl Die<T>
where
    TD: DieOnce<T>,
    F: Fn() -> TD,
{
    from_fn(move |fate| f().roll_once(fate))
}
