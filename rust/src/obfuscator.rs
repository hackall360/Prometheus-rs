//! High level interface for obfuscating Lua code.

use crate::lexer::tokenize;
use crate::parser::parse;

/// Obfuscate the provided Lua source code.
pub fn obfuscate(source: &str) -> String {
    let tokens = tokenize(source);
    let _ast = parse(&tokens);

    // Future work: apply transformation pipeline and generate obfuscated code.
    unimplemented!("obfuscation pipeline not yet implemented");
}

