# Prompt Library — Spiral

Canonical prompts for recurring agent tasks. **Living document** —
append new patterns as you discover them. Each prompt is a starting
point; adapt to context, never paste verbatim.

The full read-first sequence lives in
[`onboarding.md`](onboarding.md). Read that first.

---

## 1. Read Context and Propose a Plan

Use when: starting a session with no specific task, or asked
"what should we work on next?"

```
You are picking up work in the Spiral Browser repository. Before
touching code, read in this exact order and summarise what you found:

1. AGENTS.md
2. docs/active_context.md (current Phase)
3. docs/implementation_tracker.md (next unchecked packets)
4. docs/progress_ledger.md (last 3 entries)
5. docs/agents/README.md (your role)
6. The role doc matching your task
7. The relevant architecture doc in docs/architecture/
8. .spiral/rules/ — the three rule files

Then:
- State the current Phase goal
- Identify the next 3 unchecked packets
- Pick the first one and propose a plan
- Identify the ADR (if any) that must be read first
- Identify the risk and the wiring path
```

---

## 2. Pick Up a Packet from the Tracker

Use when: starting work on a specific packet in the implementation
tracker.

```
You are picking up Packet X.Y.N — <title> from
docs/implementation_tracker.md § Phase X.

Before writing any code:

1. Read the Step context (Phase X § Step X.Y).
2. Read the linked architecture doc.
3. Read the linked ADR (if any) in docs/decisions/.
4. Read the relevant role doc in docs/agents/.
5. Run the audit script: ./scripts/audit-orphan-exports.sh
6. Confirm the verification protocol is green:
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace

Then execute TDFlow:
- Write the failing test first
- Run it (must fail for the right reason)
- Write the minimum code
- Run the test (must pass)
- Refactor

End with:
- Update the packet: [ ] -> [x]
- Append a ledger entry to docs/progress_ledger.md
- Update docs/active_context.md if the in-flight Step changed
- Run the verification protocol again
```

---

## 3. Write an ADR for a Cross-Cutting Decision

Use when: the change meets the Decision Protocol threshold (rename,
dep swap, public type, build graph, novel/first claim).

```
You are writing ADR NNNN for the following decision:

<one sentence: the decision>

Steps:
1. Copy docs/decisions/0000-template.md to docs/decisions/NNNN-<slug>.md
2. Fill in: Status (Proposed), Date (today), Deciders, Related.
3. Write the Context section: 1-3 paragraphs, anchored in specifics.
4. Write the Decision section: one paragraph, greppable.
5. Write the Alternatives Considered section: at least 2 alternatives.
6. Write the Consequences section: positive and negative.
7. Write the Wiring & Integration section: which crates, which call
   sites, which test coverage, which end-to-end surface.
8. Add the ADR index entry to docs/decisions/README.md.
9. Link the ADR from the relevant Step in
   docs/implementation_tracker.md.
10. Do NOT commit. Hand off for review.
```

---

## 4. Run the Verification Protocol

Use when: claiming "done" or "tests pass".

```
Run these in order. Paste the output.

1. cargo fmt --all -- --check
2. cargo clippy --workspace --all-targets -- -D warnings
3. cargo test --workspace
4. cargo test --doc --workspace
5. ./scripts/audit-orphan-exports.sh

All five must exit 0. If any exits non-zero, fix it before claiming
done. The audit script exits 1 on orphan exports; treat that as a
build break (see docs/implementation_tracker.md § Wiring & Integration
Rule).
```

---

## 5. Map Today's Task to the Implementation Tracker

Use when: someone says "I'm working on X" and X is not yet a packet.

```
Take the work the user is describing and map it to the tracker:

1. Which Group? (Engines / Networking / Presentation / Cross-cutting)
2. Which Phase? (existing or new?)
3. Which Step? (existing or new?)
4. Which packets?

For each new packet, write:
- Title (imperative form: "Implement X", "Audit Y")
- Acceptance criteria (1-3 bullets)
- A test surface (which crate's tests will exercise it)
- The wiring path (which crate consumes it)

Append to docs/implementation_tracker.md. If the new Step crosses
a Phase boundary, add a Phase entry too. If it takes a cross-cutting
decision, write an ADR.
```

