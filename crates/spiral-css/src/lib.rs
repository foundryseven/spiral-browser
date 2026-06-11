//! Spiral Browser — CSS Parser
//!
//! CSS parser and cascade engine for the Spiral Browser.

use spiral_core::{Error, Result};
use std::collections::HashMap;

/// CSS property value.
#[derive(Debug, Clone)]
pub enum CssValue {
    Length(f32),
    Percentage(f32),
    Auto,
    Color(Color),
    String(String),
    Keyword(String),
    None,
}

/// CSS color.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

/// CSS property.
#[derive(Debug, Clone)]
pub struct CssProperty {
    pub name: String,
    pub value: CssValue,
    pub important: bool,
}

/// CSS rule selector.
#[derive(Debug, Clone)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

/// Selector part.
#[derive(Debug, Clone)]
pub enum SelectorPart {
    Element(String),
    Class(String),
    Id(String),
    Universal,
}

/// CSS rule.
#[derive(Debug, Clone)]
pub struct CssRule {
    pub selector: Selector,
    pub properties: Vec<CssProperty>,
    pub specificity: (u32, u32, u32),
}

/// CSS stylesheet.
#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<CssRule>,
}

/// CSS parser.
pub struct CssParser {
    stylesheet: Stylesheet,
}

impl CssParser {
    /// Create a new CSS parser.
    pub fn new() -> Self {
        Self {
            stylesheet: Stylesheet {
                rules: Vec::new(),
            },
        }
    }

    /// Parse CSS string into a stylesheet.
    pub fn parse(&mut self, css: &str) -> Result<()> {
        // For Phase 1, implement basic CSS parsing
        // Phase 2 will integrate cssparser properly
        self.parse_simple(css)
    }

    /// Get the stylesheet.
    pub fn stylesheet(&self) -> &Stylesheet {
        &self.stylesheet
    }

    /// Simple CSS parser for basic rules.
    fn parse_simple(&mut self, css: &str) -> Result<()> {
        let mut i = 0;
        let bytes = css.as_bytes();

        while i < bytes.len() {
            // Skip whitespace
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }

            if i >= bytes.len() {
                break;
            }

            // Find selector (everything before '{')
            let selector_start = i;
            while i < bytes.len() && bytes[i] != b'{' {
                i += 1;
            }

            if i >= bytes.len() {
                break;
            }

            let selector_text = css[selector_start..i].trim();
            i += 1; // skip '{'

            // Find properties (everything before '}')
            let props_start = i;
            while i < bytes.len() && bytes[i] != b'}' {
                i += 1;
            }

            let props_text = css[props_start..i].trim();
            i += 1; // skip '}'

            // Parse selector
            let selector = self.parse_selector(selector_text);

            // Parse properties
            let properties = self.parse_properties(props_text);

            // Calculate specificity
            let specificity = self.calculate_specificity(&selector);

            let rule = CssRule {
                selector,
                properties,
                specificity,
            };

            self.stylesheet.rules.push(rule);
        }

        Ok(())
    }

    /// Parse a selector string.
    fn parse_selector(&self, text: &str) -> Selector {
        let parts: Vec<SelectorPart> = text
            .split_whitespace()
            .map(|part| {
                if part.starts_with('.') {
                    SelectorPart::Class(part[1..].to_string())
                } else if part.starts_with('#') {
                    SelectorPart::Id(part[1..].to_string())
                } else if part == "*" {
                    SelectorPart::Universal
                } else {
                    SelectorPart::Element(part.to_string())
                }
            })
            .collect();

        Selector { parts }
    }

    /// Parse properties string.
    fn parse_properties(&self, text: &str) -> Vec<CssProperty> {
        let mut properties = Vec::new();

        for declaration in text.split(';') {
            let declaration = declaration.trim();
            if declaration.is_empty() {
                continue;
            }

            if let Some((name, value)) = declaration.split_once(':') {
                let name = name.trim().to_string();
                let value_str = value.trim();
                let important = value_str.ends_with("!important");
                let value_str = if important {
                    value_str.trim_end_matches("!important").trim()
                } else {
                    value_str
                };

                let value = self.parse_value(value_str);

                properties.push(CssProperty {
                    name,
                    value,
                    important,
                });
            }
        }

        properties
    }

    /// Parse a CSS value.
    fn parse_value(&self, text: &str) -> CssValue {
        if text == "auto" {
            CssValue::Auto
        } else if text == "none" {
            CssValue::None
        } else if let Some(pct) = text.strip_suffix('%') {
            if let Ok(val) = pct.parse::<f32>() {
                return CssValue::Percentage(val);
            }
        } else if let Some(px) = text.strip_suffix("px") {
            if let Ok(val) = px.parse::<f32>() {
                return CssValue::Length(val);
            }
        } else if text.starts_with('#') && text.len() == 7 {
            // Parse hex color
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&text[1..3], 16),
                u8::from_str_radix(&text[3..5], 16),
                u8::from_str_radix(&text[5..7], 16),
            ) {
                return CssValue::Color(Color { r, g, b, a: 1.0 });
            }
        }

        CssValue::Keyword(text.to_string())
    }

    /// Calculate selector specificity.
    fn calculate_specificity(&self, selector: &Selector) -> (u32, u32, u32) {
        let mut ids = 0;
        let mut classes = 0;
        let mut elements = 0;

        for part in &selector.parts {
            match part {
                SelectorPart::Id(_) => ids += 1,
                SelectorPart::Class(_) => classes += 1,
                SelectorPart::Element(_) => elements += 1,
                SelectorPart::Universal => {}
            }
        }

        (ids, classes, elements)
    }
}

impl Default for CssParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_css() {
        let mut parser = CssParser::new();
        parser.parse("div { color: red; }").unwrap();
        assert_eq!(parser.stylesheet().rules.len(), 1);
    }

    #[test]
    fn test_parse_selector() {
        let parser = CssParser::new();
        let selector = parser.parse_selector(".container #main div");
        assert_eq!(selector.parts.len(), 4);
    }

    #[test]
    fn test_parse_properties() {
        let parser = CssParser::new();
        let props = parser.parse_properties("color: red; font-size: 16px;");
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_calculate_specificity() {
        let parser = CssParser::new();
        let selector = parser.parse_selector("#main .container div");
        let spec = parser.calculate_specificity(&selector);
        assert_eq!(spec, (1, 1, 1));
    }

    #[test]
    fn test_parse_hex_color() {
        let parser = CssParser::new();
        match parser.parse_value("#ff0000") {
            CssValue::Color(c) => {
                assert_eq!(c.r, 255);
                assert_eq!(c.g, 0);
                assert_eq!(c.b, 0);
            }
            _ => panic!("Expected color"),
        }
    }
}
