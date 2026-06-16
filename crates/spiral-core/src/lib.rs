//! Spiral Browser — Core Types
//!
//! Shared types and IPC protocol for the Spiral Browser.

use serde::{Deserialize, Serialize};

/// Unique identifier for a browser tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TabId(pub u64);

impl std::fmt::Display for TabId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a render node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RenderNodeId(pub u64);

/// RGBA colour with 8-bit sRGB channels and a linear `[0.0, 1.0]` alpha.
///
/// This is the canonical colour type used across all Spiral crates.
/// `spiral-css`, `spiral-paint`, and `spiral-render` re-export it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
    /// Alpha channel in `[0.0, 1.0]`.
    pub a: f32,
}

impl Color {
    /// Fully opaque black.
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };
    /// Fully opaque white.
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    };
    /// Fully transparent black.
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0.0,
    };

    /// Parse a `#RRGGBB` hex string. Returns `None` on malformed input.
    #[must_use]
    pub fn from_hex(s: &str) -> Option<Self> {
        let b = s.as_bytes();
        if b.len() < 7 || b[0] != b'#' {
            return None;
        }
        let r = u8::from_str_radix(core::str::from_utf8(&b[1..3]).ok()?, 16).ok()?;
        let g = u8::from_str_radix(core::str::from_utf8(&b[3..5]).ok()?, 16).ok()?;
        let b_ch = u8::from_str_radix(core::str::from_utf8(&b[5..7]).ok()?, 16).ok()?;
        Some(Self {
            r,
            g,
            b: b_ch,
            a: 1.0,
        })
    }
}

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
    /// Process-to-process handshake (used by renderer on connect).
    Hello(HelloMessage),
}

/// Handshake payload exchanged when a renderer process connects.
///
/// Sent as `IPCMessage::Hello(HelloMessage)` immediately after the transport
/// is established. The renderer identifies itself and announces its initial
/// viewport; the browser replies with a `BrowserToRenderer::Resize` and a
/// `BrowserToRenderer::Navigate` for the tab's current URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloMessage {
    /// Tab this renderer is bound to.
    pub tab_id: TabId,
    /// Protocol version the renderer speaks.
    pub protocol_version: u32,
    /// Initial viewport width in logical pixels.
    pub viewport_width: f32,
    /// Initial viewport height in logical pixels.
    pub viewport_height: f32,
}

impl HelloMessage {
    /// Current protocol version. Bump on breaking changes to `IPCMessage`.
    pub const PROTOCOL_VERSION: u32 = 1;
}

/// Messages from browser process to renderer process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserToRenderer {
    /// Navigate to a URL.
    Navigate {
        /// Tab to navigate.
        tab_id: TabId,
        /// Destination URL.
        url: String,
    },
    /// Update DOM operations.
    UpdateDOM {
        /// Target tab.
        tab_id: TabId,
        /// Root node of the diff.
        node_id: u64,
        /// Operations to apply.
        operations: Vec<DomOp>,
    },
    /// Resize the viewport.
    Resize {
        /// Tab to resize.
        tab_id: TabId,
        /// New width in logical pixels.
        width: f32,
        /// New height in logical pixels.
        height: f32,
    },
    /// Input event.
    InputEvent {
        /// Tab receiving the input.
        tab_id: TabId,
        /// Event payload.
        event: InputEvent,
    },
    /// Reload the page.
    Reload {
        /// Tab to reload.
        tab_id: TabId,
    },
    /// Stop loading.
    Stop {
        /// Tab to stop.
        tab_id: TabId,
    },
    /// Log a message to the renderer's logger.
    Log {
        /// Severity.
        level: LogLevel,
        /// Message body.
        message: String,
    },
    /// Acknowledge a previously-issued screenshot request.
    ScreenshotAck {
        /// Request id being acknowledged.
        request_id: u64,
    },
}

