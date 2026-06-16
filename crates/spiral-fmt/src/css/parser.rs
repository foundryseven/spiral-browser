//! CSS parser — turns a token stream into a [`Stylesheet`].
//!
//! The parser is deliberately small. It groups tokens into
//! three top-level structures:
//!
//! - **Qualified rules** — a selector list followed by a
//!   `{ … }` block of declarations.
//! - **At-rules** — an `@keyword` (with optional prelude)
//!   followed by either a `{ … }` block (for `@media`,
//!   `@supports`, `@document`, `@layer`, `@keyframes`,
//!   `@font-face`, etc.) or a `;` (for `@import`,
//!   `@charset`, `@namespace`).
//! - **Declarations** — a `name: value;` sequence inside
//!   a qualified rule or at-rule block.
//!
//! What this parser does NOT do (deferred to M5+):
//!
//! - The CSS cascade (priority / origin ordering).
//! - The shorthand expansion of `margin`, `padding`, etc.
//!   (we record the raw value list).
//! - `var(--x)` and `calc(...)` resolution.
//! - Selector matching against a DOM (the matcher
//!   consumes the parsed `Stylesheet` and lives in
//!   `spiral-gyre`).
//! - Charset detection (we treat the input as UTF-8).

use super::selector::{parse_selector_list, SelectorList};
use super::tokenizer::{tokenize, Token};
use super::value::{named_color, parse_hex_color, Value};

/// A parsed CSS stylesheet.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stylesheet {
    /// The top-level rules in source order. The cascade
    /// is responsible for ordering these; the parser
    /// preserves source order verbatim.
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    /// All qualified rules (selector + declarations),
    /// in source order. At-rules are returned in
    /// [`Self::at_rules`].
    pub fn qualified_rules(&self) -> impl Iterator<Item = &QualifiedRule> {
        self.rules.iter().filter_map(|r| match r {
            Rule::Qualified(q) => Some(q),
            _ => None,
        })
    }

    /// All at-rules, in source order.
    pub fn at_rules(&self) -> impl Iterator<Item = &AtRule> {
        self.rules.iter().filter_map(|r| match r {
            Rule::At(a) => Some(a),
            _ => None,
        })
    }
}

/// A single top-level entry in a stylesheet.
#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
    /// `selector { decl; decl; }`.
    Qualified(QualifiedRule),
    /// `@name prelude [ { … } | ; ]`.
    At(AtRule),
}

/// A qualified rule.
#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedRule {
    /// The selector list. Each entry is a comma-separated
    /// alternative.
    pub selector: SelectorList,
    /// The declarations inside the block.
    pub declarations: Vec<Declaration>,
}

/// An at-rule.
#[derive(Debug, Clone, PartialEq)]
pub struct AtRule {
    /// The at-keyword without the leading `@`, lower-cased.
    /// Examples: `"media"`, `"import"`, `"font-face"`,
    /// `"keyframes"`, `"supports"`, `"layer"`.
    pub name: String,
    /// The raw prelude text (everything between the
    /// at-keyword and the `{` or `;`).
    pub prelude: Vec<Token>,
    /// The block contents, if the at-rule uses a `{ }`
    /// form. The parser does not interpret these — the
    /// caller is responsible for routing `@media` and
    /// `@supports` to the rules they contain, and
    /// `@keyframes` to its keyframe rules.
    pub block: Option<AtBlock>,
}

/// The body of an at-rule, stored as a generic list of
/// nested rules. The parser does not specialise per
/// at-rule; the runtime layer (cascade / animation
/// engine) interprets the contents.
#[derive(Debug, Clone, PartialEq)]
pub struct AtBlock {
    /// The rules nested inside the at-rule block, in
    /// source order.
    pub rules: Vec<Rule>,
}

/// A single `name: value [!important]?;` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    /// The property name. Lower-cased at parse time.
    pub name: String,
    /// The value. The parser stores a `Value::List` if
    /// the value is more than one component, otherwise
    /// the single component.
    pub value: Value,
    /// Whether the declaration was marked `!important`.
    pub important: bool,
}

// ------------------------------------------------------------
// Public entry point
// ------------------------------------------------------------

/// Parse a CSS stylesheet from a source string.
pub fn parse(source: &str) -> Result<Stylesheet, String> {
    let tokens = tokenize(source)?;
    let mut parser = Parser { tokens, pos: 0 };
    let rules = parser.parse_stylesheet()?;
    // Drain any trailing whitespace / EOF noise.
    parser.skip_whitespace();
    if !matches!(parser.peek(), Some(Token::Eof) | None) {
        return Err(format!(
            "unexpected token at end of stylesheet: {:?}",
            parser.peek()
        ));
    }
    Ok(Stylesheet { rules })
}

