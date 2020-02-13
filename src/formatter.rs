//! Converts runner results into a human-readable format.

use std::fmt::{self, Display};
use std::iter::FromIterator;

use crate::die::Limit;
use crate::hints::Hints;
use crate::runner::{Config, Counterexample, Error, Run, Sample, Summary};
use crate::stats::Stats;

mod summary_formatting;
pub use summary_formatting::SummaryFormatting;

struct DisplayFromFn<F>(F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> Display for DisplayFromFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

fn impl_display<'a>(fmt: impl Fn(&mut fmt::Formatter) -> fmt::Result + 'a) -> impl Display + 'a {
    DisplayFromFn(fmt)
}

/// Converts the given `Sample` into a human-readable format.
pub fn display_sample<'a>(sample: &'a Sample) -> impl Display + 'a {
    impl_display(move |f| {
        let passed = sample.error.is_none();

        write_sample_headline(f, passed)?;
        write!(f, "\n\n")?;
        write_run_section(f, sample)
    })
}

/// Converts the given `Summary` into a human-readable format.
/// The output format can be configured with the `SummaryFormatting`.
pub fn display_summary<'a>(
    summary: &'a Summary,
    formatting: &'a SummaryFormatting,
) -> impl Display + 'a {
    impl_display(move |f| {
        let config = &summary.config.clone().with_seed(Some(summary.seed));
        let counterexample = &summary.counterexample;
        let passed = summary.counterexample.is_none();

        write_summary_headline(f, passed, summary.passes)?;
        write!(f, "\n\n")?;
        write_config_section(f, config)?;

        if let Some(ref stats) = summary.stats {
            writeln!(f)?;
            write_stats_section(
                f,
                &stats,
                formatting.stats_max_value_count,
                formatting.stats_percent_precision,
            )?;
        };

        if let Some(ref counterexample) = counterexample {
            let hints_enabled = cfg!(feature = "hints") && config.hints_enabled;

            writeln!(f)?;
            write_counterexample_section(f, hints_enabled, &counterexample)?;
        };

        Ok(())
    })
}

fn write_sample_headline(f: &mut fmt::Formatter, passed: bool) -> fmt::Result {
    let text = if passed {
        "The test passed."
    } else {
        "The test failed."
    };
    write!(f, "{}", text)
}

fn write_run_section(f: &mut fmt::Formatter, sample: &Sample) -> fmt::Result {
    write_section(
        f,
        "Run",
        impl_display(|f| {
            write_run_code_item(f, 0, &sample.run)?;
            write_limit_item(f, 0, sample.run.limit)?;
            write_hints_item(f, 0, &sample.hints)?;

            if let Some(ref error) = sample.error {
                write_error_item(f, 0, error)?;
            };

            Ok(())
        }),
    )
}

fn write_summary_headline(f: &mut fmt::Formatter, passed: bool, passes: u64) -> fmt::Result {
    let suffix = if passed {
        "The test withstood"
    } else {
        "The test failed after"
    };

    write!(f, "{} {} passes.", suffix, passes)
}

fn write_config_section(f: &mut fmt::Formatter, config: &Config) -> fmt::Result {
    write_section(
        f,
        "Config",
        impl_display(|f| {
            write_key_value_item(
                f,
                0,
                "seed",
                impl_display(|f| write_some_or(f, config.seed.map(|seed| seed.0), "none")),
            )?;
            write_key_value_item(f, 0, "start limit", config.start_limit.0)?;
            write_key_value_item(f, 0, "end limit", config.end_limit.0)?;
            write_key_value_item(f, 0, "passes", config.passes)
        }),
    )
}

