//! Spiral Browser — IPC Transport Layer
//!
//! Inter-process communication via Unix domain sockets (Linux/macOS)
//! or named pipes (Windows). Provides a transport-agnostic `IpcTransport`
//! trait, length-prefixed bincode framing, and platform-native backends.

use std::future::Future;
use std::pin::Pin;

use log::debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use spiral_core::{Error, IPCMessage, Result};

// ---------------------------------------------------------------------------
// Framing: length-prefixed bincode (u32 LE header + payload)
// ---------------------------------------------------------------------------

/// Encode an `IPCMessage` into a length-prefixed frame.
pub fn encode_message(msg: &IPCMessage) -> Result<Vec<u8>> {
    let payload = bincode::serialize(msg).map_err(|e| Error::Serialization(e.to_string()))?;
    let len = payload.len() as u32;
    let mut buf = Vec::with_capacity(4 + payload.len());
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(&payload);
    Ok(buf)
}

/// Decode a single `IPCMessage` from a length-prefixed frame.
///
/// Returns the decoded message and the total number of bytes consumed
/// (4-byte header + payload).
pub fn decode_message(data: &[u8]) -> Result<(IPCMessage, usize)> {
    if data.len() < 4 {
        return Err(Error::Ipc("Frame too short (< 4 bytes)".to_string()));
    }
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + len {
        return Err(Error::Ipc(format!(
            "Incomplete payload: expected {} bytes, got {}",
            len,
            data.len() - 4,
        )));
    }
    let msg =
        bincode::deserialize(&data[4..4 + len]).map_err(|e| Error::Serialization(e.to_string()))?;
    Ok((msg, 4 + len))
}

/// Read exactly `n` bytes from an async reader.
async fn read_exact<R: AsyncReadExt + Unpin>(reader: &mut R, n: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; n];
    reader
        .read_exact(&mut buf)
        .await
        .map_err(|e| Error::Ipc(format!("Read failed: {e}")))?;
    Ok(buf)
}

// ---------------------------------------------------------------------------
// IpcTransport trait
// ---------------------------------------------------------------------------

/// Transport-agnostic interface for sending and receiving `IPCMessage`s.
///
/// Implementations must handle length-prefixed framing internally so that
/// callers deal only with high-level messages.
pub trait IpcTransport: Send {
    fn send<'a>(
        &'a mut self,
        msg: &'a IPCMessage,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

    fn recv(&mut self) -> Pin<Box<dyn Future<Output = Result<IPCMessage>> + Send + '_>>;

    fn close(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
}

// ---------------------------------------------------------------------------
// Unix domain socket transport (Linux / macOS)
// ---------------------------------------------------------------------------

#[cfg(unix)]
pub mod unix {
    use super::*;
    use tokio::net::{UnixListener as TokioUnixListener, UnixStream};

    /// Listens on a Unix domain socket for incoming IPC connections.
    pub struct UnixListener {
        listener: TokioUnixListener,
        path: String,
    }

    impl UnixListener {
        /// Bind to `path`, removing any stale socket file first.
        pub fn bind(path: &str) -> Result<Self> {
            let _ = std::fs::remove_file(path);
            let listener = TokioUnixListener::bind(path)
                .map_err(|e| Error::Ipc(format!("Bind failed for {path}: {e}")))?;
            debug!("Unix IPC listener bound to {path}");
            Ok(Self {
                listener,
                path: path.to_string(),
            })
        }

        /// Accept the next incoming connection.
        pub async fn accept(&self) -> Result<UnixTransport> {
            let (stream, _addr) = self
                .listener
                .accept()
                .await
                .map_err(|e| Error::Ipc(format!("Accept failed: {e}")))?;
            debug!("Accepted Unix IPC connection");
            Ok(UnixTransport { stream })
        }

        /// Return the socket path.
        pub fn path(&self) -> &str {
            &self.path
        }
    }

    impl Drop for UnixListener {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    /// Bidirectional IPC transport over a Unix domain socket.
    pub struct UnixTransport {
        stream: UnixStream,
    }

    impl UnixTransport {
        /// Connect to a Unix domain socket at `path`.
        pub async fn connect(path: &str) -> Result<Self> {
            let stream = UnixStream::connect(path)
                .await
                .map_err(|e| Error::Ipc(format!("Connect failed for {path}: {e}")))?;
            debug!("Connected to Unix IPC server at {path}");
            Ok(Self { stream })
        }
    }

