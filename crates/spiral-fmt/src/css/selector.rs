//! CSS selectors — types and parser.
//!
//! The selector syntax modelled here is the Selectors
//! Level 4 surface that the M4.4.1 minimum-viable parser
//! supports:
//!
//! - Type selector (`div`, `h1`, …)
//! - Universal selector (`*`)
//! - Class (`.foo`)
//! - ID (`#bar`)
//! - Attribute (`[attr]`, `[attr=val]`, `[attr~=val]`,
//!   `[attr|=val]`, `[attr^=val]`, `[attr$=val]`,
//!   `[attr*=val]`, plus the case-insensitive `i` flag)
//! - Pseudo-class (`:hover`, `:first-child`, …) — the
//!   parser accepts the colon and the identifier; the
//!   runtime matcher is a separate concern
//! - Pseudo-element (`::before`, `::after`, …) — same
//!   shape as pseudo-class
//! - Combinators: descendant (whitespace), child (`>`),
//!   adjacent sibling (`+`), general sibling (`~`)
//!
//! Specificity is computed for the whole selector list
//! (each comma-separated alternative) at parse time and
//! stored on the [`ComplexSelector`] so the cascade does
//! not have to recompute it.
//!
//! What this module does NOT do:
//!
//! - Pseudo-class / pseudo-element argument parsing
//!   beyond a balanced-paren read (so `:nth-child(2n+1)`
//!   keeps its argument as a string)
//! - Selector matching (the matcher consumes the parsed
//!   tree and a DOM node to decide whether the selector
//!   applies)
//! - `:not(...)`, `:is(...)`, `:where(...)` — listed in
//!   the audit as M5+ work

use super::specificity::Specificity;
use super::tokenizer::Token;

/// A single attribute selector, including its matcher
/// and optional case-insensitivity flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeSelector {
    /// The attribute name. Lower-cased at parse time.
    pub name: String,
    /// The matcher — `Present` for `[attr]`, the others
    /// for `[attr OP val]`.
    pub matcher: AttributeMatcher,
    /// The `i` / `s` flag on the attribute selector.
    /// Defaults to case-sensitive.
    pub case: AttributeCase,
}

impl AttributeSelector {
    /// Specificity contribution: 1 class slot.
    pub fn specificity_bump() -> Specificity {
        Specificity::new(0, 1, 0)
    }
}

/// The four matchers from Selectors Level 4 plus "present
/// only" for the bare `[attr]` form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeMatcher {
    /// `[attr]` — attribute is present, value not
    /// constrained.
    Present,
    /// `[attr=value]` — exact match.
    Exact(String),
    /// `[attr~=value]` — whitespace-separated word.
    Includes(String),
    /// `[attr|=value]` — exact or prefix followed by `-`.
    DashMatch(String),
    /// `[attr^=value]` — starts with.
    Prefix(String),
    /// `[attr$=value]` — ends with.
    Suffix(String),
    /// `[attr*=value]` — contains.
    Substring(String),
}

/// The case-sensitivity flag on an attribute selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttributeCase {
    /// `[attr=val]` — case-sensitive.
    #[default]
    Sensitive,
    /// `[attr=val i]` — case-insensitive ASCII.
    Insensitive,
    /// `[attr=val s]` — explicitly case-sensitive (M5+,
    /// the default case already does this).
    ExplicitSensitive,
}

/// A single compound selector — a sequence of simple
/// selectors with no combinators inside.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompoundSelector {
    /// Optional type or universal selector.
    pub type_selector: Option<TypeSelector>,
    /// ID selectors, in source order.
    pub ids: Vec<String>,
    /// Class selectors, in source order.
    pub classes: Vec<String>,
    /// Attribute selectors, in source order.
    pub attributes: Vec<AttributeSelector>,
    /// Pseudo-class selectors, in source order. Each
    /// entry is `(name, optional_argument)`.
    pub pseudo_classes: Vec<(String, Option<String>)>,
    /// Pseudo-element selectors, in source order. Each
    /// entry is `(name, optional_argument)`.
    pub pseudo_elements: Vec<(String, Option<String>)>,
}

