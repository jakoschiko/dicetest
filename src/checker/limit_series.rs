use ::gen::Limit;

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
        let diff = if start <= end { end - start } else { start - end };
        LimitSeries { start, end, diff, len }
    }

    /// Returns the n-th interpolated `Limit` or `None` if `n` is out of bounds.
    pub fn nth(&self, n: u64) -> Option<Limit> {
        if n >= self.len {
            None
        } else if self.start <= self.end {
            Some(self.nth_with_min(n, self.start))
        } else {
            Some(self.nth_with_min(self.len - n, self.end))
        }
    }

    fn nth_with_min(&self, n: u64, min: u64) -> Limit {
        let delta = ((n as u128 * self.diff as u128) / self.len as u128) as u64;
        Limit(min + delta)
    }

    /// Returns an interator that emits all `Limit`s.
    pub fn into_iter(self) -> impl Iterator<Item=Limit> {
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
