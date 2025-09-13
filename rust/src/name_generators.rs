use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, seq::SliceRandom};

use crate::pipeline::NameGenerator;

const MANGLED_VAR_DIGITS: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
const MANGLED_VAR_START: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Generates simple increasing identifiers in the form `_1`, `_2`, …
pub struct NumberGenerator {
    counter: u64,
}

impl NumberGenerator {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl NameGenerator for NumberGenerator {
    fn generate(&mut self) -> String {
        self.counter += 1;
        format!("_{}", self.counter)
    }
}

/// Mimics the `mangled.lua` name generator from the Lua implementation.
pub struct MangledGenerator {
    counter: u64,
}

impl MangledGenerator {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl NameGenerator for MangledGenerator {
    fn generate(&mut self) -> String {
        self.counter += 1;
        let mut id = self.counter;
        let base_start = MANGLED_VAR_START.len() as u64;
        let base_digits = MANGLED_VAR_DIGITS.len() as u64;

        let mut name = String::new();
        let d = (id % base_start) as usize;
        id = (id - d as u64) / base_start;
        name.push(MANGLED_VAR_START[d] as char);

        while id > 0 {
            let d = (id % base_digits) as usize;
            id = (id - d as u64) / base_digits;
            name.push(MANGLED_VAR_DIGITS[d] as char);
        }

        name
    }
}

/// Equivalent to `mangled_shuffled.lua` where the character order is randomised.
pub struct MangledShuffledGenerator {
    counter: u64,
    var_digits: Vec<u8>,
    var_start_digits: Vec<u8>,
}

impl MangledShuffledGenerator {
    pub fn new(seed: u64) -> Self {
        let mut var_digits = MANGLED_VAR_DIGITS.to_vec();
        let mut var_start_digits = MANGLED_VAR_START.to_vec();
        let mut rng = StdRng::seed_from_u64(seed);
        var_digits.shuffle(&mut rng);
        var_start_digits.shuffle(&mut rng);
        Self {
            counter: 0,
            var_digits,
            var_start_digits,
        }
    }
}

impl NameGenerator for MangledShuffledGenerator {
    fn generate(&mut self) -> String {
        self.counter += 1;
        let mut id = self.counter;
        let base_start = self.var_start_digits.len() as u64;
        let base_digits = self.var_digits.len() as u64;

        let mut name = String::new();
        let d = (id % base_start) as usize;
        id = (id - d as u64) / base_start;
        name.push(self.var_start_digits[d] as char);

        while id > 0 {
            let d = (id % base_digits) as usize;
            id = (id - d as u64) / base_digits;
            name.push(self.var_digits[d] as char);
        }

        name
    }
}

const IL_VAR_DIGITS: &[u8] = b"Il1";
const IL_VAR_START: &[u8] = b"Il";
const MIN_CHARACTERS: u32 = 5;
const MAX_INITIAL_CHARACTERS: u32 = 10;

/// Generator producing confusing names consisting only of `I`, `l` and `1`.
pub struct IlGenerator {
    counter: u64,
    offset: u64,
    var_digits: Vec<u8>,
    var_start_digits: Vec<u8>,
}

impl IlGenerator {
    pub fn new(seed: u64) -> Self {
        let mut var_digits = IL_VAR_DIGITS.to_vec();
        let mut var_start_digits = IL_VAR_START.to_vec();
        let mut rng = StdRng::seed_from_u64(seed);
        var_digits.shuffle(&mut rng);
        var_start_digits.shuffle(&mut rng);
        let min = 3u64.pow(MIN_CHARACTERS);
        let max = 3u64.pow(MAX_INITIAL_CHARACTERS);
        let offset = rng.gen_range(min..=max);
        Self {
            counter: 0,
            offset,
            var_digits,
            var_start_digits,
        }
    }
}

impl NameGenerator for IlGenerator {
    fn generate(&mut self) -> String {
        self.counter += 1;
        let mut id = self.counter + self.offset;
        let base_start = self.var_start_digits.len() as u64;
        let base_digits = self.var_digits.len() as u64;

        let mut name = String::new();
        let d = (id % base_start) as usize;
        id = (id - d as u64) / base_start;
        name.push(self.var_start_digits[d] as char);

        while id > 0 {
            let d = (id % base_digits) as usize;
            id = (id - d as u64) / base_digits;
            name.push(self.var_digits[d] as char);
        }

        name
    }
}

/// Generator that assembles identifiers from shuffled common variable names.
pub struct ConfuseGenerator {
    counter: u64,
    names: Vec<&'static str>,
}

impl ConfuseGenerator {
    pub fn new(seed: u64) -> Self {
        let mut names = VAR_NAMES.to_vec();
        let mut rng = StdRng::seed_from_u64(seed);
        names.shuffle(&mut rng);
        Self { counter: 0, names }
    }
}

impl NameGenerator for ConfuseGenerator {
    fn generate(&mut self) -> String {
        self.counter += 1;
        let mut id = self.counter;
        let base = self.names.len() as u64;
        let mut parts: Vec<&'static str> = Vec::new();
        let d = (id % base) as usize;
        id = (id - d as u64) / base;
        parts.push(self.names[d]);
        while id > 0 {
            let d = (id % base) as usize;
            id = (id - d as u64) / base;
            parts.push(self.names[d]);
        }
        parts.join("_")
    }
}

const VAR_NAMES: [&str; 141] = [
    "index",
    "iterator",
    "length",
    "size",
    "key",
    "value",
    "data",
    "count",
    "increment",
    "include",
    "string",
    "number",
    "type",
    "void",
    "int",
    "float",
    "bool",
    "char",
    "double",
    "long",
    "short",
    "unsigned",
    "signed",
    "program",
    "factory",
    "Factory",
    "new",
    "delete",
    "table",
    "array",
    "object",
    "class",
    "arr",
    "obj",
    "cls",
    "dir",
    "directory",
    "isWindows",
    "isLinux",
    "game",
    "roblox",
    "gmod",
    "gsub",
    "gmatch",
    "gfind",
    "onload",
    "load",
    "loadstring",
    "loadfile",
    "dofile",
    "require",
    "parse",
    "byte",
    "code",
    "bytecode",
    "idx",
    "const",
    "loader",
    "loaders",
    "module",
    "export",
    "exports",
    "import",
    "imports",
    "package",
    "packages",
    "_G",
    "math",
    "os",
    "io",
    "write",
    "print",
    "read",
    "readline",
    "readlines",
    "close",
    "flush",
    "open",
    "popen",
    "tmpfile",
    "tmpname",
    "rename",
    "remove",
    "seek",
    "setvbuf",
    "lines",
    "call",
    "apply",
    "raise",
    "pcall",
    "xpcall",
    "coroutine",
    "create",
    "resume",
    "status",
    "wrap",
    "yield",
    "debug",
    "traceback",
    "getinfo",
    "getlocal",
    "setlocal",
    "getupvalue",
    "setupvalue",
    "getuservalue",
    "setuservalue",
    "upvalueid",
    "upvaluejoin",
    "sethook",
    "gethook",
    "hookfunction",
    "hooks",
    "error",
    "setmetatable",
    "getmetatable",
    "rand",
    "randomseed",
    "next",
    "ipairs",
    "hasnext",
    "loadlib",
    "searchpath",
    "oldpath",
    "newpath",
    "path",
    "rawequal",
    "rawset",
    "rawget",
    "rawnew",
    "rawlen",
    "select",
    "tonumber",
    "tostring",
    "assert",
    "collectgarbage",
    "a",
    "b",
    "c",
    "i",
    "j",
    "m",
];
