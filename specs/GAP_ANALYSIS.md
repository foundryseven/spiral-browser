# Spiral Browser — Gap Analysis (Spec & Delta Tracker)

**Author:** autonomous architect pass
**Date:** 2026-06-15
**Status:** initial discovery — pre-implementation
**Last SSOT sync:** `docs/active_context.md`, `docs/progress_ledger.md`

**Related artifacts (each with one job):**
- `docs/audits/2026-06-15-baseline.md` — functional baseline audit + M4.4–M4.6 prioritised plan
- `docs/audit-sprint-m4.md` — originality / license / novelty audit of M4 first sprint
- `docs/baseline-warnings.md` — `cargo check --workspace` warning drift baseline

This document is the **single source of truth for what is built, what is
partially built, and what is missing** across the four engine sub-domains.
Read it at the start of every session. Append deltas; do not rewrite
history.

Status legend: `[x]` shipped · `[~]` partial / stub / wired but inert ·
`[ ]` missing / not started · `[!]` actively broken or insecure

---

## 0. Headline Verdict

| Domain | Coverage | Comment |
|--------|----------|---------|
| Core Engines (HTML/CSS/DOM/Layout/Render/JS) | **~30%** | Foundation present; engine layers are scaffolds. |
| Networking & Storage (HTTP/TLS/DNS/cookies/cache) | **~5%** | Stubs only. Cookies, LocalStorage, IndexedDB, OPFS absent. |
| Presentation Layer (chrome UI, tabs, URL bar) | **~15%** | `spiral-ui` stubbed; tabs+URL bar not built. |
| Cross-Cutting (SOP/CSP/HSTS/sandbox/WebGL routing) | **~2%** | Almost nothing. |

**Test posture:** 272 tests across 18 crates, 6 failing in
`spiral-html` (html5ever 0.39.0 tree_builder panic — blocking). Rest of
workspace green.

**Single biggest blocker:** the `spiral-fmt` crate does not exist. M4.4
deliverable; the entire foundation of HTML/CSS parsing depends on it.

**Single biggest active defect:** `spiral-crypto::sha256` returns
32 zero bytes (not a placeholder — a security bug; `taffy` was never
in tree per mandate but `rustls` is imported yet unused).

**Competitive-parity research (2026-06-16):** 20 new P2 sprint items
were identified by the competitive-parity research (Delta 5), covering
HTML tree-builder depth (adoption agency, active formatting elements,
foster parenting, fragment parsing) and DOM IDL surfaces (NodeList,
HTMLCollection, DOMTokenList, Attr, dataset, structuredClone, URL). 1
item re-ranked to P2 sprint item (Delta 6: `<template>` content). See
§6 priority stack entries #19–#37.

---

## 1. Domain 1 — Core Engines

### 1.1 HTML parser (`spiral-html`)

| Item | Status | Notes |
|------|--------|-------|
| `spiral-fmt` vendored parser crate | `[ ]` | **Does not exist on disk.** M4.4 next sprint. |
| `html5ever 0.39.0` upstream integration | `[!]` | **6 tests panic** in `tree_builder/mod.rs:685` "no current element". Code at `crates/spiral-html/src/lib.rs:82-318` uses an outdated `TreeSink` shape; html5ever 0.39 expects stricter sink semantics. |
| `parse_document` → `spiral-dom::Dom` pipeline | `[~]` | Wired but panics. |
| `markup5ever = "0.39"` and `tendril = "0.5"` direct deps in `spiral-html/Cargo.toml:12-13` | `[!]` | M4.4 calls for eliminating these by routing through `spiral-fmt`. |
| DOCTYPE handling (`append_doctype_to_document`) | `[~]` | Wired but emits a comment node (DOM has no DOCTYPE variant). Lossy. |
| `<template>` element fragment handling (`get_template_contents`) | `[~]` | Returns the element itself (no document fragment). Lossy. |
| Adoption agency, foster parenting, format-extracted-character references | `[ ]` | Not exercised by current tests. |
| Encoding detection (BOM, `<meta charset>`, content sniff) | `[ ]` | Assumes UTF-8 only. |
| Character reference / entity decoding | `[ ]` | Not implemented; relies on `StrTendril` and html5ever internals. |

### 1.2 CSS parser & cascade (`spiral-css`)

