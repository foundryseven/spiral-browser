# Progress Ledger

Append-only log of every meaningful change. Format:

```
## [ISO-date] [model] [crate/area] — change summary
  - Tests run: <pass/fail, count>
  - Status: <merged|in-progress|blocked>
```

---

## [2026-06-14] [custom] [repo] — Foundation scaffolding
- Added `README.md`, `LICENSE`, `CHANGELOG.md`, `SECURITY.md`, `CODEOWNERS`.
- Added `.github/ISSUE_TEMPLATE/{bug,feature_request,task}.yml`,
  `.github/PULL_REQUEST_TEMPLATE.md`.
- Added `docs/system_architecture.md`, `docs/active_context.md`,
  `docs/progress_ledger.md` (this file), `docs/phase1-tasks.md`.
- Added `tests/.gitkeep`, `benches/.gitkeep`.
- Updated `AGENTS.md` with active phase, model routing, SSOT references.
- Updated `opencode.jsonc` default model to `ozore/custom`.
- Corrected `CODEX.md` crate count from 17 to 18.
- Tests run: `cargo check --workspace` baseline captured in `docs/baseline-warnings.md`.
- Status: merged (commit `5a432f1`).

## [2026-06-14] [custom] [spiral-core] — Sprint 1: core type test coverage (tasks 1.2–1.5)
- **Task 1.2 — BrowserConfig:** bincode round-trip, default field assertions, clone equality.
- **Task 1.3 — TabId:** equality, hash dedup, `Display` impl added + verified. RenderNodeId hash test.
- **Task 1.4 — IPCMessage:** round-trip tests for all 13 variants (Navigate, UpdateDOM 6 ops,
  Resize, 3 InputEvent variants, Reload, Stop, DOMLoaded, LoadProgress, NavigateComplete,
  RequestNavigate, ConsoleMessage). Added corrupt-payload error test.
- **Task 1.5 — Error:** `From<io::Error>` conversion, `Display` message uniqueness across 9 variants,
  `Result` propagation via `?` operator.
- Added `TabId` `Display` impl (missing, blocked test).
- Tests run: `cargo test -p spiral-core` — 18 passed, 0 failed.
- Status: merged (commit `d78640e`).

## [2026-06-14] [custom] [repo/ci] — Sprint 2: CI/CD and lint hygiene (tasks 1.6–1.7)
- **Task 1.6 — CI/CD:** rewrote `.github/workflows/ci.yml` with separate `fmt`, `clippy`, `test`,
  `build` jobs across `{ubuntu,macos,windows}-latest`. Removed `CARGO_INCREMENTAL` (conflicts with
  CI caching). Switched triggers to `master` branch. Added `-D warnings` to clippy.
- **Task 1.7 — lint baseline:** fixed clippy warnings in `spiral-css` (`manual_strip` → `strip_prefix`),
  `spiral-dom` (`vec_init_then_push`, `unused_mut`), `spiral-ipc` (`unused_import`), `spiral-ui`
  (`unused_variable`). Fixed `cargo fmt` across workspace.
- **Bugfix:** `spiral-css::test_parse_selector` — assertion expected 4 parts for 3-part selector, fixed to 3.
- **Bugfix:** `spiral-layout::test_layout_empty_dom` — Document branch missing `content.width` assignment.
- Tests run: `cargo test --workspace` — 73 passed across 35 test targets, 0 failed.
- Tests run: `cargo clippy --workspace --all-targets` — 0 warnings.
- Tests run: `cargo fmt --all -- --check` — clean.
- Status: merged (commit `7029077`).

## [2026-06-14] [custom] [spiral-ipc] — Sprint 3: IPC transport layer (tasks 2.1–2.7)
- **Task 2.1 — Unix transport:** `unix::UnixListener` + `unix::UnixTransport` implementing
  `IpcTransport` trait. Accept/connect/send/recv/close all async. `read_exact`-based framing
  with 64 MiB size guard. Socket cleanup on drop. Echo test passes.
- **Task 2.2 — Windows transport:** `pipe::PipeListener` + `pipe::PipeTransport` behind
  `#[cfg(windows)]` — same `IpcTransport` interface, tokio named pipes. Uncompilable on
  macOS/Linux (intentional platform guard).
- **Task 2.3 — Framing:** public `encode_message` / `decode_message` functions. u32-LE
  length header + bincode payload. Tested: round-trip, large 100k payload, consumed-bytes
  match, truncated header, incomplete payload, zero-length payload.
- **Task 2.4 — IpcTransport trait:** `async fn send(&mut self, msg)`, `async fn recv(&mut self)`,
  `async fn close(&mut self)`, all returning `Result`. `MockTransport` (MPSC-backed) with
  `pair()` constructor passes all interface tests.
