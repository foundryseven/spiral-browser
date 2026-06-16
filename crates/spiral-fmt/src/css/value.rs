//! CSS value types.
//!
//! The set of value variants is deliberately a small,
//! minimum-viable subset. It is enough to express the
//! values that appear in the M4.4.1 test corpus and the
//! values the layout engine actually consumes:
//!
//! - Keywords (`auto`, `none`, `block`, `flex`, etc.)
//! - `<length>` with a `px` unit (the unit Spiral's
//!   layout engine currently understands)
//! - `<percentage>`
//! - `<color>` in `#rrggbb` and `#rgb` form, plus the
//!   small set of named colours the M4.4.1 tests use
//! - `<number>` (unitless)
//! - A `String` for quoted strings (e.g. `url("...")`,
//!   `content: "..."`)
//! - An `Important` flag captured at the parser level,
//!   surfaced separately on the property

use spiral_core::Color;

/// A single CSS property value, post-tokenisation.
///
/// Variants are tagged by syntactic shape rather than by
/// property name. A `Keyword("red")` and a `Color(...)`
/// containing the same RGB triple are different things —
/// the former is an unparsed token the cascade has to
/// resolve, the latter has been resolved to a colour.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A unitless number (e.g. `0.5`, `1`, `1e3`).
    Number(f32),
    /// A `<length>` with an explicit `px` unit. The unit
    /// is dropped; the layout engine operates in pixels.
    Length(f32),
    /// A `<percentage>` (e.g. `50%`).
    Percentage(f32),
    /// A colour. Hex (`#rgb`, `#rrggbb`, `#rrggbbaa`)
    /// and named colours land here; `transparent` and
    /// `currentColor` are also resolved to `Color` when
    /// they appear.
    Color(Color),
    /// A CSS keyword (e.g. `auto`, `none`, `block`,
    /// `flex`, `center`, `inherit`, …).
    Keyword(String),
    /// A quoted string value.
    String(String),
    /// A list of values separated by whitespace (e.g.
    /// `1px solid black`). The parser produces this when
    /// a declaration's value is more than one token but
    /// we have not yet dispatched on the property name.
    List(Vec<Value>),
    /// A function call — `name(arg, arg)`. The M4.4.1
    /// minimum stores the function name and the raw
    /// argument token slice; the runtime is responsible
    /// for interpreting it.
    Function(String, Vec<Value>),
}

impl Value {
    /// Render the value back to CSS source for diagnostic
    /// and snapshot purposes. This is not a round-trip
    /// canonisation — it's a debug helper.
    pub fn to_css(&self) -> String {
        match self {
            Value::Number(n) => format!("{}", n),
            Value::Length(n) => format!("{}px", n),
            Value::Percentage(n) => format!("{}%", n),
            Value::Color(c) => {
                if c.a >= 1.0 {
                    format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)
                } else {
                    format!(
                        "#{:02x}{:02x}{:02x}{:02x}",
                        (c.a * 255.0) as u8,
                        c.r,
                        c.g,
                        c.b
                    )
                }
            }
            Value::Keyword(k) => k.clone(),
            Value::String(s) => format!("\"{}\"", s),
            Value::List(values) => values
                .iter()
                .map(|v| v.to_css())
                .collect::<Vec<_>>()
                .join(" "),
            Value::Function(name, args) => {
                let arg_s = args
                    .iter()
                    .map(|v| v.to_css())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", name, arg_s)
            }
        }
    }
}

