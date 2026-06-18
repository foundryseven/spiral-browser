# Spiral Browser — Implementation Plan

> **Project:** Spiral Browser
> **Language:** Rust (edition 2021)
> **License:** MPL-2.0
> **Target platforms:** Windows, macOS, Linux
> **Scope:** Full production browser, 6–8 year horizon
> **Status:** Alpha. Phase 0 complete. Phase 1.5 (SSOT Restructure) shipped at `v0.0.0-bootstrap`. Phase 1 in flight.
> **Methodology:** LLM-assisted, human-directed, adversarially reviewed. See [`docs/methodology.md`](docs/methodology.md).

---

## 1. Project vision

Spiral is a fully independent web browser, built from scratch in Rust. The entire web platform stack — HTML5 parser, CSS parser, layout engine, JavaScript engine, paint, GPU render, networking, IPC — is written in Spiral-native Rust. We do not depend on Chromium, WebKit, Gecko, or any browser-engine code (HTML5 parser, CSS parser, layout engine, JS engine, or otherwise). The three engines that bear the Spiral brand (Gyre, Vortex, Fmt) are fully in-house.

### Why?

The web runs on a three-engine monoculture. The same vulnerabilities appear in three places. The same standards drift propagates to three billion users. The same architectural assumptions are baked into every browser you can install. The result is a brittle, opaque, surprisingly homogeneous web platform.

Spiral exists to be a fourth engine — different in architecture, auditable in code, and written in a memory-safe language. The aim is not to beat V8 on a benchmark. The aim is to prove that an independent, from-scratch browser is buildable in 2026, that the engineering is tractable, and that the result is a system the people who depend on it can actually inspect.

### Aims

1. **Engine independence.** No browser-engine code vendored. Three branded engines (Gyre, Vortex, Fmt) plus two in-design branded subsystems (Filter, Context).
2. **Memory safety end-to-end.** Safe Rust across the render path. The only `unsafe` is in audited dependencies and FFI shims with narrow surface.
3. **From-spec implementations.** HTML5, CSS Syntax Level 3, CSS Display + Box Model — all cited inline.
4. **Capability-typed page context.** A research-grade runtime (Filter + Context) that gates cross-origin and sensitive operations at compile time.
5. **A working browser, eventually.** Phase 1 ships a render-only browser. Phase 9 ships a production browser.
6. **Open source, MPL-2.0, day one.** Every commit, every decision, every metric — public.
7. **Open methodology.** The way Spiral is built — LLM-assisted, human-directed, adversarially reviewed, with a public failure log — is part of the project. See [`docs/methodology.md`](docs/methodology.md).

### Non-aims

- **Performance leadership.** We are not faster than V8. We are not aiming to be. We are aiming to be *correct*, *independent*, and *inspectable*.
- **A privacy browser.** Privacy is a use case the capability types *enable*; we are not building it in the engine.
- **A drop-in Chrome replacement.** Sites that depend on Chrome-only Web Platform features will not work. We track the standards, not the implementation shortcuts.
- **A hand-coded artefact.** Spiral is LLM-assisted. We are not Ladybird. We are not claiming craftsmanship as our selling point. We are claiming a methodology.
- **A proprietary codebase.** Everything is public.

---

## 2. Core principles

1. **Independence over performance.** A slower browser you can inspect beats a faster browser you cannot.
2. **From-spec over upstream.** If the spec says so, we do what the spec says. If the spec is ambiguous, we file an issue.
3. **Memory safety as architecture, not patch.** No `unsafe` in the engines. No `unsafe` in the parsers. `unsafe` only at the FFI boundary, with a safety comment per block.
4. **Per-packet completeness.** A `pub` symbol is not done when it compiles; it is done when an external consumer uses it. The `audit-orphan-exports.sh` gate enforces this. See `docs/decisions/0006-cross-cutting-features.md`.
5. **The brand belongs to the engine.** A new engine gets a name. A wire protocol does not.
6. **Honest novelty claims.** Any claim of "novel", "first", "unique", "no prior art" is verified by a research agent before committing. The M4 audit methodology is the canonical standard.
7. **Adversarial review over single-author trust.** Every change lands through review — human, mechanical, or both. The audit scripts are the floor, not the ceiling. See [`docs/methodology.md`](docs/methodology.md) §5.

