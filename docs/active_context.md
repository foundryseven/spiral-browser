# Active Context

**Last updated:** 2026-06-14
**Current phase:** Phase 1 — Foundation (Months 1–3)
**Current sprint:** Sprint 2 — CI/CD and lint hygiene

---

## Sprint Goal

Tasks 1.6–1.7 complete Month 1 of Phase 1. All core types compile, pass tests,
and are lint-clean across Linux, macOS, and Windows CI.

## In Progress

- [x] **1.6** CI/CD — GitHub Actions 4-job pipeline: `fmt`, `clippy`, `test`, `build`
  across 3 platforms. Triggers on `master` push/PR.
- [x] **1.7** Dependency versions pinned in workspace `Cargo.toml`. `cargo check --workspace`
  clean. All clippy warnings resolved.
- [ ] Commit Sprint 2 changes

## Blocked

None.

## Do Not Touch

- `Cargo.lock` is gitignored; never edit manually.
- `Cargo.toml` workspace members are stable; do not add or remove crates
  without an architectural discussion first.
- IPC protocol types in `spiral-core` (`BrowserToRenderer`,
  `RendererToBrowser`) are drafts; do not refactor until Month 2.
- `spiral-browser/src/main.rs` — `#[tokio::main]` entry point is unimplemented
  shell; do not change until IPC is ready (task 2.x).

## Key Decisions (frozen for this sprint)

| Decision | Value | Rationale |
|----------|-------|-----------|
| Default model | `ozore/custom` | User confirmed; single model for all agents |
| CI branch | `master` | Matches current repo default |
| Clippy policy | `-D warnings` (deny all) | Catch regressions early |
| CI platform matrix | ubuntu-latest, macos-latest, windows-latest | Phase 1 exit criterion |

## Next Sprint

Sprint 3 — Month 2 IPC transport layer. Tasks 2.1–2.4 in `spiral-ipc`:
Unix domain sockets, named pipes, length-prefixed framing, `IpcTransport` trait.
See `docs/phase1-tasks.md` §Month 2.
