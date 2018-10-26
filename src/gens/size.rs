use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::fmt::Debug;

use ::prelude::gens::*;

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
    gens::reveal_limit().flat_map(move |lim| {
        let (lower, upper) = range.bounds(lim);
        gens::usize_uniform(lower..=upper)
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use ::prelude::tests::*;
    use ::gen::Limit;

    fn range_contains_size_prop<B, R>(
        range_data_gen: impl GenOnce<B>,
        create_range: impl FnOnce(B) -> R,
        is_in_range: impl FnOnce(B, usize) -> bool,
    ) -> impl Prop
    where
        B: Copy + Debug,
        R: gens::SizeRange + Debug,
    {
        props::forall_3(
            gens::rng_fork().name("rng"),
            gens::u64(..).name("limit"),
            range_data_gen.name("range_data"),
            |log, mut rng, limit, range_data| {
                let range = create_range(range_data);
                log.print(|| format!("range: {:?}", range));
                let size = gens::size(range).gen(&mut rng, Limit(limit));
                log.print(|| format!("size: {}", size));
                is_in_range(range_data, size)
            }
        )
    }

    #[test]
    fn size_is_equal_to_target() {
        assert_prop(|| {
            range_contains_size_prop(
                gens::usize(..),
                |target| target,
                |target, size| size == target,
            ).dyn()
        })
    }

    #[test]
    fn size_is_in_range() {
        assert_prop(|| {
            range_contains_size_prop(
                gens::array_2(gens::usize(..usize::max_value() - 1))
                    .map(|[a, b]| (a.min(b), a.max(b) + 1)),
                |(lower, upper)| lower..upper,
                |(lower, upper), size| lower <= size && size < upper,
            ).dyn()
        })
    }

    #[test]
    fn size_is_in_range_from() {
        assert_prop(|| {
            range_contains_size_prop(
                gens::usize(..),
                |lower| lower..,
                |lower, size| lower <= size,
            ).dyn()
        })
    }

    #[test]
    fn size_is_in_range_inclusive() {
        assert_prop(|| {
            range_contains_size_prop(
                gens::array_2(gens::usize(..))
                    .map(|[a, b]| (a.min(b), a.max(b))),
                |(lower, upper)| lower..=upper,
                |(lower, upper), size| lower <= size && size <= upper,
            ).dyn()
        })
    }

    #[test]
    fn size_is_in_range_to() {
        assert_prop(|| {
            range_contains_size_prop(
                gens::usize(1..),
                |upper| ..upper,
                |upper, size| size < upper,
            ).dyn()
        })
    }

    #[test]
    fn size_is_in_range_to_inclusive() {
        assert_prop(|| {
            range_contains_size_prop(
                gens::usize(..),
                |upper| ..=upper,
                |upper, size| size <= upper,
            ).dyn()
        })
    }

}