impl CompoundSelector {
    /// Compute the specificity of this compound selector.
    pub fn specificity(&self) -> Specificity {
        let mut s = Specificity::default();
        if matches!(self.type_selector, Some(TypeSelector::Element(_))) {
            s.add_type();
        }
        // Universal selector contributes 0 per spec; we
        // silently ignore it here.
        s.ids = self.ids.len() as u32;
        s.classes = self.classes.len() as u32
            + self.attributes.len() as u32
            + self.pseudo_classes.len() as u32;
        // Pseudo-elements contribute to the type slot.
        s.types += self.pseudo_elements.len() as u32;
        s
    }
}

/// The leading type / universal selector on a compound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSelector {
    /// `*`.
    Universal,
    /// `tagname` — the tag name. Lower-cased at parse time.
    Element(String),
}

/// A combinator that joins two [`CompoundSelector`]s into a
/// larger sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combinator {
    /// ` ` (whitespace) — descendant.
    Descendant,
    /// `>` — child.
    Child,
    /// `+` — adjacent sibling.
    NextSibling,
    /// `~` — general sibling.
    SubsequentSibling,
}

impl Combinator {
    /// Specificity contribution: none, per spec.
    pub fn specificity_bump() -> Specificity {
        Specificity::default()
    }
}

/// One step in a complex selector — a compound plus the
/// combinator that joins it to the next step. The final
/// step has no combinator (`None`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexSelectorStep {
    /// The compound at this step.
    pub compound: CompoundSelector,
    /// The combinator joining this step to the next.
    /// `None` on the last step.
    pub combinator: Option<Combinator>,
}

/// A single alternative in a selector list. One
/// complex selector = a sequence of compounds joined by
/// combinators. Multiple alternatives are separated by
/// commas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexSelector {
    /// The steps in source order.
    pub steps: Vec<ComplexSelectorStep>,
    /// Cached specificity for the whole complex selector.
    pub specificity: Specificity,
}

impl ComplexSelector {
    /// Compute the specificity of this complex selector
    /// by summing each step's specificity.
    pub fn compute_specificity(steps: &[ComplexSelectorStep]) -> Specificity {
        let mut s = Specificity::default();
        for step in steps {
            s.ids += step.compound.specificity().ids;
            s.classes += step.compound.specificity().classes;
            s.types += step.compound.specificity().types;
        }
        s
    }
}

// ------------------------------------------------------------
// Parser
// ------------------------------------------------------------

/// Outcome of a selector parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectorList {
    /// The comma-separated alternatives.
    pub alternatives: Vec<ComplexSelector>,
}

impl SelectorList {
    /// The most-specific alternative. Used as the
    /// cascade tiebreaker for this rule.
    pub fn specificity(&self) -> Specificity {
        self.alternatives
            .iter()
            .map(|c| c.specificity)
            .max()
            .unwrap_or_default()
    }
}

/// Parse a CSS selector list from a token stream. The
/// caller is responsible for stripping any leading / trailing
/// whitespace and for handing in the right token slice
/// (typically everything from the start of a qualified
/// rule up to the `{`).
pub(crate) fn parse_selector_list(tokens: &[Token]) -> Result<SelectorList, String> {
    let mut alternatives = Vec::new();
    let mut current: Vec<Token> = Vec::new();
    for tok in tokens {
        if matches!(tok, Token::Eof) {
            break;
        }
        if matches!(tok, Token::Comma) {
            if current.is_empty() {
                return Err("empty selector alternative before ','".to_string());
            }
            alternatives.push(parse_complex_selector(&current)?);
            current.clear();
        } else {
            current.push(tok.clone());
        }
    }
    if current.is_empty() {
        if alternatives.is_empty() {
            return Err("empty selector list".to_string());
        }
    } else {
        alternatives.push(parse_complex_selector(&current)?);
    }
    Ok(SelectorList { alternatives })
}

