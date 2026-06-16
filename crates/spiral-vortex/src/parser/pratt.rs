//! Pratt parsing helpers — binding power tables for operator precedence.

use crate::lexer::token::TokenKind;

/// Return the infix binding power (higher = tighter binding).
/// Returns 0 for non-operators (used as the loop break condition).
pub fn infix_binding_power(kind: &TokenKind) -> u32 {
    match kind {
        TokenKind::Assign
        | TokenKind::PlusAssign
        | TokenKind::MinusAssign
        | TokenKind::StarAssign
        | TokenKind::SlashAssign
        | TokenKind::PercentAssign => 2,

        TokenKind::QuestionQuestion => 4,
        TokenKind::PipePipe => 6,
        TokenKind::AmpAmp => 8,

        TokenKind::Pipe => 10,
        TokenKind::Caret => 12,
        TokenKind::Amp => 14,

        TokenKind::EqEq | TokenKind::EqEqEq | TokenKind::BangEq | TokenKind::BangEqEq => 16,

        TokenKind::Lt
        | TokenKind::LtEq
        | TokenKind::Gt
        | TokenKind::GtEq
        | TokenKind::In
        | TokenKind::Instanceof => 18,

        TokenKind::LtLt | TokenKind::GtGt | TokenKind::GtGtGt => 20,
        TokenKind::Plus | TokenKind::Minus => 22,
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 24,
        TokenKind::StarStar => 26,

        TokenKind::Dot | TokenKind::LeftBracket | TokenKind::LeftParen => 30,

        _ => 0,
    }
}
