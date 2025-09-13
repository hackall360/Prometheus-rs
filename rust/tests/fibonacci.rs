#[path = "common/mod.rs"]
mod common;

#[test]
fn fibonacci_program() {
    let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fibonacci.lua"));
    common::assert_equivalent(code);
}
