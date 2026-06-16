# ADR NNNN: <Decision Title in Imperative Form>

**Status:** Proposed | Accepted | Deprecated | Superseded by ADR-NNNN
**Date:** YYYY-MM-DD
**Deciders:** <who decided — names, roles, or "the team">
**Related:** <links to architecture docs, plan sections, prior ADRs, tracker items, ledger entries>

---

## Context

<1–3 paragraphs.>

What is the situation? What forces are at play? What constraint
makes this decision non-trivial? Why is "just decide" not enough?

Anchor in specifics: a metric, a test failure, a phase boundary,
a dependency constraint, a naming rule, a sprint plan. Avoid
hand-waving.

---

## Decision

<One paragraph. State the decision so it is greppable.>

A future agent searching for "<keyword>" should find this ADR.
Use the canonical subsystem name if one applies (Gyre, Vortex,
Forge, spiral-filter, etc.). Reference the implementation
plan / chunk / item in `docs/active_context.md` or the relevant
ledger entry.

---

## Consequences

<Bullet list — what is now possible, what is now forbidden,
what the migration path looks like.>

- **Positive:** …
- **Negative:** …
- **Migration:** …

---

## Alternatives considered

<One paragraph per option that was seriously considered and
rejected. Include the reason.>

### Option A: …

Rejected because …

### Option B: …

Rejected because …

---

## Wiring & Integration

<Per the project rule (see `AGENTS.md`): a decision is not done
until its outcome is reachable from a real surface. List the
crates, call sites, and tests that exercise the decision's
outcome.>

- **Crates affected:** …
- **Call sites:** …
- **Test coverage:** …

---

## Notes

<Optional. Open questions, follow-on decisions, links to
discussion threads.>
