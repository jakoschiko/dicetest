use crate::{Die, Limit, dice};

/// Provides a [`Die`] implementation that generats pseudorandom values of `Self`.
pub trait Dice: Sized {
    /// Indicates whether the [`Die`] uses [`Limit`].
    ///
    /// If the value is `false`, the caller may always pass a zero [`Limit`] to the [`Die`].
    /// This is useful for structs which want to split the [`Limit`] for their fields,
    /// but only for fields that actually use the [`Limit`].
    const USES_LIMIT: bool;

    /// Returns a [`Die`] for `Self` with a reasonable value distribution for many use cases.
    ///
    /// Ideally, it generates the full range of possible values. Generating known edge
    /// cases is more important than uniform distribution.
    fn die() -> impl Die<Self>;
}

/// Summons a [`Die`] for `T` based on [`Dice`].
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let _byte: u8 = fate.roll(die());
/// ```
pub fn die<T: Dice>() -> impl Die<T> {
    T::die()
}

macro_rules! impl_dice_for_primitve {
    ($type:ty, $die:expr) => {
        impl Dice for $type {
            const USES_LIMIT: bool = false;

            fn die() -> impl Die<Self> {
                $die
            }
        }
    };
}

impl_dice_for_primitve!((), dice::just(()));
impl_dice_for_primitve!(bool, dice::bool());
impl_dice_for_primitve!(usize, dice::usize(..));
impl_dice_for_primitve!(u8, dice::u8(..));
impl_dice_for_primitve!(u16, dice::u16(..));
impl_dice_for_primitve!(u32, dice::u32(..));
impl_dice_for_primitve!(u64, dice::u64(..));
impl_dice_for_primitve!(u128, dice::u128(..));
impl_dice_for_primitve!(isize, dice::isize(..));
impl_dice_for_primitve!(i8, dice::i8(..));
impl_dice_for_primitve!(i16, dice::i16(..));
impl_dice_for_primitve!(i32, dice::i32(..));
impl_dice_for_primitve!(i64, dice::i64(..));
impl_dice_for_primitve!(i128, dice::i128(..));
impl_dice_for_primitve!(f32, dice::any_f32());
impl_dice_for_primitve!(f64, dice::any_f64());
impl_dice_for_primitve!(char, dice::char());
impl_dice_for_primitve!(
    std::num::NonZeroU8,
    dice::u8(1..).map(|int| std::num::NonZeroU8::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroU16,
    dice::u16(1..).map(|int| std::num::NonZeroU16::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroU32,
    dice::u32(1..).map(|int| std::num::NonZeroU32::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroU64,
    dice::u64(1..).map(|int| std::num::NonZeroU64::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroU128,
    dice::u128(1..).map(|int| std::num::NonZeroU128::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroUsize,
    dice::usize(1..).map(|int| std::num::NonZeroUsize::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroI8,
    dice::one_of_die()
        .two(dice::i8(..=-1), dice::i8(1..))
        .map(|int| std::num::NonZeroI8::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroI16,
    dice::one_of_die()
        .two(dice::i16(..=-1), dice::i16(1..))
        .map(|int| std::num::NonZeroI16::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroI32,
    dice::one_of_die()
        .two(dice::i32(..=-1), dice::i32(1..))
        .map(|int| std::num::NonZeroI32::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroI64,
    dice::one_of_die()
        .two(dice::i64(..=-1), dice::i64(1..))
        .map(|int| std::num::NonZeroI64::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroI128,
    dice::one_of_die()
        .two(dice::i128(..=-1), dice::i128(1..))
        .map(|int| std::num::NonZeroI128::new(int).unwrap())
);
impl_dice_for_primitve!(
    std::num::NonZeroIsize,
    dice::one_of_die()
        .two(dice::isize(..=-1), dice::isize(1..))
        .map(|int| std::num::NonZeroIsize::new(int).unwrap())
);

impl<T: ?Sized> Dice for std::marker::PhantomData<T> {
    const USES_LIMIT: bool = false;

    fn die() -> impl Die<Self> {
        dice::just(Self)
    }
}

impl Dice for std::marker::PhantomPinned {
    const USES_LIMIT: bool = false;

    fn die() -> impl Die<Self> {
        dice::just(Self)
    }
}

impl<T: Dice> Dice for Option<T> {
    const USES_LIMIT: bool = T::USES_LIMIT;

    fn die() -> impl Die<Self> {
        dice::option(T::die())
    }
}

impl<T: Dice, E: Dice> Dice for Result<T, E> {
    const USES_LIMIT: bool = T::USES_LIMIT || E::USES_LIMIT;

    fn die() -> impl Die<Self> {
        dice::result(T::die(), E::die())
    }
}

macro_rules! impl_dice_for_wrapper {
    ($type:ty, $new:expr) => {
        impl<T: Dice> Dice for $type {
            const USES_LIMIT: bool = T::USES_LIMIT;

            fn die() -> impl Die<Self> {
                T::die().map($new)
            }
        }
    };
}

impl_dice_for_wrapper!(Box<T>, Self::new);
impl_dice_for_wrapper!(std::rc::Rc<T>, Self::new);
impl_dice_for_wrapper!(std::sync::Arc<T>, Self::new);
impl_dice_for_wrapper!(std::cell::Cell<T>, Self::new);
impl_dice_for_wrapper!(std::cell::UnsafeCell<T>, Self::new);
impl_dice_for_wrapper!(std::cmp::Reverse<T>, Self);

impl<'a, T> Dice for std::borrow::Cow<'a, T>
where
    T: ToOwned + ?Sized,
    T::Owned: Dice,
{
    const USES_LIMIT: bool = T::Owned::USES_LIMIT;

    fn die() -> impl Die<Self> {
        T::Owned::die().map(Self::Owned)
    }
}

impl<T: Dice> Dice for std::cell::OnceCell<T> {
    const USES_LIMIT: bool = T::USES_LIMIT;

    fn die() -> impl Die<Self> {
        dice::option(T::die()).map(|value| {
            let cell = Self::new();
            if let Some(value) = value {
                let _result = cell.set(value);
            }
            cell
        })
    }
}

impl<T: Dice> Dice for std::task::Poll<T> {
    const USES_LIMIT: bool = T::USES_LIMIT;

    fn die() -> impl Die<Self> {
        dice::option(T::die()).map(|value| value.map_or(Self::Pending, Self::Ready))
    }
}

impl Dice for std::cmp::Ordering {
    const USES_LIMIT: bool = false;

    fn die() -> impl Die<Self> {
        dice::one_of().three(
            std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater,
        )
    }
}

impl Dice for String {
    const USES_LIMIT: bool = true;

    fn die() -> impl Die<Self> {
        dice::string(char::die(), ..)
    }
}

macro_rules! impl_dice_for_collection {
    ($type:ty, $builder_die:expr) => {
        impl<T: Dice> Dice for $type {
            const USES_LIMIT: bool = true;

            fn die() -> impl Die<Self> {
                let outer_collection_die = {
                    let builder_die = $builder_die;
                    let t_die = T::die();
                    dice::outer_collection(builder_die, t_die, ..)
                };
                let collection_die = {
                    let builder_die = $builder_die;
                    let t_die = T::die();
                    let t_die =
                        dice::from_fn(move |mut fate| fate.with_limit(Limit(0)).roll(&t_die));
                    dice::collection(builder_die, t_die, ..)
                };

                dice::from_fn(move |mut fate| {
                    if T::USES_LIMIT {
                        fate.roll(&outer_collection_die)
                    } else {
                        fate.roll(&collection_die)
                    }
                })
            }
        }
    };
}

impl_dice_for_collection!(Vec<T>, dice::from_fn(|_| dice::VecBuilder));
impl_dice_for_collection!(
    std::collections::VecDeque<T>,
    dice::from_fn(|_| dice::VecDequeBuilder)
);
impl_dice_for_collection!(
    std::collections::LinkedList<T>,
    dice::from_fn(|_| dice::LinkedListBuilder)
);
// TODO: hashmap, hashset, btreemap, btreeset, binaryheap?

impl<T: Dice, const N: usize> Dice for [T; N] {
    const USES_LIMIT: bool = T::USES_LIMIT;

    fn die() -> impl Die<Self> {
        dice::todo()
    }
}

macro_rules! impl_dice_for_tuple {
    ($($Ti:ident, $ti:ident, $die_i:ident)+) => {
        impl<$($Ti: Dice,)*> Dice for ($($Ti,)*) {
            const USES_LIMIT: bool = $($Ti::USES_LIMIT ||)* false;

            fn die() -> impl Die<Self> {
                let limit_part_count: usize = $($Ti::USES_LIMIT as usize +)* 0;
                $(let $die_i = $Ti::die();)*

                dice::from_fn(move |mut fate| {
                    if limit_part_count == 0 {
                        $(
                            let $ti = fate.with_limit(Limit(0)).roll(&$die_i);
                        )*

                        ($($ti,)*)
                    } else {
                        let limit = fate.limit();
                        let limit_parts_die = dice::split_limit_n(limit, limit_part_count);
                        let mut limit_parts = fate.with_limit(Limit(0)).roll(limit_parts_die);

                        $(
                            let $ti = if $Ti::USES_LIMIT {
                                let limit = limit_parts.pop().unwrap();
                                fate.with_limit(limit).roll(&$die_i)
                            } else {
                                fate.with_limit(Limit(0)).roll(&$die_i)
                            };
                        )*

                        ($($ti,)*)
                    }
                })
            }
        }
    };
}

impl_dice_for_tuple! {
    T1, t1, die_1
}
impl_dice_for_tuple! {
    T1, t1, die_1
    T2, t2, die_2
}

impl_dice_for_tuple! {
    T1, t1, die_1
    T2, t2, die_2
    T3, t3, die_3
}

impl_dice_for_tuple! {
    T1, t1, die_1
    T2, t2, die_2
    T3, t3, die_3
    T4, t4, die_4
}

impl_dice_for_tuple! {
    T1, t1, die_1
    T2, t2, die_2
    T3, t3, die_3
    T4, t4, die_4
    T5, t5, die_5
}

impl_dice_for_tuple! {
    T1, t1, die_1
    T2, t2, die_2
    T3, t3, die_3
    T4, t4, die_4
    T5, t5, die_5
    T6, t6, die_6
}

impl_dice_for_tuple! {
    T1, t1, die_1
    T2, t2, die_2
    T3, t3, die_3
    T4, t4, die_4
    T5, t5, die_5
    T6, t6, die_6
    T7, t7, die_7
}

impl_dice_for_tuple! {
    T1, t1, die_1
    T2, t2, die_2
    T3, t3, die_3
    T4, t4, die_4
    T5, t5, die_5
    T6, t6, die_6
    T7, t7, die_7
    T8, t8, die_8
}

impl_dice_for_tuple! {
    T1, t1, die_1
    T2, t2, die_2
    T3, t3, die_3
    T4, t4, die_4
    T5, t5, die_5
    T6, t6, die_6
    T7, t7, die_7
    T8, t8, die_8
    T9, t9, die_9
}

#[cfg(test)]
mod tests {
    use crate::{Limit, prelude::*};

    #[derive(Debug)]
    struct WithoutLimit {
        limit: Limit,
    }

    impl Dice for WithoutLimit {
        const USES_LIMIT: bool = false;

        fn die() -> impl Die<Self> {
            dice::from_fn(|fate| WithoutLimit {
                limit: fate.limit(),
            })
        }
    }

    #[derive(Debug)]
    struct WithLimit {
        limit: Limit,
    }

    impl Dice for WithLimit {
        const USES_LIMIT: bool = true;

        fn die() -> impl Die<Self> {
            dice::from_fn(|fate| WithLimit {
                limit: fate.limit(),
            })
        }
    }

    #[test]
    fn unit_uses_limit() {
        assert!(!<Box<()> as Dice>::USES_LIMIT);
    }

    #[test]
    fn u8_uses_limit() {
        assert!(!<Box<u8> as Dice>::USES_LIMIT);
    }

    #[test]
    fn box_uses_limit() {
        assert!(!<Box<WithoutLimit> as Dice>::USES_LIMIT);
        assert!(<Box<WithLimit> as Dice>::USES_LIMIT);
    }

    #[test]
    fn box_limit() {
        Dicetest::repeatedly().run(|mut fate| {
            let limit = fate.limit();

            {
                let elem: Box<WithoutLimit> = fate.roll(die());
                assert_eq!(elem.limit, limit);
            }

            {
                let elem: Box<WithLimit> = fate.roll(die());
                assert_eq!(elem.limit, limit);
            }
        })
    }

    #[test]
    fn vec_uses_limit() {
        assert!(<Vec<WithoutLimit> as Dice>::USES_LIMIT);
        assert!(<Vec<WithLimit> as Dice>::USES_LIMIT);
    }

    #[test]
    fn vec_limit() {
        Dicetest::repeatedly().run(|mut fate| {
            let limit = fate.limit();

            {
                let vec: Vec<WithoutLimit> = fate.roll(die());

                assert!(vec.len() <= limit.saturating_to_usize());

                for elem in vec {
                    assert_eq!(elem.limit, Limit(0));
                }
            }

            {
                let vec: Vec<WithLimit> = fate.roll(die());

                if !vec.is_empty() {
                    let total_size: u64 = vec.iter().map(|elem| elem.limit.0).sum();
                    assert_eq!(total_size, limit.0);
                }
            }
        })
    }

    #[test]
    fn tuple_uses_limit() {
        assert!(!<(WithoutLimit,) as Dice>::USES_LIMIT);
        assert!(<(WithLimit,) as Dice>::USES_LIMIT);

        assert!(!<(WithoutLimit, WithoutLimit) as Dice>::USES_LIMIT);
        assert!(<(WithoutLimit, WithLimit) as Dice>::USES_LIMIT);
        assert!(<(WithLimit, WithoutLimit) as Dice>::USES_LIMIT);
        assert!(<(WithLimit, WithLimit) as Dice>::USES_LIMIT);

        assert!(!<(WithoutLimit, WithoutLimit, WithoutLimit) as Dice>::USES_LIMIT);
        assert!(<(WithoutLimit, WithLimit, WithoutLimit) as Dice>::USES_LIMIT);
        assert!(<(WithLimit, WithoutLimit, WithoutLimit) as Dice>::USES_LIMIT);
        assert!(<(WithLimit, WithLimit, WithLimit) as Dice>::USES_LIMIT);
    }

    #[test]
    fn tuple_total_size() {
        Dicetest::repeatedly().run(|mut fate| {
            {
                let (a,): (String,) = fate.roll(die());
                let total_size = a.chars().count();
                assert!(total_size <= fate.limit().saturating_to_usize());
            }

            {
                let (a, b): (String, String) = fate.roll(die());
                let total_size = a.chars().count() + b.chars().count();
                assert!(total_size <= fate.limit().saturating_to_usize());
            }

            {
                let (a, b, c): (String, String, String) = fate.roll(die());
                let total_size = a.chars().count() + b.chars().count() + c.chars().count();
                assert!(total_size <= fate.limit().saturating_to_usize());
            }
        })
    }

    #[test]
    fn tuple_limit() {
        Dicetest::repeatedly().run(|mut fate| {
            let limit = fate.limit();

            {
                let (a, b, c): (WithoutLimit, WithoutLimit, WithoutLimit) = fate.roll(die());

                assert_eq!(a.limit, Limit(0));
                assert_eq!(b.limit, Limit(0));
                assert_eq!(c.limit, Limit(0));
            }

            {
                let (a, b, c, d): (WithLimit, WithoutLimit, WithLimit, WithoutLimit) =
                    fate.roll(die());

                assert!(a.limit <= limit);
                assert_eq!(b.limit, Limit(0));
                assert!(c.limit <= limit);
                assert_eq!(d.limit, Limit(0));

                assert_eq!(a.limit.0 + b.limit.0 + c.limit.0 + d.limit.0, limit.0)
            }
        })
    }
}
