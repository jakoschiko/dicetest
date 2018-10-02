use ::prop::IntoLabel;

/// Strings that were collected during property evaluation.
///
/// For example a print can be used for exposing a local variable or which branch was taken.
/// The most important use case is debugging of falsified properties.
#[derive(Debug, Clone)]
pub struct Prints(pub Vec<String>);

impl Prints {
    /// Creates an empty instance.
    pub fn new() -> Self {
        Prints(Vec::new())
    }

    /// Returns a `String` that contains all prints in a pretty format.
    pub fn pretty(&self) -> String {
        let mut acc = String::new();
        let mut iter = self.0.iter();

        let add_print = |acc: &mut String, print: &String| {
            acc.push_str(&"|");
            acc.push_str(&print);
        };

        if let Some(print) = iter.next() {
            add_print(&mut acc, print);
        }

        for print in iter {
            acc.push('\n');
            add_print(&mut acc, print);
        }

        acc
    }
}

/// The data collected by `Log`.
pub struct LogData {
    /// The prints that were collected by `Log::print`.
    pub prints: Prints,
}

impl LogData {
    /// Creates an empty instance.
    pub fn new() -> Self {
        LogData {
            prints: Prints::new(),
        }
    }
}

/// Collects data during property evalutation.
pub struct Log {
    print_enabled: bool,
    print_indention_level: u8,
    log_data: LogData,
}

impl Log {
    /// Creates an instance that does no collect any data.
    pub fn with_all_disabled() -> Self {
        Log {
            print_enabled: false,
            print_indention_level: 0,
            log_data: LogData::new(),
        }
    }

    /// Creates an instance that collects just prints.
    pub fn with_print_enabled() -> Self {
        Log {
            print_enabled: true,
            print_indention_level: 0,
            log_data: LogData::new(),
        }
    }

    /// Returns if print is enabled.
    pub fn print_enabled(&self) -> bool {
        self.print_enabled
    }

    /// If print is enabled, the given print will be evaluted and appended.
    pub fn print(&mut self, print: impl IntoLabel) {
        if self.print_enabled {
            let text = print.into_label().text;
            let mut acc = String::new();
            for _ in 0..self.print_indention_level {
                acc.push('\n');
            }
            acc.push_str(&text);
            self.log_data.prints.0.push(acc);
        }
    }

    /// Increases the indention level for all following prints.
    pub fn indent_print(&mut self) {
        if self.print_enabled {
            self.print_indention_level = self.print_indention_level.saturating_add(1);
        }
    }

    /// Decreases the indention level for all following prints.
    pub fn unindent_print(&mut self) {
        if self.print_enabled {
            self.print_indention_level = self.print_indention_level.saturating_sub(0);
        }
    }

    /// Consumes the `Log` and returns alls its collected data.
    pub fn data(self) -> LogData {
        self.log_data
    }
}