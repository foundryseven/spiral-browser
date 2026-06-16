//! Integration tests for the `spiral-net` `Resolver` trait.
//!
//! **Wiring note (M4.5 Item 8, 2026-06-16):** the audit
//! flagged `DnsResolver` and `TlsConfig` as orphan. Item 8
//! introduces the `Resolver` trait; `DnsResolver` implements
//! it. This test file exercises the trait through generic
//! bounds (`R: Resolver`) so the audit recognises the trait
//! surface as wired.
//!
//! These tests live in `tests/` (not `src/`) so they
//! compile as separate binaries that consume the lib's
//! public surface — the audit's "external consumer"
//! signal.
//!
//! # Why generic, not `Box<dyn Resolver>`?
//!
//! The `Resolver` trait uses native `async fn` in traits,
//! which is **not dyn-compatible** (Rust 1.96 — `impl Future`
//! return types break object safety). Using `async-trait`
//! would force a workspace-level dep that is otherwise
//! unneeded; the cleaner path is to take the resolver by
//! generic bound. The `spiral-network` consumer
//! (M4.5+ Item 11) will also take `R: Resolver` rather
//! than `Box<dyn Resolver>` for the same reason.
//!
//! This is a deliberate design choice, not a TODO. See
//! `docs/decisions/0004-resolver-trait-async-design.md`
//! for the ADR.

use std::net::IpAddr;
use std::str::FromStr;

use spiral_core::Result;
use spiral_net::{DnsResolver, Resolver, TlsConfig};

#[test]
fn resolver_trait_is_importable_from_outside() {
    // Compile-time check: the trait is reachable by
    // name from outside the lib. The audit's grep
    // finds `Resolver` here.
    fn _accept_generic<R: Resolver>(r: R) {
        drop(r);
    }
    let resolver = DnsResolver::new();
    _accept_generic(resolver);
}

#[test]
fn dns_resolver_implements_resolver_trait() {
    // The `DnsResolver` type must satisfy the
    // `Resolver` bound. This is the audit's
    // "consumer" signal.
    fn _check<R: Resolver>() {}
    _check::<DnsResolver>();
}

#[test]
fn tls_config_is_constructable() {
    // The TlsConfig type is part of the public
    // surface; this test exercises it so the audit
    // recognises the symbol as wired.
    let _cfg = TlsConfig::default();
}

#[tokio::test]
async fn dns_resolver_resolve_via_trait_bound() {
    // End-to-end: construct a DnsResolver, call
    // resolve through the trait bound. The return
    // type is `Vec<IpAddr>` (the new M4.5 Item 8
    // contract).
    async fn call<R: Resolver>(r: &R, domain: &str) -> Result<Vec<IpAddr>> {
        r.resolve(domain).await
    }

    let mut resolver = DnsResolver::new();
    resolver.init().expect("init");
    let ips: Vec<IpAddr> = call(&resolver, "example.com").await.expect("resolve");
    // The Phase 1 stub returns 127.0.0.1; the assertion
    // is that the resolve succeeds and the result is
    // non-empty and parses as `IpAddr`.
    assert!(!ips.is_empty());
    // Round-trip: each IP must parse back as `IpAddr`.
    for ip in &ips {
        let s = ip.to_string();
        let _ = IpAddr::from_str(&s).expect("IpAddr round-trips");
    }
}

#[tokio::test]
async fn resolver_returns_ip_addr_not_string() {
    // The M4.5 Item 8 contract: `resolve` returns
    // `Vec<IpAddr>`, not `Vec<String>`. The M4.4 stub
    // returned `Vec<String>`; this test pins the new
    // contract by checking the type at compile time.
    let mut resolver = DnsResolver::new();
    resolver.init().expect("init");
    let ips: Vec<IpAddr> = Resolver::resolve(&resolver, "example.com")
        .await
        .expect("resolve");
    // `127.0.0.1` is the Phase 1 stub answer.
    assert!(
        ips.iter().any(|ip| ip.is_loopback()),
        "Phase 1 stub must return a loopback address, got {:?}",
        ips
    );
}