fn write_stats_section(
    f: &mut fmt::Formatter,
    stats: &Stats,
    max_value_count: Option<usize>,
    percent_precision: usize,
) -> fmt::Result {
    write_section(
        f,
        "Stats",
        impl_display(|f| {
            if stats.0.is_empty() {
                write_item(f, 0, "No stats has been collected.")
            } else {
                for (key, stat) in &stats.0 {
                    let total = stat.total_counter().value().filter(|&n| n != 0);

                    let mut values = Vec::from_iter(stat.0.clone());
                    values.sort_by(|&(_, c1), &(_, c2)| c1.cmp(&c2).reverse());

                    let omitted_value_count = match max_value_count {
                        None => 0,
                        Some(max_value_count) => {
                            let omitted_value_count = values.len().saturating_sub(max_value_count);
                            values.truncate(max_value_count);
                            omitted_value_count
                        }
                    };

                    write_key_item(f, 0, key)?;

                    for (value, counter) in values.into_iter() {
                        let count = counter.value();
                        let numerator = count.and_then(|count| count.checked_mul(100));
                        let percent = numerator.and_then(move |numerator| {
                            total.map(move |total| numerator as f64 / total as f64)
                        });
                        write_key_value_item(
                            f,
                            1,
                            impl_display(|f| {
                                match percent {
                                    None => write!(f, "ovf")?,
                                    Some(percent) => {
                                        write!(f, "{:.n$}", percent, n = percent_precision)?
                                    }
                                };
                                write!(f, "% (")?;
                                write_some_or(f, count, "ovf")?;
                                write!(f, ")")
                            }),
                            value,
                        )?;
                    }

                    if omitted_value_count != 0 {
                        write_item(
                            f,
                            1,
                            impl_display(|f| {
                                write!(f, "{} values were omitted", omitted_value_count)
                            }),
                        )?;
                    }
                }
                Ok(())
            }
        }),
    )
}

fn write_counterexample_section(
    f: &mut fmt::Formatter,
    hints_enabled: bool,
    counterexample: &Counterexample,
) -> fmt::Result {
    write_section(
        f,
        "Counterexample",
        impl_display(|f| {
            write_run_code_item(f, 0, &counterexample.run)?;
            write_limit_item(f, 0, counterexample.run.limit)?;

            match counterexample.hints {
                None if !hints_enabled => (),
                None => write_item(
                    f,
                    0,
                    "Hints could not be collected afterwards, test is not deterministic.",
                )?,
                Some(ref hints) => write_hints_item(f, 0, hints)?,
            };

            write_error_item(f, 0, &counterexample.error)
        }),
    )
}

fn write_section(
    f: &mut fmt::Formatter,
    title: &'static str,
    content: impl Display,
) -> fmt::Result {
    writeln!(f, "# {}", title)?;
    write!(f, "{}", content)
}

fn write_run_code_item(f: &mut fmt::Formatter, indent: usize, run: &Run) -> fmt::Result {
    let run_code = run.to_run_code();
    write_key_value_item(
        f,
        indent,
        "run code",
        impl_display(|f| write!(f, "{:?}", run_code)),
    )
}

fn write_limit_item(f: &mut fmt::Formatter, indent: usize, limit: Limit) -> fmt::Result {
    write_key_value_item(f, indent, "limit", limit.0)
}

fn write_hints_item(f: &mut fmt::Formatter, indent: usize, hints: &Hints) -> fmt::Result {
    if hints.0.is_empty() {
        write_item(f, indent, "No hints has been collected.")
    } else {
        write_key_item(f, indent, "hints")?;

        let hint_ident_start = indent.saturating_add(1);
        for hint in &hints.0 {
            let hint_ident = hint_ident_start.saturating_add(hint.indent);
            write_item(f, hint_ident, &hint.text)?;
        }

        Ok(())
    }
}

fn write_error_item(f: &mut fmt::Formatter, indent: usize, error: &Error) -> fmt::Result {
    let err = &error.0;

    let string_repr = if let Some(string) = err.downcast_ref::<String>() {
        Some(string.as_str())
    } else if let Some(&str) = err.downcast_ref::<&str>() {
        Some(str)
    } else {
        None
    };

    match string_repr {
        None => write_item(
            f,
            indent,
            "The error has an unknown type and cannot be displayed.",
        ),
        Some(string_repr) => write_key_value_item(f, indent, "error", string_repr),
    }
}