    impl IpcTransport for UnixTransport {
        fn send<'a>(
            &'a mut self,
            msg: &'a IPCMessage,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
            Box::pin(async move {
                let frame = encode_message(msg)?;
                self.stream
                    .write_all(&frame)
                    .await
                    .map_err(|e| Error::Ipc(format!("Send failed: {e}")))?;
                Ok(())
            })
        }

        fn recv(&mut self) -> Pin<Box<dyn Future<Output = Result<IPCMessage>> + Send + '_>> {
            Box::pin(async move {
                let header = read_exact(&mut self.stream, 4).await?;
                let len = u32::from_le_bytes([header[0], header[1], header[2], header[3]]) as usize;
                if len > 64 * 1024 * 1024 {
                    return Err(Error::Ipc(format!(
                        "Frame too large: {len} bytes (max 64 MiB)"
                    )));
                }
                let payload = read_exact(&mut self.stream, len).await?;
                let msg = bincode::deserialize(&payload)
                    .map_err(|e| Error::Serialization(e.to_string()))?;
                Ok(msg)
            })
        }

        fn close(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
            Box::pin(async move {
                // Best-effort shutdown — ignore "not connected" errors.
                let _ = self.stream.shutdown().await;
                Ok(())
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Named pipe transport (Windows)
// ---------------------------------------------------------------------------

#[cfg(windows)]
pub mod pipe {
    use super::*;
    use tokio::net::windows::named_pipe::{NamedPipeClient, NamedPipeServer, ServerOptions};

    /// Listens on a Windows named pipe for incoming IPC connections.
    pub struct PipeListener {
        pipe_name: String,
    }

    impl PipeListener {
        /// Create a named pipe listener on `pipe_name` (e.g. `\\.\pipe\spiral-ipc`).
        pub fn bind(pipe_name: &str) -> Result<Self> {
            debug!("Named pipe IPC listener registered for {pipe_name}");
            Ok(Self {
                pipe_name: pipe_name.to_string(),
            })
        }

        /// Accept the next incoming connection.
        pub async fn accept(&self) -> Result<PipeTransport> {
            let server = ServerOptions::new()
                .first_pipe_instance(false)
                .create(&self.pipe_name)
                .map_err(|e| Error::Ipc(format!("Pipe create failed: {e}")))?;
            server
                .connect()
                .await
                .map_err(|e| Error::Ipc(format!("Pipe connect failed: {e}")))?;
            debug!("Accepted named pipe IPC connection");
            Ok(PipeTransport { pipe: server })
        }

        pub fn pipe_name(&self) -> &str {
            &self.pipe_name
        }
    }

    /// Bidirectional IPC transport over a Windows named pipe.
    pub struct PipeTransport {
        pipe: NamedPipeServer,
    }

    impl PipeTransport {
        /// Connect to a named pipe at `pipe_name`.
        pub async fn connect(pipe_name: &str) -> Result<Self> {
            let client = NamedPipeClient::connect(pipe_name)
                .await
                .map_err(|e| Error::Ipc(format!("Pipe connect failed: {e}")))?;
            debug!("Connected to named pipe IPC server at {pipe_name}");
            Ok(Self { pipe: client })
        }
    }

    impl IpcTransport for PipeTransport {
        fn send<'a>(
            &'a mut self,
            msg: &'a IPCMessage,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
            Box::pin(async move {
                let frame = encode_message(msg)?;
                self.pipe
                    .write_all(&frame)
                    .await
                    .map_err(|e| Error::Ipc(format!("Send failed: {e}")))?;
                Ok(())
            })
        }

        fn recv(&mut self) -> Pin<Box<dyn Future<Output = Result<IPCMessage>> + Send + '_>> {
            Box::pin(async move {
                let header = read_exact(&mut self.pipe, 4).await?;
                let len = u32::from_le_bytes([header[0], header[1], header[2], header[3]]) as usize;
                if len > 64 * 1024 * 1024 {
                    return Err(Error::Ipc(format!(
                        "Frame too large: {len} bytes (max 64 MiB)"
                    )));
                }
                let payload = read_exact(&mut self.pipe, len).await?;
                let msg = bincode::deserialize(&payload)
                    .map_err(|e| Error::Serialization(e.to_string()))?;
                Ok(msg)
            })
        }

        fn close(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
            Box::pin(async move {
                self.pipe
                    .shutdown()
                    .await
                    .map_err(|e| Error::Ipc(format!("Shutdown failed: {e}")))?;
                Ok(())
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Mock transport (for testing)
// ---------------------------------------------------------------------------

/// In-memory transport backed by tokio MPSC channels.
///
/// Create a connected pair with `MockTransport::pair()`.
pub struct MockTransport {
    tx: tokio::sync::mpsc::Sender<Vec<u8>>,
    rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
}

impl MockTransport {
    /// Channel buffer capacity for `pair()`.
    const PAIR_CAPACITY: usize = 64;

    /// Create a connected pair of mock transports.
    ///
    /// Messages sent on one side are received on the other.
    pub fn pair() -> (MockTransport, MockTransport) {
        let (tx_a, rx_b) = tokio::sync::mpsc::channel(Self::PAIR_CAPACITY);
        let (tx_b, rx_a) = tokio::sync::mpsc::channel(Self::PAIR_CAPACITY);
        (
            MockTransport { tx: tx_a, rx: rx_a },
            MockTransport { tx: tx_b, rx: rx_b },
        )
    }
}

impl IpcTransport for MockTransport {
    fn send<'a>(
        &'a mut self,
        msg: &'a IPCMessage,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let frame = encode_message(msg)?;
            self.tx
                .send(frame)
                .await
                .map_err(|_| Error::Ipc("Channel closed".to_string()))
        })
    }

    fn recv(&mut self) -> Pin<Box<dyn Future<Output = Result<IPCMessage>> + Send + '_>> {
        Box::pin(async move {
            let frame = self
                .rx
                .recv()
                .await
                .ok_or_else(|| Error::Ipc("Channel closed".to_string()))?;
            let (msg, _) = decode_message(&frame)?;
            Ok(msg)
        })
    }

    fn close(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move {
            // Dropping the sender signals closure to the remote side.
            // We can't drop self.tx in-place, but we can close it.
            self.tx.closed().await;
            Ok(())
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_core::{BrowserToRenderer, InputEvent, LogLevel, RendererToBrowser, TabId};

    const TAB: TabId = TabId(1);

    // -- Framing unit tests (task 2.3) ------------------------------------

    #[test]
    fn encode_decode_round_trip() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
            tab_id: TAB,
            url: "https://example.com".to_string(),
        });
        let frame = encode_message(&msg).unwrap();
        let (decoded, consumed) = decode_message(&frame).unwrap();
        assert_eq!(consumed, frame.len());
        match decoded {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url, .. }) => {
                assert_eq!(url, "https://example.com");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn encode_decode_large_payload() {
        let large_url = "https://example.com/".to_string() + &"a".repeat(100_000);
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
            tab_id: TAB,
            url: large_url.clone(),
        });
        let frame = encode_message(&msg).unwrap();
        assert!(frame.len() > 100_000);
        let (decoded, _) = decode_message(&frame).unwrap();
        match decoded {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url, .. }) => {
                assert_eq!(url, large_url);
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn decode_frame_too_short() {
        let result = decode_message(&[0, 0]);
        assert!(result.is_err());
    }

    #[test]
    fn decode_incomplete_payload() {
        // Header says 100 bytes but only 2 bytes of payload follow
        let mut frame = vec![100, 0, 0, 0];
        frame.extend_from_slice(&[0xAA, 0xBB]);
        let result = decode_message(&frame);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().contains("Incomplete"),
            "expected incomplete payload error"
        );
    }

    #[test]
    fn decode_zero_length_payload() {
        // Valid header but zero-length payload should fail to deserialise
        let frame = [0, 0, 0, 0];
        let result = decode_message(&frame);
        assert!(result.is_err());
    }

    #[test]
    fn decode_truncated_header() {
        let result = decode_message(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn framing_consumed_bytes_match() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Reload { tab_id: TAB });
        let frame = encode_message(&msg).unwrap();
        let (_, consumed) = decode_message(&frame).unwrap();
        assert_eq!(consumed, frame.len());
    }

    // -- MockTransport echo tests (task 2.4) ------------------------------

    async fn echo_through_transport(
        client: &mut impl IpcTransport,
        server: &mut impl IpcTransport,
    ) {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
            tab_id: TAB,
            url: "https://spiral-browser.example".to_string(),
        });
        client.send(&msg).await.unwrap();
        let received = server.recv().await.unwrap();
        match received {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url, .. }) => {
                assert_eq!(url, "https://spiral-browser.example");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[tokio::test]
    async fn mock_echo_navigate() {
        let (mut client, mut server) = MockTransport::pair();
        echo_through_transport(&mut client, &mut server).await;
    }

    async fn round_trip_various_messages(
        client: &mut impl IpcTransport,
        server: &mut impl IpcTransport,
    ) {
        let messages = vec![
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
                tab_id: TAB,
                url: "https://one.example".to_string(),
            }),
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Reload { tab_id: TAB }),
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Stop { tab_id: TAB }),
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Resize {
                tab_id: TAB,
                width: 1920.0,
                height: 1080.0,
            }),
            IPCMessage::BrowserToRenderer(BrowserToRenderer::InputEvent {
                tab_id: TAB,
                event: InputEvent::MouseDown {
                    x: 100.0,
                    y: 200.0,
                    button: spiral_core::MouseButton::Right,
                },
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::DOMLoaded {
                tab_id: TAB,
                title: "Test Page".to_string(),
                url: "https://test.example".to_string(),
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::LoadProgress {
                tab_id: TAB,
                progress: 0.75,
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::NavigateComplete {
                tab_id: TAB,
                url: "https://done.example".to_string(),
                title: "Done Page".to_string(),
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::RequestNavigate {
                tab_id: TAB,
                url: "https://click.example".to_string(),
            }),
            IPCMessage::RendererToBrowser(RendererToBrowser::ConsoleMessage {
                tab_id: TAB,
                level: LogLevel::Error,
                text: "something broke".to_string(),
            }),
        ];

        for original in &messages {
            client.send(original).await.unwrap();
        }
        for original in &messages {
            let received = server.recv().await.unwrap();
            let orig_bytes = bincode::serialize(original).unwrap();
            let recv_bytes = bincode::serialize(&received).unwrap();
            assert_eq!(orig_bytes, recv_bytes);
        }
    }

    #[tokio::test]
    async fn mock_round_trip_all_variants() {
        let (mut client, mut server) = MockTransport::pair();
        round_trip_various_messages(&mut client, &mut server).await;
    }

    #[tokio::test]
    async fn mock_bidirectional_echo() {
        let (mut side_a, mut side_b) = MockTransport::pair();

        let msg_a = IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
            tab_id: TAB,
            url: "a->b".to_string(),
        });
        let msg_b = IPCMessage::RendererToBrowser(RendererToBrowser::DOMLoaded {
            tab_id: TAB,
            title: "b->a".to_string(),
            url: "https://b.example".to_string(),
        });

        side_a.send(&msg_a).await.unwrap();
        side_b.send(&msg_b).await.unwrap();

        let recv_b = side_b.recv().await.unwrap();
        let recv_a = side_a.recv().await.unwrap();

        match recv_b {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url, .. }) => {
                assert_eq!(url, "a->b");
            }
            other => panic!("unexpected: {other:?}"),
        }
        match recv_a {
            IPCMessage::RendererToBrowser(RendererToBrowser::DOMLoaded { title, .. }) => {
                assert_eq!(title, "b->a");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    // -- MockTransport channel lifecycle (task 2.5) -----------------------

    #[tokio::test]
    async fn mock_recv_after_sender_drop_returns_error() {
        let (client, server) = MockTransport::pair();
        drop(client);
        let mut server = server;
        let result = server.recv().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn mock_multiple_messages_in_order() {
        let (mut client, mut server) = MockTransport::pair();

        for i in 0..50u64 {
            let msg = IPCMessage::RendererToBrowser(RendererToBrowser::LoadProgress {
                tab_id: TAB,
                progress: i as f32 / 50.0,
            });
            client.send(&msg).await.unwrap();
        }

        for i in 0..50u64 {
            let msg = server.recv().await.unwrap();
            match msg {
                IPCMessage::RendererToBrowser(RendererToBrowser::LoadProgress { progress, .. }) => {
                    let expected = i as f32 / 50.0;
                    assert!(
                        (progress - expected).abs() < f32::EPSILON,
                        "progress mismatch at index {i}: {progress} != {expected}"
                    );
                }
                other => panic!("unexpected: {other:?}"),
            }
        }
    }

    // -- Unix transport integration test (task 2.1) -----------------------

    #[cfg(unix)]
    #[tokio::test]
    async fn unix_echo_round_trip() {
        use std::time::Duration;
        let socket_path = format!("/tmp/spiral-ipc-test-{}", std::process::id());
        let listener = unix::UnixListener::bind(&socket_path).unwrap();

        let handle = tokio::spawn(async move {
            let mut server = listener.accept().await.unwrap();
            let msg = server.recv().await.unwrap();
            server.send(&msg).await.unwrap();
            server.close().await.unwrap();
        });

        let mut client = unix::UnixTransport::connect(&socket_path).await.unwrap();
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
            tab_id: TAB,
            url: "https://unix-socket-test.example".to_string(),
        });
        client.send(&msg).await.unwrap();
        let reply = tokio::time::timeout(Duration::from_secs(5), client.recv())
            .await
            .expect("timeout waiting for reply")
            .expect("recv failed");
        match reply {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url, .. }) => {
                assert_eq!(url, "https://unix-socket-test.example");
            }
            other => panic!("unexpected: {other:?}"),
        }
        client.close().await.unwrap();
        handle.await.unwrap();
    }

    // -- Integration: spiral-core types through transport (task 2.7) ------

    #[tokio::test]
    async fn core_types_through_mock_transport() {
        let (mut browser, mut renderer) = MockTransport::pair();

        // Browser sends navigate
        browser
            .send(&IPCMessage::BrowserToRenderer(
                BrowserToRenderer::Navigate {
                    tab_id: TAB,
                    url: "https://spiral.example".to_string(),
                },
            ))
            .await
            .unwrap();

        // Renderer receives and acknowledges with DOMLoaded
        let nav = renderer.recv().await.unwrap();
        match &nav {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url, .. }) => {
                assert_eq!(url, "https://spiral.example");
            }
            other => panic!("unexpected: {other:?}"),
        }

        renderer
            .send(&IPCMessage::RendererToBrowser(
                RendererToBrowser::DOMLoaded {
                    tab_id: TAB,
                    title: "Spiral".to_string(),
                    url: "https://spiral.example".to_string(),
                },
            ))
            .await
            .unwrap();

        browser
            .send(&IPCMessage::BrowserToRenderer(BrowserToRenderer::Resize {
                tab_id: TAB,
                width: 1280.0,
                height: 720.0,
            }))
            .await
            .unwrap();

        renderer
            .send(&IPCMessage::RendererToBrowser(
                RendererToBrowser::LoadProgress {
                    tab_id: TAB,
                    progress: 1.0,
                },
            ))
            .await
            .unwrap();

        renderer
            .send(&IPCMessage::RendererToBrowser(
                RendererToBrowser::NavigateComplete {
                    tab_id: TAB,
                    url: "https://spiral.example".to_string(),
                    title: "Spiral".to_string(),
                },
            ))
            .await
            .unwrap();

        // Verify all messages arrive in order
        let dom_loaded = browser.recv().await.unwrap();
        assert!(matches!(
            dom_loaded,
            IPCMessage::RendererToBrowser(RendererToBrowser::DOMLoaded { .. })
        ));

        let resize = renderer.recv().await.unwrap();
        assert!(matches!(
            resize,
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Resize { .. })
        ));

        let progress = browser.recv().await.unwrap();
        assert!(matches!(
            progress,
            IPCMessage::RendererToBrowser(RendererToBrowser::LoadProgress { .. })
        ));

        let nav_complete = browser.recv().await.unwrap();
        assert!(matches!(
            nav_complete,
            IPCMessage::RendererToBrowser(RendererToBrowser::NavigateComplete { .. })
        ));
    }

    // -- Fuzz smoke test (task 2.6) --------------------------------------
    // Exercises the decoder with many structured malformed inputs to ensure
    // it never panics — only returns Err.

    #[test]
    fn fuzz_smoke_decode_never_panics() {
        let inputs: Vec<Vec<u8>> = vec![
            // empty
            vec![],
            // truncated header
            vec![0],
            vec![0, 0],
            vec![0, 0, 0],
            // header claims huge payload
            vec![0xFF, 0xFF, 0xFF, 0xFF],
            // header claims 1 byte, no payload
            vec![1, 0, 0, 0],
            // header claims 0 bytes, valid header
            vec![0, 0, 0, 0],
            // header + truncated payload
            vec![10, 0, 0, 0, 1, 2, 3],
            // valid header (4 bytes) + garbage payload
            vec![4, 0, 0, 0, 0xFF, 0xFE, 0xFD, 0xFC],
            // valid header + random-ish data
            vec![8, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8],
            // multiple frames concatenated
            {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Reload { tab_id: TAB });
                let frame = encode_message(&msg).unwrap();
                let mut multi = frame.clone();
                multi.extend_from_slice(&frame);
                multi.extend_from_slice(&[0, 0, 0, 0]); // garbage frame
                multi
            },
        ];

        for (i, input) in inputs.iter().enumerate() {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _ = decode_message(input);
            }));
            assert!(
                result.is_ok(),
                "decode_message panicked on input #{i}: {input:02X?}"
            );
        }
    }

    #[test]
    fn fuzz_smoke_decode_random_u8_permutations() {
        // Try all single-byte values as a header[0] with known-good trailing bytes
        for b in 0..=255u8 {
            let input = vec![b, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD];
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _ = decode_message(&input);
            }));
            assert!(
                result.is_ok(),
                "decode_message panicked on [{b:#04x}, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD]"
            );
        }
    }
}