// ------------------------------------------------------------
// Recursive-descent parser
// ------------------------------------------------------------

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn skip_whitespace(&mut self) {
        while let Some(t) = self.peek() {
            if matches!(t, Token::Whitespace) {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Parse a top-level stylesheet body. We loop until
    /// EOF or an unbalanced `}`.
    fn parse_stylesheet(&mut self) -> Result<Vec<Rule>, String> {
        let mut rules = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                None | Some(Token::Eof) => return Ok(rules),
                Some(Token::RBrace) => return Ok(rules),
                Some(Token::AtKeyword(_)) => {
                    rules.push(self.parse_at_rule()?);
                }
                Some(Token::Semicolon) => {
                    // Stray semicolon — skip silently.
                    self.advance();
                }
                _ => {
                    rules.push(Rule::Qualified(self.parse_qualified_rule()?));
                }
            }
        }
    }

    fn parse_qualified_rule(&mut self) -> Result<QualifiedRule, String> {
        // Selector list: everything up to the first `{`.
        let mut selector_tokens: Vec<Token> = Vec::new();
        loop {
            match self.peek() {
                None | Some(Token::Eof) => {
                    return Err("unterminated qualified rule".to_string());
                }
                Some(Token::LBrace) => break,
                Some(t) => {
                    selector_tokens.push(t.clone());
                    self.advance();
                }
            }
        }
        let selector = parse_selector_list(&selector_tokens)?;
        self.advance(); // consume {
        let declarations = self.parse_declaration_list()?;
        match self.peek() {
            Some(Token::RBrace) => {
                self.advance();
            }
            _ => {
                return Err("expected '}' to close qualified rule".to_string());
            }
        }
        Ok(QualifiedRule {
            selector,
            declarations,
        })
    }

    fn parse_at_rule(&mut self) -> Result<Rule, String> {
        let name = if let Some(Token::AtKeyword(n)) = self.peek() {
            let n = n.to_ascii_lowercase();
            self.advance();
            n
        } else {
            return Err("expected @-keyword".to_string());
        };
        let mut prelude: Vec<Token> = Vec::new();
        let block = loop {
            match self.peek() {
                None | Some(Token::Eof) => {
                    return Err(format!("unterminated @{}", name));
                }
                Some(Token::Semicolon) => {
                    self.advance();
                    break None;
                }
                Some(Token::LBrace) => {
                    self.advance();
                    let rules = self.parse_stylesheet()?;
                    match self.peek() {
                        Some(Token::RBrace) => {
                            self.advance();
                        }
                        _ => {
                            return Err(format!("expected '}}' to close @{}", name));
                        }
                    }
                    break Some(AtBlock { rules });
                }
                Some(t) => {
                    prelude.push(t.clone());
                    self.advance();
                }
            }
        };
        Ok(Rule::At(AtRule {
            name,
            prelude,
            block,
        }))
    }

    fn parse_declaration_list(&mut self) -> Result<Vec<Declaration>, String> {
        let mut out = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                None | Some(Token::Eof) | Some(Token::RBrace) => return Ok(out),
                Some(Token::Semicolon) => {
                    self.advance();
                }
                Some(Token::AtKeyword(_)) => {
                    // Nested at-rule inside a block: parse
                    // it as a Rule so the block is fully
                    // round-tripped, then keep going.
                    // For the M4.4.1 minimum we treat this
                    // as a hard error — at-rules inside
                    // block rules are rare and the test
                    // corpus doesn't exercise them.
                    return Err("nested @-rule inside a block rule is not supported".to_string());
                }
                _ => {
                    let decl = self.parse_declaration()?;
                    out.push(decl);
                }
            }
        }
    }

    fn parse_declaration(&mut self) -> Result<Declaration, String> {
        // `name: value [!important]?;`
        let name = match self.peek() {
            Some(Token::Ident(n)) => n.to_ascii_lowercase(),
            _ => return Err("expected property name".to_string()),
        };
        self.advance();
        match self.peek() {
            Some(Token::Colon) => {
                self.advance();
            }
            _ => {
                return Err(format!("expected ':' after property name '{}'", name));
            }
        }
        let value_tokens = self.read_value_tokens()?;
        let value = parse_value(&value_tokens)?;
        // Check for `!important`.
        let mut important = false;
        if let Some(Token::Delim('!')) = self.peek() {
            self.advance();
            if let Some(Token::Ident(s)) = self.peek() {
                if s.eq_ignore_ascii_case("important") {
                    self.advance();
                    important = true;
                }
            }
        }
        Ok(Declaration {
            name,
            value,
            important,
        })
    }

    /// Read the token sequence that makes up a value:
    /// everything up to the next `;`, `}`, `!`, or end
    /// of input. The `!` is the marker for `!important`,
    /// which we strip from the value to be parsed
    /// separately.
    fn read_value_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut out = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                None | Some(Token::Eof) => return Ok(out),
                Some(Token::Semicolon) | Some(Token::RBrace) => return Ok(out),
                Some(Token::Delim('!')) => return Ok(out),
                Some(t) => {
                    out.push(t.clone());
                    self.advance();
                }
            }
        }
    }
}

