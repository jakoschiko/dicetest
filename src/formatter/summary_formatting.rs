/// The configuration for formatting the `Summary`.
#[derive(Debug, Clone)]
pub struct SummaryFormatting {
    /// The maximum numbers of values per key that will be used when formatting the stats.
    pub stats_max_value_count: Option<usize>,
    /// The number of decimal places for percent values that will be used when formatting the stats.
    pub stats_percent_precision: usize,
}

impl SummaryFormatting {
    pub fn with_stats_max_value_count(self, stats_max_value_count: Option<usize>) -> Self {
        Self {
            stats_max_value_count,
            ..self
        }
    }

    pub fn with_stats_percent_precision(self, stats_percent_precision: usize) -> Self {
        Self {
            stats_percent_precision,
            ..self
        }
    }
}

impl Default for SummaryFormatting {
    fn default() -> Self {
        Self {
            stats_max_value_count: Some(20),
            stats_percent_precision: 2,
        }
    }
}
