use std::fmt::Debug;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::prelude::dice::*;

/// Non-empty range for `dice::size`.
pub trait SizeRange: Clone + Debug {
    /// Returns the inclusive bounds `(lower, upper)` that represent the range. They must hold
    /// `lower <= upper`. If the upper bound is open, it will be limited with `Limit`.
    ///
    /// # Panics
    /// Panics if the range cannot be represented as `(lower, upper)` with `lower <= upper`.
    fn bounds(&self, lim: Limit) -> (usize, usize);
}

fn empty_size_range(range: &(impl SizeRange + Debug)) -> ! {
    panic!(
        "SizeRange is invalid because it contains no values: {:?}",
        range
    )
}

impl SizeRange for usize {
    fn bounds(&self, _lim: Limit) -> (usize, usize) {
        (*self, *self)
    }
}

impl SizeRange for Range<usize> {
    fn bounds(&self, _lim: Limit) -> (usize, usize) {
        if self.start < self.end {
            let lower = self.start;
            let upper = self.end - 1;
            (lower, upper)
        } else {
            empty_size_range(self);
        }
    }
}

impl SizeRange for RangeFrom<usize> {
    fn bounds(&self, lim: Limit) -> (usize, usize) {
        let lower = self.start;
        let upper = lower.saturating_add(lim.saturating_to_usize());
        (lower, upper)
    }
}

impl SizeRange for RangeFull {
    fn bounds(&self, lim: Limit) -> (usize, usize) {
        (0, lim.saturating_to_usize())
    }
}

impl SizeRange for RangeInclusive<usize> {
    fn bounds(&self, _lim: Limit) -> (usize, usize) {
        let lower = *self.start();
        let upper = *self.end();
        if lower <= upper {
            (lower, upper)
        } else {
            empty_size_range(self);
        }
    }
}

impl SizeRange for RangeTo<usize> {
    fn bounds(&self, _lim: Limit) -> (usize, usize) {
        if self.end > 0 {
            let lower = 0;
            let upper = self.end - 1;
            (lower, upper)
        } else {
            empty_size_range(self);
        }
    }
}

impl SizeRange for RangeToInclusive<usize> {
    fn bounds(&self, _lim: Limit) -> (usize, usize) {
        (0, self.end)
    }
}

/// Generates a random size that can be used for collections, etc. The size is bounded by the
/// given range and the `Limit` parameter passed to `Die::roll`.
///
/// # Panics
/// Panics if the range is invalid, see `SizeRange::bounds`.
pub fn size(range: impl SizeRange) -> impl Die<usize> {
    dice::from_fn(move |fate| {
        let (lower, upper) = range.bounds(fate.limit());
        dice::uni_usize(lower..=upper).roll(fate)
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::die::{Fate, Limit};
    use crate::prelude::tests::*;

    fn range_contains_size<B, R>(
        fate: &mut Fate,
        range_data_die: impl DieOnce<B>,
        create_range: impl FnOnce(B) -> R,
        is_in_range: impl FnOnce(B, usize) -> bool,
    ) where
        B: Copy + Debug,
        R: dice::SizeRange + Debug,
    {
        let mut prng = dice::prng_fork().roll(fate);
        let limit = dice::u64(..).roll(fate);
        let range_data = range_data_die.roll_once(fate);

        hint_debug!(prng);
        hint_debug!(limit);
        hint_debug!(range_data);

        let mut fate = Fate::new(&mut prng, Limit(limit));

        let range = create_range(range_data);
        hint_debug!(range);

        let size = dice::size(range).roll(&mut fate);
        hint_debug!(size);

        assert!(is_in_range(range_data, size));
    }

    #[test]
    fn size_is_equal_to_target() {
        dicetest!(|fate| {
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
        dicetest!(|fate| {
            range_contains_size(
                fate,
                dice::array_2(dice::usize(..usize::max_value() - 1))
                    .map(|[a, b]| (a.min(b), a.max(b) + 1)),
                |(lower, upper)| lower..upper,
                |(lower, upper), size| lower <= size && size < upper,
            );
        })
    }

    #[test]
    fn size_is_in_range_from() {
        dicetest!(|fate| {
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
        dicetest!(|fate| {
            range_contains_size(
                fate,
                dice::array_2(dice::usize(..)).map(|[a, b]| (a.min(b), a.max(b))),
                |(lower, upper)| lower..=upper,
                |(lower, upper), size| lower <= size && size <= upper,
            );
        })
    }

    #[test]
    fn size_is_in_range_to() {
        dicetest!(|fate| {
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
        dicetest!(|fate| {
            range_contains_size(
                fate,
                dice::usize(..),
                |upper| ..=upper,
                |upper, size| size <= upper,
            );
        })
    }

}