/// Parse one complex selector from a token slice. The
/// slice is taken as a flat list of tokens in source
/// order; whitespace tokens are preserved in the slice
/// and detected by this function as the descendant
/// combinator.
pub(crate) fn parse_complex_selector(tokens: &[Token]) -> Result<ComplexSelector, String> {
    let mut steps: Vec<ComplexSelectorStep> = Vec::new();
    let mut current = CompoundSelector::default();
    let mut i = 0;
    let n = tokens.len();
    if n == 0 {
        return Err("empty selector".to_string());
    }

    while i < n {
        // Skip leading whitespace.
        if matches!(tokens[i], Token::Whitespace) {
            // If there is content in `current` and a
            // non-whitespace, non-combinator token follows,
            // this is the descendant combinator.
            let mut j = i;
            while j < n && matches!(tokens[j], Token::Whitespace) {
                j += 1;
            }
            if j < n && !current_is_empty(&current) && !is_combinator(tokens.get(j)) {
                steps.push(ComplexSelectorStep {
                    compound: std::mem::take(&mut current),
                    combinator: Some(Combinator::Descendant),
                });
            }
            i = j;
            continue;
        }

        let tok = &tokens[i];
        match tok {
            Token::Ident(name) => {
                if current.type_selector.is_none() {
                    current.type_selector = Some(TypeSelector::Element(name.to_ascii_lowercase()));
                } else {
                    return Err(format!("unexpected ident '{}' after type selector", name));
                }
            }
            Token::Delim('*') => {
                if current.type_selector.is_none() {
                    current.type_selector = Some(TypeSelector::Universal);
                } else {
                    return Err("unexpected '*' after type selector".to_string());
                }
            }
            Token::Delim('.') => {
                i += 1;
                let name = match tokens.get(i) {
                    Some(Token::Ident(n)) => n.to_ascii_lowercase(),
                    _ => return Err("expected ident after '.'".to_string()),
                };
                current.classes.push(name);
            }
            Token::Hash(name) => {
                if !is_ident(name) {
                    return Err(format!("expected ident after '#', got '{}'", name));
                }
                current.ids.push(name.to_ascii_lowercase());
            }
            Token::LBracket => {
                let (attr, consumed) = parse_attribute_selector(&tokens[i..])?;
                current.attributes.push(attr);
                i += consumed;
                continue;
            }
            Token::Colon => {
                i += 1;
                if let Some(Token::Colon) = tokens.get(i) {
                    i += 1;
                    let name = match tokens.get(i) {
                        Some(Token::Ident(n)) => n.to_ascii_lowercase(),
                        _ => return Err("expected ident after '::'".to_string()),
                    };
                    i += 1;
                    let arg = if let Some(Token::LParen) = tokens.get(i) {
                        let (s, c) = read_balanced(&tokens[i..], '(', ')')?;
                        i += c;
                        Some(s)
                    } else {
                        None
                    };
                    current.pseudo_elements.push((name, arg));
                } else {
                    let name = match tokens.get(i) {
                        Some(Token::Ident(n)) => n.to_ascii_lowercase(),
                        _ => return Err("expected ident after ':'".to_string()),
                    };
                    i += 1;
                    let arg = if let Some(Token::LParen) = tokens.get(i) {
                        let (s, c) = read_balanced(&tokens[i..], '(', ')')?;
                        i += c;
                        Some(s)
                    } else {
                        None
                    };
                    current.pseudo_classes.push((name, arg));
                }
                continue;
            }
            Token::Delim('>') => {
                steps.push(ComplexSelectorStep {
                    compound: std::mem::take(&mut current),
                    combinator: Some(Combinator::Child),
                });
                i += 1;
                continue;
            }
            Token::Delim('+') => {
                steps.push(ComplexSelectorStep {
                    compound: std::mem::take(&mut current),
                    combinator: Some(Combinator::NextSibling),
                });
                i += 1;
                continue;
            }
            Token::Delim('~') => {
                steps.push(ComplexSelectorStep {
                    compound: std::mem::take(&mut current),
                    combinator: Some(Combinator::SubsequentSibling),
                });
                i += 1;
                continue;
            }
            Token::ParenthesisClose | Token::Comma | Token::Eof => {
                return Err(format!("unexpected token {:?} in selector", tok));
            }
            _ => {
                return Err(format!("unexpected token {:?} in selector", tok));
            }
        }
        i += 1;
    }

    steps.push(ComplexSelectorStep {
        compound: current,
        combinator: None,
    });

    let specificity = ComplexSelector::compute_specificity(&steps);
    Ok(ComplexSelector { steps, specificity })
}

