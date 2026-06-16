//! CSS Syntax Level 3 tokeniser.
//!
//! The tokeniser is the only input to the CSS parser. It
//! walks the source string and produces a flat stream of
//! [`Token`]s; the parser then groups them into rules,
//! at-rules, declarations, and values.
//!
//! The surface implemented is the CSS Syntax 3 tokeniser
//! as specified in <https://www.w3.org/TR/css-syntax-3/>
//! restricted to the surface the M4.4.1 minimum-viable
//! parser actually consumes:
//!
//! - Identifiers, numbers, percentages, dimensions
//! - Strings (single and double quoted, with escape
//!   handling)
//! - Hash tokens (`#` followed by an ident, treated as an
//!   ID selector marker)
//! - At-keywords (`@media`, `@import`, …)
//! - Punctuation: `{`, `}`, `(`, `)`, `[`, `]`, `,`, `;`,
//!   `:`, `~`, `>`, `+`, `=`, `*`, `.`
//! - Whitespace (preserved as `Whitespace` tokens so the
//!   parser can detect descendant combinators)
//!
//! The tokeniser is fully from-spec: it does not pull in
//! `cssparser` from Servo, the `cssparser` workspace
//! dependency, or any other third-party CSS parsing
//! library. It uses the [`crate::cursor::Cursor`] helper
//! from the HTML side for byte walking and `Position`
//! tracking, the same surface the HTML5 tokeniser uses.

// Reuse the HTML tokeniser's cursor for the
// whitespace-sensitive byte walk. It is `pub(crate)` so we
// can share it across modules.
use crate::cursor::Cursor;

/// The set of CSS tokens the parser operates on. Each
/// variant is a single token of source — the parser groups
/// tokens into higher-level structures.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Run of CSS whitespace (spaces, tabs, newlines). The
    /// parser uses these to detect the descendant
    /// combinator.
    Whitespace,
    /// `ident` — a name like `div`, `color`, `bold`.
    Ident(String),
    /// `1234` or `1.5` — a unitless number. Floats match
    /// the spec's `parse-a-number` algorithm.
    Number(f32),
    /// `12.5%` — a percentage; the `%` is dropped.
    Percentage(f32),
    /// `12.5px` — a dimension with a unit. The unit is
    /// preserved as a string (we only consume `px` /
    /// `em` / `rem` in the value parser, but the
    /// tokeniser is permissive about units it doesn't
    /// understand).
    Dimension(f32, String),
    /// `"…"` or `'…'` — a string literal. The body is
    /// the raw string (escape sequences already resolved).
    String(String),
    /// `#…` — a hash token. The body is the bytes after
    /// `#`. The parser uses the first-char-is-ident rule
    /// to decide between ID selector and colour
    /// shorthand; for our purposes we just record the
    /// body.
    Hash(String),
    /// `@keyword` — an at-rule keyword like `@media` or
    /// `@import`. The body is the keyword without `@`.
    AtKeyword(String),
    /// `(` — open paren.
    LParen,
    /// `)` — close paren.
    ParenthesisClose,
    /// `[` — open bracket.
    LBracket,
    /// `]` — close bracket.
    RBracket,
    /// `{` — open brace.
    LBrace,
    /// `}` — close brace.
    RBrace,
    /// `,` — comma.
    Comma,
    /// `;` — semicolon.
    Semicolon,
    /// `:` — colon.
    Colon,
    /// `~=` — whitespace-separated word match.
    Include,
    /// `|=` — exact or dash-prefixed match.
    DashMatch,
    /// `^=` — prefix match.
    PrefixMatch,
    /// `$=` — suffix match.
    SuffixMatch,
    /// `*=` — substring match.
    SubstringMatch,
    /// A single-character delimiter. Used for `>`, `+`,
    /// `~`, `*`, `.`, and any other non-recognised
    /// punctuation.
    Delim(char),
    /// `<!--` — CDO (comment-open) at the top of a
    /// stylesheet. CSS allows it for HTML-compatibility
    /// parse hacks; we drop it.
    Cdo,
    /// `-->` — CDC (comment-close). Same.
    Cdc,
    /// End of input.
    Eof,
}

