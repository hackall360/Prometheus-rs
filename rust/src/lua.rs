use serde::Deserialize;

/// Supported Lua language versions.
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum LuaVersion {
    Lua51,
    LuaU,
}

impl Default for LuaVersion {
    fn default() -> Self {
        LuaVersion::Lua51
    }
}

/// Language conventions for a particular [`LuaVersion`].
#[derive(Debug, Clone)]
pub struct LuaConventions {
    pub keywords: &'static [&'static str],
    pub symbol_chars: &'static str,
    pub max_symbol_length: usize,
    pub symbols: &'static [&'static str],
    pub ident_chars: &'static str,
    pub number_chars: &'static str,
    pub hex_number_chars: &'static str,
    pub binary_number_chars: &'static [&'static str],
    pub decimal_exponent: &'static [&'static str],
    pub hexadecimal_nums: &'static [&'static str],
    pub binary_nums: &'static [&'static str],
    pub decimal_separators: Option<&'static [&'static str]>,
    pub escape_sequences: &'static [(char, char)],
    pub numerical_escapes: bool,
    pub escape_z_ignore_next_whitespace: bool,
    pub hex_escapes: bool,
    pub unicode_escapes: bool,
}

impl LuaVersion {
    /// Get the conventions associated with this Lua version.
    pub fn conventions(&self) -> &'static LuaConventions {
        match self {
            LuaVersion::Lua51 => &LUA51_CONVENTIONS,
            LuaVersion::LuaU => &LUAU_CONVENTIONS,
        }
    }
}

/// Conventions for Lua 5.1.
pub static LUA51_CONVENTIONS: LuaConventions = LuaConventions {
    keywords: &[
        "and", "break", "do", "else", "elseif",
        "end", "false", "for", "function", "if",
        "in", "local", "nil", "not", "or",
        "repeat", "return", "then", "true", "until", "while",
    ],
    symbol_chars: "+-*/%^#=~<>(){}[];:,.",
    max_symbol_length: 3,
    symbols: &[
        "+", "-", "*", "/", "%", "^", "#",
        "==", "~=", "<=", ">=", "<", ">", "=",
        "(", ")", "{", "}", "[", "]",
        ";", ":", ",", ".", "..", "...",
    ],
    ident_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
    number_chars: "0123456789",
    hex_number_chars: "0123456789abcdefABCDEF",
    binary_number_chars: &["0", "1"],
    decimal_exponent: &["e", "E"],
    hexadecimal_nums: &["x", "X"],
    binary_nums: &["b", "B"],
    decimal_separators: None,
    escape_sequences: &[
        ('a', '\u{07}'),
        ('b', '\u{08}'),
        ('f', '\u{0C}'),
        ('n', '\n'),
        ('r', '\r'),
        ('t', '\t'),
        ('v', '\u{0B}'),
        ('\\', '\\'),
        ('"', '"'),
        ('\'', '\'')
    ],
    numerical_escapes: true,
    escape_z_ignore_next_whitespace: true,
    hex_escapes: true,
    unicode_escapes: true,
};

/// Conventions for the LuaU dialect.
pub static LUAU_CONVENTIONS: LuaConventions = LuaConventions {
    keywords: &[
        "and", "break", "do", "else", "elseif", "continue",
        "end", "false", "for", "function", "if",
        "in", "local", "nil", "not", "or",
        "repeat", "return", "then", "true", "until", "while",
    ],
    symbol_chars: "+-*/%^#=~<>(){}[];:,.",
    max_symbol_length: 3,
    symbols: &[
        "+", "-", "*", "/", "%", "^", "#",
        "==", "~=", "<=", ">=", "<", ">", "=",
        "+=", "-=", "/=", "%=", "^=", "..=", "*=",
        "(", ")", "{", "}", "[", "]",
        ";", ":", ",", ".", "..", "...",
        "::", "->", "?", "|", "&",
    ],
    ident_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789",
    number_chars: "0123456789",
    hex_number_chars: "0123456789abcdefABCDEF",
    binary_number_chars: &["0", "1"],
    decimal_exponent: &["e", "E"],
    hexadecimal_nums: &["x", "X"],
    binary_nums: &["b", "B"],
    decimal_separators: Some(&["_"]),
    escape_sequences: &[
        ('a', '\u{07}'),
        ('b', '\u{08}'),
        ('f', '\u{0C}'),
        ('n', '\n'),
        ('r', '\r'),
        ('t', '\t'),
        ('v', '\u{0B}'),
        ('\\', '\\'),
        ('"', '"'),
        ('\'', '\'')
    ],
    numerical_escapes: true,
    escape_z_ignore_next_whitespace: true,
    hex_escapes: true,
    unicode_escapes: true,
};

