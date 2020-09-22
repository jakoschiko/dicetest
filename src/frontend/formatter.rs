use std::fmt::{self, Display};
use std::iter::FromIterator;

use crate::frontend::RunCode;
use crate::hints::Hints;
use crate::runner::{self, repeatedly::Counterexample, Error};
use crate::stats::Stats;
use crate::{Limit, Seed};

#[derive(Debug, Clone)]
pub struct Formatting {
    pub stats_max_value_count: Option<usize>,
    pub stats_percent_precision: usize,
}

impl Default for Formatting {
    fn default() -> Self {
        Self {
            stats_max_value_count: Some(20),
            stats_percent_precision: 2,
        }
    }
}

struct DisplayFromFn<F>(F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> Display for DisplayFromFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

fn impl_display<'a>(fmt: impl Fn(&mut fmt::Formatter) -> fmt::Result + 'a) -> impl Display + 'a {
    DisplayFromFn(fmt)
}

pub fn display_run_once_report<'a>(
    run_code: &'a RunCode,
    seed: Option<Seed>,
    report: &'a runner::once::Report,
    formatting: &'a Formatting,
) -> impl Display + 'a {
    impl_display(move |f| {
        let passed = report.error.is_none();

        write_run_once_headline(f, passed)?;
        write!(f, "\n\n")?;
        write_run_once_section(f, run_code, seed, report)?;

        if let Some(ref stats) = report.stats {
            writeln!(f)?;
            write_stats_section(f, stats, formatting)?;
        }

        Ok(())
    })
}

pub fn display_run_repeatedly_report<'a>(
    seed: Seed,
    config: &'a runner::repeatedly::Config,
    report: &'a runner::repeatedly::Report,
    formatting: &'a Formatting,
) -> impl Display + 'a {
    impl_display(move |f| {
        let counterexample = &report.counterexample;
        let passed = report.counterexample.is_none();

        write_run_repeatedly_headline(f, passed, report.passes)?;
        write!(f, "\n\n")?;
        write_run_repeatedly_parameters_section(f, seed, config)?;

        if let Some(ref stats) = report.stats {
            writeln!(f)?;
            write_stats_section(f, stats, formatting)?;
        };

        if let Some(ref counterexample) = counterexample {
            let hints_enabled = cfg!(feature = "hints") && config.hints_enabled;

            writeln!(f)?;
            write_counterexample_section(f, hints_enabled, counterexample)?;
        };

        Ok(())
    })
}

fn write_run_once_headline(f: &mut fmt::Formatter, passed: bool) -> fmt::Result {
    let text = if passed {
        "The test passed."
    } else {
        "The test failed."
    };
    write!(f, "{}", text)
}

fn write_run_once_section(
    f: &mut fmt::Formatter,
    run_code: &RunCode,
    seed: Option<Seed>,
    report: &runner::once::Report,
) -> fmt::Result {
    write_section(
        f,
        "Run",
        impl_display(|f| {
            write_run_code_item(f, 0, run_code)?;

            if let Some(seed) = seed {
                write_seed_item(f, 0, seed)?;
            };

            write_limit_item(f, 0, run_code.limit)?;

            if let Some(ref hints) = report.hints {
                write_hints_item(f, 0, hints)?;
            }

            if let Some(ref error) = report.error {
                write_error_item(f, 0, error)?;
            };

            Ok(())
        }),
    )
}

fn write_run_repeatedly_headline(f: &mut fmt::Formatter, passed: bool, passes: u64) -> fmt::Result {
    let suffix = if passed {
        "The test withstood"
    } else {
        "The test failed after"
    };

    write!(f, "{} {} passes.", suffix, passes)
}

fn write_run_repeatedly_parameters_section(
    f: &mut fmt::Formatter,
    seed: Seed,
    config: &runner::repeatedly::Config,
) -> fmt::Result {
    write_section(
        f,
        "Config",
        impl_display(|f| {
            write_seed_item(f, 0, seed)?;
            write_key_value_item(f, 0, "start limit", config.start_limit.0)?;
            write_key_value_item(f, 0, "end limit", config.end_limit.0)?;
            write_key_value_item(f, 0, "passes", config.passes)
        }),
    )
}