---

## 3. Engine identity — the Spiral brand

The brand belongs to the engines, not the wire. Each engine is fully in-house, written in safe Rust, and follows the relevant spec section-by-section.

### Gyre — layout (`spiral-gyre`)

Gyre is Spiral's custom layout engine. It implements the CSS Display, Box Model, and (in Phase 2) Flexbox and Grid specs. The crate name reflects the spiral-gyre glyph in the brand mark: layout values flow inward toward a centre, like the arms of a spiral.

- **Phase 1:** box model + block layout.
- **Phase 2:** flexbox (Packet 2.5.x), grid (Packet 2.6.x).
- **Phase 4:** writing modes, Bidi.

See `docs/decisions/0003-gyre-rename.md` and `docs/architecture/gyre/`.

### Vortex — JavaScript (`spiral-vortex`)

Vortex is Spiral's from-scratch JavaScript engine. The crate name reflects the rotation of evaluation: each call frame spins up, runs, spins down. Lexer → parser (Pratt) → AST → tree-walking interpreter (Phase 1) → bytecode VM (Phase 2) → baseline JIT (Phase 3).

- **Phase 1:** tree-walking interpreter. Console, math, object, array, GC.
- **Phase 2:** bytecode VM, ~5–10× faster. Closures, prototypes, classes.
- **Phase 3:** Cranelift-based baseline JIT.
- **Phase 4+:** optimisation, modules, async/await.

`rusty_v8` is available behind a `v8` Cargo feature for CI compliance testing only — it is the "V8 oracle", not the production engine. The `JSRuntime` trait abstraction enables future engine swapping via feature flag.

See `docs/decisions/0002-vortex-from-scratch.md` and `docs/architecture/vortex/`.

### Fmt — HTML5 + CSS parsers (`spiral-fmt`)

Fmt is Spiral's from-spec HTML5 and CSS parser. The crate name reflects the *Forge* brand: the place where raw bytes are hammered into structured form. Parser → tokens → AST → spec-compliant tree.

- **HTML5:** 8 insertion modes (Initial, BeforeHtml, BeforeHead, InHead, AfterHead, InBody, AfterBody, AfterAfterBody). Adoption agency algorithm, active formatting elements, foster parenting, fragment parsing.
- **CSS Syntax 3:** 8 modules — tokeniser, parser, the CSS Selectors spec, specificity, values, at-rules, declarations, attribute matchers.
- **CSS Cascade:** user-agent / user / author / `!important` ordering. Specificity: inline > ID > class > element.

`spiral-css` is a deprecated shim forwarding to `spiral_fmt::css::*` and providing a `CssParser` adapter. New code depends on `spiral-fmt` directly. The `deprecation` lint is set on the shim crate.

See `docs/decisions/0001-css-parser-spiral-fmt.md` (the "Fork 1-B" decision) and `docs/architecture/fmt/`.

### Filter — policy (`spiral-filter`)

Filter is Spiral's compile-time HTML/CSS policy engine. It decides, at parse time and at request time, whether a request or selector is allowed. The brand is the *filter* of the spiralled paths through a browser: a sieve.

- **Phase 1:** runtime policy (Packet 1.6.4, shipped).
- **Phase 5:** compile-time policy via `const` evaluation.
- **Phase 6:** content-security-policy integration, same-origin tracking.

Filter is research-grade. The design is end-state, but the implementation is incremental.

See `docs/decisions/0005-filter-runtime-design.md` and `docs/architecture/filter/`.

### Context — capability types (`spiral-context`)

Context is Spiral's capability-typed page context. The brand is the *context* in which a page runs: not the box, but the rules of the box. The shape:

```rust
pub struct Context<'brand, Mode> {
    mode: PhantomData<Mode>,
    capabilities: CapabilitySet<'brand>,
}
```

- **Phase 1:** skeleton + types (Packet 1.6.2, shipped).
- **Phase 5:** capability inheritance, downgrade, leak detection.
- **Phase 6+:** Vortex + DOM integration.

Context is research-grade. It is the most novel part of the Spiral design — see the M4 audit methodology in `docs/audit-sprint-m4.md` and the novel claim verification rule in `AGENTS.md`.

See `docs/architecture/context/`.

---

