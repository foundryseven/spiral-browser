# Spiral Browser — Gap Analysis (Spec)

> **⚠️ Spec-only as of 2026-06-16.** The `[x]` / `[~]` / `[ ]` / `[!]`
> status markers have been removed from the per-item tables in this
> document; the live status lives in
> [`docs/implementation_tracker.md`](../docs/implementation_tracker.md)
> (Group → Phase → Step → Packet). Per
> [`docs/audits/2026-06-16-doc-drift.md`](../docs/audits/2026-06-16-doc-drift.md)
> P1 #13, this is a spec-only document; the per-row "Item / Notes"
> content is retained for traceability, but the "Status" column is
> now empty (the live status is in the tracker).

**Author:** autonomous architect pass
**Date:** 2026-06-15
**Status:** spec-only (status moved to
[`docs/implementation_tracker.md`](../docs/implementation_tracker.md))
**Last SSOT sync:** `docs/implementation_tracker.md`, `docs/active_context.md`

**Related artifacts (each with one job):**
- `docs/audits/2026-06-15-baseline.md` — functional baseline audit
- `docs/audit-sprint-m4.md` — originality / license / novelty audit
- `docs/baseline-warnings.md` — `cargo check --workspace` warning drift baseline

This document is the **spec** — what the browser is supposed to do across
the four engine sub-domains. It is **not** the status tracker. Status
(what is built, in flight, missing) lives in
[`docs/implementation_tracker.md`](../docs/implementation_tracker.md).
Per the SSOT restructure of 2026-06-16, the deltas that previously lived
in this file are now entries in the implementation tracker, the progress
ledger, and the active context.

The `Status` column in the tables below is **historical** (as of the
2026-06-15 audit pass) and is kept for spec traceability only. Do not
update it. The current state is in the tracker.

---

## 0. Headline Verdict

| Domain | Coverage | Comment |
|--------|----------|---------|
| Core Engines (HTML/CSS/DOM/Layout/Render/JS) | **~30%** | Foundation present; engine layers are scaffolds. |
| Networking & Storage (HTTP/TLS/DNS/cookies/cache) | **~5%** | Stubs only. Cookies, LocalStorage, IndexedDB, OPFS absent. |
| Presentation Layer (chrome UI, tabs, URL bar) | **~15%** | `spiral-ui` stubbed; tabs+URL bar not built. |
| Cross-Cutting (SOP/CSP/HSTS/sandbox/WebGL routing) | **~2%** | Almost nothing. |

**Test posture:** 272 tests across 18 crates, 6 failing in
the retired `spiral-html` shim (html5ever 0.39.0 tree_builder panic —
blocking). Rest of workspace green. **Closed in Step 1.5
(SSOT restructure) of 2026-06-16**: `spiral-html` was retired;
`spiral-fmt::html` is the sole HTML parser; the 6 panics never existed
in the `spiral-fmt` code path. See
[`docs/progress_ledger.md`](../docs/progress_ledger.md) for the
restructure entry.

**Single biggest blocker (as of audit):** the `spiral-fmt` crate did not exist.
Phase 1 Step 1.3/1.5 deliverable; the entire foundation of HTML/CSS parsing
depends on it. Status: shipped 2026-06-16.

**Single biggest active defect:** `spiral-crypto::sha256` returns
32 zero bytes (not a placeholder — a security bug; `taffy` was never
in tree per mandate but `rustls` is imported yet unused).

**Competitive-parity research (2026-06-16):** 20 new P2 sprint items
were identified by the competitive-parity research, covering
HTML tree-builder depth (adoption agency, active formatting elements,
foster parenting, fragment parsing) and DOM IDL surfaces (NodeList,
HTMLCollection, DOMTokenList, Attr, dataset, structuredClone, URL). 1
item re-ranked (the `<template>` content work). These are mapped onto
[`docs/implementation_tracker.md`](../docs/implementation_tracker.md)
§ Phase 2 — Engines Depth.

---

## 1. Domain 1 — Core Engines

### 1.1 HTML parser (`spiral-html`)

