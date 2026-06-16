# Spiral Browser — Implementation Plan

**Project:** Spiral Browser
**Language:** Rust
**License:** MPL-2.0
**Target Platforms:** Windows, macOS, Linux
**Scope:** Full production browser (6–8 year roadmap)

---

## 1. Project Vision

Spiral is a fully independent web browser built from scratch in Rust. Not based on Chromium, WebKit, or Gecko. Features a custom rendering engine, custom layout engine, and the **Vortex** JavaScript engine — a from-scratch Rust JS engine with its own lexer, parser, AST, bytecode compiler, and interpreter. Zen-browser-inspired UI: vertical sidebar tabs, floating URL bar, and single-accent-colour theming.

### Engine Identity

Spiral's stack has two custom-built engines that bear the Spiral brand:

| Engine | Crate | Role | Architecture |
|--------|-------|------|--------------|
| **Gyre** | `spiral-gyre` | Layout (block, flex, grid) | Fully in-house Rust. No Taffy. |
| **Vortex** | `spiral-vortex` | JavaScript | From-scratch Rust JS engine. `rusty_v8` available behind `v8` feature for CI compliance testing only. |

Vortex is **not** a V8 wrapper. It is a ground-up Rust JavaScript engine with its own lexer, parser, AST, bytecode compiler, mark-sweep GC, and (future) baseline JIT — comparable in ambition to Ladybird's LibJS or QuickJS. Google's V8 (`rusty_v8`) is available only behind a `v8` Cargo feature flag for CI compliance testing: the test harness runs the same JS snippets through both Vortex and V8 and compares outputs.

Everything else (HTML/CSS parser, paint, network, GPU) is written in Spiral-native Rust with thin upstream wrappers (e.g. hyper, hickory-dns, rustls, vello, wgpu). The two engines above — plus the from-spec HTML+CSS parser in `spiral-fmt` (which the docs/glossary calls **Fmt**) — are the parts of the stack that carry the Spiral brand.

### Core Principles
- **Independence:** No browser engine dependencies (Chromium/WebKit/Gecko)
- **Open Source:** MPL-2.0 license, all dependencies permissive/open-source
- **Security:** Per-platform sandboxing, memory-safe Rust
- **Performance:** GPU-accelerated rendering, multi-process architecture
- **Design:** Zen-browser-inspired clean UI, auto-hide chrome, accent colors

---

## 2. Architecture Summary

```
┌─────────────────────────────────────────────────┐
│                  Browser Process                 │
│  (Tab management, IPC routing, UI chrome)        │
├──────────┬──────────┬──────────┬────────────────┤
│ Renderer │ Renderer │ Network  │ GPU            │
│ Process  │ Process  │ Process  │ Process        │
│ (per-tab)│ (per-tab)│ (HTTP)   │ (wgpu/vello)   │
└──────────┴──────────┴──────────┴────────────────┘
```

- **Browser Process:** Main entry, spawns child processes, manages tabs, renders UI chrome
- **Renderer Process:** One per tab, parses HTML/CSS, computes layout, builds display list
- **Network Process:** HTTP client, DNS resolution, TLS, connection pooling
- **GPU Process:** wgpu device management, Vello rendering, texture allocation

**IPC:** Unix domain sockets (Linux/macOS), named pipes (Windows), tokio async, bincode serialization.

**Sandboxing:** seccomp-bpf + Landlock (Linux), Seatbelt (macOS), Restricted Token + Job Object (Windows).

---

## 3. Crate Structure

```
C:\Browser Project\
├── Cargo.toml                    # Workspace root
├── .github/workflows/ci.yml     # CI/CD
├── resources/
│   ├── icons/                    # App icons (ICO, ICNS, SVG)
│   ├── fonts/                    # Bundled fonts
│   ├── locales/                  # i18n translation files
│   └── profiles/                 # Default profile templates
├── tests/
│   └── wpt/                      # Web Platform Tests integration
├── benches/
│   └── layout/                   # Layout engine benchmarks
└── crates/
    ├── spiral-core/              # Shared types, IPC protocol, config
    ├── spiral-browser/           # Browser process (main entry, tab management)
    ├── spiral-fmt/               # From-spec HTML5 tokeniser + tree builder, CSS parser
    ├── spiral-css/               # Deprecated shim → spiral-fmt
    ├── spiral-gyre/              # Gyre — custom layout engine (block, flex, grid)
    ├── spiral-render/            # 2D GPU renderer (Vello + wgpu)
    ├── spiral-vortex/            # Vortex — from-scratch JavaScript engine
    ├── spiral-dom/               # DOM tree (Node, Element, Document)
    ├── spiral-paint/             # Display list + compositing
    ├── spiral-network/           # HTTP client (hyper + hickory-dns), wrapped
    ├── spiral-ipc/               # IPC transport layer
    ├── spiral-sandbox/           # Process sandboxing (Landlock/Seatbelt/JobObject)
    ├── spiral-ui/                # Browser chrome (tabs, URL bar, controls)
    ├── spiral-theme/             # Theme engine (Zen-style tokens)
    ├── spiral-net/               # TLS + DNS resolution wrappers (rustls, hickory-dns)
    ├── spiral-crypto/            # TLS primitives wrapper
    ├── spiral-gpu/               # GPU abstraction (wgpu)
    ├── spiral-imagedecoder/      # Image decoding (PNG, JPEG, WebP, AVIF)
    ├── spiral-context/           # Capability-typed API (Bet 1)
    └── spiral-filter/            # Compile-time HTML/CSS policy (Bet 3)
```

