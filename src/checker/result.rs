use ::prop::Labels;
use ::checker::{EvalParams, EvalSeriesStatus, Params, Status, ThreadErr};

/// The result of the checker.
#[derive(Debug, Clone)]
#[must_use]
pub struct Result {
    /// The inital seed used for random number generation.
    ///
    /// The seed is taken from `Params::seed` or else generated randomly.
    pub seed: u64,
    /// The orginal parameters passed to the checker.
    pub params: Params,
    /// The merged result of all property evaluations.
    pub status: Status,
}

impl Result {
    /// Converts the `Result` to a human-readable summary.
    pub fn summary(&self) -> String {
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

fn headline_and_sections(result: &Result) -> (String, Vec<String>) {
    match result.status {
        Status::Checked(ref eval_series_result) => {
            match eval_series_result.status {
                EvalSeriesStatus::True => {
                    let headline = format!(
                        "Property was proven after {} passed tests",
                        eval_series_result.passed_tests,
                    );

                    let sections = vec!(
                        section_parameters(result.seed, &result.params),
                    );

                    (headline, sections)
                }
                EvalSeriesStatus::Passed => {
                    let headline = format!(
                        "Property has passed {} tests",
                        eval_series_result.passed_tests,
                    );

                    let sections = vec!(
                        section_parameters(result.seed, &result.params),
                    );

                    (headline, sections)
                }
                EvalSeriesStatus::False { ref counterexample, ref labels } => {
                    let headline = format!(
                        "Property was falsified after {} passed tests",
                        eval_series_result.passed_tests,
                    );

                    let sections = vec!(
                        section_parameters(result.seed, &result.params),
                        section_counterexample(counterexample, labels),
                    );

                    (headline, sections)
                }
            }
        }
        Status::Panic(ref thread_err) => {
            let headline =
                "Property could not be checked because a thread had panicked".to_string();

            let sections = vec!(
                section_parameters(result.seed, &result.params),
                section_panic(thread_err),
            );

            (headline, sections)
        }
        Status::Timeout => {
            let headline = format!(
                "Property could not be checked because a timeout occured after {:?}",
                &result.params.timeout,
            );

            let sections = vec!(
                section_parameters(result.seed, &result.params),
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
        Start size: {}; End size: {}\n\
        Min passed: {}; Worker count: {}; Timeout: {:?}",
        seed,
        params.start_size,
        params.end_size,
        params.min_passed,
        params.worker_count,
        params.timeout,
    );
    section("Parameters", &parameters_text)
}

fn section_counterexample(counterexample: &EvalParams, labels: &Labels) -> String {
    let eval_code = counterexample.eval_code();
    let eval_code_help = format!(
        "You can rerun the counterexample by using its evaluation code:\n\
        debug_prop(\"{}\", || my_prop)",
        eval_code,
    );
    let pretty_labels = if labels.is_empty() {
        "The counterexample has no labels".to_string()
    } else {
        format!("Labels of the counterexample:\n{}", labels.pretty_labels())
    };
    let debug_help = format!("{}\n\n{}", eval_code_help, pretty_labels);
    section("Counterexample", &debug_help)
}

fn section_panic(thread_err: &ThreadErr) -> String {
    let pretty_error = match thread_err.error_string() {
        Some(error) => format!("Error: {:?}", error),
        None => "The error has an unknown type and cannot be displayed".to_string(),
    };
    section("Panic", &pretty_error)
}
