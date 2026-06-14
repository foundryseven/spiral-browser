//! IPC event handling for `BrowserShell`.

use log::{info, warn};

use spiral_core::{BrowserToRenderer, HelloMessage, IPCMessage, LogLevel, RendererToBrowser};
use spiral_ipc::IpcTransport;

use crate::tab::TabRegistry;

#[derive(Debug, Clone)]
pub enum ProcessOutcome {
    Idle,
    Reply(Vec<IPCMessage>),
}

#[must_use]
pub fn process_message(registry: &mut TabRegistry, msg: IPCMessage) -> ProcessOutcome {
    match msg {
        IPCMessage::Hello(hello) => handle_hello(registry, hello),
        IPCMessage::RendererToBrowser(ev) => handle_event(registry, ev),
        IPCMessage::BrowserToRenderer(_) => {
            warn!("browser received BrowserToRenderer envelope — ignoring");
            ProcessOutcome::Idle
        }
    }
}

fn handle_hello(registry: &mut TabRegistry, hello: HelloMessage) -> ProcessOutcome {
    if hello.protocol_version != HelloMessage::PROTOCOL_VERSION {
        warn!(
            "renderer protocol version {} != {} — refusing handshake",
            hello.protocol_version,
            HelloMessage::PROTOCOL_VERSION
        );
        return ProcessOutcome::Reply(vec![IPCMessage::BrowserToRenderer(
            BrowserToRenderer::Log {
                level: LogLevel::Warn,
                message: format!(
                    "protocol mismatch: expected {}, got {}",
                    HelloMessage::PROTOCOL_VERSION,
                    hello.protocol_version
                ),
            },
        )]);
    }
    let Some(tab) = registry.get_mut(hello.tab_id) else {
        warn!("renderer handshake for unknown tab {:?}", hello.tab_id);
        return ProcessOutcome::Reply(vec![IPCMessage::BrowserToRenderer(
            BrowserToRenderer::Log {
                level: LogLevel::Error,
                message: format!("unknown tab id {}", hello.tab_id),
            },
        )]);
    };
    tab.set_viewport(hello.viewport_width, hello.viewport_height);
    info!(
        "renderer handshake for tab {} ({}x{})",
        hello.tab_id, hello.viewport_width, hello.viewport_height
    );
    ProcessOutcome::Reply(vec![IPCMessage::BrowserToRenderer(BrowserToRenderer::Resize {
        tab_id: hello.tab_id,
        width: hello.viewport_width,
        height: hello.viewport_height,
    })])
}

fn handle_event(registry: &mut TabRegistry, ev: RendererToBrowser) -> ProcessOutcome {
    match ev {
        RendererToBrowser::RendererReady { tab_id } => {
            info!("renderer ready for tab {tab_id}");
            ProcessOutcome::Idle
        }
        RendererToBrowser::RequestNavigate { tab_id, url } => {
            info!("renderer requests navigate tab {tab_id} -> {url}");
            if let Some(t) = registry.get_mut(tab_id) {
                t.url = url.clone();
                t.loading = true;
                t.progress = 0.0;
                t.loaded_at = None;
            }
            ProcessOutcome::Reply(vec![IPCMessage::BrowserToRenderer(
                BrowserToRenderer::Navigate { tab_id, url },
            )])
        }
        RendererToBrowser::NavigateComplete { tab_id, url, title } => {
            info!("renderer reports navigate complete for tab {tab_id}");
            if let Some(t) = registry.get_mut(tab_id) {
                if !url.is_empty() {
                    t.url = url;
                }
                if !title.is_empty() {
                    t.title = title;
                }
                t.mark_loaded();
            }
            ProcessOutcome::Idle
        }
        RendererToBrowser::LoadProgress { tab_id, progress } => {
            if let Some(t) = registry.get_mut(tab_id) {
                t.set_progress(progress);
            }
            ProcessOutcome::Idle
        }
        RendererToBrowser::DOMLoaded { tab_id, title, url } => {
            if let Some(t) = registry.get_mut(tab_id) {
                if !url.is_empty() {
                    t.url = url;
                }
                if !title.is_empty() {
                    t.title = title;
                }
            }
            ProcessOutcome::Idle
        }
        RendererToBrowser::ConsoleMessage { tab_id: _, level, text } => {
            match level {
                LogLevel::Error => log::error!("[console] {text}"),
                LogLevel::Warn => log::warn!("[console] {text}"),
                LogLevel::Info => log::info!("[console] {text}"),
                LogLevel::Debug => log::debug!("[console] {text}"),
                LogLevel::Trace => log::trace!("[console] {text}"),
            }
            ProcessOutcome::Idle
        }
        RendererToBrowser::Input { tab_id: _, event: _ } => ProcessOutcome::Idle,
        RendererToBrowser::Screenshot { tab_id: _, request_id } => ProcessOutcome::Reply(vec![
            IPCMessage::BrowserToRenderer(BrowserToRenderer::ScreenshotAck { request_id }),
        ]),
    }
}

