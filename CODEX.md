# Spiral Browser — Quick Reference for LLMs

## Project Overview
- **Name:** Spiral Browser
- **Language:** Rust (edition 2021)
- **License:** MPL-2.0
- **Platforms:** Windows, macOS, Linux
- **Scope:** Independent browser (not Chromium/WebKit/Gecko)

## Repository Structure
```
├── Cargo.toml              # Workspace root
├── AGENTS.md               # LLM instructions
├── ARCHITECTURE.md         # System design
├── PLAN.md                 # Implementation plan
├── ROADMAP.md              # Development phases
├── TESTING.md              # Test guide
├── BUILD.md                # Build instructions
├── ERRORS.md               # Common errors
├── CODEX.md                # This file
├── CONTRIBUTING.md         # Contribution guide
├── crates/                 # 19 Rust crates
│   ├── spiral-core/        # Shared types
│   ├── spiral-browser/     # Main process
│   ├── spiral-fmt/         # Vendored Servo parsers (html5ever, cssparser, selectors)
│   ├── spiral-html/        # HTML pipeline (uses spiral-fmt)
│   ├── spiral-css/         # CSS cascade (uses spiral-fmt)
│   ├── spiral-gyre/        # Gyre — custom layout engine (block, flex, grid)
│   ├── spiral-render/      # GPU renderer
│   ├── spiral-vortex/      # Vortex — JavaScript engine (rusty_v8 / V8)
│   ├── spiral-dom/         # DOM tree
│   ├── spiral-paint/       # Display list
│   ├── spiral-network/     # HTTP client (hyper, wrapped)
│   ├── spiral-ipc/         # IPC transport
│   ├── spiral-sandbox/     # Sandboxing
│   ├── spiral-ui/          # Browser chrome
│   ├── spiral-theme/       # Theme engine
│   ├── spiral-net/         # TLS/DNS wrappers
│   ├── spiral-crypto/      # Crypto wrapper
│   ├── spiral-gpu/         # GPU abstraction
│   └── spiral-imagedecoder/# Image formats
├── tests/wpt/              # Web Platform Tests
└── benches/layout/         # Benchmarks
```

## Essential Commands
```bash
cargo build                  # Build all crates
cargo test --workspace       # Run all tests
cargo clippy --workspace     # Lint check
cargo fmt --check           # Format check
cargo bench                  # Run benchmarks
```

## Dependency Graph
```
spiral-core (foundation)
├── spiral-ipc
├── spiral-dom
│   ├── spiral-fmt (vendored html5ever, cssparser, selectors)
│   │   ├── spiral-html
│   │   └── spiral-css
│   │       └── spiral-gyre
│   │           └── spiral-paint
│   │               └── spiral-render
│   └── spiral-vortex (rusty_v8)
├── spiral-gpu
│   └── spiral-render
├── spiral-network (hyper, wrapped)
│   └── spiral-net (rustls, hickory-dns, wrapped)
├── spiral-crypto
├── spiral-imagedecoder
├── spiral-sandbox
├── spiral-theme
├── spiral-ui
└── spiral-browser
```

## Key Types
```rust
// spiral-core/src/lib.rs
pub struct BrowserConfig { /* ... */ }
pub struct TabId(pub u64);
pub struct RenderNodeId(pub u64);

// spiral-ipc/src/lib.rs
pub struct IpcServer { /* ... */ }
pub struct IpcClient { /* ... */ }

// spiral-dom/src/lib.rs
pub enum Node { Element(Element), Text(Text), Comment(Comment), Document(Document) }
pub struct Element { tag: String, attributes: Vec<(String, String)>, children: Vec<NodeId> }

// spiral-gyre/src/lib.rs
pub struct BoxModel { margin: EdgeSizes, border: EdgeSizes, padding: EdgeSizes, content: Rect }

// spiral-render/src/lib.rs
pub enum RenderOp { FillRect, StrokeRect, DrawText, Clip, Transform, PushLayer, PopLayer }
```

## IPC Protocol
```rust
enum BrowserToRenderer {
    Navigate { url: String },
    UpdateDOM { node_id: u64, operations: Vec<DomOp> },
    Resize { width: f32, height: f32 },
    InputEvent { event: InputEvent },
    Reload,
    Stop,
}

enum RendererToBrowser {
    DOMLoaded { title: String },
    LoadProgress { progress: f32 },
    NavigateComplete { url: String },
    RequestNavigate { url: String },
    ConsoleMessage { level: LogLevel, text: String },
}
```

## Coding Conventions
- **Functions/variables:** `snake_case`
- **Types/structs/enums:** `PascalCase`
- **Modules:** `snake_case`
- **Constants:** `SCREAMING_SNAKE_CASE`
- **Error handling:** `?` operator, no `.unwrap()` in library code
- **Imports:** std → external crates → internal crates
- **Tests:** `#[cfg(test)] mod tests` in same file

## Platform-Specific Code
```rust
#[cfg(target_os = "linux")]
fn linux_only() { /* ... */ }

#[cfg(target_os = "macos")]
fn macos_only() { /* ... */ }

#[cfg(target_os = "windows")]
fn windows_only() { /* ... */ }
```

## Testing
```bash
# Unit tests
cargo test

# Specific crate
cargo test spiral-core

# Specific test
cargo test test_box_model

# With output
cargo test -- --nocapture

# Benchmarks
cargo bench
```

## Current Phase
**Phase 2: Core Engine** (Months 4-12)
- [ ] Vendor html5ever + cssparser + selectors into spiral-fmt
- [ ] Block layout (custom)
- [ ] Flex layout (custom, no Taffy)
- [ ] Text rendering (harfrust + swash + cosmic-text)
- [ ] Vortex JS engine integration (from-scratch: lexer, parser, interpreter, bytecode VM)

## Key Technologies
| Component | Crate | Purpose |
|-----------|-------|---------|
| HTML | spiral-fmt (vendored html5ever) | Parse HTML5 (owned) |
| CSS | spiral-fmt (vendored cssparser + selectors) | Parse CSS (owned) |
| Layout | spiral-gyre / Gyre (custom) | Block/Flex/Grid (in-house, no Taffy) |
| Render | vello, wgpu | GPU 2D rendering |
| JS | spiral-vortex (from-scratch) | JavaScript — lexer, parser, AST, bytecode VM, GC, future JIT |
| HTTP | hyper | HTTP client (wrapped in spiral-network) |
| TLS | rustls | TLS 1.3 (wrapped in spiral-net) |
| DNS | hickory-dns | DNS resolution (wrapped in spiral-net) |
| IPC | tokio, bincode | Process communication |

## License
MPL-2.0 — compatible with Servo crates.
