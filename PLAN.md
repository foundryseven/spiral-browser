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

Everything else (HTML/CSS parser, paint, network, GPU) uses vendored or upstream code with our wrappers. The two engines above are the only parts of the stack that carry the Spiral brand.

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
    ├── spiral-fmt/               # Vendored Servo parsers (html5ever, cssparser, selectors)
    ├── spiral-html/              # HTML pipeline (uses spiral-fmt)
    ├── spiral-css/               # CSS cascade + specificity (uses spiral-fmt)
    ├── spiral-gyre/              # Gyre — custom layout engine (block, flex, grid)
    ├── spiral-render/            # 2D GPU renderer (Vello + wgpu)
    ├── spiral-vortex/            # Vortex — JavaScript engine (rusty_v8 / V8)
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
    └── spiral-imagedecoder/      # Image decoding (PNG, JPEG, WebP, AVIF)
```

---

## 4. Crate Dependencies

```
spiral-core        → (no deps, foundation)
spiral-ipc         → spiral-core, tokio, serde, bincode
spiral-dom         → spiral-core
spiral-fmt         → spiral-dom (vendored html5ever, cssparser, selectors internals)
spiral-html        → spiral-core, spiral-dom, spiral-fmt
spiral-css         → spiral-core, spiral-dom, spiral-fmt
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

## 6. Implementation Phases

### Phase 1: Foundation (Months 1-3) ✅ COMPLETE
**Milestone:** Cargo workspace, IPC shell, renders "Hello World"

| Week | Task | Crate | Deliverable |
|------|------|-------|-------------|
| 1-2 | Workspace setup | Cargo.toml | 18 crate stubs |
| 3-4 | Core types | spiral-core | `BrowserConfig`, `TabId`, `IPCMessage` |
| 5-6 | IPC transport | spiral-ipc | `IpcServer`, `IpcClient` |
| 7-8 | Browser shell | spiral-browser | Process spawning, tab management |
| 9-10 | Renderer shell | spiral-render | "Hello World" display list |
| 11-12 | End-to-end | all | Launch → parse → render → display |

### Phase 2: Core Engine (Months 4-12)
**Milestone:** Vendored parsers, block/flex layout, text rendering, JS engine

This phase integrates two parallel tracks. Track A (parser/vendor) and Track E
(wrapper/integrate) run alongside Track B (layout + text).

#### Month 4–6: Vendoring & Foundation

**Track A — Vendor Servo parsers into `spiral-fmt`**
| Month | Task | Crate |
|-------|------|-------|
| 4 | Vendor `html5ever` into `spiral-fmt`; modernise deps (tendril→compact_str, string_cache→owned Atoms) | spiral-fmt |
| 4 | Vendor `cssparser` + `selectors` into `spiral-fmt`; update Cargo manifests | spiral-fmt |
| 4 | Unified facade: `spiral_fmt::parse_html()`, `spiral_fmt::parse_css()` | spiral-fmt |
| 5 | Port html5ever HTML5 lib test subset; fix all build warnings | spiral-fmt |
| 5 | Port cssparser + selectors unit tests; fuzz harness for both parsers | spiral-fmt |
| 6 | Rewire `spiral-html` from `html5ever` → `spiral-fmt` | spiral-html |
| 6 | Rewire `spiral-css` from `cssparser`+`selectors` → `spiral-fmt` | spiral-css |

**Track E — Thin integration wrappers (background, parallelisable)**
| Month | Task | Crate |
|-------|------|-------|
| 4 | `spiral_net::Resolver` trait wrapping hickory-dns | spiral-net |
| 5 | `spiral_net::TlsConnector` trait wrapping rustls | spiral-net |
| 5 | `spiral_imagedecoder::Decoder` enum dispatching per-format | spiral-imagedecoder |
| 6 | `spiral_network::Client` trait wrapping hyper | spiral-network |

#### Month 6–9: DOM, Block Layout & Text

**Track B — Core layout and text**
| Month | Task | Crate |
|-------|------|-------|
| 6 | Implement spiral-dom tree structure (`Node`, `Element`, `Text`, `Document`); attribute access, parent/child relationships | spiral-dom |
| 6 | HTML→DOM pipeline: produce `Document` from bytes via spiral-fmt + spiral-dom | spiral-html |
| 7 | Box model: margin, border, padding, content area | spiral-gyre |
| 7 | Block layout: vertical stacking, margin collapse, BFC/IFC | spiral-gyre |
| 8 | Floats (left/right, clear, BFC containment); positioning (static, relative, absolute, fixed, sticky) | spiral-gyre |
| 8 | Cascade engine: origin order, specificity, `!important`, inheritance | spiral-css |
| 9 | Text shaping via harfrust, text rendering via swash, text layout via cosmic-text | spiral-render |
| 9 | Text display end-to-end: styled text in layout box, rendered to display list | spiral-render |

#### Month 10–12: Flex Layout & JS Engine

