# Spiral Browser — Architecture

> **The canonical system design.** This file is the source of truth for the architecture.
> `docs/system_architecture.md` is the *delta file* for in-flight changes that haven't been folded in yet.
> If you are looking for project vision: [`README.md`](README.md).
> If you are looking for the implementation plan and crate structure: [`PLAN.md`](PLAN.md).
> If you are looking for the phase index: [`ROADMAP.md`](ROADMAP.md).

---

## 1. Overview

Spiral is a multi-process, multi-threaded web browser. The browser process owns the tab manager, IPC router, UI chrome, and configuration. Each tab runs in a separate renderer process. Networking and GPU run in dedicated processes.

> Spiral is built with LLM assistance under human direction. The
> methodology is documented at [`docs/methodology.md`](docs/methodology.md)
> and is part of the project.

The render path is **100% safe Rust** end-to-end. The only `unsafe` in the tree is in audited dependencies and in FFI shims with narrow surface area (Vello's wgpu integration, the image decoder's codec bindings, the OS sandbox syscalls).

Three in-house engines carry the Spiral brand:

- **Gyre** (`spiral-gyre`) — layout
- **Vortex** (`spiral-vortex`) — JavaScript
- **Fmt** (`spiral-fmt`) — HTML5 + CSS parsers

Two more are in design and partial shipping:

- **Filter** (`spiral-filter`) — compile-time HTML/CSS policy
- **Context** (`spiral-context`) — capability-typed page context

Everything else is plumbing. The brand belongs to the engine, not the wire. See [`docs/glossary.md`](docs/glossary.md) for the canonical mapping and [`PLAN.md`](PLAN.md) § 3 for per-engine design.

---

## 2. Process model

```
┌──────────────────────────────────────────────────────────┐
│                    Browser Process                        │
│  ┌─────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐ │
│  │ Tab Mgr │ │ IPC Rtr  │ │ UI Chrome│ │ Config Mgr   │ │
│  └─────────┘ └──────────┘ └──────────┘ └──────────────┘ │
│  (vello GPU surface, native window)                      │
├──────────────────────────────────────────────────────────┤
│                     IPC Layer                            │
│  Unix Domain Sockets (Linux/macOS) · Named Pipes (Win)   │
│  bincode length-prefixed framing · tokio async runtime   │
├────────┬────────┬────────────┬──────────────────────────┤
│Render-1│Render-2│  Network   │         GPU              │
│ (Tab1) │ (Tab2) │  Process   │       Process            │
│  Fmt   │  Fmt   │  HTTP/1.1  │     wgpu Device/Queue    │
│  Gyre  │  Gyre  │  HTTP/2    │     Vello Renderer       │
│  Vortex│  Vortex│  DNS/TLS   │     Texture Management   │
│  Paint │  Paint │  Cookies   │     Swap Chain           │
│  Filter│  Filter│            │                          │
│ Context│ Context│            │                          │
├────────┴────────┴────────────┴──────────────────────────┤
│                Sandbox Layer                              │
│  Linux: Landlock + seccomp-bpf                           │
│  macOS: Seatbelt profiles                                │
│  Windows: Restricted Tokens + Job Objects                │
└──────────────────────────────────────────────────────────┘
```

### Browser process

The browser process is the trust root. It owns:

- **Tab Manager.** Tracks the set of open tabs, their state, and their renderer process. Allocates `TabId` values (a `u64` in `spiral-core`).
- **IPC Router.** Receives `IPCMessage` frames from renderer / network / GPU processes and routes them by message type. The router is a tokio task per socket.
- **UI Chrome.** The browser chrome itself — vertical sidebar tabs, floating URL bar, address-bar suggestions, settings UI. Renders via Vello in the browser process.
- **Configuration Manager.** Loads and persists `BrowserConfig` (user agent, default search, theme, permissions).

The browser process is the only process that holds the user's profile directory on disk. It brokers all disk and network I/O on behalf of the renderers.

### Renderer process

One renderer per tab. The renderer is **sandboxed**:

