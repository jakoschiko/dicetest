use std::marker::PhantomData;
use std::fmt::Debug;

use ::prop::{Show, DebugShow, IntoLabel, NoLabel};
use ::gen::GenOnce;

/// Represents a random argument of type `T` that can be used for properties. The struct provides
/// 	* a generator for a random value of type `T` and
/// 	* human-readable details for creating a property label that describes the generated value.
pub struct Arg<T, G, L, S>
where
	G: GenOnce<T>,
	L: IntoLabel,
	S: Show<T>,
{
	/// Generates a single random value of type `T`.
	pub gen: G,
	/// An optional description of the argument. E.g. "length" would be an appropriate
	/// description of an integer used as the length of a vector.
	pub label_opt: Option<L>,
	/// Converts the generated value of type `T` into a human-readable text.
	pub show: S,
	_t: PhantomData<T>,
}

impl<T, G, L, S> Arg<T, G, L, S>
where
	G: GenOnce<T>,
	L: IntoLabel,
	S: Show<T>,
{
	/// Constructor for an `Arg` with a label.
	pub fn new(gen: G, label: L, show: S) -> Self {
		Arg {
			gen,
			label_opt: Some(label),
			show,
			_t: PhantomData,
		}
	}

	/// Sets the given label.
	pub fn label<LL>(self, label: LL) -> Arg<T, G, LL, S>
	where
		LL: IntoLabel,
	{
		Arg {
			gen: self.gen,
			label_opt: Some(label),
			show: self.show,
			_t: PhantomData,
		}
	}

	/// Sets the label to `None`.
	pub fn no_label(self) -> Arg<T, G, NoLabel, S> {
		Arg {
			gen: self.gen,
			label_opt: None,
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
			label_opt: self.label_opt,
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
	// Constructor for an `Arg` without a label.
	pub fn new_no_label(gen: G, show: S) -> Self {
		Arg {
			gen,
			label_opt: None,
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

impl<T, G> IntoArg<T, G, NoLabel, DebugShow> for G
where
	T: Debug,
	G: GenOnce<T>,
{
	fn into_arg(self) -> Arg<T, G, NoLabel, DebugShow> {
		Arg::new_no_label(self, DebugShow)
	}
}

/// Extension methods for converting `GenOnce` into `Arg`.
pub trait GenArgExt<T>: GenOnce<T> + Sized
where
	T: Debug,
{
	/// Converts the `GenOnce` into an `Arg` with given label and `DebugShow`.
	fn label<L: IntoLabel>(self, label: L) -> Arg<T, Self, L, DebugShow>;

	/// Converts the `GenOnce` into an `Arg` without a label and the given `Show`.
	fn show<S: Show<T>>(self, show: S) -> Arg<T, Self, NoLabel, S>;
}

impl<T, G> GenArgExt<T> for G
where
	T: Debug,
	G: GenOnce<T>,
{
	fn label<L: IntoLabel>(self, label: L) -> Arg<T, Self, L, DebugShow> {
		Arg::new(self, label, DebugShow)
	}

	fn show<S: Show<T>>(self, show: S) -> Arg<T, Self, NoLabel, S> {
		Arg::new_no_label(self, show)
	}
}
