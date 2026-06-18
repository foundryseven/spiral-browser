# 001 — `spiral-core::RenderNodeId` over-published, no consumer

- **Date:** 2026-06-18
- **Commit introduced:** `b49fafe`
- **Commit fixed:** `19dcf9e`
- **Category:** wiring-leaks

## What went wrong

During the SSOT restructure work of 2026-06-16,
`crates/spiral-core/src/lib.rs` declared `pub struct RenderNodeId(u32);`
and `pub enum DomOp { ... }` with the intent of cross-crate use. Both
types were designed as part of an upcoming rendering pipeline, but no
call site outside `spiral-core` had been written when the work was
committed. The `pub` visibility shipped, the intended consumers did not.

This is the most common LLM failure mode in this codebase: the AI
tends to declare APIs "just in case," exposing surface area that
has no consumer. The visibility is correct from a compilation
standpoint (`pub` is `pub`) but wrong from a wiring standpoint
(nothing depends on the type).

## How it was caught

`scripts/audit-orphan-exports.sh` was introduced on 2026-06-16 as
part of the SSOT restructure (per `docs/progress_ledger.md` line 1780
and the leak cleanup entry at line 1899). On its first run, the audit
flagged 48
candidate `pub` symbols with no external consumer across 19 crates.
12 of them were real leaks. `RenderNodeId` and `DomOp` were
among them.

The audit grep is the mechanical adversary: it greps for the
symbol's name across all workspace crates and reports zero hits
outside the declaring crate. The pattern is straightforward, the
detection is reliable, and humans do not catch this category
reliably because the issue is invisible at the file level — the
type compiles, the tests pass, the build is green.

## How it was fixed

Commit `19dcf9e` added `tests/spiral-core-surface.rs` with three
integration tests that exercise `RenderNodeId` and `DomOp`
through the public surface. Per the audit's exclude-pattern fix
recorded in `docs/active_context.md:107-111`, an integration test
that imports the type counts as a cross-crate consumer. After
this commit, 11 of 19 crates flipped from "leak candidates" to
"OK (all wired)" in a single change.

## Lesson learned

A `pub` symbol is not done when it compiles. It is done when at
least one consumer outside the symbol's home crate imports it.
The audit is not a checklist; an exit 1 is a build break. The
audit-script enforcement is the standing guard against this
category of failure; the human reviewer is the secondary defence.
