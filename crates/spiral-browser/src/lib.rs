//! Spiral Browser — Main Process
//!
//! Library entry point exposing the `BrowserShell` and the Phase 1 hello-world
//! display list. The binary in `main.rs` is a thin wrapper that initialises
//! logging, parses CLI args, and runs the shell.

pub mod display_list;
pub mod event_loop;
pub mod shell;
pub mod tab;
pub mod theme;

pub use display_list::{build_hello_display_list, HELLO_HEADLINE};
pub use event_loop::{process_message, run_event_loop, ProcessOutcome};
pub use shell::{BrowserShell, RenderError, RenderResult, DEFAULT_RENDER_PATH};
pub use tab::{TabRegistry, TabState};
pub use theme::BrowserTheme;
