# Spiral Browser — Development Roadmap

## Timeline

| Phase | Duration | Milestone |
|-------|----------|-----------|
| 1 | Months 1-3 | Core IPC, basic shell, renders "Hello World" |
| 2 | Months 4-9 | CSS box model, flexbox, text rendering, basic page load |
| 3 | Months 10-18 | Full CSS layout, JS engine, networking, form submission |
| 4 | Months 19-30 | Zen UI, GPU rendering, security sandboxing |
| 5 | Months 31-36 | WPT compliance, performance optimization, release |

---

## Phase 1: Foundation (Months 1-3)

**Goal:** Cargo workspace, IPC shell, renders "Hello World"

### Month 1
- [ ] Create Cargo workspace with all 17 crate stubs
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

**Goal:** CSS box model, flexbox, text rendering, basic page load

### Month 4
- [ ] Integrate html5ever into spiral-html
- [ ] Produce spiral-dom Document from HTML bytes
- [ ] Handle `<!DOCTYPE html>`, encoding detection
- [ ] Test with malformed HTML

### Month 5
- [ ] Implement spiral-dom tree structure
- [ ] `Node`, `Element`, `Text`, `Document` types
- [ ] Tree manipulation methods
- [ ] Attribute access, parent/child relationships

### Month 6
- [ ] Integrate cssparser + selectors into spiral-css
- [ ] Parse stylesheets, media queries, selectors
- [ ] Implement cascade order (origin, specificity)
- [ ] Resolve `!important`, inheritance

### Month 7
- [ ] Implement box model in spiral-layout
- [ ] Margin, border, padding, content area
- [ ] Block layout: vertical stacking, margin collapse
- [ ] BFC/IFC (Block/Inline Formatting Context)

### Month 8
- [ ] Integrate Taffy for flexbox
- [ ] Main axis, cross axis, wrap, gap
- [ ] `flex-direction`, `justify-content`, `align-items`
- [ ] Test with common flex patterns

### Month 9
- [ ] Text shaping via harfrust (HarfBuzz)
- [ ] Text rendering via swash
- [ ] Text layout via cosmic-text
- [ ] Basic text display in renderer

**Exit Criteria:**
- Renders a basic HTML page with text
- Flexbox layout works for simple layouts
- Text is shaped and rendered correctly
- 60fps for static content

---

## Phase 3: Full Engine (Months 10-18)

**Goal:** Full CSS layout, JS engine, networking, form submission

### Month 10-11
- [ ] Integrate Taffy for CSS Grid
- [ ] Template columns/rows
- [ ] Grid area placement
- [ ] `grid-template-areas`, `grid-gap`

### Month 12-13
- [ ] Integrate Boa JS engine
- [ ] `console.log`, `setTimeout`
- [ ] Basic DOM manipulation from JS
- [ ] Event listeners

### Month 14-15
- [ ] HTTP client via hyper
- [ ] DNS resolution via hickory-dns
- [ ] Connection pooling
- [ ] Redirect following

### Month 16
- [ ] TLS via rustls
- [ ] Certificate verification
- [ ] HTTPS support

### Month 17
- [ ] Cookie jar implementation
- [ ] Form submission (GET/POST)
- [ ] `FormData` support

### Month 18
- [ ] Image decoding pipeline
- [ ] PNG, JPEG, WebP, AVIF support
- [ ] Lazy loading, progressive loading

**Exit Criteria:**
- Can navigate to real websites
- JavaScript executes (basic)
- Forms submit correctly
- Images load and display

---

## Phase 4: UI & Polish (Months 19-30)

**Goal:** Zen UI, GPU rendering, security sandboxing

### Month 19-20
- [ ] Zen-style theme engine
- [ ] Design tokens (`--color-bg-primary`, `--color-accent`, etc.)
- [ ] Light/dark mode
- [ ] System preference detection

### Month 21-22
- [ ] Sidebar tabs UI
- [ ] Tab creation, switching, closing
- [ ] Tab dragging/reordering
- [ ] Tab context menu

### Month 23-24
- [ ] Floating URL bar
- [ ] Autocomplete suggestions
- [ ] Navigation buttons (back/forward/reload/home)
- [ ] Settings panel

### Month 25-26
- [ ] GPU rendering pipeline via Vello
- [ ] Display list → Vello scene → GPU texture
- [ ] Swap chain presentation
- [ ] Dirty rect optimization

### Month 27-28
- [ ] Platform sandboxing
- [ ] Linux: seccomp-bpf + Landlock
- [ ] macOS: Seatbelt profiles
- [ ] Windows: Restricted Token + Job Object

### Month 29-30
- [ ] DevTools basics
- [ ] Element inspector
- [ ] Console output
- [ ] Network panel

**Exit Criteria:**
- Zen-inspired UI is functional
- Sidebar tabs work
- URL bar navigates
- GPU rendering is smooth
- Sandbox is active on all platforms

---

## Phase 5: Production (Months 31-36)

**Goal:** WPT compliance, performance, release

### Month 31-32
- [ ] WPT test integration
- [ ] Run WPT layout tests
- [ ] Target: 50%+ pass rate
- [ ] Track regressions

### Month 33
- [ ] Performance benchmarks
- [ ] Layout: <1ms for simple pages
- [ ] Render: <16.67ms (60fps)
- [ ] IPC: <1ms round-trip

### Month 34
- [ ] Cross-platform packaging
- [ ] Linux: AppImage
- [ ] macOS: .app bundle
- [ ] Windows: installer

### Month 35
- [ ] Security audit
- [ ] Fuzzing
- [ ] Penetration testing
- [ ] Fix findings

### Month 36
- [ ] v0.1.0 release
- [ ] Release notes
- [ ] Documentation
- [ ] Blog post

**Exit Criteria:**
- v0.1.0 released on all platforms
- 50%+ WPT pass rate
- 60fps rendering
- No known security vulnerabilities

---

## Future (Post v0.1.0)

- [ ] WebAssembly support
- [ ] Service Workers
- [ ] IndexedDB
- [ ] Web Notifications
- [ ] WebGL/WebGPU
- [ ] WebRTC
- [ ] Extensions API
- [ ] Tab grouping
- [ ] Session restore
- [ ] Import from Chrome/Firefox
