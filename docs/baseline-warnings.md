# `cargo check --workspace` Baseline Warnings

**Captured:** 2026-06-14
**Toolchain:** stable (macOS / Apple Silicon)
**Purpose:** Drift detection. If new warnings appear that are not listed below, the PR that introduced them must fix them.

---

## Warning Summary

**Zero warnings.** Clean baseline after fixing all unused imports and variables in stub crates.

If a future `cargo check --workspace` produces warnings not present at this
snapshot, the introducing PR must resolve them before merging.
