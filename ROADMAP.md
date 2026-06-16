# Spiral Browser — Development Roadmap

## Timeline

| Phase | Duration | Milestone |
|-------|----------|-----------|
| 1 | Months 1-3 | Core IPC, basic shell, renders "Hello World" |
| 2 | Months 4-9 | Vendored parsers, Gyre block layout, Vortex lexer/parser/interpreter |
| 3 | Months 10-24 | Flex layout, text rendering, Vortex bytecode VM, basic DOM from JS |
| 4 | Months 25-42 | Grid layout, networking, HTTP/HTTPS, DOM manipulation, image decoding |
| 5 | Months 43-60 | Zen UI, GPU rendering, Vello optimisation, sandbox, Vortex baseline JIT |
| 6 | Months 61-84 | WPT compliance, performance tuning, cross-platform packaging, v0.1.0 |

---

## Phase 1: Foundation (Months 1-3)

**Goal:** Cargo workspace, IPC shell, renders "Hello World"

### Month 1
- [x] Create Cargo workspace with all 18 crate stubs
- [ ] Define core types in spiral-core (`BrowserConfig`, `TabId`, `IPCMessage`)
- [ ] Implement basic error types
- [ ] Set up CI/CD (GitHub Actions, Linux/macOS/Windows matrix)

### Month 2
- [ ] Implement IPC transport layer (spiral-ipc)
- [ ] Unix domain socket support (Linux/macOS)
- [ ] Named pipe support (Windows)
- [ ] Message framing (length-prefixed bincode)
- [ ] Unit tests for IPC

### Month 3
- [ ] Browser process shell (spiral-browser)
- [ ] Process spawning for renderer
- [ ] Basic tab management
- [ ] Renderer process shell (spiral-render)
- [ ] "Hello World" display list
- [ ] End-to-end test: launch → parse → render → display

**Exit Criteria:**
- `cargo build` succeeds on all platforms
- `cargo test` passes all tests
- Browser launches and displays "Hello World" in a window

---

## Phase 2: Core Engine (Months 4-9)

**Goal:** Vendored parsers, Gyre block layout, Vortex lexer/parser/interpreter

### Month 4 — Vendor Servo parsers
- [ ] Vendor `html5ever` into `spiral-fmt`; modernise deps (tendril→compact\_str, string\_cache→owned Atoms)
- [ ] Vendor `cssparser` + `selectors` into `spiral-fmt`; update Cargo manifests
- [ ] Unified facade: `spiral_fmt::parse_html()`, `spiral_fmt::parse_css()`
- [ ] `spiral_net::Resolver` trait wrapping hickory-dns (Track E wrapper)

### Month 5 — Test porting & wrappers
- [ ] Port html5ever HTML5 lib test subset into `spiral-fmt`; fix all build warnings
- [ ] Port cssparser + selectors unit tests; fuzz harness for both parsers (10k corpus, no panics)
- [ ] `spiral_net::TlsConnector` trait wrapping rustls (Track E wrapper)
- [ ] `spiral_imagedecoder::Decoder` enum dispatching per-format (Track E wrapper)

### Month 6 — Rewire to spiral-fmt + DOM
- [ ] Rewire `spiral-html` from `html5ever` → `spiral-fmt`
- [ ] Rewire `spiral-css` from `cssparser`+`selectors` → `spiral-fmt`
- [ ] Implement spiral-dom tree structure: `Node`, `Element`, `Text`, `Document`
- [ ] HTML→DOM pipeline: produce `Document` from bytes via spiral-fmt + spiral-dom
- [ ] `spiral_network::Client` trait wrapping hyper (Track E wrapper)

### Month 7 — Box model & block layout
- [ ] Box model: margin, border, padding, content area
- [ ] Block layout: vertical stacking, margin collapse
- [ ] BFC/IFC (Block/Inline Formatting Context)

### Month 8 — Floats, positioning & cascade
- [ ] Floats: left/right, clear, BFC containment
- [ ] Positioning: static, relative, absolute, fixed, sticky
- [ ] Cascade engine: origin order, specificity, `!important`, inheritance

