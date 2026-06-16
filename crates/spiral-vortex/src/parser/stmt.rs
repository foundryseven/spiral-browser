//! Statement parser — recursive descent over statement productions.

use super::expr::parse_expression;
use super::Parser;
use crate::ast::expr::BindingPattern;
use crate::ast::stmt::*;
use crate::ast::Span;
use crate::ast::Stmt;
use crate::error::VortexResult;
use crate::lexer::token::TokenKind;

/// Parse a single statement.
pub fn parse_statement(p: &mut Parser) -> VortexResult<Stmt> {
    let line = p.current().line;
    let col = p.current().col;

    match p.current_kind().clone() {
        TokenKind::Var => parse_var_declaration(p, line, col),
        TokenKind::Let => parse_var_declaration(p, line, col),
        TokenKind::Const => parse_var_declaration(p, line, col),
        TokenKind::If => parse_if_statement(p, line, col),
        TokenKind::While => parse_while_statement(p, line, col),
        TokenKind::For => parse_for_statement(p, line, col),
        TokenKind::Return => parse_return_statement(p, line, col),
        TokenKind::Break => parse_break_statement(p, line, col),
        TokenKind::Continue => parse_continue_statement(p, line, col),
        TokenKind::Throw => parse_throw_statement(p, line, col),
        TokenKind::Try => parse_try_statement(p, line, col),
        TokenKind::LeftBrace => {
            let stmts = p.parse_block()?;
            Ok(Stmt {
                kind: StmtKind::Block(stmts),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Semicolon => {
            p.advance();
            Ok(Stmt {
                kind: StmtKind::Empty,
                span: Span::new(0, 0, line, col),
            })
        }
        // Default: expression statement.
        _ => {
            let expr = parse_expression(p, 0)?;
            p.eat_semicolon();
            Ok(Stmt {
                kind: StmtKind::Expr(expr),
                span: Span::new(0, 0, line, col),
            })
        }
    }
}

fn parse_var_declaration(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    let keyword = p.advance().kind.clone();
    let name_token = p.advance();
    let name = match &name_token.kind {
        TokenKind::Identifier(n) => n.clone(),
        other => {
            return Err(crate::error::VortexError::Parse {
                message: format!("expected identifier, found {other:?}"),
                line: name_token.line,
                col: name_token.col,
            });
        }
    };
    let init = if p.current_kind() == &TokenKind::Assign {
        p.advance();
        Some(parse_expression(p, 0)?)
    } else {
        None
    };
    p.eat_semicolon();

    let kind = match keyword {
        TokenKind::Var => StmtKind::Var {
            declarations: vec![VarDeclarator {
                pattern: BindingPattern::Identifier(name),
                init,
            }],
        },
        TokenKind::Let => StmtKind::Let {
            declarations: vec![VarDeclarator {
                pattern: BindingPattern::Identifier(name),
                init,
            }],
        },
        TokenKind::Const => StmtKind::Const {
            declarations: vec![VarDeclarator {
                pattern: BindingPattern::Identifier(name),
                init,
            }],
        },
        _ => unreachable!(),
    };

    Ok(Stmt {
        kind,
        span: Span::new(0, 0, line, col),
    })
}

fn parse_if_statement(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    p.advance(); // if
    p.expect(&TokenKind::LeftParen)?;
    let test = parse_expression(p, 0)?;
    p.expect(&TokenKind::RightParen)?;
    let consequent = Box::new(parse_block_or_stmt(p)?);
    let alternate = if p.current_kind() == &TokenKind::Else {
        p.advance();
        Some(Box::new(parse_block_or_stmt(p)?))
    } else {
        None
    };
    Ok(Stmt {
        kind: StmtKind::If {
            test,
            consequent,
            alternate,
        },
        span: Span::new(0, 0, line, col),
    })
}

fn parse_while_statement(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    p.advance(); // while
    p.expect(&TokenKind::LeftParen)?;
    let test = parse_expression(p, 0)?;
    p.expect(&TokenKind::RightParen)?;
    let body = Box::new(parse_block_or_stmt(p)?);
    Ok(Stmt {
        kind: StmtKind::While { test, body },
        span: Span::new(0, 0, line, col),
    })
}

fn parse_for_statement(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    p.advance(); // for
    p.expect(&TokenKind::LeftParen)?;

    // Phase 1: only `for (var/let/const x = ...; cond; update)` supported.
    // for-in and for-of are a follow-up.
    let init = if p.current_kind() == &TokenKind::Semicolon {
        p.advance();
        None
    } else {
        // Parse as an expression for now (not a full ForInit).
        Some(ForInit::Expr(parse_expression(p, 0)?))
    };

    // Semicolon already eaten above or we expect it now.
    let test = if p.current_kind() != &TokenKind::Semicolon {
        Some(parse_expression(p, 0)?)
    } else {
        None
    };
    p.expect(&TokenKind::Semicolon)?;

    let update = if p.current_kind() != &TokenKind::RightParen {
        Some(parse_expression(p, 0)?)
    } else {
        None
    };
    p.expect(&TokenKind::RightParen)?;

    let body = Box::new(parse_block_or_stmt(p)?);

    Ok(Stmt {
        kind: StmtKind::For {
            init,
            test,
            update,
            body,
        },
        span: Span::new(0, 0, line, col),
    })
}

fn parse_return_statement(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    p.advance(); // return
    let value = if p.current_kind() == &TokenKind::Semicolon
        || p.current_kind() == &TokenKind::RightBrace
        || p.current_kind() == &TokenKind::Eof
    {
        None
    } else {
        Some(parse_expression(p, 0)?)
    };
    p.eat_semicolon();
    Ok(Stmt {
        kind: StmtKind::Return { value },
        span: Span::new(0, 0, line, col),
    })
}

fn parse_break_statement(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    p.advance(); // break
    let label = if let TokenKind::Identifier(name) = p.current_kind() {
        let n = name.clone();
        p.advance();
        Some(n)
    } else {
        None
    };
    p.eat_semicolon();
    Ok(Stmt {
        kind: StmtKind::Break { label },
        span: Span::new(0, 0, line, col),
    })
}

fn parse_continue_statement(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    p.advance(); // continue
    let label = if let TokenKind::Identifier(name) = p.current_kind() {
        let n = name.clone();
        p.advance();
        Some(n)
    } else {
        None
    };
    p.eat_semicolon();
    Ok(Stmt {
        kind: StmtKind::Continue { label },
        span: Span::new(0, 0, line, col),
    })
}

fn parse_throw_statement(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    p.advance(); // throw
    let value = parse_expression(p, 0)?;
    p.eat_semicolon();
    Ok(Stmt {
        kind: StmtKind::Throw { value },
        span: Span::new(0, 0, line, col),
    })
}

fn parse_try_statement(p: &mut Parser, line: u32, col: u32) -> VortexResult<Stmt> {
    p.advance(); // try
    let block = p.parse_block()?;

    let handler = if p.current_kind() == &TokenKind::Catch {
        p.advance(); // catch
        p.expect(&TokenKind::LeftParen)?;
        let param_name = if let TokenKind::Identifier(name) = p.current_kind() {
            let n = name.clone();
            p.advance();
            Some(BindingPattern::Identifier(n))
        } else {
            None
        };
        p.expect(&TokenKind::RightParen)?;
        let catch_body = p.parse_block()?;
        Some(CatchClause {
            param: param_name,
            body: catch_body,
        })
    } else {
        None
    };

    let finalizer = if p.current_kind() == &TokenKind::Finally {
        p.advance(); // finally
        Some(p.parse_block()?)
    } else {
        None
    };

    Ok(Stmt {
        kind: StmtKind::Try {
            block,
            handler,
            finalizer,
        },
        span: Span::new(0, 0, line, col),
    })
}

fn parse_block_or_stmt(p: &mut Parser) -> VortexResult<Stmt> {
    if p.current_kind() == &TokenKind::LeftBrace {
        let line = p.current().line;
        let col = p.current().col;
        let stmts = p.parse_block()?;
        Ok(Stmt {
            kind: StmtKind::Block(stmts),
            span: Span::new(0, 0, line, col),
        })
    } else {
        parse_statement(p)
    }
}
