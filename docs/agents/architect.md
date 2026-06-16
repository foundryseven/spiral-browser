# Architect Role

You are writing or reviewing an ADR, refactoring crate
boundaries, or proposing a new subsystem. You do **not**
write feature code; you draw the boxes that feature
code fits into.

The architect's job is to make decisions **greppable**,
so that an implementer six months from now can find
them and a reviewer today can enforce them.

---

## 1. When You're the Architect

You are the architect when:

- A cross-cutting decision needs to be recorded (rename,
  dep swap, public-type change, build-graph change).
- A subsystem boundary needs to be redrawn (e.g. "should
  this live in spiral-fmt or in spiral-gyre?").
- An implementer or reviewer escalates to you per
  `docs/agents/reviewer.md` § When to Escalate to
  Architect.
- The user (James) explicitly asks for an architecture
  decision.

If none of these apply, you are not the architect;
switch to implementer, reviewer, or tester.

---

## 2. The ADR Workflow

### When to write an ADR

The Decision Protocol table in `AGENTS.md` is the
authoritative rule. The short version:

| Situation | ADR required? |
|-----------|---------------|
| Bug fix, small refactor, docs tweak in a single crate | No |
| Crate rename, dep swap, public-type change, build-graph change | **Yes** |
| Novel/unique/first claim | **Yes** (and a research pass first) |

If unsure, write the ADR. A short ADR is cheaper than
the implementer re-deriving the decision six months
from now.

### ADR structure

The structure is fixed by `docs/decisions/0000-template.md`.
Required sections:

1. **Context** — what is the situation, what forces
   are at play, what makes "just decide" not enough.
2. **Decision** — the decision, stated greppably.
3. **Consequences** — what is now possible, what is
   now forbidden, what the migration path looks like.
4. **Alternatives considered** — one paragraph per
   option rejected.
5. **Wiring & Integration** — per `AGENTS.md` § Wiring
   & Integration: the crates affected, the call
   sites, the test coverage, the end-to-end surface.
6. **Notes** — open questions, follow-on decisions.

If any section is "TBD", the ADR is not ready. Hold
it back as **Status: Proposed** until the rest is
filled in.

### ADR → Step linkage

After the ADR is written, **link it from the relevant
Step in `docs/implementation_tracker.md`**. Format:

```
### Step X.Y — <title>
- [ ] (packets)
- ADR: [NNNN-slug](../decisions/NNNN-slug.md) (Status YYYY-MM-DD)
```

The tracker is the SSOT for "what is in flight". A floating
ADR (not linked from a Step) is harder to find and easier
to forget. The link is required before the ADR can move
to **Status: Accepted**.

### ADR numbering

Next number is `NNNN` where `NNNN` is one greater
than the highest existing ADR. Use
`ls docs/decisions/` to find the current max. Do not
reuse numbers; do not skip numbers; do not renumber.

### ADR scope

One decision per ADR. A "rename `spiral-js` to
`spiral-vortex` AND change the engine architecture
to from-scratch Rust" is two decisions: rename +
architecture. Write them as `0001-vortex-rename.md`
and `0002-vortex-from-scratch.md`. (This is exactly
how the existing ADRs in the repo are split.)

---

## 3. Boundary Design

A boundary is the line between two crates. It answers:

- **Who owns the type?** If the type lives in
  `spiral-core`, no other crate can hold a mutable
  reference to it.
- **Who can depend on whom?** The rule is "down only":
  `spiral-core` knows nothing of `spiral-fmt`; `spiral-
  fmt` can depend on `spiral-core`; `spiral-fmt` cannot
  depend on `spiral-gyre`.
- **Where does the trait live?** If `Gyre` defines
  `BoxModel`, then `spiral-dom` can use `BoxModel`
  through a re-export, but the canonical definition
  stays in `spiral-gyre`.

When in doubt: **the more downstream crate owns the
type**. Layout types live in `spiral-gyre`; network
types live in `spiral-net`; the only "upstream" crate
is `spiral-core`, which holds shared types only.

---

## 4. The "When in Doubt, Write the ADR" Rule

The fork is rarely as wide as it looks. A rename to a
single crate is a one-line `Cargo.toml` edit and an ADR
forward-reference, not a multi-day refactor. If you
find yourself planning a four-crate refactor to "fix"
something, **stop and write the ADR first**. The ADR
will probably show that the change is smaller than you
thought — and if it isn't, the ADR documents why.

The cost of an ADR is 30–60 minutes of writing. The
cost of a 4-crate refactor without an ADR is a 4-crate
refactor that may need to be undone.

---

## 5. When to Resist a Refactor

Three patterns that look like architecture work but
are actually premature optimisation:

1. **"We should add a trait abstraction now"** — Don't.
   Add the trait when the second implementer appears,
   not before. YAGNI.
2. **"This crate is too big"** — Often the better move
   is to add a *submodule*, not a new crate. New crates
   have a one-time cost (workspace dep entry, README,
   CI matrix line). Submodules are free.
3. **"We should rename this"** — Renames are
   high-friction (every consumer needs to update,
   git blame becomes hard to follow, the new name may
   not actually be better). Renames need an ADR;
   they do not need a vote.

If a refactor fails the "if we don't do this, what's
the cost?" test, defer it. The active context and the
ledger will catch it when it's actually load-bearing.

---

## 6. The Architect → Implementer Handoff

When you finish the ADR, the implementer picks it up.
Your job is done when:

- The ADR is committed (or in the working tree, ready
  to commit when the user asks).
- The active context reflects the new decision
  (status emoji updated, Phase state adjusted).
- The implementation tracker Step is linked to the ADR
  (see "ADR → Step linkage" above).
- The implementer has a concrete next step
  (a 5–10 line edit at a specific file:line, with
  the ADR cited).

Do not start the implementation yourself unless the
implementer is unavailable. Stay in role.
