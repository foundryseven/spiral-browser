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

**Always start with the context primer.** Before reading
any docs manually, run:

```bash
bin/spiral-context.sh             # session start (no packet)
bin/spiral-context.sh <packet-id> # picking up a specific packet
# or equivalently:
just context [<packet-id>]
```

This prints the 6 always-relevant files plus the
packet-specific tracker line, Step header, architecture
doc, pre-expanded block, and recent test files. It
replaces the manual "read 6 files" sequence below with a
single command. Skip the manual sequence **only if** the
context script fails (e.g., no `bash` available); in that
case fall through to the manual sequence.

If you are picking up a specific packet and the context
script output is sufficient (it surfaces the architecture
doc, the expansion block, and any recent relevant tests),
you do NOT need to re-read those files manually. Re-read
only if the script's output is stale or missing.

### Manual fallback (only when the script fails)

Before touching any file, confirm you have read in this
order:

- [ ] `AGENTS.md` (project operating contract)
- [ ] `docs/active_context.md` (live Phase state)
- [ ] `docs/implementation_tracker.md` (the source of
  truth for status — Group → Phase → Step → Packet)
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
- [ ] The relevant rule file in `.spiral/rules/`
  (architecture, coding-standards, testing)

If you cannot point to the unchecked packet you
are picking up in `docs/implementation_tracker.md`,
**stop and ask**. The tracker is the source of truth;
the user prompt may be a hint, but it is not a ticket.

---

## 1.1 Packet Pre-Expansion (when writing the tracker)

When a packet is first added to
`docs/implementation_tracker.md`, expand it beyond the
one-line summary. The next implementer who picks it up
should not have to guess where the code goes or what
consumes it. Use this expansion shape:

```
- [ ] **Packet X.Y.Z** — Title (WHATWG / RFC / etc. §NN).
      - **Spec:** one-line spec reference + the precise
        section to read.
      - **Crates affected:** which crates will be touched.
      - **Call sites expected:** the file:line locations
        where the new symbol will be invoked from. Use
        `?` for not-yet-known sites (better than nothing).
      - **Tests expected:** the test file path + a list of
        test names that must exist before the packet
        ships.
      - **End-to-end surface:** the human-verifiable
        artefact. "API added" is not end-to-end; "running
        `cargo run --bin spiral-foo` prints X" is.
      - **ADR required:** YES if the change swaps a dep,
        renames a crate, or alters the public type
        surface; NO otherwise.
      - **Architecture doc:** which `docs/architecture/<...>.md`
        to read first.
```

A fully-expanded packet saves the next implementer 15-30
minutes of "where does this go?" archaeology. If you are
the implementer picking up an unexpanded packet, expand it
*before* writing code — that's a small upfront cost that
returns on every subsequent reader.

If you are picking up one of these, **read the linked architecture doc
first** (`docs/architecture/fmt.md`, `vortex.md`, `gyre.md`, `net.md`),
then read the matching ADR if any, then write the failing test, then
write the code (TDFlow).

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

**Self-check before claiming "done":**

- [ ] Did I name the call sites (file:line if
      specific)?
- [ ] Did I name the test coverage that exercises
      the path?
- [ ] Did I name the end-to-end surface (a CLI
      command, a `#[test]`, a fixture run, a render
      output) that a human can verify?
- [ ] Did the Wiring & Integration section in the
      ledger entry satisfy all three of the above?

Before claiming "done", run:

```
./scripts/audit-orphan-exports.sh
```

The script exits 1 on orphans; treat exit 1 as a build
break. (The script can flag Phase 1.6+ skeletons that
genuinely are not yet wired — that is a feature, not a
bug. The point is to know which is which.)

Every ledger entry you write must include a
**Wiring & Integration** section naming the call
sites, the test coverage, and the end-to-end surface.
This requirement is **required**, not optional — it is
the same shape as the four-field template in
`AGENTS.md` SSOT Update Protocol and is the
reviewer-agent's primary liveness check. See the
Phase 1 Step 1.5 ledger entry for an example.

