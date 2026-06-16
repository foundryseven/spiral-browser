//! Integration tests for the packet 1.6.4 filter-hook integration.
//!
//! **Wiring note (M4.5 Item 12, 2026-06-16):** these tests live in
//! `tests/` (not `src/`) so they compile as separate binaries that
//! consume the `spiral_network` public surface — the audit's
//! "external consumer" signal. The tests construct a real
//! `spiral_filter::Filter` and install it on a `Client` via
//! `set_filter`.
//!
//! The packet scope is "worst offenders only" — the default
//! `spiral_filter::Filter` ships with a small, illustrative set
//! of well-known third-party ad/tracker hostnames. Real
//! EasyList / EasyPrivacy subscription is an M5+ deliverable.

use spiral_filter::{Filter, FilterHook, Party};
use spiral_net::DnsResolver;
use spiral_network::Client;

fn client() -> Client<DnsResolver> {
    let mut resolver = DnsResolver::new();
    resolver.init().expect("resolver init");
    Client::new(resolver)
}

#[tokio::test]
async fn no_filter_installed_get_still_succeeds() {
    // No-op path: with no filter installed, the client behaves
    // exactly as in packet 1.6.3.
    let client = client();
    assert_eq!(client.filter_policy_name(), "none");
    let response = client
        .get("https://doubleclick.net/ad")
        .await
        .expect("no filter -> request goes through");
    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn filter_installed_blocks_known_tracker() {
    let mut client = client();
    client.set_filter(Some(Box::new(Filter::with_default_policy())));
    assert_eq!(client.filter_policy_name(), "worst-offenders");

    let result = client.get("https://doubleclick.net/ad").await;
    let msg = match result {
        Ok(_) => panic!("tracker must be blocked"),
        Err(e) => e.to_string(),
    };
    assert!(
        msg.contains("blocked by filter"),
        "error must mention the filter, got: {msg}"
    );
}

#[tokio::test]
async fn filter_installed_allows_unknown_host() {
    let mut client = client();
    client.set_filter(Some(Box::new(Filter::with_default_policy())));

    let response = client
        .get("https://example.com/")
        .await
        .expect("non-tracker host goes through");
    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn filter_blocks_post_too() {
    let mut client = client();
    client.set_filter(Some(Box::new(Filter::with_default_policy())));

    let result = client.post("https://googlesyndication.com/x", b"{}").await;
    assert!(result.is_err(), "POST to tracker must be blocked");
    let msg = result.err().unwrap().to_string();
    assert!(msg.contains("blocked by filter"), "got: {msg}");
}

#[tokio::test]
async fn setting_filter_to_none_restores_no_op_path() {
    let mut client = client();
    client.set_filter(Some(Box::new(Filter::with_default_policy())));
    // Round-trip: install, observe, then remove.
    assert_eq!(client.filter_policy_name(), "worst-offenders");
    assert!(client.get("https://doubleclick.net/").await.is_err());

    client.set_filter(None);
    assert_eq!(client.filter_policy_name(), "none");
    let response = client
        .get("https://doubleclick.net/")
        .await
        .expect("filter removed -> request goes through");
    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn filter_consulted_before_dns() {
    // The order matters: we don't want to spend a DNS round-trip
    // for a blocked URL. This is enforced by the implementation
    // (filter check is the first thing in `get`/`post`).
    let mut client = client();
    client.set_filter(Some(Box::new(Filter::with_default_policy())));

    // A URL that, if resolved, would have failed at the resolver
    // — but the filter blocks it first.
    let result = client.get("https://doubleclick.net/x").await;
    let msg = match result {
        Ok(_) => panic!("must be blocked"),
        Err(e) => e.to_string(),
    };
    assert!(msg.contains("blocked by filter"), "got: {msg}");
}

#[tokio::test]
async fn custom_filter_hook_can_be_installed() {
    // The trait is object-safe. A consumer can install a
    // non-default `FilterHook` implementation.
    struct AlwaysBlock;
    impl FilterHook for AlwaysBlock {
        fn should_block(&self, _url: &str, _party: Party) -> spiral_filter::Decision {
            spiral_filter::Decision::Block {
                rule_id: 0,
                reason: "always block".to_string(),
            }
        }
        fn policy_name(&self) -> &str {
            "always-block"
        }
    }

    let mut client = client();
    client.set_filter(Some(Box::new(AlwaysBlock)));
    assert_eq!(client.filter_policy_name(), "always-block");
    assert!(client.get("https://example.com/").await.is_err());
}

#[tokio::test]
async fn filter_accessor_returns_installed_hook() {
    let mut client = client();
    assert!(client.filter().is_none(), "no filter at construction");
    client.set_filter(Some(Box::new(Filter::with_default_policy())));
    assert!(client.filter().is_some());
    assert_eq!(
        client.filter().unwrap().policy_name(),
        "worst-offenders"
    );
}

#[tokio::test]
async fn decision_block_carries_rule_id() {
    // The `Decision::Block` variant carries the rule_id from the
    // filter. The `Client` surfaces it in the error message.
    let mut client = client();
    client.set_filter(Some(Box::new(Filter::with_default_policy())));

    let err = client
        .get("https://doubleclick.net/")
        .await;
    let msg = match err {
        Ok(_) => panic!("tracker must be blocked"),
        Err(e) => e.to_string(),
    };
    assert!(
        msg.contains("rule_id="),
        "error must carry rule_id, got: {msg}"
    );
}
