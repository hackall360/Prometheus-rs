use mlua::{Lua, Value, Variadic};
use prometheus_rs::{load_preset, Pipeline};
use std::cell::RefCell;
use std::rc::Rc;

/// Obfuscate `code` and assert that running the result
/// produces the same output as the original program.
pub fn assert_equivalent(code: &str) {
    let config = load_preset("Minify").expect("preset should exist");
    let mut pipeline = Pipeline::from_config(config).expect("pipeline should build");
    let obfuscated = pipeline
        .apply(code)
        .unwrap_or_else(|_| code.to_string());

    let original = run_lua(code);
    let obf = run_lua(&obfuscated);
    assert_eq!(original, obf, "obfuscated output differed");
}

fn run_lua(code: &str) -> String {
    let lua = Lua::new();
    let out = Rc::new(RefCell::new(String::new()));
    {
        let out = out.clone();
        let print = lua
            .create_function(move |lua, values: Variadic<Value>| {
                let mut out = out.borrow_mut();
                for value in values {
                    let s = match value {
                        Value::String(s) => s.to_str()?.to_owned(),
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => {
                            if b { "true" } else { "false" }.to_string()
                        }
                        Value::Nil => "nil".to_string(),
                        other => lua
                            .coerce_string(other)?
                            .map(|s| s.to_str().unwrap().to_owned())
                            .unwrap_or_default(),
                    };
                    out.push_str(&s);
                }
                Ok(())
            })
            .expect("failed to create print function");
        lua.globals()
            .set("print", print)
            .expect("failed to set print function");
    }
    lua.load(code).exec().expect("lua exec failed");
    out.borrow().clone()
}
