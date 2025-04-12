#![forbid(missing_docs)]

//! Implementation of `log`, aiming to emulate `clang`
//!
//! # How to use:
//! To use `clang_log`, first include it, and `log` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! log = "0.4.22"
//! clang_log = "1.0.2"
//! ```
//!
//! Then, initialize it at the start of the program:
//! ```rust
//! use log::Level;
//!
//! clang_log::init(Level::Trace, "clang");
//! ```
//!
//! To use it, just use the macros provided by `log`
//!
//! ```rust
//! use log::*;
//!
//! error!("Could not find files!");
//! ```

use colored::Colorize;
use log::*;

/// Initialize logger with fields
/// # Example
/// ```rust
/// use log::*;
///
/// clang_log::init(Level::Trace, "clang");
/// ```

pub fn init(min_level: Level, prog_name: &str) {
    set_max_level(min_level.to_level_filter());

    let logger = Logger {
        min_level,
        prog_name: String::from(prog_name),
        newline_sep: format!("\n{} ", "    | ".white().bold()),
    };
    if set_boxed_logger(Box::new(logger)).is_err() {
        debug!("Logger initialized twice");
    }
}

/// Logger struct. Use `init()` instead, since `Logger` is useless without it being set as the logger, and `init()` does that anyway.

pub struct Logger {
    /// Minimum level this logger will print. For example: `Level::Trace`
    pub min_level: Level,
    /// Name of the program, set to "clang" in clang. (If clang used clang_log)
    pub prog_name: String,
    /// Constant newline separator, inserted between every newline. Avoids many allocations by storing this as a field.
    pub newline_sep: String,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.min_level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // not quite sure whether to use println or eprintln
        println!(
            "{}: {} {}",
            self.prog_name,
            match record.level() {
                Level::Error => {
                    "error:".red().bold()
                }
                Level::Warn => {
                    "warning:".bright_purple().bold()
                }
                Level::Info => {
                    "info:".bright_black().bold()
                    //"note".black().bold() // Clang Behavior
                }
                Level::Debug => {
                    "debug:".yellow().bold() // Clang doesn't have debug logs
                }
                Level::Trace => {
                    "trace:".white().bold() // Clang doesn't have trace logs
                }
            },
            record.args().to_string().replace('\n', &self.newline_sep)
        );
    }

    fn flush(&self) {}
}