**Track B — Flex layout (custom, no Taffy)**
| Month | Task | Crate |
|-------|------|-------|
| 10 | Flex container model: main/cross axis, flex lines; `flex-direction`, `flex-wrap`, `flex-flow` | spiral-gyre |
| 10 | `justify-content`, `align-items`, `align-content`, `align-self` | spiral-gyre |
| 11 | `flex-grow`, `flex-shrink`, `flex-basis`; `min-width`/`max-width` interaction; `order` | spiral-gyre |
| 11 | WPT fixtures for block + flex layout | tests/wpt |
**Track D — JS engine (Vortex, from-scratch)**

| Month | Task | Crate |
|-------|------|-------|
| 10 | Spike: Vortex hello world — lexer + parser + tree-walking interpreter | spiral-vortex |
| 11 | `console.log`/`info`/`warn`/`error` → `RendererToBrowser::ConsoleMessage` | spiral-vortex |
| 12 | DOM bindings: `createElement`, `appendChild`, `insertBefore`; `setAttribute`, `getAttribute` | spiral-vortex |
| 12 | `addEventListener`, `dispatchEvent`; event dispatch skeleton | spiral-vortex |
| 12 | `setTimeout`, `setInterval`, `queueMicrotask` | spiral-vortex |

**Exit Criteria (Phase 2):**
- `spiral-html` and `spiral-css` depend on `spiral-fmt` only; no Servo crates in `cargo tree`
- Block layout renders: margin collapse, floats, BFC/IFC, all positioning modes
- Flex layout renders common patterns (centring, sidebar+content, wrap)
- Text is shaped and rendered correctly (basic Latin, CJK start)
- Vortex is the default JS engine — from-scratch Rust, interpreter-ready; `boa_engine` is gone from `Cargo.toml`
- DOM manipulation from JS triggers re-layout
- Basic page renders at 60fps
- WPT block layout pass rate ≥ 40%; flex pass rate ≥ 30%

### Phase 3: Full Engine (Months 13-21)
**Milestone:** Grid layout, networking, HTTP navigation, form submission, image decoding

#### Month 13–15: Grid Layout (custom)

| Month | Task | Crate |
|-------|------|-------|
| 13 | Grid container: explicit tracks, implicit tracks, `grid-template-columns/rows` | spiral-gyre |
| 13 | Grid template areas, named lines, line-based placement | spiral-gyre |
| 14 | `grid-auto-flow`, `grid-gap`, span placement | spiral-gyre |
| 14 | `subgrid` (Level 2) | spiral-gyre |
| 15 | WPT grid fixtures; pass target ≥ 40% of `css/css-grid/` | tests/wpt |

#### Month 15–18: Networking

| Month | Task | Crate |
|-------|------|-------|
| 15-16 | HTTP client via hyper (through `spiral_network::Client`) | spiral-network |
| 16 | DNS resolution via hickory-dns (through `spiral_net::Resolver`) | spiral-net |
| 17 | TLS via rustls (through `spiral_net::TlsConnector`); certificate verification | spiral-net |
| 17 | Connection pooling, redirect following, cookie jar | spiral-network |
| 18 | Form submission (GET/POST); `FormData` support | spiral-network |

#### Month 18–21: DOM Polish & Image Decoding

| Month | Task | Crate |
|-------|------|-------|
| 18 | (N/A — Gyre is in-house from day one; no Taffy to remove) | — |
| 18 | Image decoding pipeline (through `spiral_imagedecoder::Decoder`) | spiral-imagedecoder |
| 19 | PNG, JPEG, WebP, AVIF support; lazy + progressive loading | spiral-imagedecoder |
| 19 | DOM manipulation from JS: `removeChild`, `textContent`, `style` | spiral-vortex |
| 20 | CSS matching on DOM mutation; incremental re-style | spiral-css, spiral-dom |
| 21 | Full event dispatch: `MouseEvent`, `KeyboardEvent`, `FocusEvent` | spiral-vortex, spiral-dom |

**Exit Criteria (Phase 3):**
- Grid layout renders common patterns (holy grail, dashboard, gallery)
- Can navigate to real websites over HTTP/HTTPS
- JavaScript executes DOM manipulation; events fire correctly
- Forms submit; cookies persist; images load and display
- `taffy` was never added to `Cargo.toml` — Gyre is custom from day one
- WPT grid pass rate ≥ 40%
- Layout benchmarks within 2× of Taffy on representative pages

### Phase 4: UI & Polish (Months 22-33)
**Milestone:** Zen UI, GPU rendering, Vello optimisation, security sandboxing

