# 0004 — Resolver trait: native `async fn` + generic bounds, not `Box<dyn>`

> **Decision:** Use native `async fn` in traits (stable
> in Rust 1.75+) with generic bounds (`R: Resolver`),
> not `async-trait` + `Box<dyn Resolver>`. The trait
> method returns `impl Future<…> + Send`, which is
> not dyn-compatible.
>
> **Status:** Accepted. 2026-06-16.
>
> **Driver:** M4.5 Item 8 (`spiral_net::Resolver`).
> **Deciders:** `ozore/custom` (Implementer) +
> user.
>
> **Scope:** This pattern applies to all new
> async-trait definitions in the Spiral workspace.
> It is a workspace-wide convention, not a
> one-crate choice.

---

## Context

M4.5 Item 8 introduces the `Resolver` trait, the
canonical abstraction for DNS resolution. The trait
method is `async fn resolve(&self, domain: &str) -> Result<Vec<IpAddr>>`.

Two design options were on the table:

1. **Native `async fn` + `Box<dyn Resolver>`.**
   Requires the `async-trait` macro (or hand-rolled
   `Pin<Box<dyn Future + Send>>`) to make the trait
   dyn-compatible. Adds a workspace-level dependency.

2. **Native `async fn` + generic bounds
   (`R: Resolver`).** The trait is not
   dyn-compatible; consumers hold the resolver by
   generic bound. No new dep.

## Decision

**Option 2.** Use native `async fn` in traits. The
trait is not dyn-compatible. Consumers take the
resolver by generic bound.

The audit script's "wired" signal is a word-boundary
grep for the trait name; a generic-bound consumer
(`fn foo<R: Resolver>(r: R)`) references the trait by
name and is therefore sufficient to mark the trait
as wired.

## Consequences

### Positive

- **No new dep.** `async-trait` is a small, mature
  crate, but adding it would be the only dep at
  the workspace level that exists solely to support
  one trait. The generic-bound pattern is the
  zero-dep solution.
- **Future-friendly.** Native `async fn` in traits
  is stable, well-typed, and gets the same
  monomorphisation as ordinary `async fn`. The
  compiler inlines the call site.
- **Object-safety is not a goal here.** The audit
  cares about reachability by name; a `Box<dyn>` is
  a stronger requirement than the audit needs.

### Negative

- **No dynamic dispatch.** Consumers cannot hold a
  `Box<dyn Resolver>`. If a future implementer needs
  to swap resolvers at runtime (e.g. test fixtures
  that swap a mock for a real resolver), they must
  use enums or `Rc<RefCell<…>>` instead. This is a
  rare requirement in the engine.
- **Monomorphisation cost.** Each `R: Resolver`
  consumer instantiates the function once per
  implementer. The cost is real but small: the
  engine has at most 2-3 implementers
  (`DnsResolver`, `HickoryResolver`, possibly a
  test mock).
- **Workaround for the object-safety gap.** When
  the test suite needs object safety (e.g. to
  test the trait through a trait object), it must
  use `async-trait`. We accept this — tests are
  the only case where object safety is needed.

## Alternatives considered

- **`async-trait` macro.** The de-facto standard
  for `Box<dyn AsyncTrait>`. Rejected because it
  adds a workspace dep for a single trait.
- **Hand-rolled `Pin<Box<dyn Future + Send>>`.**
  Works without `async-trait`, but the boilerplate
  is high. Rejected because the generic-bound
  pattern is simpler.
- **Synchronous `fn resolve(&self) -> Result<Vec<IpAddr>>`
  with a thread-pool.** Rejected because the
  natural consumer (HTTP, TLS handshake) is async
  and would need to wrap the call in
  `spawn_blocking` everywhere. The trait is async
  because the engine is async.

## Cross-cutting implications

- **M4.5+ Item 11 (HTTP client).** The `spiral-
  network` HTTP client will take `R: Resolver` by
  generic bound, not `Box<dyn Resolver>`. The
  pattern is inherited.
- **M4.5+ Item 12 (filter runtime).** The filter
  hook that takes DNS will also use the generic-
  bound pattern.
- **Future async traits.** Any new async trait in
  the workspace (Vortex's `JSRuntime` is one
  candidate) will use this pattern unless there is
  a specific reason to need object safety.

## References

- `crates/spiral-net/src/lib.rs` — the trait
  definition and the doc comment.
- `crates/spiral-net/tests/resolver_surface.rs` —
  the consumer pattern (generic bound).
- Rust RFC 3185: "Async functions in traits" — the
  language feature this decision depends on.
- `scripts/audit-orphan-exports.sh` — the
  verification signal (trait name referenced from
  outside the lib).
