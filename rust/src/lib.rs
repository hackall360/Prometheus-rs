//! Scaffold for a Rust port of the Prometheus Lua obfuscator.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod obfuscator;
pub mod config;

pub use obfuscator::obfuscate;
pub use config::{Config, load_preset, LuaVersion};

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true);
    }
}

