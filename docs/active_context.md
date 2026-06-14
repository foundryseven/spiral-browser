# Active Context

**Last updated:** 2026-06-14
**Current phase:** Phase 1 — Foundation (Months 1–3)
**Current sprint:** Sprint 0 — Repository scaffolding and documentation baseline

---

## Sprint Goal

Stand up all foundation documentation, multi-model agent infrastructure, and
repo skeleton so that feature work can begin cleanly from Sprint 1.

## In Progress

- [ ] Foundation documents: `README.md`, `LICENSE`, `CHANGELOG.md`, `SECURITY.md`,
      `CODEOWNERS`, issue/PR templates (Phase A)
- [ ] `/docs` SSOT: this file, `system_architecture.md`, `progress_ledger.md`,
      `phase1-tasks.md` (Phase B)
- [ ] Root `AGENTS.md` update with phase pointer, model routing, SSOT refs (Phase C)
- [ ] Global agent configs: architect, implementer, reviewer, test-writer (Phase D)
- [ ] Repo skeleton: `tests/`, `benches/` with `.gitkeep`; `cargo check` baseline (Phase E)

## Blocked

None.

## Do Not Touch

- `Cargo.lock` is gitignored; never edit manually.
- `Cargo.toml` workspace members are stable; do not add or remove crates
  without an architectural discussion first.
- IPC protocol types in `spiral-core` (`BrowserToRenderer`,
  `RendererToBrowser`) are drafts; do not refactor until Month 2.

## Key Decisions (frozen for this sprint)

| Decision | Value | Rationale |
|----------|-------|-----------|
| Default model | `ozore/custom` | User confirmed; single model for all agents |
| Active context write policy | Agents update autonomously per task loop | Implementer appends; reviewer flags stale state |
| Progress ledger format | Markdown bullets with ISO date | Human- and agent-readable; migrate to JSONL only if tooling requires |
| `phase1-tasks.md` | Separate file, not inline here | Keeps active_context small and frequently rewritten |

## Next Sprint

Sprint 1 — Define core types in `spiral-core` (`BrowserConfig`, `TabId`,
`IPCMessage`, `Error`). See `docs/phase1-tasks.md` §Month 1.