---

## 6. Do a Wiring & Integration Audit

Use when: reviewing a diff, or claiming "this is done".

```
For each `pub` symbol in the diff:

1. Does it have a consumer outside its home crate?
2. If no: it's an orphan. Add an integration test that imports it
   from another crate, or mark it as `#[doc(hidden)]` if it's
   truly internal.

For each new public function:

1. Is there at least one call site in another crate, or in the
   binary surface (spiral-browser)?
2. Is the test coverage external (tests/<filename>_surface.rs)?

For each new crate:

1. Is spiral-browser (the binary) actually importing from it?
2. If no: the crate is un-wired. Either add a consumer or remove
   the crate from the workspace.

Run ./scripts/audit-orphan-exports.sh. Exit 0 = wired. Exit 1 = gaps.
```

---

## 7. The Novel / First Claim Verification

Use when: someone says "novel", "first", "unique", "no prior art",
"no shipped browser does this".

```
You are verifying a novelty claim. Be skeptical; overclaim is the
default failure mode.

1. State the claim precisely.
2. Check prior art across:
   - V8 (Google's JavaScript engine)
   - SpiderMonkey (Firefox)
   - JavaScriptCore (Safari)
   - Servo (research browser)
   - Ladybird (independent browser)
   - Flow Browser
   - Brave (Chromium fork with novel policies)
   - Academic literature (search arXiv, ACM, USENIX)
3. Wikipedia is a starting point, NOT a conclusion.
4. If prior art is found:
   - Downgrade the claim honestly.
   - "Partially novel (combination is new)" or
   - "Configuration choice" are valid categories.
5. If no prior art is found:
   - Document the search.
   - Cite the prior art that was considered and ruled out.
6. Append the verification to docs/innovations/backlog.md with a
   clear "verified novel" or "downgraded" tag.

The M4 audit methodology (docs/audit-sprint-m4.md) is the canonical
standard. Four rounds of retrospective correction taught us that
overclaiming is the default failure mode — gate it proactively.
```

---

## 8. The Phase-Close Walk

Use when: the release role says "Phase X is closing".

```
Walk the Phase in docs/implementation_tracker.md:

1. For each Step:
   - All packets are [x]? If any are [ ], move them to a later Phase
     with a note.
   - If any are [~] (partial), that means the Step is not done.
     Either finish it or split the partial packets out.
2. Confirm the Phase's Wiring & Integration section is complete.
3. Append a "Closed" section at the top of the Phase block:
   "Closed @ vX.Y.Z on YYYY-MM-DD. See docs/releases/X.Y.Z.md."
4. Update the Phases table at the top of the tracker.
5. Bump the version in Cargo.toml.
6. Update CHANGELOG.md.
7. Write docs/releases/X.Y.Z.md.
8. Update docs/active_context.md to the next in-flight Phase.
9. Append a "Phase close" entry to docs/progress_ledger.md.
10. Tag the commit vX.Y.Z.
```

---

## 9. The "I Don't Know My Role" Tie-Breaker

Use when: an agent is unsure which role to play.

```
If you are:
- Writing feature code ............ implementer
- Reviewing a diff ................ reviewer
- Designing a new crate or boundary  architect
- Writing tests only .............. tester
- Auditing a security surface ..... security
- Cutting a release .............. release
- New to the repo ................. onboarding

Default: implementer. Read the role doc end-to-end before touching
code. If you are still unsure, ask the user.
```

---

Borrowed 2026-06-16 from the Zeus repo's
[`docs/agents/PROMPT_LIBRARY.md`](https://github.com/zeus-repo/zeus/),
adapted to Spiral's Group → Phase → Step → Packet model.
