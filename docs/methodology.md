# Spiral Methodology

> **The single source of truth for "how Spiral is built."**
>
> Spiral is LLM-assisted, human-directed, adversarially reviewed, and
> test-verified. This document describes what each of those words
> means in practice, and what the project commits to.
>
> **Audience:** contributors, reviewers, and hostile readers.
>
> **Evidence base:** [`docs/research/11-llm-assisted-prior-art.md`](research/11-llm-assisted-prior-art.md)
> documents the prior art for the framing in §8.
>
> **Failure log:** [`docs/failures/`](failures/README.md) is the
> running record of LLM-specific mistakes the project has caught.
>
> **Audit scripts:** [`scripts/audit-orphan-exports.sh`](../scripts/audit-orphan-exports.sh)
> and [`scripts/audit-doc-drift.sh`](../scripts/audit-doc-drift.sh)
> are the mechanical enforcement layer. An exit 1 from either is a
> build break per [`AGENTS.md`](../AGENTS.md).

---

## §1 Position

Spiral is built with LLM assistance under human direction. Every
change is reviewed before it lands. The methodology is documented
publicly so it can be audited, replicated, and improved.

The author of Spiral is a designer, not an engineer. The author
directs the work, sets the scope, decides what to ship, decides the
public voice, and reviews the output. The author does not write
parser grammars, layout algorithms, or compiler passes. The author
proposes, the AI proposes, the review decides, the audit enforces,
the test gate is the floor.

This document is part of the project. It is not a marketing
artifact. It is a working contract between the author, the AI tools
used, the reviewers, and the readers of the code. If the
methodology drifts, the project drifts. The drift is recorded in the
failure log.

---

## §2 The model

A human (a designer, not an engineer) sets direction. The human
names the work to be done, sets the architectural constraints, and
decides what is in scope and what is not. The LLM proposes
implementations. A reviewer — human or a second LLM agent with a
different prompt — adversarially reviews every proposal. Tests gate
the merge. The audit scripts gate the merge.

Concretely, on every change:

1. The author writes a `packet` describing the work to be done:
   scope, non-scope, tests to add, audit expectations, rationale.
   The packet lives in [`docs/implementation_tracker.md`](implementation_tracker.md)
   as part of a `Step` under a `Phase` under a `Group`.

2. The LLM proposes an implementation against the packet. The
   proposal is committed to a feature branch, never to `main`.

3. The reviewer — human or LLM — reads the proposal against the
   packet. The reviewer is looking for the change to be wrong.
   The review is not a "looks good to me" stamp; it is an
   adversarial pass.

4. The CI audit runs:
   [`scripts/audit-orphan-exports.sh`](../scripts/audit-orphan-exports.sh)
   and
   [`scripts/audit-doc-drift.sh`](../scripts/audit-doc-drift.sh).
   An exit 1 is a build break. The audit cannot be bypassed by the
   author, the reviewer, or the LLM.

5. The test suite runs. An exit 1 is a build break.

6. If the audit and the tests are clean, the change lands on
   `main`. The packet is ticked. The progress ledger is updated.

7. If the LLM shipped something that the audit or the tests caught
   and that would have been embarrassing in production, an entry is
   added to [`docs/failures/`](failures/README.md) before the change
   lands. The failure is named, the audit that caught it is
   credited, and the lesson is recorded.

A `pub` symbol is not done when it compiles. It is done when at
least one consumer outside the symbol's home crate imports it. A
change is not done when it lands. It is done when the tests pass and
the audit is clean. A packet is not done when the code lands. It is
done when the failure log is current.

---

## §3 What the human decides

Vision, scope, architectural constraints, the priority of work, which
LLM proposals to accept and which to reject, the public voice, the
relationship to other engines, what to ship and what to defer. The
list is short and named.

Specifically:

- **Scope.** What is in the project and what is not. The author
  decides. The AI proposes, the author accepts or rejects.
- **Architecture.** The choice of paradigms, the choice of brand
  names (Gyre, Vortex, Forge), the choice of which dependencies
  are vendored and which are written from-scratch. The author
  decides. The AI proposes, the author accepts or rejects.
- **The packets.** The author writes the packets. The AI
  implements against them. The packet is the contract.
- **The public voice.** The author writes the methodology, the
  colophon, the README, the marketing site, the news posts. The
  AI may draft; the author edits.
- **The failures log.** The author (or the auditor agent) writes
  the entries. The AI does not edit its own failure entries
  without human review.
