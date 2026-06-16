# Implementer Role

The **default role** for any agent picking up a feature
ticket, wiring task, or subsystem implementation. You write
code, tests, and documentation. You update the progress
ledger. You commit (only when the user asks). You hand off
for review.

If you have not been told your role, you are an
implementer.

---

## 1. Pre-Flight Checklist

Before touching any file, confirm you have read in this
order:

- [ ] `AGENTS.md` (project operating contract)
- [ ] `docs/active_context.md` (live sprint state)
- [ ] `docs/progress_ledger.md` (last 3 entries —
  to see what just shipped)
- [ ] `docs/glossary.md` (so engine names make sense)
- [ ] The relevant `docs/architecture/<subsystem>.md` for
  the crate you are touching (e.g. `docs/architecture/
  gyre.md` for layout work)
- [ ] The relevant `docs/decisions/NNNN-…md` if the
  work touches a documented cross-cutting decision
  (e.g. `docs/decisions/0001-css-parser-spiral-fmt.md`
  before changing the CSS parser)

If you cannot point to the unchecked tracker item you
are picking up, **stop and ask**. The task tracker
(`specs/GAP_ANALYSIS.md`, `docs/active_context.md`,
`docs/progress_ledger.md`) is the source of truth;
the user prompt may be a hint, but it is not a ticket.

---

## 2. The TDFlow Loop

Spiral follows **TDFlow** (test-driven flow). When
generating a new function, struct, parser rule, layout
algorithm, or data controller:

1. **Write the test first.** The test must assert the
   real behaviour, not a placeholder.
2. **Run the test. It must fail.** A test that passes
   before the implementation is a smell — either the
   test is hollow or the implementation already
   exists.
3. **Write the minimum implementation to pass the
   test.**
4. **Run the test. It must pass.**
5. **Refactor.** Tighten the implementation, keep the
   test green.
6. **Wire it.** Move to §3 below.

For pure documentation work (no new code) the TDFlow
loop does not apply. Skip to §3.

---

## 3. The Wiring & Integration Rule

A change is not done until its outcome is reachable
from a real surface. Concretely:

- A `pub` symbol must be imported by at least one
  consumer outside its home crate.
- A library function must be exercised by at least one
  call site in another crate, or by the same crate's
  binary surface.
- A new crate must be imported by the binary surface
  (`spiral-browser`) or have a unit-test entry that
  exercises its public API.

Before claiming "done", run:

```
./scripts/audit-orphan-exports.sh
```

The script exits 1 on orphans; treat exit 1 as a build
break. (The script can flag M4.5+ skeletons that
genuinely are not yet wired — that is a feature, not
a bug. The point is to know which is which.)

Every ledger entry you write must include a
**Wiring & Integration** section naming the call
sites, the test coverage, and the end-to-end surface.
See the M4.4.1 Item 4 ledger entry for an example.

---

## 4. The SSOT Update Protocol

After completing any task loop, the implementer must:

1. **Append an entry to `docs/progress_ledger.md`.**
   Use the existing entry format (date · agent · scope ·
   summary). Match the style of recent entries; the
   ledger is append-only.
2. **Update `docs/active_context.md`** if sprint state,
   blockers, or "do not touch" zones changed. The
   status emoji (`🟢 COMPLETE`, `🟡 IN PROGRESS`, `🔴
   BLOCKED`) and the `Last updated:` header must be
   current.
3. **Append a Delta to `specs/GAP_ANALYSIS.md`** if
   you closed a tracked gap (e.g. Item 4 added Delta
   4 marking G1.2 fixed).
4. **Create an ADR** under `docs/decisions/` if the
   task took a cross-cutting design choice. Use
   `0000-template.md`. The ADR is required if you
   renamed a crate, swapped a dep, or changed a
   public type (see the Decision Protocol table in
   `AGENTS.md`).

If you are uncertain whether a change qualifies, the
Decision Protocol table is the rule of thumb: if the
fork is wider than a single-crate bug fix, write the
ADR.

---

## 5. The Verification Checklist

Before committing (when the user asks you to commit),
or before claiming "complete":

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
./scripts/audit-orphan-exports.sh
```

All five must pass. The verification protocol is
listed in `AGENTS.md` § Wiring & Integration. The
audit script is the ground truth for "wired or not".

If the audit reports orphans in crates you did not
touch, that is information, not a failure — the
ledger will explain which are M4.5+ skeletons
(un-wired by design) and which are real M4.4 leaks.

---

## 6. Style & Conventions

Spiral's style is set in `AGENTS.md` § Project Rules.
Key reminders:

- **Australian English** in all comments, docstrings,
  commit messages, ADRs, and design docs. The global
  config catches this at scan time; do not rely on it.
- **`?` over `.unwrap()`** in library code. `unwrap`
  is acceptable in tests and in `main`-style entry
  points where panic is the right failure mode.
- **`#[must_use]`** on functions that return important
  values (`pub fn parse(…) -> Result<…>`, getters,
  builders).
- **No `// …` or `// TODO: finish logic` placeholders.**
  Either write the full code or do not write the
  function. The global config bans truncation.
- **`snake_case` functions, `PascalCase` types.** No
  exceptions.
- **Sorted imports**: std, external crates, internal
  crates. `cargo fmt` handles this for you.

---

## 7. The Handover Rule

When your session is running long or the work is
mid-flight and another session is going to take over,
leave a **HANDOVER** entry at the bottom of
`docs/progress_ledger.md` with:

- The current state of the work (X of Y tests passing,
  which file is in flight).
- The next concrete step (a 5–10 line change at a
  specific file:line).
- The do-not-touch zones that the next session must
  respect.
- The verification command sequence to run on pickup.

See the `HANDOVER: Item 4 mid-flight` entry dated
2026-06-16 for a worked example.