| Item | Status | Notes |
|------|--------|-------|
| `spiral-fmt` vendored parser crate | **Does not exist on disk.** M4.4 next sprint. |
| `html5ever 0.39.0` upstream integration | **6 tests panic** in `tree_builder/mod.rs:685` "no current element". Code at `crates/spiral-html/src/lib.rs:82-318` uses an outdated `TreeSink` shape; html5ever 0.39 expects stricter sink semantics. |
| `parse_document` → `spiral-dom::Dom` pipeline | Wired but panics. |
| `markup5ever = "0.39"` and `tendril = "0.5"` direct deps in `spiral-html/Cargo.toml:12-13` | M4.4 calls for eliminating these by routing through `spiral-fmt`. |
| DOCTYPE handling (`append_doctype_to_document`) | Wired but emits a comment node (DOM has no DOCTYPE variant). Lossy. |
| `<template>` element fragment handling (`get_template_contents`) | Returns the element itself (no document fragment). Lossy. |
| Adoption agency, foster parenting, format-extracted-character references | Not exercised by current tests. |
| Encoding detection (BOM, `<meta charset>`, content sniff) | Assumes UTF-8 only. |
| Character reference / entity decoding | Not implemented; relies on `StrTendril` and html5ever internals. |

### 1.2 CSS parser & cascade (`spiral-css`)

| Item | Status | Notes |
|------|--------|-------|
| `spiral-fmt` unified CSS parser | M4.4. |
| `cssparser 0.37` + `selectors 0.38` upstream | Compiles; 5 tests pass. |
| Cascade (origin order, specificity, `!important`, inheritance) | Designed for M8; not implemented. |
| `parse_selector` round-trip | 4 selector variants tested. |
| Vendor-prefix handling | Unspecified. |

### 1.3 DOM (`spiral-dom`)

| Item | Status | Notes |
|------|--------|-------|
| `Node` (Element, Text, Comment, Document) | 13 tests pass. |
| Parent/child via indices | |
| Attribute storage `Vec<(String, String)>` | |
| Arena allocation `Vec<Node>` + indices | |
| `insert_before` / `replace_child` | `spiral-html` open-codes insert-before via remove+re-append. DOM API itself lacks it (`spiral-html/src/lib.rs:243-253` admits "Dom doesn't have insert-before"). |
| `<template>` content document fragment | |
| DOCTYPE node variant | |
| Mutation events / observer API | |
| `getElementById`, `querySelector` | |

### 1.4 Layout — Gyre (`spiral-gyre`)

| Item | Status | Notes |
|------|--------|-------|
| Box model (`margin`, `border`, `padding`, `content`) | Type defined. |
| Block layout: vertical stacking | 1 trivial pass; no margin collapse, BFC, IFC. |
| Margin collapse (positive/negative/nested) | Not implemented. |
| BFC / IFC | |
| Floats (left/right, clear, BFC containment) | |
| Positioning (static, relative, absolute, fixed, sticky) | |
| Flex (M10–11) | |
| Grid (M13–14) | |
| WPT pass rate | 0%. |
| `LayoutEngine` ignores the stylesheet | Signature takes `&Stylesheet` but `_stylesheet` parameter is unused (`spiral-gyre/src/lib.rs:65`). CSS is structurally disconnected from layout. |
| Text shaping integration (harfrust) | M9. |
| Inline / line layout | |
| Lazy / incremental relayout | Bet 4 work, M30+. |

### 1.5 Render (`spiral-render` + `spiral-paint`)

| Item | Status | Notes |
|------|--------|-------|
| Software rasteriser (`SoftwareRenderer`) | Display-list walker; fill, stroke, text, clip, transform, layers. 14 tests. |
| 5×7 bitmap font (ASCII 0x20–0x7E) | |
| RGBA8 → PNG (`encode_png`) | |
| `spiral-paint` display list + compositing | Phase 4 "do not touch" per active_context. |
| Vello / wgpu GPU path | Phase 4. |
| `spiral-gpu` abstraction | Phase 4. |
| Vello fork with picture cache | Phase 4. |
| Display list built from layout output | `spiral-browser` builds the hello-world display list by hand; no glue from layout → paint. |

### 1.6 Vortex — JavaScript engine (`spiral-vortex`)

