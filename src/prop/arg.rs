use std::marker::PhantomData;
use std::fmt::Debug;

use crate::prop::{Show, DebugShow};
use crate::gen::GenOnce;

/// Represents a random argument of type `T` that can be used for properties.
pub struct Arg<T, G, S>
where
    G: GenOnce<T>,
    S: Show<T>,
{
    /// Generator for a single random value of type `T`.
    pub gen: G,
    /// The optional name of the argument.
    pub name_opt: Option<&'static str>,
    /// Converter for creating for a human-readable representation of the generated value.
    pub show: S,
    _t: PhantomData<T>,
}

impl<T, G, S> Arg<T, G, S>
where
    G: GenOnce<T>,
    S: Show<T>,
{
    /// Constructor for an `Arg` with a name.
    pub fn new(gen: G, name: &'static str, show: S) -> Self {
        Arg {
            gen,
            name_opt: Some(name),
            show,
            _t: PhantomData,
        }
    }

    /// Sets the given name.
    pub fn name(self, name: &'static str) -> Self {
        Arg {
            gen: self.gen,
            name_opt: Some(name),
            show: self.show,
            _t: PhantomData,
        }
    }

    /// Removes the name.
    pub fn no_name(self) -> Self {
        Arg {
            gen: self.gen,
            name_opt: None,
            show: self.show,
            _t: PhantomData,
        }
    }

    /// Sets the given `Show`.
    pub fn show<SS>(self, show: SS) -> Arg<T, G, SS>
    where
        SS: Show<T>,
    {
        Arg {
            gen: self.gen,
            name_opt: self.name_opt,
            show: show,
            _t: PhantomData,
        }
    }
}

impl<T, G, S> Arg<T, G, S>
where
    G: GenOnce<T>,
    S: Show<T>,
{
    // Constructor for an `Arg` without a name.
    pub fn new_no_name(gen: G, show: S) -> Self {
        Arg {
            gen,
            name_opt: None,
            show,
            _t: PhantomData,
        }
    }
}

/// Trait for converting a type into `Arg`.
pub trait IntoArg<T, G, S>
where
    G: GenOnce<T>,
    S: Show<T>,
{
    fn into_arg(self) -> Arg<T, G, S>;
}

impl<T, G, S> IntoArg<T, G, S> for Arg<T, G, S>
where
    G: GenOnce<T>,
    S: Show<T>,
{
    fn into_arg(self) -> Arg<T, G, S> {
        self
    }
}

impl<T, G> IntoArg<T, G, DebugShow> for G
where
    T: Debug,
    G: GenOnce<T>,
{
    fn into_arg(self) -> Arg<T, G, DebugShow> {
        Arg::new_no_name(self, DebugShow)
    }
}

/// Extension methods for converting `GenOnce` into `Arg`.
pub trait GenArgExt<T>: GenOnce<T> + Sized
where
    T: Debug,
{
    /// Converts the `GenOnce` into an `Arg` with the given name and `DebugShow`.
    fn name(self, name: &'static str) -> Arg<T, Self, DebugShow>;

    /// Converts the `GenOnce` into an `Arg` without a name and the given `Show`.
    fn show<S: Show<T>>(self, show: S) -> Arg<T, Self, S>;
}

impl<T, G> GenArgExt<T> for G
where
    T: Debug,
    G: GenOnce<T>,
{
    fn name(self, name: &'static str) -> Arg<T, Self, DebugShow> {
        Arg::new(self, name, DebugShow)
    }

    fn show<S: Show<T>>(self, show: S) -> Arg<T, Self, S> {
        Arg::new_no_name(self, show)
    }
}
