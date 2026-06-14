# Active Context

**Last updated:** 2026-06-14
**Current phase:** Phase 1 — Foundation (Months 1–3)
**Current sprint:** Sprint 1 — Core types and shared types definitions

---

## Sprint Goal

Complete Phase 1 Month 1 tasks 1.2–1.5 in `spiral-core`: establish a tested
type foundation (`BrowserConfig`, `TabId`, `IPCMessage`, `Error`) that every
other crate depends on.

## In Progress

- [x] **1.2** `BrowserConfig` — bincode round-trip, default, clone equality
- [x] **1.3** `TabId` + `RenderNodeId` — equality, hash, `Display`
- [x] **1.4** `IPCMessage` — all 13 enum variants round-trip tested
- [x] **1.5** `Error` — `From<io::Error>`, `Display` per variant, `?` propagation
- [ ] Commit Sprint 1 changes

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
| Active context write policy | Agents update autonomously per task loop | Implementer appends; reviewer flags stale state |
| Progress ledger format | Markdown bullets with ISO date | Human- and agent-readable |
| `phase1-tasks.md` | Separate file, not inline here | Keeps active_context small |

## Next Sprint

Sprint 2 — Month 1 remaining tasks 1.6–1.8: extract types into dedicated
modules (`config.rs`, `ipc.rs`, `error.rs`, `tab.rs`), integrate `clippy`
and `cargo fmt` checks into CI, prepare shared `Result` re-export for
downstream crates. See `docs/phase1-tasks.md` §Month 1.
