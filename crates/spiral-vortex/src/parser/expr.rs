//! Expression parser — recursive descent with Pratt parsing for
//! operator precedence.

use super::pratt::infix_binding_power;
use super::Parser;
use crate::ast::expr::*;
use crate::ast::Span;
use crate::error::{VortexError, VortexResult};
use crate::lexer::token::TokenKind;

/// Parse an expression with the given minimum binding power.
///
/// Uses Pratt parsing: each operator has a left binding power (lbp)
/// and the parser recurses with the operator's lbp as the minimum
/// for the right-hand side.
pub fn parse_expression(p: &mut Parser, min_bp: u32) -> VortexResult<Expr> {
    let _line = p.current().line;
    let _col = p.current().col;

    let mut left = parse_prefix(p)?;

    loop {
        let bp = infix_binding_power(p.current_kind());
        if bp == 0 || bp < min_bp {
            break;
        }
        left = parse_infix(p, left, bp)?;
    }

    Ok(left)
}

fn parse_prefix(p: &mut Parser) -> VortexResult<Expr> {
    let line = p.current().line;
    let col = p.current().col;

    match p.current_kind().clone() {
        TokenKind::Number(n) => {
            p.advance();
            Ok(Expr {
                kind: ExprKind::Number(n),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::String(s) => {
            p.advance();
            Ok(Expr {
                kind: ExprKind::String(s),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::True => {
            p.advance();
            Ok(Expr {
                kind: ExprKind::Bool(true),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::False => {
            p.advance();
            Ok(Expr {
                kind: ExprKind::Bool(false),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Null => {
            p.advance();
            Ok(Expr {
                kind: ExprKind::Null,
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Undefined => {
            p.advance();
            Ok(Expr {
                kind: ExprKind::Undefined,
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::This => {
            p.advance();
            Ok(Expr {
                kind: ExprKind::This,
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Identifier(name) => {
            p.advance();
            Ok(Expr {
                kind: ExprKind::Identifier(name),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::LeftParen => {
            p.advance();
            let expr = parse_expression(p, 0)?;
            p.expect(&TokenKind::RightParen)?;
            Ok(expr)
        }
        TokenKind::LeftBracket => parse_array_literal(p, line, col),
        TokenKind::LeftBrace => parse_object_literal(p, line, col),
        TokenKind::Minus => {
            p.advance();
            let operand = parse_prefix(p)?;
            Ok(Expr {
                kind: ExprKind::Unary(UnaryOp::Neg, Box::new(operand), PrefixOrPostfix::Prefix),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Plus => {
            p.advance();
            let operand = parse_prefix(p)?;
            Ok(Expr {
                kind: ExprKind::Unary(UnaryOp::Pos, Box::new(operand), PrefixOrPostfix::Prefix),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Bang => {
            p.advance();
            let operand = parse_prefix(p)?;
            Ok(Expr {
                kind: ExprKind::Unary(UnaryOp::Not, Box::new(operand), PrefixOrPostfix::Prefix),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Typeof => {
            p.advance();
            let operand = parse_prefix(p)?;
            Ok(Expr {
                kind: ExprKind::Unary(UnaryOp::Typeof, Box::new(operand), PrefixOrPostfix::Prefix),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Void => {
            p.advance();
            let operand = parse_prefix(p)?;
            Ok(Expr {
                kind: ExprKind::Unary(UnaryOp::Void, Box::new(operand), PrefixOrPostfix::Prefix),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Tilde => {
            p.advance();
            let operand = parse_prefix(p)?;
            Ok(Expr {
                kind: ExprKind::Unary(UnaryOp::BitNot, Box::new(operand), PrefixOrPostfix::Prefix),
                span: Span::new(0, 0, line, col),
            })
        }
        other => Err(VortexError::Parse {
            message: format!("unexpected token in expression: {other:?}"),
            line,
            col,
        }),
    }
}

fn parse_infix(p: &mut Parser, left: Expr, bp: u32) -> VortexResult<Expr> {
    let line = p.current().line;
    let col = p.current().col;

    match p.current_kind().clone() {
        TokenKind::Plus => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::Add, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Minus => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::Sub, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Star => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::Mul, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Slash => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::Div, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Percent => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::Mod, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::StarStar => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::Pow, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Lt => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::Lt, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::LtEq => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::LtEq, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Gt => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::Gt, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::GtEq => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::GtEq, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::EqEq => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::EqEq, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::EqEqEq => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::EqEqEq, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::BangEq => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::BangEq, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::BangEqEq => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Binary(BinaryOp::BangEqEq, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::AmpAmp => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Logical(LogicalOp::And, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::PipePipe => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Logical(LogicalOp::Or, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::QuestionQuestion => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Logical(
                    LogicalOp::NullishCoalescing,
                    Box::new(left),
                    Box::new(right),
                ),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Dot => {
            p.advance();
            if let TokenKind::Identifier(name) = p.current_kind() {
                let name = name.clone();
                p.advance();
                Ok(Expr {
                    kind: ExprKind::Member(
                        MemberOp::Dot,
                        Box::new(left),
                        Box::new(Expr {
                            kind: ExprKind::Identifier(name),
                            span: Span::new(0, 0, line, col),
                        }),
                    ),
                    span: Span::new(0, 0, line, col),
                })
            } else {
                Err(VortexError::Parse {
                    message: "expected identifier after '.'".into(),
                    line,
                    col,
                })
            }
        }
        TokenKind::LeftBracket => {
            p.advance();
            let prop = parse_expression(p, 0)?;
            p.expect(&TokenKind::RightBracket)?;
            Ok(Expr {
                kind: ExprKind::Member(MemberOp::Bracket, Box::new(left), Box::new(prop)),
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::LeftParen => {
            p.advance();
            let mut args = Vec::new();
            if p.current_kind() != &TokenKind::RightParen {
                loop {
                    args.push(parse_expression(p, 0)?);
                    if p.current_kind() == &TokenKind::Comma {
                        p.advance();
                    } else {
                        break;
                    }
                }
            }
            p.expect(&TokenKind::RightParen)?;
            Ok(Expr {
                kind: ExprKind::Call {
                    callee: Box::new(left),
                    args,
                },
                span: Span::new(0, 0, line, col),
            })
        }
        TokenKind::Assign => {
            p.advance();
            let right = parse_expression(p, bp)?;
            Ok(Expr {
                kind: ExprKind::Assign(AssignOp::Assign, Box::new(left), Box::new(right)),
                span: Span::new(0, 0, line, col),
            })
        }
        other => Err(VortexError::Parse {
            message: format!("unexpected infix operator: {other:?}"),
            line,
            col,
        }),
    }
}

fn parse_array_literal(p: &mut Parser, line: u32, col: u32) -> VortexResult<Expr> {
    p.advance(); // [
    let mut elements = Vec::new();
    while p.current_kind() != &TokenKind::RightBracket {
        if p.current_kind() == &TokenKind::Comma {
            elements.push(None);
            p.advance();
        } else {
            elements.push(Some(parse_expression(p, 0)?));
            if p.current_kind() == &TokenKind::Comma {
                p.advance();
            }
        }
    }
    p.expect(&TokenKind::RightBracket)?;
    Ok(Expr {
        kind: ExprKind::Array(elements),
        span: Span::new(0, 0, line, col),
    })
}

fn parse_object_literal(p: &mut Parser, line: u32, col: u32) -> VortexResult<Expr> {
    p.advance(); // {
    let mut props = Vec::new();
    while p.current_kind() != &TokenKind::RightBrace {
        let key = match p.current_kind().clone() {
            TokenKind::Identifier(name) => {
                p.advance();
                PropertyKey::Ident(name)
            }
            TokenKind::String(s) => {
                p.advance();
                PropertyKey::String(s)
            }
            TokenKind::Number(n) => {
                p.advance();
                PropertyKey::Number(n)
            }
            _ => {
                return Err(VortexError::Parse {
                    message: "expected property name".into(),
                    line: p.current().line,
                    col: p.current().col,
                });
            }
        };
        p.expect(&TokenKind::Colon)?;
        let value = parse_expression(p, 0)?;
        props.push(Property {
            key,
            value,
            kind: PropertyKind::Init,
        });
        if p.current_kind() == &TokenKind::Comma {
            p.advance();
        }
    }
    p.expect(&TokenKind::RightBrace)?;
    Ok(Expr {
        kind: ExprKind::Object(props),
        span: Span::new(0, 0, line, col),
    })
}
