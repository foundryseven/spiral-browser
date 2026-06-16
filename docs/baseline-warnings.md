# `cargo check --workspace` Baseline Warnings

**Captured:** 2026-06-14
**Toolchain:** stable (macOS / Apple Silicon)
**Purpose:** Drift detection. If new warnings appear that are not listed below, the PR that introduced them must fix them.

**Related artifacts (each with one job):**
- `docs/audits/2026-06-15-baseline.md` — functional baseline audit + M4.4–M4.6 prioritised plan
- `docs/audit-sprint-m4.md` — originality / license / novelty audit of M4 first sprint
- `specs/GAP_ANALYSIS.md` — live checkbox tracker, what is built / missing

---

## Warning Summary

**Zero warnings.** Clean baseline after fixing all unused imports and variables in stub crates.

If a future `cargo check --workspace` produces warnings not present at this
snapshot, the introducing PR must resolve them before merging.
