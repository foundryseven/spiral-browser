//! Spiral Browser — Main Process
//!
//! Binary entry point. Initialises logging, builds a default `BrowserConfig`,
//! constructs the `BrowserShell`, renders the Phase 1 hello-world frame, and
//! prints the output path.
//!
//! Phase 1 has no real IPC transport and no windowing; the binary is
//! intentionally headless so it can run in CI. Phase 2 will add a
//! `winit`/`softbuffer` window loop and a `unix::UnixListener` IPC server.

use std::path::PathBuf;

use spiral_browser::{BrowserShell, DEFAULT_RENDER_PATH, HELLO_HEADLINE};
use spiral_core::BrowserConfig;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = BrowserConfig::default();
    let mut shell = BrowserShell::new(config);
    shell.init();

    let out_path = PathBuf::from(DEFAULT_RENDER_PATH);
    shell.render_active_tab_to(&out_path)?;

    let tab = shell.registry().active().expect("homepage tab always open");
    println!(
        "Spiral Browser Phase 1 — rendered \"{HELLO_HEADLINE}\" for tab {} ({}) at {}",
        tab.id,
        tab.url,
        out_path.display()
    );

    Ok(())
}
