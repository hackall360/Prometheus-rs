local args = {...}
local cmd = {"cargo", "run", "--manifest-path", "rust/Cargo.toml", "--bin", "prometheus", "--"}
for _, a in ipairs(args) do
    cmd[#cmd + 1] = string.format("%q", a)
end
os.exit(os.execute(table.concat(cmd, " ")))
