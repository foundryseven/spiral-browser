# `spiral-filter` (Filter) — Compile-time Ad & Policy Filter

> **Brand:** *(unbranded; "Spirit" in the priority
> taxonomy is unrelated — see `docs/glossary.md`.)*
> **Crate:** `spiral-filter`. **Scope:** URL pattern
> matchers, CSS-selector matchers, element-attribute
> matchers, action enum (block / hide / constrain /
> allow). **Status:** M4.4 crate skeleton in place;
> runtime hook is M4.5 Item 12.

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

pub struct FilterEngine { … }
impl FilterEngine {
    pub fn new(policy: &Policy) -> Self;
    pub fn should_block(&self, url: &Url) -> Decision;
    pub fn transform(&self, dom: &mut Dom, stylesheet: &mut Stylesheet);
}
```

The M4.4 skeleton has the **rule model types** but
not the runtime. M4.5 Item 12 wires the engine into
the network boundary as a no-op; the M5+ work fills
in the actual rule evaluation.

---

## Internal layout

```
spiral-filter/src/
├── lib.rs           — public surface
├── rule.rs          — Rule, Matcher, Action, RuleKind
├── policy.rs        — Policy, default-policy constant
├── syntax.rs        — rule syntax (the user-facing DSL) (M5+)
├── lists.rs         — built-in filter lists (M5+)
├── compile.rs       — compile-time rule evaluation (M5+)
├── runtime.rs       — runtime hook (Decision::Allow / Block) (M4.5)
└── error.rs         — FilterError
```

The M4.4 skeleton has the rule model types and the
error type; M5+ adds the rest.

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
  (constructor, match, action enums). The runtime
  is M4.5+ work; the runtime tests land with it.
- M4.5 Item 12 adds the runtime-hook test
  (allow-by-default, block-by-explicit-rule).
- M5+ adds the syntax tests, the policy tests,
  and the integration tests with
  `spiral-network`.

Total projected: ~30 lib tests + ~10 integration
tests for `spiral-filter`.

---

## Do-not-touch zones (M4.4)

- The `Action` enum variants. Adding a variant is
  a breaking change.
- The `Matcher` shape. Changing the matcher API is
  a breaking change.
- The default-policy constant. Changing what
  "worst offenders only" means requires an ADR.

---

## Related

- `docs/glossary.md` — the unbranded entry (and the
  "Spirit" priority clarification).
- `AGENTS.md` § `spiral-filter` (forthcoming).
- `docs/design-filter-rule-model.md` — the rule
  model design.
- `docs/audits/2026-06-15-baseline.md` §1.7 — the
  M4.5 priority for `spiral-filter` runtime.
