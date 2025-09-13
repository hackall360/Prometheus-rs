//! Tokenizer for Lua source code.

use crate::lua::LuaVersion;
use crate::util::{chararray, lookupify};

/// Representation of a token in Lua.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Placeholder token.
    Unknown,
}

/// Convert Lua source code into a sequence of tokens.
pub fn tokenize(_input: &str, version: LuaVersion) -> Vec<Token> {
    // Demonstrate usage of utility helpers for future implementation.
    let chars = chararray(" \t");
    let _lookup = lookupify(&chars);
    let _conventions = version.conventions();
    unimplemented!("lexer not yet implemented");
}

