//! Spiral Browser — HTTP Client
//!
//! HTTP client and networking for Spiral Browser.
//!
//! # Surfaces
//!
//! - [`HttpClient`] — the Phase 1 hello-world client. Stateful
//!   (`init()`-then-use), no resolver coupling, returns a stub
//!   `HttpResponse` with status 200. Kept for backward compat with the
//!   M4.4 call sites; not the recommended path forward.
//! - [`Client`] — the **packet 1.6.3 surface**. Generic over a DNS
//!   resolver (`R: spiral_net::Resolver`) and exposes `get` and `post`
//!   against a real `HttpResponse` shape. This is the surface the
//!   renderer pipeline will call from M5+.
//!
//! Both are stubs today — they construct a `200 OK` with an empty body
//! and log the URL. The point of packet 1.6.3 is to lock in the
//! `Client<R: Resolver>` generic-bound shape (per
//! `docs/decisions/0004-resolver-trait-async-design.md`) and to wire
//! the audit's "external consumer" surface (the integration test +
//! the `http_get` binary). Real `hyper` HTTP/1.1 I/O lands in M5 when
//! the resolver implementer (`HickoryResolver`) and the TLS hand-off
//! (`TlsConfig` from `spiral-net`) are ready.

use std::collections::HashMap;

use spiral_core::{Decision, Error, FilterHook, Party, Result};
use spiral_net::Resolver;

/// HTTP response.
///
/// The body is `Vec<u8>` rather than `String` so binary responses
/// (images, fonts, gzipped HTML) round-trip without a lossy decode.
/// Decoding to `String` / structured types is the caller's job.
pub struct HttpResponse {
    /// HTTP status code (e.g. 200, 404, 500).
    pub status: u16,
    /// Response headers (lowercased key → value).
    pub headers: HashMap<String, String>,
    /// Response body bytes.
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Construct an empty 200 OK response.
    pub fn ok() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Construct a response with the given status and an empty body.
    pub fn with_status(status: u16) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

/// HTTP client (Phase 1 hello-world surface).
///
/// Kept for backward compat with the M4.4 call sites. New code should
/// use [`Client<R>`] — the generic-bound surface introduced in packet
/// 1.6.3.
pub struct HttpClient {
    /// Client is initialized.
    initialized: bool,
}

impl HttpClient {
    /// Create a new HTTP client.
    pub fn new() -> Self {
        Self { initialized: false }
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
        Ok(HttpResponse::ok())
    }