## 4. Workspace structure — 20 crates

```
spiral-core         # shared types (BrowserConfig, TabId, IPCMessage, Error)
spiral-ipc          # cross-process messaging (UDS / named pipes, bincode, tokio)
spiral-dom          # DOM tree (Node, Element, Document; arena-allocated)
spiral-fmt          # **Fmt** — HTML5 tokeniser + tree builder, CSS parser
spiral-css          # Deprecated shim → spiral-fmt
spiral-gyre         # **Gyre** — custom layout engine
spiral-vortex       # **Vortex** — from-scratch JavaScript engine
spiral-context      # **Context** — capability-typed page context
spiral-filter       # **Filter** — compile-time HTML/CSS policy
spiral-network      # HTTP client (hyper + hickory-dns)
spiral-net          # TLS + DNS resolution wrappers (rustls, hickory-dns)
spiral-crypto       # TLS primitives wrapper
spiral-render       # Vello + wgpu render path
spiral-paint        # Display list construction
spiral-gpu          # wgpu integration
spiral-imagedecoder # PNG / JPEG / WebP / AVIF
spiral-sandbox      # OS sandbox profiles (Landlock / Seatbelt / Job Object)
spiral-ui           # Browser chrome (sidebar tabs, floating URL bar)
spiral-theme        # Zen-style design tokens
spiral-browser      # Binary entry point
```

### Dependency hierarchy (no crate depends "up")

```
spiral-core  →  spiral-ipc  →  spiral-dom  →  spiral-fmt  (Fmt)
                                     │        spiral-gyre  (Gyre)
                                     │        spiral-vortex (Vortex)
                                     │        spiral-context (Context)
                                     │        spiral-filter  (Filter)
                                     │
                spiral-browser  ←  spiral-ui  ←  spiral-theme
                                  spiral-network, spiral-net, spiral-crypto
                                  spiral-render, spiral-paint, spiral-gpu
                                  spiral-imagedecoder
                                  spiral-sandbox
```

### Per-crate role

| Crate | Role | Brand | Status |
|-------|------|-------|--------|
| `spiral-core` | Shared types (BrowserConfig, TabId, IPCMessage, Error) | — | ✅ Phase 0 |
| `spiral-ipc` | Cross-process messaging. UDS / named pipes, bincode, tokio | — | ✅ Phase 0 |
| `spiral-dom` | DOM tree, arena-allocated, parent/child by index | — | ✅ Phase 0 |
| `spiral-fmt` | Fmt — HTML5 tokeniser + tree builder, CSS parser | **Fmt** | ✅ Phase 1 |
| `spiral-css` | Deprecated shim → spiral-fmt | — | ✅ (deprecated) |
| `spiral-gyre` | Gyre — box model + block (Phase 1), flex + grid (Phase 2) | **Gyre** | ✅ Phase 1 partial |
| `spiral-vortex` | Vortex — tree-walking interpreter + mark-sweep GC | **Vortex** | ✅ Phase 1 first slice |
| `spiral-context` | Context — `Context<'brand, Mode>`, `CapabilitySet<'brand>` | **Context** | ✅ Phase 1 skeleton |
| `spiral-filter` | Filter — runtime policy engine | **Filter** | ✅ Phase 1 (Packet 1.6.4) |
| `spiral-network` | HTTP client (hyper + hickory-dns) | — | ✅ Phase 1 |
| `spiral-net` | TLS + DNS resolution wrappers | — | ✅ Phase 1 |
| `spiral-crypto` | TLS primitives wrapper | — | ✅ Phase 1 |
| `spiral-render` | Vello + wgpu render path | — | ✅ Phase 1 |
| `spiral-paint` | Display list construction | — | ✅ Phase 1 |
| `spiral-gpu` | wgpu integration | — | ✅ Phase 1 |
| `spiral-imagedecoder` | PNG / JPEG / WebP / AVIF | — | ✅ Phase 1 |
| `spiral-sandbox` | OS sandbox profiles | — | ✅ Phase 1 scaffolded |
| `spiral-ui` | Browser chrome | — | ✅ Phase 1 |
| `spiral-theme` | Zen-style design tokens | — | ✅ Phase 1 |
| `spiral-browser` | Binary entry point | — | ✅ Phase 0 |

---

