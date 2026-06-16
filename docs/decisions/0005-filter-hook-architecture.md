# ADR 0005 — `FilterHook` lives in `spiral-core` (not `spiral-filter`)

- **Date:** 2026-06-16
- **Status:** Accepted
- **Supersedes:** none
- **Superseded by:** none
- **Author:** implementer agent (with 6 parallel `explore` subagents auditing the doc drift)
- **Scope:** packet 1.6.4 (filter runtime hook) + the post-audit Wave A fix.

## Context

Packet 1.6.4 (Bet 3) shipped the `spiral-filter` runtime: `Filter`,
`FilterHook`, `Decision`, and `default_network_rules`. To wire the
hook into `spiral-network::Client`, the implementer promoted
`spiral-filter` from a dev-dep to a regular dep of `spiral-network`
in `crates/spiral-network/Cargo.toml:16` and used
`spiral_filter::FilterHook` directly from `lib.rs`.

The post-packet doc-drift audit
(`docs/audits/2026-06-16-doc-drift.md` §1 P0 #1) flagged this as an
**architectural rule violation**: the canonical dep graph
(`.spiral/rules/architecture.md:16-53`) places `spiral-filter`
*upstream* of `spiral-network` in the DAG, and the "down-only" rule
on lines 55–56 forbids upward arrows.

```
$ grep -E "spiral-(filter|network)" .spiral/rules/architecture.md
                  spiral-filter
            spiral-network
```

The architecture diagram is explicit: `spiral-filter` is above
`spiral-network`. The 1.6.4 dep arrow inverted that.

## Decision

**Move `FilterHook`, `Decision`, and `Party` from `spiral-filter` to
`spiral-core`.** The `Filter` struct (the default implementer) stays
in `spiral-filter`. `spiral-network` takes a
`Box<dyn spiral_core::FilterHook>`. `spiral-filter/Cargo.toml` keeps
its existing `spiral-core` dep (it needs it for other types — `Error`,
`Result`); `spiral-network/Cargo.toml` reverts `spiral-filter` to a
dev-dep.

The dep arrows become:
- `spiral-filter → spiral-core` (existing, no change)
- `spiral-network → spiral-core` (existing, no change)
- `spiral-network → spiral-filter` (dev-dep only, used by
  `tests/filter_hook.rs` and the future `spiral-browser` binary)

The dev-dep does not violate the "down-only" rule because dev-deps
do not show up in the production dep graph (Cargo separates the
two). The `Filter` default implementer is constructed by *callers*
in `spiral-browser` and by tests, not by `spiral-network` itself.

## Alternatives considered

### Option B — Invert the API to a callback (function pointer)

`spiral-network` exposes `fn(&str, Party) -> Decision`; the policy
engine adapts. Works, but:

1. Loses the object-safe `Box<dyn FilterHook>` ergonomics.
2. The trait can never grow (no `policy_name()`, no future
   `PolicyOverride` parameter, no `Send + Sync` enforcement).
3. Doesn't help if a future caller wants to install *multiple*
   filters (e.g. per-context + global).

Rejected because the trait is the right abstraction and option A
gives it the right home.

### Option C — Ratify the upward dep arrow

Amend `.spiral/rules/architecture.md` to allow `network → filter`.
Rejected because:

1. It complicates the graph: every future packet on `spiral-filter`
   would need to think about the `spiral-network` consumer, not
   just `spiral-core`.
2. It would force a "filter can use network types" property
   (e.g. for HTTP fetch of the ruleset) that isn't actually needed.
3. The "down-only" rule is the right default; one-off exceptions
   erode the rule.

## Consequences

### Positive

- The dep graph is back to "down-only". `.spiral/rules/architecture.md:16-53`
  is accurate again.
- `spiral-core` is now the canonical home for "types that bridge
  engine crates" (per the existing re-export rule on
  `architecture.md:60` — "if a type is needed by 3+ crates,
  promote it to `spiral-core`"; `FilterHook` will be needed by
  `spiral-filter`, `spiral-network`, and the future
  `spiral-sandbox` packet that exposes it as a capability).
