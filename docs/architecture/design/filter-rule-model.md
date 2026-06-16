# Design — `spiral-filter` Rule Model

**Status:** design pass complete (2026-06-15), user-approved
**Author:** implementer agent (synthesis from research passes)
**Crate:** `crates/spiral-filter/`
**Phase:** M4 (skeleton + surgical default policy)
**Depends on:** `spiral-dom`, `spiral-css`, `spiral-net`

---

## 1. Purpose

`spiral-filter` is Spiral's compile-time HTML/CSS policy engine. It sits
between the network layer and the HTML parser. It receives raw HTTP body
bytes, parses HTML+CSS, and produces a *transformed* document with
worst-offender ads removed or constrained. The runtime never sees the
offending markup.

This is structurally different from uBlock Origin, which blocks at
runtime after the page has paid the cost. Spiral avoids the cost
entirely.

---

## 2. Rule Taxonomy

Every rule is one of three *kinds*:

| Kind | Trigger | Consumer | Layer |
|------|---------|----------|-------|
| **Network** | Outgoing subresource fetch (request URL) | `spiral-net` before connection | L1 |
| **Cosmetic** | DOM element already in the tree | `spiral-fmt` (parse-time) + the user-origin stylesheet (in `spiral-fmt::css`) | L2 + L3 |
| **Policy** | CBA-derived page-level threshold | `spiral-fmt` + the user-origin stylesheet (layout-aware evaluation) | L2 + L5 |

> **Pre-rename note.** The original table said
> "`spiral-html` + `spiral-css`". `spiral-html`
> was retired in Phase 1 Step 1.2 (2026-06-15) and
> its HTML parser is now in `spiral-fmt::html`;
> `spiral-css` is a deprecated shim that forwards
> to `spiral-fmt::css::*` (per ADR 0001). The
> "Cosmetic" and "Policy" rows should both read
> "`spiral-fmt`" today.

---

## 3. Layered Filtering Model

The five layers, in execution order:

| Layer | When | What | Cost |
|-------|------|------|------|
| **L1 — Network filter** | HTTP request | Drop ad/tracker requests; rewrite/redirect | Cheap, network-only |
| **L2 — Parse-time HTML filter** | Between network and HTML parser | Strip elements whose selectors are deterministic and in the response body | Cheap, runs once |
| **L3 — User-origin CSSOM sheet** | Document start | Inject compiled cosmetic rules into user-origin `StyleSheet` | One-time, free at runtime |
| **L4 — Inline CSP rewrite** | Response header | Spiral-owned `$csp`-equivalent filters to disable inline script on selected paths | Cheap, runs once |
| **L5 — Runtime MutationSink** | After parser, on DOM mutations | Apply procedural filters to script-injected content | Only ongoing cost; batched per frame |

**L1 + L2 are the performance win.** They make the brand promise
"the filter is a performance optimisation" defensible. L3 is the
correctness backstop for elements that slip through L2. L5 is the
last resort for script-injected content.

---

## 4. Rule AST

### 4.1 Top-level rule

```rust
pub struct Rule {
    pub id: RuleId,              // 64-bit seahash; stable; dedup + logging
    pub kind: RuleKind,          // Network | Cosmetic | Policy
    pub matcher: Matcher,
    pub action: Action,
    pub severity: Severity,
    pub source: Source,
    pub stewardship: Stewardship,
}

pub enum RuleKind { Network, Cosmetic, Policy }
```

### 4.2 Matcher

```rust
pub enum Matcher {
    Network(NetworkMatcher),
    Cosmetic(CosmeticMatcher),
    Policy(PolicyMatcher),
}

pub struct NetworkMatcher {
    pub pattern: NetPattern,
    pub request_kinds: RequestKinds,   // bitflag of ResourceKind
    pub party: Party,                  // Any | First | Third
    pub match_case: bool,
    pub hostname_anchor: bool,
    pub left_anchor: bool,
    pub right_anchor: bool,
    pub domains: DomainConstraint,
}

pub enum NetPattern {
    Plain(String),                  // hot path — bucket-indexed by 8-byte tokens
    Regex(Regex),                   // only when /.../ form is used
    HostnameAnchor(String),         // bucket-indexed by hostname hash
    Empty,
}

pub struct CosmeticMatcher {
    pub selector: CompiledSelector,
    pub hostname_scope: HostnameScope, // Generic | Hosted(HashSet<Hash>) | Negated(...)
    pub unhide: bool,                  // true for #@#
}

pub enum CompiledSelector {
    Css(SelectorList<SimpleSelector>),
    Procedural(Vec<ProcOp>),
}

pub enum ProcOp {
    HasText(String),
    Has(SelectorList<SimpleSelector>),
    XPath(String),
    MinTextLength(usize),
    WatchAttr,
    Upward(SelectorList<SimpleSelector>),
}

pub struct PolicyMatcher {
    pub shape: PolicyShape,
    pub context: PolicyContext,
}

pub enum PolicyShape {
    StickyHeightPercent(u8),
    AdDensityPercent(u8),
    FixedAutoPlayMedia,
    Scrollover,
    FlashRate(f32),
    PreRollSeconds(u16),
    NonSkippablePreRoll,
    MidRollShortForm,
    Popunder,
    InterstitialMs(u32),
}

pub enum PolicyContext {
    Desktop, MobileWeb, ShortFormVideo, MobileApp, Any,
}
```