// ------------------------------------------------------------
// Value parser
// ------------------------------------------------------------

/// Parse a value from a list of tokens. The token list
/// is the body of a declaration. Single tokens become
/// single values; multiple tokens become a `Value::List`.
/// `name(...)` patterns become a `Value::Function`.
pub(crate) fn parse_value(tokens: &[Token]) -> Result<Value, String> {
    if tokens.is_empty() {
        return Ok(Value::Keyword("initial".to_string()));
    }
    // Group `Ident LParen ... ParenthesisClose` into a
    // single Value::Function so callers don't have to
    // glue parts together.
    let mut values: Vec<Value> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        // Skip whitespace tokens — we treat them as
        // separators between value components.
        if matches!(tokens[i], Token::Whitespace) {
            i += 1;
            continue;
        }
        if let Token::Ident(name) = &tokens[i] {
            if let Some(Token::LParen) = tokens.get(i + 1) {
                // Find the matching close paren.
                let mut depth = 1;
                let mut j = i + 2;
                while j < tokens.len() && depth > 0 {
                    match &tokens[j] {
                        Token::LParen => depth += 1,
                        Token::ParenthesisClose => depth -= 1,
                        _ => {}
                    }
                    if depth == 0 {
                        break;
                    }
                    j += 1;
                }
                if depth == 0 {
                    let args = parse_value(&tokens[i + 2..j])?;
                    values.push(Value::Function(
                        name.to_ascii_lowercase(),
                        match args {
                            Value::List(v) => v,
                            other => vec![other],
                        },
                    ));
                    i = j + 1;
                    continue;
                }
            }
        }
        values.push(parse_single_value(&tokens[i])?);
        i += 1;
    }
    if values.len() == 1 {
        Ok(values.into_iter().next().expect("len 1"))
    } else {
        Ok(Value::List(values))
    }
}

