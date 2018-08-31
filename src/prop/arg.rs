use std::marker::PhantomData;
use std::fmt::Debug;

use ::prop::{Show, DebugShow, IntoLabel, NoLabel};
use ::gen::GenOnce;

/// Represents a random argument of type `T` that can be used for properties. The struct provides
///     * a generator for a random value of type `T` and
///     * human-readable details that can be used for creating a property label.
pub struct Arg<T, G, L, S>
where
    G: GenOnce<T>,
    L: IntoLabel,
    S: Show<T>,
{
    /// Generator for a single random value of type `T`.
    pub gen: G,
    /// The optional name of the argument.
    pub name_opt: Option<L>,
    /// Converter for creating for a human-readable representation of the generated value.
    pub show: S,
    _t: PhantomData<T>,
}

impl<T, G, L, S> Arg<T, G, L, S>
where
    G: GenOnce<T>,
    L: IntoLabel,
    S: Show<T>,
{
    /// Constructor for an `Arg` with a name.
    pub fn new(gen: G, name: L, show: S) -> Self {
        Arg {
            gen,
            name_opt: Some(name),
            show,
            _t: PhantomData,
        }
    }

    /// Sets the given name.
    pub fn name<LL>(self, name: LL) -> Arg<T, G, LL, S>
    where
        LL: IntoLabel,
    {
        Arg {
            gen: self.gen,
            name_opt: Some(name),
            show: self.show,
            _t: PhantomData,
        }
    }

    /// Removes the name.
    pub fn no_name(self) -> Arg<T, G, NoLabel, S> {
        Arg {
            gen: self.gen,
            name_opt: None,
            show: self.show,
            _t: PhantomData,
        }
    }

    /// Sets the given `Show`.
    pub fn show<SS>(self, show: SS) -> Arg<T, G, L, SS>
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

impl<T, G, S> Arg<T, G, NoLabel, S>
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
pub trait IntoArg<T, G, L, S>
where
    G: GenOnce<T>,
    L: IntoLabel,
    S: Show<T>,
{
    fn into_arg(self) -> Arg<T, G, L, S>;
}

impl<T, G, L, S> IntoArg<T, G, L, S> for Arg<T, G, L, S>
where
    G: GenOnce<T>,
    L: IntoLabel,
    S: Show<T>,
{
    fn into_arg(self) -> Arg<T, G, L, S> {
        self
    }
}

impl<T, G> IntoArg<T, G, NoLabel, DebugShow> for G
where
    T: Debug,
    G: GenOnce<T>,
{
    fn into_arg(self) -> Arg<T, G, NoLabel, DebugShow> {
        Arg::new_no_name(self, DebugShow)
    }
}

/// Extension methods for converting `GenOnce` into `Arg`.
pub trait GenArgExt<T>: GenOnce<T> + Sized
where
    T: Debug,
{
    /// Converts the `GenOnce` into an `Arg` with the given name and `DebugShow`.
    fn name<L: IntoLabel>(self, name: L) -> Arg<T, Self, L, DebugShow>;

    /// Converts the `GenOnce` into an `Arg` without a name and the given `Show`.
    fn show<S: Show<T>>(self, show: S) -> Arg<T, Self, NoLabel, S>;
}

impl<T, G> GenArgExt<T> for G
where
    T: Debug,
    G: GenOnce<T>,
{
    fn name<L: IntoLabel>(self, name: L) -> Arg<T, Self, L, DebugShow> {
        Arg::new(self, name, DebugShow)
    }

    fn show<S: Show<T>>(self, show: S) -> Arg<T, Self, NoLabel, S> {
        Arg::new_no_name(self, show)
    }
}
