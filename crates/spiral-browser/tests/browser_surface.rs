//! Integration tests for the `spiral-browser` public surface.

use spiral_browser::{
    BrowserShell, BrowserTheme, ProcessOutcome, RenderResult, TabRegistry, TabState,
    DEFAULT_RENDER_PATH, HELLO_HEADLINE,
};

#[test]
fn test_browser_surface_wiring() {
    let _shell: Option<BrowserShell> = None;
    let _theme: Option<BrowserTheme> = None;
    let _registry: Option<TabRegistry> = None;
    let _state: Option<TabState> = None;
    let _outcome: Option<ProcessOutcome> = None;
    let _result: Option<RenderResult<()>> = None;
    let _path = DEFAULT_RENDER_PATH;
    let _headline = HELLO_HEADLINE;
}