impl std::fmt::Display for Token {
    /// Render the token back to CSS source for diagnostic
    /// purposes. The result is not guaranteed to be a
    /// valid CSS round-trip.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Whitespace => f.write_str(" "),
            Token::Ident(s) => f.write_str(s),
            Token::Number(n) => write!(f, "{}", n),
            Token::Percentage(n) => write!(f, "{}%", n),
            Token::Dimension(n, u) => write!(f, "{}{}", n, u),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Hash(s) => write!(f, "#{}", s),
            Token::AtKeyword(s) => write!(f, "@{}", s),
            Token::LParen => f.write_str("("),
            Token::ParenthesisClose => f.write_str(")"),
            Token::LBracket => f.write_str("["),
            Token::RBracket => f.write_str("]"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),
            Token::Comma => f.write_str(","),
            Token::Semicolon => f.write_str(";"),
            Token::Colon => f.write_str(":"),
            Token::Include => f.write_str("~="),
            Token::DashMatch => f.write_str("|="),
            Token::PrefixMatch => f.write_str("^="),
            Token::SuffixMatch => f.write_str("$="),
            Token::SubstringMatch => f.write_str("*="),
            Token::Delim(c) => {
                let mut buf = [0u8; 4];
                let s = c.encode_utf8(&mut buf);
                f.write_str(s)
            }
            Token::Cdo => f.write_str("<!--"),
            Token::Cdc => f.write_str("-->"),
            Token::Eof => Ok(()),
        }
    }
}

/// Tokenise a CSS source string into a flat token stream.
///
/// Whitespace runs become a single `Token::Whitespace`; the
/// parser is responsible for interpreting runs as the
/// descendant combinator where appropriate. Comments are
/// skipped silently (they have no semantic effect on
/// style).
pub(crate) fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut cursor = Cursor::new(source);
    let mut out: Vec<Token> = Vec::new();
    loop {
        let tok = next_token(&mut cursor)?;
        out.push(tok.clone());
        if matches!(tok, Token::Eof) {
            return Ok(out);
        }
    }
}