- **The relationship to other engines.** Chromium, WebKit, Gecko,
  Ladybird, Servo. The author decides the relationship: are we
  competing, cooperating, learning, ignoring?
- **The release schedule.** The author decides what ships and
  when. There is no calendar estimate; the author commits only to
  "the work is in the open, the audit is in the CI, the failure
  log is current."

---

## §4 What the AI does

Drafts code, drafts tests, drafts documentation, surfaces options.
The AI does not make unilateral decisions. The AI does not push
commits. The AI does not open pull requests. The AI does not edit
its own failure-log entries without human review.

Specifically:

- **The AI proposes.** It drafts implementations against a packet
  written by the author. The proposal is a starting point, not a
  finished change.
- **The AI drafts tests.** Every implementation proposal ships
  with a test. The test asserts something specific. A test that
  does not assert is a deleted test (per
  [`.spiral/rules/testing.md`](../.spiral/rules/testing.md)).
- **The AI drafts documentation.** ADRs, packet notes, code
  comments, the failure log itself. The author edits.
- **The AI surfaces options.** When the choice between two
  approaches is unclear, the AI names both and names the trade-off.
  The author decides.
- **The AI does not push to `main`.** The change goes through
  review and CI before it lands. The AI's role ends at the
  proposal stage.
- **The AI does not edit the failure log.** The failure log is
  about the AI's mistakes. Letting the AI edit its own
  failure log would be a category error.

---

## §5 Adversarial review

Every change is reviewed. The review is adversarial in two senses:
first, the reviewer is looking for the change to be wrong; second,
the audit scripts are mechanical adversaries that exit non-zero on a
class of regressions that humans do not catch reliably (over-published
`pub` symbols, retired-vocabulary regressions, dep-graph violations).
The scripts are the floor, not the ceiling.

### §5.1 The human reviewer

The human reviewer reads the proposal against the packet. The
reviewer is looking for:

- The change does what the packet said, and only what the packet
  said.
- The change is well-tested. New tests are specific; existing
  tests still pass; coverage is not a single-line test that
  asserts `true`.
- The change does not introduce a regression in a category the
  audit does not catch. The reviewer is the second pair of eyes
  for issues like silent semantic shifts, design drift, and
  architectural assumption violations.
- The change is honest. If the AI shipped something wrong and
  the audit caught it, the failure is recorded in
  [`docs/failures/`](failures/README.md). The reviewer checks
  the failure log is current.

### §5.2 The mechanical adversary

[`scripts/audit-orphan-exports.sh`](../scripts/audit-orphan-exports.sh)
and [`scripts/audit-doc-drift.sh`](../scripts/audit-doc-drift.sh)
run on every commit. An exit 1 is a build break. The audits catch
categories of regression that humans do not catch reliably:

- **`audit-orphan-exports.sh`.** Flags `pub` symbols with no
  external consumer. This is the AI's most common failure mode
  (the LLM tends to over-publish APIs "just in case"). The audit
  was    introduced during the SSOT restructure work of 2026-06-16 and caught
  twelve real leaks on its first run. See
  [`docs/failures/2026-06-18-001-render-node-id.md`](failures/2026-06-18-001-render-node-id.md)
  for the first entry.
- **`audit-doc-drift.sh`.** Flags doc-drift between the SSOT
  documents (`AGENTS.md`, the implementation tracker, the active
  context, the system architecture, the glossary, the decision
  log, the rule files) and the live state of the codebase. This
  catches retired-vocabulary regressions, dep-graph violations,
  status-row parity breaks, and the like.

The audit scripts are the *enforcement layer* on which Spiral's
novelty claim is grounded. A regression in the audit scripts is a
regression in the project's core claim.

---

## §6 Test-and-regression discipline

Every change ships with a test. Every regression is named in the
public failure log. Test counts and regression counts are public on
the marketing site and in the progress ledger. A test that does not
assert anything is a deleted test (per
[`.spiral/rules/testing.md`](../.spiral/rules/testing.md)).

Concretely:

- **Every `pub` symbol change ships with a consumer test.** The
  `audit-orphan-exports.sh` audit enforces this. A `pub` symbol
  with no external consumer is a build break.
- **Every commit that touches a parser ships with at least one
  new test for the touched grammar.** The test is added in the
  same commit. The test is specific — it asserts a real behaviour
  change, not a smoke test.
- **Every regression is named in
  [`docs/failures/`](failures/README.md).** The naming
  convention is `YYYY-MM-DD-NNN-<short-slug>.md`. Each entry
  follows the schema in the failures README.
