#[derive(Debug, Clone)]
pub struct Portions {
    pub total: u64,
    pub count: u64,
}

impl Portions {
    pub fn nth(&self, n: u64) -> u64 {
        (self.start(n + 1) - self.start(n)) as u64
    }

    fn start(&self, n: u64) -> u128 {
        (self.total as u128 * n as u128) / self.count as u128
    }

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
        if self.idx >= self.portions.count {
            None
        } else {
            let portion = self.portions.nth(self.idx);
            self.idx += 1;
            Some(portion)
        }
    }
}
