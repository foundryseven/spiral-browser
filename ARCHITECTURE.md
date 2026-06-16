# Spiral Browser — Architecture

## Overview

Spiral uses a multi-process architecture with separate processes for browsing, rendering, networking, and GPU operations. Each renderer process is isolated per-tab for security.

---

## Process Model

```
┌──────────────────────────────────────────────────────────┐
│                    Browser Process                        │
│  ┌─────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐ │
│  │ Tab Mgr │ │ IPC Rtr  │ │ UI Chrome│ │ Config Mgr   │ │
│  └─────────┘ └──────────┘ └──────────┘ └──────────────┘ │
├──────────────────────────────────────────────────────────┤
│                     IPC Layer                            │
│  ┌─────────────────────────────────────────────────────┐ │
│  │ Unix Domain Sockets (Linux/macOS)                   │ │
│  │ Named Pipes (Windows)                               │ │
│  │ bincode serialization + length prefix framing       │ │
│  └─────────────────────────────────────────────────────┘ │
├────────┬────────┬────────────┬──────────────────────────┤
│Render-1│Render-2│  Network   │         GPU              │
│ (Tab1) │ (Tab2) │  Process   │       Process            │
│┌──────┐│┌──────┐│┌──────────┐│┌────────────────────────┐│
││ HTML │││ HTML │││ HTTP/1.1 │││ wgpu Device/Queue      ││
││ CSS  │││ CSS  │││ HTTP/2   │││ Vello Renderer         ││
││Layout│││Layout│││ DNS/TLS  │││ Texture Management     ││
││ Paint│││ Paint│││ Cookies  │││ Swap Chain             ││
│└──────┘│└──────┘│└──────────┘│└────────────────────────┘│
└────────┴────────┴────────────┴──────────────────────────┘
```

---

## Renderer Process Pipeline

Each renderer process follows this pipeline:

```
Network Response (HTML bytes)
        │
        ▼
┌───────────────┐
│  HTML Parser  │  spiral-fmt (vendored html5ever) → DOM Tree
│  (spiral-html)│
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  CSS Parser   │  spiral-fmt (vendored cssparser + selectors) → Style Rules
│  (spiral-css) │
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  Style        │  Cascade + Specificity → Computed Styles
│  Resolution   │
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  Layout       │  Box Model + custom flex/grid → Layout Tree
│(spiral-gyre) │  (Gyre — custom block, flex, grid)
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  Paint        │  Layout Tree → Display List (RenderOps)
│ (spiral-paint)│  (FillRect, DrawText, Clip, Transform)
└───────┬───────┘
        │
        ▼
┌───────────────┐
│  Render       │  Display List → GPU Texture → Screen
│(spiral-render)│  (Vello + wgpu)
└───────────────┘
```

---

## Crate Responsibilities

### spiral-core
Foundation types used by all crates.
- `BrowserConfig`: startup settings (homepage, proxy, font size)
- `TabId`: unique tab identifier (u64 newtype)
- `RenderNodeId`: unique node identifier (u64 newtype)
- `IPCMessage`: serialization envelope for IPC
- `Error`: unified error type

### spiral-ipc
Transport layer for inter-process communication.
- `IpcServer`: listens for renderer connections
- `IpcClient`: connects to browser process
- `MessageFraming`: length-prefixed bincode
- Platform-specific socket/pipe implementation

### spiral-html
HTML5 parser.
- Uses `spiral-fmt` (vendored html5ever, maintained by Spiral)
- Produces `spiral_dom::Document` tree
- Handles encoding detection (UTF-8, Latin-1, etc.)
- Supports `<!DOCTYPE html>`, fragments

### spiral-css
CSS parser and cascade engine.
- Uses `spiral-fmt` (vendored cssparser + selectors, maintained by Spiral)
- Parses stylesheets, media queries, selectors
- Computes cascade order (origin, specificity)
- Resolves `!important`, inheritance

### spiral-gyre (Gyre)
Gyre is Spiral's custom, in-house layout engine. Box model, block flow,
floats, BFC/IFC, flex, and grid are all implemented in Rust by us; no
Taffy, no Servo layout code.

- Box model: `margin`, `border`, `padding`, `content`
- Block layout: normal flow, floats, BFC/IFC
- Flexbox: custom implementation (Phase 2, Month 10-11)
- Grid: custom implementation (Phase 3, Month 13-14)
- CSS values: `Length`, `Percentage`, `Auto`

### spiral-dom
DOM tree representation.
- `Node` enum: `Element`, `Text`, `Comment`, `Document`
- `Element`: tag name, attributes, children
- `Document`: root node, quirks mode
- Tree manipulation: `append_child`, `remove_child`

### spiral-render
2D GPU rendering pipeline.
- Vello for GPU compute rendering
- wgpu for graphics abstraction
- Display list execution
- Texture management

### spiral-paint
Display list construction.
- `RenderOp` enum: `FillRect`, `StrokeRect`, `DrawText`, `Clip`, `Transform`
- Z-ordering and layer composition
- Dirty rect tracking

### spiral-vortex (Vortex)
Vortex is Spiral's **from-scratch** JavaScript engine, written entirely in
safe Rust. It implements ECMAScript from the ground up: lexer, parser, AST,
bytecode compiler, interpreter, mark-sweep GC, and (future) a baseline JIT.