/// Messages from renderer process to browser process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RendererToBrowser {
    /// Renderer is ready to accept commands for `tab_id`.
    RendererReady {
        /// Tab the renderer is bound to.
        tab_id: TabId,
    },
    /// DOM has finished loading.
    DOMLoaded {
        /// Tab the event refers to.
        tab_id: TabId,
        /// Document title once parsed.
        title: String,
        /// Final URL after redirects.
        url: String,
    },
    /// Load progress update.
    LoadProgress {
        /// Tab the event refers to.
        tab_id: TabId,
        /// Progress in `[0.0, 1.0]`.
        progress: f32,
    },
    /// Navigation completed.
    NavigateComplete {
        /// Tab the event refers to.
        tab_id: TabId,
        /// Final URL.
        url: String,
        /// Document title.
        title: String,
    },
    /// Request navigation (link click, form submission).
    RequestNavigate {
        /// Tab the event refers to.
        tab_id: TabId,
        /// Destination URL.
        url: String,
    },
    /// Console message from JavaScript.
    ConsoleMessage {
        /// Tab the event refers to.
        tab_id: TabId,
        /// Severity.
        level: LogLevel,
        /// Message text.
        text: String,
    },
    /// Input event (mouse, keyboard, scroll) the renderer wants to forward.
    Input {
        /// Tab the event refers to.
        tab_id: TabId,
        /// Event payload.
        event: InputEvent,
    },
    /// Renderer-side screenshot request, e.g. for a thumbnail.
    Screenshot {
        /// Tab the event refers to.
        tab_id: TabId,
        /// Opaque request id to correlate the eventual ack / response.
        request_id: u64,
    },
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

    #[error("DOM error: {0}")]
    Dom(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("JavaScript error: {0}")]
    JavaScript(String),

    #[error("Sandbox error: {0}")]
    Sandbox(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // ---------- Task 1.2: BrowserConfig ----------

    #[test]
    fn browser_config_default_matches_spec() {
        let config = BrowserConfig::default();
        assert_eq!(config.homepage, "about:blank");
        assert_eq!(config.proxy, None);
        assert_eq!(config.font_size, 16.0);
        assert_eq!(config.accent_color, AccentColor::Indigo);
        assert!(config.dark_mode);
        assert_eq!(config.tab_position, TabPosition::Left);
        assert!(config.auto_hide_chrome);
        assert!(config.sandbox_renderer);
    }

    #[test]
    fn browser_config_bincode_round_trip() {
        let config = BrowserConfig {
            homepage: "https://spiral-browser.example".to_string(),
            proxy: Some("socks5://127.0.0.1:9050".to_string()),
            font_size: 18.5,
            accent_color: AccentColor::Emerald,
            dark_mode: false,
            tab_position: TabPosition::Top,
            auto_hide_chrome: false,
            sandbox_renderer: false,
        };

        let bytes = bincode::serialize(&config).expect("serialise");
        let decoded: BrowserConfig = bincode::deserialize(&bytes).expect("deserialise");

        assert_eq!(decoded.homepage, config.homepage);
        assert_eq!(decoded.proxy, config.proxy);
        assert_eq!(decoded.font_size, config.font_size);
        assert_eq!(decoded.accent_color, config.accent_color);
        assert_eq!(decoded.dark_mode, config.dark_mode);
        assert_eq!(decoded.tab_position, config.tab_position);
        assert_eq!(decoded.auto_hide_chrome, config.auto_hide_chrome);
        assert_eq!(decoded.sandbox_renderer, config.sandbox_renderer);
    }

    #[test]
    fn browser_config_clone_preserves_equality() {
        let original = BrowserConfig::default();
        let cloned = original.clone();
        let lhs = bincode::serialize(&original).unwrap();
        let rhs = bincode::serialize(&cloned).unwrap();
        assert_eq!(lhs, rhs);
    }

    // ---------- Task 1.3: TabId ----------

    #[test]
    fn tab_id_equality_and_hash() {
        let a = TabId(42);
        let b = TabId(42);
        let c = TabId(43);
        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut set: HashSet<TabId> = HashSet::new();
        set.insert(a);
        set.insert(b);
        set.insert(c);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&a));
    }

    #[test]
    fn tab_id_display_and_debug() {
        let id = TabId(7);
        assert_eq!(format!("{id}"), "7");
        assert_eq!(format!("{id:?}"), "TabId(7)");
    }

    #[test]
    fn render_node_id_equality_and_hash() {
        let a = RenderNodeId(1);
        let b = RenderNodeId(1);
        assert_eq!(a, b);
        let mut set: HashSet<RenderNodeId> = HashSet::new();
        set.insert(a);
        set.insert(b);
        assert_eq!(set.len(), 1);
    }

    // ---------- Task 1.4: IPCMessage ----------

    fn round_trip(msg: &IPCMessage) -> IPCMessage {
        let bytes = bincode::serialize(msg).expect("serialise");
        bincode::deserialize(&bytes).expect("deserialise")
    }

    #[test]
    fn ipc_navigate_round_trip() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
            tab_id: TabId(1),
            url: "https://example.com".to_string(),
        });
        match round_trip(&msg) {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url, .. }) => {
                assert_eq!(url, "https://example.com");
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn ipc_update_dom_round_trip() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::UpdateDOM {
            tab_id: TabId(1),
            node_id: 99,
            operations: vec![
                DomOp::AppendChild,
                DomOp::SetAttribute {
                    name: "class".to_string(),
                    value: "container".to_string(),
                },
                DomOp::SetTextContent {
                    text: "hello".to_string(),
                },
                DomOp::RemoveAttribute {
                    name: "id".to_string(),
                },
                DomOp::RemoveChild,
                DomOp::InsertBefore,
            ],
        });
        match round_trip(&msg) {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::UpdateDOM {
                node_id,
                operations,
                ..
            }) => {
                assert_eq!(node_id, 99);
                assert_eq!(operations.len(), 6);
                assert!(matches!(operations[0], DomOp::AppendChild));
                assert!(matches!(operations[1], DomOp::SetAttribute { .. }));
                assert!(matches!(operations[2], DomOp::SetTextContent { .. }));
                assert!(matches!(operations[3], DomOp::RemoveAttribute { .. }));
                assert!(matches!(operations[4], DomOp::RemoveChild));
                assert!(matches!(operations[5], DomOp::InsertBefore));
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn ipc_resize_round_trip() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Resize {
            tab_id: TabId(1),
            width: 1024.0,
            height: 768.5,
        });
        match round_trip(&msg) {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Resize { width, height, .. }) => {
                assert_eq!(width, 1024.0);
                assert_eq!(height, 768.5);
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn ipc_input_event_round_trip() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::InputEvent {
            tab_id: TabId(1),
            event: InputEvent::MouseDown {
                x: 10.0,
                y: 20.0,
                button: MouseButton::Left,
            },
        });
        match round_trip(&msg) {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::InputEvent {
                event: InputEvent::MouseDown { x, y, button },
                ..
            }) => {
                assert_eq!(x, 10.0);
                assert_eq!(y, 20.0);
                assert_eq!(button, MouseButton::Left);
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn ipc_keyboard_input_event_round_trip() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::InputEvent {
            tab_id: TabId(1),
            event: InputEvent::KeyDown {
                key: "Enter".to_string(),
                modifiers: 0b1010,
            },
        });
        match round_trip(&msg) {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::InputEvent {
                event: InputEvent::KeyDown { key, modifiers },
                ..
            }) => {
                assert_eq!(key, "Enter");
                assert_eq!(modifiers, 0b1010);
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn ipc_scroll_input_event_round_trip() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::InputEvent {
            tab_id: TabId(1),
            event: InputEvent::Scroll {
                delta_x: -5.0,
                delta_y: 12.5,
            },
        });
        match round_trip(&msg) {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::InputEvent {
                event: InputEvent::Scroll { delta_x, delta_y },
                ..
            }) => {
                assert_eq!(delta_x, -5.0);
                assert_eq!(delta_y, 12.5);
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn ipc_reload_and_stop_round_trip() {
        for msg in [
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Reload { tab_id: TabId(1) }),
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Stop { tab_id: TabId(1) }),
        ] {
            match round_trip(&msg) {
                IPCMessage::BrowserToRenderer(BrowserToRenderer::Reload { .. }) => {}
                IPCMessage::BrowserToRenderer(BrowserToRenderer::Stop { .. }) => {}
                other => panic!("unexpected variant: {other:?}"),
            }
        }
    }

    #[test]
    fn ipc_renderer_to_browser_round_trip() {
        let messages = vec![
            IPCMessage::RendererToBrowser(RendererToBrowser::DOMLoaded {
                tab_id: TabId(1),
                title: "Example Domain".to_string(),
                url: "https://example.com".to_string(),
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::LoadProgress {
                tab_id: TabId(1),
                progress: 0.42,
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::NavigateComplete {
                tab_id: TabId(1),
                url: "https://example.com".to_string(),
                title: "Example Domain".to_string(),
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::RequestNavigate {
                tab_id: TabId(1),
                url: "https://rust-lang.org".to_string(),
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::ConsoleMessage {
                tab_id: TabId(1),
                level: LogLevel::Warn,
                text: "deprecated API".to_string(),
            }),
        ];

        for original in messages {
            let decoded = round_trip(&original);
            let original_bytes = bincode::serialize(&original).unwrap();
            let decoded_bytes = bincode::serialize(&decoded).unwrap();
            assert_eq!(
                original_bytes, decoded_bytes,
                "bytes diverged for {original:?}"
            );
        }
    }

    #[test]
    fn ipc_message_corrupt_payload_errors() {
        let bytes = bincode::serialize(&IPCMessage::BrowserToRenderer(BrowserToRenderer::Reload {
            tab_id: TabId(1),
        }))
        .unwrap();
        let truncated = &bytes[..bytes.len() - 1];
        let result: std::result::Result<IPCMessage, _> = bincode::deserialize(truncated);
        assert!(
            result.is_err(),
            "truncated payload must fail to deserialise"
        );
    }

    // ---------- Task 1.5: Error ----------

    #[test]
    fn error_io_from_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing file");
        let err: Error = io_err.into();
        match err {
            Error::Io(_) => {}
            other => panic!("expected Error::Io, got {other:?}"),
        }
        assert!(err.to_string().contains("missing file"));
    }

    #[test]
    fn error_variants_display_distinct_messages() {
        let variants = [
            Error::Ipc("ipc boom".to_string()),
            Error::Parse("parse boom".to_string()),
            Error::Layout("layout boom".to_string()),
            Error::Render("render boom".to_string()),
            Error::Network("network boom".to_string()),
            Error::JavaScript("js boom".to_string()),
            Error::Sandbox("sandbox boom".to_string()),
            Error::Config("config boom".to_string()),
            Error::Crypto("crypto boom".to_string()),
            Error::Serialization("ser boom".to_string()),
        ];

        let mut seen = HashSet::new();
        for variant in &variants {
            let msg = variant.to_string();
            assert!(msg.contains("boom"), "missing payload in {msg}");
            assert!(seen.insert(msg.clone()), "duplicate message {msg}");
        }
        assert_eq!(seen.len(), variants.len());
    }

    #[test]
    fn error_propagates_through_result() {
        fn producer() -> Result<u32> {
            Err(Error::Config("bad field".to_string()))
        }
        fn consumer() -> Result<u32> {
            let _v = producer()?;
            Ok(1)
        }
        match consumer() {
            Err(Error::Config(msg)) => assert_eq!(msg, "bad field"),
            other => panic!("expected Error::Config, got {other:?}"),
        }
    }
}
