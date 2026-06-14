# Active Context

**Last updated:** 2026-06-14
**Current phase:** Phase 1 — Foundation (Months 1–3)
**Current sprint:** Sprint 3 — IPC transport layer (Month 2 complete)

---

## Sprint Goal

Build a transport-agnostic IPC layer in `spiral-ipc`: `IpcTransport` trait,
Unix domain sockets, Windows named pipes, length-prefixed bincode framing,
mock transport for testing, and fuzz-smoke validation.

## In Progress

- [x] **2.1** `unix::UnixListener` + `unix::UnixTransport` (Linux/macOS)
- [x] **2.2** `pipe::PipeListener` + `pipe::PipeTransport` (Windows, `#[cfg(windows)]`)
- [x] **2.3** `encode_message` / `decode_message` — u32-LE header + bincode payload
- [x] **2.4** `IpcTransport` trait with `MockTransport::pair()` for testing
- [x] **2.5** 16 tests in `spiral-ipc` — framing, mock, unix echo, integration
- [x] **2.6** Fuzz smoke: 11 malformed patterns + 256 single-byte header permutations
- [x] **2.7** Integration: full browser↔renderer message flow through trait
- [ ] Commit Sprint 3 changes

## Blocked

None.

## Do Not Touch

- `Cargo.lock` is gitignored; never edit manually.
- `Cargo.toml` workspace members are stable; do not add or remove crates
  without an architectural discussion first.
- IPC protocol types in `spiral-core` (`BrowserToRenderer`,
  `RendererToBrowser`) are drafts; do not refactor until Month 3+.
- `spiral-browser/src/main.rs` — `#[tokio::main]` entry point is unimplemented
  shell; do not change until Month 3 when IPC + browser process wiring starts.

## Key Decisions (frozen for this sprint)

| Decision | Value | Rationale |
|----------|-------|-----------|
| Framing format | u32-LE length header + bincode payload | Simple, fast, zero-copy deserialise |
| Max frame size | 64 MiB | Prevents OOM from malicious lengths |
| Transport trait | `IpcTransport` with `Pin<Box<dyn Future>>` | Object-safe, works with `async fn` |
| Mock transport | tokio MPSC channels | No real OS sockets needed for tests |
| Close semantics | Best-effort, no error on already-closed | Avoids noisy errors during teardown |

## Next Sprint

Sprint 4 — Month 2 remaining + Month 3 start. Check `docs/phase1-tasks.md`
for remaining tasks. Month 3 covers browser process wiring (task 3.x):
`spiral-browser` main loop, IPC server startup, renderer process spawn,
basic "Hello World" rendering pipeline.