- **Task 2.5 — Unit tests:** 16 tests total — 7 framing tests, 5 mock transport tests
  (echo, bidirectional, all-variants, 50-message ordering, channel-close detection),
  1 Unix socket echo, 1 integration (browser↔renderer message flow), 2 fuzz smoke tests.
- **Task 2.6 — Fuzz smoke:** deterministic malformed-input corpus (11 patterns) + all 256
  single-byte header permutations. `decode_message` never panics, always returns `Err`.
- **Task 2.7 — Integration:** `core_types_through_mock_transport` exercises a full browser→renderer
  navigate→DOMLoaded→resize→progress→complete flow through the trait interface.
- Tests run: `cargo test -p spiral-ipc` — 16 passed, 0 failed.
- Tests run: `cargo test --workspace` — 86 passed, 0 failed.
- Tests run: `cargo clippy --workspace --all-targets` — 0 warnings.
- Tests run: `cargo fmt --all -- --check` — clean.
- Status: merged (commit `5a0d0ee`).

## [2026-06-14] [custom] [spiral-browser, spiral-render, spiral-core] — Sprint 4: Browser shell + software renderer + hello-world PNG

- **Protocol change — IPCMessage:** added `Hello(HelloMessage)` handshake variant with
  `tab_id`, `protocol_version`, `viewport_width`, `viewport_height`. `HelloMessage::PROTOCOL_VERSION`
  constant set to 1.
- **Protocol change — BrowserToRenderer:** every variant now carries `tab_id: TabId`. Added
  `Log { level, message }` and `ScreenshotAck { request_id }` variants. `Reload` and `Stop`
  changed from tuple variants to struct variants with `tab_id`.
- **Protocol change — RendererToBrowser:** every variant now carries `tab_id: TabId`. Added
  `RendererReady { tab_id }`, `Input { tab_id, event }`, `Screenshot { tab_id, request_id }`.
  `DOMLoaded` gained `url: String`; `NavigateComplete` gained `title: String`.
- **Task 3.3 — TabRegistry + TabState:** tab model with `id`, `url`, `title`, `loading`,
  `progress`, `loaded_at`, viewport dimensions. `TabRegistry` supports `open()`, `activate()`,
  `get()`/`get_mut()`, `active()`/`active_mut()`, `allocate_id()`. 8 unit tests.
- **Task 3.4 — BrowserTheme:** parses `ThemeTokens` hex strings into `spiral_paint::Color`.
  `from_engine()` / `from_tokens()` / `From<&ThemeEngine>`. Malformed hex falls back to black.
  3 unit tests.
- **Task 3.5 — SoftwareRenderer:** full display list rasteriser that walks nested `Clip`/`Transform`
  scopes depth-first. Supports `FillRect`, `StrokeRect`, `DrawText`, `Clip`, `Transform` (2D affine),
  `PushLayer`/`PopLayer` (alpha compositing). Exports `Rgba`, `Transform`. 8 unit tests.
- **Task 3.6 — Built-in 5×7 bitmap font:** ASCII 0x20–0x7E (95 glyphs), `glyph()`, `space_glyph()`,
  `text_width()`. 5 unit tests.
- **Task 3.7 — PNG output:** `encode_png()` encodes `SoftwareRenderer` framebuffer as RGBA8 PNG.
  `PngError` type. 1 unit test (validates PNG signature + IHDR chunk).
- **Task 3.8 — hello-world display list:** `build_hello_display_list()` produces 5 ops:
  background fill, centred "Hello, Spiral!" headline, accent underline, status-strip background,
  URL+title status text. 3 unit tests.
- **Task 3.9 — IPC event loop:** `process_message()` translates `Hello`/`RendererToBrowser` events
  into `TabRegistry` mutations and returns `ProcessOutcome::Reply(...)`. `run_event_loop()` drives
  an `IpcTransport` until channel close. 4 unit tests.
- **Task 3.10 — BrowserShell:** owns config + `ThemeEngine` + `TabRegistry`. `new()` opens homepage tab.
  `render_active_tab()` returns `(width, height, png_bytes)`. `render_active_tab_to()` writes to disk.
  `display_list()` builds the display list. `run()` drives the IPC loop. 6 unit tests (including
  async `run_drains_mock_transport` integration).
- **Task 3.11 — Binary:** `cargo run` initialises shell, renders hello-world PNG to
  `target/hello-world.png`, prints path and tab info.
- **Task 3.12 — Tests:** 143 total workspace tests, 0 failures.
- Tests run: `cargo test -p spiral-render` — 14 passed, 0 failed.
- Tests run: `cargo test -p spiral-browser` — 23 passed, 0 failed.
- Tests run: `cargo test --workspace` — 143 passed, 0 failed.
- Tests run: `cargo clippy --workspace` — 4 warnings (expected: too-many-arguments in rasteriser).
- Status: in-progress (not yet committed).
