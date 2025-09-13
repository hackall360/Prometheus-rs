//! Parser that builds an AST from tokens.

use crate::ast::AstNode;
use crate::lexer::Token;

/// Parse a slice of tokens into an AST.
pub fn parse(_tokens: &[Token]) -> AstNode {
    // Implementation will mirror the existing Lua parser.
    unimplemented!("parser not yet implemented");
}

