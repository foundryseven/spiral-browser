//! Parser — transforms a token stream into an AST.
//!
//! Vortex uses a **recursive descent parser** with Pratt parsing for
//! expressions (operator precedence is handled by binding power, not
//! precedence climbing). This is the same approach V8's parser uses.
//!
//! # Error recovery
//!
//! Phase 1 parser is **fail-fast**: the first syntax error aborts
//! parsing. Phase 2 adds error recovery (panic-mode synchronisation
//! at statement boundaries) so the IDE can show multiple errors.

pub mod expr;
pub mod pratt;
pub mod stmt;

use crate::ast::Program;
use crate::error::{VortexError, VortexResult};
use crate::lexer::token::{Token, TokenKind};

/// Parse a token stream into a `Program` AST node.
pub fn parse(tokens: &[Token]) -> VortexResult<Program> {
    let mut parser = Parser::new(tokens);
    let body = parser.parse_statement_list()?;
    Ok(Program {
        body,
        is_module: false,
    })
}

/// Stateful recursive descent parser.
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Current token.
    pub fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token {
            kind: TokenKind::Eof,
            line: 0,
            col: 0,
        })
    }

    /// Current token kind.
    pub fn current_kind(&self) -> &TokenKind {
        &self.current().kind
    }

    /// Advance past the current token and return it.
    pub fn advance(&mut self) -> Token {
        let tok = self.current().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    /// Expect the current token to be of the given kind, advance past it,
    /// and return it. Returns a parse error otherwise.
    pub fn expect(&mut self, expected: &TokenKind) -> VortexResult<Token> {
        if self.current_kind() == expected {
            Ok(self.advance())
        } else {
            let tok = self.current();
            Err(VortexError::Parse {
                message: format!("expected {expected:?}, found {:?}", tok.kind),
                line: tok.line,
                col: tok.col,
            })
        }
    }

    /// Consume a semicolon (explicit or ASI).
    pub fn eat_semicolon(&mut self) {
        if self.current_kind() == &TokenKind::Semicolon {
            self.advance();
        }
        // ASI: also consume if the next token is `}`, EOF, or a newline
        // before the current position. Phase 1 simplification: we treat
        // semicolons as optional at statement boundaries.
    }

    /// Whether the current token is `}` or EOF (used for ASI).
    pub fn at_statement_end(&self) -> bool {
        matches!(self.current_kind(), TokenKind::RightBrace | TokenKind::Eof)
    }

    /// Parse a list of statements until `}` or EOF.
    pub fn parse_statement_list(&mut self) -> VortexResult<Vec<crate::ast::Stmt>> {
        let mut stmts = Vec::new();
        while !matches!(self.current_kind(), TokenKind::RightBrace | TokenKind::Eof) {
            stmts.push(stmt::parse_statement(self)?);
        }
        Ok(stmts)
    }

    /// Parse a block statement (curly-brace enclosed).
    pub fn parse_block(&mut self) -> VortexResult<Vec<crate::ast::Stmt>> {
        self.expect(&TokenKind::LeftBrace)?;
        let stmts = self.parse_statement_list()?;
        self.expect(&TokenKind::RightBrace)?;
        Ok(stmts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::stmt::StmtKind;
    use crate::lexer;

    fn parse_stmt(src: &str) -> crate::ast::Stmt {
        let tokens = lexer::lex(src);
        stmt::parse_statement(&mut Parser::new(&tokens)).expect("parse failed")
    }

    fn parse_prog(src: &str) -> Program {
        let tokens = lexer::lex(src);
        parse(&tokens).expect("parse failed")
    }

    #[test]
    fn test_parse_number_literal() {
        let prog = parse_prog("42;");
        assert_eq!(prog.body.len(), 1);
    }

    #[test]
    fn test_parse_var_declaration() {
        let stmt = parse_stmt("var x = 10;");
        assert!(matches!(stmt.kind, StmtKind::Var { .. }));
    }

    #[test]
    fn test_parse_if_statement() {
        let stmt = parse_stmt("if (true) { 1; }");
        assert!(matches!(stmt.kind, StmtKind::If { .. }));
    }

    #[test]
    fn test_parse_while_loop() {
        let stmt = parse_stmt("while (true) { break; }");
        assert!(matches!(stmt.kind, StmtKind::While { .. }));
    }
}
