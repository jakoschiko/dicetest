use crate::gen::Limit;

/// Generates a series of interpolated `Limit`s.
#[derive(Debug, Clone)]
pub struct LimitSeries {
    start: u64,
    end: u64,
    diff: u64,
    len: u64,
}

impl LimitSeries {
    /// Creates a new instance that produces `len` linearly interpolated `Limit`s between `start`
    /// and `end`.
    pub fn new(start: u64, end: u64, len: u64) -> Self {
        let diff = if start <= end {
            end - start
        } else {
            start - end
        };
        LimitSeries {
            start,
            end,
            diff,
            len,
        }
    }

    /// Returns the n-th interpolated `Limit` or `None` if `n` is out of bounds.
    pub fn nth(&self, n: u64) -> Option<Limit> {
        if n >= self.len {
            None
        } else if self.len == 1 {
            Some(Limit(self.start))
        } else if self.start <= self.end {
            Some(self.interpolate(n, self.start))
        } else {
            Some(self.interpolate(self.len - n - 1, self.end))
        }
    }

    fn interpolate(&self, x: u64, offset_y: u64) -> Limit {
        let delta_x = u128::from(self.len) - 1;
        let delta_y = u128::from(self.diff);

        let ipol_y = ((u128::from(x) * delta_y) / delta_x) as u64;

        Limit(offset_y + ipol_y)
    }

    /// Returns an interator that emits all `Limit`s.
    pub fn into_iter(self) -> impl Iterator<Item = Limit> {
        LimitSeriesIntoIter {
            series: self,
            idx: 0,
        }
    }
}

struct LimitSeriesIntoIter {
    series: LimitSeries,
    idx: u64,
}

impl Iterator for LimitSeriesIntoIter {
    type Item = Limit;

    fn next(&mut self) -> Option<Self::Item> {
        let lim = self.series.nth(self.idx);
        if lim.is_some() {
            self.idx += 1;
        }
        lim
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;
    use crate::runner::LimitSeries;

    fn assert_example(start: u64, end: u64, len: u64, expected_limits: Vec<u64>) {
        let series = LimitSeries::new(start, end, len);
        let actual_limits = series.into_iter().map(|limit| limit.0).collect::<Vec<_>>();

        assert_eq!(actual_limits, expected_limits);
    }

    #[test]
    fn examples() {
        assert_example(0, 2, 2, vec![0, 2]);
        assert_example(2, 0, 2, vec![2, 0]);
        assert_example(0, 2, 3, vec![0, 1, 2]);
        assert_example(2, 0, 3, vec![2, 1, 0]);
    }

    #[test]
    fn iterator_produces_exact_len_limits() {
        dicetest!(|fate| {
            let start = gens::u64(..).gen(fate);
            let end = gens::u64(..).gen(fate);
            let len = gens::u64(..=fate.limit().0).gen(fate);

            let series = LimitSeries::new(start, end, len);
            let iter = series.into_iter();
            let iter_len: u64 = iter.map(|_| 1).sum();

            assert_eq!(iter_len, len);
        })
    }

    #[test]
    fn if_len_gt_1_then_start_is_first_limit() {
        dicetest!(|fate| {
            let start = gens::u64(..).gen(fate);
            let end = gens::u64(..).gen(fate);
            let len = gens::u64(1..).gen(fate);

            hint!(start);
            hint!(end);
            hint!(len);

            let series = LimitSeries::new(start, end, len);
            let first_limit = series.nth(0).unwrap().0;

            assert_eq!(first_limit, start);
        })
    }

    #[test]
    fn if_len_is_gt_2_then_end_is_last_limit() {
        dicetest!(|fate| {
            let start = gens::u64(..).gen(fate);
            let end = gens::u64(..).gen(fate);
            let len = gens::u64(2..).gen(fate);

            hint!(start);
            hint!(end);
            hint!(len);

            let series = LimitSeries::new(start, end, len);
            let last_limit = series.nth(len - 1).unwrap().0;

            assert_eq!(last_limit, end);
        })
    }
}