---

## 4. The SSOT Update Protocol

After completing any task loop, the implementer must:

1. **Append an entry to `docs/progress_ledger.md`.**
   **Use the canonical template at
   [`docs/agents/ledger-template.md`](ledger-template.md).**
   The template's seven fields (What · Files changed ·
   Tests added · Wiring & Integration · Verification ·
   SSOT updates · Status) are required. The reviewer
   agent checks for these; missing any one is a blocker.
   For mid-flight handoffs (not a completed packet),
   use the HANDOVER entry format described in §7 instead.
2. **Update `docs/active_context.md`** if Phase state,
   blockers, or "do not touch" zones changed. The
   status emoji and the `Last updated:` header must be
   current.
3. **Tick the corresponding packet** in
   `docs/implementation_tracker.md` (change `[ ]` to
   `[x]`). This is the primary status update; the
   tracker is the SSOT.
4. **Create an ADR** under `docs/decisions/` if the
   task took a cross-cutting design choice. Use
   `0000-template.md`. The ADR is required if you
   renamed a crate, swapped a dep, or changed a
   public type (see the Decision Protocol table in
   `AGENTS.md`). Link the ADR from the relevant Step
   in the tracker.
5. **Run both audit scripts** before claiming
   complete: `./scripts/audit-orphan-exports.sh` and
   `./scripts/audit-doc-drift.sh`. Both must exit 0.
   This is also enforced by `bin/spiral-pr.sh` and
   `just verify-packet`.

**Decision Protocol compliance check (mandatory):**

Before writing a task loop's code, ask:

- Does the change fit the existing plan and use the
  existing toolchain? → Proceed; mention in the next
  ledger entry.
- Is it a bug fix, small refactor, or docs tweak in a
  single crate? → Proceed; mention in the next ledger
  entry.
- Does it rename a crate, swap a dep, change a public
  type, or alter the build graph? → **STOP.** Write
  an ADR first (`docs/decisions/NNNN-…md` from
  `0000-template.md`). The ADR goes in *this* commit;
  the implementation may follow.
- Does it claim "novel", "first", "unique", "no prior
  art", or "no shipped browser does this"? → **STOP.**
  Run the Novelty Claims rule (see §Novelty Claims
  in `AGENTS.md`) *before* writing code. The research
  agent must check V8, SpiderMonkey, JSC, Servo,
  Ladybird, Flow, Brave, and relevant academic
  literature.

If you are uncertain whether a change qualifies, the
Decision Protocol table is the rule of thumb: if the
fork is wider than a single-crate bug fix, write the
ADR. **The ADR is in the same commit as the
implementation, not in a follow-up commit.**

---

## 5. The Verification Checklist

