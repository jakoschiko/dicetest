/// Helper for splitting a number into portions of similar sizes.
#[derive(Debug, Clone)]
pub struct Portions {
    total: u64,
    count: u64,
}

impl Portions {
    /// Creates an instance that splits `total` into `count` portions of similar sizes.
    pub fn new(total: u64, count: u64) -> Self {
        Portions { total, count }
    }

    /// Returns the n-th portion or `None` if `n` is out of bounds.
    pub fn nth(&self, n: u64) -> Option<u64> {
        if n >= self.count {
            None
        } else {
            Some((self.start(n + 1) - self.start(n)) as u64)
        }
    }

    fn start(&self, n: u64) -> u128 {
        (self.total as u128 * n as u128) / self.count as u128
    }

    /// Returns an iterator that emits all portions.
    pub fn into_iter(self) -> impl Iterator<Item=u64> {
        PortionsIntoIter {
            portions: self,
            idx: 0,
        }
    }
}

struct PortionsIntoIter {
    portions: Portions,
    idx: u64,
}

impl Iterator for PortionsIntoIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.portions.nth(self.idx);
        if size.is_some() {
            self.idx += 1;
        }
        size
    }
}
