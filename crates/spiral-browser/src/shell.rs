//! Top-level browser shell.

use std::path::{Path, PathBuf};

use log::info;
use thiserror::Error;

use spiral_core::{BrowserConfig, TabId};
use spiral_ipc::IpcTransport;
use spiral_paint::DisplayList;
use spiral_render::{encode_png, SoftwareRenderer};
use spiral_theme::ThemeEngine;

use crate::display_list::build_hello_display_list;
use crate::event_loop::run_event_loop;
use crate::tab::TabRegistry;
use crate::theme::BrowserTheme;

pub const DEFAULT_RENDER_PATH: &str = "target/hello-world.png";

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("no active tab to render")]
    NoActiveTab,
    #[error("renderer error: {0}")]
    Renderer(#[from] spiral_render::RenderError),
    #[error("png encoder error: {0}")]
    Png(#[from] spiral_render::PngError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type RenderResult<T> = std::result::Result<T, RenderError>;

pub struct BrowserShell {
    config: BrowserConfig,
    theme_engine: ThemeEngine,
    registry: TabRegistry,
}

impl BrowserShell {
    pub fn new(config: BrowserConfig) -> Self {
        let theme_engine = ThemeEngine::new(&config);
        let mut registry = TabRegistry::new();
        registry.open(config.homepage.clone());
        Self {
            config,
            theme_engine,
            registry,
        }
    }

    #[must_use]
    pub fn config(&self) -> &BrowserConfig {
        &self.config
    }

    #[must_use]
    pub fn theme_engine(&self) -> &ThemeEngine {
        &self.theme_engine
    }

    #[must_use]
    pub fn theme(&self) -> BrowserTheme {
        BrowserTheme::from_engine(&self.theme_engine)
    }

    #[must_use]
    pub fn registry(&self) -> &TabRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut TabRegistry {
        &mut self.registry
    }

    pub fn open_tab(&mut self, url: impl Into<String>) -> TabId {
        self.registry.open(url)
    }

    pub fn activate_tab(&mut self, id: TabId) -> bool {
        self.registry.activate(id)
    }

    pub fn init(&mut self) {
        info!(
            "Spiral Browser initialised — homepage = {}",
            self.config.homepage
        );
    }

    pub async fn run<T: IpcTransport>(&mut self, transport: &mut T) -> spiral_core::Result<()> {
        run_event_loop(&mut self.registry, transport).await
    }

    pub fn display_list(&self) -> RenderResult<DisplayList> {
        let tab = self.registry.active().ok_or(RenderError::NoActiveTab)?;
        Ok(build_hello_display_list(tab, &self.theme()))
    }

    pub fn render_active_tab(&self) -> RenderResult<(u32, u32, Vec<u8>)> {
        let list = self.display_list()?;
        let tab = self.registry.active().ok_or(RenderError::NoActiveTab)?;
        let w = tab.viewport_width.max(1.0) as u32;
        let h = tab.viewport_height.max(1.0) as u32;
        let bg = self.theme().bg_primary;
        let bg_rgba = [bg.r, bg.g, bg.b, (bg.a.clamp(0.0, 1.0) * 255.0) as u8];
        let mut renderer = SoftwareRenderer::new(w, h, bg_rgba)?;
        renderer.draw(&list)?;
        let png = encode_png(&renderer)?;
        Ok((w, h, png))
    }

    pub fn render_active_tab_to(&self, path: impl AsRef<Path>) -> RenderResult<PathBuf> {
        let (w, h, png) = self.render_active_tab()?;
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, &png)?;
        info!("rendered {w}x{h} hello-world frame to {}", path.display());
        Ok(path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TabState;
    use spiral_core::{AccentColor, TabPosition};
    use spiral_ipc::MockTransport;

    fn config() -> BrowserConfig {
        BrowserConfig {
            homepage: "https://example.com/".to_string(),
            proxy: None,
            font_size: 16.0,
            accent_color: AccentColor::Indigo,
            dark_mode: false,
            tab_position: TabPosition::Left,
            auto_hide_chrome: false,
            sandbox_renderer: false,
        }
    }

    #[test]
    fn new_opens_homepage_tab() {
        let shell = BrowserShell::new(config());
        assert_eq!(shell.registry().len(), 1);
        let tab = shell.registry().active().unwrap();
        assert_eq!(tab.url, "https://example.com/");
    }

    #[test]
    fn open_tab_makes_it_active() {
        let mut shell = BrowserShell::new(config());
        let id = shell.open_tab("https://other.test/");
        assert_eq!(shell.registry().active_id(), Some(id));
        assert_eq!(shell.registry().len(), 2);
    }

    #[test]
    fn display_list_is_non_empty() {
        let shell = BrowserShell::new(config());
        let list = shell.display_list().unwrap();
        assert!(!list.ops.is_empty());
    }

    #[test]
    fn render_active_tab_produces_png_matching_viewport() {
        let shell = BrowserShell::new(config());
        let (w, h, png) = shell.render_active_tab().unwrap();
        let (vw, vh) = TabState::DEFAULT_VIEWPORT;
        assert_eq!(w, vw as u32);
        assert_eq!(h, vh as u32);
        assert_eq!(
            &png[0..8],
            &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
        );
    }

    #[test]
    fn render_active_tab_writes_file() {
        let shell = BrowserShell::new(config());
        let path = std::env::temp_dir().join("spiral-hello-test.png");
        let _ = std::fs::remove_file(&path);
        let returned = shell.render_active_tab_to(&path).unwrap();
        assert_eq!(returned, path);
        let bytes = std::fs::read(&path).unwrap();
        assert!(!bytes.is_empty());
        assert_eq!(
            &bytes[0..8],
            &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
        );
    }

    #[tokio::test]
    async fn run_drains_mock_transport() {
        let mut shell = BrowserShell::new(config());
        let tab_id = shell.registry().active_id().unwrap();
        let (mut browser_side, mut renderer_side) = MockTransport::pair();

        renderer_side
            .send(&spiral_core::IPCMessage::Hello(spiral_core::HelloMessage {
                tab_id,
                protocol_version: spiral_core::HelloMessage::PROTOCOL_VERSION,
                viewport_width: 640.0,
                viewport_height: 480.0,
            }))
            .await
            .unwrap();
        renderer_side
            .send(&spiral_core::IPCMessage::RendererToBrowser(
                spiral_core::RendererToBrowser::NavigateComplete {
                    tab_id,
                    url: "https://example.com/".to_string(),
                    title: "Example".to_string(),
                },
            ))
            .await
            .unwrap();

        // Dropping renderer_side closes its senders, causing browser_side's
        // recv to return Err ("Channel closed"). The event loop sees this as
        // a clean exit.
        drop(renderer_side);
        shell.run(&mut browser_side).await.unwrap();
    }
}
