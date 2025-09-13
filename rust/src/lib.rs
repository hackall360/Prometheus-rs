//! Scaffold for a Rust port of the Prometheus Lua obfuscator.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod obfuscator;
pub mod config;
pub mod colors;
pub mod logger;
pub mod util;
pub mod lua;

pub use obfuscator::obfuscate;
pub use config::{Config, load_preset};
pub use lua::{LuaVersion, LuaConventions};
pub use logger::{Logger, LogLevel};

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true);
    }
}
