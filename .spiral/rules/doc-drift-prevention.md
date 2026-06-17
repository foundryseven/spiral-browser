---
paths:
  - "**/*.md"
  - "scripts/audit-doc-drift.sh"
---

# Doc-Drift Prevention

Documentation must reflect the live state of the codebase. Obsolete claims, incorrect status, and retired terminology are considered build breaks.

## SSOT Hierarchy

The codebase defines a strict hierarchy of documents to ensure consistency:

1. **Status SSOT**: [`docs/implementation_tracker.md`](../docs/implementation_tracker.md)
   - The single source of truth for implementation progress.
   - Organized by Group → Phase → Step → Packet.
   - Only this document carries status checkbox ticks (`[x]` / `[ ]`).
2. **Architecture SSOT**: [`docs/system_architecture.md`](../docs/system_architecture.md)
   - Defines the overall design and architecture bounds of the system.
3. **Live Pointer**: [`docs/active_context.md`](../docs/active_context.md)
   - Reflects the currently active state, active focus, active worktree, and do-not-touch zones.
4. **Append-Only Change Log**: [`docs/progress_ledger.md`](../docs/progress_ledger.md)
   - Lists the chronologically completed works and structural decisions.
5. **Decisions**: [`docs/decisions/`](../docs/decisions/)
   - Architectural Decision Records (ADRs) explaining technical forks and choices.

## Update Invariants

When completing any implementation, task, or packet, the implementer MUST:

1. **Append to the Ledger**: Add a chronological entry to `docs/progress_ledger.md` including a "Wiring & Integration" section naming crates, call sites, and test coverage.
2. **Update Active Context**: Update `docs/active_context.md` if blockers, target dates, or do-not-touch zones have changed. Ensure the claimed counts (test binaries and orphan candidates) are exactly in parity with current runs.
3. **Tick the Tracker**: Change `[ ]` to `[x]` on the completed Packet in `docs/implementation_tracker.md`.
4. **Add ADRs**: If a cross-cutting choice was made, write an ADR and reference it from the tracker.
5. **Clean Spec Files**: Ensure spec-only files (like `specs/GAP_ANALYSIS.md`) remain spec-only and do not carry checkbox status markers.

## Pre-Merge PR Checklist

Before submitting or merging any Pull Request, ensure that:

1. **Wiring Audit Passes**: Run `./scripts/audit-orphan-exports.sh` and ensure it exits 0 (no orphan exports).
2. **Doc-Drift Audit Passes**: Run `./scripts/audit-doc-drift.sh` and ensure it exits 0 (no documentation drift or retired vocabulary).
3. **Rust Hygiene is Clean**:
   - `cargo fmt --all -- --check` runs cleanly.
   - `cargo clippy --workspace --all-targets -- -D warnings` runs cleanly.
   - `cargo test --workspace` runs cleanly.

**All three are enforced automatically by `bin/spiral-pr.sh`**,
which runs them as pre-flight checks before pushing. Use
that script for the PR flow rather than running the checks
manually — see `.spiral/rules/workflow.md` for the rule on
when to invoke the workflow scripts.
