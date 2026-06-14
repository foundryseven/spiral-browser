//! Browser tab model.
//!
//! A `Tab` is a renderable surface owned by the browser. Phase 1 keeps each
//! tab's display state in a `TabState` struct; the browser is responsible for
//! producing a `DisplayList` from it on every frame.

use std::time::Instant;

use spiral_core::TabId;

/// Per-tab rendering state.
#[derive(Debug, Clone)]
pub struct TabState {
    /// Tab id (matches `TabId`).
    pub id: TabId,
    /// Current URL.
    pub url: String,
    /// Display title (defaults to the URL's host, or "New Tab").
    pub title: String,
    /// `true` while a navigation is in flight.
    pub loading: bool,
    /// Load progress in `[0.0, 1.0]`.
    pub progress: f32,
    /// When the last `LoadComplete` was received.
    pub loaded_at: Option<Instant>,
    /// Viewport width in logical pixels.
    pub viewport_width: f32,
    /// Viewport height in logical pixels.
    pub viewport_height: f32,
}

impl TabState {
    /// Default viewport for a new tab.
    pub const DEFAULT_VIEWPORT: (f32, f32) = (1024.0, 768.0);

    /// Build a fresh tab for `url` with default viewport.
    pub fn new(id: TabId, url: impl Into<String>) -> Self {
        let url = url.into();
        let title = title_from_url(&url);
        Self {
            id,
            url,
            title,
            loading: true,
            progress: 0.0,
            loaded_at: None,
            viewport_width: Self::DEFAULT_VIEWPORT.0,
            viewport_height: Self::DEFAULT_VIEWPORT.1,
        }
    }

    /// Mark navigation finished.
    pub fn mark_loaded(&mut self) {
        self.loading = false;
        self.progress = 1.0;
        self.loaded_at = Some(Instant::now());
    }

    /// Update progress (clamped to `[0.0, 1.0]`).
    pub fn set_progress(&mut self, p: f32) {
        self.progress = p.clamp(0.0, 1.0);
    }

    /// Update viewport dimensions (clamped to a sane minimum).
    pub fn set_viewport(&mut self, w: f32, h: f32) {
        self.viewport_width = w.max(1.0);
        self.viewport_height = h.max(1.0);
    }
}

/// A registry of every open tab keyed by `TabId`.
#[derive(Debug, Default)]
pub struct TabRegistry {
    tabs: Vec<TabState>,
    active: Option<TabId>,
    next_id: u64,
}

impl TabRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate a new `TabId`.
    pub fn allocate_id(&mut self) -> TabId {
        let id = TabId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Open a tab for `url` and make it active. Returns the new `TabId`.
    pub fn open(&mut self, url: impl Into<String>) -> TabId {
        let id = self.allocate_id();
        let tab = TabState::new(id, url);
        self.tabs.push(tab);
        self.active = Some(id);
        id
    }

    /// Number of open tabs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    /// `true` if there are no open tabs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    /// Lookup a tab by id.
    #[must_use]
    pub fn get(&self, id: TabId) -> Option<&TabState> {
        self.tabs.iter().find(|t| t.id == id)
    }

    /// Mutable lookup by id.
    pub fn get_mut(&mut self, id: TabId) -> Option<&mut TabState> {
        self.tabs.iter_mut().find(|t| t.id == id)
    }

    /// Active tab, if any.
    #[must_use]
    pub fn active(&self) -> Option<&TabState> {
        self.active.and_then(|id| self.get(id))
    }

    /// Mutable active tab, if any.
    pub fn active_mut(&mut self) -> Option<&mut TabState> {
        let id = self.active?;
        self.get_mut(id)
    }

    /// Activate the given tab. Returns `true` if the tab existed.
    pub fn activate(&mut self, id: TabId) -> bool {
        if self.get(id).is_some() {
            self.active = Some(id);
            true
        } else {
            false
        }
    }

    /// Active tab id, if any.
    #[must_use]
    pub fn active_id(&self) -> Option<TabId> {
        self.active
    }
}

fn title_from_url(url: &str) -> String {
    let after_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .or_else(|| url.strip_prefix("file://"))
        .unwrap_or(url);
    let host = after_scheme
        .split('/')
        .next()
        .unwrap_or(after_scheme)
        .split('?')
        .next()
        .unwrap_or(after_scheme);
    if host.is_empty() {
        "New Tab".to_string()
    } else {
        host.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_id_is_unique_and_monotonic() {
        let mut reg = TabRegistry::new();
        let a = reg.allocate_id();
        let b = reg.allocate_id();
        assert_ne!(a, b);
        assert!(b.0 > a.0);
    }

    #[test]
    fn open_makes_tab_active() {
        let mut reg = TabRegistry::new();
        let id = reg.open("https://example.com/");
        assert_eq!(reg.active_id(), Some(id));
        assert_eq!(reg.len(), 1);
        assert!(!reg.is_empty());
    }

    #[test]
    fn title_strips_scheme_and_path() {
        let mut reg = TabRegistry::new();
        let id = reg.open("https://example.com/some/page?x=1");
        assert_eq!(reg.get(id).unwrap().title, "example.com");
    }

    #[test]
    fn title_for_blank_url() {
        let mut reg = TabRegistry::new();
        let id = reg.open("");
        assert_eq!(reg.get(id).unwrap().title, "New Tab");
    }

    #[test]
    fn activate_unknown_tab_returns_false() {
        let mut reg = TabRegistry::new();
        assert!(!reg.activate(TabId(999)));
        assert!(reg.active().is_none());
    }

    #[test]
    fn progress_is_clamped() {
        let mut reg = TabRegistry::new();
        let id = reg.open("https://x.test/");
        let tab = reg.get_mut(id).unwrap();
        tab.set_progress(2.0);
        assert_eq!(tab.progress, 1.0);
        tab.set_progress(-0.5);
        assert_eq!(tab.progress, 0.0);
    }

    #[test]
    fn viewport_is_clamped_to_minimum() {
        let mut reg = TabRegistry::new();
        let id = reg.open("https://x.test/");
        let tab = reg.get_mut(id).unwrap();
        tab.set_viewport(0.0, 0.0);
        assert_eq!(tab.viewport_width, 1.0);
        assert_eq!(tab.viewport_height, 1.0);
    }
}
