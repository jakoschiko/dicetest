//! Stats helps you to analyze repeated test runs.
//!
//! For any key you can count the occurrences of its values. Use it to reveal the
//! distribution of generated test data or the probability of branches.
//! Stats must enabled with the feature `stats`.

use std::collections::{btree_map::Entry, BTreeMap};

#[cfg(feature = "stats")]
use crate::util::events;

/// A counter for occurrences of a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Counter {
    /// The counter has overflowed.
    Overflow,
    /// Contains the current count.
    Value(u64),
}

impl Counter {
    /// Returns an initial counter.
    pub fn new() -> Self {
        Counter::Value(0)
    }

    /// Increments the counter by one.
    pub fn inc(self) -> Self {
        match self {
            Counter::Overflow => Counter::Overflow,
            Counter::Value(n) => {
                let incremented = n.checked_add(1);
                incremented.map_or(Counter::Overflow, Counter::Value)
            }
        }
    }

    /// Sums up both counters.
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Counter::Value(left), Counter::Value(right)) => {
                let sum = left.checked_add(right);
                sum.map_or(Counter::Overflow, Counter::Value)
            }
            _ => Counter::Overflow,
        }
    }

    // Returns the count as option.
    pub fn value(self) -> Option<u64> {
        match self {
            Counter::Overflow => None,
            Counter::Value(n) => Some(n),
        }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// Contains the counters of different values with the same key.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stat(pub BTreeMap<String, Counter>);

impl Stat {
    /// Returns an instance without any counters.
    pub fn new() -> Self {
        let counters = BTreeMap::new();
        Stat(counters)
    }

    // Increases the counter for the given value by one.
    pub fn inc(&mut self, value: String) {
        let counter_entry = self.0.entry(value).or_insert_with(Counter::new);
        *counter_entry = counter_entry.inc();
    }

    /// Merges both instances by merging the counters that belong to same value.
    pub fn merge(mut self, other: Self) -> Self {
        if self.0.len() < other.0.len() {
            other.merge(self)
        } else {
            for (value, right_counter) in other.0.into_iter() {
                match self.0.entry(value) {
                    Entry::Vacant(entry) => {
                        entry.insert(right_counter);
                    }
                    Entry::Occupied(entry) => {
                        let left_counter = *entry.get();
                        *entry.into_mut() = left_counter.merge(right_counter);
                    }
                }
            }

            self
        }
    }

    // Returns the total occurrence count for all keys.
    pub fn total_counter(&self) -> Counter {
        self.0
            .values()
            .fold(Counter::new(), |left, &right| left.merge(right))
    }
}

/// Contains the stats for different keys.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stats(pub BTreeMap<&'static str, Stat>);

impl Stats {
    /// Returns an instance without any stats.
    pub fn new() -> Self {
        Stats(BTreeMap::new())
    }

    // Increases the counter for the given key and value by one.
    pub fn inc(&mut self, key: &'static str, value: String) {
        let stat_entry = self.0.entry(key).or_insert_with(Stat::new);
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
}

#[cfg(feature = "stats")]
impl events::Events for Stats {
    fn new() -> Self {
        Stats::new()
    }

    fn take(&mut self) -> Self {
        let first_key = self.0.keys().next().cloned();
        match first_key {
            None => Stats::new(),
            Some(first_key) => {
                let stats = self.0.split_off(first_key);
                Stats(stats)
            }
        }
    }
}

#[cfg(feature = "stats")]
thread_local! {
    static LOCAL: events::Stack<Stats> = events::new_stack();
}

/// Returns the stats for the evaluation of `f`.
pub fn collect<R>(f: impl FnOnce() -> R) -> (R, Stats) {
    #[cfg(feature = "stats")]
    {
        events::collect(&LOCAL, f)
    }
    #[cfg(not(feature = "stats"))]
    {
        (f(), Stats::new())
    }
}

/// Returns if stats are currently enabled.
///
/// Stats are enabled if and only if this function is executed inside of `collect` and
/// the feature `hints` is present.
pub fn enabled() -> bool {
    #[cfg(feature = "stats")]
    {
        events::enabled(&LOCAL)
    }
    #[cfg(not(feature = "stats"))]
    {
        false
    }
}

/// If stats are enabled, this function evaluates the given value and increment its counter for
/// the given key. Otherwise this funcition is a noop.
pub fn inc(key: &'static str, value: impl FnOnce() -> String) {
    #[cfg(feature = "stats")]
    {
        events::modify(&LOCAL, move |stack| {
            let value = value();
            let len = stack.len();

            stack[0..len - 1]
                .iter_mut()
                .for_each(|stats| stats.inc(key, value.clone()));
            stack[len - 1].inc(key, value);
        });
    }
    #[cfg(not(feature = "stats"))]
    {
        drop(key);
        drop(value);
    }
}

#[cfg(test)]
mod tests {
    use crate::stats::Counter::{self, Overflow, Value};

    #[test]
    fn counter_inc_examples() {
        assert_eq!(Counter::new().inc(), Value(1));
        assert_eq!(Value(u64::max_value()).inc(), Overflow);
    }

    #[test]
    fn counter_merge_examples() {
        assert_eq!(Overflow.merge(Overflow), Overflow);
        assert_eq!(Overflow.merge(Counter::new()), Overflow);
        assert_eq!(Counter::new().merge(Overflow), Overflow);
        assert_eq!(Value(1).merge(Value(1)), Value(2));
    }

    #[cfg(feature = "stats")]
    #[test]
    fn stats_take_takes_all_elements() {
        use crate::stats::{Stat, Stats};
        use crate::util::events::Events;

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
