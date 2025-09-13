//! Abstract Syntax Tree definitions for Lua code.

/// Root AST node representing a Lua chunk.
#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    pub block: Block,
}

impl AstNode {
    /// Create a new [`AstNode`] from a block of statements.
    pub fn new(block: Block) -> Self {
        Self { block }
    }
}

/// Sequence of Lua statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

/// Statements supported by the simplified parser.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// `local name = expr`
    LocalAssignment { name: String, expr: Expression },
    /// `name = expr`
    Assignment { name: String, expr: Expression },
    /// `return expr?`
    Return(Option<Expression>),
    /// `break`
    Break,
    /// `continue` – LuaU only.
    Continue,
    /// Bare expression as a statement.
    Expression(Expression),
}

/// Expressions supported by the simplified parser.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Variable(String),
    /// Binary operator expression, such as `a + b`.
    BinaryOp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
}

/// Parsing produced an error.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self { message: message.into(), line, column }
    }
}

/// Parsing produced a warning that did not abort parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseWarning {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseWarning {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self { message: message.into(), line, column }
    }
}

/// Result of a parse operation.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseResult {
    pub ast: AstNode,
    pub warnings: Vec<ParseWarning>,
}

impl ParseResult {
    pub fn new(ast: AstNode, warnings: Vec<ParseWarning>) -> Self {
        Self { ast, warnings }
    }
}