| Item | Status | Notes |
|------|--------|-------|
| `spiral-fmt` unified CSS parser | `[ ]` | M4.4. |
| `cssparser 0.37` + `selectors 0.38` upstream | `[x]` | Compiles; 5 tests pass. |
| Cascade (origin order, specificity, `!important`, inheritance) | `[~]` | Designed for M8; not implemented. |
| `parse_selector` round-trip | `[x]` | 4 selector variants tested. |
| Vendor-prefix handling | `[ ]` | Unspecified. |

### 1.3 DOM (`spiral-dom`)

| Item | Status | Notes |
|------|--------|-------|
| `Node` (Element, Text, Comment, Document) | `[x]` | 13 tests pass. |
| Parent/child via indices | `[x]` | |
| Attribute storage `Vec<(String, String)>` | `[x]` | |
| Arena allocation `Vec<Node>` + indices | `[x]` | |
| `insert_before` / `replace_child` | `[ ]` | `spiral-html` open-codes insert-before via remove+re-append. DOM API itself lacks it (`spiral-html/src/lib.rs:243-253` admits "Dom doesn't have insert-before"). |
| `<template>` content document fragment | `[ ]` | |
| DOCTYPE node variant | `[ ]` | |
| Mutation events / observer API | `[ ]` | |
| `getElementById`, `querySelector` | `[ ]` | |

### 1.4 Layout — Gyre (`spiral-gyre`)

| Item | Status | Notes |
|------|--------|-------|
| Box model (`margin`, `border`, `padding`, `content`) | `[x]` | Type defined. |
| Block layout: vertical stacking | `[~]` | 1 trivial pass; no margin collapse, BFC, IFC. |
| Margin collapse (positive/negative/nested) | `[ ]` | Not implemented. |
| BFC / IFC | `[ ]` | |
| Floats (left/right, clear, BFC containment) | `[ ]` | |
| Positioning (static, relative, absolute, fixed, sticky) | `[ ]` | |
| Flex (M10–11) | `[ ]` | |
| Grid (M13–14) | `[ ]` | |
| WPT pass rate | `[ ]` | 0%. |
| `LayoutEngine` ignores the stylesheet | `[~]` | Signature takes `&Stylesheet` but `_stylesheet` parameter is unused (`spiral-gyre/src/lib.rs:65`). CSS is structurally disconnected from layout. |
| Text shaping integration (harfrust) | `[ ]` | M9. |
| Inline / line layout | `[ ]` | |
| Lazy / incremental relayout | `[ ]` | Bet 4 work, M30+. |

### 1.5 Render (`spiral-render` + `spiral-paint`)

| Item | Status | Notes |
|------|--------|-------|
| Software rasteriser (`SoftwareRenderer`) | `[x]` | Display-list walker; fill, stroke, text, clip, transform, layers. 14 tests. |
| 5×7 bitmap font (ASCII 0x20–0x7E) | `[x]` | |
| RGBA8 → PNG (`encode_png`) | `[x]` | |
| `spiral-paint` display list + compositing | `[~]` | Phase 4 "do not touch" per active_context. |
| Vello / wgpu GPU path | `[ ]` | Phase 4. |
| `spiral-gpu` abstraction | `[ ]` | Phase 4. |
| Vello fork with picture cache | `[ ]` | Phase 4. |
| Display list built from layout output | `[ ]` | `spiral-browser` builds the hello-world display list by hand; no glue from layout → paint. |

### 1.6 Vortex — JavaScript engine (`spiral-vortex`)

| Item | Status | Notes |
|------|--------|-------|
| Lexer (full ECMAScript tokens) | `[x]` | |
| Parser (recursive descent + Pratt) | `[x]` | |
| AST (ES2015+ node types) | `[x]` | |
| Tree-walking interpreter (Phase A) | `[x]` | |
| `console.log`/`info`/`warn`/`error` | `[x]` | Not yet wired to `RendererToBrowser::ConsoleMessage` IPC. |
| Event loop (microtask/macrotask, setTimeout/setInterval) | `[x]` | Wired in-process; no integration with browser event loop. |
| Builtins (`Object`, `Array`, `String`, `Number`, `Boolean`, `Math`, `JSON`) | `[x]` | Partial. |
| Vortex GC rewrite (per-origin arenas, TaggedCell, GcKey) | `[x]` | 84 GC tests. |
| `VortexHeap` ↔ `Runtime` glue | `[x]` | Fixed `gc_live_count` bug in audit pass. |
| Bytecode VM (Phase B) | `[ ]` | M10–24. |
| Mark-sweep from real stack+globals roots | `[~]` | Roots from environment chain; not yet from VM stack. |
| Closures / `this` / prototype chain | `[~]` | Partial in interpreter; design only for bytecode. |
| ES2015+ syntax (let/const, arrow, classes, spread) | `[~]` | Partial. |
| `rusty_v8` oracle behind `v8` feature | `[x]` | Feature-gated. CI compliance path open. |
| DOM bindings (`createElement`, `appendChild`, `setAttribute`, `addEventListener`) | `[ ]` | `dom_bindings/mod.rs` is a stub. |
| Test262 compliance | `[ ]` | 0%; ~5–10% target for end of M9. |
| Build warnings | `[!]` | 12+ dead-code warnings (`interval_ms`, etc.). Not blocking but noisy. |

