#[path = "common/mod.rs"]
mod common;

#[test]
fn closures_program() {
    let code = r#"local arr = {}
for i = 1, 100 do
        local x;
        x = (x or 1) + i;
        arr[i] = function()
                return x;
        end
end

for i, func in ipairs(arr) do
        print(func())
end
"#;
    common::assert_equivalent(code);
}