| Month | Task | Crate |
|-------|------|-------|
| 22-23 | Zen-style theme engine; design tokens; light/dark mode; system preference detection | spiral-theme |
| 24-25 | Sidebar tabs UI: creation, switching, closing, drag/reorder, context menu | spiral-ui |
| 26-27 | Floating URL bar: autocomplete, navigation buttons, settings panel | spiral-ui |
| 28-29 | GPU rendering pipeline via Vello; display list → Vello scene → swap chain | spiral-render |
| 29-30 | Vello fork: tile-based picture caching; dirty-rect invalidation; scroll at 120fps | spiral-render (vello fork) |
| 30 | `trait JSRuntime` abstraction in Vortex; Phase 1 interpreter → Phase 2 bytecode VM → Phase 3 baseline JIT | spiral-vortex |
| 31-32 | Platform sandboxing: Linux seccomp-bpf + Landlock, macOS Seatbelt, Windows Restricted Token | spiral-sandbox |
| 33 | DevTools basics: element inspector, console output, network panel | spiral-ui |

**Exit Criteria (Phase 4):**
- Zen-inspired UI is functional: sidebar tabs, floating URL bar, accent colours
- GPU rendering is smooth; scrolling at 120fps on integrated graphics
- Vello picture cache memory overhead < 200 MB for typical pages
- `trait JSRuntime` is defined; Vortex is the default engine; V8 oracle available behind `v8` feature for CI compliance
- Sandbox is active on all platforms
- DevTools shows DOM tree and console output

### Phase 5: Production (Months 34-39)
**Milestone:** WPT compliance, performance, release

| Month | Task | Crate |
|-------|------|-------|
| 34-35 | WPT test integration; layout test pass target ≥ 50% | tests/wpt |
| 35 | Performance benchmarks: layout <1ms, render <16.67ms (60fps), IPC <1ms round-trip | benches/ |
| 36 | Cross-platform packaging: Linux AppImage, macOS .app, Windows installer | CI/CD |
| 37 | Security audit, fuzzing, penetration testing; fix findings | all |
| 38-39 | v0.1.0 release: release notes, documentation, blog post | all |

**Exit Criteria (Phase 5):**
- v0.1.0 released on all platforms
- 50%+ WPT pass rate
- 60fps rendering
- No known security vulnerabilities

---

## 7. Technology Choices

| Component | Choice | Reason |
|-----------|--------|--------|
| Language | Rust | Memory safety, modern ecosystem |
| Windowing | winit | Cross-platform, Rust-native |
| GPU Abstraction | wgpu | Vulkan/Metal/DX12/OpenGL |
| 2D Rendering | Vello | GPU compute, modern pipeline |
| HTML Parsing | spiral-fmt (vendored html5ever) | Owned, maintained by us; upstream stale since 2021 |
| CSS Parsing | spiral-fmt (vendored cssparser + selectors) | Owned, maintained by us; upstream stale since 2022 |
| Layout | Custom (Gyre / `spiral-gyre`) | Full CSS spec parity; in-house from day one, no Taffy |
| Text Shaping | harfrust (HarfBuzz) | Industry standard, pure Rust |
| Text Rendering | swash | Rust-native, CPU-efficient |
| Text Layout | cosmic-text | Full text pipeline |
| JS Engine | Vortex (from-scratch Rust) | Custom lexer, parser, AST, bytecode VM, mark-sweep GC, future baseline JIT; V8 oracle behind `v8` feature for CI only |
| HTTP | hyper | Production-grade, Rust-native, wrapped in spiral-network |
| TLS | rustls | Memory-safe, no OpenSSL, wrapped in spiral-net |
| DNS | hickory-dns | Rust-native, full resolver, wrapped in spiral-net |
| Serialization | serde + bincode | Fast, type-safe IPC |
| Async | tokio | Industry standard |
| Image Loading | png, zune-jpeg, webp, ravif | Per-format decoders, wrapped in spiral-imagedecoder |

---

## 8. Testing Strategy

| Level | Tool | Scope |
|-------|------|-------|
| Unit | `cargo test` | Per-crate, isolated |
| Integration | `cargo test --workspace` | Cross-crate flows |
| WPT | `tests/wpt/` | Web standards compliance |
| Bench | `criterion` | Layout, render, IPC throughput |
| CI | GitHub Actions | Linux/macOS/Windows matrix |
| Lint | `clippy` | Code quality |
| Format | `rustfmt` | Consistent style |

---

## 9. Success Criteria

- [ ] Launches on Windows, macOS, Linux
- [ ] Renders basic HTML pages (text, images, links)
- [ ] Block layout correct (margin collapse, floats, BFC/IFC, positioning)
- [ ] Flexbox layout works for common patterns
- [ ] Grid layout works for common patterns
- [ ] CSS cascade and specificity correct
- [ ] JavaScript execution via Vortex (console, DOM, events)
- [ ] HTTP/HTTPS navigation with TLS
- [ ] Multi-process architecture (tab isolation)
- [ ] Zen-inspired UI (sidebar tabs, floating URL bar)
- [ ] Passes 50%+ of WPT layout tests
- [ ] 60fps rendering for static pages
- [ ] Servo upstream crates fully vendored (no stale deps in tree)
- [ ] Taffy removed from Cargo.toml (layout is fully custom)
- [ ] JS engine is JIT-capable (Vortex = from-scratch Rust, future baseline JIT)