### 1.7 Shared-Everything Multi-Process (Bet 1)

| Item | Status | Notes |
|------|--------|-------|
| `spiral-context` capability types (brands, tokens) | `[x]` | M4 skeleton. 21 tests. |
| `Origin`, `CapabilitySet`, `Context`, `ContextOps` | `[x]` | |
| Real runtime (per-origin contexts in one process) | `[ ]` | M25. |
| `InProcess` vs `Escalated` modes | `[x]` | Types only. |
| Integration with Vortex heap / Gyre / parser | `[ ]` | None yet. |

---

## 2. Domain 2 — Networking & Storage

### 2.1 HTTP / TLS / DNS

| Item | Status | Notes |
|------|--------|-------|
| `spiral-network` HTTP client via `hyper` | `[~]` | Stub at `crates/spiral-network/src/lib.rs:24-74`. `get`/`post` return 200 with empty body. No actual hyper call. |
| `spiral-net` DNS resolver via `hickory-dns` | `[~]` | Stub at `crates/spiral-net/src/lib.rs:25-56`. `resolve` returns `["127.0.0.1"]`. No `hickory_resolver::TokioResolver` integration. |
| `spiral-net` TLS via `rustls` | `[ ]` | `TlsConfig` struct exists (`spiral-net/src/lib.rs:8-17`) but unused. No `rustls::ClientConfig` glue. |
| `spiral_net::Resolver` trait | `[ ]` | M4 deliverable; not done. |
| `spiral_net::TlsConnector` trait | `[ ]` | M5 deliverable. |
| `spiral_network::Client` trait | `[ ]` | M6 deliverable. |
| Connection pooling | `[ ]` | |
| Redirect policy | `[ ]` | |
| Cookie jar | `[ ]` | |
| HTTP/2 (h2, h2c) | `[ ]` | |
| HTTP/3 (quinn) | `[ ]` | |
| WebSockets | `[ ]` | |
| WebRTC | `[ ]` | |
| Speculative caching / preload (prerender, prefetch) | `[ ]` | |
| HSTS preload list | `[ ]` | |
| Certificate pinning | `[ ]` | M5 stub. |

### 2.2 Cryptography (`spiral-crypto`)

| Item | Status | Notes |
|------|--------|-------|
| `Crypto::random_bytes` | `[!]` | Returns `((i % 256) as u8)` — **deterministic, NOT random**. `crates/spiral-crypto/src/lib.rs:18`. Security bug. |
| `Crypto::sha256` | `[!]` | Returns 32 zero bytes. **Not a hash**. `crates/spiral-crypto/src/lib.rs:24`. Security bug. |
| `rustls` dep declared but unused | `[~]` | `spiral-crypto/Cargo.toml:11`. Dead dep. |
| HKDF, HMAC, Ed25519 | `[ ]` | |
| `SecureRandom` trait wrapping `getrandom` | `[ ]` | Iteration-options Option E. |
| Subresource Hash integrity (SRI) | `[ ]` | |
| WebCrypto API | `[ ]` | M30+. |

### 2.3 Storage

| Item | Status | Notes |
|------|--------|-------|
| Cookie jar | `[ ]` | Not present. |
| LocalStorage | `[ ]` | |
| SessionStorage | `[ ]` | |
| IndexedDB | `[ ]` | |
| OPFS (Origin Private File System) | `[ ]` | |
| CacheStorage (Service Workers) | `[ ]` | |
| Quota management / eviction | `[ ]` | |
| Storage partitioning (Bet 1) | `[ ]` | M30+. |
| Origin-keyed encryption (Bet 1, capability) | `[ ]` | |
| `spiral-storage` crate | `[ ]` | Not in workspace. Should exist. |

