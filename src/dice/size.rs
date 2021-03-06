use std::fmt::Debug;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::prelude::*;

/// Non-empty range for [`dice::size`].
///
/// [`dice::size`]: dice::size()
pub trait SizeRange {
    /// Returns the inclusive lower bound and the optional inclusive upper bound that represent
    /// the range.
    ///
    /// # Panics
    ///
    /// Panics if the range is empty.
    fn bounds(self) -> (usize, Option<usize>);
}

fn empty_size_range(range: &(impl SizeRange + Debug)) -> ! {
    panic!(
        "SizeRange is invalid because it contains no values: {:?}",
        range
    )
}

impl SizeRange for usize {
    fn bounds(self) -> (usize, Option<usize>) {
        (self, Some(self))
    }
}

impl SizeRange for Range<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        if self.start < self.end {
            let lower = self.start;
            let upper = self.end - 1;
            (lower, Some(upper))
        } else {
            empty_size_range(&self);
        }
    }
}

impl SizeRange for RangeFrom<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        (self.start, None)
    }
}

impl SizeRange for RangeFull {
    fn bounds(self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl SizeRange for RangeInclusive<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        if self.start() <= self.end() {
            let (lower, upper) = self.into_inner();
            (lower, Some(upper))
        } else {
            empty_size_range(&self);
        }
    }
}

impl SizeRange for RangeTo<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        if self.end > 0 {
            let lower = 0;
            let upper = self.end - 1;
            (lower, Some(upper))
        } else {
            empty_size_range(&self);
        }
    }
}

impl SizeRange for RangeToInclusive<usize> {
    fn bounds(self) -> (usize, Option<usize>) {
        (0, Some(self.end))
    }
}

/// Generates a random size that can be used for collections, etc. The size is bounded by the
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
/// This example generates sizes without panicking:
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// assert!(fate.roll(dice::size(42)) == 42);
///
/// let size = fate.with_limit(100.into()).roll(dice::size(42..));
/// assert!(size >= 42 && size <= 142);
///
/// assert!(fate.roll(dice::size(..=71)) <= 71);
///
/// assert!(fate.roll(dice::size(..71)) < 71);
///
/// let size = fate.roll(dice::size(42..=71));
/// assert!(size >= 42 && size <= 71);
///
/// let size = fate.roll(dice::size(42..71));
/// assert!(size >= 42 && size < 71);
///
/// let size = fate.with_limit(100.into()).roll(dice::size(..));
/// assert!(size >= 0 && size <= 100);
/// ```
///
/// This example panics:
///
/// ```should_panic
/// use dicetest::prelude::*;
///
/// // Oh no, panic!
/// let _size_die = dice::size(71..42);
/// ```
pub fn size(range: impl SizeRange) -> impl Die<usize> {
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

    fn range_contains_size<B, R>(
        mut fate: Fate,
        range_data_die: impl DieOnce<B>,
        create_range: impl FnOnce(B) -> R,
        is_in_range: impl FnOnce(B, usize) -> bool,
    ) where
        B: Copy + Debug,
        R: dice::SizeRange + Debug,
    {
        let prng = &mut fate.fork_prng();
        let limit = fate.roll(dice::u64(..)).into();
        let range_data = fate.roll(range_data_die);

        hint_debug!(prng);
        hint_debug!(limit);
        hint_debug!(range_data);

        let range = create_range(range_data);
        hint_debug!(range);

        let size = dice::size(range).roll(Fate::new(prng, limit));
        hint_debug!(size);

        assert!(is_in_range(range_data, size));
    }

    #[test]
    fn size_is_equal_to_target() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_size(
                fate,
                dice::usize(..),
                |target| target,
                |target, size| size == target,
            );
        })
    }

    #[test]
    fn size_is_in_range() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_size(
                fate,
                dice::array(dice::usize(..usize::max_value() - 1))
                    .map(|[a, b]| (a.min(b), a.max(b) + 1)),
                |(lower, upper)| lower..upper,
                |(lower, upper), size| lower <= size && size < upper,
            );
        })
    }

    #[test]
    fn size_is_in_range_from() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_size(
                fate,
                dice::usize(..),
                |lower| lower..,
                |lower, size| lower <= size,
            );
        })
    }

    #[test]
    fn size_is_in_range_inclusive() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_size(
                fate,
                dice::array(dice::usize(..)).map(|[a, b]| (a.min(b), a.max(b))),
                |(lower, upper)| lower..=upper,
                |(lower, upper), size| lower <= size && size <= upper,
            );
        })
    }

    #[test]
    fn size_is_in_range_to() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_size(
                fate,
                dice::usize(1..),
                |upper| ..upper,
                |upper, size| size < upper,
            );
        })
    }

    #[test]
    fn size_is_in_range_to_inclusive() {
        Dicetest::repeatedly().run(|fate| {
            range_contains_size(
                fate,
                dice::usize(..),
                |upper| ..=upper,
                |upper, size| size <= upper,
            );
        })
    }
}