/// Read the next token from `cursor`. Returns
/// `Token::Eof` at the end of input. Whitespace runs are
/// collapsed into a single `Token::Whitespace`.
fn next_token(cursor: &mut Cursor<'_>) -> Result<Token, String> {
    if cursor.eof() {
        return Ok(Token::Eof);
    }
    let b = cursor.peek_byte().expect("non-eof cursor has byte");
    match b {
        b' ' | b'\t' | b'\n' | b'\x0c' | b'\r' => {
            // Whitespace — collapse runs.
            while let Some(b) = cursor.peek_byte() {
                if !matches!(b, b' ' | b'\t' | b'\n' | b'\x0c' | b'\r') {
                    break;
                }
                cursor.advance();
            }
            Ok(Token::Whitespace)
        }
        b'"' | b'\'' => {
            cursor.advance();
            read_string(cursor, b)
        }
        b'#' => {
            cursor.advance();
            let body = read_name(cursor);
            Ok(Token::Hash(body))
        }
        b'@' => {
            cursor.advance();
            let name = read_ident(cursor);
            Ok(Token::AtKeyword(name))
        }
        b'{' => {
            cursor.advance();
            Ok(Token::LBrace)
        }
        b'}' => {
            cursor.advance();
            Ok(Token::RBrace)
        }
        b'(' => {
            cursor.advance();
            Ok(Token::LParen)
        }
        b')' => {
            cursor.advance();
            Ok(Token::ParenthesisClose)
        }
        b'[' => {
            cursor.advance();
            Ok(Token::LBracket)
        }
        b']' => {
            cursor.advance();
            Ok(Token::RBracket)
        }
        b',' => {
            cursor.advance();
            Ok(Token::Comma)
        }
        b';' => {
            cursor.advance();
            Ok(Token::Semicolon)
        }
        b':' => {
            cursor.advance();
            Ok(Token::Colon)
        }
        b'=' => {
            cursor.advance();
            Ok(Token::Delim('='))
        }
        b'.' => {
            // `.5` is a number; `.foo` is a Delim('.') then
            // an Ident. The CSS spec handles this with a
            // 3-token lookahead in the tokeniser.
            cursor.advance();
            if let Some(b) = cursor.peek_byte() {
                if b.is_ascii_digit() {
                    return read_number_or_dimension(cursor, '0', true);
                }
            }
            Ok(Token::Delim('.'))
        }
        b'+' | b'-' | b'*' | b'~' | b'>' => {
            // These are all 1-char delimiters. The
            // multi-char matchers (`~=`, `|=`, `^=`, `*=`)
            // are handled as 2-char lookups.
            let c = b as char;
            cursor.advance();
            if b == b'~' && cursor.peek_byte() == Some(b'=') {
                cursor.advance();
                return Ok(Token::Include);
            }
            if b == b'*' && cursor.peek_byte() == Some(b'=') {
                cursor.advance();
                return Ok(Token::SubstringMatch);
            }
            if b == b'-' {
                if let Some(b'-') = cursor.peek_byte() {
                    cursor.advance();
                    // `-->` ? Or `--ident` (CSS custom prop)
                    if cursor.peek_byte() == Some(b'>') {
                        cursor.advance();
                        return Ok(Token::Cdc);
                    }
                    // CSS custom property: consume the rest
                    // of the ident and return a normal
                    // `Ident`. The parser doesn't currently
                    // distinguish them; that's a M5+ item.
                    return Ok(Token::Ident(read_name(cursor)));
                }
                // `-5` is a number; the leading `-` is the
                // sign.
                if let Some(b) = cursor.peek_byte() {
                    if b.is_ascii_digit() {
                        return read_number_or_dimension(cursor, '-', false);
                    }
                }
                return Ok(Token::Delim('-'));
            }
            Ok(Token::Delim(c))
        }
        b'^' => {
            cursor.advance();
            if cursor.peek_byte() == Some(b'=') {
                cursor.advance();
                return Ok(Token::PrefixMatch);
            }
            Ok(Token::Delim('^'))
        }
        b'$' => {
            cursor.advance();
            if cursor.peek_byte() == Some(b'=') {
                cursor.advance();
                return Ok(Token::SuffixMatch);
            }
            Ok(Token::Delim('$'))
        }
        b'|' => {
            cursor.advance();
            if cursor.peek_byte() == Some(b'=') {
                cursor.advance();
                return Ok(Token::DashMatch);
            }
            Ok(Token::Delim('|'))
        }
        b'!' => {
            // `!important` — read the rest of the ident
            // starting at the next byte.
            cursor.advance();
            Ok(Token::Delim('!'))
        }
        b'/' => {
            // Comment.
            cursor.advance();
            if cursor.peek_byte() == Some(b'*') {
                cursor.advance();
                skip_block_comment(cursor);
                // After a comment, collapse any leading
                // whitespace into the comment skip — the
                // next emitted Whitespace would otherwise
                // come out as two tokens.
                while let Some(b) = cursor.peek_byte() {
                    if !matches!(b, b' ' | b'\t' | b'\n' | b'\x0c' | b'\r') {
                        break;
                    }
                    cursor.advance();
                }
                return next_token(cursor);
            }
            Ok(Token::Delim('/'))
        }
        b'<' => {
            // `<!--` is the CDO token.
            cursor.advance();
            if cursor.peek_byte() == Some(b'!')
                && cursor.peek_byte_at(1) == Some(b'-')
                && cursor.peek_byte_at(2) == Some(b'-')
            {
                cursor.advance_n(3);
                return Ok(Token::Cdo);
            }
            Ok(Token::Delim('<'))
        }
        b'0'..=b'9' => read_number_or_dimension(cursor, '0', false),
        _ if is_ident_start(b) => {
            let name = read_ident(cursor);
            // `<ident>(` is a function token — but the
            // parser just wants the ident + a LParen, so we
            // emit the ident and let the next call produce
            // the LParen. Same for `url(...)` and friends.
            Ok(Token::Ident(name))
        }
        _ => {
            // Unknown byte — emit as a delim and continue.
            let c = b as char;
            cursor.advance();
            Ok(Token::Delim(c))
        }
    }
}

