use std::collections::{BTreeMap, btree_map::Entry};

/// A counter for occurrences of a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Occurence {
    /// The counter has overflowed.
    Overflow,
    /// Contains the current count.
    Value(u64)
}

impl Occurence {
    /// Returns an initial counter.
    pub fn new() -> Self {
        Occurence::Value(0)
    }

    /// Increments the counter by one.
    pub fn inc(self) -> Self {
        match self {
            Occurence::Overflow => Occurence::Overflow,
            Occurence::Value(n) => {
                let incremented = n.checked_add(1);
                incremented.map_or(Occurence::Overflow, Occurence::Value)
            },
        }
    }

    /// Sums up both counters.
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Occurence::Value(left), Occurence::Value(right)) => {
                let sum = left.checked_add(right);
                sum.map_or(Occurence::Overflow, Occurence::Value)
            }
            _ => Occurence::Overflow,
        }
    }

    // Returns the count as option.
    pub fn value(self) -> Option<u64> {
        match self {
            Occurence::Overflow => None,
            Occurence::Value(n) => Some(n),
        }
    }
}

/// Contains the occurence counters of different values with the same key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stat(pub BTreeMap<String, Occurence>);

impl Stat {
    /// Returns an instance without any counters.
    pub fn new() -> Self {
        let occurrences = BTreeMap::new();
        Stat(occurrences)
    }

    // Increases the counter for the given value by one.
    pub fn inc(&mut self, value: String) {
        let occurence_entry = self.0
            .entry(value)
            .or_insert(Occurence::new());
        *occurence_entry = occurence_entry.inc();
    }

    /// Merges both instances by merging the counters that belong to same value.
    pub fn merge(mut self, other: Self) -> Self {
        if self.0.len() < other.0.len() {
            other.merge(self)
        } else {
            for (value, right_occurence) in other.0.into_iter() {
                match self.0.entry(value) {
                    Entry::Vacant(entry) => {
                        entry.insert(right_occurence);
                    }
                    Entry::Occupied(entry) => {
                        let left_occurence = *entry.get();
                        *entry.into_mut() = left_occurence.merge(right_occurence);
                    }
                }
            }

            self
        }
    }

    // Returns the total occurrence count for all keys.
    pub fn total_occurence(&self) -> Occurence {
        self.0.values().fold(Occurence::new(), |left, &right| left.merge(right))
    }
}

/// Contains the stats for different keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stats(pub BTreeMap<&'static str, Stat>);

impl Stats {
    /// Returns an instance without any stats.
    pub fn new() -> Self {
        Stats(BTreeMap::new())
    }

    // Takes ownership of the stats.
    pub fn take(&mut self) -> Self {
        let first_key = self.0.keys().next().map(|k| *k);
        match first_key {
            None => Stats::new(),
            Some(first_key) => {
                let stats = self.0.split_off(first_key);
                Stats(stats)
            }
        }
    }

    // Increases the counter for the given key and value by one.
    pub fn inc(&mut self, key: &'static str, value: String) {
        let stat_entry = self.0.entry(key).or_insert_with(|| Stat::new());
        stat_entry.inc(value);
    }

    /// Merges both instances by merging the stats that belong to the same key.
    pub fn merge(mut self, other: Self) -> Self {
        if self.0.len() < other.0.len() {
            other.merge(self)
        } else {
            for (key, right_stat) in other.0.into_iter() {
                let left_stat = self.0.remove(key);
                let stat = match left_stat {
                    None => right_stat,
                    Some(left_stat) => left_stat.merge(right_stat),
                };
                self.0.insert(key, stat);
            }

            self
        }
    }

    /// Returns a `String` that contains all stats in a pretty format.
    pub fn pretty(&self) -> String {
        let mut acc = String::new();
        let mut iter = self.0.iter();

        let add_stat = |acc: &mut String, key: &'static str, stat: &Stat| {
            acc.push_str(key);
            acc.push(':');

            let total = stat.total_occurence().value().filter(|&n| n != 0);

            let mut occurrences = stat.0
                .iter()
                .collect::<Vec<_>>();

            occurrences.sort_by_key(|(_, &occ)| occ);

            for (value, occurrence) in occurrences.iter().rev() {
                let overflow = || "ovf".to_string();
                let n = occurrence.value();

                let percent_str = {
                    let numerator = n.and_then(|n| n.checked_mul(100));

                    match (numerator, total) {
                        (Some(numerator), Some(total)) => format!("{:3}", numerator / total),
                        _ => overflow(),
                    }
                };
                let occurrence_str = n.map_or_else(overflow, |n| format!("{}", n));

                acc.push_str("\n\t");
                acc.push_str(&percent_str);
                acc.push_str("% (");
                acc.push_str(&occurrence_str);
                acc.push_str("): ");
                acc.push_str(&value);
            }
        };

        if let Some((key, stat)) = iter.next() {
            add_stat(&mut acc, key, stat);
        }

        for (key, stat) in iter {
            acc.push('\n');
            add_stat(&mut acc, key, stat);
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::counter::{Occurence::{self, Value, Overflow}, Stat, Stats};

    #[test]
    fn occurence_inc_examples() {
        assert_eq!(Occurence::new().inc(), Value(1));
        assert_eq!(Value(u64::max_value()).inc(), Overflow);
    }

    #[test]
    fn occurence_merge_examples() {
        assert_eq!(Overflow.merge(Overflow), Overflow);
        assert_eq!(Overflow.merge(Occurence::new()), Overflow);
        assert_eq!(Occurence::new().merge(Overflow), Overflow);
        assert_eq!(Value(1).merge(Value(1)), Value(2));
    }

    #[test]
    fn stats_take_takes_all_elements() {
        let mut stat1 = Stat::new();
        let mut stat2 = Stat::new();
        let mut stats = Stats::new();

        stat1.0.insert("foofoo".to_string(), Value(1));
        stat2.0.insert("barbar".to_string(), Overflow);
        stats.0.insert("foo", stat1);
        stats.0.insert("bar", stat2);

        assert_eq!(stats, stats.clone().take());
    }
}