Before committing (when the user asks you to commit),
or before claiming "complete", run **all six**:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
./scripts/audit-orphan-exports.sh
./scripts/audit-doc-drift.sh
```

All six must pass. The verification protocol is
listed in `AGENTS.md` § Wiring & Integration. The two
audit scripts are the ground truth for "is my change
both wired AND consistent with the SSOT?" —
`audit-orphan-exports.sh` exit 1 means at least one
`pub` symbol has no consumer and the change is not
actually wired. `audit-doc-drift.sh` exit 1 means the
SSOT (tracker, active context, AGENTS.md status row)
is inconsistent with the actual state.

**Faster scoped check during a packet:**

```bash
just verify-packet <crate>
```

This wraps `fmt + clippy + test + audit-orphan-exports`
scoped to one crate. Use it for in-cycle checks;
use the full sweep above for pre-commit / pre-PR. The
PR script (`bin/spiral-pr.sh`) runs all six
automatically before pushing — see §8.

If the audit reports orphans in crates you did not
touch, that is information, not a failure — the
tracker and ledger will explain which are Phase 1.6+
skeletons (un-wired by design) and which are real
Phase 1.x leaks.

---

## 5.1 Workflow Gates (cross-references)

The rule files under [`.spiral/rules/`](../../.spiral/rules/)
are the operative contract for "what tool, when". When
you reach one of the moments below, the cited rule file's
`MUST` line is what you follow. Do not invent a different
command; the table is the routing.

| Moment | MUST run | Rule file |
|--------|----------|-----------|
| Before adding any `Cargo.toml` dep | `cargo tree --workspace --edges normal -i <crate>` | [`.spiral/rules/architecture.md`](../../.spiral/rules/architecture.md) § Crate Boundaries |
| After promoting `pub(crate)` to `pub` | `./scripts/audit-orphan-exports.sh` | [`.spiral/rules/architecture.md`](../../.spiral/rules/architecture.md) § Wiring |
| After writing an ADR | `bin/spiral-context.sh` to re-surface SSOT | [`.spiral/rules/architecture.md`](../../.spiral/rules/architecture.md) § Workflow Tools |
| Before claiming any code change complete | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` | [`.spiral/rules/coding-standards.md`](../../.spiral/rules/coding-standards.md) § Workflow Tools |
| After editing any `.md` file | `./scripts/audit-doc-drift.sh` | [`.spiral/rules/coding-standards.md`](../../.spiral/rules/coding-standards.md) § Workflow Tools |
| Mid-cycle on a single packet | `just test-fast <crate> [pattern]` | [`.spiral/rules/testing.md`](../../.spiral/rules/testing.md) § Iteration speed |
| After a `pub` API change in `<crate>` | `just test-with-deps <crate>` | [`.spiral/rules/testing.md`](../../.spiral/rules/testing.md) § Iteration speed |
| Before claiming a perf-related packet complete | `cargo bench --workspace` | [`.spiral/rules/performance.md`](../../.spiral/rules/performance.md) § Workflow Tools |
| Before adding any `unsafe` block or `unsafe fn` | `cargo miri test -p <crate>` | [`.spiral/rules/unsafe-standards.md`](../../.spiral/rules/unsafe-standards.md) § Workflow Tools |
| Before claiming an `unsafe`-touching packet complete | `./scripts/audit-orphan-exports.sh` | [`.spiral/rules/unsafe-standards.md`](../../.spiral/rules/unsafe-standards.md) § Workflow Tools |

The full pre-commit / pre-PR sweep is the six-step
Verification Checklist in §5 above; the table above is
the per-moment routing table you consult *before* the
sweep fires. If a moment is missing from the table, the
rule file is the source of truth.

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

See the `HANDOVER: Phase 1 Step 1.5 mid-flight` entry
dated 2026-06-16 for a worked example.

For a **completed packet** (not a mid-flight handover),
write the entry using [`docs/agents/ledger-template.md`](ledger-template.md)
instead — the template's seven fields (What, Files
changed, Tests added, Wiring & Integration, Verification,
SSOT updates, Status) are the standard format the reviewer
agent checks for. A HANDOVER-style entry for a shipped
packet is a smell — the implementer ran out of time and
fired a partial entry. Finish it before handing off.

---

## 8. The Session End Rule

When the packet is green and the user wants a PR:

```bash
bin/spiral-pr.sh <packet-id>             # full flow
bin/spiral-pr.sh --dry-run <packet-id>   # preview only
bin/spiral-pr.sh --skip-tests <packet-id> # hot-fix escape
```

The PR script runs the full six-step verification
(fmts, clippys, tests, both audits) before pushing,
opens the PR with a standardised body and reviewer
checklist, and tags it `agent-implemented`. It will
refuse to push if any check fails — this is the
"PRs always go out clean" guarantee.

If the user does NOT want a PR (pure local iteration,
or they're driving the merge themselves), skip the
script and just commit. The PR script is a workflow
helper, not a gate.

**When the session is ending without a PR**, ensure:

- The tracker packet checkbox is ticked.
- The ledger entry follows the template in
  `docs/agents/ledger-template.md`.
- The active context "next up" advances.
- Both audit scripts pass (or the session-end
  warning has been acknowledged by the user).

If any of those are incomplete, write a HANDOVER entry
(see §7) instead of a normal ledger entry — the
next session needs to know what's still pending.