fn write_key_item(f: &mut fmt::Formatter, indent: usize, key: impl Display) -> fmt::Result {
    write_item(f, indent, impl_display(|f| write!(f, "{}:", key)))
}

fn write_key_value_item(
    f: &mut fmt::Formatter,
    indent: usize,
    key: impl Display,
    value: impl Display,
) -> fmt::Result {
    write_item(f, indent, impl_display(|f| write!(f, "{}: {}", key, value)))
}

fn write_item(f: &mut fmt::Formatter, indent: usize, content: impl Display) -> fmt::Result {
    for _ in 0..indent {
        write!(f, "\t")?;
    }
    writeln!(f, "- {}", content)
}

fn write_some_or(
    f: &mut fmt::Formatter,
    content: Option<impl Display>,
    none: &'static str,
) -> fmt::Result {
    match content {
        None => write!(f, "{}", none),
        Some(content) => write!(f, "{}", content),
    }
}

#[cfg(test)]
mod tests {
    use crate::die::Limit;
    use crate::hints::{Hint, Hints};
    use crate::prand::Prng;
    use crate::runner::{Config, Counterexample, Error, Run, Summary};
    use crate::stats::{Counter, Stat, Stats};

    use super::*;

    fn contains_line(text: &str, expected_line: &str) -> bool {
        text.lines().any(|line| line == expected_line)
    }

    fn example_hints() -> Hints {
        Hints(vec![
            Hint {
                indent: 0,
                text: "Uh".into(),
            },
            Hint {
                indent: 1,
                text: "Ah".into(),
            },
            Hint {
                indent: 0,
                text: "Ih".into(),
            },
        ])
    }

    fn example_run() -> Run {
        Run {
            prng: Prng::from_seed(42.into()),
            limit: Limit(71),
        }
    }

    fn example_error() -> Error {
        Error(Box::new("Something bad happened!"))
    }

    fn example_config() -> Config {
        Config {
            seed: None,
            start_limit: 0.into(),
            end_limit: 100.into(),
            passes: 1000,
            hints_enabled: true,
            stats_enabled: false,
        }
    }

