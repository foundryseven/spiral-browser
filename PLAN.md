# Spiral Browser — Implementation Plan

**Project:** Spiral Browser
**Language:** Rust
**License:** MPL-2.0
**Target Platforms:** Windows, macOS, Linux
**Scope:** Full production browser (3-5 year roadmap)

---

## 1. Project Vision

Spiral is a fully independent web browser built from scratch in Rust. Not based on Chromium, WebKit, or Gecko. Features a custom rendering engine, custom JavaScript engine, and Zen-browser-inspired UI with vertical sidebar tabs, floating URL bar, and single-accent-color theming.

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
    ├── spiral-html/              # HTML5 parser (wraps html5ever)
    ├── spiral-css/               # CSS parser (wraps cssparser + selectors)
    ├── spiral-layout/            # Layout engine (box model + Taffy)
    ├── spiral-render/            # 2D GPU renderer (Vello + wgpu)
    ├── spiral-js/                # JavaScript engine (Boa wrapper)
    ├── spiral-dom/               # DOM tree (Node, Element, Document)
    ├── spiral-paint/             # Display list + compositing
    ├── spiral-network/           # HTTP client (hyper + hickory-dns)
    ├── spiral-ipc/               # IPC transport layer
    ├── spiral-sandbox/           # Process sandboxing (Landlock/Seatbelt/JobObject)
    ├── spiral-ui/                # Browser chrome (tabs, URL bar, controls)
    ├── spiral-theme/             # Theme engine (Zen-style tokens)
    ├── spiral-net/               # TLS + DNS resolution
    ├── spiral-crypto/            # TLS primitives
    ├── spiral-gpu/               # GPU abstraction (wgpu)
    └── spiral-imagedecoder/      # Image decoding (PNG, JPEG, WebP, AVIF)
```

---

## 4. Crate Dependencies

```
spiral-core        → (no deps, foundation)
spiral-ipc         → spiral-core, tokio, serde, bincode
spiral-dom         → spiral-core
spiral-html        → spiral-core, spiral-dom, html5ever
spiral-css         → spiral-core, spiral-dom, cssparser, selectors
spiral-layout      → spiral-core, spiral-css, spiral-dom, taffy
spiral-paint       → spiral-core, spiral-dom, spiral-layout
spiral-render      → spiral-core, spiral-paint, vello, wgpu, spiral-gpu
spiral-gpu         → spiral-core, wgpu
spiral-js          → spiral-core, spiral-dom, boa_engine
spiral-network     → spiral-core, hyper, hickory-dns, rustls
spiral-net         → spiral-core, spiral-network, rustls
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

### Phase 1: Foundation (Months 1-3)
**Milestone:** Cargo workspace, IPC shell, renders "Hello World"

| Week | Task | Crate | Deliverable |
|------|------|-------|-------------|
| 1-2 | Workspace setup | Cargo.toml | 18 crate stubs |
| 3-4 | Core types | spiral-core | `BrowserConfig`, `TabId`, `IPCMessage` |
| 5-6 | IPC transport | spiral-ipc | `IpcServer`, `IpcClient` |
| 7-8 | Browser shell | spiral-browser | Process spawning, tab management |
| 9-10 | Renderer shell | spiral-render | "Hello World" display list |
| 11-12 | End-to-end | all | Launch → parse → render → display |

### Phase 2: Core Engine (Months 4-9)
**Milestone:** CSS box model, flexbox, text rendering, basic page load

| Month | Task | Crate |
|-------|------|-------|
| 4 | HTML parser integration | spiral-html |
| 5 | DOM tree construction | spiral-dom |
| 6 | CSS parser + cascade | spiral-css |
| 7 | Box model + block layout | spiral-layout |
| 8 | Flexbox via Taffy | spiral-layout |
| 9 | Text shaping (harfrust) + rendering (swash) | spiral-render |

### Phase 3: Full Engine (Months 10-18)
**Milestone:** Full CSS layout, JS engine, networking, form submission

| Month | Task | Crate |
|-------|------|-------|
| 10-11 | CSS Grid via Taffy | spiral-layout |
| 12-13 | Boa JS engine integration | spiral-js |
| 14-15 | HTTP client + DNS | spiral-network |
| 16 | TLS (rustls) | spiral-net |
| 17 | Cookie jar, redirects, form submission | spiral-network |
| 18 | Image decoding pipeline | spiral-imagedecoder |

### Phase 4: UI & Polish (Months 19-30)
**Milestone:** Zen UI, GPU rendering, security sandboxing

| Month | Task | Crate |
|-------|------|-------|
| 19-20 | Zen-style theme engine | spiral-theme |
| 21-22 | Sidebar tabs UI | spiral-ui |
| 23-24 | Floating URL bar | spiral-ui |
| 25-26 | GPU rendering pipeline | spiral-render |
| 27-28 | Platform sandboxing | spiral-sandbox |
| 29-30 | DevTools basics | spiral-ui |

### Phase 5: Production (Months 31-36)
**Milestone:** WPT compliance, performance, release

| Month | Task | Crate |
|-------|------|-------|
| 31-32 | WPT test integration | tests/wpt |
| 33 | Performance benchmarks | benches/layout |
| 34 | Cross-platform packaging | .github/workflows |
| 35 | Security audit | all |
| 36 | v0.1.0 release | all |

---

## 7. Technology Choices

| Component | Choice | Reason |
|-----------|--------|--------|
| Language | Rust | Memory safety, modern ecosystem |
| Windowing | winit | Cross-platform, Rust-native |
| GPU Abstraction | wgpu | Vulkan/Metal/DX12/OpenGL |
| 2D Rendering | Vello | GPU compute, modern pipeline |
| HTML Parsing | html5ever | Servo crate, production-tested |
| CSS Parsing | cssparser + selectors | Servo crate, W3C compliant |
| Layout | Taffy | Flexbox + Grid, Rust-native |
| Text Shaping | harfrust (HarfBuzz) | Industry standard |
| Text Rendering | swash | Rust-native, CPU-efficient |
| Text Layout | cosmic-text | Full text pipeline |
| JS Engine | Boa | Pure Rust, >90% test262 |
| HTTP | hyper | Production-grade, Rust-native |
| TLS | rustls | Memory-safe, no OpenSSL |
| DNS | hickory-dns | Rust-native, full resolver |
| Serialization | serde + bincode | Fast, type-safe IPC |
| Async | tokio | Industry standard |
| Image Loading | png, zune-jpeg, webp, ravif | Per-format decoders |

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
- [ ] Flexbox layout works
- [ ] CSS cascade and specificity correct
- [ ] Basic JavaScript execution (console.log, DOM manipulation)
- [ ] HTTP navigation with TLS
- [ ] Multi-process architecture (tab isolation)
- [ ] Zen-inspired UI (sidebar tabs, floating URL bar)
- [ ] Passes 50%+ of WPT layout tests
- [ ] 60fps rendering for static pages
