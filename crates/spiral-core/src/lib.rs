//! Spiral Browser — Core Types
//!
//! Shared types and IPC protocol for the Spiral Browser.

use serde::{Deserialize, Serialize};

/// Unique identifier for a browser tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TabId(pub u64);

/// Unique identifier for a render node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RenderNodeId(pub u64);

/// Browser configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    /// Homepage URL.
    pub homepage: String,
    /// Proxy configuration.
    pub proxy: Option<String>,
    /// Default font size in pixels.
    pub font_size: f32,
    /// Accent color.
    pub accent_color: AccentColor,
    /// Dark mode preference.
    pub dark_mode: bool,
    /// Tab position.
    pub tab_position: TabPosition,
    /// Auto-hide chrome.
    pub auto_hide_chrome: bool,
    /// Sandbox renderer processes.
    pub sandbox_renderer: bool,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            homepage: "about:blank".to_string(),
            proxy: None,
            font_size: 16.0,
            accent_color: AccentColor::Indigo,
            dark_mode: true,
            tab_position: TabPosition::Left,
            auto_hide_chrome: true,
            sandbox_renderer: true,
        }
    }
}

/// Accent color for the browser theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccentColor {
    Indigo,
    Violet,
    Emerald,
    Sky,
    Amber,
    Rose,
}

/// Tab position in the browser UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabPosition {
    Left,
    Right,
    Top,
}

/// IPC message envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IPCMessage {
    /// Browser to renderer message.
    BrowserToRenderer(BrowserToRenderer),
    /// Renderer to browser message.
    RendererToBrowser(RendererToBrowser),
}

/// Messages from browser process to renderer process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserToRenderer {
    /// Navigate to a URL.
    Navigate { url: String },
    /// Update DOM operations.
    UpdateDOM {
        node_id: u64,
        operations: Vec<DomOp>,
    },
    /// Resize the viewport.
    Resize { width: f32, height: f32 },
    /// Input event.
    InputEvent { event: InputEvent },
    /// Reload the page.
    Reload,
    /// Stop loading.
    Stop,
}

/// Messages from renderer process to browser process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RendererToBrowser {
    /// DOM has finished loading.
    DOMLoaded { title: String },
    /// Load progress update.
    LoadProgress { progress: f32 },
    /// Navigation completed.
    NavigateComplete { url: String },
    /// Request navigation (link click, form submission).
    RequestNavigate { url: String },
    /// Console message from JavaScript.
    ConsoleMessage { level: LogLevel, text: String },
}

/// DOM manipulation operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomOp {
    AppendChild,
    RemoveChild,
    InsertBefore,
    SetAttribute { name: String, value: String },
    RemoveAttribute { name: String },
    SetTextContent { text: String },
}

/// Input events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEvent {
    MouseDown { x: f32, y: f32, button: MouseButton },
    MouseUp { x: f32, y: f32, button: MouseButton },
    MouseMove { x: f32, y: f32 },
    KeyDown { key: String, modifiers: u32 },
    KeyUp { key: String, modifiers: u32 },
    Scroll { delta_x: f32, delta_y: f32 },
}

/// Mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Log levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Unified error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IPC error: {0}")]
    Ipc(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Layout error: {0}")]
    Layout(String),

    #[error("Render error: {0}")]
    Render(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("JavaScript error: {0}")]
    JavaScript(String),

    #[error("Sandbox error: {0}")]
    Sandbox(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_config_default() {
        let config = BrowserConfig::default();
        assert_eq!(config.homepage, "about:blank");
        assert_eq!(config.font_size, 16.0);
        assert!(config.dark_mode);
    }

    #[test]
    fn test_tab_id() {
        let id = TabId(1);
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_ipc_message_serialization() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
            url: "https://example.com".to_string(),
        });
        let serialized = bincode::serialize(&msg).unwrap();
        let deserialized: IPCMessage = bincode::deserialize(&serialized).unwrap();
        match deserialized {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url }) => {
                assert_eq!(url, "https://example.com");
            }
            _ => panic!("Wrong message type"),
        }
    }
}
