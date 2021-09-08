use crate::prelude::*;

/// Intermediate result of [`dice::zip_once`].
///
/// [`dice::zip_once`]: dice::zip_once()
#[non_exhaustive]
pub struct ZipOnceArities;

macro_rules! zip_once_with_arity {
    ($arity:ident: $($Ti:ident, $die_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<$($Ti,)*>(
            self,
            $($die_i: impl DieOnce<$Ti>,)*
        ) -> impl DieOnce<($($Ti,)*)> {
            dice::from_fn_once(move |mut fate| {
                ($(fate.roll($die_i),)*)
            })
        }
    )
}

impl ZipOnceArities {
    zip_once_with_arity! { two:
        T1, die_1
        T2, die_2
    }

    zip_once_with_arity! { three:
        T1, die_1
        T2, die_2
        T3, die_3
    }

    zip_once_with_arity! { four:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
    }

    zip_once_with_arity! { five:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
    }

    zip_once_with_arity! { six:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
        T6, die_6
    }

    zip_once_with_arity! { seven:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
        T6, die_6
        T7, die_7
    }

    zip_once_with_arity! { eight:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
        T6, die_6
        T7, die_7
        T8, die_8
    }

    zip_once_with_arity! { nine:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
        T6, die_6
        T7, die_7
        T8, die_8
        T9, die_9
    }
}

/// Generates a tuple containing the generated values of several generators.
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
/// let zero_die = dice::just_once(0);
/// let one_die = dice::just_once(1.0);
/// let (zero, one) = fate.roll(dice::zip_once().two(zero_die, one_die));
/// assert_eq!(zero, 0);
/// assert_eq!(one, 1.0);
/// ```
pub fn zip_once() -> ZipOnceArities {
    ZipOnceArities {}
}

/// Intermediate result of [`dice::zip`].
///
/// [`dice::zip`]: dice::zip()
#[non_exhaustive]
pub struct ZipArities;

macro_rules! zip_with_arity {
    ($arity:ident: $($Ti:ident, $die_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<$($Ti,)*>(
            self,
            $($die_i: impl Die<$Ti>,)*
        ) -> impl Die<($($Ti,)*)> {
            dice::from_fn(move |mut fate| {
                ($(fate.roll(&$die_i),)*)
            })
        }
    )
}

impl ZipArities {
    zip_with_arity! { two:
        T1, die_1
        T2, die_2
    }

    zip_with_arity! { three:
        T1, die_1
        T2, die_2
        T3, die_3
    }

    zip_with_arity! { four:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
    }

    zip_with_arity! { five:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
    }

    zip_with_arity! { six:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
        T6, die_6
    }

    zip_with_arity! { seven:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
        T6, die_6
        T7, die_7
    }

    zip_with_arity! { eight:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
        T6, die_6
        T7, die_7
        T8, die_8
    }

    zip_with_arity! { nine:
        T1, die_1
        T2, die_2
        T3, die_3
        T4, die_4
        T5, die_5
        T6, die_6
        T7, die_7
        T8, die_8
        T9, die_9
    }
}

/// Generates a tuple containing the generated values of several generators.
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
/// let zero_die = dice::just(0);
/// let one_die = dice::just(1.0);
/// let (zero, one) = fate.roll(dice::zip().two(zero_die, one_die));
/// assert_eq!(zero, 0);
/// assert_eq!(one, 1.0);
/// ```
pub fn zip() -> ZipArities {
    ZipArities {}
}
