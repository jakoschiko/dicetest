use crate::logger::Messages;
use crate::counter::Stats;
use crate::brooder::{EvalParams, EvalSummary, Config, Status, ThreadErr};

/// The result of the brooder.
#[derive(Debug, Clone)]
#[must_use]
pub struct Report {
    /// The inital seed used for random number generation.
    ///
    /// If defined, the seed was taken from `Config::seed`. Otherwise the seed was generated
    /// randomly.
    pub seed: u64,
    /// The configuration that the brooder used to generate this report.
    pub config: Config,
    /// The merged result of all property evaluations.
    pub status: Status,
}

impl Report {
    /// Converts the `Report` to a pretty text.
    pub fn pretty(&self) -> String {
        let (headline, sections) = headline_and_sections(self);

        let mut acc = headline;

        for section in sections.iter() {
            acc += "\n\n";
            acc += section;
        }
        if !sections.is_empty() {
            acc += "\n";
        }

        acc
    }
}

fn headline_and_sections(report: &Report) -> (String, Vec<String>) {
    match report.status {
        Status::Checked(ref eval_series) => {
            match eval_series.summary {
                EvalSummary::True => {
                    let headline = format!(
                        "Property was proven after {} passed tests.",
                        eval_series.passed_tests,
                    );

                    let sections = vec!(
                        section_config(report.seed, &report.config),
                        section_stats(&eval_series.stats),
                    );

                    (headline, sections)
                }
                EvalSummary::Passed => {
                    let headline = format!(
                        "Property has passed {} tests.",
                        eval_series.passed_tests,
                    );

                    let sections = vec!(
                        section_config(report.seed, &report.config),
                        section_stats(&eval_series.stats),
                    );

                    (headline, sections)
                }
                EvalSummary::False { ref counterexample, ref messages } => {
                    let headline = format!(
                        "Property was falsified after {} passed tests.",
                        eval_series.passed_tests,
                    );

                    let mut sections = vec!(
                        section_config(report.seed, &report.config),
                        section_stats(&eval_series.stats),
                    );

                    if report.config.counter_enabled {
                        sections.push(section_counterexample(counterexample, messages));
                    }

                    (headline, sections)
                }
            }
        }
        Status::Panic(ref thread_err) => {
            let headline =
                "Property could not be checked because a thread had panicked.".to_string();

            let sections = vec!(
                section_config(report.seed, &report.config),
                section_panic(thread_err),
            );

            (headline, sections)
        }
        Status::Timeout => {
            let headline = format!(
                "Property could not be checked because a timeout occured after {:?}.",
                &report.config.timeout,
            );

            let sections = vec!(
                section_config(report.seed, &report.config),
            );

            (headline, sections)
        }
    }
}

fn section(title: &str, body: &str) -> String {
    format!("-- {} --\n{}", title, body)
}

fn section_config(seed: u64, config: &Config) -> String {
    let config_text = format!(
        "Seed: {}\n\
        Start limit: {}; End limit: {}\n\
        Min passed: {}; Worker count: {}; Timeout: {:?}",
        seed,
        config.start_limit,
        config.end_limit,
        config.min_passed,
        config.worker_count,
        config.timeout,
    );
    section("Config", &config_text)
}

fn section_stats(stats: &Stats) -> String {
   let pretty_stats = if stats.0.is_empty() {
        "No stats were collected.".to_string()
    } else {
        format!("These stats were collected during all evaluations:\n{}", stats.pretty())
    };
    section("Stats", &pretty_stats)
}

fn section_counterexample(counterexample: &EvalParams, messages: &Messages) -> String {
    let eval_code = counterexample.eval_code();
    let eval_code_help = format!(
        "You can rerun the counterexample by using its evaluation code:\n{}",
        eval_code,
    );
    let pretty_messages = if messages.0.is_empty() {
        "The counterexample has no log messages.".to_string()
    } else {
        format!("Log messages of the counterexample:\n{}", messages.pretty())
    };
    let debug_help = format!("{}\n\n{}", eval_code_help, pretty_messages);
    section("Counterexample", &debug_help)
}

fn section_panic(thread_err: &ThreadErr) -> String {
    let pretty_error = match thread_err.error_string() {
        Some(error) => format!("Error: {:?}", error),
        None => "The error has an unknown type and cannot be displayed.".to_string(),
    };
    section("Panic", &pretty_error)
}
