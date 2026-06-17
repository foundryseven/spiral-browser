# Progress Ledger Entry Template

This is the canonical template for a `docs/progress_ledger.md` entry.
Use this when appending an entry after shipping a packet. Match the
existing tone — terse, technical, file:line references throughout.
The reviewer agent uses the four required fields to verify completeness;
missing any one is a blocker.

---

## YYYY-MM-DD — Packet X.Y.Z (Title) SHIPPED

- **What:** One to three sentences. State the spec reference
  (e.g. "WHATWG HTML5 §12.4 fragment parsing algorithm"). State the
  user-visible behaviour unlocked. State any cross-cutting choice that
  required an ADR (and link it).

- **Files changed:**
  - `crates/spiral-foo/src/bar.rs:NN-MM` — what changed and why.
  - `crates/spiral-foo/tests/baz_test.rs:NN-MM` — new tests added.
  - `docs/architecture/foo.md:NN-MM` — doc updates if any.

  Each entry must use `file:line` references, not paragraph descriptions.
  The reviewer uses these to navigate.

- **Tests added:** `path/to/tests.rs:NN` — bullet list of test names
  and what each asserts. Co-generation is mandatory; this list is
  the contract.

- **Wiring & Integration:**
  - **Crates affected:** `spiral-foo`, `spiral-bar`.
  - **Call sites:** `crates/spiral-foo/src/lib.rs:NN` calls into
    `crates/spiral-bar/src/baz.rs:MM`. Each call site named with
    file:line.
  - **Test coverage:** which tests exercise the new path
    (`tests/foo_test.rs:test_xyz`).
  - **End-to-end surface:** the human-verifiable artefact. A
    `cargo run` command, a fixture file output, a `#[test]` that
    a human can read. "I added an API" is not end-to-end; "I ran
    `cargo run --bin spiral-foo` and saw X" is.

- **Verification:**
  - `cargo test -p spiral-foo` → N tests pass, 0 failures.
  - `cargo test --workspace` → N/N binaries pass, 0 failures.
  - `cargo clippy --workspace --all-targets -- -D warnings` → clean.
  - `./scripts/audit-orphan-exports.sh` → 0 findings, M/M crates wired.
  - `./scripts/audit-doc-drift.sh` → 0 findings.

- **SSOT updates:**
  - `docs/implementation_tracker.md:NN` — Packet X.Y.Z ticked, link
    to file:line of the implementation.
  - `docs/implementation_tracker.md:NN` — Phase / Step / "next up"
    header advanced.
  - `docs/active_context.md:NN` — Status row, Next up section,
    packet list refreshed.
  - `docs/agents/...` — only if a role contract changed.
  - `AGENTS.md` — only if the project operating contract changed.

- **Status:** Shipped Packet X.Y.Z. One sentence on what step
  this advances and what the next packet is.

---

## Rules of thumb

- **Date format:** `YYYY-MM-DD`. Matches the existing ledger.
- **Tense:** past. "Shipped", "Implemented", "Fixed", "Added."
  Not "Will ship" or "Shipping".
- **File:line, always.** No "see the implementation" — the
  reviewer will not grep for it. `crates/spiral-fmt/src/html/tree.rs:894`
  is greppable; "the algorithm" is not.
- **Numbers, not feelings.** "12/12 tests pass" not "tests look
  good". "149 LoC" not "small change".
- **No retrofitted sections.** If you forget a section, write the
  entry again, do not patch it. The ledger is append-only; old
  entries are not edited.
- **One entry per packet.** Do not bundle multiple packets in one
  entry; the date + packet id is the unit of granularity.
- **The "What" is the public face.** If a non-technical reader
  scans only that line, they should still know what shipped.

## When the entry does not fit the template

If the packet is unusually large or unusually small, the template
scales. A 3-line bug fix can be a single bullet under "What:" with
file:line. A cross-cutting refactor can have multiple "Files changed"
sections. But the seven required fields always appear:
**What · Files changed · Tests added · Wiring & Integration ·
Verification · SSOT updates · Status.**

Adopted 2026-06-17 per the implementer-agent ergonomics review.
See `docs/active_context.md` for the rationale and the
session-pace retrospective.