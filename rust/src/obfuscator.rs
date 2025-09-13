//! High level interface for obfuscating Lua code.

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::lua::LuaVersion;

/// Obfuscate the provided Lua source code.
pub fn obfuscate(source: &str) -> String {
    let version = LuaVersion::Lua51;
    let tokens = tokenize(source, version);
    let result = parse(&tokens, version).expect("failed to parse source");
    for warning in result.warnings {
        println!("warning: {}", warning.message);
    }
    let _ast = result.ast;

    // Future work: apply transformation pipeline and generate obfuscated code.
    unimplemented!("obfuscation pipeline not yet implemented");
}

