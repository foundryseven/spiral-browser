# Active Context

**Last updated:** 2026-06-14
**Current phase:** Phase 1 — Foundation (Months 1–3)
**Current sprint:** Sprint 4 — Browser shell + software renderer + hello-world PNG (Month 3)

---

## Sprint Goal

Build `BrowserShell` in `spiral-browser`, `SoftwareRenderer` in `spiral-render`,
the hello-world display list builder, the IPC event loop for renderer messages,
and a headless `cargo run` that renders "Hello, Spiral!" to
`target/hello-world.png`. Phase 1 exit criteria met.

---

## In Progress

- [x] **3.1** Extend `IPCMessage` with `Hello(HelloMessage)` handshake variant
- [x] **3.2** Extend `BrowserToRenderer` / `RendererToBrowser` with `tab_id` on
      every variant; add `Log`, `ScreenshotAck`, `RendererReady`, `Screenshot`
- [x] **3.3** `TabRegistry` + `TabState` — tab model keyed by `TabId`
- [x] **3.4** `BrowserTheme` adapter — parses `ThemeTokens` hex strings into
      `spiral_paint::Color` for the renderer
- [x] **3.5** `build_hello_display_list()` — background, centred headline, accent
      underline, URL+title status strip
- [x] **3.6** `SoftwareRenderer` — nested-scope-aware display list rasteriser
      (FillRect, StrokeRect, DrawText, Clip, Transform, PushLayer/PopLayer)
- [x] **3.7** Built-in 5×7 bitmap font (`spiral-render::font`) covering ASCII
      0x20–0x7E, 14 unit tests
- [x] **3.8** `encode_png()` — RGBA8 framebuffer → PNG byte stream via `png` crate
- [x] **3.9** `event_loop::process_message()` — translates `RendererToBrowser`
      events into `TabRegistry` mutations and returns replies
- [x] **3.10** `BrowserShell` — owns config + theme + registry; `render_active_tab()`
      writes `target/hello-world.png`; `run()` drives an `IpcTransport` loop
- [x] **3.11** Binary: `cargo run` initialises shell, renders hello-world PNG,
      prints path
- [x] **3.12** Tests: 23 tests in `spiral-browser`, 14 in `spiral-render`; 143
      total workspace tests, 0 failures

---

## Completed

- Sprint 0: repo scaffolding, docs baseline
- Sprint 1: core types (`BrowserConfig`, `TabId`, `IPCMessage`, `Error`, tests)
- Sprint 2: CI matrix, lint hygiene
- Sprint 3: IPC transport layer (`IpcTransport`, Unix/Windows, framing, mock)
- Sprint 4: browser shell, software renderer, hello-world PNG

---

## Do Not Touch

- `spiral-gpu`, `spiral-paint`, `spiral-ui`, `spiral-theme` — Phase 2+
- `spiral-js` — Phase 3
- `spiral-network`, `spiral-net` — Phase 3
- `spiral-sandbox` — Phase 4

---

## Phase 1 Exit Criteria — Status

| Criterion | Status |
|-----------|--------|
| `cargo build --workspace` succeeds | ✅ |
| `cargo test --workspace` passes | ✅ (143 tests) |
| Browser renders "Hello World" | ✅ (`target/hello-world.png`) |

**Phase 1 is complete.** The next phase (Phase 2) adds CSS box model, text layout,
and a windowed GPU surface.

---

## Key Architecture Decisions This Sprint

1. **Software renderer tree walk** — `RenderOp::Clip` and `RenderOp::Transform`
   are scope-bearing (they contain `ops: Vec<RenderOp>`). The rasteriser walks
   this tree depth-first, accumulating clip rect and affine transform at each
   level. This avoids a separate flattening pass and matches how paint engines
   work natively.

2. **TabId on every protocol message** — all `BrowserToRenderer` and
   `RendererToBrowser` variants carry `tab_id: TabId`. This was a breaking
   change to the `spiral-core` protocol but is architecturally correct — a
   real browser must route messages to specific tabs.

3. **BrowserTheme hex adapter** — `spiral-theme` stores colours as hex strings;
   `spiral_paint::Color` uses `u8` fields. The bridge lives in
   `spiral-browser::theme`, keeping the two crates decoupled.