### 4.3 Action

```rust
pub enum Action {
    Block,                                  // drop the request
    Allow,                                  // pass through
    Exception,                              // @@ — cancel a Block
    Hide { css: String },                   // cosmetic CSS rule
    Remove,                                 // strip from DOM entirely
    Scriptlet { name: String, args: Vec<String>, body: String },
    Csp { policy: String },                 // inject CSP header
    RemoveParam { name: String },           // $removeparam
    Redirect { token: String, priority: i32 },
    BlockAndReport,                         // block + log to audit
}
```

### 4.4 Severity

```rust
pub enum Severity {
    WorstOffender,  // CBA "least preferred" — on by default
    Annoying,       // common consumer complaints
    Privacy,        // tracker, no visual intrusion
    Spec,           // hard CBA/W3C violation
    Critical,       // malware, fingerprinting, exploit kit
}
```

### 4.5 Source

```rust
pub struct Source {
    pub list: SourceList,
    pub origin_url: Option<String>,
    pub last_seen_unix: u64,
    pub version_hash: u64,
    pub format: SourceFormat,
}

pub enum SourceList {
    DefaultSteward,            // compiled-in, CBA-derived, zero network
    SpiralCurated,             // Spiral's own curated list
    UserSubscribed { name: String, license: String },
    UserCustom,                // user-added via settings
    RuntimeGenerated,          // derived from policy evaluation
}

pub enum SourceFormat {
    AdblockPlus,
    Hosts,
    Native,
}
```

### 4.6 Stewardship

```rust
pub enum Stewardship {
    Untracked,
    Stewarded { steward_id: StewardId },
    Offender { offence_count: u32 },
    UserPinnedAllow,
    UserPinnedBlock,
}

pub struct StewardId(pub u32);
```

### 4.7 Worked example — CBA "Large Sticky Ad"

```rust
Rule {
    id: hash64("cba:desktop/large-sticky:height>30"),
    kind: RuleKind::Policy,
    matcher: Matcher::Policy(PolicyMatcher {
        shape: PolicyShape::StickyHeightPercent(30),
        context: PolicyContext::Desktop,
    }),
    action: Action::Remove,
    severity: Severity::WorstOffender,
    source: Source {
        list: SourceList::DefaultSteward,
        origin_url: None,
        last_seen_unix: 0,
        version_hash: 1,
        format: SourceFormat::Native,
    },
    stewardship: Stewardship::Untracked,
}
```

---

## 5. CBA Thresholds (the default policy)

The Coalition for Better Ads standard defines the exact thresholds
that Spiral's default "worst offenders only" policy is derived from.

### Desktop

| Experience | Threshold |
|------------|-----------|
| Pop-up Ads | New window the user did not request |
| Auto-playing Video with Sound | Audible audio, no user initiation |
| Prestitial with Countdown | Full-page ad before content, countdown ≥ 5 s |
| Large Sticky Ad | `position: fixed`, height > 30% viewport |
| Ad Density > 50% | Σ(ad heights) / main content height > 50% |
| Ad Density > 30% + Sticky Video | Same ratio > 30% with a sticky video ad |

### Mobile Web

| Experience | Threshold |
|------------|-----------|
| Pop-up Ads | Interstitials or new windows |
| Prestitial Ads | Full-page ad > 30% viewport |
| Ad Density > 30% | Same ratio, 30% threshold |
| Flashing Animated Ads | Animation flashes > 3 Hz |
| Auto-playing Video with Sound | Same as desktop |
| Full-screen Scrollover | Full viewport takeover on scroll |
| Large Sticky Ad | Sticky > 30% viewport |
| Sticky Pop-out Video | Inline video pops out to sticky |

### Short-Form Video

| Experience | Threshold |
|------------|-----------|
| Non-skippable pre-roll > 6 s | Pre-roll that cannot be skipped, > 6 s |
| Non-skippable pre-roll > 30 s | Skippable but > 30 s |
| Mid-roll in < 2 min content | Mid-roll ads in short-form video |
| Large display overlay | > 30% player area for > 10 s |

---

## 6. EasyList Integration

### 6.1 Spiral's approach — fully custom

Spiral writes its own ABP/EasyList parser from scratch. No `adblock`
crate dependency. Our tech where it matters.

