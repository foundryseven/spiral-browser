//! Spiral Browser — Main Process
//!
//! Main browser process for the Spiral Browser.

use spiral_core::{BrowserConfig, TabId};
use spiral_theme::ThemeEngine;

/// Browser process.
pub struct BrowserProcess {
    /// Browser configuration.
    config: BrowserConfig,
    /// Theme engine.
    theme: ThemeEngine,
    /// Active tabs.
    tabs: Vec<TabId>,
}

impl BrowserProcess {
    /// Create a new browser process.
    pub fn new(config: BrowserConfig) -> Self {
        let theme = ThemeEngine::new(&config);
        Self {
            config,
            theme,
            tabs: Vec::new(),
        }
    }

    /// Initialize the browser process.
    pub async fn init(&mut self) -> spiral_core::Result<()> {
        log::info!("Initializing Spiral Browser");

        // Initialize theme
        log::info!("Theme mode: {:?}", self.theme.tokens());

        // Phase 2: Initialize IPC server
        // Phase 3: Spawn renderer processes

        Ok(())
    }

    /// Create a new tab.
    pub fn create_tab(&mut self, url: &str) -> TabId {
        let id = TabId(self.tabs.len() as u64);
        self.tabs.push(id);
        log::info!("Created tab {} for {}", id.0, url);
        id
    }

    /// Run the browser.
    pub async fn run(&self) -> spiral_core::Result<()> {
        log::info!("Spiral Browser running");
        log::info!("Configuration: {:?}", self.config);

        // Phase 2: Start IPC server
        // Phase 3: Spawn renderer processes
        // Phase 4: Render UI

        Ok(())
    }
}

#[tokio::main]
async fn main() -> spiral_core::Result<()> {
    // Initialize logger
    env_logger::init();

    // Load configuration
    let config = BrowserConfig::default();

    // Create browser process
    let mut browser = BrowserProcess::new(config);

    // Initialize
    browser.init().await?;

    // Create initial tab
    browser.create_tab("about:blank");

    // Run
    browser.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_browser() {
        let config = BrowserConfig::default();
        let browser = BrowserProcess::new(config);
        assert!(browser.tabs.is_empty());
    }

    #[test]
    fn test_create_tab() {
        let config = BrowserConfig::default();
        let mut browser = BrowserProcess::new(config);
        let id = browser.create_tab("https://example.com");
        assert_eq!(browser.tabs.len(), 1);
        assert_eq!(id.0, 0);
    }
}