- **The progress ledger is honest about the numbers.** Test
  counts, regression counts, audit pass/fail counts are
  recorded. Falsified numbers are a project-defining failure.

---

## §7 Named failures

The project publishes a failure log of LLM-specific mistakes: real,
named, specific, small. The log is in
[`docs/failures/`](failures/README.md) in the engine repository and
mirrored at `https://spiralbrowser.com/failures` on the marketing
site. The first entry is a `pub` struct the AI shipped with no
external consumer, caught by the audit script on its first run.

The failure log is not a confession. It is a working tool. Each
entry is a small, specific, reproducible record of an LLM failure
mode the project has encountered and addressed. The categories
emerge from the data:

- **Wiring leaks.** Over-published `pub` symbols. The audit is
  the standing guard.
- **Dependence on stale third-party crates.** The AI defaults
  to wrapping or vendoring mature third-party crates and the
  project later has to retire the wrapper and write the
  equivalent from-spec. The relevant ADRs are
  [`docs/decisions/0001-css-parser-spiral-fmt.md`](decisions/0001-css-parser-spiral-fmt.md),
  [`0002-vortex-from-scratch.md`](decisions/0002-vortex-from-scratch.md),
  and
  [`0003-gyre-rename.md`](decisions/0003-gyre-rename.md).
- **Vocabulary drift.** The AI uses time-based words
  (`Sprint`, `Chunk`, `Month`) that don't match the actual
  `Phase` / `Step` / `Packet` structure. The retired-vocabulary
  denylist in `audit-doc-drift.sh` is the standing guard.

The failure log is a load-bearing artifact. If it is empty, the
project has either shipped zero changes (unlikely) or the AI's
mistakes are not being recorded. Both are project-defining failures.
The audit cannot catch what the failure log does not record.

---

## §8 What this is not

This is a configuration choice, not a technical novelty. Two prior
projects must be named honestly.

1. **Ladybird** documented an AI-assisted, human-directed,
   adversarially-reviewed port of a browser engine in February
   2026 (Andreas Kling,
   [`https://ladybird.org/posts/adopting-rust/`](https://ladybird.org/posts/adopting-rust/)).
   The role analogue is exact: Kling is a designer and manager
   directing the work, not an engineer. The post is the
   methodology writeup. What Ladybird does *not* have is a
   CI-enforced audit script.

2. **FastRender** (Wilson Lin / Cursor, January 2026) shipped a
   300-line `AGENTS.md` methodology doc on a Rust browser engine,
   with a third-party adversarial quality audit (Software
   Improvement Group, February 2026,
   [`https://www.softwareimprovementgroup.com/blog/quality-of-fastrender/`](https://www.softwareimprovementgroup.com/blog/quality-of-fastrender/)).
   What FastRender does *not* have is a self-published, ongoing,
   LLM-mistake-specific failure log.

The evidence base for this section is
[`docs/research/11-llm-assisted-prior-art.md`](research/11-llm-assisted-prior-art.md).
The claim that *does* survive the prior art is the audit-script
enforcement layer: Spiral's methodology is operative, not
aspirational, because the CI audit grep is a build break.

The claim that does *not* survive the prior art is any of:

- "First browser to use AI assistance" — Ladybird and FastRender
  ship prior art.
- "First browser to publish a methodology document" — FastRender
  has 300 lines of it.
- "First browser to publish a named-failure log" — Mozilla
  Security Blog, Project Zero, and Monocypher all do this.
- "First browser to use adversarial review" — Chromium, Firefox,
  WebKit, and most crypto projects require it.

If you are evaluating Spiral on the AI axis, the relevant question
is: does the audit hold? If you are evaluating on the engineering
axis, the relevant question is: does the test suite pass? The
methodology is the *why*; the audit script is the *proof*.

---

## §9 What this commits to

- The methodology is part of the project. It is not a marketing
  artifact.
- The failure log is part of the project. It is not a confession.
  It is a working tool.
- The audit scripts run on every commit. An exit 1 is a build
  break. The audit cannot be bypassed.
- The prior art is cited honestly. The novelty claim is narrowed
  to the audit-enforcement axis.
- The colophon is honest. The author is a designer, not an
  engineer. See [`docs/colophon.md`](colophon.md) (when it is
  written; the colophon currently lives at
  `https://spiralbrowser.com/colophon` on the marketing site).
- The marketing site is honest. There are no binaries. The
  source is on GitHub. The release notes are empty until there
  is a release.

If any of these commitments is broken, the methodology is broken.
A broken methodology is a project-defining failure. The failure
log is the place to record the break.