**Phase 1 (M4–M8):** hand-roll the ABP parser for the *subset*
Spiral cares about — basic network rules (`||host^`, path patterns),
basic `##` cosmetic rules, `#@#` exceptions, hostname-anchored
rules, and `$third-party` / `$domain=` modifiers. No `+js()`
scriptlets in v0.1.

**Phase 2 (M10–M18):** extend to the full ABP + uBO superset —
procedural cosmetic operators (`:has-text`, `:xpath`, `:upward`),
`$csp`, `$redirect`, `$removeparam`, regex patterns.

The parser is ~2,000–3,000 lines of Rust. The grammar is well-
documented (ABP wiki, uBO wiki). Getting it right is a matter
of fixture-driven development — 500+ real-world EasyList lines
as test fixtures.

### 6.3 Network vs cosmetic split in Spiral

| Concern | Layer | Crate | When |
|---------|-------|-------|------|
| Network filter rules | L1 | `spiral-net` | Before connection |
| Cosmetic CSS rules | L2 + L3 | `spiral-fmt` (parse-time + user-origin stylesheet) | Parse-time |
| Procedural operators | L5 | `spiral-filter::runtime` | DOM mutation callback (Phase 2+) |

---

## 7. Cosmetic CSS Injection

### 7.1 Mechanism

The user-origin stylesheet sits *above* author CSS in W3C CSS
Cascade Level 4 ordering:

```
important UA > important user > important author > animations >
normal author > normal user > normal UA
```

Spiral's user-origin `StyleSheet` with `display: none !important`
reliably wins against page CSS that sets `display: block`.

### 7.2 Where the stylesheet lives

Per-context (each origin has its own CSSOM and its own cosmetic
sheet built from the filter rule store + per-site bucket). The
*rules* are compiled once at filter-list load time and shared
process-wide via the Gyre style cache.

### 7.3 CSP immunity

A page's Content-Security-Policy cannot override user-origin
cosmetic rules. CSP governs script and resource loading, not
CSS-authoring origin. The user-origin stylesheet is an internal
object in the cascade; CSP is enforced at the network layer and
cannot reach into Gyre's cascade.

### 7.4 The anti-fight pattern

Pages can set `el.style.display = "block"` via inline style,
which beats any selector (inline styles are higher specificity
than any selector). Mitigation:

1. Parse-time removal (L2) — the element never exists.
2. User-origin `display: none !important` (L3) — beats normal
   author CSS.
3. Runtime `MutationSink` + `el.remove()` (L5) — catches the
   case where inline style defeats L3.

This is the uBO pattern. It works.

---

## 8. Runtime DOM Mutation Hook

### 8.1 The Rust-side `MutationSink` advantage

Because Spiral owns the DOM (`spiral-dom`), the filter engine
does not need a JS `MutationObserver`. It gets mutations as Rust
events from the DOM `NodeStore` mutators:

- `Element.append_child`, `remove_child`, `set_attribute`,
  `insert_before`, `replace_with`, `inner_html_setter` — all
  funnel through a `MutationSink` trait.

This has near-zero per-mutation overhead: no string event encoding,
no observer registration, no DOM-walking to find the observer.

### 8.2 Batching

Mutations are accumulated into a per-frame `Vec<MutationRecord>`
and drained on the next animation frame or microtask boundary.
Matching runs once over the union of all added nodes — never
per-record.

### 8.3 Attribute observation

Do NOT observe attributes globally. Observe `childList` + `subtree`
only. Gate attribute observation on the `:watch-attr` procedural
operator. uBO explicitly chose this and documented why: attribute
observation is expensive and rarely needed.

### 8.4 Shadow DOM

Closed shadow roots are out of scope for v0.1. Cosmetic filters
reach the light DOM only. Document as a known gap. Revisit in
Phase 3+.

---

## 9. Stewardship Model

### 9.1 The "good web steward" contract

The stewardship score lets Spiral honour the user's stated policy:
"Block the worst ads, allow reasonable ones, reward good stewards."

- **Positive stewardship** (self-attested to Better Ads Standards):
  rules with severity ≤ Annoying are downgraded to `Allow`.
- **Negative stewardship** (on the violation list): even
  low-severity rules get promoted to `Block`.
- **User override**: `UserPinnedAllow` / `UserPinnedBlock` wins
  regardless.

### 9.2 Authority model

- Seed with CBA "Better Ads Standards" as the built-in default
  steward list.
