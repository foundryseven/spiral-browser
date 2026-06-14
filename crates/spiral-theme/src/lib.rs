//! Spiral Browser — Theme Engine
//!
//! Theme engine for the Spiral Browser.

use serde::{Deserialize, Serialize};
use spiral_core::{AccentColor, BrowserConfig};

/// Theme mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

/// Theme tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeTokens {
    /// Background primary color.
    pub bg_primary: String,
    /// Background secondary color.
    pub bg_secondary: String,
    /// Text primary color.
    pub text_primary: String,
    /// Text secondary color.
    pub text_secondary: String,
    /// Accent color.
    pub accent: String,
    /// Border color.
    pub border: String,
    /// Shadow color.
    pub shadow: String,
}

/// Theme engine.
pub struct ThemeEngine {
    /// Current theme mode.
    mode: ThemeMode,
    /// Current accent color.
    accent: AccentColor,
}

impl ThemeEngine {
    /// Create a new theme engine.
    pub fn new(config: &BrowserConfig) -> Self {
        Self {
            mode: if config.dark_mode {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
            accent: config.accent_color,
        }
    }

    /// Get theme tokens for the current theme.
    pub fn tokens(&self) -> ThemeTokens {
        match self.mode {
            ThemeMode::Dark => self.dark_tokens(),
            ThemeMode::Light => self.light_tokens(),
            ThemeMode::System => {
                // Phase 2: Detect system preference
                self.dark_tokens()
            }
        }
    }

    /// Get dark theme tokens.
    fn dark_tokens(&self) -> ThemeTokens {
        let accent = self.accent_color_hex();
        ThemeTokens {
            bg_primary: "#1a1b26".to_string(),
            bg_secondary: "#24283b".to_string(),
            text_primary: "#c0caf5".to_string(),
            text_secondary: "#a9b1d6".to_string(),
            accent,
            border: "#3b4261".to_string(),
            shadow: "#000000".to_string(),
        }
    }

    /// Get light theme tokens.
    fn light_tokens(&self) -> ThemeTokens {
        let accent = self.accent_color_hex();
        ThemeTokens {
            bg_primary: "#ffffff".to_string(),
            bg_secondary: "#f5f5f5".to_string(),
            text_primary: "#1a1b26".to_string(),
            text_secondary: "#4c566a".to_string(),
            accent,
            border: "#e0e0e0".to_string(),
            shadow: "#000000".to_string(),
        }
    }

    /// Get accent color hex value.
    fn accent_color_hex(&self) -> String {
        match self.accent {
            AccentColor::Indigo => "#6366f1".to_string(),
            AccentColor::Violet => "#8b5cf6".to_string(),
            AccentColor::Emerald => "#10b981".to_string(),
            AccentColor::Sky => "#0ea5e9".to_string(),
            AccentColor::Amber => "#f59e0b".to_string(),
            AccentColor::Rose => "#f43f5e".to_string(),
        }
    }

    /// Set theme mode.
    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
    }

    /// Set accent color.
    pub fn set_accent(&mut self, accent: AccentColor) {
        self.accent = accent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_core::BrowserConfig;

    #[test]
    fn test_theme_engine_default() {
        let config = BrowserConfig::default();
        let engine = ThemeEngine::new(&config);
        assert_eq!(engine.mode, ThemeMode::Dark);
    }

    #[test]
    fn test_dark_tokens() {
        let config = BrowserConfig::default();
        let engine = ThemeEngine::new(&config);
        let tokens = engine.tokens();
        assert_eq!(tokens.bg_primary, "#1a1b26");
    }

    #[test]
    fn test_light_tokens() {
        let config = BrowserConfig {
            dark_mode: false,
            ..Default::default()
        };
        let engine = ThemeEngine::new(&config);
        let tokens = engine.tokens();
        assert_eq!(tokens.bg_primary, "#ffffff");
    }

    #[test]
    fn test_accent_color() {
        let config = BrowserConfig {
            accent_color: AccentColor::Violet,
            ..Default::default()
        };
        let engine = ThemeEngine::new(&config);
        let tokens = engine.tokens();
        assert_eq!(tokens.accent, "#8b5cf6");
    }
}