/// True if `c` has nothing on it. Used to detect
/// "leading whitespace" so we don't emit a phantom
/// descendant combinator.
fn current_is_empty(c: &CompoundSelector) -> bool {
    c.type_selector.is_none()
        && c.ids.is_empty()
        && c.classes.is_empty()
        && c.attributes.is_empty()
        && c.pseudo_classes.is_empty()
        && c.pseudo_elements.is_empty()
}

/// True if `t` is one of the four explicit combinator
/// delimiters (`>`, `+`, `~`). Used so that whitespace
/// before a combinator is not interpreted as the
/// descendant combinator.
fn is_combinator(t: Option<&Token>) -> bool {
    matches!(
        t,
        Some(Token::Delim('>')) | Some(Token::Delim('+')) | Some(Token::Delim('~'))
    )
}

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn parse_attribute_selector(tokens: &[Token]) -> Result<(AttributeSelector, usize), String> {
    // tokens[0] is '['
    let mut i = 1;
    let name = match tokens.get(i) {
        Some(Token::Ident(n)) => n.to_ascii_lowercase(),
        _ => return Err("expected ident after '['".to_string()),
    };
    i += 1;

    let mut matcher = AttributeMatcher::Present;
    let mut case = AttributeCase::default();

    if let Some(Token::RBracket) = tokens.get(i) {
        return Ok((
            AttributeSelector {
                name,
                matcher,
                case,
            },
            i + 1,
        ));
    }

    let matcher_kind = match tokens.get(i) {
        Some(Token::Delim('=')) => 0,
        Some(Token::Include) => 1,
        Some(Token::DashMatch) => 2,
        Some(Token::PrefixMatch) => 3,
        Some(Token::SuffixMatch) => 4,
        Some(Token::SubstringMatch) => 5,
        _ => return Err("expected matcher in attribute selector".to_string()),
    };
    i += 1;
    let (value, consumed) = read_attr_value(&tokens[i..])?;
    i += consumed;
    matcher = match matcher_kind {
        0 => AttributeMatcher::Exact(value),
        1 => AttributeMatcher::Includes(value),
        2 => AttributeMatcher::DashMatch(value),
        3 => AttributeMatcher::Prefix(value),
        4 => AttributeMatcher::Suffix(value),
        5 => AttributeMatcher::Substring(value),
        _ => unreachable!(),
    };

    // Skip whitespace before the optional case flag or the
    // closing `]`. `[type=text i]` has whitespace between
    // the value and the `i` flag, and we must consume it
    // before looking at the next token.
    while matches!(tokens.get(i), Some(Token::Whitespace)) {
        i += 1;
    }

    // Optional case flag.
    if let Some(Token::Ident(flag)) = tokens.get(i) {
        let lower = flag.to_ascii_lowercase();
        case = match lower.as_str() {
            "i" => AttributeCase::Insensitive,
            "s" => AttributeCase::ExplicitSensitive,
            _ => AttributeCase::Sensitive,
        };
        i += 1;
    }

    match tokens.get(i) {
        Some(Token::RBracket) => Ok((
            AttributeSelector {
                name,
                matcher,
                case,
            },
            i + 1,
        )),
        _ => Err("expected ']' to close attribute selector".to_string()),
    }
}

