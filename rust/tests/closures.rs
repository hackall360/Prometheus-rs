#[path = "common/mod.rs"]
mod common;

#[test]
fn closures_program() {
    let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/closures.lua"));
    common::assert_equivalent(code);
}
