//! Parser that builds an AST from tokens.

use crate::ast::AstNode;
use crate::lexer::Token;
use crate::lua::LuaVersion;
use crate::util::escape;

/// Parse a slice of tokens into an AST.
pub fn parse(_tokens: &[Token], version: LuaVersion) -> AstNode {
    let _ = version.conventions();
    let _ = escape("test");
    // Implementation will mirror the existing Lua parser.
    unimplemented!("parser not yet implemented");
}