- **Linux:** Landlock filesystem restrictions + seccomp-bpf syscall filter. The renderer can read its own scratch dir and pipe to the IPC layer; nothing else.
- **macOS:** Seatbelt profile applied at spawn. No filesystem write outside the renderer scratch dir, no network, no other-process exec.
- **Windows:** Restricted token + Job Object with the `JOB_OBJECT_LIMIT_BREAKAWAY_NONE` flag.

The renderer runs the full render pipeline (see § 4). It has no direct access to disk or network; all I/O is mediated by the browser process via `IPCMessage`. The browser process enforces the same-origin policy and the capability types on the renderer's behalf.

### Network process

Owns the HTTP client (hyper + hickory-dns), the TLS stack (rustls), the cookie store, and the HTTP cache. One network process serves all renderers via IPC. Cookies are scoped to origin and sent to renderers as part of `IPCMessage::Navigation` responses; the renderer does not see the cookie jar.

### GPU process

Owns the wgpu `Device` and `Queue`, the Vello renderer, and the swap chain. Each renderer submits display lists to the GPU process via IPC; the GPU process owns all GPU memory and surfaces. This isolates GPU driver bugs from the renderer and centralises texture management.

A future Phase 7+ change may merge the GPU process into the browser process for performance; the IPC layer is already in place to support either topology.

---

## 3. The render pipeline

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

Each stage runs in the renderer process. Each stage is a pure function of the previous stage's output plus a `Context`. The renderer's job is to keep the pipeline running for each frame the user sees.

### 3.1 Fmt — HTML tokeniser and tree builder (`spiral-fmt`)

The HTML parser implements the WHATWG HTML Living Standard. The tree builder has 8 insertion modes (Initial, BeforeHtml, BeforeHead, InHead, AfterHead, InBody, AfterBody, AfterAfterBody) plus the algorithm pack (adoption agency, active formatting elements, foster parenting, fragment parsing). Output is a `spiral-dom::Document`.

The CSS parser implements the CSS Syntax Level 3 module. It has 8 sub-modules: tokeniser, parser, selectors, specificity, values, at-rules, declarations, attribute matchers. Cascade is user-agent < user < author < `!important`. Specificity is inline > ID > class > element.

`spiral-css` is a deprecated shim that forwards to `spiral_fmt::css::*` and provides a `CssParser` adapter. New code depends on `spiral-fmt` directly. See `docs/decisions/0001-css-parser-spiral-fmt.md`.

### 3.2 Vortex — JavaScript engine (`spiral-vortex`)

Vortex is a from-scratch JavaScript engine. The pipeline:

```
JS source bytes
    → Lexer       → Tokens
    → Parser      → AST        (Pratt precedence climbing)
    → Tree-walking interpreter (Phase 1)
        OR
    → Bytecode compiler (Phase 2)
        → Bytecode VM
        OR
    → Cranelift JIT (Phase 3+)
        → Native code
```

Phase 1 (in flight, packets 1.6.x) is the tree-walking interpreter: lex → parse → AST → walk. First slice covers console, math, object, array, and mark-sweep GC. Phase 2 introduces a stack-based bytecode VM (~5–10× faster) and adds closures, prototypes, classes. Phase 3 introduces a Cranelift-based baseline JIT.

The `JSRuntime` trait abstraction enables future engine swapping via Cargo feature flag. `rusty_v8` is available behind a `v8` feature for CI compliance testing only — it is the "V8 oracle" for conformance, not the production engine. See `docs/decisions/0002-vortex-from-scratch.md`.

### 3.3 Gyre — layout engine (`spiral-gyre`)

Gyre implements the CSS Display + Box Model specs. The pipeline:

```
DOM + Computed Styles
    → Box tree construction                (DOM nodes → CSS boxes)
    → Block layout                         (Phase 1)
    → Flex layout                          (Phase 2, Packet 2.5.x)
    → Grid layout                          (Phase 2, Packet 2.6.x)
    → Writing modes + Bidi                 (Phase 4)
    → Layout tree                          (boxes with positions)
```

Phase 1 (in flight) covers box model + block layout. Flex and grid land in Phase 2. Writing modes and Bidi land in Phase 4.