    #[test]
    fn display_sample_passed_example() {
        let sample = Sample {
            run: example_run(),
            hints: example_hints(),
            error: None,
        };

        let expected = format!(
            "\
The test passed.

# Run
- run code: {:?}
- limit: 71
- hints:
\t- Uh
\t\t- Ah
\t- Ih
",
            example_run().to_run_code(),
        );

        let actual = format!("{}", display_sample(&sample));

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_sample_failed_example() {
        let sample = Sample {
            run: example_run(),
            hints: example_hints(),
            error: Some(example_error()),
        };

        let expected = format!(
            "\
The test failed.

# Run
- run code: {:?}
- limit: 71
- hints:
\t- Uh
\t\t- Ah
\t- Ih
- error: Something bad happened!
",
            example_run().to_run_code(),
        );

        let actual = format!("{}", display_sample(&sample));

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_summary_passed_example() {
        let summary = Summary {
            config: example_config(),
            seed: 42.into(),
            passes: 1000,
            stats: None,
            counterexample: None,
        };

        let expected = "\
The test withstood 1000 passes.

# Config
- seed: 42
- start limit: 0
- end limit: 100
- passes: 1000
";

        let actual = format!(
            "{}",
            display_summary(&summary, &SummaryFormatting::default())
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_summary_failed_example() {
        let summary = Summary {
            config: example_config(),
            seed: 42.into(),
            passes: 123,
            stats: None,
            counterexample: Some(Counterexample {
                run: example_run(),
                hints: Some(example_hints()),
                error: example_error(),
            }),
        };

        let expected = format!(
            "\
The test failed after 123 passes.

# Config
- seed: 42
- start limit: 0
- end limit: 100
- passes: 1000

# Counterexample
- run code: {:?}
- limit: 71
- hints:
\t- Uh
\t\t- Ah
\t- Ih
- error: Something bad happened!
",
            example_run().to_run_code(),
        );

        let actual = format!(
            "{}",
            display_summary(&summary, &SummaryFormatting::default())
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_summary_preferes_the_seed_from_summary() {
        let summary = Summary {
            config: example_config().with_seed(Some(71.into())),
            seed: 42.into(),
            passes: 1000,
            stats: None,
            counterexample: None,
        };

        let actual = format!(
            "{}",
            display_summary(&summary, &SummaryFormatting::default())
        );

        assert!(contains_line(&actual, "- seed: 42"));
        assert!(!contains_line(&actual, "- seed: 471"));
    }

    #[test]
    fn display_summary_detects_missing_hints() {
        if cfg!(feature = "hints") {
            let summary = Summary {
                config: example_config(),
                seed: 42.into(),
                passes: 123,
                stats: None,
                counterexample: Some(Counterexample {
                    run: Run {
                        prng: Prng::from_seed(42.into()),
                        limit: Limit(71),
                    },
                    hints: None,
                    error: Error(Box::new("Something bad happened!")),
                }),
            };

            let actual = format!(
                "{}",
                display_summary(&summary, &SummaryFormatting::default())
            );

            assert!(contains_line(
                &actual,
                "- Hints could not be collected afterwards, test is not deterministic.",
            ));
        }
    }

    #[test]
    fn display_summary_detects_empty_hints() {
        if cfg!(feature = "hints") {
            let summary = Summary {
                config: Config::default().with_hints_enabled(true),
                seed: 42.into(),
                passes: 123,
                stats: None,
                counterexample: Some(Counterexample {
                    run: Run {
                        prng: Prng::from_seed(42.into()),
                        limit: Limit(71),
                    },
                    hints: Some(Hints::new()),
                    error: Error(Box::new("Something bad happened!")),
                }),
            };

            let actual = format!(
                "{}",
                display_summary(&summary, &&SummaryFormatting::default())
            );

            assert!(contains_line(&actual, "- No hints has been collected.",));
        }
    }

    #[test]
    fn stats_section_example() {
        let stats = Stats(
            vec![
                (
                    "foo".into(),
                    Stat(
                        vec![
                            ("a".into(), Counter::Value(10)),
                            ("b".into(), Counter::Value(20)),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
                (
                    "bar".into(),
                    Stat(
                        vec![
                            ("x".into(), Counter::Value(10)),
                            ("y".into(), Counter::Overflow),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
                (
                    "foobar".into(),
                    Stat(vec![("i".into(), Counter::Value(0))].into_iter().collect()),
                ),
                (
                    "foofoo".into(),
                    Stat(
                        vec![
                            ("x1".into(), Counter::Value(25)),
                            ("x2".into(), Counter::Value(10)),
                            ("x3".into(), Counter::Value(5)),
                            ("x4".into(), Counter::Value(50)),
                            ("x5".into(), Counter::Value(10)),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );

        let expected = "\
# Stats
- bar:
\t- ovf% (ovf): y
\t- ovf% (10): x
- foo:
\t- 66.67% (20): b
\t- 33.33% (10): a
- foobar:
\t- ovf% (0): i
- foofoo:
\t- 50.00% (50): x4
\t- 25.00% (25): x1
\t- 10.00% (10): x2
\t- 2 values were omitted
";

        let actual = format!(
            "{}",
            impl_display(|f| write_stats_section(f, &stats, Some(3), 2))
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_summary_detects_empty_stats() {
        if cfg!(feature = "hints") {
            let summary = Summary {
                config: Config::default().with_stats_enabled(true),
                seed: 42.into(),
                passes: 123,
                stats: Some(Stats::new()),
                counterexample: None,
            };

            let actual = format!(
                "{}",
                display_summary(&summary, &SummaryFormatting::default())
            );

            assert!(contains_line(&actual, "- No stats has been collected.",));
        }
    }
}
