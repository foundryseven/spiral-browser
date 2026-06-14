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
- Status: merged (pending commit).
