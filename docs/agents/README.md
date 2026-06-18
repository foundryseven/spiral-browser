# Agent Roles — Spiral Browser

This directory is the **operating contract** for autonomous
agents (human or AI) working on the Spiral Browser repository.
Every agent must identify its role before starting work, then
read the matching role doc end-to-end.

The model is borrowed from the Zeus repo's
`docs/agents/README.md` (2026-06-16, full Tier 1+2+3
restructure).

---

## The Roster

| Role | Doc | When to invoke |
|------|-----|----------------|
| **Implementer** | [`implementer.md`](./implementer.md) | Default. You are picking up a feature ticket and writing code. |
| **Reviewer** | [`reviewer.md`](./reviewer.md) | You are reviewing a diff, PR, or commit before merge. |
| **Architect** | [`architect.md`](./architect.md) | You are writing an ADR, refactoring boundaries, or proposing a new subsystem. |
| **Tester** | [`tester.md`](./tester.md) | You are writing, auditing, or hardening tests. |
| **Bot Reviewer** | [`bot-reviewer.md`](./bot-reviewer.md) | Automated first-line gatekeeper for internal PRs. |
| **Bot Fixer** | [`bot-fixer.md`](./bot-fixer.md) | Automated corrector for internal/external PR defects. |
| **Bot Reviewer (Ext)** | [`bot-reviewer-external.md`](./bot-reviewer-external.md) | Automated reviewer for fork PRs running on Cloudflare. |

If you have not been told your role, **you are an
implementer**. This is the default. Switching roles
without explicit reason is a red flag.

---

## How role docs work with the rest of the SSOT

```
AGENTS.md                  — operating contract, "read first"
docs/active_context.md     — current sprint state (SSOT for in-flight work)
docs/progress_ledger.md    — chronological log of completed work
docs/glossary.md           — engine brand names (Gyre, Vortex, Forge, …)
docs/decisions/            — cross-cutting decisions (ADRs)
docs/architecture/         — per-subsystem architecture (gyre, vortex, …)
docs/agents/<role>.md      — your operating contract for this task
docs/agents/README.md      — this file
specs/GAP_ANALYSIS.md      — gap tracker + Deltas (1, 2, 3, 4)
docs/audits/               — baseline audit + sprint plan
```

The role doc is the **one** file you read *in addition
to* `AGENTS.md` and `active_context.md`. Do not read
every role doc — that wastes context window. Read the
one that matches the work.

---

## Hard prohibitions (apply to all roles)

- **Do not commit unless the user explicitly asks.**
  This is the project rule, not a suggestion. The
  default working state is "all changes in the
  working tree, unstaged."
- **Do not change the build graph without an ADR.**
  Renames, dep swaps, public-type changes all need
  `docs/decisions/NNNN-…md`.
- **Do not overclaim.** If a feature is "novel,
  unique, first", verify against V8, SpiderMonkey,
  JSC, Servo, Ladybird, Flow, Brave, and the
  academic literature *before* writing the design
  doc. The M4 audit methodology is the canonical
  standard.
- **Do not leave orphan exports.** A `pub` symbol
  is not done until something outside its home
  crate imports it. Run
  `./scripts/audit-orphan-exports.sh` before any
  "wiring complete" claim.
- **Do not use American English.** Australian
  English spelling (`initialise`, `colour`,
  `behaviour`). Tracked at the global developer
  config level.
