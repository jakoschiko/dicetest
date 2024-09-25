use std::fmt::Debug;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::prelude::*;

/// Non-empty range for [`dice::length`].
///
/// [`dice::length`]: dice::length()
pub trait LengthRange {
    /// Returns the inclusive lower bound and the optional inclusive upper bound that represent
    /// the range.
    ///
    /// # Panics
    ///
    /// Panics if the range is empty.
    fn bounds(self) -> (usize, Option<usize>);
}

fn empty_length_range(range: &(impl LengthRange + Debug)) -> ! {
    panic!(
        "LengthRange is invalid because it contains no values: {:?}",
        range
    )
}

impl LengthRange for usize {
    fn bounds(self) -> (usize, Option<usize>) {
        (self, Some(self))
    }
}

impl LengthRange for Range<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        if self.start < self.end {
            let lower = self.start;
            let upper = self.end - 1;
            (lower, Some(upper))
        } else {
            empty_length_range(&self);
        }
    }
}

impl LengthRange for RangeFrom<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        (self.start, None)
    }
}

impl LengthRange for RangeFull {
    fn bounds(self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl LengthRange for RangeInclusive<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        if self.start() <= self.end() {
            let (lower, upper) = self.into_inner();
            (lower, Some(upper))
        } else {
            empty_length_range(&self);
        }
    }
}

impl LengthRange for RangeTo<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        if self.end > 0 {
            let lower = 0;
            let upper = self.end - 1;
            (lower, Some(upper))
        } else {
            empty_length_range(&self);
        }
    }
}

impl LengthRange for RangeToInclusive<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        (0, Some(self.end))
    }
}

/// Generates a random length that can be used for collections, etc. The length is bounded by the
/// given range and the [`Limit`] parameter passed to [`Die::roll`].
///
/// [`Limit`]: crate::Limit
///
/// # Panics
///
/// Panics if the range is empty.
///
/// # Examples
///
/// This example generates lengths without panicking:
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// assert!(fate.roll(dice::length(42)) == 42);
///
/// let length = fate.with_limit(100.into()).roll(dice::length(42..));
/// assert!(length >= 42 && length <= 142);
///
/// assert!(fate.roll(dice::length(..=71)) <= 71);
///
/// assert!(fate.roll(dice::length(..71)) < 71);
///
/// let length = fate.roll(dice::length(42..=71));
/// assert!(length >= 42 && length <= 71);
///
/// let length = fate.roll(dice::length(42..71));
/// assert!(length >= 42 && length < 71);
///
/// let length = fate.with_limit(100.into()).roll(dice::length(..));
/// assert!(length >= 0 && length <= 100);
/// ```
///
/// This example panics:
///
/// ```should_panic
/// use dicetest::prelude::*;
///
/// // Oh no, panic!
/// let _length_die = dice::length(71..42);
/// ```
pub fn length(range: impl LengthRange) -> impl Die<usize> {
    let (lower, upper_opt) = range.bounds();

    dice::from_fn(move |mut fate| {
        let upper =
            upper_opt.unwrap_or_else(|| lower.saturating_add(fate.limit().saturating_to_usize()));

        fate.roll(dice::uni_usize(lower..=upper))
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::prelude::*;

    fn range_contains_length<B, R>(
        mut fate: Fate,
        range_data_die: impl DieOnce<B>,
        create_range: impl FnOnce(B) -> R,
        is_in_range: impl FnOnce(B, usize) -> bool,
    ) where
        B: Copy + Debug,
        R: dice::LengthRange + Debug,
    {
        let prng = &mut fate.fork_prng();
        let limit = fate.roll(dice::u64(..)).into();
        let range_data = fate.roll(range_data_die);

        hint_debug!(prng);
        hint_debug!(limit);
        hint_debug!(range_data);

        let range = create_range(range_data);
        hint_debug!(range);

        let length = dice::length(range).roll(Fate::new(prng, limit));
        hint_debug!(length);

        assert!(is_in_range(range_data, length));
    }

    #[test]
    fn length_is_equal_to_target() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_length(
                fate,
                dice::usize(..),
                |target| target,
                |target, length| length == target,
            );
        })
    }

    #[test]
    fn length_is_in_range() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_length(
                fate,
                dice::array(dice::usize(..usize::MAX - 1)).map(|[a, b]| (a.min(b), a.max(b) + 1)),
                |(lower, upper)| lower..upper,
                |(lower, upper), length| lower <= length && length < upper,
            );
        })
    }

    #[test]
    fn length_is_in_range_from() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_length(
                fate,
                dice::usize(..),
                |lower| lower..,
                |lower, length| lower <= length,
            );
        })
    }

    #[test]
    fn length_is_in_range_inclusive() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_length(
                fate,
                dice::array(dice::usize(..)).map(|[a, b]| (a.min(b), a.max(b))),
                |(lower, upper)| lower..=upper,
                |(lower, upper), length| lower <= length && length <= upper,
            );
        })
    }

    #[test]
    fn length_is_in_range_to() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_length(
                fate,
                dice::usize(1..),
                |upper| ..upper,
                |upper, length| length < upper,
            );
        })
    }

    #[test]
    fn length_is_in_range_to_inclusive() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_length(
                fate,
                dice::usize(..),
                |upper| ..=upper,
                |upper, length| length <= upper,
            );
        })
    }
}
