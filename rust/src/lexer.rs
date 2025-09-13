//! Tokenizer for Lua source code.

/// Representation of a token in Lua.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Placeholder token.
    Unknown,
}

/// Convert Lua source code into a sequence of tokens.
pub fn tokenize(_input: &str) -> Vec<Token> {
    // Implementation will mirror the existing Lua tokenizer.
    unimplemented!("lexer not yet implemented");
}

