use clap::ValueEnum;

use crate::{colors::{self, Color}, config};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Log,
    Info,
    Debug,
}

impl LogLevel {
    fn as_u8(self) -> u8 {
        match self {
            LogLevel::Error => 0,
            LogLevel::Warn => 1,
            LogLevel::Log | LogLevel::Info => 2,
            LogLevel::Debug => 3,
        }
    }
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    /// Create a new logger with the given level.
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    /// Get the current log level.
    pub fn level(&self) -> LogLevel {
        self.level
    }

    /// Set the current log level.
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn debug(&self, msg: impl AsRef<str>) {
        if self.level.as_u8() >= LogLevel::Debug.as_u8() {
            println!(
                "{}",
                colors::colorize(
                    format!("{}: {}", config::NAME_UPPER, msg.as_ref()),
                    &[Color::Grey],
                )
            );
        }
    }

    pub fn log(&self, msg: impl AsRef<str>) {
        if self.level.as_u8() >= LogLevel::Log.as_u8() {
            println!(
                "{}",
                colors::colorize(
                    format!("{}: {}", config::NAME_UPPER, msg.as_ref()),
                    &[Color::Magenta],
                )
            );
        }
    }

    pub fn info(&self, msg: impl AsRef<str>) {
        if self.level.as_u8() >= LogLevel::Log.as_u8() {
            println!(
                "{}",
                colors::colorize(
                    format!("{}: {}", config::NAME_UPPER, msg.as_ref()),
                    &[Color::Magenta],
                )
            );
        }
    }

    pub fn warn(&self, msg: impl AsRef<str>) {
        if self.level.as_u8() >= LogLevel::Warn.as_u8() {
            println!(
                "{}",
                colors::colorize(
                    format!("{}: {}", config::NAME_UPPER, msg.as_ref()),
                    &[Color::Yellow],
                )
            );
        }
    }

    pub fn error(&self, msg: impl AsRef<str>) -> ! {
        eprintln!(
            "{}",
            colors::colorize(
                format!("{}: {}", config::NAME_UPPER, msg.as_ref()),
                &[Color::Red],
            )
        );
        panic!("{}", msg.as_ref());
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            level: LogLevel::Log,
        }
    }
}
