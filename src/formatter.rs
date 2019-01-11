use std::fmt::{Debug, Display};
use std::iter;
use std::mem;
use std::str::Chars;

use crate::die::Limit;
use crate::hints::Hints;
use crate::runner::{Config, Counterexample, Error, Run, Sample, Summary};
use crate::stats::Stats;

pub fn pretty_sample(sample: &Sample) -> String {
    let passed = sample.error.is_none();

    empty()
        .chain(sample_headline(passed))
        .chain(line_feed(2))
        .chain(run_section(sample))
        .collect()
}

fn sample_headline(passed: bool) -> impl Iterator<Item = char> {
    let text = if passed {
        "The test passed."
    } else {
        "The test failed."
    };
    str(text)
}

fn run_section(sample: &Sample) -> impl Iterator<Item = char> {
    let title = str("Run");
    let content = {
        let acc = boxed(
            empty()
                .chain(run_code_item(0, &sample.run))
                .chain(limit_item(0, sample.run.limit))
                .chain(hints_item(0, &sample.hints)),
        );

        match sample.error {
            None => acc,
            Some(ref error) => boxed(acc.chain(error_item(0, &error))),
        }
    };

    section(title, content)
}

pub fn pretty_summary(summary: &Summary) -> String {
    let config = &summary.config.clone().with_seed(Some(summary.seed));
    let counterexample = &summary.counterexample;
    let passed = summary.counterexample.is_none();

    let acc = boxed(
        empty()
            .chain(summary_headline(passed, summary.passes))
            .chain(line_feed(2))
            .chain(config_section(config)),
    );

    let acc = match summary.stats {
        None => acc,
        Some(ref stats) => boxed(acc.chain(line_feed(1)).chain(stats_section(&stats))),
    };

    let acc = match counterexample {
        None => acc,
        Some(ref counterexample) => {
            let hints_enabled = cfg!(feature = "hints") && config.hints_enabled;

            boxed(
                acc.chain(line_feed(1))
                    .chain(counterexample_section(hints_enabled, &counterexample)),
            )
        }
    };

    acc.collect()
}

fn summary_headline(passed: bool, passes: u64) -> impl Iterator<Item = char> {
    let suffix = if passed {
        "The test withstood "
    } else {
        "The test failed after "
    };

    empty()
        .chain(str(suffix))
        .chain(display(Some(&passes)))
        .chain(str(" passes."))
}

fn config_section(config: &Config) -> impl Iterator<Item = char> {
    let title = str("Config");
    let content = empty()
        .chain(key_value_item(
            0,
            str("seed"),
            display(config.seed.map(|seed| seed.0).as_ref()),
        ))
        .chain(key_value_item(
            0,
            str("start limit"),
            display(Some(&config.start_limit.0)),
        ))
        .chain(key_value_item(
            0,
            str("end limit"),
            display(Some(&config.end_limit.0)),
        ))
        .chain(key_value_item(
            0,
            str("passes"),
            display(Some(&config.passes)),
        ));

    section(title, content)
}

fn stats_section(stats: &Stats) -> impl Iterator<Item = char> {
    let title = str("Stats");
    let content = if stats.0.is_empty() {
        boxed(item(0, str("No stats has been collected.")))
    } else {
        let stats_iter = stats.0.clone().into_iter();

        let pretty_stats = stats_iter.flat_map(|(key, stat)| {
            let total = stat.total_counter().value().filter(|&n| n != 0);

            let stat_iter = {
                let mut sorted = stat.0.into_iter().collect::<Vec<_>>();

                sorted.sort_by_key(|&(_, counter)| counter);

                sorted.into_iter().rev()
            };

            let values = stat_iter.flat_map(move |(value, counter)| {
                let overflow = || boxed(str("ovf"));
                let count = counter.value();

                let pretty_percent = {
                    let numerator = count.and_then(|count| count.checked_mul(100));

                    match (numerator, total) {
                        (Some(numerator), Some(total)) => {
                            let percent = numerator / total;
                            boxed(display(Some(&percent)))
                        }
                        _ => overflow(),
                    }
                };
                let pretty_count =
                    count.map_or_else(overflow, |count| boxed(display(Some(&count))));

                let pretty_occurrence = empty()
                    .chain(pretty_percent)
                    .chain(str("% ("))
                    .chain(pretty_count)
                    .chain(str(")"));

                key_value_item(1, pretty_occurrence, string(value))
            });

            empty().chain(key_item(0, str(key))).chain(values)
        });

        boxed(pretty_stats)
    };

    section(title, content)
}

fn counterexample_section(
    hints_enabled: bool,
    counterexample: &Counterexample,
) -> impl Iterator<Item = char> {
    let title = str("Counterexample");
    let content = {
        let acc = boxed(
            empty()
                .chain(run_code_item(0, &counterexample.run))
                .chain(limit_item(0, counterexample.run.limit)),
        );

        let acc = match counterexample.hints {
            None if !hints_enabled => acc,
            None => {
                let text = "Hints could not be collected afterwards, test is not deterministic.";
                boxed(acc.chain(item(0, str(text))))
            }
            Some(ref hints) => boxed(acc.chain(hints_item(0, hints))),
        };

        acc.chain(error_item(0, &counterexample.error))
    };

    section(title, content)
}

fn section(
    title: impl Iterator<Item = char>,
    content: impl Iterator<Item = char>,
) -> impl Iterator<Item = char> {
    empty()
        .chain(str("# "))
        .chain(title)
        .chain(line_feed(1))
        .chain(content)
}