### 2.4 Image decoding (`spiral-imagedecoder`)

| Item | Status | Notes |
|------|--------|-------|
| Format sniffing (PNG/JPEG/WebP/AVIF) | `[x]` | Magic-byte detection works. |
| PNG decode | `[~]` | Returns 1×1 white pixel. No real decode via `png` crate. |
| JPEG decode (`zune-jpeg`) | `[~]` | Same placeholder. |
| WebP decode | `[~]` | Same. |
| AVIF decode (`ravif`) | `[~]` | Same. |
| `Decoder` enum dispatch per format | `[ ]` | Iteration-options Option E; not done. |
| Lazy / progressive loading | `[ ]` | M19. |
| Animated images (APNG, animated WebP, animated AVIF) | `[ ]` | |

---

## 3. Domain 3 — Presentation Layer (Chrome UI)

> **Do not touch zone** per `docs/active_context.md`: `spiral-ui`, `spiral-theme`,
> `spiral-paint`. Phase 4 work. Listed for completeness only.

| Item | Status | Notes |
|------|--------|-------|
| `spiral-theme` design tokens (Zen-style) | `[~]` | Phase 4; "do not touch". |
| `spiral-ui` winit window / event loop | `[~]` | Phase 4. |
| Sidebar tabs (create, switch, close, drag) | `[ ]` | |
| Floating URL bar / Omnibox | `[ ]` | |
| Navigation buttons (back/forward/reload/home) | `[ ]` | |
| Tab context menu | `[ ]` | |
| DevTools element inspector, console, network | `[ ]` | |
| Tab Provenance Graph (innovations #12) | `[ ]` | M12+ novelty. |
| Find-in-page | `[ ]` | |
| Downloads UI / manager | `[ ]` | |
| Settings panel | `[ ]` | |
| WebExtensions API (manifest v3) | `[ ]` | v1.0. |
| Content script injection (sandboxed) | `[ ]` | |
| Extensions ↔ page typed message bus | `[ ]` | |

---

## 4. Domain 4 — Cross-Cutting Concerns

### 4.1 Security / Privacy

| Item | Status | Notes |
|------|--------|-------|
| Same-Origin Policy enforcement | `[ ]` | Not yet. |
| Content Security Policy (CSP) parser & enforcement | `[ ]` | |
| HSTS / HSTS preload | `[ ]` | |
| Secure cookie flags (`Secure`, `HttpOnly`, `SameSite`) | `[ ]` | |
| Cookie partitioning (CHIPS / Storage Access API) | `[ ]` | M18+ per roadmap. |
| Referrer-Policy | `[ ]` | |
| Permissions Policy / Feature Policy | `[ ]` | |
| COOP, COEP, CORP, CORS | `[ ]` | |
| Mixed-content blocking | `[ ]` | |
| Subresource Integrity (SRI) | `[ ]` | |
| Mixed scripting (`X-Frame-Options`, frame-ancestors) | `[ ]` | |
| Spectre mitigations (shared-everything model) | `[ ]` | Designed in M4, runtime in M25. |
| Memory zeroisation for secrets | `[ ]` | |
| OCSP / OCSP stapling | `[ ]` | |
| Certificate transparency | `[ ]` | |
| `spiral-sandbox` per-platform profiles | `[~]` | Phase 4 "do not touch"; re-evaluated under Bet 1 — capability-typed, not OS-level. |
| Linux: Landlock + seccomp-bpf | `[ ]` | |
| macOS: Seatbelt | `[ ]` | |
| Windows: Restricted Token + Job Object | `[ ]` | |
| Third-party tracker blocking | `[~]` | `spiral-filter` is the policy engine; nothing wires it in. |
| Anti-fingerprinting posture | `[ ]` | |
| Telemetry / phone-home (must be none) | `[x]` | None — by design. |

### 4.2 Filter / Ad-blocking (`spiral-filter`, Bet 3)

| Item | Status | Notes |
|------|--------|-------|
| ABP/EasyList rule AST | `[x]` | `rule.rs`. |
| ABP/EasyList parser (cosmetic + network) | `[x]` | `syntax/{cosmetic,network}.rs`. 40 tests. |
| Hostname trie compilation | `[x]` | `compile/trie.rs`. |
| Coalition for Better Ads thresholds | `[x]` | `lists/cba.rs`. Audited, corrected. |
| Policy level slider (Off / WorstOffenders / CommonAnnoyances / PrivacyFocused / Strict / Maximum) | `[x]` | `policy/default_policy.rs`. |
| Site stewardship score | `[x]` | In `Rule` type; not yet applied at runtime. |
| `spiral_filter::runtime::Filter` engine | `[ ]` | Skeleton only. M5+. |
| Compile-time policy application (between network and parser) | `[ ]` | Not wired in. |
| User-tunable slider UI | `[ ]` | M12+. |
| Stewardship registry (opt-in) | `[ ]` | M12+. |
| Network filter request matching | `[ ]` | |

### 4.3 Graphics / Compositing (GPU)

| Item | Status | Notes |
|------|--------|-------|
| `spiral-gpu` wgpu abstraction | `[~]` | Phase 4 "do not touch". |
| Vello 2D scene build | `[ ]` | Phase 4. |
| Display list → Vello scene → swap chain | `[ ]` | |
| Tile-based picture cache (Vello fork) | `[ ]` | Phase 4. |
| Dirty-rect invalidation | `[ ]` | |
| WebGL binding (wgpu backend) | `[ ]` | M49–54. |
| WebGPU binding | `[ ]` | |
| Hardware video decode (`spiral-media`) | `[ ]` | M30+. |
| AV1 (dav1d), VP9, HEVC, Opus, AAC | `[ ]` | |
| Widevine CDM bridge | `[ ]` | M36+. |
| ClearKey EME | `[ ]` | M12. |

### 4.4 Media (`spiral-media`)

| Crate exists? | Status | Notes |
|---------------|--------|-------|
| `spiral-media` | `[ ]` | Not in workspace. Required for v0.1 (NYT test page = text only, but YouTube/Netflix in scope). M30+ deliverable. |

### 4.5 Internationalisation / a11y / extensions

| Item | Status | Notes |
|------|--------|-------|
| ICU integration (`icu` crate) | `[ ]` | M61–84. |
| Locale-aware text shaping | `[ ]` | M9 base, M61–84 broad. |
| Screen reader / ARIA | `[ ]` | M61–84. |
| Extensions API | `[ ]` | M61–84. |

---

## 5. Other gaps (not in the four domains but observed)

### 5.1 Process / IPC

| Item | Status | Notes |
|------|--------|-------|
| `spiral-ipc` Unix + Windows transport | `[x]` | 16 tests. |
| `IpcTransport` trait + `MockTransport` | `[x]` | |
| Length-prefixed bincode framing | `[x]` | Fuzz-tested. |
| Per-process routing keys (renderer ↔ network ↔ GPU) | `[ ]` | Phase 4. |
| Per-tab renderer process spawn | `[ ]` | Phase 4; Bet 1 makes this optional. |
| `BrowserToRenderer::Hello` handshake | `[x]` | |
| `RendererToBrowser::ConsoleMessage` from Vortex | `[ ]` | Vortex is in-process; no bridge yet. |

### 5.2 WPT / test infrastructure

| Item | Status | Notes |
|------|--------|-------|
| `tests/wpt/` directory | `[ ]` | Empty. |
| HTML5 lib test subset port | `[ ]` | Blocked on M4.4. |
| CSS parser test subset port | `[ ]` | Blocked on M4.4. |
| Block layout WPT fixture harness | `[ ]` | |
| Fuzz harness for parsers | `[~]` | IPC has one; parsers do not. |
| Cargo bench (`benches/layout/`) | `[ ]` | Directory exists, empty. |
| Criterion benchmarks | `[ ]` | |
| Coverage measurement (cargo-llvm-cov) | `[ ]` | |

### 5.3 Build / CI / docs

| Item | Status | Notes |
|------|--------|-------|
| `cargo build --workspace` | `[x]` | |
| `cargo test --workspace` (default build) | `[!]` | 6 spiral-html failures block the green-CI gate. |
| `cargo test --workspace --exclude spiral-html` | `[x]` | 266 tests pass. |
| `cargo clippy --workspace --all-targets -D warnings` | `[~]` | 0 clippy errors; `spiral-vortex` has 12+ dead-code warnings. |
| `cargo fmt --all -- --check` | `[x]` | |
| CI matrix `ubuntu-latest` + `macos-latest` + `windows-latest` | `[x]` | |
| `docs/specs/` (single source of truth) | `[x]` | This file. |
| `docs/plans/iteration-options.md` | `[x]` | Maintained. |
| `docs/active_context.md` | `[x]` | |
| `docs/progress_ledger.md` | `[x]` | |
| `docs/innovations-backlog.md` | `[x]` | 70-idea backlog, fully audited. |
| `docs/architecture-shared-everything.md` | `[x]` | Bet 1 writeup. |

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

5. **Phase 2 backlog overflow:** 140 capabilities tagged P2 (months 4–9).
   At ~40 working days per sprint, M4.5 through M9 is ~6 sprints (~240
   working days). 140 capabilities at an average of 2 days each is 280
   days. This exceeds the sprint window. Should some P2 items be
   re-tagged P3 to fit the timeline?

6. **Top-20 bias toward HTML/DOM:** The scoring formula weights prevalence
   heavily. HTML/DOM items dominate because they are "ubiquitous." Should
   the scoring include a "Spiral-specific urgency" weight (e.g. items
   that block the next milestone vs items that can wait)?

7. **HTTP/1.1 pull-forward:** The research shows HTTP/1.1 is a
   prerequisite for loading any remote page. Should it be pulled forward
   from Phase 4 to Phase 3?

8. **Cookie jar pull-forward:** The research shows cookies are a
   prerequisite for session management. Should the cookie jar be pulled
   forward from Phase 4 to Phase 3?

9. **DevTools scope:** The research identifies 113 developer-surface
   capabilities. Should Phase 6 DevTools be scoped to just Elements +
   Console + Network (the minimum viable set), or should it include
   Performance, Memory, Security, and Application panels?

10. **Flow engine verification:** Per methodology §11.1, the Flow row was
    to be re-verified at chunk 12 time. The matrix files use "no" for Flow
    on most rows. If Flow's scope has shifted, the Flow column should be
    updated.

---

## 9. SSOT Links

- [`docs/active_context.md`](../docs/active_context.md) — current sprint state
- [`docs/progress_ledger.md`](../docs/progress_ledger.md) — change log
- [`docs/system_architecture.md`](../docs/system_architecture.md) — architecture deltas
- [`docs/architecture-shared-everything.md`](../docs/architecture-shared-everything.md) — Bet 1
- [`docs/plans/iteration-options.md`](../docs/plans/iteration-options.md) — Options A–E
- [`docs/phase1-tasks.md`](../docs/phase1-tasks.md) — completed Phase 1 tasks
- [`docs/audit-sprint-m4.md`](../docs/audit-sprint-m4.md) — M4 originality audit
- [`docs/innovations-backlog.md`](../docs/innovations-backlog.md) — 70-idea backlog
- [`ROADMAP.md`](../ROADMAP.md) — phase plan
- [`PLAN.md`](../PLAN.md) — implementation plan
- [`ARCHITECTURE.md`](../ARCHITECTURE.md) — canonical architecture

---

## 10. Delta Log

Per the document header: "Append deltas; do not rewrite history."

### Delta 1 — 2026-06-15 (Chunks 1, 1.5, 2A, 3)

The following items from the initial gap analysis have been completed since
the document was first written. The sections above retain their original
state for traceability.

| Gap | Was | Now | Evidence |
|-----|-----|-----|----------|
| **G0.1** `spiral-crypto` security bugs | `[!]` P0 | `[x]` Fixed | `sha256` uses `sha2` crate (FIPS 180-2 KATs); `random_bytes` uses `getrandom` (CSPRNG). 18 tests. Dead `rustls` dep removed. See ledger entry "Chunk 1". |
| **G0.2** `spiral-html` 6 panicking tests | `[!]` P0 | `[x]` Resolved | `spiral-html` retired from workspace. `spiral-fmt` passes all 13 e2e cases including the 6 former panics. See ledger entry "Chunk 3". |
| **§1.1** `spiral-fmt` does not exist | `[ ]` | `[x]` Shipped | From-spec HTML5 tokeniser + simplified tree builder. 29 tests (16 unit + 13 e2e). Zero Servo deps. See ledger entry "Chunk 2A". |
| **§1.1** `html5ever`/`markup5ever`/`tendril` in workspace | `[!]` | `[x]` Removed | `cargo tree \| grep -iE 'html5ever\|markup5ever\|tendril'` returns empty. |
| **§1.6** Vortex 12+ dead-code warnings | `[!]` | `[x]` Fixed | `cargo clippy --workspace --all-targets -- -D warnings` passes clean. See ledger entry "Chunk 1.5". |
| **§5.3** `cargo test --workspace` red | `[!]` 6 failures | `[x]` Green | 275 tests passing, 0 failures. |

**Updated headline numbers:**
- Test posture: **275 tests across 17 crates, 0 failing** (was 272/18/6).
- Crate count: 17 (was 18; `spiral-html` removed).
- Single biggest blocker: ~~`spiral-fmt` does not exist~~ → now **G1.2** (CSS parser is a stub).
- Single biggest active defect: ~~`spiral-crypto::sha256` returns zeros~~ → **none at P0 severity**.

### Delta 2 — 2026-06-15 (M4.4.1 Item 2: Rawtext + ScriptData)

| Gap | Was | Now | Evidence |
|-----|-----|-----|----------|
| **G1.3** HTML5 rawtext / script-data: `<` inside `<script>`, `<style>`, `<title>`, `<textarea>` tokenised as tag-open | `[!]` | `[x]` Fixed | `spiral-fmt` tokeniser gained `Mode` (Normal/Rawtext/ScriptData) and `read_raw_body` byte-scan; tree builder tracks `rawtext_depth` and short-circuits `handle_character`. 9 new tests (4 unit + 5 e2e). `parse_html("<script>if (a < b) {}</script>")` preserves the `<` as text. See ledger entry "M4.4 Item 2". |

**Updated headline numbers (post-Item 2):**
- Test posture: **326 tests across 20 binaries, 0 failing** (was 275/17/0).
- `spiral-fmt` itself: **23 lib + 18 e2e = 41 tests**, up from 16 + 13.
- Single biggest blocker: still **G1.2** (CSS parser stub — Fork 1-B still pending).
- Single biggest active defect: **none at P0 severity**.

### Delta 3 — 2026-06-15 (M4.4.1 Item 3: Numeric character references)

| Gap | Was | Now | Evidence |
|-----|-----|-----|----------|
| **G1.4** HTML5 numeric character references (`&#65;`, `&#x41;`) | `[!]` | `[x]` Fixed | `spiral-fmt` tokeniser gained `try_character_reference` (named + numeric) plus `try_numeric_ref` with the spec's replacement table (null, surrogates, out-of-range, 0x80..=0x9F Windows-1252 fixup). 20 new tests (13 unit + 7 e2e). See ledger entry "M4.4 Item 3". |

**Updated headline numbers (post-Item 3):**
- Test posture: **347 tests across 42 binaries, 0 failing** (was 326).
- `spiral-fmt` itself: **37 lib + 25 e2e = 62 tests**, up from 41.
- Single biggest blocker: still **G1.2** (CSS parser stub — Fork 1-B still pending).
- Single biggest active defect: **none at P0 severity**.

### Delta 4 — 2026-06-16 (M4.4.1 Item 4: CSS parser, Fork 1-B)

| Gap | Was | Now | Evidence |
|-----|-----|-----|----------|
| **G1.2** `spiral-fmt` unified CSS parser (CSS Syntax Level 3, from-spec) | `[ ]` | `[x]` Shipped | 6 source files in `spiral-fmt/src/css/` (`mod.rs`, `parser.rs`, `selector.rs`, `specificity.rs`, `tokenizer.rs`, `value.rs`). Tokeniser: ident/number/percentage/dimension/string/hash/at-keyword plus all CSS3 punctuation. Parser: qualified rules, at-rules (block and `;` terminator forms), declarations with `!important`. Selectors: type, universal, class, ID, attribute (all 6 matchers plus `i` / `s` case flags), pseudo-class, pseudo-element, all 4 combinators. Specificity per Selectors Level 4 (a, b, c). 88 lib + 14 new e2e tests pass. `spiral-css` rewired as a `#[deprecated]` shim re-exporting `spiral_fmt::css::*`; `spiral-gyre` (the only consumer of `spiral_css::Stylesheet`) still compiles. See ledger entry "M4.4.1 Item 4: CSS parser (Fork 1-B) shipped". |
| **G1.2a** `spiral-css` depended on upstream `cssparser` / `selectors` | `[x]` | `[x]` Removed | `spiral-css/Cargo.toml` no longer lists `cssparser` or `selectors`; replaced with `spiral-fmt` workspace dep. Old API surface re-exported verbatim so the existing `spiral_css::Stylesheet` import path continues to work; the shim is `#[deprecated]` with a note pointing callers at `spiral_fmt::parse_css`. |

**Updated headline numbers (post-Item 4):**
- Test posture: **409 tests across 42 binaries, 0 failing** (was 347).
- `spiral-fmt` itself: **88 lib + 39 e2e = 127 tests**, up from 62. (39 = 25 pre-existing HTML e2e + 14 new CSS e2e.)
- `spiral-css`: 2 lib tests (shim coverage) on top of the re-exports.
- Single biggest blocker: **none at P0 severity** for the first time — Item 4 closes the last M4.4.1 stub.
- Single biggest active defect: **none at P0 severity**.

**Verification (run today, 2026-06-16):**
- `cargo fmt --all -- --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `cargo test --workspace` — 409 passing, 0 failing.
- `cargo build --workspace` — clean.

### Delta 5 — 2026-06-16 (Competitive parity research: top-20 gaps)

| Gap | Was | Now | Evidence |
|-----|-----|-----|----------|
| **G1.3a** Adoption agency algorithm (misnested formatting) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 2. WHATWG HTML §12.2.6.1. Ubiquitous across all engines. |
| **G1.3b** Active formatting elements list | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 3. WHATWG HTML §12.2.6.1. Required by adoption agency. |
| **G1.3c** Foster parenting (out-of-table / in-table) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 4. WHATWG HTML §12.2.6.1. Blocks correct table parsing. |
| **G1.3d** Fragment parsing algorithm | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 6. WHATWG HTML §12.4. Blocks innerHTML, insertAdjacentHTML. |
| **G1.3e** Quirk mode classifier | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 8. WHATWG HTML §12.1. Blocks correct CSS on legacy sites. |
| **G1.3f** `<noscript>` element | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 7. WHATWG HTML §4.6.7. Blocks correct rendering with JS enabled. |
| **G1.3g** Global attributes IDL (`id`, `class`, `style`, `title`, `lang`, `dir`, `hidden`, `tabindex`, `contenteditable`, `inert`, `popover`, etc.) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 1. WHATWG HTML §3.2.6. Blocks all DOM IDL and accessibility. |
| **G1.3h** `data-*` custom data attributes (`dataset` IDL) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 15. WHATWG HTML §3.2.6.3. Blocks `element.dataset` from JS. |
| **G1.4a** `DOMTokenList` (`classList`, `relList`, etc.) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 9. WHATWG DOM §7.1. Blocks class manipulation from JS. |
| **G1.4b** `NodeList` (static or live) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 10. WHATWG DOM §4.4. Blocks querySelectorAll result handling. |
| **G1.4c** `HTMLCollection` (live ordered collection) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 11. WHATWG DOM §4.5. Blocks getElementsByTagName result handling. |
| **G1.4d** `Attr` interface | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 12. WHATWG DOM §4.9. Blocks getAttributeNode and attribute iteration. |
| **G1.4e** `NamedNodeMap` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 13. WHATWG DOM §4.8. Blocks `.attributes` on Element. |
| **G1.4f** `DocumentType` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 14. WHATWG DOM §4.6. Blocks `document.doctype`. |
| **G1.6a** `globalThis` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 16. ECMA-262 §19.4.1. Blocks universal global reference. |
| **G1.6b** `structuredClone` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 17. WHATWG HTML §8.2.7. Blocks postMessage structured data. |
| **G1.6c** `Proxy` (handler traps) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 18. ECMA-262 §10.5. Blocks transparent object interception. |
| **G1.6d** `Reflect` (static reflection namespace) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 19. ECMA-262 §28.1. Blocks Proxy + Reflect idiom. |
| **G1.6e** `URL` and `URLSearchParams` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 20. WHATWG URL §4. Blocks URL manipulation everywhere. |

### Delta 6 — 2026-06-16 (Competitive parity research: priority re-ranking)

| Gap | Was | Now | Evidence |
|-----|-----|-----|----------|
| **#10** `<template>` content fragment; DOCTYPE; insertBefore | P2 | **P2 sprint item** (bump from general P2 to active sprint) | `02-competitive-matrix-index.md` §6, rank 5. Ubiquitous, not-started, P2. Blocks Web Components, Shadow DOM, declarative templates. |

