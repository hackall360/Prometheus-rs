#[path = "common/mod.rs"]
mod common;

#[test]
fn primes_program() {
    let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/primes.lua"));
    common::assert_equivalent(code);
}