## 5. IPC protocol

Cross-process messaging is over length-prefixed bincode frames. The transport is platform-specific: Unix domain sockets on Linux/macOS, named pipes on Windows. The framing is shared.

- **Frame:** `[u32 LE length][bincode payload]`
- **Payload type:** `IPCMessage` (13 variants defined in `spiral-core`).
- **Transport:** `tokio::net::UnixStream` / `tokio::net::windows::NamedPipeServer`, behind a `Transport` trait.
- **Mock transport:** `MockTransport` for tests.

See `crates/spiral-ipc/` for the implementation. Per AGENTS.md § Common Pitfalls: changing the `IPCMessage` enum breaks bincode, so any payload change requires a versioned variant.

---

## 6. Render pipeline

The end-to-end pipeline from a network response to a frame on screen:

```
Network Response (HTML bytes)
    → Fmt (HTML tokeniser + tree builder)            → DOM
    → Fmt (CSS parser)                               → Stylesheet
    → Style Resolution                               → Computed Styles
    → Gyre (layout: block, flex, grid)               → Layout Tree
    → Paint (display list construction)              → Display List
    → Render (Vello + wgpu)                          → GPU texture
    → Swap chain                                     → screen
```

Each stage runs in the renderer process. The browser process owns the tab manager, IPC router, UI chrome, and config manager. The network and GPU processes are siblings of the renderer.

Full data flow and per-stage responsibilities: [`ARCHITECTURE.md`](ARCHITECTURE.md).

---

## 7. Security model

- **Process isolation:** one renderer per tab. Sandboxed via `spiral-sandbox` (Linux Landlock + seccomp-bpf, macOS Seatbelt, Windows restricted tokens).
- **IPC framing:** u32-LE length prefix + bincode. Buffer-overflow checks at deserialisation.
- **TLS:** rustls. No OpenSSL.
- **Capability-typed context:** Phase 5+ integrates with Vortex and DOM to gate cross-origin and sensitive operations at compile time.
- **Vulnerability disclosure:** private channels only. See [`SECURITY.md`](SECURITY.md).

---

## 8. Configuration

`BrowserConfig` lives in `spiral-core`. It covers:

- User agent string (the long-term aim is to be a "real" UA, not a Chromium clone).
- Default search engine.
- Per-site permissions (geolocation, notifications, etc.) — gated by Context in Phase 5+.
- Theme (light, dark, system) — `spiral-theme`.
- Per-process resource limits (renderer memory cap, network timeouts).

---

## 9. The implementation groups (high level)

The full phase index is at [`ROADMAP.md`](ROADMAP.md). The status is at [`docs/implementation_tracker.md`](docs/implementation_tracker.md).

| Group | What it ships |
|-------|---------------|
| **Engines** | Gyre, Vortex, Fmt, Filter, Context — the engines that bear the Spiral brand. |
| **Networking** | spiral-network, spiral-crypto, spiral-ipc, spiral-sandbox — the wire-level plumbing. |
| **Presentation** | spiral-render, spiral-ui, spiral-theme — the visual layer. |
| **Cross-cutting** | spiral-core, spiral-browser (binary) — the foundation. |

Each group has its own phases. The group/phase/step/packet vocabulary is canonical; see `AGENTS.md` § Workflow Discipline for the gating contract.

---

## 10. Where the work lives

- **SSOT for status:** [`docs/implementation_tracker.md`](docs/implementation_tracker.md)
- **SSOT for live state:** [`docs/active_context.md`](docs/active_context.md)
- **Append-only change log:** [`docs/progress_ledger.md`](docs/progress_ledger.md)
- **ADRs:** [`docs/decisions/`](docs/decisions/)
- **Rule files:** [`.spiral/rules/`](.spiral/rules/)
- **Role contracts:** [`docs/agents/`](docs/agents/)
- **Architecture delta:** [`docs/system_architecture.md`](docs/system_architecture.md)
- **Architecture (canonical):** [`ARCHITECTURE.md`](ARCHITECTURE.md)
- **Phase index:** [`ROADMAP.md`](ROADMAP.md)
- **This file:** [`PLAN.md`](PLAN.md)
- **Front door:** [`README.md`](README.md)
- **LLM cheatsheet:** [`CODEX.md`](CODEX.md)
