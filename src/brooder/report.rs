use crate::prop::Prints;
use crate::brooder::{EvalParams, EvalSummary, Params, Status, ThreadErr};

/// The result of the brooder.
#[derive(Debug, Clone)]
#[must_use]
pub struct Report {
    /// The inital seed used for random number generation.
    ///
    /// The seed is taken from `Params::seed` or else generated randomly.
    pub seed: u64,
    /// The orginal parameters passed to the brooder.
    pub params: Params,
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
                        "Property was proven after {} passed tests",
                        eval_series.passed_tests,
                    );

                    let sections = vec!(
                        section_parameters(report.seed, &report.params),
                    );

                    (headline, sections)
                }
                EvalSummary::Passed => {
                    let headline = format!(
                        "Property has passed {} tests",
                        eval_series.passed_tests,
                    );

                    let sections = vec!(
                        section_parameters(report.seed, &report.params),
                    );

                    (headline, sections)
                }
                EvalSummary::False { ref counterexample, ref prints } => {
                    let headline = format!(
                        "Property was falsified after {} passed tests",
                        eval_series.passed_tests,
                    );

                    let sections = vec!(
                        section_parameters(report.seed, &report.params),
                        section_counterexample(counterexample, prints),
                    );

                    (headline, sections)
                }
            }
        }
        Status::Panic(ref thread_err) => {
            let headline =
                "Property could not be checked because a thread had panicked".to_string();

            let sections = vec!(
                section_parameters(report.seed, &report.params),
                section_panic(thread_err),
            );

            (headline, sections)
        }
        Status::Timeout => {
            let headline = format!(
                "Property could not be checked because a timeout occured after {:?}",
                &report.params.timeout,
            );

            let sections = vec!(
                section_parameters(report.seed, &report.params),
            );

            (headline, sections)
        }
    }
}

fn section(title: &str, body: &str) -> String {
    format!("-- {} --\n{}", title, body)
}

fn section_parameters(seed: u64, params: &Params) -> String {
    let parameters_text = format!(
        "Seed: {}\n\
        Start limit: {}; End limit: {}\n\
        Min passed: {}; Worker count: {}; Timeout: {:?}",
        seed,
        params.start_limit,
        params.end_limit,
        params.min_passed,
        params.worker_count,
        params.timeout,
    );
    section("Parameters", &parameters_text)
}

fn section_counterexample(counterexample: &EvalParams, prints: &Prints) -> String {
    let eval_code = counterexample.eval_code();
    let eval_code_help = format!(
        "You can rerun the counterexample by using its evaluation code:\n\
        debug_prop_eval_with_code(\"{}\", ...)",
        eval_code,
    );
    let pretty_prints = if prints.0.is_empty() {
        "The counterexample has no prints".to_string()
    } else {
        format!("Prints of the counterexample:\n{}", prints.pretty())
    };
    let debug_help = format!("{}\n\n{}", eval_code_help, pretty_prints);
    section("Counterexample", &debug_help)
}

fn section_panic(thread_err: &ThreadErr) -> String {
    let pretty_error = match thread_err.error_string() {
        Some(error) => format!("Error: {:?}", error),
        None => "The error has an unknown type and cannot be displayed".to_string(),
    };
    section("Panic", &pretty_error)
}
