# Phase 1 — Granular Task Breakdown

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
| 1.6 | Set up CI/CD (GitHub Actions, Linux/macOS/Windows matrix) | root | Green CI on all three platforms | [ ] |
| 1.7 | Pin workspace dependency versions | root | `cargo check --workspace` clean | [ ] |

### Exit gate for Month 1
`cargo test --workspace` passes on CI. All core types compile and are documented.

---

## Month 2 — IPC Transport Layer

| # | Task | Crate | Acceptance Test | Status |
|---|------|-------|-----------------|--------|
| 2.1 | Implement Unix domain socket transport (Linux/macOS) | spiral-ipc | Echo test: send message, receive same message | [ ] |
| 2.2 | Implement named pipe transport (Windows) | spiral-ipc | Echo test on Windows CI | [ ] |
| 2.3 | Implement length-prefixed bincode message framing | spiral-ipc | Round-trip test with variable-length payloads | [ ] |
| 2.4 | Implement `IpcTransport` trait abstraction | spiral-ipc | Mock transport passes the same interface tests | [ ] |
| 2.5 | Unit tests for IPC layer | spiral-ipc | `cargo test -p spiral-ipc` passes | [ ] |
| 2.6 | Fuzz test: malformed IPC frames | spiral-ipc | Fuzzer runs 60s without panic | [ ] |
| 2.7 | Integration: spiral-core ↔ spiral-ipc types | spiral-ipc + spiral-core | `BrowserToRenderer` and `RendererToBrowser` round-trip through transport | [ ] |

### Exit gate for Month 2
`cargo test -p spiral-ipc` passes on all three platforms. Fuzz harness exists
and runs clean.

---

## Month 3 — Browser & Renderer Shells

| # | Task | Crate | Acceptance Test | Status |
|---|------|-------|-----------------|--------|
| 3.1 | Browser process main entry | spiral-browser | `cargo run -p spiral-browser -- --help` prints usage | [ ] |
| 3.2 | Process spawning for renderer | spiral-browser + spiral-ipc | Spawn renderer subprocess; receive IPC handshake | [ ] |
| 3.3 | Basic tab management (open/close/list) | spiral-browser | Unit test: open tab, close tab, list tabs | [ ] |
| 3.4 | Renderer process main loop | spiral-render | Receive `Navigate` message via IPC, log it | [ ] |
| 3.5 | "Hello World" display list | spiral-render | Hard-coded display list: white background, "Hello World" text | [ ] |
| 3.6 | End-to-end integration test | root (tests/) | Launch browser process → receive `Navigate` → renderer responds with display list | [ ] |
| 3.7 | Stub window creation (winit) | spiral-browser or spiral-render | winit window opens and shows blank white frame | [ ] |
| 3.8 | Stub render output (Vello/WGPU) | spiral-render + spiral-gpu | GPU device initialised; white frame rendered to window | [ ] |

### Exit gate for Month 3
`cargo test --workspace` passes on all platforms. Running
`cargo run -p spiral-browser` opens a window showing "Hello World" on a white
background. CI is green on Linux (at minimum).

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