fn parse_single_value(tok: &Token) -> Result<Value, String> {
    match tok {
        Token::Number(n) => Ok(Value::Number(*n)),
        Token::Percentage(n) => Ok(Value::Percentage(*n)),
        Token::Dimension(n, u) => {
            let unit_lower = u.to_ascii_lowercase();
            match unit_lower.as_str() {
                "px" | "" => Ok(Value::Length(*n)),
                "em" | "rem" | "ex" | "ch" => {
                    // Treat as Length for the M4.4.1
                    // minimum; the layout engine only
                    // understands pixels but we preserve
                    // the numeric value as-is.
                    Ok(Value::Length(*n))
                }
                other => {
                    // Unknown unit — record the value
                    // with the unit string in a Keyword
                    // so it round-trips through debug
                    // output.
                    Ok(Value::Keyword(format!("{}{}", n, other)))
                }
            }
        }
        Token::String(s) => Ok(Value::String(s.clone())),
        Token::Ident(s) => {
            // Colour or keyword.
            if let Some(c) = named_color(s) {
                return Ok(Value::Color(c));
            }
            Ok(Value::Keyword(s.to_ascii_lowercase()))
        }
        Token::Hash(body) => {
            if let Some(c) = parse_hex_color(&format!("#{}", body)) {
                Ok(Value::Color(c))
            } else {
                // Non-hex `#` tokens are ID selectors,
                // not values — we shouldn't see them in
                // a value position. Treat as a parse
                // error rather than silently emitting
                // junk.
                Err(format!("invalid colour hash '#{}'", body))
            }
        }
        Token::AtKeyword(s) => Ok(Value::Keyword(format!("@{}", s))),
        _ => {
            // Unexpected token in a value position
            // (delim, paren, bracket, etc.). Drop it
            // and let the rest of the value parse.
            Err(format!("unexpected token in value position: {:?}", tok))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::value::named_color;

    fn parse_one(s: &str) -> Declaration {
        let toks = tokenize(&format!("x: {};", s)).expect("tokenize");
        let mut p = Parser {
            tokens: toks,
            pos: 0,
        };
        p.parse_declaration().expect("declaration")
    }

    #[test]
    fn empty_stylesheet() {
        let s = parse("").expect("parse");
        assert_eq!(s.rules.len(), 0);
    }

    #[test]
    fn whitespace_only() {
        let s = parse("   \n\n  ").expect("parse");
        assert_eq!(s.rules.len(), 0);
    }

    #[test]
    fn single_qualified_rule() {
        let s = parse("div { color: red; }").expect("parse");
        assert_eq!(s.rules.len(), 1);
        let q = match &s.rules[0] {
            Rule::Qualified(q) => q,
            _ => panic!("expected qualified rule"),
        };
        assert_eq!(q.declarations.len(), 1);
        assert_eq!(q.declarations[0].name, "color");
        match &q.declarations[0].value {
            Value::Color(c) => {
                assert_eq!(*c, named_color("red").unwrap());
            }
            _ => panic!("expected color, got {:?}", q.declarations[0].value),
        }
    }

    #[test]
    fn declaration_with_length() {
        let d = parse_one("12px");
        assert_eq!(d.name, "x");
        assert!(matches!(d.value, Value::Length(n) if (n - 12.0).abs() < 1e-6));
    }

    #[test]
    fn declaration_with_percentage() {
        let d = parse_one("50%");
        assert!(matches!(d.value, Value::Percentage(n) if (n - 50.0).abs() < 1e-6));
    }

    #[test]
    fn declaration_important() {
        let d = parse_one("red !important");
        assert!(d.important);
        assert!(matches!(d.value, Value::Color(_)));
    }

    #[test]
    fn declaration_auto() {
        let d = parse_one("auto");
        assert!(matches!(d.value, Value::Keyword(ref s) if s == "auto"));
    }

    #[test]
    fn declaration_color_hash() {
        let d = parse_one("#ff0000");
        match d.value {
            Value::Color(c) => {
                assert_eq!(c.r, 255);
                assert_eq!(c.g, 0);
                assert_eq!(c.b, 0);
            }
            _ => panic!("expected color"),
        }
    }

    #[test]
    fn at_rule_with_block() {
        let s = parse("@media screen { div { color: red; } }").expect("parse");
        assert_eq!(s.rules.len(), 1);
        match &s.rules[0] {
            Rule::At(a) => {
                assert_eq!(a.name, "media");
                let block = a.block.as_ref().expect("block");
                assert_eq!(block.rules.len(), 1);
            }
            _ => panic!("expected at-rule"),
        }
    }

    #[test]
    fn at_rule_with_semicolon() {
        let s = parse("@import url(\"x.css\");").expect("parse");
        match &s.rules[0] {
            Rule::At(a) => {
                assert_eq!(a.name, "import");
                assert!(a.block.is_none());
            }
            _ => panic!("expected at-rule"),
        }
    }

    #[test]
    fn nested_at_rule_in_block_unsupported() {
        // We don't parse at-rules inside a block; this
        // should error. Documenting the limitation.
        let r = parse("div { @media screen { color: red; } }");
        assert!(r.is_err());
    }

    #[test]
    fn multiple_declarations() {
        let s = parse("p { color: red; font-size: 14px; display: block; }").expect("parse");
        let q = match &s.rules[0] {
            Rule::Qualified(q) => q,
            _ => panic!(),
        };
        assert_eq!(q.declarations.len(), 3);
        assert_eq!(q.declarations[0].name, "color");
        assert_eq!(q.declarations[1].name, "font-size");
        assert_eq!(q.declarations[2].name, "display");
    }

    #[test]
    fn selector_list_with_alternatives() {
        let s = parse("h1, h2, h3 { color: red; }").expect("parse");
        let q = match &s.rules[0] {
            Rule::Qualified(q) => q,
            _ => panic!(),
        };
        assert_eq!(q.selector.alternatives.len(), 3);
    }

    #[test]
    fn specific_stylesheet() {
        let s = parse(
            "/* comment */ .a > #b.c[d=e]:hover::before g { color: red; font-weight: bold; }",
        )
        .expect("parse");
        let q = match &s.rules[0] {
            Rule::Qualified(q) => q,
            _ => panic!(),
        };
        assert_eq!(q.declarations.len(), 2);
        assert_eq!(q.selector.specificity().ids, 1);
    }
}