/// Read the value side of an attribute matcher. The value
/// is one of:
///   - a single ident (`foo`)
///   - a single string (`"foo"`)
///   - a single number (`42`)
///   - an ident preceded by a `.` delim (e.g. `.pdf` for
///     `href$=.pdf`)
///   - a value that starts with `-` followed by an ident
///     or number (e.g. `-1` for `data-foo=-1`)
fn read_attr_value(tokens: &[Token]) -> Result<(String, usize), String> {
    let mut s = String::new();
    let mut i = 0;
    if let Some(Token::Delim('.')) = tokens.get(i) {
        s.push('.');
        i += 1;
    } else if let Some(Token::Delim('-')) = tokens.get(i) {
        s.push('-');
        i += 1;
    }
    match tokens.get(i) {
        Some(Token::Ident(n)) => {
            s.push_str(n);
            Ok((s, i + 1))
        }
        Some(Token::String(n)) => {
            s.push_str(n);
            Ok((s, i + 1))
        }
        Some(Token::Number(n)) => {
            s.push_str(&format!("{}", n));
            Ok((s, i + 1))
        }
        Some(Token::Dimension(n, u)) => {
            s.push_str(&format!("{}{}", n, u));
            Ok((s, i + 1))
        }
        _ => Err("expected value in attribute selector".to_string()),
    }
}

