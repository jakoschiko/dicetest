use std::iter;
use std::fmt::{Debug, Display};

use crate::hints::Hints;
use crate::stats::Stats;
use crate::gen::Limit;
use crate::runner::{Run, Config, Error, Sample, Counterexample, Summary};

pub fn pretty_sample(sample: &Sample) -> String {
    let passed = sample.error.is_none();

    empty()
        .chain(sample_headline(passed))
        .chain(line_feed(2))
        .chain(run_section(sample))
        .collect()
}

fn sample_headline(passed: bool) -> impl Iterator<Item=char> {
    let text = if passed { "The test passed." } else { "The test failed." };
    str(text)
}

fn run_section(sample: &Sample) -> impl Iterator<Item=char> {
    let title = str("Run");
    let content = {
        let acc = boxed(empty()
            .chain(run_code_item(0, &sample.run))
            .chain(limit_item(0, sample.run.limit))
            .chain(hints_item(0, &sample.hints)));

        let acc = match sample.error {
            None => acc,
            Some(ref error) => boxed(acc.chain(error_item(0, &error))),
        };

        acc
    };

    section(title, content)
}

pub fn pretty_summary(summary: &Summary) -> String {
    let config = &summary.config.clone().with_seed(Some(summary.seed));
    let counterexample = &summary.counterexample;
    let passed = summary.counterexample.is_none();

    let acc = boxed(empty()
        .chain(summary_headline(passed, summary.passes))
        .chain(line_feed(2))
        .chain(config_section(config)));

    let acc = match summary.stats {
        None => acc,
        Some(ref stats) => {
            boxed(acc
                .chain(line_feed(1))
                .chain(stats_section(&stats)))
        }
    };

    let acc = match counterexample {
        None => acc,
        Some(ref counterexample) => {
            let hints_enabled = cfg!(feature = "hints") && config.hints_enabled;

            boxed(acc
                .chain(line_feed(1))
                .chain(counterexample_section(hints_enabled, &counterexample)))
        }
    };

    acc.collect()
}


fn summary_headline(passed: bool, passes: u64) -> impl Iterator<Item=char> {
    let suffix = if passed { "The test withstood " } else { "The test failed after " };

    empty()
        .chain(str(suffix))
        .chain(display(Some(&passes)))
        .chain(str(" passes."))
}

fn config_section(config: &Config) -> impl Iterator<Item=char> {
    let title = str("Config");
    let content = empty()
        .chain(keyed_item(0, str("seed"), display(config.seed.as_ref())))
        .chain(keyed_item(0, str("start limit"), display(Some(&config.start_limit))))
        .chain(keyed_item(0, str("end limit"), display(Some(&config.end_limit))))
        .chain(keyed_item(0, str("passes"), display(Some(&config.passes))));

    section(title, content)
}

fn stats_section(stats: &Stats) -> impl Iterator<Item=char> {
    let title = str("Stats");
    let content = if stats.0.is_empty() {
        boxed(item(0, str("No stats has been collected.")))
    } else {
        let stats_iter = stats.0.clone().into_iter();

        let pretty_stats = stats_iter.flat_map(|(key, stat)| {
            let total = stat.total_counter().value().filter(|&n| n != 0);

            let stat_iter = {
                let mut sorted = stat.0
                    .into_iter()
                    .collect::<Vec<_>>();

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
                let pretty_count = count.map_or_else(overflow, |count| {
                    boxed(display(Some(&count)))
                });

                let pretty_occurrence = empty()
                    .chain(pretty_percent)
                    .chain(str("% ("))
                    .chain(pretty_count)
                    .chain(str(")"));

                keyed_item(1, pretty_occurrence, string(value))
            });

            empty()
                .chain(keyed_item(0, str(key), empty()))
                .chain(values)
        });

        boxed(pretty_stats)
    };

    section(title, content)
}

fn counterexample_section(
    hints_enabled: bool,
    counterexample: &Counterexample
) -> impl Iterator<Item=char> {
    let title = str("Counterexample");
    let content = {
        let acc = boxed(empty()
            .chain(run_code_item(0, &counterexample.run))
            .chain(limit_item(0, counterexample.run.limit)));

        let acc = match counterexample.hints {
            None if !hints_enabled => acc,
            None => {
                let text = "Hints could not be collected afterwards, test is not deterministic.";
                boxed(acc.chain(item(0, str(text))))
            },
            Some(ref hints) => boxed(acc.chain(hints_item(0, hints))),
        };

        let acc = acc.chain(error_item(0, &counterexample.error));

        acc
    };

    section(title, content)
}

fn section(
    title: impl Iterator<Item=char>,
    content: impl Iterator<Item=char>
) -> impl Iterator<Item=char> {
    empty()
        .chain(str("# "))
        .chain(title)
        .chain(line_feed(1))
        .chain(content)
}

fn run_code_item(indent: usize, run: &Run) -> impl Iterator<Item=char> {
    let run_code = run.run_code();
    keyed_item(indent, str("run code"), debug(Some(&run_code)))
}

fn limit_item(indent: usize, limit: Limit) -> impl Iterator<Item=char> {
    keyed_item(indent, str("limit"), display(Some(&limit.0)))
}

fn hints_item(indent: usize, hints: &Hints) -> impl Iterator<Item=char> {
    if hints.0.is_empty() {
        boxed(item(indent, str("No hints has been collected.")))
    } else {
        let hints_ident = indent.saturating_add(1);

        let pretty_hints = hints.0
            .clone()
            .into_iter()
            .flat_map(move |hint| {
                item(hints_ident.saturating_add(hint.indent), string(hint.text))
            });

        boxed(empty()
            .chain(keyed_item(indent, str("hints"), empty()))
            .chain(pretty_hints))
    }
}

fn error_item(indent: usize, error: &Error) -> impl Iterator<Item=char> {
    let err = &error.0;

    let string_repr =
        if let Some(string) = err.downcast_ref::<String>() {
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
        Some(string_repr) => boxed(keyed_item(indent, str("error"), string(string_repr))),
    }
}

fn keyed_item(
    indent: usize,
    key: impl Iterator<Item=char>,
    content: impl Iterator<Item=char>
) -> impl Iterator<Item=char> {
    let key_with_content = key.chain(str(": ")).chain(content);
    item(indent, key_with_content)
}

fn item(indent: usize, content: impl Iterator<Item=char>) -> impl Iterator<Item=char> {
    iter::repeat("\t")
        .take(indent)
        .flat_map(|indent| str(indent))
        .chain(str("- "))
        .chain(content)
        .chain(line_feed(1))
}

fn boxed(iter: impl Iterator<Item=char> + 'static) -> Box<dyn Iterator<Item=char>> {
    Box::new(iter)
}

fn debug(content: Option<&impl Debug>) -> impl Iterator<Item=char> {
    match content {
        None => boxed(str("none")),
        Some(content) => boxed(string(format!("{:?}", content))),
    }
}

fn display(content: Option<&impl Display>) -> impl Iterator<Item=char> {
    match content {
        None => boxed(str("none")),
        Some(content) => boxed(string(format!("{}", content))),
    }
}

fn str(str: &'static str) -> impl Iterator<Item=char> {
    str.chars()
}

fn string(string: String) -> impl Iterator<Item=char> {
    string
        .chars()
        .collect::<Vec<_>>()
        .into_iter()
}

fn line_feed(n: usize) -> impl Iterator<Item=char> {
    iter::repeat('\n').take(n)
}

fn empty() -> impl Iterator<Item=char> {
    iter::empty()
}

// TODO: tests
