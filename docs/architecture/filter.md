# `spiral-filter` (Filter) — Compile-time Ad & Policy Filter

> **Brand:** *(unbranded; "Spirit" in the priority
> taxonomy is unrelated — see `docs/glossary.md`.)*
> **Crate:** `spiral-filter`. **Scope:** URL pattern
> matchers, CSS-selector matchers, element-attribute
> matchers, action enum (block / hide / constrain /
> allow), runtime hook. **Status:** Step 1.6 /
> Packet 1.6.4 shipped (runtime hook + `FilterHook`
> trait in `spiral-core` per ADR 0005). All orphans
> closed in 1.6.4 (see
> `audit-orphan-exports.sh`: `spiral-filter OK (10/10)`).

`spiral-filter` is Spiral's compile-time HTML/CSS
policy engine. It sits between the network layer and
the HTML parser. It receives raw HTTP body bytes,
parses HTML+CSS, and produces a *transformed* document
with worst-offender ads removed or constrained.

The default policy is **"worst offenders only"**:
block layout-breaking banners, popups, autoplay
video/audio, interstitials. Allow reasonable ads.
No telemetry, no third-party tracking.

---

## Public surface (target, M5+)

```rust
pub struct Rule { … }                // matcher + action + metadata
pub struct Matcher { … }             // URL / selector / attribute
pub enum Action { … }                // Block, Hide, Constrain, Allow
pub enum RuleKind { … }              // UrlRule, CssRule, ElementRule
pub enum Severity { … }              // Advisory, Warning, Strict
pub enum Party { First, Third }      // first-party vs third-party (in spiral-core per ADR 0005)
pub enum Decision { Allow, Block { reason: String } }
pub trait FilterHook { fn decide(&self, ctx: &FilterContext) -> Decision; }

pub struct FilterEngine { … }
impl FilterEngine {
    pub fn new(policy: &Policy) -> Self;
    pub fn should_block(&self, url: &Url) -> Decision;
    pub fn transform(&self, dom: &mut Dom, stylesheet: &mut Stylesheet);
}
```

The M4.4 skeleton has the **rule model types**;
M4.5 Item 12 / Packet 1.6.4 ships the runtime
(`Filter` struct, `default_network_rules`,
`match_url::extract_host`). The
`FilterHook` trait itself lives in
`spiral-core` (see "Fork 2" below) and is
re-exported from `spiral-filter::lib.rs` for
backwards compatibility. The M5+ work adds
the user-facing DSL parser, the built-in
filter lists, and the integration tests
with `spiral-network` (already wired in
Packet 1.6.4).

---

## Internal layout

```
spiral-filter/src/
├── lib.rs           — public surface (re-exports FilterHook from spiral-core per ADR 0005)
├── rule.rs          — Rule, Matcher, Action, RuleKind, Party (re-exported)
├── policy/          — Policy, default-policy constant, slider
├── syntax/          — rule syntax (the user-facing DSL)
├── lists/           — built-in filter lists
├── compile/         — compile-time rule evaluation
├── runtime/
│   ├── mod.rs       — runtime hook (Filter, default_network_rules)
│   └── match_url.rs — extract_host() (URL host extractor)
└── error.rs         — FilterError
```

The skeleton has the rule model types, the error
type, and (post-1.6.4) the runtime hook. M5+ adds
the rest of the user-facing surface (network filter
lists, integration tests against real filter lists,
network integration tests).

---

## Constraints

- **No telemetry, no third-party tracking.** The
  filter does not phone home. The default policy
  explicitly bans this. ADR equivalent: not yet
  written; flag if a future change proposes it.
- **No `unsafe`.** The filter is pure logic; the
  Rust standard library is sufficient.
- **Compile-time evaluation where possible.** Rules
  with a static URL pattern should be evaluated at
  link time, not at runtime. The M5+ `compile`
  module owns this.
- **Default to "Allow".** When in doubt, the rule
  engine must err on the side of letting content
  through. The "worst offenders only" policy is
  conservative; over-blocking is a defect.

---

## Test posture

- 6 lib tests in M4.4 cover the rule-model types
  (constructor, match, action enums).