fn run_code_item(indent: usize, run: &Run) -> impl Iterator<Item = char> {
    let run_code = run.to_run_code();
    key_value_item(indent, str("run code"), debug(Some(&run_code)))
}

fn limit_item(indent: usize, limit: Limit) -> impl Iterator<Item = char> {
    key_value_item(indent, str("limit"), display(Some(&limit.0)))
}

fn hints_item(indent: usize, hints: &Hints) -> impl Iterator<Item = char> {
    if hints.0.is_empty() {
        boxed(item(indent, str("No hints has been collected.")))
    } else {
        let hints_ident = indent.saturating_add(1);

        let pretty_hints =
            hints.0.clone().into_iter().flat_map(move |hint| {
                item(hints_ident.saturating_add(hint.indent), string(hint.text))
            });

        boxed(
            empty()
                .chain(key_item(indent, str("hints")))
                .chain(pretty_hints),
        )
    }
}

fn error_item(indent: usize, error: &Error) -> impl Iterator<Item = char> {
    let err = &error.0;

    let string_repr = if let Some(string) = err.downcast_ref::<String>() {
        Some(string.clone())
    } else if let Some(str) = err.downcast_ref::<&str>() {
        Some(str.to_string())
    } else {
        None
    };

    match string_repr {
        None => {
            let text = "The error has an unknown type and cannot be displayed.";
            boxed(item(indent, str(text)))
        }
        Some(string_repr) => boxed(key_value_item(indent, str("error"), string(string_repr))),
    }
}

fn key_item(indent: usize, key: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
    item(indent, key.chain(str(":")))
}

fn key_value_item(
    indent: usize,
    key: impl Iterator<Item = char>,
    value: impl Iterator<Item = char>,
) -> impl Iterator<Item = char> {
    let key_with_value = key.chain(str(": ")).chain(value);
    item(indent, key_with_value)
}

fn item(indent: usize, content: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
    iter::repeat("\t")
        .take(indent)
        .flat_map(|indent| str(indent))
        .chain(str("- "))
        .chain(content)
        .chain(line_feed(1))
}

fn boxed(iter: impl Iterator<Item = char> + 'static) -> Box<dyn Iterator<Item = char>> {
    Box::new(iter)
}

fn debug(content: Option<&impl Debug>) -> impl Iterator<Item = char> {
    match content {
        None => boxed(str("none")),
        Some(content) => boxed(string(format!("{:?}", content))),
    }
}

fn display(content: Option<&impl Display>) -> impl Iterator<Item = char> {
    match content {
        None => boxed(str("none")),
        Some(content) => boxed(string(format!("{}", content))),
    }
}

fn str(str: &'static str) -> impl Iterator<Item = char> {
    str.chars()
}

fn string(string: String) -> impl Iterator<Item = char> {
    struct OwningChars {
        _chars_owner: String,
        chars: Chars<'static>,
    }

    impl Iterator for OwningChars {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            self.chars.next()
        }
    }

    let chars = unsafe { mem::transmute(string.chars()) };
    OwningChars {
        _chars_owner: string,
        chars,
    }
}

fn line_feed(n: usize) -> impl Iterator<Item = char> {
    iter::repeat('\n').take(n)
}

fn empty() -> impl Iterator<Item = char> {
    iter::empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::die::Limit;
    use crate::hints::{Hint, Hints};
    use crate::prand::Prng;
    use crate::runner::{Config, Counterexample, Error, Run, Summary};
    use crate::stats::{Counter, Stat, Stats};

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
    fn pretty_sample_passed_example() {
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

        let actual = pretty_sample(&sample);

        assert_eq!(expected, actual);
    }

    #[test]
    fn pretty_sample_failed_example() {
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

        let actual = pretty_sample(&sample);

        assert_eq!(expected, actual);
    }

    #[test]
    fn pretty_summary_passed_example() {
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

        let actual = pretty_summary(&summary);

        assert_eq!(expected, actual);
    }

    #[test]
    fn pretty_summary_failed_example() {
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

        let actual = pretty_summary(&summary);

        assert_eq!(expected, actual);
    }

    #[test]
    fn pretty_summary_preferes_the_seed_from_summary() {
        let summary = Summary {
            config: example_config().with_seed(Some(71.into())),
            seed: 42.into(),
            passes: 1000,
            stats: None,
            counterexample: None,
        };

        let actual = pretty_summary(&summary);

        assert!(contains_line(&actual, "- seed: 42"));
        assert!(!contains_line(&actual, "- seed: 471"));
    }

    #[test]
    fn pretty_summary_detects_missing_hints() {
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

            let actual = pretty_summary(&summary);

            assert!(contains_line(
                &actual,
                "- Hints could not be collected afterwards, test is not deterministic.",
            ));
        }
    }

    #[test]
    fn pretty_summary_detects_empty_hints() {
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

            let actual = pretty_summary(&summary);

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
\t- 66% (20): b
\t- 33% (10): a
- foobar:
\t- ovf% (0): i
";

        let actual = stats_section(&stats).collect::<String>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn pretty_summary_detects_empty_stats() {
        if cfg!(feature = "hints") {
            let summary = Summary {
                config: Config::default().with_stats_enabled(true),
                seed: 42.into(),
                passes: 123,
                stats: Some(Stats::new()),
                counterexample: None,
            };

            let actual = pretty_summary(&summary);

            assert!(contains_line(&actual, "- No stats has been collected.",));
        }
    }
}