### Month 9 — Text rendering
- [ ] Text shaping via harfrust (HarfBuzz)
- [ ] Text rendering via swash
- [ ] Text layout via cosmic-text
- [ ] Styled text in layout box → rendered to display list

### Month 10 — Flex layout & Vortex bytecode VM
- [ ] Custom flex layout: container model, main/cross axis, flex lines
- [ ] `flex-direction`, `flex-wrap`, `flex-flow`, `justify-content`, `align-items`
- [ ] Vortex bytecode compiler: AST → bytecode instructions
- [ ] Vortex stack VM: execute bytecode (replaces tree-walking interpreter)

### Month 11 — Flex continued & Vortex closures
- [ ] `flex-grow`, `flex-shrink`, `flex-basis`, `min-width`/`max-width` interaction, `order`
- [ ] WPT fixtures for block + flex layout
- [ ] Vortex closures, `this` binding, prototype chain
- [ ] `console.log`/`info`/`warn`/`error` → `RendererToBrowser::ConsoleMessage`

### Month 12 — Vortex DOM bindings & events
- [ ] Vortex ES2015 syntax: `let`/`const`, arrow functions, template literals, classes
- [ ] DOM bindings: `createElement`, `appendChild`, `insertBefore`, `setAttribute`, `getAttribute`
- [ ] `addEventListener`, `dispatchEvent`; event dispatch skeleton
- [ ] `setTimeout`, `setInterval`, `queueMicrotask`

**Exit Criteria:**
- `spiral-html` and `spiral-css` depend on `spiral-fmt` only; no Servo crates in `cargo tree`
- Block layout: margin collapse, floats, BFC/IFC, all positioning modes
- Flex layout: common patterns (centring, sidebar+content, wrap)
- Text is shaped and rendered correctly (basic Latin, CJK start)
- Vortex is the default JS engine — from-scratch Rust, interpreter-ready; `boa_engine` is gone from `Cargo.toml`
- V8 oracle available behind `v8` feature flag for CI compliance testing
- DOM manipulation from JS triggers re-layout
- WPT block pass rate ≥ 40%; flex pass rate ≥ 30%

---

## Phase 3: Flex, Text & Vortex Bytecode (Months 10-24)

**Goal:** Flex layout, text rendering, Vortex bytecode VM with ES2015+ syntax, basic DOM from JS

### Month 10 — Flex layout & Vortex bytecode compiler
- [ ] Custom flex layout: container model, main/cross axis, flex lines
- [ ] `flex-direction`, `flex-wrap`, `flex-flow`, `justify-content`, `align-items`
- [ ] Vortex bytecode compiler: AST → bytecode instructions
- [ ] Vortex stack VM: execute bytecode (replaces tree-walking interpreter)

### Month 11 — Flex continued & Vortex closures
- [ ] `flex-grow`, `flex-shrink`, `flex-basis`, `min-width`/`max-width` interaction, `order`
- [ ] WPT fixtures for block + flex layout
- [ ] Vortex closures, `this` binding, prototype chain
- [ ] `console.log`/`info`/`warn`/`error` → `RendererToBrowser::ConsoleMessage`

### Month 12 — Vortex ES2015 syntax
- [ ] `let`/`const` (block scoping)
- [ ] Arrow functions, template literals, destructuring
- [ ] Classes (extends, super, static methods)
- [ ] `for...of`, iterators, spread operator

### Month 13-14 — Grid layout (custom)
- [ ] Grid container: explicit/implicit tracks, `grid-template-columns/rows`
- [ ] Grid template areas, named lines, line-based placement, span
- [ ] `grid-auto-flow: row/column/dense`, `grid-gap`, `subgrid` (Level 2)

### Month 15-16 — Text rendering & Vortex Promises
- [ ] Text shaping via harfrust (HarfBuzz)
- [ ] Text rendering via swash
- [ ] Text layout via cosmic-text
- [ ] Styled text in layout box → rendered to display list
- [ ] Vortex Promises, `async`/`await`, generators