- `spiral-network` can add more filter surfaces in the future
  (`CookiePolicy`, `ReferrerPolicy`, `CSP`) without growing the
  regular dep graph.

### Negative

- `spiral-core` grows. It now contains the network policy
  contract in addition to the IPC protocol. This is a one-time
  growth; the new types are small (~70 lines) and well-scoped.
- The implementer didn't follow the Decision Protocol (AGENTS.md
  § Decision Protocol row 3) for packet 1.6.4. The next packet
  that does a dep-graph change must write an ADR first. This is
  a process fix, not a code fix, and is captured in the doc-drift
  audit P1 #30.

### Neutral

- The 1.6.4 surface (`Filter::with_default_policy`,
  `default_network_rules`, the `PolicyLevel` enum) is unchanged.
  All 1.6.4 tests pass without modification (verified
  2026-06-16: 58/58 test binaries, 0 failed).
- `docs/decisions/README.md` does not need updating (this ADR is
  0005, the next slot).

## Implementation

1. `spiral-core/src/lib.rs`: add `pub enum Party`, `pub enum
   Decision`, `pub trait FilterHook` (with `Send + Sync`,
   `should_block`, `policy_name`).
2. `spiral-filter/src/lib.rs`: re-export `Party`, `Decision`,
   `FilterHook` from `spiral-core`; remove the local definitions
   in `src/runtime/mod.rs`.
3. `spiral-filter/src/rule.rs`: replace `pub enum Party` with
   `pub use spiral_core::Party;` (keeps the `rule::Party` path
   working for the two existing callers in
   `src/syntax/network.rs:13` and
   `tests/rule_model_surface.rs:20`).
4. `spiral-network/src/lib.rs`: change `use spiral_filter::...` to
   `use spiral_core::{Decision, FilterHook, Party, ...}`.
5. `spiral-network/Cargo.toml`: move `spiral-filter` from
   `[dependencies]` to `[dev-dependencies]`.

## Wiring & Integration

- Crates affected: `spiral-core` (gains 3 types), `spiral-filter`
  (loses 3 local types, re-exports from core),
  `spiral-network` (imports change, dep-arrow reverted to
  dev-dep).
- Call sites: `spiral-network::Client::set_filter` and
  `Client::should_block` (renamed to `check_filter` internally)
  consume `Box<dyn spiral_core::FilterHook>`; tests in
  `spiral-network/tests/filter_hook.rs` install
  `Box<dyn FilterHook>` from a `spiral_filter::Filter`.
- Test coverage: all 1.6.4 tests in `spiral-filter` (23 unit + 7
  integration) and `spiral-network` (9 integration) pass without
  modification.
- End-to-end surface: `cargo test --workspace` → 58 test binaries,
  0 failed. `audit-orphan-exports.sh` → `spiral-filter OK (10
  symbols, all wired)`, `spiral-network OK (3 symbols, all wired)`,
  `spiral-core OK (16 symbols, all wired)` (was 16; the new types
  add to the surface but are immediately consumed by
  `spiral-filter` and `spiral-network` so they don't add orphans).

## Open followups (deferred to later packets, not in scope)

- **Real first/third-party detection from the document origin**
  (currently `spiral-network::Client::check_filter` hardcodes
  `Party::Third`). The `Context::run_script` packet 2.7.1 or a
  new packet 1.6.X can wire this. The trait signature
  (`should_block(url, party)`) already takes the party by value;
  only the caller needs to know the document origin.
- **Per-context `PolicyOverride`.** The trait method signature
  reserves space for a future override parameter; this packet
  doesn't add it.
- **EasyList / EasyPrivacy subscription.** M5+ deliverable.
  Out of scope for the filter *engine*; the `Filter` struct is
  ready to consume a larger ruleset when one is available.
