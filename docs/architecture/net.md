# `spiral-net` (DNS + TLS Resolution)

> **Brand:** *(unbranded).* **Crate:** `spiral-net`.
> **Scope:** DNS resolution + TLS configuration.
> **Status:** Step 1.6 / Packet 1.6.3 shipped (`Resolver` trait,
> `DnsResolver` Phase 1 stub, `TlsConfig`); consumed by
> `spiral-network` (Packet 1.6.4 — `Client<R>` is generic
> over `R: Resolver`). All orphans closed in 1.6.3
> (see `audit-orphan-exports.sh`).

`spiral-net` is Spiral's DNS + TLS resolution layer.
It is the boundary between Spiral's high-level
network code (in `spiral-network`) and the
underlying transport (in `spiral-ipc`). It wraps
`hickory-resolver` for real DNS (M5+) and
`rustls` for TLS (M5+).

The M4.5 posture is "Track E wrapper" — a thin
trait over the underlying library, with the Phase 1
stub returning loopback for every domain. The
Phase 2 work (M5+) replaces the stub with the real
`hickory-dns` integration.

---

## Public surface (Step 1.6 / Packet 1.6.3)

```rust
// DNS resolution trait.
pub trait Resolver: Send + Sync {
    fn resolve(
        &self,
        domain: &str,
    ) -> impl std::future::Future<Output = Result<Vec<IpAddr>>> + Send;
}

// Phase 1 stub implementer.
pub struct DnsResolver { initialized: bool }
impl Resolver for DnsResolver { … }

// TLS configuration.
pub struct TlsConfig { pub verify: bool }
```

The `Resolver` trait is **not dyn-compatible** —
it uses native `async fn` in traits, which returns
`impl Future<…> + Send`. Consumers take the
resolver by generic bound (`R: Resolver`). See
[ADR 0004](../decisions/0004-resolver-trait-async-
design.md) for the design rationale.

Packet 1.6.3 added no new public types — it landed the
generic-bound consumer wiring (`Client<R: Resolver>`)
in `spiral-network`, which is the live "wired" signal
for the trait. Pre-1.6.3 the `Resolver` was a symbol
with no consumer (orphan); post-1.6.3
`./scripts/audit-orphan-exports.sh` reports
`spiral-net OK (3 symbols, all wired)`.

---

## Internal layout

```
spiral-net/src/
└── lib.rs   — Resolver trait + DnsResolver + TlsConfig
              (single file; M5+ may split into
              resolver.rs + tls.rs)
```

The M4.5 file is ~190 lines. The split is
**not** done in M4.5; a single file is fine
until M5+ adds `HickoryResolver` and a real
`TlsConnector`.

---

## Constraints

- **No `unsafe`.** The resolver is pure logic; the
  Rust standard library is sufficient.
- **No `hickory-dns` call sites in M4.5.** The
  Phase 1 stub returns `127.0.0.1` for every
  domain. The `hickory-resolver` workspace dep is
  declared in `Cargo.toml` for M5+ use.
- **Trait is not object-safe.** See ADR 0004. The
  audit's "wired" signal is the trait name
  referenced from a generic-bound consumer
  (`fn foo<R: Resolver>(r: R)`), not a trait
  object.
- **`Vec<IpAddr>` return, not `Vec<String>`.**
  The M4.4 stub returned `Vec<String>`; the M4.5
  contract parses the IP at the resolver boundary
  so downstream code does not have to re-validate.

---

## Test posture

- 6 lib tests in M4.5 cover the trait impl, the
  Phase 1 stub, and the init lifecycle.
- 5 integration tests in
  `tests/resolver_surface.rs` cover the trait's
  reachability from outside the lib, the
  generic-bound consumer pattern, the
  `Vec<IpAddr>` return type, and the
  `TlsConfig` symbol.
- M5+ will add the `HickoryResolver` tests
  (real DNS over the network, ~10 tests).

Total projected: ~25 lib tests + ~15 integration
tests for `spiral-net`.

---

## Do-not-touch zones (M4.5)

- The `Resolver` trait signature
  (`async fn resolve(&self, domain: &str) -> Result<Vec<IpAddr>>`).
  Changing it is a breaking change.
- The `TlsConfig` fields. Adding a field is a
  breaking change.
- The `DnsResolver` Phase 1 stub behaviour
  (loopback for every domain). The Phase 2
  replacement (`HickoryResolver`) is a separate
  type; do not modify the stub to call
  `hickory-resolver`.

---

## Related

- `docs/decisions/0004-resolver-trait-async-design.md`
  — the async-trait / generic-bound design choice.
- `docs/audits/2026-06-15-baseline.md` § Track E —
  the wrapper-posture for `spiral-net`.
- `docs/audits/2026-06-15-baseline.md` § Item 8 —
  the M4.5 deliverable scope.
- `AGENTS.md` § `spiral-net` (forthcoming; the
  working rules for this crate).
- `crates/spiral-network/` — the HTTP consumer
  that will take `R: Resolver` (M4.5+ Item 11).

---

## Filter hook integration (Packet 1.6.4)

`spiral-net` itself does not call into the filter — the
filter integration lives in `spiral-network::Client`,
not here. However, the network path the filter
intercepts is "before DNS resolution":

```
caller → Client::request → FilterHook::decide → Resolver::resolve → TLS
```

So a blocked request short-circuits before any DNS
work happens. The DNS resolver only sees URLs that
the filter has already accepted. See
`spiral-network/src/lib.rs:152-230` (the `Client<R>`
generic-bound consumer) and
[`ADR 0005`](../decisions/0005-filter-hook-architecture.md)
for the architecture decision that moved
`FilterHook` + `Decision` + `Party` from
`spiral-filter` to `spiral-core` (so the network
crate could depend on them without depending on
the whole `spiral-filter` dep graph).