Gyre does not depend on `taffy` or any external layout crate. The crate name reflects the spiral-gyre glyph in the brand mark: layout values flow inward toward a centre, like the arms of a spiral. See `docs/decisions/0003-gyre-rename.md`.

### 3.4 Paint and Render

`spiral-paint` walks the layout tree and produces a display list (a flat sequence of draw operations: fill rect, draw text, draw image, etc.). The display list is renderer-agnostic; it does not know about Vello or wgpu.

`spiral-render` consumes the display list and submits it to Vello + wgpu. Vello handles the GPU acceleration (vector paths, text shaping, image composition). wgpu handles the device/queue management and the surface. The output is a GPU texture that the browser process presents to the screen via the swap chain.

`spiral-paint` is the *what*; `spiral-render` is the *how*.

### 3.5 Image decoding

`spiral-imagedecoder` decodes PNG, JPEG, WebP, and AVIF images. The decoders are wrapped behind a `Decoder` trait so the paint layer does not depend on a specific codec crate. AVIF uses the `dav1d` C library through a narrow FFI shim; the FFI is the only `unsafe` in the path.

The brand integration: the spiral-browser startup logo (`resources/icons/logo.png`) is loaded through `spiral-imagedecoder`. See `e762d09 feat(browser,imagedecoder,render): wire spiral-browser startup logo via spiral-imagedecoder (ADR 0006)`.

---

## 4. IPC protocol

Cross-process messaging is over **length-prefixed bincode frames**.

- **Frame:** `[u32 LE length][bincode payload]`
- **Payload type:** `IPCMessage` (13 variants defined in `spiral-core`).
- **Transport:** `tokio::net::UnixStream` (Linux/macOS) or `tokio::net::windows::NamedPipeServer` (Windows), behind a `Transport` trait.
- **Mock transport:** `MockTransport` for tests, in `spiral-ipc`.
- **Async runtime:** `tokio`.

The 13 `IPCMessage` variants cover the four cross-process flows: navigation requests/responses, display list submissions, GPU texture handles, and control messages (tab close, navigation abort, etc.). Per `AGENTS.md` § Common Pitfalls: changing the `IPCMessage` enum breaks bincode, so any payload change requires a versioned variant.

---

## 5. The capability-typed context (Filter + Context)

Filter and Context are Spiral's bet on **compile-time policy enforcement**. The idea: a web page should not be able to reach a network resource unless it can prove it has the capability. Not as a runtime check that can be bypassed — as a type-system property the compiler enforces.

### Context — `spiral-context`

```rust
pub struct Context<'brand, Mode> {
    mode: PhantomData<Mode>,
    capabilities: CapabilitySet<'brand>,
}
```

A page with `Mode = NoNetwork` cannot call `fetch()`. The compiler rejects it. The `Mode` type parameter is a phantom type that the type system uses to track the page's permission level.

Capabilities include: `Network`, `LocalStorage`, `Geolocation`, `Notifications`, `Camera`, `Microphone`, `Bluetooth`, `Usb`, `Midi`, `Clipboard`, `PersistentStorage`.

### Filter — `spiral-filter`

Filter decides at parse time and request time whether an operation is allowed. The runtime policy is in Packet 1.6.4 (shipped); the compile-time policy lands in Phase 5. Filter and Context integrate with Vortex and DOM in Phase 5+.

Filter is research-grade. The design is end-state, but the implementation is incremental. See `docs/decisions/0005-filter-runtime-design.md` and `docs/audit-sprint-m4.md` for the M4 audit methodology that gates novelty claims in this area.

---

## 6. The end-to-end data flow

A user types a URL in the floating URL bar:

1. **UI Chrome** sends `IPCMessage::Navigate(TabId, url)` to the Browser Process.
2. **Browser Process** checks the URL against the capability types and the same-origin policy. Sends `IPCMessage::Fetch(url)` to the Network Process.
3. **Network Process** performs the HTTP/1.1 or HTTP/2 request (Phase 1+), resolves DNS, performs TLS handshake, retrieves cookies, returns the response body to the Browser Process.
4. **Browser Process** routes the response to the target Renderer via `IPCMessage::NavigationResponse`.
5. **Renderer** parses the HTML with **Fmt** (HTML tokeniser → tree builder). Output: a `spiral-dom::Document`.
6. **Renderer** parses any inline `<style>` blocks and external stylesheets with **Fmt** (CSS parser). Output: a `Stylesheet`.
7. **Renderer** runs style resolution: matches selectors to elements, computes cascade order and specificity. Output: `Computed Styles` per element.
8. **Renderer** runs **Gyre** layout: box tree → block → flex → grid (Phase 2). Output: a `Layout Tree` with positions and sizes.
9. **Renderer** runs **Paint**: walks the layout tree, builds a display list. Output: a `Display List`.
10. **Renderer** sends the display list to the GPU Process via `IPCMessage::DisplayList`.
11. **GPU Process** consumes the display list with Vello + wgpu. Output: a GPU texture.
12. **GPU Process** hands the texture handle to the Browser Process via `IPCMessage::FrameReady`.
13. **Browser Process** presents the texture to the screen via the swap chain. The user sees the page.

If the page contains JavaScript, step 7 also runs **Vortex** to evaluate inline scripts and event handlers. **Filter** gates every step that touches the network, the disk, or a sensitive capability. **Context** types track the page's permission level end-to-end.

---

## 7. Security model

### Process isolation

- One renderer per tab.
- Renderer is sandboxed (`spiral-sandbox`):
  - **Linux:** Landlock + seccomp-bpf. Filesystem restrictions + syscall filter.
  - **macOS:** Seatbelt profile at spawn. No write outside the scratch dir, no network, no exec.
  - **Windows:** Restricted Token + Job Object.
- Renderer has no direct disk or network access. All I/O is mediated by the Browser Process via IPC.

### IPC framing

- u32-LE length prefix + bincode.
- Buffer-overflow checks at deserialisation (length must not exceed the read budget; bincode must terminate before the frame end).
- No remote code execution surface in the IPC layer itself.

### TLS

- rustls. No OpenSSL.
- TLS 1.2 minimum, TLS 1.3 preferred.
- Cipher suites: rustls defaults (X25519 + AES-256-GCM + ChaCha20-Poly1305).

### Capability-typed context

Phase 5+ integrates Filter and Context with Vortex and DOM. A page with `Mode = NoNetwork` cannot call `fetch()`. The compiler enforces it. See § 5.

### Vulnerability disclosure

Private channels only. See [`SECURITY.md`](SECURITY.md).

---

## 8. Configuration

`BrowserConfig` lives in `spiral-core` (`crates/spiral-core/src/config.rs`). It covers:

- **User agent.** The long-term aim is to be a "real" UA, not a Chromium clone. Phase 1 ships a Spiral-branded UA string.
- **Default search engine.** Configurable per profile.
- **Per-site permissions.** Geolocation, notifications, etc. — gated by Context in Phase 5+.
- **Theme.** Light, dark, system — `spiral-theme`.
- **Per-process resource limits.** Renderer memory cap, network timeouts, idle tab eviction.

`BrowserConfig` is loaded at browser-process startup, persisted to the profile dir, and re-loaded on changes (SIGHUP-style reload is a Phase 7+ feature).

---

## 9. Where the architecture lives

- **This file:** [`ARCHITECTURE.md`](ARCHITECTURE.md) (canonical)
- **Architecture delta:** [`docs/system_architecture.md`](docs/system_architecture.md) (in-flight changes)
- **Per-subsystem stubs:** [`docs/architecture/`](docs/architecture/) (one per brand)
- **ADRs:** [`docs/decisions/`](docs/decisions/)
- **Implementation plan:** [`PLAN.md`](PLAN.md)
- **Phase index:** [`ROADMAP.md`](ROADMAP.md)
- **Front door:** [`README.md`](README.md)
- **LLM cheatsheet:** [`CODEX.md`](CODEX.md)
- **Status SSOT:** [`docs/implementation_tracker.md`](docs/implementation_tracker.md)
