# Unsafe Code Registry

This document lists and audits all `unsafe` blocks and declarations within the Spiral Browser codebase, maintaining compliance with the [Unsafe Code Standards](file:///Users/james/spiral-browser/.spiral/rules/unsafe-standards.md).

## 1. Registry Status

As of **2026-06-16**, the codebase is verified to contain **0 active unsafe blocks**. 

| Crate | File & Line | Purpose | Safety Justification | Last Audited Commit | Audited By |
|-------|-------------|---------|----------------------|---------------------|------------|
| *None* | *N/A* | *N/A* | *N/A* | *N/A* | *N/A* |

## 2. Registration Procedure

When an implementer must introduce an `unsafe` block or function (e.g., in `spiral-vortex` or `spiral-sandbox` for hot-path optimization or OS sandboxing integrations):

1. **Safety Proof:** Precede the block with a formal `// SAFETY:` comment proving undefined behavior is impossible.
2. **Criterion Proof:** If performance-driven, attach Criterion microbenchmarks proving a significant speedup.
3. **Register Entry:** Append a row to the table in Section 1 above with:
   - Crate name.
   - File path and line range.
   - Brief purpose.
   - Safety justification summary.
   - The commit hash where it is introduced.
   - The name/role of the reviewer who approved the PR.
