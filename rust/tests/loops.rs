#[path = "common/mod.rs"]
mod common;

#[test]
fn loops_program() {
    let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/loops.lua"));
    common::assert_equivalent(code);
}
