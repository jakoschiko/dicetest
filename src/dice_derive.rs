/// Derives the trait [`Dice`](./trait.Dice.html) for structs and enums.
///
/// # Limit
///
/// The derived [`Dice`](./trait.Dice.html) implementation will split the [`Limit`](crate::Limit)
/// randomly when generting the fields of `Self`. This helps to limit the overall size of the
/// generated values.
///
/// # Recursion
///
/// Deriving [`Dice`](./trait.Dice.html) for recursive types isn't supported for now.
/// You need to implement it manually.
///
/// ```rust
/// // Recursive types like Foo are not supported!
/// struct Foo {
///   next: Option<Box<Foo>>,
/// }
///
/// // Mutual recursive types like Bar and Baz are also not supported!
/// struct Bar {
///   baz: Baz,
/// }
/// struct Baz {
///   bar: Option<Box<Bar>>,
/// }
/// ```
///
/// # Attributes
///
/// The derived [`Dice`](./trait.Dice.html) implementation can be customized by adding attributes
/// with `#[dice(...)]`.
///
/// ## die
/// The attribute `die` can be used on a field to define the `Die` that is used for
/// this field. It expects an expression that returns a `Die` for the type of the field.
///
/// ```rust
/// use dicetest::prelude::*;
///
/// #[derive(Dice)]
/// struct Foo {
///     name: String,
///     #[dice(die = dice::just(Vec::new()))]
///     _cache: Vec<u8>,
/// }
/// ```
///
/// ## weight
///
/// The attribute `weight` can be used on a variant to define the probability of this variant
/// relative to the other variants. It expects a literal of type `u32`. The default weight of
/// a variant is 1.
///
/// ```rust
/// use dicetest::prelude::*;
///
/// #[derive(Dice)]
/// enum Foo {
///     #[dice(weight = 10)]
///     VeryLikely,
///     #[dice(weight = 5)]
///     Likely,
///     Unlikely,
///     #[dice(weight = 0)]
///     Never,
/// }
/// ```
///
/// # Examples
///
/// ```rust
/// use dicetest::prelude::*;
///
/// #[derive(Dice)]
/// struct Foo(u8);
///
/// #[derive(Dice)]
/// struct Bar {
///     a: String,
///     b: Vec<u8>,
/// }
///
/// #[derive(Dice)]
/// enum FooBar {
///     Foo(Foo),
///     Bar(Bar),
/// }
///
/// #[derive(Dice)]
/// struct Baz<T> {
///     a: Option<T>,
///     b: Vec<T>,
/// }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use dicetest_derive::Dice;
