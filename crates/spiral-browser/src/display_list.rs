//! Phase 1 "Hello World" display list builder.

use spiral_paint::{DisplayList, RenderOp};
use spiral_render::font;

use crate::tab::TabState;
use crate::theme::BrowserTheme;

/// Default headline shown in the Phase 1 hello-world page.
pub const HELLO_HEADLINE: &str = "Hello, Spiral!";

/// Build the hello-world display list for `tab`, themed with `theme`.
#[must_use]
pub fn build_hello_display_list(tab: &TabState, theme: &BrowserTheme) -> DisplayList {
    let mut list = DisplayList { ops: Vec::new() };
    let w = tab.viewport_width;
    let h = tab.viewport_height;

    list.ops.push(RenderOp::FillRect {
        x: 0.0,
        y: 0.0,
        width: w,
        height: h,
        color: theme.bg_primary,
    });

    let font_size: f32 = 64.0;
    let text_w = font::text_width(HELLO_HEADLINE) as f32 * (font_size / font::GLYPH_HEIGHT as f32);
    let tx = ((w - text_w) * 0.5).max(0.0);
    let ty = ((h - font_size) * 0.5).max(0.0);
    list.ops.push(RenderOp::DrawText {
        x: tx,
        y: ty,
        text: HELLO_HEADLINE.to_string(),
        font_size,
        color: theme.text_primary,
    });

    list.ops.push(RenderOp::FillRect {
        x: tx,
        y: ty + font_size + 4.0,
        width: text_w,
        height: 3.0,
        color: theme.accent,
    });

    let status_text = format!("{} - {}", tab.title, tab.url);
    let status_size: f32 = 16.0;
    let status_y = h - status_size - 8.0;
    list.ops.push(RenderOp::FillRect {
        x: 0.0,
        y: status_y - 6.0,
        width: w,
        height: status_size + 12.0,
        color: theme.bg_secondary,
    });
    list.ops.push(RenderOp::DrawText {
        x: 8.0,
        y: status_y,
        text: status_text,
        font_size: status_size,
        color: theme.text_secondary,
    });

    list
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_core::{AccentColor, BrowserConfig, TabId};
    use spiral_theme::ThemeEngine;

    fn theme() -> BrowserTheme {
        let cfg = BrowserConfig {
            homepage: "https://example.com/".to_string(),
            proxy: None,
            font_size: 16.0,
            accent_color: AccentColor::Indigo,
            dark_mode: false,
            tab_position: spiral_core::TabPosition::Left,
            auto_hide_chrome: false,
            sandbox_renderer: false,
        };
        let engine = ThemeEngine::new(&cfg);
        BrowserTheme::from_engine(&engine)
    }

    #[test]
    fn hello_list_has_five_ops() {
        let tab = TabState::new(TabId(0), "https://example.com/");
        let list = build_hello_display_list(&tab, &theme());
        assert_eq!(list.ops.len(), 5);
    }

    #[test]
    fn background_fills_viewport() {
        let tab = TabState::new(TabId(0), "https://example.com/");
        let list = build_hello_display_list(&tab, &theme());
        match &list.ops[0] {
            RenderOp::FillRect { x, y, width, height, .. } => {
                assert_eq!(*x, 0.0);
                assert_eq!(*y, 0.0);
                assert_eq!(*width, tab.viewport_width);
                assert_eq!(*height, tab.viewport_height);
            }
            other => panic!("expected FillRect, got {other:?}"),
        }
    }

    #[test]
    fn status_includes_url_and_title() {
        let tab = TabState::new(TabId(0), "https://example.com/path");
        let list = build_hello_display_list(&tab, &theme());
        let mut found_status = false;
        for op in &list.ops {
            if let RenderOp::DrawText { text, .. } = op {
                if text.contains("example.com") && text.contains("https://example.com/path") {
                    found_status = true;
                }
            }
        }
        assert!(found_status, "expected status text containing both title and URL");
    }
}
