//! Spiral Browser — IPC Transport Layer
//!
//! Inter-process communication via Unix domain sockets (Linux/macOS)
//! or named pipes (Windows).

use spiral_core::{IPCMessage, Error, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};

/// IPC server that listens for renderer connections.
pub struct IpcServer {
    #[cfg(unix)]
    listener: UnixListener,
}

/// IPC client that connects to the browser process.
pub struct IpcClient {
    #[cfg(unix)]
    stream: UnixStream,
}

/// Message framing: length-prefixed bincode.
fn encode_message(msg: &IPCMessage) -> Result<Vec<u8>> {
    let encoded = bincode::serialize(msg)
        .map_err(|e| Error::Serialization(e.to_string()))?;
    let len = encoded.len() as u32;
    let mut buffer = Vec::with_capacity(4 + encoded.len());
    buffer.extend_from_slice(&len.to_le_bytes());
    buffer.extend_from_slice(&encoded);
    Ok(buffer)
}

fn decode_message(data: &[u8]) -> Result<(IPCMessage, usize)> {
    if data.len() < 4 {
        return Err(Error::Ipc("Message too short".to_string()));
    }
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + len {
        return Err(Error::Ipc("Incomplete message".to_string()));
    }
    let msg: IPCMessage = bincode::deserialize(&data[4..4 + len])
        .map_err(|e| Error::Serialization(e.to_string()))?;
    Ok((msg, 4 + len))
}

#[cfg(unix)]
impl IpcServer {
    /// Create a new IPC server listening on a Unix domain socket.
    pub async fn new(path: &str) -> Result<Self> {
        // Remove old socket if it exists
        let _ = std::fs::remove_file(path);
        let listener = UnixListener::bind(path)
            .map_err(|e| Error::Ipc(format!("Failed to bind: {}", e)))?;
        Ok(Self { listener })
    }

    /// Accept a new connection.
    pub async fn accept(&self) -> Result<IpcClient> {
        let (stream, _addr) = self.listener.accept()
            .await
            .map_err(|e| Error::Ipc(format!("Failed to accept: {}", e)))?;
        Ok(IpcClient { stream })
    }
}

#[cfg(unix)]
impl IpcClient {
    /// Connect to an IPC server.
    pub async fn connect(path: &str) -> Result<Self> {
        let stream = UnixStream::connect(path)
            .await
            .map_err(|e| Error::Ipc(format!("Failed to connect: {}", e)))?;
        Ok(Self { stream })
    }

    /// Send a message.
    pub async fn send(&mut self, msg: &IPCMessage) -> Result<()> {
        let data = encode_message(msg)?;
        self.stream.write_all(&data)
            .await
            .map_err(|e| Error::Ipc(format!("Failed to send: {}", e)))?;
        Ok(())
    }

    /// Receive a message.
    pub async fn recv(&mut self) -> Result<IPCMessage> {
        let mut buffer = vec![0u8; 65536];
        let n = self.stream.read(&mut buffer)
            .await
            .map_err(|e| Error::Ipc(format!("Failed to receive: {}", e)))?;
        if n == 0 {
            return Err(Error::Ipc("Connection closed".to_string()));
        }
        let (msg, _) = decode_message(&buffer[..n])?;
        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_core::{BrowserToRenderer, RendererToBrowser};

    #[test]
    fn test_encode_decode_message() {
        let msg = IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate {
            url: "https://example.com".to_string(),
        });
        let encoded = encode_message(&msg).unwrap();
        let (decoded, consumed) = decode_message(&encoded).unwrap();
        assert_eq!(consumed, encoded.len());
        match decoded {
            IPCMessage::BrowserToRenderer(BrowserToRenderer::Navigate { url }) => {
                assert_eq!(url, "https://example.com");
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_decode_incomplete_message() {
        let data = vec![0, 0, 0, 0];
        let result = decode_message(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_empty_message() {
        let data = vec![];
        let result = decode_message(&data);
        assert!(result.is_err());
    }
}