/// Read a balanced `(...)` / `[...]` / `{...}` group from
/// `tokens`, returning the contents as a string and the
/// number of tokens consumed. Used for pseudo-class /
/// pseudo-element arguments.
fn read_balanced(tokens: &[Token], open: char, close: char) -> Result<(String, usize), String> {
    // We re-tokenise a substring. Cheap and clear.
    let open_tok = match open {
        '(' => Token::LParen,
        '[' => Token::LBracket,
        '{' => Token::LBrace,
        _ => return Err(format!("unsupported balanced opener '{}'", open)),
    };
    let close_tok = match close {
        ')' => Token::ParenthesisClose,
        ']' => Token::RBracket,
        '}' => Token::RBrace,
        _ => return Err(format!("unsupported balanced closer '{}'", close)),
    };
    if tokens.first() != Some(&open_tok) {
        return Err(format!("expected '{}' to start balanced group", open));
    }
    let mut depth = 0;
    let mut s = String::new();
    for (i, t) in tokens.iter().enumerate() {
        if *t == open_tok {
            depth += 1;
            if depth > 1 {
                s.push(open);
            }
        } else if *t == close_tok {
            depth -= 1;
            if depth == 0 {
                return Ok((s, i + 1));
            }
            s.push(close);
        } else {
            s.push_str(&t.to_string());
        }
    }
    Err(format!("unbalanced '{}…{}' group", open, close))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::tokenizer::tokenize;

    fn parse(src: &str) -> SelectorList {
        let toks = tokenize(src).expect("tokenize");
        parse_selector_list(&toks).expect("parse selector list")
    }

    #[test]
    fn type_selector() {
        let s = parse("div");
        assert_eq!(s.alternatives.len(), 1);
        assert_eq!(s.alternatives[0].steps.len(), 1);
        let c = &s.alternatives[0].steps[0].compound;
        assert!(matches!(c.type_selector, Some(TypeSelector::Element(ref n)) if n == "div"));
        assert_eq!(s.specificity(), Specificity::new(0, 0, 1));
    }

    #[test]
    fn universal_selector() {
        let s = parse("*");
        assert_eq!(s.specificity(), Specificity::new(0, 0, 0));
    }

    #[test]
    fn class_and_id() {
        let s = parse("div#main.warning");
        let c = &s.alternatives[0].steps[0].compound;
        assert_eq!(c.ids, vec!["main".to_string()]);
        assert_eq!(c.classes, vec!["warning".to_string()]);
        // 1 ID + 1 class + 1 type = (1, 1, 1)
        assert_eq!(s.specificity(), Specificity::new(1, 1, 1));
    }

    #[test]
    fn child_combinator() {
        let s = parse("ul > li");
        assert_eq!(s.alternatives[0].steps.len(), 2);
        assert_eq!(
            s.alternatives[0].steps[0].combinator,
            Some(Combinator::Child)
        );
    }

    #[test]
    fn descendant_combinator_is_whitespace() {
        let s = parse("ul li");
        assert_eq!(s.alternatives[0].steps.len(), 2);
        assert_eq!(
            s.alternatives[0].steps[0].combinator,
            Some(Combinator::Descendant)
        );
    }

    #[test]
    fn adjacent_and_general_sibling() {
        let a = parse("h1 + p");
        assert_eq!(
            a.alternatives[0].steps[0].combinator,
            Some(Combinator::NextSibling)
        );
        let b = parse("h1 ~ p");
        assert_eq!(
            b.alternatives[0].steps[0].combinator,
            Some(Combinator::SubsequentSibling)
        );
    }

    #[test]
    fn pseudo_class_and_element() {
        let s = parse("a:hover::after");
        let c = &s.alternatives[0].steps[0].compound;
        assert_eq!(c.pseudo_classes, vec![("hover".to_string(), None)]);
        assert_eq!(c.pseudo_elements, vec![("after".to_string(), None)]);
        // 1 type + 1 pseudo-class + 1 pseudo-element
        // (pseudo-element counts as a type slot).
        assert_eq!(s.specificity(), Specificity::new(0, 1, 2));
    }

    #[test]
    fn pseudo_class_with_arg() {
        let s = parse("li:nth-child(2n+1)");
        let c = &s.alternatives[0].steps[0].compound;
        assert_eq!(
            c.pseudo_classes,
            vec![("nth-child".to_string(), Some("2n+1".to_string()))]
        );
    }

    #[test]
    fn attribute_selector_present() {
        let s = parse("input[disabled]");
        let c = &s.alternatives[0].steps[0].compound;
        assert_eq!(c.attributes.len(), 1);
        assert_eq!(c.attributes[0].name, "disabled");
        assert!(matches!(c.attributes[0].matcher, AttributeMatcher::Present));
    }

    #[test]
    fn attribute_selector_exact() {
        let s = parse("a[href=\"https://example.com\"]");
        let c = &s.alternatives[0].steps[0].compound;
        assert_eq!(c.attributes.len(), 1);
        assert!(matches!(
            c.attributes[0].matcher,
            AttributeMatcher::Exact(ref v) if v == "https://example.com"
        ));
    }

    #[test]
    fn attribute_selector_misc_matchers() {
        let s = parse("[class~=foo]");
        let c = &s.alternatives[0].steps[0].compound;
        assert!(matches!(c.attributes[0].matcher, AttributeMatcher::Includes(ref v) if v == "foo"));
        let s = parse("[lang|=en]");
        let c = &s.alternatives[0].steps[0].compound;
        assert!(matches!(c.attributes[0].matcher, AttributeMatcher::DashMatch(ref v) if v == "en"));
        let s = parse("[href^=https]");
        let c = &s.alternatives[0].steps[0].compound;
        assert!(matches!(c.attributes[0].matcher, AttributeMatcher::Prefix(ref v) if v == "https"));
        let s = parse("[href$=.pdf]");
        let c = &s.alternatives[0].steps[0].compound;
        assert!(matches!(c.attributes[0].matcher, AttributeMatcher::Suffix(ref v) if v == ".pdf"));
        let s = parse("[href*=example]");
        let c = &s.alternatives[0].steps[0].compound;
        assert!(
            matches!(c.attributes[0].matcher, AttributeMatcher::Substring(ref v) if v == "example")
        );
    }

    #[test]
    fn attribute_selector_case_insensitive() {
        let s = parse("[type=text i]");
        let c = &s.alternatives[0].steps[0].compound;
        assert_eq!(c.attributes[0].case, AttributeCase::Insensitive);
    }

    #[test]
    fn selector_list_alternatives() {
        let s = parse("h1, h2, h3");
        assert_eq!(s.alternatives.len(), 3);
    }

    #[test]
    fn specificity_id_beats_class() {
        let s = parse("#a.b .c.d");
        // 1 ID + 3 classes = (1, 3, 0)
        assert_eq!(s.specificity(), Specificity::new(1, 3, 0));
    }

    #[test]
    fn complex_specificity() {
        // .a > #b.c[d=e] ::f g
        // = (1 ID, 2 classes, 1 attribute, 1 pseudo-class, 1
        // pseudo-element, 1 type) = (1, 4, 2)
        let s = parse(".a > #b.c[d=e]:hover::before g");
        assert_eq!(s.specificity(), Specificity::new(1, 4, 2));
    }
}