### Month 17-18 — Vortex builtins & DOM bindings
- [ ] Builtins: `Object`, `Array`, `String`, `Number`, `Boolean`, `Math`, `JSON`, `Date`, `RegExp`
- [ ] `Map`, `Set`, `WeakMap`, `WeakSet`, `Symbol`, `Error` types
- [ ] DOM bindings: `createElement`, `appendChild`, `insertBefore`, `setAttribute`, `getAttribute`
- [ ] `addEventListener`, `dispatchEvent`; event dispatch skeleton
- [ ] `setTimeout`, `setInterval`, `queueMicrotask`

### Month 19-20 — Vortex GC & DOM polish
- [ ] Mark-sweep GC with roots tracing from stack + globals
- [ ] DOM manipulation from JS: `removeChild`, `textContent`, `style`
- [ ] CSS matching on DOM mutation; incremental re-style
- [ ] Full event dispatch: `MouseEvent`, `KeyboardEvent`, `FocusEvent`

### Month 21-22 — Networking start
- [ ] HTTP client via hyper (through `spiral_network::Client`)
- [ ] DNS resolution via hickory-dns (through `spiral_net::Resolver`)
- [ ] TLS via rustls (through `spiral_net::TlsConnector`); certificate verification; HTTPS

### Month 23-24 — Images & integration
- [ ] Image decoding pipeline (through `spiral_imagedecoder::Decoder`)
- [ ] PNG, JPEG, WebP, AVIF support; lazy + progressive loading
- [ ] End-to-end: navigate → fetch HTML → parse → layout → render → Vortex run → event dispatch

### Competitive parity additions (from M4.5 research, 2026-06-16)
- [ ] Adoption agency algorithm (WHATWG HTML §12.2.6.1)
- [ ] Active formatting elements list (WHATWG HTML §12.2.6.1)
- [ ] Foster parenting (WHATWG HTML §12.2.6.1)
- [ ] Fragment parsing algorithm (WHATWG HTML §12.4)
- [ ] DOM collection types: `NodeList`, `HTMLCollection`, `DOMTokenList`, `Attr`, `NamedNodeMap`, `DocumentType`
- [ ] Global attributes IDL (`id`, `class`, `style`, `title`, `lang`, `dir`, `hidden`, `tabindex`, `contenteditable`, `inert`, `popover`)
- [ ] `data-*` custom data attributes (`dataset` IDL)
- [ ] `globalThis` (ECMA-262 §19.4.1)
- [ ] `structuredClone` (WHATWG HTML §8.2.7)
- [ ] `Proxy` + `Reflect` (ECMA-262 §10.5, §28.1)
- [ ] `URL` + `URLSearchParams` (WHATWG URL §4)
- [ ] Quirk mode classifier (WHATWG HTML §12.1)
- [ ] `<noscript>` element (WHATWG HTML §4.6.7)
- [ ] `<template>` content document-fragment construction (bump to active sprint)

**Exit Criteria:**
- Block + flex + grid layout: common patterns render correctly
- Text is shaped and rendered correctly (basic Latin, CJK start)
- Vortex runs ES2015+ code via bytecode VM; ~30% Test262 pass rate
- DOM manipulation from JS triggers re-layout
- Can navigate to real websites over HTTP/HTTPS
- Images load and display
- `taffy` was never in `Cargo.toml` — Gyre is custom from day one

### Competitive parity additions (from M4.5 research, 2026-06-16)
- [ ] HTTP/1.1 client (basic page fetching)
- [ ] SOP enforcement (origin checks)
- [ ] CORS (simple requests + preflight)
- [ ] Cookie jar (basic session management)
- [ ] WebAssembly (basic Module/Instance/Memory)
- [ ] Fetch API (basic `fetch()`)
- [ ] ARIA reflection (`role`, `aria-*`, `label`, `labelledby`)
- [ ] Keyboard navigation (tab order, focus management, `focus-visible`)

---

## Phase 4: UI & Vortex JIT (Months 25-42)

**Goal:** Zen UI, GPU rendering, Vello optimisation, security sandboxing, Vortex baseline JIT

### Month 25-27 — Theme engine & sidebar
- [ ] Zen-style theme engine; design tokens (`--color-bg-primary`, `--color-accent`, etc.)
- [ ] Light/dark mode; system preference detection
- [ ] Sidebar tabs UI: creation, switching, closing, dragging/reordering
- [ ] Tab context menu

