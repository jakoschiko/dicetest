#[derive(Debug, Clone)]
pub struct SizeSeries {
    start: u64,
    end: u64,
    diff: u64,
    len: u64,
}

impl SizeSeries {
    pub fn new(start: u64, end: u64, len: u64) -> Self {
        let diff = if start <= end { end - start } else { start - end };
        SizeSeries { start, end, diff, len }
    }

    pub fn nth(&self, n: u64) -> Option<u64> {
        if n >= self.len {
            None
        } else if self.start <= self.end {
            Some(self.nth_with_min(n, self.start))
        } else {
            Some(self.nth_with_min(self.len - n, self.end))
        }
    }

    fn nth_with_min(&self, n: u64, min: u64) -> u64 {
        let delta = ((n as u128 * self.diff as u128) / self.len as u128) as u64;
        min + delta
    }

    pub fn into_iter(self) -> impl Iterator<Item=u64> {
        SizeSeriesIntoIter {
            series: self,
            idx: 0,
        }
    }
}

struct SizeSeriesIntoIter {
    series: SizeSeries,
    idx: u64,
}

impl Iterator for SizeSeriesIntoIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.series.nth(self.idx);
        if size.is_some() {
            self.idx += 1;
        }
        size
    }
}