/// Read a CSS identifier from the cursor, leaving the
/// cursor positioned at the first non-ident byte.
fn read_ident(cursor: &mut Cursor<'_>) -> String {
    read_name(cursor)
}

/// Read a `name` token (which the CSS3 spec defines as a
/// sequence of ident characters). The leading
/// ident-start check is the caller's job.
fn read_name(cursor: &mut Cursor<'_>) -> String {
    let mut s = String::new();
    while let Some(b) = cursor.peek_byte() {
        if is_ident_char(b) {
            s.push(b as char);
            cursor.advance();
        } else if b == b'\\' {
            // CSS escape sequence: \<hex>{1,6} or \\<any>.
            // We only handle the simple forms the M4.4.1
            // minimum needs; anything else is a non-fatal
            // best-effort.
            cursor.advance();
            match read_escape(cursor) {
                Ok(c) => s.push(c),
                Err(_) => break,
            }
        } else {
            break;
        }
    }
    s
}

/// Read a number and optional unit, starting at the
/// current cursor. `sign` is the leading sign character
/// (`'+'` or `'-'`) or `0` if no sign was consumed.
/// `already_advanced_dot` is set when the caller has
/// consumed a leading `.` (so the number starts with
/// "0." for the `.5` form).
fn read_number_or_dimension(
    cursor: &mut Cursor<'_>,
    sign: char,
    already_advanced_dot: bool,
) -> Result<Token, String> {
    let mut num: String = String::new();
    if sign != '0' {
        num.push(sign);
    }
    if already_advanced_dot {
        // The caller consumed the `.`; the rest of the
        // number is digits. We prepend a `0` so `0.5`
        // parses correctly.
        num.push('0');
        num.push('.');
        while let Some(b) = cursor.peek_byte() {
            if b.is_ascii_digit() {
                num.push(b as char);
                cursor.advance();
            } else {
                break;
            }
        }
    } else {
        // Consume leading digits.
        while let Some(b) = cursor.peek_byte() {
            if b.is_ascii_digit() {
                num.push(b as char);
                cursor.advance();
            } else {
                break;
            }
        }
        // Optional `.` then more digits.
        if cursor.peek_byte() == Some(b'.') {
            num.push('.');
            cursor.advance();
            while let Some(b) = cursor.peek_byte() {
                if b.is_ascii_digit() {
                    num.push(b as char);
                    cursor.advance();
                } else {
                    break;
                }
            }
        }
        // Optional `e`/`E` then optional sign then digits.
        if let Some(b) = cursor.peek_byte() {
            if b == b'e' || b == b'E' {
                if let Some(b2) = cursor.peek_byte_at(1) {
                    if b2.is_ascii_digit() || b2 == b'+' || b2 == b'-' {
                        num.push(b as char);
                        cursor.advance();
                        if let Some(b) = cursor.peek_byte() {
                            if b == b'+' || b == b'-' {
                                num.push(b as char);
                                cursor.advance();
                            }
                        }
                        while let Some(b) = cursor.peek_byte() {
                            if b.is_ascii_digit() {
                                num.push(b as char);
                                cursor.advance();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    let value: f32 = num
        .parse()
        .map_err(|e| format!("invalid number '{}' in CSS: {}", num, e))?;

    // If the next char is a letter or `-`, this is a
    // dimension; if `%`, a percentage; otherwise a number.
    match cursor.peek_byte() {
        Some(b'%') => {
            cursor.advance();
            Ok(Token::Percentage(value))
        }
        Some(b) if is_ident_start(b) => {
            let unit = read_name(cursor);
            Ok(Token::Dimension(value, unit))
        }
        _ => Ok(Token::Number(value)),
    }
}

/// Read a quoted string (single or double). The leading
/// quote has already been seen; the cursor is positioned
/// just past it.
fn read_string(cursor: &mut Cursor<'_>, quote: u8) -> Result<Token, String> {
    let mut s = String::new();
    loop {
        match cursor.peek_byte() {
            None => {
                return Err("unterminated string in CSS".to_string());
            }
            Some(b) if b == quote => {
                cursor.advance();
                return Ok(Token::String(s));
            }
            Some(b'\\') => {
                cursor.advance();
                match read_escape(cursor) {
                    Ok(c) => s.push(c),
                    Err(_) => {
                        return Err("invalid escape in CSS string".to_string());
                    }
                }
            }
            Some(b'\n') | Some(b'\r') | Some(b'\x0c') => {
                // Per spec, an unescaped newline in a
                // string is a parse error; we treat it as
                // a string terminator for tolerability.
                return Err("newline in CSS string".to_string());
            }
            Some(_) => {
                let ch = cursor.current_char().expect("peek_byte returned Some");
                s.push(ch);
                // `advance` is UTF-8 aware — it consumes
                // the whole scalar value, not just one
                // byte.
                cursor.advance();
            }
        }
    }
}

/// Read a CSS escape sequence (`\X`, `\<hex>+<ws>?`,
/// `\<any>`). The leading backslash has already been
/// consumed. Returns the resolved character.
fn read_escape(cursor: &mut Cursor<'_>) -> Result<char, String> {
    match cursor.peek_byte() {
        None => Err("EOF in escape".to_string()),
        Some(b) if b.is_ascii_hexdigit() => {
            // Up to 6 hex digits, terminated by an optional
            // single whitespace.
            let mut hex = String::new();
            for _ in 0..6 {
                match cursor.peek_byte() {
                    Some(b) if b.is_ascii_hexdigit() => {
                        hex.push(b as char);
                        cursor.advance();
                    }
                    _ => break,
                }
            }
            // Optional single trailing whitespace.
            if matches!(
                cursor.peek_byte(),
                Some(b' ' | b'\t' | b'\n' | b'\r' | b'\x0c')
            ) {
                cursor.advance();
            }
            let code = u32::from_str_radix(&hex, 16)
                .map_err(|e| format!("invalid hex escape '\\{}': {}", hex, e))?;
            char::from_u32(code).ok_or_else(|| format!("invalid code point U+{:X}", code))
        }
        Some(_) => {
            let ch = cursor.current_char().expect("peek_byte returned Some");
            // `advance` is UTF-8 aware — it consumes
            // the whole scalar value, not just one
            // byte.
            cursor.advance();
            Ok(ch)
        }
    }
}

/// Skip a `/* ... */` comment. The opening `/*` has
/// already been consumed.
fn skip_block_comment(cursor: &mut Cursor<'_>) {
    while let Some(b) = cursor.peek_byte() {
        if b == b'*' && cursor.peek_byte_at(1) == Some(b'/') {
            cursor.advance_n(2);
            return;
        }
        cursor.advance();
    }
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b == b'\x80' || b > b'\x80'
        // Non-ASCII bytes are valid ident starts per the
        // CSS3 spec (any letter other than ASCII).
        || b > b'\x7f'
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b > b'\x7f'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(s: &str) -> Vec<Token> {
        tokenize(s).expect("tokenize")
    }

    #[test]
    fn empty_input() {
        assert_eq!(t(""), vec![Token::Eof]);
    }

    #[test]
    fn whitespace_collapse() {
        // Multiple spaces / tabs / newlines collapse to
        // a single Whitespace token.
        let ts = t("  \n\t  ");
        assert_eq!(ts, vec![Token::Whitespace, Token::Eof]);
    }

    #[test]
    fn ident_and_numbers() {
        let ts = t("div 12 0.5 1e3 .5 -3");
        // div / ws / 12 / ws / 0.5 / ws / 1e3 / ws / .5 /
        // ws / -3
        assert!(matches!(ts[0], Token::Ident(ref s) if s == "div"));
        assert!(matches!(ts[1], Token::Whitespace));
        assert!(matches!(ts[2], Token::Number(n) if (n - 12.0).abs() < 1e-6));
        assert!(matches!(ts[3], Token::Whitespace));
        assert!(matches!(ts[4], Token::Number(n) if (n - 0.5).abs() < 1e-6));
        assert!(matches!(ts[5], Token::Whitespace));
        assert!(matches!(ts[6], Token::Number(n) if (n - 1e3).abs() < 1e-6));
        assert!(matches!(ts[7], Token::Whitespace));
        assert!(matches!(ts[8], Token::Number(n) if (n - 0.5).abs() < 1e-6));
        assert!(matches!(ts[9], Token::Whitespace));
        assert!(matches!(ts[10], Token::Number(n) if (n + 3.0).abs() < 1e-6));
    }

    #[test]
    fn dimension_and_percentage() {
        let ts = t("12px 50%");
        assert!(
            matches!(ts[0], Token::Dimension(n, ref u) if (n - 12.0).abs() < 1e-6 && u == "px")
        );
        assert!(matches!(ts[1], Token::Whitespace));
        assert!(matches!(ts[2], Token::Percentage(n) if (n - 50.0).abs() < 1e-6));
    }

    #[test]
    fn strings() {
        let ts = t("\"hi\" 'lo'");
        assert!(matches!(ts[0], Token::String(ref s) if s == "hi"));
        assert!(matches!(ts[2], Token::String(ref s) if s == "lo"));
    }

    #[test]
    fn unterminated_string_is_error() {
        let r = tokenize("\"unterminated");
        assert!(r.is_err());
    }

    #[test]
    fn comment_skipped() {
        let ts = t("a /* comment */ b");
        // a / ws / b
        assert!(matches!(ts[0], Token::Ident(ref s) if s == "a"));
        assert!(matches!(ts[1], Token::Whitespace));
        assert!(matches!(ts[2], Token::Ident(ref s) if s == "b"));
        assert!(matches!(ts[3], Token::Eof));
    }

    #[test]
    fn attribute_matchers() {
        let ts = t("[a~=b] [c|=d] [e^=f] [g$=h] [i*=j]");
        // Each matcher: LBracket Ident Name Ident RBracket
        // Whitespace — the matcher is at index 2, 8, 14,
        // 20, 26.
        assert!(matches!(ts[2], Token::Include));
        assert!(matches!(ts[8], Token::DashMatch));
        assert!(matches!(ts[14], Token::PrefixMatch));
        assert!(matches!(ts[20], Token::SuffixMatch));
        assert!(matches!(ts[26], Token::SubstringMatch));
    }

    #[test]
    fn at_keyword() {
        let ts = t("@media screen");
        assert!(matches!(ts[0], Token::AtKeyword(ref s) if s == "media"));
    }

    #[test]
    fn cdo_cdc() {
        let ts = t("<!-- -->");
        assert!(matches!(ts[0], Token::Cdo));
        assert!(matches!(ts[1], Token::Whitespace));
        assert!(matches!(ts[2], Token::Cdc));
    }

    #[test]
    fn important_bang() {
        let ts = t("!important");
        assert!(matches!(ts[0], Token::Delim('!')));
    }

    #[test]
    fn hash_with_ident() {
        let ts = t("#main");
        assert!(matches!(ts[0], Token::Hash(ref s) if s == "main"));
    }
}
