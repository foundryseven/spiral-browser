//! `spiral-network` HTTP/1.1 client smoke binary.
//!
//! **Packet 1.6.3 (M4.5 Item 11).** Demonstrates the full pipeline:
//!
//! ```text
//!   DnsResolver  ──▶  Client<R: Resolver>  ──▶  get(url)  ──▶  HttpResponse
//! ```
//!
//! Run with:
//!
//! ```bash
//! cargo run -p spiral-network --bin http_get -- https://example.com/
//! ```
//!
//! Phase 1 is a stub: it resolves the host, logs the resolved IP, and
//! returns a `200 OK` with an empty body. Real HTTP/1.1 I/O lands in
//! M5 alongside `HickoryResolver` and the TLS hand-off.

use spiral_net::DnsResolver;
use spiral_network::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let url = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "https://example.com/".to_string());

    let mut resolver = DnsResolver::new();
    resolver.init()?;

    let client = Client::new(resolver);
    let ua = client.user_agent();
    eprintln!("[http_get] User-Agent: {ua}");

    let response = client.get(&url).await?;
    eprintln!(
        "[http_get] GET {} -> status={} body_len={}",
        url,
        response.status,
        response.body.len()
    );
    Ok(())
}