- Phase 1 (tree-walking interpreter): lex → parse → AST → walk
- Phase 2 (bytecode VM): AST → bytecode → stack-based interpreter
- Phase 3 (baseline JIT): Cranelift for hot functions
- `trait JSRuntime` abstraction for future engine swapping
- `rusty_v8` available behind `v8` feature flag for CI compliance testing
- DOM bindings: `createElement`, `appendChild`, `setAttribute`, etc.
- Event system: `addEventListener`, `dispatchEvent`
- Console: `console.log`/`info`/`warn`/`error`
- Timers: `setTimeout`, `setInterval`, `queueMicrotask`

### spiral-network
HTTP client and networking.
- hyper for HTTP/1.1 and HTTP/2
- hickory-dns for DNS resolution
- Connection pooling
- Redirect following
- Cookie jar

### spiral-net
TLS and DNS resolution.
- rustls for TLS
- Certificate verification
- Connection upgrade from HTTP to HTTPS

### spiral-gpu
GPU abstraction layer.
- wgpu device/queue management
- Surface creation per platform
- Texture allocation

### spiral-imagedecoder
Image format support.
- PNG (png crate)
- JPEG (zune-jpeg)
- WebP (webp crate)
- AVIF (ravif)
- Lazy progressive loading

### spiral-sandbox
Process sandboxing.
- Linux: seccomp-bpf + Landlock
- macOS: Seatbelt profiles
- Windows: Restricted Token + Job Object
- Capability-based restrictions

### spiral-ui
Browser chrome UI.
- Vertical sidebar tabs
- Floating URL bar
- Navigation buttons
- Settings panel
- GPU-rendered

### spiral-theme
Theme engine.
- Zen-style design tokens
- Single accent color system
- Light/dark mode
- CSS custom properties

### spiral-browser
Main browser process.
- Process spawning and management
- Tab lifecycle
- IPC message routing
- Entry point (`main.rs`)

---

## Data Flow: Loading a Page

1. User enters URL in spiral-ui URL bar
2. spiral-ui sends `BrowserToRenderer::Navigate { url }` via spiral-ipc
3. Browser process routes message to target renderer
4. Renderer sends HTTP request via spiral-network
5. spiral-network returns HTML bytes
6. spiral-html parses bytes → spiral-dom Document
7. spiral-css parses `<style>` + linked stylesheets → Style Rules
8. Style Resolution computes Computed Styles on DOM nodes
9. spiral-gyre (Gyre) computes the Layout Tree (box positions + sizes)
10. spiral-paint builds Display List from Layout Tree
11. spiral-render executes Display List via Vello → GPU texture
12. Texture displayed in wgpu surface → screen

---

## Data Flow: User Interaction

1. User clicks on screen (mouse event)
2. winit captures event → spiral-browser
3. Browser performs hit testing on layout tree
4. If link clicked: trigger navigation (step 2 above)
5. If input focused: send `BrowserToRenderer::InputEvent`
6. Renderer updates DOM (e.g., checkbox toggle)
7. Re-run layout (dirty rect optimization)
8. Re-render affected region

---

## Security Model

### Process Isolation
- Each tab runs in separate renderer process
- Renderer cannot access filesystem directly
- Renderer cannot make network requests (delegated to network process)
- Browser process validates all IPC messages

### Sandboxing (per-platform)
| Platform | Mechanism | Restrictions |
|----------|-----------|--------------|
| Linux | seccomp-bpf + Landlock | No `execve`, no filesystem, no network |
| macOS | Seatbelt | App sandbox profile, no `fork`, limited IPC |
| Windows | Restricted Token + Job Object | Low integrity, limited token groups |

### IPC Security
- Message type validation (reject unknown variants)
- Size limits on messages (prevent OOM)
- Rate limiting on renderer → browser messages
- No raw pointer passing between processes

---

## Rendering Pipeline Detail

### Display List Format
```
RenderOp::FillRect { rect: Rect, color: Color }
RenderOp::StrokeRect { rect: Rect, color: Color, width: f32 }
RenderOp::DrawText { glyphs: Vec<GlyphInstance>, font: FontId }
RenderOp::Clip { rect: Rect, ops: Vec<RenderOp> }
RenderOp::Transform { matrix: Mat3x3, ops: Vec<RenderOp> }
RenderOp::PushLayer { opacity: f32 }
RenderOp::PopLayer
```

### Vello Integration
1. spiral-render receives Display List
2. Converts `RenderOp`s to Vello scene primitives
3. Submits scene to Vello encoder
4. Vello executes on GPU compute pipeline
5. Output texture presented to wgpu surface
6. wgpu swap chain presents to window

---

## Text Rendering Pipeline

```
Text String
    │
    ▼
┌──────────────┐
│ Shaping      │  harfrust (HarfBuzz) → positioned glyphs
│ (spiral-render)│
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Rasterize    │  swash → glyph bitmaps
│ (spiral-render)│
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Cache        │  GPU texture atlas
│ (spiral-gpu) │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Draw         │  Vello draws cached glyphs
│ (spiral-render)│
└──────────────┘
```

---

## Configuration

```rust
// Default config
BrowserConfig {
    homepage: "about:blank",
    proxy: None,
    font_size: 16.0,
    accent_color: AccentColor::Indigo,
    dark_mode: true,  // follow system preference
    tab_position: TabPosition::Left,  // sidebar tabs
    auto_hide_chrome: true,
    sandbox_renderer: true,
}
```

Stored at:
- Linux: `~/.config/spiral/config.toml`
- macOS: `~/Library/Application Support/Spiral/config.toml`
- Windows: `%APPDATA%\Spiral\config.toml`
