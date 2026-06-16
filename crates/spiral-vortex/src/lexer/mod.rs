//! Vortex lexer — transforms source text into a token stream.

pub mod cursor;
pub mod token;

use token::{Token, TokenKind};

/// Lexes the full source string into a vector of tokens.
///
/// This is the Phase 1 (tree-walking) approach — we allocate all tokens
/// up front. A future streaming lexer will yield tokens lazily via an
/// iterator over borrowed source slices.
pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let is_eof = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if is_eof {
            break;
        }
    }
    tokens
}

/// Stateful lexer that advances through source text character by character.
pub struct Lexer<'a> {
    source: &'a str,
    /// Byte offset of the current character.
    pos: usize,
    /// Current line number (1-indexed).
    line: u32,
    /// Current column number (1-indexed, byte-based).
    col: u32,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer over the given source string.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Produce the next token from the source stream.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        if self.pos >= self.source.len() {
            return Token::new(TokenKind::Eof, self.line, self.col);
        }

        let start_line = self.line;
        let start_col = self.col;
        let ch = self.current_char();

        match ch {
            // Single-character tokens
            '(' => {
                self.advance();
                Token::new(TokenKind::LeftParen, start_line, start_col)
            }
            ')' => {
                self.advance();
                Token::new(TokenKind::RightParen, start_line, start_col)
            }
            '{' => {
                self.advance();
                Token::new(TokenKind::LeftBrace, start_line, start_col)
            }
            '}' => {
                self.advance();
                Token::new(TokenKind::RightBrace, start_line, start_col)
            }
            '[' => {
                self.advance();
                Token::new(TokenKind::LeftBracket, start_line, start_col)
            }
            ']' => {
                self.advance();
                Token::new(TokenKind::RightBracket, start_line, start_col)
            }
            ';' => {
                self.advance();
                Token::new(TokenKind::Semicolon, start_line, start_col)
            }
            ',' => {
                self.advance();
                Token::new(TokenKind::Comma, start_line, start_col)
            }
            ':' => {
                self.advance();
                Token::new(TokenKind::Colon, start_line, start_col)
            }
            '?' => {
                self.advance();
                Token::new(TokenKind::Question, start_line, start_col)
            }
            '~' => {
                self.advance();
                Token::new(TokenKind::Tilde, start_line, start_col)
            }

            // Operators that may be doubled or followed by '='
            '+' => self.lex_plus(start_line, start_col),
            '-' => self.lex_minus(start_line, start_col),
            '*' => self.lex_star(start_line, start_col),
            '%' => self.lex_percent(start_line, start_col),
            '&' => self.lex_amp(start_line, start_col),
            '|' => self.lex_pipe(start_line, start_col),
            '^' => {
                self.advance();
                self.eat_eq(
                    TokenKind::Caret,
                    TokenKind::CaretAssign,
                    start_line,
                    start_col,
                )
            }
            '=' => self.lex_eq(start_line, start_col),
            '!' => self.lex_bang(start_line, start_col),
            '<' => self.lex_lt(start_line, start_col),
            '>' => self.lex_gt(start_line, start_col),

            // Dot or number starting with '.'
            '.' => self.lex_dot(start_line, start_col),

            // String literals
            '"' | '\'' => self.lex_string(ch, start_line, start_col),

            // Template literal
            '`' => self.lex_template(start_line, start_col),

            // Division or regex — for now treat '/' as division; regex
            // requires context-awareness (is it after an operand?).
            '/' => self.lex_slash(start_line, start_col),

            // Identifiers and keywords
            _ if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' => {
                self.lex_identifier(start_line, start_col)
            }

            // Numeric literals
            _ if ch.is_ascii_digit() => self.lex_number(start_line, start_col),

            _ => {
                self.advance();
                Token::new(TokenKind::Unknown(ch), start_line, start_col)
            }
        }
    }

    // -- helpers ---------------------------------------------------------------

    fn current_char(&self) -> char {
        self.source[self.pos..].chars().next().unwrap_or('\0')
    }

    fn peek_char(&self) -> char {
        let mut chars = self.source[self.pos..].chars();
        chars.next();
        chars.next().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let ch = self.current_char();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.pos += ch.len_utf8();
        ch
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.pos >= self.source.len() {
                return;
            }
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.advance();
                continue;
            }
            // Line comment
            if ch == '/' && self.peek_char() == '/' {
                while self.pos < self.source.len() && self.current_char() != '\n' {
                    self.advance();
                }
                continue;
            }
            // Block comment
            if ch == '/' && self.peek_char() == '*' {
                self.advance(); // /
                self.advance(); // *
                while self.pos < self.source.len() {
                    if self.current_char() == '*' && self.peek_char() == '/' {
                        self.advance(); // *
                        self.advance(); // /
                        break;
                    }
                    self.advance();
                }
                continue;
            }
            break;
        }
    }

    fn eat_eq(&mut self, plain: TokenKind, assign: TokenKind, line: u32, col: u32) -> Token {
        self.advance(); // consume the operator char
        if self.pos < self.source.len() && self.current_char() == '=' {
            self.advance();
            Token::new(assign, line, col)
        } else {
            Token::new(plain, line, col)
        }
    }

    // -- multi-char operator stubs (to be expanded) ---------------------------

    fn lex_plus(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos < self.source.len() {
            match self.current_char() {
                '+' => {
                    self.advance();
                    Token::new(TokenKind::PlusPlus, line, col)
                }
                '=' => {
                    self.advance();
                    Token::new(TokenKind::PlusAssign, line, col)
                }
                _ => Token::new(TokenKind::Plus, line, col),
            }
        } else {
            Token::new(TokenKind::Plus, line, col)
        }
    }

    fn lex_minus(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos < self.source.len() {
            match self.current_char() {
                '-' => {
                    self.advance();
                    Token::new(TokenKind::MinusMinus, line, col)
                }
                '=' => {
                    self.advance();
                    Token::new(TokenKind::MinusAssign, line, col)
                }
                _ => Token::new(TokenKind::Minus, line, col),
            }
        } else {
            Token::new(TokenKind::Minus, line, col)
        }
    }

    fn lex_star(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos < self.source.len() {
            match self.current_char() {
                '*' => {
                    self.advance();
                    Token::new(TokenKind::StarStar, line, col)
                }
                '=' => {
                    self.advance();
                    Token::new(TokenKind::StarAssign, line, col)
                }
                _ => Token::new(TokenKind::Star, line, col),
            }
        } else {
            Token::new(TokenKind::Star, line, col)
        }
    }

    fn lex_percent(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos < self.source.len() && self.current_char() == '=' {
            self.advance();
            Token::new(TokenKind::PercentAssign, line, col)
        } else {
            Token::new(TokenKind::Percent, line, col)
        }
    }

    fn lex_amp(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos < self.source.len() {
            match self.current_char() {
                '&' => {
                    self.advance();
                    Token::new(TokenKind::AmpAmp, line, col)
                }
                '=' => {
                    self.advance();
                    Token::new(TokenKind::AmpAssign, line, col)
                }
                _ => Token::new(TokenKind::Amp, line, col),
            }
        } else {
            Token::new(TokenKind::Amp, line, col)
        }
    }

    fn lex_pipe(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos < self.source.len() {
            match self.current_char() {
                '|' => {
                    self.advance();
                    Token::new(TokenKind::PipePipe, line, col)
                }
                '=' => {
                    self.advance();
                    Token::new(TokenKind::PipeAssign, line, col)
                }
                _ => Token::new(TokenKind::Pipe, line, col),
            }
        } else {
            Token::new(TokenKind::Pipe, line, col)
        }
    }

    fn lex_eq(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos >= self.source.len() {
            return Token::new(TokenKind::Assign, line, col);
        }
        match self.current_char() {
            '=' => {
                self.advance();
                if self.pos < self.source.len() && self.current_char() == '=' {
                    self.advance();
                    Token::new(TokenKind::EqEqEq, line, col)
                } else {
                    Token::new(TokenKind::EqEq, line, col)
                }
            }
            '>' => {
                self.advance();
                Token::new(TokenKind::Arrow, line, col)
            }
            _ => Token::new(TokenKind::Assign, line, col),
        }
    }

    fn lex_bang(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos >= self.source.len() {
            return Token::new(TokenKind::Bang, line, col);
        }
        match self.current_char() {
            '=' => {
                self.advance();
                if self.pos < self.source.len() && self.current_char() == '=' {
                    self.advance();
                    Token::new(TokenKind::BangEqEq, line, col)
                } else {
                    Token::new(TokenKind::BangEq, line, col)
                }
            }
            _ => Token::new(TokenKind::Bang, line, col),
        }
    }

    fn lex_lt(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos >= self.source.len() {
            return Token::new(TokenKind::Lt, line, col);
        }
        match self.current_char() {
            '=' => {
                self.advance();
                Token::new(TokenKind::LtEq, line, col)
            }
            '<' => {
                self.advance();
                if self.pos < self.source.len() && self.current_char() == '=' {
                    self.advance();
                    Token::new(TokenKind::LtLtAssign, line, col)
                } else {
                    Token::new(TokenKind::LtLt, line, col)
                }
            }
            _ => Token::new(TokenKind::Lt, line, col),
        }
    }

    fn lex_gt(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos >= self.source.len() {
            return Token::new(TokenKind::Gt, line, col);
        }
        match self.current_char() {
            '=' => {
                self.advance();
                Token::new(TokenKind::GtEq, line, col)
            }
            '>' => {
                self.advance();
                if self.pos < self.source.len() && self.current_char() == '>' {
                    self.advance();
                    if self.pos < self.source.len() && self.current_char() == '=' {
                        self.advance();
                        Token::new(TokenKind::GtGtGtAssign, line, col)
                    } else {
                        Token::new(TokenKind::GtGtGt, line, col)
                    }
                } else if self.pos < self.source.len() && self.current_char() == '=' {
                    self.advance();
                    Token::new(TokenKind::GtGtAssign, line, col)
                } else {
                    Token::new(TokenKind::GtGt, line, col)
                }
            }
            _ => Token::new(TokenKind::Gt, line, col),
        }
    }

    fn lex_dot(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos < self.source.len() && self.current_char().is_ascii_digit() {
            // Number starting with '.'  — fall through to number lexer
            // by rewinding; we'll re-lex the dot as part of the number.
            self.pos -= 1;
            self.col -= 1;
            self.lex_number(line, col)
        } else {
            Token::new(TokenKind::Dot, line, col)
        }
    }

    fn lex_slash(&mut self, line: u32, col: u32) -> Token {
        self.advance();
        if self.pos < self.source.len() && self.current_char() == '=' {
            self.advance();
            Token::new(TokenKind::SlashAssign, line, col)
        } else {
            Token::new(TokenKind::Slash, line, col)
        }
    }

    fn lex_string(&mut self, quote: char, line: u32, col: u32) -> Token {
        self.advance(); // opening quote
        let mut buf = String::new();
        loop {
            if self.pos >= self.source.len() {
                return Token::new(TokenKind::UnterminatedString, line, col);
            }
            let ch = self.advance();
            if ch == quote {
                break;
            }
            if ch == '\\' {
                if self.pos >= self.source.len() {
                    return Token::new(TokenKind::UnterminatedString, line, col);
                }
                let esc = self.advance();
                match esc {
                    'n' => buf.push('\n'),
                    'r' => buf.push('\r'),
                    't' => buf.push('\t'),
                    '\\' => buf.push('\\'),
                    '\'' => buf.push('\''),
                    '"' => buf.push('"'),
                    '0' => buf.push('\0'),
                    _ => {
                        buf.push('\\');
                        buf.push(esc);
                    }
                }
            } else {
                buf.push(ch);
            }
        }
        Token::new(TokenKind::String(buf), line, col)
    }

    fn lex_template(&mut self, line: u32, col: u32) -> Token {
        self.advance(); // opening backtick
        let mut buf = String::new();
        loop {
            if self.pos >= self.source.len() {
                return Token::new(TokenKind::UnterminatedString, line, col);
            }
            let ch = self.advance();
            if ch == '`' {
                break;
            }
            if ch == '\\' {
                if self.pos >= self.source.len() {
                    return Token::new(TokenKind::UnterminatedString, line, col);
                }
                let esc = self.advance();
                buf.push(esc);
            } else {
                buf.push(ch);
            }
        }
        // For now, treat template literals as plain strings.
        // Template expression support (`${}`) is a follow-up.
        Token::new(TokenKind::String(buf), line, col)
    }

    fn lex_identifier(&mut self, line: u32, col: u32) -> Token {
        let start = self.pos;
        while self.pos < self.source.len() {
            let ch = self.current_char();
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                self.advance();
            } else {
                break;
            }
        }
        let word = &self.source[start..self.pos];
        let kind = match word {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "undefined" => TokenKind::Undefined,
            "this" => TokenKind::This,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "switch" => TokenKind::Switch,
            "case" => TokenKind::Case,
            "default" => TokenKind::Default,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            "throw" => TokenKind::Throw,
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            "finally" => TokenKind::Finally,
            "function" => TokenKind::Function,
            "class" => TokenKind::Class,
            "extends" => TokenKind::Extends,
            "new" => TokenKind::New,
            "delete" => TokenKind::Delete,
            "typeof" => TokenKind::Typeof,
            "void" => TokenKind::Void,
            "in" => TokenKind::In,
            "instanceof" => TokenKind::Instanceof,
            "var" => TokenKind::Var,
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "yield" => TokenKind::Yield,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "from" => TokenKind::From,
            "of" => TokenKind::Of,
            "super" => TokenKind::Super,
            "debugger" => TokenKind::Debugger,
            "with" => TokenKind::With,
            "static" => TokenKind::Static,
            "get" => TokenKind::Get,
            "set" => TokenKind::Set,
            _ => TokenKind::Identifier(word.to_string()),
        };
        Token::new(kind, line, col)
    }

    fn lex_number(&mut self, line: u32, col: u32) -> Token {
        let start = self.pos;

        // Handle hex, octal, binary
        if self.current_char() == '0' && self.pos + 1 < self.source.len() {
            let next = self.peek_char();
            if next == 'x' || next == 'X' {
                self.advance(); // 0
                self.advance(); // x
                while self.pos < self.source.len() && self.current_char().is_ascii_hexdigit() {
                    self.advance();
                }
                let text = &self.source[start..self.pos];
                return match u64::from_str_radix(&text[2..], 16) {
                    Ok(n) => Token::new(TokenKind::Number(n as f64), line, col),
                    Err(_) => Token::new(TokenKind::Unknown('0'), line, col),
                };
            }
        }

        while self.pos < self.source.len() && self.current_char().is_ascii_digit() {
            self.advance();
        }

        if self.pos < self.source.len() && self.current_char() == '.' {
            self.advance();
            while self.pos < self.source.len() && self.current_char().is_ascii_digit() {
                self.advance();
            }
        }

        if self.pos < self.source.len()
            && (self.current_char() == 'e' || self.current_char() == 'E')
        {
            self.advance();
            if self.pos < self.source.len()
                && (self.current_char() == '+' || self.current_char() == '-')
            {
                self.advance();
            }
            while self.pos < self.source.len() && self.current_char().is_ascii_digit() {
                self.advance();
            }
        }

        let text = &self.source[start..self.pos];
        match text.parse::<f64>() {
            Ok(n) => Token::new(TokenKind::Number(n), line, col),
            Err(_) => Token::new(TokenKind::Unknown('0'), line, col),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenKind;

    #[test]
    fn test_empty_source() {
        let tokens = lex("");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn test_single_number() {
        let tokens = lex("42");
        assert_eq!(tokens[0].kind, TokenKind::Number(42.0));
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }

    #[test]
    fn test_string_literal() {
        let tokens = lex(r#""hello""#);
        assert_eq!(tokens[0].kind, TokenKind::String("hello".to_string()));
    }

    #[test]
    fn test_operators() {
        let tokens = lex("a + b * c");
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(kinds.contains(&&TokenKind::Plus));
        assert!(kinds.contains(&&TokenKind::Star));
    }

    #[test]
    fn test_keywords() {
        let tokens = lex("function foo() {}");
        assert_eq!(tokens[0].kind, TokenKind::Function);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("foo".to_string()));
    }

    #[test]
    fn test_var_let_const() {
        let tokens = lex("var x = 1; let y = 2; const z = 3;");
        assert_eq!(tokens[0].kind, TokenKind::Var);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Assign);
        assert_eq!(tokens[3].kind, TokenKind::Number(1.0));
        assert_eq!(tokens[4].kind, TokenKind::Semicolon);
        assert_eq!(tokens[5].kind, TokenKind::Let);
        assert_eq!(tokens[6].kind, TokenKind::Identifier("y".to_string()));
    }

    #[test]
    fn test_line_comment_skipped() {
        let tokens = lex("42 // comment\n43");
        assert_eq!(tokens[0].kind, TokenKind::Number(42.0));
        assert_eq!(tokens[1].kind, TokenKind::Number(43.0));
    }

    #[test]
    fn test_strict_equality() {
        let tokens = lex("a === b");
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(kinds.contains(&&TokenKind::EqEqEq));
    }

    #[test]
    fn test_arrow_function() {
        let tokens = lex("x => x");
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(kinds.contains(&&TokenKind::Arrow));
    }

    #[test]
    fn test_float_number() {
        let tokens = lex("3.5");
        assert!(matches!(tokens[0].kind, TokenKind::Number(n) if (n - 3.5).abs() < 1e-9));
    }
}
