use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::fmt::Debug;

use crate::prelude::gens::*;

/// Non-empty range for `gens::size`.
pub trait SizeRange: Clone + Debug {
    /// Returns the inclusive bounds `(lower, upper)` that represent the range. They must hold
    /// `lower <= upper`. If the upper bound is open, it will be limited with `Limit`.
    ///
    /// # Panics
    /// Panics if the range cannot be represented as `(lower, upper)` with `lower <= upper`.
    fn bounds(&self, lim: Limit) -> (usize, usize);
}

fn empty_size_range(range: &(impl SizeRange + Debug)) -> ! {
    panic!("SizeRange is invalid because it contains no values: {:?}", range)
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
        let upper = lower.saturating_add(lim.saturating_usize());
        (lower, upper)
    }
}

impl SizeRange for RangeFull {
    fn bounds(&self, lim: Limit) -> (usize, usize) {
        (0, lim.saturating_usize())
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
/// given range and the `Limit` parameter passed to `Gen::gen`.
///
/// # Panics
/// Panics if the range is invalid, see `SizeRange::bounds`.
pub fn size(range: impl SizeRange) -> impl Gen<usize> {
    gens::from_fn(move |dice| {
        let (lower, upper) = range.bounds(dice.limit());
        gens::uni_usize(lower..=upper).gen(dice)
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::prelude::tests::*;
    use crate::gen::{Limit, Dice};

    fn range_contains_size<B, R>(
        dice: &mut Dice,
        range_data_gen: impl GenOnce<B>,
        create_range: impl FnOnce(B) -> R,
        is_in_range: impl FnOnce(B, usize) -> bool,
    )
    where
        B: Copy + Debug,
        R: gens::SizeRange + Debug,
    {
        let mut prng = gens::prng_fork().gen(dice);
        let limit = gens::u64(..).gen(dice);
        let range_data = range_data_gen.gen_once(dice);

        hint!(prng);
        hint!(limit);
        hint!(range_data);

        let mut dice = Dice::new(&mut prng, Limit(limit));

        let range = create_range(range_data);
        hint!(range);

        let size = gens::size(range).gen(&mut dice);
        hint!(size);

        assert!(is_in_range(range_data, size));
    }

    #[test]
    fn size_is_equal_to_target() {
        dicetest!(|dice| {
            range_contains_size(
                dice,
                gens::usize(..),
                |target| target,
                |target, size| size == target,
            );
        })
    }

    #[test]
    fn size_is_in_range() {
        dicetest!(|dice| {
            range_contains_size(
                dice,
                gens::array_2(gens::usize(..usize::max_value() - 1))
                    .map(|[a, b]| (a.min(b), a.max(b) + 1)),
                |(lower, upper)| lower..upper,
                |(lower, upper), size| lower <= size && size < upper,
            );
        })
    }

    #[test]
    fn size_is_in_range_from() {
        dicetest!(|dice| {
            range_contains_size(
                dice,
                gens::usize(..),
                |lower| lower..,
                |lower, size| lower <= size,
            );
        })
    }

    #[test]
    fn size_is_in_range_inclusive() {
        dicetest!(|dice| {
            range_contains_size(
                dice,
                gens::array_2(gens::usize(..))
                    .map(|[a, b]| (a.min(b), a.max(b))),
                |(lower, upper)| lower..=upper,
                |(lower, upper), size| lower <= size && size <= upper,
            );
        })
    }

    #[test]
    fn size_is_in_range_to() {
        dicetest!(|dice| {
            range_contains_size(
                dice,
                gens::usize(1..),
                |upper| ..upper,
                |upper, size| size < upper,
            );
        })
    }

    #[test]
    fn size_is_in_range_to_inclusive() {
        dicetest!(|dice| {
            range_contains_size(
                dice,
                gens::usize(..),
                |upper| ..=upper,
                |upper, size| size <= upper,
            );
        })
    }

}