### Month 28-30 — URL bar & navigation
- [ ] Floating URL bar
- [ ] Autocomplete suggestions
- [ ] Navigation buttons (back/forward/reload/home)
- [ ] Settings panel; cookie jar; form submission

### Month 31-33 — GPU rendering
- [ ] GPU rendering pipeline via Vello
- [ ] Display list → Vello scene → swap chain
- [ ] Fork Vello for tile-based picture caching
- [ ] Dirty-rect invalidation; only re-render changed tiles
- [ ] Scroll at 120fps on integrated graphics; cache overhead < 200 MB

### Month 34-36 — Vortex baseline JIT
- [ ] Cranelift-based baseline JIT for hot functions
- [ ] Type feedback, inline caches for property access
- [ ] Optimising tier for frequently-called functions
- [ ] ~60% Test262 pass rate target

### Month 37-39 — Platform sandboxing
- [ ] Linux: seccomp-bpf + Landlock
- [ ] macOS: Seatbelt profiles
- [ ] Windows: Restricted Token + Job Object

### Month 40-42 — DevTools & modules
- [ ] Element inspector; console output; network panel
- [ ] Vortex ES modules (`import`/`export`)
- [ ] `fetch()` API; `XMLHttpRequest` (legacy compat)

**Exit Criteria:**
- Zen-inspired UI: sidebar tabs, floating URL bar, accent colours
- GPU rendering smooth; scrolling at 120fps on integrated graphics
- Vortex baseline JIT operational; ~60% Test262 pass rate
- Sandbox active on all platforms
- DevTools shows DOM tree, console output, network requests

---

## Phase 5: Production (Months 43-60)

**Goal:** WPT compliance, performance tuning, Vortex optimising JIT, cross-platform packaging

### Month 43-48
- [ ] Vortex optimising JIT (type-specialised codegen, speculative optimisations)
- [ ] Concurrent/incremental GC
- [ ] WPT test integration; layout test pass target ≥ 50%
- [ ] Performance benchmarks: layout <1ms, render <16.67ms (60fps), IPC <1ms round-trip
- [ ] ~75% Test262 pass rate target

### Month 49-54
- [ ] WebAssembly support (Vortex wasm interpreter)
- [ ] Service Workers
- [ ] IndexedDB
- [ ] WebGL/WebGPU bindings

### Month 55-60
- [ ] Cross-platform packaging: Linux AppImage, macOS .app, Windows installer
- [ ] Security audit; fuzzing; penetration testing
- [ ] ~80% Test262 pass rate; 50%+ WPT pass rate
- [ ] v0.1.0 release

**Exit Criteria:**
- v0.1.0 released on all platforms
- 50%+ WPT pass rate; 80%+ Test262 pass rate
- 60fps rendering; no known security vulnerabilities
- Vortex is a production-quality JS engine (bytecode VM + baseline JIT + optimising JIT)

---

## Phase 6: Polish & Beyond (Months 61-84)

**Goal:** Full Test262 compliance, WebRTC, extensions, v1.0

- [ ] 90%+ Test262 pass rate
- [ ] WebRTC
- [ ] Extensions API
- [ ] Tab grouping; session restore
- [ ] Import from Chrome/Firefox
- [ ] Accessibility (screen reader support, ARIA)
- [ ] Internationalisation (ICU integration)
- [ ] v1.0 release

### Competitive parity additions (from M4.5 research, 2026-06-16)
- [ ] DevTools (Elements + Console + Network panels)
- [ ] Error pages (HTTP errors, cert errors, network errors)
- [ ] View-source
- [ ] Headless mode (for automation/testing)
- [ ] Installers (Linux .deb + .rpm, macOS .dmg, Windows .exe)
- [ ] Auto-update (differential, background, signed)
- [ ] Code signing (EV cert, Apple Developer ID)
- [ ] Default browser registration
- [ ] Crash reporting (opt-in)
- [ ] Enterprise policy (ExtensionSettings, SafeBrowsing, proxy)
- [ ] WebExtensions MV3 (tabs, storage, content_scripts, action, popup)
- [ ] Extension store / gallery
- [ ] Custom themes (background image, accent colour)
