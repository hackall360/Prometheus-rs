//! Scaffold for a Rust port of the Prometheus Lua obfuscator.

pub mod ast;
pub mod colors;
pub mod config;
pub mod lexer;
pub mod logger;
pub mod lua;
pub mod name_generators;
pub mod obfuscator;
pub mod parser;
pub mod pipeline;
pub mod random_literals;
pub mod random_strings;
pub mod util;

pub use config::{Config, load_preset};
pub use logger::{LogLevel, Logger};
pub use lua::{LuaConventions, LuaVersion};
pub use obfuscator::obfuscate;
pub use pipeline::Pipeline;

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true);
    }
}