| Item | Status | Notes |
|------|--------|-------|
| Lexer (full ECMAScript tokens) | |
| Parser (recursive descent + Pratt) | |
| AST (ES2015+ node types) | |
| Tree-walking interpreter (Phase A) | |
| `console.log`/`info`/`warn`/`error` | Not yet wired to `RendererToBrowser::ConsoleMessage` IPC. |
| Event loop (microtask/macrotask, setTimeout/setInterval) | Wired in-process; no integration with browser event loop. |
| Builtins (`Object`, `Array`, `String`, `Number`, `Boolean`, `Math`, `JSON`) | Partial. |
| Vortex GC rewrite (per-origin arenas, TaggedCell, GcKey) | 84 GC tests. |
| `VortexHeap` ↔ `Runtime` glue | Fixed `gc_live_count` bug in audit pass. |
| Bytecode VM (Phase B) | M10–24. |
| Mark-sweep from real stack+globals roots | Roots from environment chain; not yet from VM stack. |
| Closures / `this` / prototype chain | Partial in interpreter; design only for bytecode. |
| ES2015+ syntax (let/const, arrow, classes, spread) | Partial. |
| `rusty_v8` oracle behind `v8` feature | Feature-gated. CI compliance path open. |
| DOM bindings (`createElement`, `appendChild`, `setAttribute`, `addEventListener`) | `dom_bindings/mod.rs` is a stub. |
| Test262 compliance | 0%; ~5–10% target for end of M9. |
| Build warnings | 12+ dead-code warnings (`interval_ms`, etc.). Not blocking but noisy. |

### 1.7 Shared-Everything Multi-Process (Bet 1)

| Item | Status | Notes |
|------|--------|-------|
| `spiral-context` capability types (brands, tokens) | M4 skeleton. 21 tests. |
| `Origin`, `CapabilitySet`, `Context`, `ContextOps` | |
| Real runtime (per-origin contexts in one process) | M25. |
| `InProcess` vs `Escalated` modes | Types only. |
| Integration with Vortex heap / Gyre / parser | None yet. |

---

## 2. Domain 2 — Networking & Storage

### 2.1 HTTP / TLS / DNS

| Item | Status | Notes |
|------|--------|-------|
| `spiral-network` HTTP client via `hyper` | Stub at `crates/spiral-network/src/lib.rs:24-74`. `get`/`post` return 200 with empty body. No actual hyper call. **Pulled forward to P3 (2026-06-16, see Delta 7).** |
| `spiral-net` DNS resolver via `hickory-dns` | Stub at `crates/spiral-net/src/lib.rs:25-56`. `resolve` returns `["127.0.0.1"]`. No `hickory_resolver::TokioResolver` integration. |
| `spiral-net` TLS via `rustls` | `TlsConfig` struct exists (`spiral-net/src/lib.rs:8-17`) but unused. No `rustls::ClientConfig` glue. |
| `spiral_net::Resolver` trait | M4 deliverable; not done. |
| `spiral_net::TlsConnector` trait | M5 deliverable. |
| `spiral_network::Client` trait | M6 deliverable. |
| Connection pooling | |
| Redirect policy | |
| HTTP/2 (h2, h2c) | |
| HTTP/3 (quinn) | |
| WebSockets | |
| WebRTC | |
| Speculative caching / preload (prerender, prefetch) | |
| HSTS preload list | |
| Certificate pinning | M5 stub. |

### 2.2 Cryptography (`spiral-crypto`)

| Item | Status | Notes |
|------|--------|-------|
| `Crypto::random_bytes` | Returns `((i % 256) as u8)` — **deterministic, NOT random**. `crates/spiral-crypto/src/lib.rs:18`. Security bug. |
| `Crypto::sha256` | Returns 32 zero bytes. **Not a hash**. `crates/spiral-crypto/src/lib.rs:24`. Security bug. |
| `rustls` dep declared but unused | `spiral-crypto/Cargo.toml:11`. Dead dep. |
| HKDF, HMAC, Ed25519 | |
| `SecureRandom` trait wrapping `getrandom` | Iteration-options Option E. |
| Subresource Hash integrity (SRI) | |
| WebCrypto API | M30+. |

### 2.3 Storage

| Item | Status | Notes |
|------|--------|-------|
| Cookie jar | Not present. **Pulled forward to P3 (2026-06-16, see Delta 7).** |
| LocalStorage | |
| SessionStorage | |
| IndexedDB | |
| OPFS (Origin Private File System) | |
| CacheStorage (Service Workers) | |
| Quota management / eviction | |
| Storage partitioning (Bet 1) | M30+. |
| Origin-keyed encryption (Bet 1, capability) | |
| `spiral-storage` crate | Not in workspace. Should exist. |

### 2.4 Image decoding (`spiral-imagedecoder`)

