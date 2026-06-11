//! Spiral Browser — HTTP Client
//!
//! HTTP client and networking for the Spiral Browser.

use spiral_core::{Error, Result};
use std::collections::HashMap;

/// HTTP response.
pub struct HttpResponse {
    /// Status code.
    pub status: u16,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Response body.
    pub body: Vec<u8>,
}

/// HTTP client.
pub struct HttpClient {
    /// Client is initialized.
    initialized: bool,
}

impl HttpClient {
    /// Create a new HTTP client.
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    /// Initialize the client.
    pub fn init(&mut self) -> Result<()> {
        // Phase 1: Basic setup
        // Phase 2: hyper integration
        self.initialized = true;
        log::info!("HTTP client initialized");
        Ok(())
    }

    /// Perform a GET request.
    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        if !self.initialized {
            return Err(Error::Network("Client not initialized".to_string()));
        }

        // Phase 1: Placeholder response
        // Phase 2: hyper GET request
        log::trace!("GET {}", url);
        Ok(HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
        })
    }

    /// Perform a POST request.
    pub async fn post(&self, url: &str, body: &[u8]) -> Result<HttpResponse> {
        if !self.initialized {
            return Err(Error::Network("Client not initialized".to_string()));
        }

        // Phase 1: Placeholder response
        // Phase 2: hyper POST request
        log::trace!("POST {} ({} bytes)", url, body.len());
        Ok(HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
        })
    }

    /// Check if client is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_client() {
        let client = HttpClient::new();
        assert!(!client.is_initialized());
    }

    #[tokio::test]
    async fn test_init_client() {
        let mut client = HttpClient::new();
        client.init().unwrap();
        assert!(client.is_initialized());
    }

    #[tokio::test]
    async fn test_get_before_init() {
        let client = HttpClient::new();
        assert!(client.get("https://example.com").await.is_err());
    }

    #[tokio::test]
    async fn test_get_after_init() {
        let mut client = HttpClient::new();
        client.init().unwrap();
        let response = client.get("https://example.com").await.unwrap();
        assert_eq!(response.status, 200);
    }
}