---

## 4. Crate Dependencies

```
spiral-core        → (no deps, foundation)
spiral-ipc         → spiral-core, tokio, serde, bincode
spiral-dom         → spiral-core
spiral-fmt         → spiral-dom  (from-spec HTML5 + CSS; no html5ever, no cssparser)
spiral-css         → spiral-core, spiral-dom, spiral-fmt  (deprecated shim → spiral_fmt)
spiral-gyre        → spiral-core, spiral-css, spiral-dom  (custom layout; no Taffy)
spiral-paint       → spiral-core, spiral-dom, spiral-gyre
spiral-render      → spiral-core, spiral-paint, vello, wgpu, spiral-gpu
spiral-gpu         → spiral-core, wgpu
spiral-vortex      → spiral-core, spiral-dom, rusty_v8 (optional, behind `v8` feature for CI oracle)
spiral-network     → spiral-core, hyper, hickory-dns, rustls
spiral-net         → spiral-core, rustls, hickory-dns (thin wrappers, no re-export)
spiral-crypto      → spiral-core, rustls
spiral-imagedecoder→ spiral-core, png, zune-jpeg, webp, ravif
spiral-sandbox     → spiral-core, rustix (Landlock), caps (capabilities)
spiral-context     → spiral-core  (capability-typed API)
spiral-filter      → spiral-core  (compile-time policy)
spiral-ui          → spiral-core, spiral-gpu, spiral-theme
spiral-theme       → spiral-core, serde
spiral-browser     → spiral-core, spiral-ipc, spiral-ui, spiral-theme
```

---

## 5. IPC Protocol

### Message Types

```rust
// Browser → Renderer
enum BrowserToRenderer {
    Navigate { url: String },
    UpdateDOM { node_id: u64, operations: Vec<DomOp> },
    Resize { width: f32, height: f32 },
    InputEvent { event: InputEvent },
    Reload,
    Stop,
}

// Renderer → Browser
enum RendererToBrowser {
    DOMLoaded { title: String },
    LoadProgress { progress: f32 },
    NavigateComplete { url: String },
    RequestNavigate { url: String },
    ConsoleMessage { level: LogLevel, text: String },
}
```

### Transport
- Linux/macOS: Unix domain sockets in `$XDG_RUNTIME_DIR/spiral/`
- Windows: Named pipes `\\.\pipe\spiral-{pid}`
- Serialization: bincode (length-prefixed)
- Async runtime: tokio

---


## 6. Implementation Phases (pointer)

> **This section was rewritten 2026-06-16 during the SSOT restructure.**
>
> The old "Month X" table that previously lived here has been removed.
> Status, sequencing, and task breakdown are SSOT in
> [`docs/implementation_tracker.md`](docs/implementation_tracker.md)
> (Group → Phase → Step → Packet). The time-based Month / Sprint / Chunk
> / Item vocabulary is retired.
>
> The plan below is **architectural and strategic**: which subsystems
> exist, what they own, and which cross-cutting decisions have been
> taken. Implementation status is the tracker's job.

### Phase 0 — Foundation ✅ COMPLETE
Crate workspace, IPC shell, hello-world render. See
[`docs/implementation_tracker.md` § Phase 0](docs/implementation_tracker.md).

### Phase 1 — Engines Foundation 🔄 IN FLIGHT
Vortex, spiral-fmt (HTML + CSS), Gyre (box model), spiral-filter.
See [`docs/implementation_tracker.md` § Phase 1](docs/implementation_tracker.md).

### Phase 2+ — Forward-projected
Engines depth (fragment parsing, DOM collections, dataset, structuredClone,
URL), Networking, Presentation, Capability types runtime, Bytecode VM,
Media + DRM, Persistent renderer, Hardening. See
[`docs/implementation_tracker.md` § Phases 2–9](docs/implementation_tracker.md).

### Exit criteria
The Phase-by-Phase exit criteria are documented in
[`docs/implementation_tracker.md`](docs/implementation_tracker.md) under
each Phase's "Wiring & Integration" subsection.

### What "v0.1.0" means
Per the release model in [`docs/agents/release.md`](docs/agents/release.md),
v0.1.0 will be tagged when a Phase boundary is crossed with all packets
closed. There is no calendar deadline. The semantic version follows the
Phase → version mapping convention adopted 2026-06-16.