| Item | Status | Notes |
|------|--------|-------|
| Format sniffing (PNG/JPEG/WebP/AVIF) | Magic-byte detection works. |
| PNG decode | Returns 1×1 white pixel. No real decode via `png` crate. |
| JPEG decode (`zune-jpeg`) | Same placeholder. |
| WebP decode | Same. |
| AVIF decode (`ravif`) | Same. |
| `Decoder` enum dispatch per format | Iteration-options Option E; not done. |
| Lazy / progressive loading | M19. |
| Animated images (APNG, animated WebP, animated AVIF) | |

---

## 3. Domain 3 — Presentation Layer (Chrome UI)

> **Do not touch zone** per `docs/active_context.md`: `spiral-ui`, `spiral-theme`,
> `spiral-paint`. Phase 4 work. Listed for completeness only.

| Item | Status | Notes |
|------|--------|-------|
| `spiral-theme` design tokens (Zen-style) | Phase 4; "do not touch". |
| `spiral-ui` winit window / event loop | Phase 4. |
| Sidebar tabs (create, switch, close, drag) | |
| Floating URL bar / Omnibox | |
| Navigation buttons (back/forward/reload/home) | |
| Tab context menu | |
| DevTools (full: Elements + Console + Network + Performance + Memory + Security + Application panels) | P6. **Full scope confirmed (2026-06-16, see Delta 7).** Performance, Memory, Security, Application panels are all needed for v1.0. |
| Tab Provenance Graph (innovations #12) | M12+ novelty. |
| Find-in-page | |
| Downloads UI / manager | |
| Settings panel | |
| WebExtensions API (manifest v3) | v1.0. |
| Content script injection (sandboxed) | |
| Extensions ↔ page typed message bus | |

---

## 4. Domain 4 — Cross-Cutting Concerns

### 4.1 Security / Privacy

| Item | Status | Notes |
|------|--------|-------|
| Same-Origin Policy enforcement | Not yet. |
| Content Security Policy (CSP) parser & enforcement | |
| HSTS / HSTS preload | |
| Secure cookie flags (`Secure`, `HttpOnly`, `SameSite`) | |
| Cookie partitioning (CHIPS / Storage Access API) | M18+ per roadmap. |
| Referrer-Policy | |
| Permissions Policy / Feature Policy | |
| COOP, COEP, CORP, CORS | |
| Mixed-content blocking | |
| Subresource Integrity (SRI) | |
| Mixed scripting (`X-Frame-Options`, frame-ancestors) | |
| Spectre mitigations (shared-everything model) | Designed in M4, runtime in M25. |
| Memory zeroisation for secrets | |
| OCSP / OCSP stapling | |
| Certificate transparency | |
| `spiral-sandbox` per-platform profiles | Phase 4 "do not touch"; re-evaluated under Bet 1 — capability-typed, not OS-level. |
| Linux: Landlock + seccomp-bpf | |
| macOS: Seatbelt | |
| Windows: Restricted Token + Job Object | |
| Third-party tracker blocking | `spiral-filter` is the policy engine; nothing wires it in. |
| Anti-fingerprinting posture | |
| Telemetry / phone-home (must be none) | None — by design. |

### 4.2 Filter / Ad-blocking (`spiral-filter`, Bet 3)

| Item | Status | Notes |
|------|--------|-------|
| ABP/EasyList rule AST | `rule.rs`. |
| ABP/EasyList parser (cosmetic + network) | `syntax/{cosmetic,network}.rs`. 40 tests. |
| Hostname trie compilation | `compile/trie.rs`. |
| Coalition for Better Ads thresholds | `lists/cba.rs`. Audited, corrected. |
| Policy level slider (Off / WorstOffenders / CommonAnnoyances / PrivacyFocused / Strict / Maximum) | `policy/default_policy.rs`. |
| Site stewardship score | In `Rule` type; not yet applied at runtime. |
| `spiral_filter::runtime::Filter` engine | Skeleton only. M5+. |
| Compile-time policy application (between network and parser) | Not wired in. |
| User-tunable slider UI | M12+. |
| Stewardship registry (opt-in) | M12+. |
| Network filter request matching | |

### 4.3 Graphics / Compositing (GPU)

| Item | Status | Notes |
|------|--------|-------|
| `spiral-gpu` wgpu abstraction | Phase 4 "do not touch". |
| Vello 2D scene build | Phase 4. |
| Display list → Vello scene → swap chain | |
| Tile-based picture cache (Vello fork) | Phase 4. |
| Dirty-rect invalidation | |
| WebGL binding (wgpu backend) | M49–54. |
| WebGPU binding | |
| Hardware video decode (`spiral-media`) | M30+. |
| AV1 (dav1d), VP9, HEVC, Opus, AAC | |
| Widevine CDM bridge | M36+. |
| ClearKey EME | M12. |

### 4.4 Media (`spiral-media`)

| Crate exists? | Status | Notes |
|---------------|--------|-------|
| `spiral-media` | Not in workspace. Required for v0.1 (NYT test page = text only, but YouTube/Netflix in scope). M30+ deliverable. |

### 4.5 Internationalisation / a11y / extensions

| Item | Status | Notes |
|------|--------|-------|
| ICU integration (`icu` crate) | M61–84. |
| Locale-aware text shaping | M9 base, M61–84 broad. |
| Screen reader / ARIA | M61–84. |
| Extensions API | M61–84. |

---

## 5. Other gaps (not in the four domains but observed)

### 5.1 Process / IPC

| Item | Status | Notes |
|------|--------|-------|
| `spiral-ipc` Unix + Windows transport | 16 tests. |
| `IpcTransport` trait + `MockTransport` | |
| Length-prefixed bincode framing | Fuzz-tested. |
| Per-process routing keys (renderer ↔ network ↔ GPU) | Phase 4. |
| Per-tab renderer process spawn | Phase 4; Bet 1 makes this optional. |
| `BrowserToRenderer::Hello` handshake | |
| `RendererToBrowser::ConsoleMessage` from Vortex | Vortex is in-process; no bridge yet. |

### 5.2 WPT / test infrastructure

| Item | Status | Notes |
|------|--------|-------|
| `tests/wpt/` directory | Empty. |
| HTML5 lib test subset port | Blocked on M4.4. |
| CSS parser test subset port | Blocked on M4.4. |
| Block layout WPT fixture harness | |
| Fuzz harness for parsers | IPC has one; parsers do not. |
| Cargo bench (`benches/layout/`) | Directory exists, empty. |
| Criterion benchmarks | |
| Coverage measurement (cargo-llvm-cov) | |

### 5.3 Build / CI / docs

| Item | Status | Notes |
|------|--------|-------|
| `cargo build --workspace` | |
| `cargo test --workspace` (default build) | 6 spiral-html failures block the green-CI gate. |
| `cargo test --workspace --exclude spiral-html` | 266 tests pass. |
| `cargo clippy --workspace --all-targets -D warnings` | 0 clippy errors; `spiral-vortex` has 12+ dead-code warnings. |
| `cargo fmt --all -- --check` | |
| CI matrix `ubuntu-latest` + `macos-latest` + `windows-latest` | |
| `docs/specs/` (single source of truth) | This file. |
| `docs/plans/iteration-options.md` | Maintained. |
| `docs/active_context.md` | |
| `docs/progress_ledger.md` | |
| `docs/innovations-backlog.md` | 70-idea backlog, fully audited. |
| `docs/architecture/design/shared-everything.md` | Bet 1 writeup. |

### 5.4 Stub crates that are listed as "do not touch" but should be visible

- `spiral-gpu` — exists as a stub, Phase 4.
- `spiral-paint` — exists, Phase 4.
- `spiral-ui` — exists, Phase 4.
- `spiral-theme` — exists, Phase 4.
- `spiral-sandbox` — exists, Phase 4 (re-evaluated under Bet 1).
- `spiral-vello` (fork) — does NOT exist yet. Not in workspace.

---

## 6. Priority Stack — The "Boats" That Need Filling

| # | Gap | Severity | Sprint | Why first |
|---|-----|----------|--------|-----------|
| **1** | **`spiral-fmt` doesn't exist; `spiral-html` broken (6 panicking tests)** | **P0** | **M4.4** | Blocking the test gate. Blocking the foundation for HTML/CSS work. User-mandated next step. |
| **2** | **`spiral-crypto` security bug** (sha256 = zeros, random_bytes = deterministic) | **P0** | immediate | Active security defect. No real crypto can be built on top until fixed. |
| **3** | **Track E wrappers inert** (spiral-net, spiral-network) | P1 | M4.5 | Iteration-options Option E. Quick wins. |
| **4** | **`spiral-imagedecoder` doesn't decode** | P1 | M4.5 | Real decode via `png`/`zune-jpeg`/`webp`/`ravif`. Quick win. |
| **5** | **Gyre block layout is a 209-line stub** | P1 | M4.6 | M4.6 deliverable. Margin collapse, BFC, IFC, positioning, floats. |
| **6** | **Vortex ↔ browser console pipe** | P1 | M4.x | `console.log` → `RendererToBrowser::ConsoleMessage` per M11 plan. |
| **7** | **Vortex dead-code warnings** | P2 | quick | Lint hygiene. |
| **8** | **`spiral-filter` not wired in** | P2 | M5+ | Need network + parser first. |
| **9** | **`spiral-context` not yet integrated** | P2 | M5+ | Designed in M4, runtime in M25. |
| **10** | **No `<template>` content fragment; no DOCTYPE node; no `insertBefore` in DOM** | P2 | M5+ | Lossy abstractions blocking edge cases. |
| **11** | **Cookies / LocalStorage / IndexedDB / OPFS** | P2 | M18+ | Roadmap. |
| **12** | **WebExtensions API** | P3 | v1.0 | |
| **13** | **SOP/CSP/HSTS** | P3 | M18+ | |
| **14** | **GPU / Vello / wgpu** | P3 | M25+ | Phase 4. |
| **15** | **`spiral-media` (MSE/EME/codecs)** | P3 | M30+ | |
| **16** | **WPT fixtures** | P3 | M5+ | |
| **17** | **WebGL / WebGPU** | P3 | M49+ | |
| **18** | **Sandboxing re-evaluated under Bet 1** | P3 | M25+ | |
| **19** | **Adoption agency algorithm (G1.3a)** | P2 | M4.5/M5 | Blocks correct rendering of real-world HTML with misnested formatting tags. WHATWG HTML §12.2.6.1. |
| **20** | **Active formatting elements list (G1.3b)** | P2 | M4.5/M5 | Required by adoption agency algorithm. WHATWG HTML §12.2.6.1. |
| **21** | **Foster parenting (G1.3c)** | P2 | M4.5/M5 | Blocks correct table parsing and in-table/in-body placement. WHATWG HTML §12.2.6.1. |
| **22** | **Fragment parsing algorithm (G1.3d)** | P2 | M5 | Blocks innerHTML, insertAdjacentHTML. WHATWG HTML §12.4. |
| **23** | **Quirk mode classifier (G1.3e)** | P2 | M5 | Blocks correct CSS behaviour on legacy sites. WHATWG HTML §12.1. |
| **24** | **`<noscript>` element (G1.3f)** | P2 | M5 | Blocks correct rendering with JS enabled. WHATWG HTML §4.6.7. |
| **25** | **Global attributes IDL (G1.3g)** | P2 | M5 | Blocks all DOM IDL and accessibility for `id`, `class`, `style`, `title`, `lang`, `dir`, `hidden`, `tabindex`, `contenteditable`, `inert`, `popover`. WHATWG HTML §3.2.6. |
| **26** | **`data-*` custom data attributes (G1.3h)** | P2 | M5 | Blocks `element.dataset` from JS. WHATWG HTML §3.2.6.3. |
| **27** | **`DOMTokenList` (G1.4a)** | P2 | M5 | Blocks `classList`, `relList` and other token-based DOM interfaces. WHATWG DOM §7.1. |
| **28** | **`NodeList` (G1.4b)** | P2 | M5 | Blocks querySelectorAll result handling. WHATWG DOM §4.4. |
| **29** | **`HTMLCollection` (G1.4c)** | P2 | M5 | Blocks getElementsByTagName result handling. WHATWG DOM §4.5. |
| **30** | **`Attr` interface (G1.4d)** | P2 | M5 | Blocks getAttributeNode and attribute iteration. WHATWG DOM §4.9. |
| **31** | **`NamedNodeMap` (G1.4e)** | P2 | M5 | Blocks `.attributes` on Element. WHATWG DOM §4.8. |
| **32** | **`DocumentType` (G1.4f)** | P2 | M5 | Blocks `document.doctype`. WHATWG DOM §4.6. |
| **33** | **`globalThis` (G1.6a)** | P2 | M5 | Blocks universal global reference. ECMA-262 §19.4.1. |
| **34** | **`structuredClone` (G1.6b)** | P2 | M5 | Blocks postMessage structured data and Workers. WHATWG HTML §8.2.7. |
| **35** | **`Proxy` + `Reflect` (G1.6c/d)** | P2 | M5/M6 | Blocks transparent object interception and Proxy+Reflect idiom. ECMA-262 §10.5, §28.1. |
| **36** | **`URL` + `URLSearchParams` (G1.6e)** | P2 | M5 | Blocks URL manipulation everywhere. WHATWG URL §4. |
| **37** | **`<template>` content fragment (bump to sprint)** | P2 | M4.5/M5 | Priority #10 re-ranked to active sprint item. Blocks Web Components, Shadow DOM, declarative templates. |

---

## 7. Proposed First Fill

**Gap #1: Create `spiral-fmt` and rewire `spiral-html` to use it.**

This is the user's next concrete sprint task (M4.4). It addresses both
the **P0 blocker** (6 panicking tests) and the **foundation gap**
(no vendored parser crate).

**Why not just patch `spiral-html` to work with html5ever 0.39.0?**

The active_context audit methodology is explicit: "All techniques are
well-documented prior art … All re-implemented from first principles in
Spiral-native Rust. **No verbatim or near-verbatim copying from any
external source.**" The user also said on 2026-06-15:

> "Our tech where it matters. Using other browser's tech defeats the
> purpose of spiral."

A pure vendoring of `html5ever` 0.39 would violate the audit
posture. The right path is a **from-spec implementation** of a
minimum-viable HTML5 tokeniser + tree builder in `spiral-fmt`, driven
by the WHATWG HTML5 standard, that handles the cases
`spiral-html/src/lib.rs` is currently panicking on, plus the WPT
conformance cases the Phase 2 budget demands.

**Concrete next chunk (this sprint):**

1. **Skeleton `spiral-fmt` crate** with:
   - `Cargo.toml` (no upstream `html5ever`/`markup5ever`/`tendril`/`cssparser`/`selectors` deps).
   - `lib.rs` exposing `parse_html(input: &str) -> Result<spiral_dom::Dom, FormatError>` and `parse_css(input: &str) -> Result<Stylesheet, FormatError>`.
   - `html.rs` — tokeniser (data state, tag open state, tag name state, attribute name/value states, character references). No full spec coverage in M4.4.1; just enough to pass `spiral-html`'s 6 tests.
   - `tree.rs` — minimum tree builder: `html`, `head`, `body` insertion modes; DOCTYPE; comments; text merging; attribute application.
2. **Rewire `spiral-html`** to depend on `spiral-fmt` instead of `html5ever`/`markup5ever`/`tendril`. Remove those direct deps.
3. **Add tests in `spiral-fmt` for:** the 6 currently-panicking inputs (simple div, attributes, nested, text merging, malformed, doctype) plus 20 more spec-derived cases.
4. **Verify:** `cargo test --workspace` green; `cargo tree | grep -E 'html5ever|markup5ever|tendril'` returns nothing outside `spiral-fmt`.
5. **Update SSOT:** append ledger entry; update `active_context.md` M4.4 to "complete".

**Estimated scope:** ~1,500–2,500 LOC of new code in `spiral-fmt`;
~150 LOC removed from `spiral-html`; ~50 tests added. One engineer,
one focused sprint. Phase 2 first-sprint exit criteria.

**Risks:**
- Spec deviation. Mitigation: use the WHATWG test corpus as the ground truth (subset of html5lib tests).
- Scope creep. Mitigation: hard cap on M4.4.1 — minimum viable parser for the 6 currently-broken cases + 20 spec tests. Container queries, custom elements, etc. deferred to M5.

**Not in this chunk:** CSS parser implementation, vendor-prefix handling, encoding detection, `<template>`, adoption agency, foster parenting. Those are M5+ tasks.

---

**Updated first fill (post-competitive-parity research, 2026-06-16):**

The competitive-parity research identified 19 new P2 sprint items (Delta 5)
plus 1 re-ranked item (Delta 6). The synthesis recommends the following
sprint order that supplements the existing M4.4 work:

**M4.5/M5 immediate (pull forward into current sprint):**
1. **Adoption agency algorithm + active formatting elements + foster parenting** (#19–#21, L complexity, 2–3 weeks). These are the top-20 competitive gaps #2–#4. Without them, the tree builder produces incorrect DOM for any non-trivial HTML.
2. **`<template>` content fragment** (#37, M complexity, 1 week). Re-ranked from general P2 to active sprint. Priority #10 now bumped.
3. **Fragment parsing** (#22, M complexity, 1 week). Priority #6 gap. Blocks innerHTML, insertAdjacentHTML.

**M5 sprint (next sprint):**
4. **DOM collection types** (#27–#32: NodeList, HTMLCollection, DOMTokenList, Attr, NamedNodeMap, DocumentType). M complexity each, 1–2 weeks total. Blocks DOM manipulation from JS.
5. **Global attributes IDL + `data-*` attributes** (#25–#26). M complexity each.
6. **`globalThis`, `structuredClone`, `URL`/`URLSearchParams`** (#33, #34, #36). S–M complexity each, 1 week total.
7. **Quirk mode classifier + `<noscript>`** (#23–#24). S complexity each.

**M5/M6 sprint:**
8. **Proxy + Reflect** (#35, L complexity, 1–2 weeks).

This supplements (does not replace) the existing M4.5 Items 9, 11, 12, 13
listed in `docs/active_context.md`.

---

## 8. Open Questions for the User

1. Confirm the **`spiral-fmt` from-spec implementation** path is preferred
   over a literal `git subtree` of Servo's `html5ever` source.
2. Confirm minimum-viable scope for M4.4.1 (the 6 broken tests + 20 spec
   cases) is acceptable. If a stricter WPT pass rate is wanted for
   M4.4.1, scope grows linearly.
3. Confirm `spiral-crypto` security bugs (#2) get fixed **before** the
   first real crypto call site, even though no call site exists yet.
4. The M4.5 Track E wrappers (#3, #4) — are `spiral_net::Resolver`,
   `spiral_net::TlsConnector`, `spiral_network::Client`,
   `spiral_imagedecoder::Decoder` traits all wanted in M4.5, or just
   the network ones first?

### Questions added 2026-06-16 (competitive-parity research)

5. **~~Phase 2 backlog overflow:~~** **RESOLVED 2026-06-16 (Q1).** Re-tag bottom 30-40 P2 items (lowest complexity/impact) to P3. See Delta 7.

6. **~~Top-20 bias toward HTML/DOM:~~** **RESOLVED 2026-06-16 (Q2).** Added `spiral_urgency_weight` to scoring formula. See Delta 7 and methodology §11.3.

7. **~~HTTP/1.1 pull-forward:~~** **RESOLVED 2026-06-16 (Q3).** Pulled HTTP/1.1 client to P3. See Delta 7.

8. **~~Cookie jar pull-forward:~~** **RESOLVED 2026-06-16 (Q4).** Pulled cookie jar to P3. See Delta 7.

9. **~~DevTools scope:~~** **RESOLVED 2026-06-16 (Q5).** Full DevTools in P6 (7 panels). See Delta 7.

10. **~~Flow engine verification:~~** **RESOLVED 2026-06-16 (Q6).** Flow column dropped entirely. See Delta 7.

---

## 9. SSOT Links

- [`docs/active_context.md`](../docs/active_context.md) — current sprint state
- [`docs/progress_ledger.md`](../docs/progress_ledger.md) — change log
- [`docs/system_architecture.md`](../docs/system_architecture.md) — architecture deltas
- [`docs/architecture/design/shared-everything.md`](../docs/architecture/design/shared-everything.md) — Bet 1
- [`docs/plans/iteration-options.md`](../docs/plans/iteration-options.md) — Options A–E
- [`docs/phase1-tasks.md`](../docs/phase1-tasks.md) — completed Phase 1 tasks
- [`docs/audit-sprint-m4.md`](../docs/audit-sprint-m4.md) — M4 originality audit
- [`docs/innovations-backlog.md`](../docs/innovations-backlog.md) — 70-idea backlog
- [`ROADMAP.md`](../ROADMAP.md) — phase plan
- [`PLAN.md`](../PLAN.md) — implementation plan
- [`ARCHITECTURE.md`](../ARCHITECTURE.md) — canonical architecture

---

## 10. Delta Log

Per the SSOT restructure of 2026-06-16: deltas now live in the
implementation tracker, progress ledger, and active context.
See the appendix below.

---

## Appendix — Moved to the implementation tracker

The Deltas that previously lived at the bottom of this document (1
through 7, covering Chunks 1, 1.5, 2A, 3, Items 2, 3, 4, and the
competitive-parity research) have been moved to:

- [`docs/implementation_tracker.md`](../docs/implementation_tracker.md) — the canonical status tracker (Phase 1 Step 1.1–1.6)
- [`docs/progress_ledger.md`](../docs/progress_ledger.md) — append-only change log
- [`docs/active_context.md`](../docs/active_context.md) — live Phase state pointer

The spec retained its original pre-delta state for traceability.
