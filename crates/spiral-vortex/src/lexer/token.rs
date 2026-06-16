//! Token types for the Vortex lexer.

/// A token produced by the lexer, carrying its kind and source position.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub col: u32,
}

impl Token {
    pub fn new(kind: TokenKind, line: u32, col: u32) -> Self {
        Self { kind, line, col }
    }
}

/// Every kind of token the lexer can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // -- Literals ---------------------------------------------------------------
    /// Numeric literal (all numbers stored as f64).
    Number(f64),
    /// String literal (contents without quotes).
    String(String),
    /// Template literal (contents without backticks).
    /// Template expressions (`${}`) are not yet supported.
    Template(String),

    // -- Keywords ---------------------------------------------------------------
    True,
    False,
    Null,
    Undefined,
    This,
    If,
    Else,
    For,
    While,
    Do,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    Return,
    Throw,
    Try,
    Catch,
    Finally,
    Function,
    Class,
    Extends,
    New,
    Delete,
    Typeof,
    Void,
    In,
    Instanceof,
    Var,
    Let,
    Const,
    Async,
    Await,
    Yield,
    Import,
    Export,
    From,
    Of,
    Super,
    Debugger,
    With,
    Static,
    Get,
    Set,

    // -- Identifiers ------------------------------------------------------------
    Identifier(String),

    // -- Punctuation ------------------------------------------------------------
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    Question,     // ?
    Tilde,        // ~

    // -- Operators --------------------------------------------------------------
    Plus,             // +
    Minus,            // -
    Star,             // *
    Slash,            // /
    Percent,          // %
    Amp,              // &
    Pipe,             // |
    Caret,            // ^
    CaretAssign,      // ^=
    Lt,               // <
    Gt,               // >
    Bang,             // !
    Assign,           // =
    EqEq,             // ==
    EqEqEq,           // ===
    BangEq,           // !=
    BangEqEq,         // !==
    LtEq,             // <=
    GtEq,             // >=
    LtLt,             // <<
    GtGt,             // >>
    GtGtGt,           // >>>
    AmpAmp,           // &&
    PipePipe,         // ||
    PlusPlus,         // ++
    MinusMinus,       // --
    StarStar,         // **
    Arrow,            // =>
    QuestionQuestion, // ??

    // -- Assignment operators ---------------------------------------------------
    PlusAssign,    // +=
    MinusAssign,   // -=
    StarAssign,    // *=
    SlashAssign,   // /=
    PercentAssign, // %=
    AmpAssign,     // &=
    PipeAssign,    // |=
    LtLtAssign,    // <<=
    GtGtAssign,    // >>=
    GtGtGtAssign,  // >>>=

    // -- Special ----------------------------------------------------------------
    Eof,
    UnterminatedString,
    /// Catch-all for characters not yet handled by the lexer.
    Unknown(char),
}
