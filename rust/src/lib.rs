//! Scaffold for a Rust port of the Prometheus Lua obfuscator.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod obfuscator;

pub use obfuscator::obfuscate;

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true);
    }
}

