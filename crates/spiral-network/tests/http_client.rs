//! Integration tests for the packet 1.6.3 HTTP/1.1 client surface.
//!
//! **Wiring note (M4.5 Item 11, 2026-06-16):** these tests live in
//! `tests/` (not `src/`) so they compile as separate binaries that
//! consume the `spiral_network` public surface — the audit's
//! "external consumer" signal.
//!
//! The tests construct a real `spiral_net::DnsResolver` (a `dev`
//! dependency of `spiral-network`) and hold it behind a
//! `Client<R: Resolver>` generic bound. This is the packet-1.6.3
//! contract:
//!
//! - `Client` is generic over the resolver type.
//! - `Client::get` resolves the host, then returns a `200 OK` stub.
//! - The `Resolver` trait is the workspace-wide async-trait convention
//!   (ADR 0004): `R: Resolver`, not `Box<dyn Resolver>`.
//!
//! The Phase 1 stub is **deliberately** non-dialing. Real HTTP/1.1
//! I/O lands in M5 alongside `HickoryResolver`. The point of this
//! packet is to lock in the shape.

use spiral_net::{DnsResolver, Resolver};
use spiral_network::Client;

#[tokio::test]
async fn client_with_dns_resolver_resolves_then_returns_200() {
    let mut resolver = DnsResolver::new();
    resolver.init().expect("resolver init");
    let client = Client::new(resolver);
    let response = client
        .get("https://example.com/")
        .await
        .expect("GET returns a stub 200");
    assert_eq!(response.status, 200);
    assert!(response.headers.is_empty());
    assert!(response.body.is_empty());
}

#[tokio::test]
async fn client_holds_resolver_by_generic_bound() {
    // This is the audit's "external consumer" signal: a consumer
    // outside `spiral_network` constructs a `Client<R>` with a
    // concrete `R: Resolver` implementer.
    fn _check_takes_resolver<R: Resolver>(r: R) -> Client<R> {
        Client::new(r)
    }
    let resolver = DnsResolver::new();
    let _client = _check_takes_resolver(resolver);
}

#[tokio::test]
async fn client_rejects_non_http_scheme() {
    let resolver = DnsResolver::new();
    let client = Client::new(resolver);
    let result = client.get("ftp://example.com").await;
    assert!(result.is_err(), "non-http URL must be rejected");
}

#[tokio::test]
async fn client_user_agent_defaults_to_package_version() {
    let resolver = DnsResolver::new();
    let client = Client::new(resolver);
    let ua = client.user_agent();
    assert!(ua.starts_with("SpiralBrowser/"), "got UA: {ua}");
}

#[tokio::test]
async fn client_user_agent_can_be_overridden() {
    let resolver = DnsResolver::new();
    let client = Client::with_user_agent(resolver, "spiral-test/0.0.0");
    assert_eq!(client.user_agent(), "spiral-test/0.0.0");
}

#[tokio::test]
async fn client_resolver_accessor_returns_same_resolver() {
    let mut resolver = DnsResolver::new();
    resolver.init().expect("init");
    let client = Client::new(resolver);
    // The client hands back the same resolver it was given.
    let r = client.resolver();
    let ips = r.resolve("example.com").await.expect("resolve");
    assert!(!ips.is_empty(), "Phase 1 stub returns a loopback IP");
}

#[tokio::test]
async fn client_post_returns_200_with_stub_body() {
    let mut resolver = DnsResolver::new();
    resolver.init().expect("resolver init");
    let client = Client::new(resolver);
    let response = client
        .post("https://example.com/api", b"{}")
        .await
        .expect("POST returns a stub 200");
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[tokio::test]
async fn client_takes_resolver_by_value() {
    // Compile-time check: `Client::new` takes the resolver by value
    // (the dev-arrow pattern in ADR 0004). If this test compiles,
    // the `Client<R>` shape is correct.
    let resolver = DnsResolver::new();
    let _client: Client<DnsResolver> = Client::new(resolver);
}

// ---------------------------------------------------------------------------
// `HttpClient` (Phase 1 hello-world surface) — exercised here so the
// audit does not flag it as an orphan. Packet 3.1.1 will introduce the
// `Client` *trait* with `HttpClient` as its first impl.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn http_client_get_after_init_returns_200() {
    use spiral_network::HttpClient;
    let mut client = HttpClient::new();
    client.init().expect("init");
    let response = client
        .get("https://example.com/")
        .await
        .expect("GET returns 200");
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[tokio::test]
async fn http_client_post_after_init_returns_200() {
    use spiral_network::{HttpClient, HttpResponse};
    let mut client = HttpClient::new();
    client.init().expect("init");
    let response: HttpResponse = client
        .post("https://example.com/api", b"{}")
        .await
        .expect("POST returns 200");
    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn http_client_get_before_init_errors() {
    use spiral_network::HttpClient;
    let client = HttpClient::new();
    assert!(client.get("https://example.com/").await.is_err());
}

#[tokio::test]
async fn http_response_with_status_sets_status() {
    use spiral_network::HttpResponse;
    let r = HttpResponse::with_status(404);
    assert_eq!(r.status, 404);
    assert!(r.body.is_empty());
}