fn write_stats_section(
    f: &mut fmt::Formatter,
    stats: &Stats,
    formatting: &Formatting,
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

                    let max_value_count = formatting.stats_max_value_count;
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
                                        let percent_precesion = formatting.stats_percent_precision;
                                        write!(f, "{:.n$}", percent, n = percent_precesion)?
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
            let run_code = RunCode {
                prng: counterexample.prng.clone(),
                limit: counterexample.limit,
            };
            write_run_code_item(f, 0, &run_code)?;
            write_limit_item(f, 0, counterexample.limit)?;

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

fn write_run_code_item(f: &mut fmt::Formatter, indent: usize, run_code: &RunCode) -> fmt::Result {
    let base64 = run_code.to_base64();
    write_key_value_item(
        f,
        indent,
        "run code",
        impl_display(|f| write!(f, "{:?}", base64)),
    )
}

fn write_seed_item(f: &mut fmt::Formatter, indent: usize, seed: Seed) -> fmt::Result {
    write_key_value_item(f, indent, "seed", seed.0)
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
    use crate::frontend::RunCode;
    use crate::hints::{Hint, Hints};
    use crate::runner::{self, repeatedly::Counterexample, Error};
    use crate::stats::{Counter, Stat, Stats};
    use crate::{Limit, Prng};

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

    fn example_run_code() -> RunCode {
        RunCode {
            prng: Prng::from_seed(42.into()),
            limit: Limit(71),
        }
    }

    fn example_error() -> Error {
        Error(Box::new("Something bad happened!"))
    }

    fn example_run_repeatedly_config() -> runner::repeatedly::Config {
        runner::repeatedly::Config {
            start_limit: 0.into(),
            end_limit: 100.into(),
            passes: 1000,
            hints_enabled: true,
            stats_enabled: false,
        }
    }

    #[test]
    fn display_run_once_report_passed_wiht_hints_example() {
        let run_code = example_run_code();
        let report = runner::once::Report {
            hints: Some(example_hints()),
            stats: None,
            error: None,
        };
        let formatting = Formatting::default();

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
            run_code.to_base64(),
        );

        let actual = format!(
            "{}",
            display_run_once_report(&run_code, None, &report, &formatting,)
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_run_once_report_passed_with_seed_example() {
        let run_code = example_run_code();
        let report = runner::once::Report {
            hints: None,
            stats: None,
            error: None,
        };
        let formatting = Formatting::default();

        let expected = format!(
            "\
The test passed.

# Run
- run code: {:?}
- seed: 42
- limit: 71
",
            run_code.to_base64(),
        );

        let actual = format!(
            "{}",
            display_run_once_report(&run_code, Some(42.into()), &report, &formatting,)
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_run_once_report_passed_with_stats_example() {
        let run_code = example_run_code();
        let stats = Stats(
            vec![(
                "foo".into(),
                Stat(
                    vec![
                        ("a".into(), Counter::Value(10)),
                        ("b".into(), Counter::Value(20)),
                    ]
                    .into_iter()
                    .collect(),
                ),
            )]
            .into_iter()
            .collect(),
        );
        let report = runner::once::Report {
            hints: None,
            stats: Some(stats),
            error: None,
        };
        let formatting = Formatting::default();

        let expected = format!(
            "\
The test passed.

# Run
- run code: {:?}
- limit: 71

# Stats
- foo:
\t- 66.67% (20): b
\t- 33.33% (10): a
",
            run_code.to_base64(),
        );

        let actual = format!(
            "{}",
            display_run_once_report(&run_code, None, &report, &formatting,)
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_run_once_report_failed_example() {
        let run_code = example_run_code();
        let report = runner::once::Report {
            hints: Some(example_hints()),
            stats: None,
            error: Some(example_error()),
        };
        let formatting = Formatting::default();

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
            example_run_code().to_base64(),
        );

        let actual = format!(
            "{}",
            display_run_once_report(&run_code, None, &report, &formatting,)
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_run_repeatedly_report_passed_example() {
        let seed = 42.into();
        let config = example_run_repeatedly_config();
        let report = runner::repeatedly::Report {
            passes: 1000,
            stats: None,
            counterexample: None,
        };
        let formatting = Formatting::default();

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
            display_run_repeatedly_report(seed, &config, &report, &formatting,)
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_run_repeatedly_report_failed_example() {
        let seed = 42.into();
        let config = example_run_repeatedly_config();
        let run_code = example_run_code();
        let report = runner::repeatedly::Report {
            passes: 123,
            stats: None,
            counterexample: Some(Counterexample {
                prng: run_code.prng.clone(),
                limit: run_code.limit,
                hints: Some(example_hints()),
                error: example_error(),
            }),
        };
        let formatting = Formatting::default();

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
            run_code.to_base64(),
        );

        let actual = format!(
            "{}",
            display_run_repeatedly_report(seed, &config, &report, &formatting,)
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_run_repeatedly_report_detects_missing_hints() {
        if cfg!(feature = "hints") {
            let seed = 42.into();
            let config = example_run_repeatedly_config();
            let report = runner::repeatedly::Report {
                passes: 123,
                stats: None,
                counterexample: Some(Counterexample {
                    prng: Prng::from_seed(42.into()),
                    limit: Limit(71),
                    hints: None,
                    error: Error(Box::new("Something bad happened!")),
                }),
            };
            let formatting = Formatting::default();

            let actual = format!(
                "{}",
                display_run_repeatedly_report(seed, &config, &report, &formatting,)
            );

            assert!(contains_line(
                &actual,
                "- Hints could not be collected afterwards, test is not deterministic.",
            ));
        }
    }

    #[test]
    fn display_run_repeatedly_report_detects_empty_hints() {
        if cfg!(feature = "hints") {
            let seed = 42.into();
            let config = runner::repeatedly::Config {
                hints_enabled: true,
                ..example_run_repeatedly_config()
            };
            let report = runner::repeatedly::Report {
                passes: 123,
                stats: None,
                counterexample: Some(Counterexample {
                    prng: Prng::from_seed(42.into()),
                    limit: Limit(71),
                    hints: Some(Hints::new()),
                    error: Error(Box::new("Something bad happened!")),
                }),
            };
            let formatting = Formatting::default();

            let actual = format!(
                "{}",
                display_run_repeatedly_report(seed, &config, &report, &formatting,)
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
        let formatting = Formatting {
            stats_max_value_count: Some(3),
            stats_percent_precision: 2,
        };

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
            impl_display(|f| write_stats_section(f, &stats, &formatting))
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_run_repeatedly_report_detects_empty_stats() {
        if cfg!(feature = "hints") {
            let seed = 42.into();
            let config = runner::repeatedly::Config {
                stats_enabled: true,
                ..example_run_repeatedly_config()
            };
            let report = runner::repeatedly::Report {
                passes: 123,
                stats: Some(Stats::new()),
                counterexample: None,
            };
            let formatting = Formatting::default();

            let actual = format!(
                "{}",
                display_run_repeatedly_report(seed, &config, &report, &formatting,)
            );

            assert!(contains_line(&actual, "- No stats has been collected.",));
        }
    }
}