pub async fn run_event_loop<T: IpcTransport>(
    registry: &mut TabRegistry,
    transport: &mut T,
) -> spiral_core::Result<()> {
    loop {
        let msg = match transport.recv().await {
            Ok(m) => m,
            Err(_) => return Ok(()), // channel closed — clean exit
        };
        let outcome = process_message(registry, msg);
        if let ProcessOutcome::Reply(replies) = outcome {
            for r in replies {
                // If the send fails (receiver dropped), exit cleanly.
                if transport.send(&r).await.is_err() {
                    return Ok(());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_core::{BrowserToRenderer, LogLevel, RendererToBrowser};

    fn hello(tab: u64) -> IPCMessage {
        IPCMessage::Hello(HelloMessage {
            tab_id: spiral_core::TabId(tab),
            protocol_version: HelloMessage::PROTOCOL_VERSION,
            viewport_width: 800.0,
            viewport_height: 600.0,
        })
    }

    #[test]
    fn hello_records_viewport() {
        let mut reg = TabRegistry::new();
        let id = reg.open("https://example.com/");
        let outcome = process_message(&mut reg, hello(id.0));
        match outcome {
            ProcessOutcome::Reply(replies) => {
                assert_eq!(replies.len(), 1);
                assert!(matches!(
                    &replies[0],
                    IPCMessage::BrowserToRenderer(BrowserToRenderer::Resize { .. })
                ));
            }
            other => panic!("expected Reply, got {other:?}"),
        }
        assert_eq!(reg.get(id).unwrap().viewport_width, 800.0);
    }

    #[test]
    fn hello_with_bad_version_warns() {
        let mut reg = TabRegistry::new();
        let id = reg.open("https://example.com/");
        let bad = IPCMessage::Hello(HelloMessage {
            tab_id: id,
            protocol_version: 99,
            viewport_width: 800.0,
            viewport_height: 600.0,
        });
        let outcome = process_message(&mut reg, bad);
        match outcome {
            ProcessOutcome::Reply(replies) => {
                assert!(matches!(
                    &replies[0],
                    IPCMessage::BrowserToRenderer(BrowserToRenderer::Log { level: LogLevel::Warn, .. })
                ));
            }
            other => panic!("expected Reply, got {other:?}"),
        }
    }

    #[test]
    fn navigate_complete_marks_tab_loaded() {
        let mut reg = TabRegistry::new();
        let id = reg.open("https://example.com/");
        let outcome = process_message(
            &mut reg,
            IPCMessage::RendererToBrowser(RendererToBrowser::NavigateComplete {
                tab_id: id,
                url: "https://example.com/done".to_string(),
                title: "Done".to_string(),
            }),
        );
        assert!(matches!(outcome, ProcessOutcome::Idle));
        let tab = reg.get(id).unwrap();
        assert!(!tab.loading);
        assert_eq!(tab.url, "https://example.com/done");
        assert_eq!(tab.title, "Done");
    }

    #[test]
    fn request_navigate_replies_with_navigate() {
        let mut reg = TabRegistry::new();
        let id = reg.open("https://example.com/");
        let outcome = process_message(
            &mut reg,
            IPCMessage::RendererToBrowser(RendererToBrowser::RequestNavigate {
                tab_id: id,
                url: "https://example.com/next".to_string(),
            }),
        );
        match outcome {
            ProcessOutcome::Reply(replies) => {
                assert!(matches!(
                    &replies[0],
                    IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url, .. })
                        if url == "https://example.com/next"
                ));
            }
            other => panic!("expected Reply, got {other:?}"),
        }
    }
}