    /// Perform a POST request.
    pub async fn post(&self, url: &str, body: &[u8]) -> Result<HttpResponse> {
        if !self.initialized {
            return Err(Error::Network("Client not initialized".to_string()));
        }

        // Phase 1: Placeholder response
        // Phase 2: hyper POST request
        log::trace!("POST {} ({} bytes)", url, body.len());
        Ok(HttpResponse::ok())
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

/// Generic-bound HTTP client (packet 1.6.3 surface).
///
/// `Client<R: Resolver>` takes a DNS resolver by **generic bound**, not
/// `Box<dyn Resolver>`. This matches the workspace-wide convention
/// (ADR 0004): the `Resolver` trait uses native `async fn`, which
/// returns `impl Future + Send` and is **not dyn-compatible**. Generic
/// bounds sidestep the object-safety problem and avoid pulling in the
/// `async-trait` crate workspace-wide.
///
/// # Phase 1 behaviour
///
/// Today the client constructs an empty `200 OK` and logs the resolved
/// IP. Real `hyper` HTTP/1.1 I/O lands in M5 once:
/// - `HickoryResolver` (Phase 2 of `spiral-net`) is shipped.
/// - The TLS hand-off with `TlsConfig` is wired.
/// - `spiral-network` gets a real connection pool + redirect policy.
///
/// The Phase 1 contract is the *shape* of the API, not the wire I/O.
///
/// # Filter hook (packet 1.6.4, Bet 3)
///
/// `Client` holds an `Option<Box<dyn FilterHook>>`. The default is
/// `None` — the no-op path. When set, every `get` / `post` consults
/// the filter before doing DNS. A `Block` decision surfaces as
/// `Err(Error::Network(...))`. First/third-party classification is
/// `Party::Third` for the Phase 1 stub (real first-party detection
/// from the document origin lands in M5).
pub struct Client<R: Resolver> {
    resolver: R,
    user_agent: String,
    filter: Option<Box<dyn FilterHook>>,
}

impl<R: Resolver> Client<R> {
    /// Construct a `Client` that uses the given resolver.
    pub fn new(resolver: R) -> Self {
        Self {
            resolver,
            user_agent: format!("SpiralBrowser/{}", env!("CARGO_PKG_VERSION")),
            filter: None,
        }
    }

    /// Construct a `Client` with a custom `User-Agent` string.
    pub fn with_user_agent(resolver: R, user_agent: impl Into<String>) -> Self {
        Self {
            resolver,
            user_agent: user_agent.into(),
            filter: None,
        }
    }

    /// The `User-Agent` string this client will send.
    #[must_use]
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Borrow the underlying resolver.
    #[must_use]
    pub fn resolver(&self) -> &R {
        &self.resolver
    }

    /// Install (or replace) the filter hook.
    ///
    /// Pass `None` to remove the hook (return to the no-op path).
    /// The default `Client::new` has no filter installed.
    pub fn set_filter(&mut self, filter: Option<Box<dyn FilterHook>>) {
        self.filter = filter;
    }

    /// Borrow the installed filter hook, if any.
    #[must_use]
    pub fn filter(&self) -> Option<&dyn FilterHook> {
        self.filter.as_deref()
    }

    /// The policy name advertised by the installed filter, or
    /// `"none"` if no filter is installed. Used for logs.
    #[must_use]
    pub fn filter_policy_name(&self) -> &str {
        self.filter
            .as_deref()
            .map(|f| f.policy_name())
            .unwrap_or("none")
    }

    /// Consult the filter (if installed) and decide whether this
    /// URL is allowed. Returns the [`Decision`] for logging or
    /// returns `Block` as an [`Error::Network`].
    fn check_filter(&self, url: &str) -> Result<()> {
        // Phase 1: real first-party detection from the
        // document origin is M5+. Default to `Third` so
        // third-party-only rules fire.
        const PARTY: Party = Party::Third;
        if let Some(f) = self.filter.as_deref() {
            match f.should_block(url, PARTY) {
                Decision::Allow => Ok(()),
                Decision::Block { rule_id, reason } => {
                    log::info!(
                        "Filter blocked: url={} party={:?} rule_id={} reason={}",
                        url,
                        PARTY,
                        rule_id,
                        reason
                    );
                    Err(Error::Network(format!(
                        "blocked by filter (rule_id={rule_id}, policy={}): {reason}",
                        f.policy_name()
                    )))
                }
            }
        } else {
            Ok(())
        }
    }

    /// Perform a `GET` request.
    ///
    /// Phase 1: resolve the host, log the resolved IP + URL, return a
    /// stub 200. Phase 2 (M5+): dial the resolved IP, send the HTTP/1.1
    /// request line + headers over `hyper`, decode the response.
    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        self.check_filter(url)?;
        let host = extract_host(url)?;
        let ips = self.resolver.resolve(&host).await?;
        if ips.is_empty() {
            return Err(Error::Network(format!(
                "Resolver returned no addresses for {host}"
            )));
        }
        let ip = ips[0];
        log::trace!(
            "Client<R>::get url={} host={} resolved_ip={} ua={}",
            url,
            host,
            ip,
            self.user_agent
        );
        // Phase 1 stub: the resolved IP is logged but not dialled.
        // The real connect lands in M5.
        Ok(HttpResponse::ok())
    }

    /// Perform a `POST` request.
    ///
    /// Phase 1: same shape as [`get`](Self::get) but logs the body
    /// length. Phase 2: real POST.
    pub async fn post(&self, url: &str, body: &[u8]) -> Result<HttpResponse> {
        self.check_filter(url)?;
        let host = extract_host(url)?;
        let ips = self.resolver.resolve(&host).await?;
        if ips.is_empty() {
            return Err(Error::Network(format!(
                "Resolver returned no addresses for {host}"
            )));
        }
        let ip = ips[0];
        log::trace!(
            "Client<R>::post url={} host={} resolved_ip={} body_len={} ua={}",
            url,
            host,
            ip,
            body.len(),
            self.user_agent
        );
        Ok(HttpResponse::ok())
    }
}

/// Extract the host component of a URL.
///
/// Phase 1: parse the `scheme://host[:port][/path]` form. Anything
/// more exotic (userinfo, IPv6 brackets, query strings) is the M5+
/// job. The parser is intentionally minimal — the goal is to give
/// the resolver a sensible input and to surface a clear error for
/// malformed URLs.
fn extract_host(url: &str) -> Result<String> {
    let rest = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .ok_or_else(|| Error::Network(format!("URL must be http:// or https:// (got {url:?})")))?;
    let host_end = rest.find(['/', ':', '?']).unwrap_or(rest.len());
    let host = &rest[..host_end];
    if host.is_empty() {
        return Err(Error::Network(format!("URL has empty host: {url:?}")));
    }
    Ok(host.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_host_strips_scheme_and_path() {
        assert_eq!(
            extract_host("http://example.com/foo").unwrap(),
            "example.com"
        );
        assert_eq!(extract_host("https://example.com").unwrap(), "example.com");
        assert_eq!(
            extract_host("https://example.com:8080/x").unwrap(),
            "example.com"
        );
    }

    #[test]
    fn extract_host_rejects_non_http() {
        assert!(extract_host("ftp://example.com").is_err());
        assert!(extract_host("not a url").is_err());
    }

    #[test]
    fn extract_host_rejects_empty_host() {
        assert!(extract_host("http:///foo").is_err());
    }

    #[test]
    fn http_response_ok_has_status_200() {
        let r = HttpResponse::ok();
        assert_eq!(r.status, 200);
        assert!(r.headers.is_empty());
        assert!(r.body.is_empty());
    }

    #[test]
    fn http_response_with_status() {
        let r = HttpResponse::with_status(404);
        assert_eq!(r.status, 404);
    }

    #[tokio::test]
    async fn test_create_http_client() {
        let client = HttpClient::new();
        assert!(!client.is_initialized());
    }

    #[tokio::test]
    async fn test_init_http_client() {
        let mut client = HttpClient::new();
        client.init().unwrap();
        assert!(client.is_initialized());
    }

    #[tokio::test]
    async fn test_http_client_get_before_init() {
        let client = HttpClient::new();
        assert!(client.get("https://example.com").await.is_err());
    }

    #[tokio::test]
    async fn test_http_client_get_after_init() {
        let mut client = HttpClient::new();
        client.init().unwrap();
        let response = client.get("https://example.com").await.unwrap();
        assert_eq!(response.status, 200);
    }
}