- Packet 1.6.4 (the runtime) adds 4 more:
  allow-by-default, block-by-explicit-rule,
  third-party default, and the
  `match_url::extract_host` host-extraction
  regression tests. Total: 10 lib tests
  (`cargo test -p spiral-filter`).
- M5+ adds the syntax tests, the policy tests,
  and the integration tests with
  `spiral-network` (network-bound `FilterHook`
  behaviour; already partially covered by
  the 1.6.4 test in
  `spiral-network/tests/net_surface.rs`).

Total projected: ~30 lib tests + ~10 integration
tests for `spiral-filter`.

---

## Fork 2 — process-global `FilterHook` (ADR 0005)

`spiral-filter` was the original home of the three
types `FilterHook`, `Decision`, and `Party`. The
audit of 2026-06-16 flagged this as a dep-arrow
violation: `spiral-network` needed to call
`FilterHook::decide` for every outbound request,
and the only way to get that type was to depend
on `spiral-filter`. That pulled in the whole
HTML/CSS/rule-DSL dep graph into the network
crate, which is wrong-shaped.

**ADR 0005** ([`docs/decisions/0005-filter-hook-architecture.md`](../decisions/0005-filter-hook-architecture.md))
moved the three types to `spiral-core` and
re-exported them from `spiral-filter` for
backwards compatibility. The current state:

- `spiral_core::FilterHook` + `Decision` + `Party`
  are the canonical definitions.
- `spiral_filter::lib.rs` does
  `pub use spiral_core::{Decision, FilterHook, Party};`
  so existing call sites in
  `spiral-filter/src/runtime/mod.rs` and
  downstream consumers keep working.
- `spiral-network` depends on `spiral-core` (not
  `spiral-filter`); `spiral-filter` is a
  `dev-dependency` only (for the integration test
  in `tests/`).

The process-global `FilterHook` is the
`spiral_filter::runtime::Filter` default
singleton (set via `set_global_filter` /
read via `current()`). See
`crates/spiral-filter/src/runtime/mod.rs:50-80`.

---

## URL host extractor (`match_url`)

The runtime's per-request decision path is:

```
Client::request(url)
  → FilterHook::decide(ctx)
  → Filter::should_block(url)
  → match_url::extract_host(url)
  → hostname trie lookup
  → Decision::Allow | Decision::Block
```

`match_url::extract_host(url: &str) -> Option<String>`
extracts the host from a URL string. It is
the boundary between "raw URL string from the
network stack" and "host key for the
hostname-trie index". It handles the common
cases (`scheme://host[:port][/path][?query][#frag]`)
and returns `None` for non-hierarchical URLs
(`data:`, `javascript:`, `about:`, malformed
input). See
`crates/spiral-filter/src/runtime/match_url.rs:22-50`.

---

## Object-safety rationale

`FilterHook` is **not object-safe**: it returns
`Decision` (an owned enum) by value and uses
no `&self` or `Box<dyn>` patterns. Consumers
take it by generic bound
(`fn decide(&self, hook: &impl FilterHook, ...)`),
not as `Box<dyn FilterHook>`. This matches
the "native trait, generic-bound consumer" pattern
used by `Resolver` in `spiral-net` (see
[ADR 0004](../decisions/0004-resolver-trait-async-design.md))
and is the project's default for trait design.

---

## Do-not-touch zones (current)

- The `Action` enum variants. Adding a variant is
  a breaking change.
- The `Matcher` shape. Changing the matcher API is
  a breaking change.
- The default-policy constant. Changing what
  "worst offenders only" means requires an ADR.
- The `FilterHook` / `Decision` / `Party` types
  in `spiral-core` (post-ADR 0005). These are
  the wire format for the network boundary;
  moving them again requires another ADR.

---

## Related

- `docs/glossary.md` — the unbranded entry (and the
  "Spirit" priority clarification).
- `AGENTS.md` § `spiral-filter` (project operating contract).
- `docs/architecture/design/filter-rule-model.md` — the rule
  model design.
- `docs/audits/2026-06-15-baseline.md` §1.7 — the
  M4.5 priority for `spiral-filter` runtime
  (now Packet 1.6.4; **shipped**).
- [`docs/decisions/0005-filter-hook-architecture.md`](../decisions/0005-filter-hook-architecture.md) —
  the architecture decision that moved
  `FilterHook` / `Decision` / `Party` to
  `spiral-core`.
