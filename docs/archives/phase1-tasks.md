# Phase 1 — Granular Task Breakdown

> **⚠️ Archived 2026-06-16.** Pre-restructure vocabulary (Month / Task).
> Retained for traceability. See
> [`docs/implementation_tracker.md`](../implementation_tracker.md)
> for the current Group → Phase → Step → Packet checklist. Per
> [`docs/audits/2026-06-16-doc-drift.md`](../audits/2026-06-16-doc-drift.md)
> P0 #8, no per-task status is updated here.

**Phase goal:** Cargo workspace, IPC shell, browser launches and renders
"Hello World" in a window.

**Exit criteria:**
- `cargo build` succeeds on all three platforms.
- `cargo test` passes all tests.
- Browser launches and displays "Hello World".

Status legend: `[ ]` pending · `[~]` in progress · `[x]` done

---

## Month 1 — Workspace & Core Types

| # | Task | Crate | Acceptance Test | Status |
|---|------|-------|-----------------|--------|
| 1.1 | Create Cargo workspace with all 18 crate stubs | root | `cargo build --workspace` succeeds | [x] |
| 1.2 | Define `BrowserConfig` struct | spiral-core | Unit test serialises/deserialises via bincode | [x] |
| 1.3 | Define `TabId` newtype | spiral-core | Unit test equality, hash, display | [x] |
| 1.4 | Define `IPCMessage` enum | spiral-core | Round-trip bincode test for each variant | [x] |
| 1.5 | Define `Error` types via thiserror | spiral-core | Unit tests for each error variant | [x] |
| 1.6 | Set up CI/CD (GitHub Actions, Linux/macOS/Windows matrix) | root | Green CI on all three platforms | [x] |
| 1.7 | Pin workspace dependency versions | root | `cargo check --workspace` clean | [x] |

### Exit gate for Month 1
`cargo test --workspace` passes on CI. All core types compile and are documented.

---

## Month 2 — IPC Transport Layer

| # | Task | Crate | Acceptance Test | Status |
|---|------|-------|-----------------|--------|
| 2.1 | Implement Unix domain socket transport (Linux/macOS) | spiral-ipc | Echo test: send message, receive same message | [x] |
| 2.2 | Implement named pipe transport (Windows) | spiral-ipc | Echo test on Windows CI | [x] |
| 2.3 | Implement length-prefixed bincode message framing | spiral-ipc | Round-trip test with variable-length payloads | [x] |
| 2.4 | Implement `IpcTransport` trait abstraction | spiral-ipc | Mock transport passes the same interface tests | [x] |
| 2.5 | Unit tests for IPC layer | spiral-ipc | `cargo test -p spiral-ipc` passes | [x] |
| 2.6 | Fuzz test: malformed IPC frames | spiral-ipc | Fuzzer runs 60s without panic | [x] |
| 2.7 | Integration: spiral-core ↔ spiral-ipc types | spiral-ipc + spiral-core | `BrowserToRenderer` and `RendererToBrowser` round-trip through transport | [x] |

### Exit gate for Month 2
`cargo test -p spiral-ipc` passes on all three platforms. Fuzz harness exists
and runs clean.

---

## Month 3 — Browser & Renderer Shells

| # | Task | Crate | Acceptance Test | Status |
|---|------|-------|-----------------|--------|
| 3.1 | Extend `IPCMessage` with `Hello(HelloMessage)` handshake | spiral-core | Bincode round-trip for new variant | [x] |
| 3.2 | Extend `BrowserToRenderer`/`RendererToBrowser` with `tab_id`, add `Log`/`ScreenshotAck`/`RendererReady`/`Screenshot` | spiral-core | All existing tests updated, `cargo test -p spiral-core` passes | [x] |
| 3.3 | `TabRegistry` + `TabState` — tab model | spiral-browser | 8 unit tests (open, activate, progress clamped, viewport clamped) | [x] |
| 3.4 | `BrowserTheme` hex adapter | spiral-browser | 3 unit tests (bg, accent, malformed fallback) | [x] |
| 3.5 | `SoftwareRenderer` — display list rasteriser | spiral-render | 8 unit tests (fill, stroke, text, clip, transform, layers) | [x] |
| 3.6 | Built-in 5×7 bitmap font (ASCII 0x20–0x7E) | spiral-render | 5 unit tests (all glyphs present, unsupported returns None, text_width) | [x] |
| 3.7 | `encode_png()` — RGBA8 → PNG | spiral-render | 1 unit test (valid PNG signature + IHDR) | [x] |
| 3.8 | `build_hello_display_list()` — background, headline, accent, status strip | spiral-browser | 3 unit tests (ops count, viewport fill, status text) | [x] |
| 3.9 | `process_message()` + `run_event_loop()` IPC event handling | spiral-browser | 4 unit tests (hello handshake, bad version, navigate complete, request navigate) | [x] |
| 3.10 | `BrowserShell` — owns config + theme + registry, `render_active_tab()` | spiral-browser | 6 unit tests (homepage tab, open tab, display list, PNG, file write, async IPC drain) | [x] |
| 3.11 | Binary: `cargo run` renders hello-world PNG | spiral-browser | `target/hello-world.png` written on run | [x] |
| 3.12 | Full workspace test suite | root | 143 tests, 0 failures | [x] |

### Exit gate for Month 3
`cargo test --workspace` passes (143 tests, 0 failures). `cargo run -p spiral-browser`
writes `target/hello-world.png` — a 1024×768 RGBA PNG with "Hello, Spiral!" centred
on the browser's theme background. No windowing (Phase 1 is headless; windowed GPU
rendering is Phase 4).

---

## Dependency Graph

```
Month 1
  1.1  1.2  1.3  1.4  1.5  1.6  1.7  (all parallel)

Month 2
  2.1 ─┐
  2.2 ─┤
  2.3 ─┤
  2.4 ─┼─ 2.7
  2.5 ─┤
  2.6 ─┘

Month 3
  3.1 ─┐
  3.2 ─┼─ 3.6
  3.3 ─┤
  3.4 ─┼─ 3.6
  3.5 ─┤
  3.7 ─┘
  3.8 ─┘
```

---

## Notes

- Tasks 1.1 through 1.7 can all be parallelised across multiple agents.
- Month 2 tasks 2.1 and 2.2 are platform-specific; only one needs to run
  on each CI matrix leg.
- Month 3 task 3.6 is the true integration milestone; it depends on all
  other Month 3 tasks.
- Every task row should have a corresponding `docs/progress_ledger.md` entry
  when completed.