/// Parse a hex colour from one of `#rgb`, `#rrggbb`, or
/// `#rrggbbaa`. Returns `None` for any other shape.
pub(crate) fn parse_hex_color(input: &str) -> Option<Color> {
    let body = input.strip_prefix('#')?;
    let bytes = body.as_bytes();
    let (r, g, b, a) = match bytes.len() {
        3 => {
            let r = expand_nibble(bytes[0])?;
            let g = expand_nibble(bytes[1])?;
            let b = expand_nibble(bytes[2])?;
            (r, g, b, 1.0)
        }
        4 => {
            let r = expand_nibble(bytes[0])?;
            let g = expand_nibble(bytes[1])?;
            let b = expand_nibble(bytes[2])?;
            let a = expand_nibble(bytes[3])? as f32 / 255.0;
            (r, g, b, a)
        }
        6 => {
            let r = u8::from_str_radix(&body[0..2], 16).ok()?;
            let g = u8::from_str_radix(&body[2..4], 16).ok()?;
            let b = u8::from_str_radix(&body[4..6], 16).ok()?;
            (r, g, b, 1.0)
        }
        8 => {
            let r = u8::from_str_radix(&body[0..2], 16).ok()?;
            let g = u8::from_str_radix(&body[2..4], 16).ok()?;
            let b = u8::from_str_radix(&body[4..6], 16).ok()?;
            let a = u8::from_str_radix(&body[6..8], 16).ok()? as f32 / 255.0;
            (r, g, b, a)
        }
        _ => return None,
    };
    Some(Color { r, g, b, a })
}

fn expand_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some((b - b'0') * 17),
        b'a'..=b'f' => Some((b - b'a' + 10) * 17),
        b'A'..=b'F' => Some((b - b'A' + 10) * 17),
        _ => None,
    }
}

/// Look up a small set of CSS named colours by name. Returns
/// `None` for unknown names — the caller should fall through
/// to a keyword.
pub(crate) fn named_color(name: &str) -> Option<Color> {
    let lower = name.to_ascii_lowercase();
    let rgb = match lower.as_str() {
        "black" => (0, 0, 0),
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 128, 0),
        "blue" => (0, 0, 255),
        "yellow" => (255, 255, 0),
        "cyan" => (0, 255, 255),
        "magenta" => (255, 0, 255),
        "gray" | "grey" => (128, 128, 128),
        "silver" => (192, 192, 192),
        "maroon" => (128, 0, 0),
        "olive" => (128, 128, 0),
        "lime" => (0, 255, 0),
        "aqua" => (0, 255, 255),
        "teal" => (0, 128, 128),
        "navy" => (0, 0, 128),
        "fuchsia" => (255, 0, 255),
        "purple" => (128, 0, 128),
        "orange" => (255, 165, 0),
        "pink" => (255, 192, 203),
        "transparent" => (0, 0, 0), // alpha 0
        _ => return None,
    };
    let a = if lower == "transparent" { 0.0 } else { 1.0 };
    Some(Color {
        r: rgb.0,
        g: rgb.1,
        b: rgb.2,
        a,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_color_short_form() {
        let c = parse_hex_color("#f00").expect("#f00");
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
        assert!((c.a - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn parse_hex_color_long_form() {
        let c = parse_hex_color("#ff8800").expect("#ff8800");
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0x88);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn parse_hex_color_with_alpha() {
        let c = parse_hex_color("#11223344").expect("#11223344");
        assert_eq!(c.r, 0x11);
        assert_eq!(c.g, 0x22);
        assert_eq!(c.b, 0x33);
        assert!((c.a - 0x44 as f32 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn parse_hex_color_rejects_invalid() {
        assert!(parse_hex_color("#xyz").is_none());
        assert!(parse_hex_color("#12345").is_none());
        assert!(parse_hex_color("ff0000").is_none());
    }

    #[test]
    fn named_color_known() {
        assert!(named_color("red").is_some());
        assert!(named_color("Blue").is_some());
        assert!(named_color("gray").is_some());
        assert!(named_color("grey").is_some());
        assert!(named_color("transparent").is_some());
    }

    #[test]
    fn named_color_unknown() {
        assert!(named_color("notacolor").is_none());
    }

    #[test]
    fn value_to_css_round_trip() {
        assert_eq!(Value::Length(16.0).to_css(), "16px");
        assert_eq!(Value::Percentage(50.0).to_css(), "50%");
        assert_eq!(Value::Keyword("auto".to_string()).to_css(), "auto");
    }
}
