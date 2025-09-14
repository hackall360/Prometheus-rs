#[path = "common/mod.rs"]
mod common;

#[test]
fn loops_program() {
    let code = r#"local x = {};
for i = 1, 100 do
    x[i] = i;
end

for i, v in ipairs(x) do
    print("x[" .. i .. "] = " .. v);
end
"#;
    common::assert_equivalent(code);
}
