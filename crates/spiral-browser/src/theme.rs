//! Theme bridge between `spiral-theme` (hex strings) and the renderer
//! (`spiral_paint::Color`).

use spiral_core::Color;
use spiral_theme::{ThemeEngine, ThemeTokens};

/// Parsed theme tokens ready to feed the renderer.
#[derive(Debug, Clone, Copy)]
pub struct BrowserTheme {
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub accent: Color,
}

impl BrowserTheme {
    #[must_use]
    pub fn from_engine(engine: &ThemeEngine) -> Self {
        Self::from_tokens(&engine.tokens())
    }

    #[must_use]
    pub fn from_tokens(tokens: &ThemeTokens) -> Self {
        Self {
            bg_primary: parse_hex(&tokens.bg_primary),
            bg_secondary: parse_hex(&tokens.bg_secondary),
            text_primary: parse_hex(&tokens.text_primary),
            text_secondary: parse_hex(&tokens.text_secondary),
            accent: parse_hex(&tokens.accent),
        }
    }
}

impl From<&ThemeEngine> for BrowserTheme {
    fn from(engine: &ThemeEngine) -> Self {
        Self::from_engine(engine)
    }
}

fn parse_hex(s: &str) -> Color {
    let bytes = s.as_bytes();
    if bytes.len() < 7 || bytes[0] != b'#' {
        return Color {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
        };
    }
    let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0);
    Color { r, g, b, a: 1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_core::{AccentColor, BrowserConfig};

    fn engine() -> ThemeEngine {
        let cfg = BrowserConfig {
            accent_color: AccentColor::Indigo,
            ..Default::default()
        };
        ThemeEngine::new(&cfg)
    }

    #[test]
    fn parses_bg_hex() {
        let t = BrowserTheme::from_engine(&engine());
        // The theme engine must have produced some non-black background.
        let bg = t.bg_primary;
        let sum = u32::from(bg.r) + u32::from(bg.g) + u32::from(bg.b);
        assert!(sum > 0, "expected non-black background, got {bg:?}");
        assert!((0.0..=1.0).contains(&bg.a));
    }

    #[test]
    fn parses_accent_hex() {
        let t = BrowserTheme::from_engine(&engine());
        let sum = u32::from(t.accent.r) + u32::from(t.accent.g) + u32::from(t.accent.b);
        assert!(sum > 0, "expected non-black accent");
    }

    #[test]
    fn malformed_hex_falls_back_to_black() {
        let c = parse_hex("not a colour");
        assert_eq!((c.r, c.g, c.b), (0, 0, 0));
    }
}
