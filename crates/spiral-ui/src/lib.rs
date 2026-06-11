//! Spiral Browser — UI Chrome
//!
//! Browser chrome UI for the Spiral Browser.

use spiral_core::{BrowserConfig, TabId};
use spiral_theme::ThemeEngine;

/// Tab information.
pub struct Tab {
    /// Unique tab ID.
    pub id: TabId,
    /// Tab title.
    pub title: String,
    /// Tab URL.
    pub url: String,
    /// Tab is loading.
    pub loading: bool,
}

/// Browser UI state.
pub struct BrowserUi {
    /// All tabs.
    tabs: Vec<Tab>,
    /// Active tab ID.
    active_tab: Option<TabId>,
    /// URL bar text.
    url_bar: String,
    /// URL bar is focused.
    url_bar_focused: bool,
}

impl BrowserUi {
    /// Create a new browser UI.
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: None,
            url_bar: String::new(),
            url_bar_focused: false,
        }
    }

    /// Add a new tab.
    pub fn add_tab(&mut self, url: &str) -> TabId {
        let id = TabId(self.tabs.len() as u64);
        let tab = Tab {
            id,
            title: "New Tab".to_string(),
            url: url.to_string(),
            loading: false,
        };
        self.tabs.push(tab);
        self.active_tab = Some(id);
        id
    }

    /// Close a tab.
    pub fn close_tab(&mut self, id: TabId) {
        self.tabs.retain(|t| t.id != id);
        if self.active_tab == Some(id) {
            self.active_tab = self.tabs.first().map(|t| t.id);
        }
    }

    /// Get the active tab.
    pub fn active_tab(&self) -> Option<&Tab> {
        self.active_tab.and_then(|id| self.tabs.iter().find(|t| t.id == id))
    }

    /// Get all tabs.
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Set URL bar text.
    pub fn set_url_bar(&mut self, url: &str) {
        self.url_bar = url.to_string();
    }

    /// Get URL bar text.
    pub fn url_bar(&self) -> &str {
        &self.url_bar
    }
}

impl Default for BrowserUi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ui() {
        let ui = BrowserUi::new();
        assert!(ui.tabs().is_empty());
    }

    #[test]
    fn test_add_tab() {
        let mut ui = BrowserUi::new();
        let id = ui.add_tab("https://example.com");
        assert_eq!(ui.tabs().len(), 1);
        assert_eq!(ui.active_tab().unwrap().url, "https://example.com");
    }

    #[test]
    fn test_close_tab() {
        let mut ui = BrowserUi::new();
        let id = ui.add_tab("https://example.com");
        ui.close_tab(id);
        assert!(ui.tabs().is_empty());
    }

    #[test]
    fn test_url_bar() {
        let mut ui = BrowserUi::new();
        ui.set_url_bar("https://example.com");
        assert_eq!(ui.url_bar(), "https://example.com");
    }
}
