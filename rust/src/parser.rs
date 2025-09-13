//! Parser that builds an AST from tokens.

use crate::ast::{
    AstNode, Block, Expression, ParseError, ParseResult, ParseWarning, Statement,
};
use crate::lexer::{Token, TokenKind, TokenValue};
use crate::lua::LuaVersion;

/// Parse a slice of tokens into an [`AstNode`].
pub fn parse(tokens: &[Token], version: LuaVersion) -> Result<ParseResult, ParseError> {
    let mut parser = Parser::new(tokens, version);
    let block = parser.parse_block()?;
    let ast = AstNode::new(block);
    Ok(ParseResult::new(ast, parser.warnings))
}

struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
    version: LuaVersion,
    warnings: Vec<ParseWarning>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token], version: LuaVersion) -> Self {
        Self { tokens, index: 0, version, warnings: Vec::new() }
    }

    fn current(&self) -> &'a Token {
        &self.tokens[self.index]
    }

    fn advance(&mut self) {
        if self.index < self.tokens.len() - 1 {
            self.index += 1;
        }
    }

    fn token_string<'b>(&self, token: &'b Token) -> Option<&'b str> {
        if let TokenValue::String(ref s) = token.value {
            Some(s)
        } else {
            None
        }
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let mut statements = Vec::new();
        while self.current().kind != TokenKind::Eof {
            // Skip semicolons; LuaU warns about them.
            if self.current().kind == TokenKind::Symbol {
                if let Some(";") = self.token_string(self.current()) {
                    if matches!(self.version, LuaVersion::LuaU) {
                        self.warnings.push(ParseWarning::new(
                            "Unnecessary semicolon in LuaU",
                            self.current().line,
                            self.current().column,
                        ));
                    }
                    self.advance();
                    continue;
                }
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        Ok(Block::new(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        if self.current().kind == TokenKind::Keyword {
            if let Some(kw) = self.token_string(self.current()) {
                match kw {
                    "local" => {
                        self.advance();
                        return self.parse_local_assignment();
                    }
                    "return" => {
                        self.advance();
                        if self.current().kind == TokenKind::Eof {
                            return Ok(Statement::Return(None));
                        }
                        // Attempt to parse expression; on error return error at expression start.
                        let expr = self.parse_expression()?;
                        return Ok(Statement::Return(Some(expr)));
                    }
                    "break" => {
                        self.advance();
                        return Ok(Statement::Break);
                    }
                    "continue" => {
                        let tok = self.current().clone();
                        self.advance();
                        if matches!(self.version, LuaVersion::Lua51) {
                            return Err(ParseError::new(
                                "`continue` is not supported in Lua 5.1",
                                tok.line,
                                tok.column,
                            ));
                        } else {
                            return Ok(Statement::Continue);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Fallback: assignment or expression statement
        self.parse_assignment_or_expr()
    }

    fn parse_local_assignment(&mut self) -> Result<Statement, ParseError> {
        let tok = self.current().clone();
        if tok.kind != TokenKind::Ident {
            return Err(ParseError::new("expected identifier after `local`", tok.line, tok.column));
        }
        let name = self.token_string(&tok).unwrap().to_string();
        self.advance();

        let eq_tok = self.current().clone();
        if !(eq_tok.kind == TokenKind::Symbol && self.token_string(&eq_tok) == Some("=")) {
            return Err(ParseError::new("expected '=' in local assignment", eq_tok.line, eq_tok.column));
        }
        self.advance();
        let expr = self.parse_expression()?;
        Ok(Statement::LocalAssignment { name, expr })
    }

    fn parse_assignment_or_expr(&mut self) -> Result<Statement, ParseError> {
        let tok = self.current().clone();
        if tok.kind == TokenKind::Ident {
            let name = self.token_string(&tok).unwrap().to_string();
            self.advance();
            if self.current().kind == TokenKind::Symbol
                && self.token_string(self.current()) == Some("=")
            {
                self.advance();
                let expr = self.parse_expression()?;
                return Ok(Statement::Assignment { name, expr });
            } else {
                let expr = Expression::Variable(name);
                return Ok(Statement::Expression(expr));
            }
        }

        // Otherwise treat as expression statement
        let expr = self.parse_expression()?;
        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_binary_expression()
    }

    fn parse_binary_expression(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary()?;
        loop {
            if self.current().kind == TokenKind::Symbol {
                if let Some(op @ "+") | Some(op @ "-") = self.token_string(self.current()) {
                    self.advance();
                    let right = self.parse_primary()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        op: op.to_string(),
                        right: Box::new(right),
                    };
                    continue;
                }
            }
            break;
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let tok = self.current().clone();
        match tok.kind {
            TokenKind::Number => {
                if let TokenValue::Number(n) = tok.value {
                    self.advance();
                    Ok(Expression::Number(n))
                } else {
                    unreachable!()
                }
            }
            TokenKind::String => {
                if let TokenValue::String(s) = tok.value.clone() {
                    self.advance();
                    Ok(Expression::String(s))
                } else {
                    unreachable!()
                }
            }
            TokenKind::Ident => {
                let name = self.token_string(&tok).unwrap().to_string();
                self.advance();
                Ok(Expression::Variable(name))
            }
            TokenKind::Symbol => {
                if self.token_string(&tok) == Some("(") {
                    self.advance();
                    let expr = self.parse_expression()?;
                    let close = self.current().clone();
                    if close.kind == TokenKind::Symbol && self.token_string(&close) == Some(")") {
                        self.advance();
                        Ok(expr)
                    } else {
                        Err(ParseError::new("expected ')'", close.line, close.column))
                    }
                } else {
                    Err(ParseError::new(
                        format!("unexpected symbol `{}`", self.token_string(&tok).unwrap_or("")),
                        tok.line,
                        tok.column,
                    ))
                }
            }
            _ => Err(ParseError::new(
                "unexpected token in expression",
                tok.line,
                tok.column,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn parse_local_assignment() {
        let tokens = tokenize("local a = 1", LuaVersion::Lua51);
        let result = parse(&tokens, LuaVersion::Lua51).unwrap();
        assert!(result.warnings.is_empty());
        assert_eq!(
            result.ast.block.statements,
            vec![Statement::LocalAssignment { name: "a".into(), expr: Expression::Number(1.0) }]
        );
    }

    #[test]
    fn parse_continue_luau() {
        let tokens = tokenize("continue", LuaVersion::LuaU);
        let result = parse(&tokens, LuaVersion::LuaU).unwrap();
        assert!(result.warnings.is_empty());
        assert_eq!(result.ast.block.statements, vec![Statement::Continue]);
    }

    #[test]
    fn parse_continue_lua51_error() {
        // Tokenize using LuaU so `continue` becomes a keyword, then parse as Lua51.
        let tokens = tokenize("continue", LuaVersion::LuaU);
        let err = parse(&tokens, LuaVersion::Lua51).unwrap_err();
        assert!(err.message.contains("continue"));
    }

    #[test]
    fn semicolon_warning_in_luau() {
        let tokens = tokenize("a = 1;", LuaVersion::LuaU);
        let result = parse(&tokens, LuaVersion::LuaU).unwrap();
        assert_eq!(result.warnings.len(), 1);
    }
}

