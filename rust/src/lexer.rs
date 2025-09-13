//! Tokenizer for Lua source code.

use std::collections::{HashMap, HashSet};

use crate::config;
use crate::logger::Logger;
use crate::lua::{LuaConventions, LuaVersion};
use crate::util::{chararray, escape, lookupify};

/// Kinds of tokens produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Eof,
    Keyword,
    Symbol,
    Ident,
    Number,
    String,
}

/// Value stored inside a [`Token`].
#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    String(String),
    Number(f64),
}

/// Representation of a token in Lua source.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
    pub start: usize,
    pub end: usize,
    pub source: String,
    pub line: usize,
    pub column: usize,
    pub annotations: Vec<String>,
}

struct Position {
    line: usize,
    column: usize,
}

struct Lexer<'a> {
    input: &'a [u8],
    index: usize,
    length: usize,
    conv: &'static LuaConventions,
    logger: Logger,

    // lookup tables
    number_chars: HashSet<char>,
    hex_number_chars: HashSet<char>,
    binary_number_chars: HashSet<char>,
    keywords: HashSet<&'static str>,
    binary_nums: HashSet<char>,
    hexadecimal_nums: HashSet<char>,
    decimal_exponent: HashSet<char>,
    decimal_separators: Option<HashSet<char>>,
    ident_chars: HashSet<char>,
    escape_sequences: HashMap<char, char>,
    numerical_escapes: bool,
    escape_z_ignore_next_whitespace: bool,
    hex_escapes: bool,
    unicode_escapes: bool,
    symbol_chars: HashSet<char>,
    max_symbol_length: usize,
    symbols: HashSet<&'static str>,
    string_start: HashSet<char>,
    whitespace: HashSet<char>,
    annotation_chars: HashSet<char>,
    annotation_start: HashSet<char>,

    annotations: Vec<String>,
    positions: Vec<Position>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str, version: LuaVersion) -> Self {
        let conv = version.conventions();

        let number_chars = lookupify(&chararray(conv.number_chars));
        let hex_number_chars = lookupify(&chararray(conv.hex_number_chars));
        let binary_number_chars = lookupify(
            &conv
                .binary_number_chars
                .iter()
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>(),
        );
        let keywords = lookupify(conv.keywords);
        let binary_nums = lookupify(
            &conv
                .binary_nums
                .iter()
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>(),
        );
        let hexadecimal_nums = lookupify(
            &conv
                .hexadecimal_nums
                .iter()
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>(),
        );
        let decimal_exponent = lookupify(
            &conv
                .decimal_exponent
                .iter()
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>(),
        );
        let decimal_separators = conv.decimal_separators.map(|seps| {
            lookupify(
                &seps
                    .iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<char>>(),
            )
        });
        let ident_chars = lookupify(&chararray(conv.ident_chars));
        let symbol_chars = lookupify(&chararray(conv.symbol_chars));
        let symbols = lookupify(conv.symbols);
        let string_start = lookupify(&['"', '\'']);
        let whitespace = lookupify(&[' ', '\t', '\n', '\r']);
        let annotation_chars = lookupify(&chararray(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_",
        ));
        let annotation_start = lookupify(&chararray("!@"));

        let escape_sequences: HashMap<char, char> = conv.escape_sequences.iter().cloned().collect();

        let mut lexer = Lexer {
            input: input.as_bytes(),
            index: 0,
            length: input.len(),
            conv,
            logger: Logger::default(),
            number_chars,
            hex_number_chars,
            binary_number_chars,
            keywords,
            binary_nums,
            hexadecimal_nums,
            decimal_exponent,
            decimal_separators,
            ident_chars,
            escape_sequences,
            numerical_escapes: conv.numerical_escapes,
            escape_z_ignore_next_whitespace: conv.escape_z_ignore_next_whitespace,
            hex_escapes: conv.hex_escapes,
            unicode_escapes: conv.unicode_escapes,
            symbol_chars,
            max_symbol_length: conv.max_symbol_length,
            symbols,
            string_start,
            whitespace,
            annotation_chars,
            annotation_start,
            annotations: Vec::new(),
            positions: Vec::new(),
        };

        lexer.prepare_positions();
        lexer
    }

    fn prepare_positions(&mut self) {
        let mut line = 1;
        let mut column = 1;
        for &b in self.input.iter() {
            self.positions.push(Position { line, column });
            if b == b'\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        // position for EOF
        self.positions.push(Position { line, column });
    }

    fn get_position(&self, idx: usize) -> (usize, usize) {
        if idx < self.positions.len() {
            let pos = &self.positions[idx];
            (pos.line, pos.column)
        } else {
            let pos = self.positions.last().unwrap();
            (pos.line, pos.column)
        }
    }

    fn peek(&self, n: usize) -> char {
        if self.index + n >= self.length {
            '\0'
        } else {
            self.input[self.index + n] as char
        }
    }

    fn get(&mut self) -> char {
        if self.index >= self.length {
            self.logger
                .error(self.generate_error("Unexpected end of input"));
        }
        let ch = self.input[self.index] as char;
        self.index += 1;
        ch
    }

    fn expect_char(&mut self, c: char) -> char {
        let ch = self.peek(0);
        if ch != c {
            self.logger
                .error(self.generate_error(&format!("Unexpected char '{}'", escape(&ch.to_string()))));
        }
        self.index += 1;
        ch
    }

    fn expect_number_start(&mut self) -> char {
        let ch = self.peek(0);
        if ch != '.' && !self.number_chars.contains(&ch) {
            self.logger
                .error(self.generate_error(&format!("Unexpected char '{}'", escape(&ch.to_string()))));
        }
        self.index += 1;
        ch
    }

    fn expect_ident_char(&mut self) -> char {
        let ch = self.peek(0);
        if !self.ident_chars.contains(&ch) {
            self.logger
                .error(self.generate_error(&format!("Unexpected char '{}'", escape(&ch.to_string()))));
        }
        self.index += 1;
        ch
    }

    fn expect_string_start(&mut self) -> char {
        let ch = self.peek(0);
        if !self.string_start.contains(&ch) {
            self.logger
                .error(self.generate_error(&format!("Unexpected char '{}'", escape(&ch.to_string()))));
        }
        self.index += 1;
        ch
    }

    fn expect_hex_digit(&mut self) -> char {
        let ch = self.peek(0);
        if !self.hex_number_chars.contains(&ch) {
            self.logger
                .error(self.generate_error(&format!("Unexpected char '{}'", escape(&ch.to_string()))));
        }
        self.index += 1;
        ch
    }

    fn is_char(&self, ch: char, n: usize) -> bool {
        self.peek(n) == ch
    }

    fn is_set(&self, set: &HashSet<char>, n: usize) -> bool {
        set.contains(&self.peek(n))
    }

    fn parse_annotation(&mut self) -> Option<char> {
        let ch = self.peek(0);
        if self.annotation_start.contains(&ch) {
            self.index += 1;
            let mut buf = String::new();
            while self.is_set(&self.annotation_chars, 0) {
                buf.push(self.get());
            }
            if !buf.is_empty() {
                self.annotations.push(buf.to_lowercase());
            }
            None
        } else if self.index < self.length {
            Some(self.get())
        } else {
            None
        }
    }

    fn skip_comment(&mut self) -> bool {
        if self.is_char('-', 0) && self.is_char('-', 1) {
            self.index += 2;
            if self.is_char('[', 0) {
                self.index += 1;
                let mut eq_count = 0;
                while self.is_char('=', 0) {
                    self.index += 1;
                    eq_count += 1;
                }
                if self.is_char('[', 0) {
                    self.index += 1;
                    loop {
                        if let Some(ch) = self.parse_annotation() {
                            if ch == ']' {
                                let mut eq2 = 0;
                                while self.is_char('=', 0) {
                                    self.index += 1;
                                    eq2 += 1;
                                }
                                if self.is_char(']', 0) && eq2 == eq_count {
                                    self.index += 1;
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            while self.index < self.length {
                match self.parse_annotation() {
                    Some('\n') => break,
                    _ => {}
                }
            }
            return true;
        }
        false
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.skip_comment() {}
        while self.is_set(&self.whitespace, 0) {
            self.index += 1;
            while self.skip_comment() {}
        }
    }

    fn int(&mut self, chars: &HashSet<char>, seps: Option<&HashSet<char>>) -> String {
        let mut buf = String::new();
        loop {
            if self.is_set(chars, 0) {
                buf.push(self.get());
            } else if let Some(s) = seps {
                if s.contains(&self.peek(0)) {
                    self.index += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        buf
    }

    fn number(&mut self) -> Token {
        let start = self.index;
        let first = self.expect_number_start();
        let mut source = first.to_string();
        let seps = self.decimal_separators.clone();

        if source == "0" {
            if self.binary_nums.contains(&self.peek(0)) {
                self.index += 1;
                let binary_chars = self.binary_number_chars.clone();
                let digits = self.int(&binary_chars, seps.as_ref());
                let value = i64::from_str_radix(&digits, 2).unwrap() as f64;
                return self.token(start, TokenKind::Number, TokenValue::Number(value));
            }
            if self.hexadecimal_nums.contains(&self.peek(0)) {
                self.index += 1;
                let hex_chars = self.hex_number_chars.clone();
                let digits = self.int(&hex_chars, seps.as_ref());
                let value = i64::from_str_radix(&digits, 16).unwrap() as f64;
                return self.token(start, TokenKind::Number, TokenValue::Number(value));
            }
        }

        if source == "." {
            let number_chars = self.number_chars.clone();
            source.push_str(&self.int(&number_chars, seps.as_ref()));
        } else {
            let number_chars = self.number_chars.clone();
            source.push_str(&self.int(&number_chars, seps.as_ref()));
            if self.is_char('.', 0) {
                source.push(self.get());
                let number_chars = self.number_chars.clone();
                source.push_str(&self.int(&number_chars, seps.as_ref()));
            }
        }

        if self.decimal_exponent.contains(&self.peek(0)) {
            source.push(self.get());
            if self.peek(0) == '+' || self.peek(0) == '-' {
                source.push(self.get());
            }
            let number_chars = self.number_chars.clone();
            let exp = self.int(&number_chars, seps.as_ref());
            if exp.is_empty() {
                self.logger
                    .error(self.generate_error("Expected a valid exponent"));
            }
            source.push_str(&exp);
        }

        let value: f64 = source.parse().unwrap_or(0.0);
        self.token(start, TokenKind::Number, TokenValue::Number(value))
    }

    fn ident(&mut self) -> Token {
        let start = self.index;
        let mut source = String::new();
        source.push(self.expect_ident_char());
        while self.is_set(&self.ident_chars, 0) {
            source.push(self.get());
        }

        if self.keywords.contains(source.as_str()) {
            self.token(start, TokenKind::Keyword, TokenValue::String(source))
        } else {
            let tk = self.token(start, TokenKind::Ident, TokenValue::String(source.clone()));
            if source.starts_with(config::IDENT_PREFIX) {
                self.logger.warn(format!(
                    "Warning at Position {}:{}, identifiers should not start with \"{}\" as this may break the program",
                    tk.line, tk.column, config::IDENT_PREFIX
                ));
            }
            tk
        }
    }

    fn single_line_string(&mut self) -> Token {
        let start = self.index;
        let start_char = self.expect_string_start();
        let mut buf = String::new();
        loop {
            if self.is_char(start_char, 0) {
                break;
            }
            let mut ch = self.get();
            if ch == '\n' {
                self.index -= 1;
                self.logger
                    .error(self.generate_error("Unterminated String"));
            }
            if ch == '\\' {
                ch = self.get();
                if let Some(&e) = self.escape_sequences.get(&ch) {
                    ch = e;
                } else if self.numerical_escapes && self.number_chars.contains(&ch) {
                    let mut num = ch.to_string();
                    if self.number_chars.contains(&self.peek(0)) {
                        num.push(self.get());
                    }
                    if self.number_chars.contains(&self.peek(0)) {
                        num.push(self.get());
                    }
                    let value = num.parse::<u8>().unwrap();
                    ch = value as char;
                } else if self.unicode_escapes && ch == 'u' {
                    self.expect_char('{');
                    let mut num = String::new();
                    while self.is_set(&self.hex_number_chars, 0) {
                        num.push(self.get());
                    }
                    self.expect_char('}');
                    let code = u32::from_str_radix(&num, 16).unwrap();
                    ch = std::char::from_u32(code).unwrap_or('\u{FFFD}');
                } else if self.hex_escapes && ch == 'x' {
                    let h = format!(
                        "{}{}",
                        self.expect_hex_digit(),
                        self.expect_hex_digit()
                    );
                    let value = u8::from_str_radix(&h, 16).unwrap();
                    ch = value as char;
                } else if self.escape_z_ignore_next_whitespace && ch == 'z' {
                    while self.is_set(&self.whitespace, 0) {
                        self.index += 1;
                    }
                    continue;
                }
            }
            buf.push(ch);
        }
        self.expect_char(start_char);
        self.token(start, TokenKind::String, TokenValue::String(buf))
    }

    fn multi_line_string(&mut self) -> Option<Token> {
        let start = self.index;
        if self.is_char('[', 0) {
            self.index += 1;
            let mut eq_count = 0;
            while self.is_char('=', 0) {
                self.index += 1;
                eq_count += 1;
            }
            if self.is_char('[', 0) {
                self.index += 1;
                if self.is_char('\n', 0) {
                    self.index += 1;
                }
                let mut value = String::new();
                loop {
                    let ch = self.get();
                    if ch == ']' {
                        let mut eq2 = 0;
                        while self.is_char('=', 0) {
                            eq2 += 1;
                            self.index += 1;
                        }
                        if self.is_char(']', 0) && eq2 == eq_count {
                            self.index += 1;
                            return Some(self.token(start, TokenKind::String, TokenValue::String(value)));
                        } else {
                            value.push(ch);
                        }
                    } else {
                        value.push(ch);
                    }
                }
            }
        }
        self.index = start;
        None
    }

    fn symbol(&mut self) -> Token {
        let start = self.index;
        for len in (1..=self.max_symbol_length).rev() {
            if self.index + len <= self.length {
                let s = std::str::from_utf8(&self.input[self.index..self.index + len]).unwrap();
                if self.symbols.contains(s) {
                    self.index += len;
                    return self.token(start, TokenKind::Symbol, TokenValue::String(s.to_string()));
                }
            }
        }
        self.logger
            .error(self.generate_error("Unknown Symbol"));
    }

    fn token(&mut self, start: usize, kind: TokenKind, value: TokenValue) -> Token {
        let (line, column) = self.get_position(self.index);
        let source = if self.index >= start && self.index <= self.length {
            String::from_utf8_lossy(&self.input[start..self.index]).into_owned()
        } else {
            String::new()
        };
        let annotations = std::mem::take(&mut self.annotations);
        Token {
            kind,
            value,
            start,
            end: self.index,
            source,
            line,
            column,
            annotations,
        }
    }

    fn generate_error(&self, msg: &str) -> String {
        let (line, column) = self.get_position(self.index);
        format!("Lexing Error at Position {}:{}, {}", line, column, msg)
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        let start = self.index;
        if start >= self.length {
            return self.token(
                start,
                TokenKind::Eof,
                TokenValue::String("<EOF>".to_string()),
            );
        }

        if self.is_set(&self.number_chars, 0) {
            return self.number();
        }

        if self.is_set(&self.ident_chars, 0) {
            return self.ident();
        }

        if self.is_set(&self.string_start, 0) {
            return self.single_line_string();
        }

        if self.is_char('[', 0) {
            if let Some(tk) = self.multi_line_string() {
                return tk;
            }
        }

        if self.is_char('.', 0) && self.is_set(&self.number_chars, 1) {
            return self.number();
        }

        if self.is_set(&self.symbol_chars, 0) {
            return self.symbol();
        }

        self.logger.error(self.generate_error(&format!(
            "Unexpected char \"{}\"!",
            escape(&self.peek(0).to_string())
        )))
    }
}

/// Convert Lua source code into a sequence of tokens.
pub fn tokenize(input: &str, version: LuaVersion) -> Vec<Token> {
    let mut lexer = Lexer::new(input, version);
    let mut tokens = Vec::new();
    loop {
        let tk = lexer.next_token();
        let end = tk.kind == TokenKind::Eof;
        tokens.push(tk);
        if end {
            break;
        }
    }
    tokens
}