- Overlay a curated top-100 worst-offender list (Spiral's own).
- Community contributions from M18+.
- Site-owner stewardship registry: opt-in, attestation to Better
  Ads Standards. The bar is real, not pay-to-play.

### 9.3 User-tunable slider

A slider from "block nothing" to "block almost everything."
Default = "worst offenders only." The slider maps to a severity
threshold:

| Slider position | Minimum severity blocked |
|----------------|--------------------------|
| Off | (none) |
| Worst offenders only | `WorstOffender` |
| Common annoyances | `Annoying` |
| Privacy-focused | `Privacy` |
| Strict | `Spec` |
| Maximum | `Critical` |

---

## 10. Crate Structure

```
crates/spiral-filter/
├── Cargo.toml
├── src/
│   ├── lib.rs                     # public facade
│   ├── error.rs                   # parse / apply errors (thiserror)
│   ├── rule.rs                    # Rule, RuleKind, Matcher, Action, Severity, Source, Stewardship
│   ├── syntax/
│   │   ├── mod.rs                 # EasyList + uBO static syntax entry points
│   │   ├── cosmetic.rs            ## and #@# parsers
│   │   ├── network.rs             # ||host^$type,domain= parsers
│   │   └── procedural.rs          # :has(:has-text(...)) operator tree
│   ├── compile/
│   │   ├── mod.rs                 # CosmeticFilter -> spiral-css::StyleRule
│   │   ├── trie.rs                # HostnameTrie
│   │   └── user_stylesheet.rs     # Origin::User sheet builder
│   ├── runtime/
│   │   ├── mod.rs                 # Filter struct (per-context; Packet 1.6.4 SHIPPED)
│   │   ├── match_url.rs           # URL host extractor (Packet 1.6.4 SHIPPED)
│   │   ├── sink.rs                # MutationSink adapter on spiral-dom (Phase 2+)
│   │   └── procedural.rs          # JS-free procedural matcher (Phase 2+)
│   ├── lists/
│   │   ├── mod.rs                 # bundled filter lists
│   │   ├── cba_defaults.toml      # CBA thresholds, human-editable
│   │   └── steward.rs             # opt-in stewardship registry
│   └── policy/
│       ├── mod.rs                 # user slider
│       └── default.rs             # "worst offenders only" default
└── tests/
    ├── parse_test.rs              # EasyList syntax fixtures (Phase 5+)
    ├── apply_test.rs              # apply filters to a DOM, assert removed (Phase 5+)
    ├── procedural_test.rs         # :has, :has-text, :upward (Phase 2+)
    └── mutation_test.rs           # simulate DOM add, assert runtime hides (Phase 2+)
```

> **Cosmetic runtime is Phase 2+ future work.**
> The `runtime/mod.rs` "CosmeticRuntime" referenced
> in the original tree-builder plan above is **not
> part of the 1.6.4 runtime**. Packet 1.6.4 ships
> the network filter (`Filter` struct +
> `match_url::extract_host`); the `CosmeticRuntime`
> (per-context cosmetic-filter state, with
> `MutationSink` and procedural matcher) lands
> in Phase 2+. The current `runtime/mod.rs`
> contains `Filter` + `match_url` only.

### 10.1 Dependencies

```toml
[dependencies]
spiral-core = { workspace = true }
spiral-dom = { workspace = true }
spiral-css = { workspace = true }
spiral-net = { workspace = true }
thiserror = { workspace = true }
seahash = "4"

[dev-dependencies]
# test fixtures — 500+ real-world EasyList lines
```

No `adblock` crate dependency. All rule parsing is Spiral-native.

### 10.2 M4 skeleton scope

For the M4 sprint, the skeleton ships:

- `rule.rs` — the full Rule AST (types only, no matching logic).
- `syntax/cosmetic.rs` — parse a basic `##` cosmetic rule.
- `syntax/network.rs` — parse a basic `||host^` network rule.
- `lists/cba_defaults.toml` — CBA thresholds as a human-editable file.
- `policy/default.rs` — the "worst offenders only" severity gate.
- `compile/trie.rs` — `HostnameTrie` skeleton.
- Tests for each public function.

Deferred to M5+: the `runtime/` module, the procedural operator
evaluator, the `MutationSink` adapter.

---

## 11. Open Questions

1. **CBA steward list as checked-in file.** `lists/cba_defaults.toml`
   is human-editable; build-time compilation produces the binary form.
   Recommend build-time generation so thresholds stay editable.

3. **Stewardship update channel.** Hardcoded for v0.1, sidecar file
   from v0.2. No telemetry.

4. **Script-injection gating.** The JS-intercept shim (M6+) lives in
   `spiral-vortex` (Vortex realm init) but consults `spiral-filter`
   over a host call. Crate boundary confirmed.

5. **Audit log.** The `BlockAndReport` action needs a sink. Recommend
   a new `spiral-audit` crate (M18+) so `Severity::Critical` rules
   can integrate with managed reporting without churn in `spiral-core`.

6. **Open shadow root cosmetic filtering.** Out of scope for v0.1.
   Revisit in Phase 3.

7. **Closed shadow root cosmetic filtering.** Out of scope permanently
   (architecturally impossible without a JS-in-shadow world).
   Document as a known gap.
