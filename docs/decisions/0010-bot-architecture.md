# ADR 0010 — Bot Infrastructure (Reviewer + Fixer + External Reviewer)

- **Date:** 2026-06-18
- **Status:** Accepted
- **Supersedes:** none
- **Superseded by:** none
- **Author:** implementer agent (Packet 2.13.1)
- **Scope:** Step 2.13 in `docs/implementation_tracker.md`; the
  `.github/workflows/`, `.opencode/`, `cloudflare/`, `bin/`,
  `docs/agents/`, `docs/plans/` trees.

## Context

The Spiral Browser repo is run under the no-code-agentic model
described in `docs/plans/no-code-agentic-refactor.md`. The model
works: the user (James) is a no-code driver, and the implementer
agent drives the workflow end-to-end via the seven rule files
under `.spiral/rules/` and the role contracts under
`docs/agents/`. The blocker is that the **review** and **fix**
loops are still manual: every PR waits on the human reviewer
(`docs/agents/reviewer.md`) to read the diff and the human
implementer to apply the changes. That is the 30-60 minute per-PR
cost the model cannot absorb at any non-trivial cadence.

The user (2026-06-18) requested: "we will end up needing to build
bots after all. A review bot to review the PR and then a fixer
bot that will fix the issues, review bot will then confirm fixers
and will allow merge button to go green for me to fix." The user
also stated they would be using their **OpenCode Go** subscription
to provide the API key, and would prefer to use either their
**paid Cloudflare Workers plan** or **GitHub Actions** since
GitHub Actions is unlimited for the public spiral-browser repo.

A research pass against the current (2026-06-18) docs at
`opencode.ai/docs/go/`, `opencode.ai/docs/providers/`, and
`docs.github.com/en/billing` confirmed four facts:

1. **OpenCode Go** is a $5 first-month / $10/month subscription
   that exposes a set of frontier models under a single API key.
   The Go model list (verified 2026-06-18) includes
   `MiniMax-M3` (high-reasoning flagship), `Mimo-V2.5` (cheap
   coder, $0.14/$0.28 per 1M tokens), and `Mimo-V2.5-Pro`
   (coder-pro, $1.74/$3.48 per 1M tokens). All are reachable
   via OpenAI-compatible chat-completions or Anthropic-format
   `/v1/messages` endpoints under
   `https://opencode.ai/zen/go/v1/`.
2. **GitHub Actions is free for public repos** on standard
   runners (per the GitHub Actions billing docs: *"The use of
   standard GitHub-hosted runners is free: In public
   repositories"*). Storage is also free within limits. The
   `anomalyco/opencode/github@<SHA>` action is first-class and
   supports `pull_request` events, custom subagents, and a
   conversational `issue_comment` trigger.
3. **Cloudflare Workers** is architecturally incompatible with
   the OpenCode runtime. Workers have a 30s default CPU limit
   (5 min max) and a 128 MB memory ceiling, and there is no
   long-running process model. The OpenCode agent loop is a
   minutes-long WebSocket/HTTP interaction with an LLM provider;
   that does not fit a Worker. **Cloudflare Containers** (paid
   plan only) is a viable substrate but is unnecessary for
   internal PRs that already run on Actions for free.
4. **Fork PRs** (external contributors) cannot read repo secrets
   on `pull_request` events. They CAN read repo secrets on
   `pull_request_target` (maintainer-context), but pushing to a
   contributor's fork branch via a maintainer token requires
   explicit trust. A third option is to run the reviewer's AI
   layer in a Cloudflare Worker that the GitHub App
   installation token authorizes to comment and set Check Runs
   (no write to fork code).

## Decision

Adopt the **three-bot architecture** with the following
split:

- **`@spiral-reviewer` (internal).** Runs in GitHub Actions
  on `pull_request [opened, synchronize, reopened,
  ready_for_review]` from same-repo branches. Uses the
  `anomalyco/opencode/github@<SHA>` action with
  `model: opencode-go/minimax-m3` and
  `agent: ./.opencode/agents/spiral-reviewer.md`. Reads the
  diff, runs `./scripts/audit-orphan-exports.sh` and
  `./scripts/audit-doc-drift.sh`, posts a conversational
  review comment, and sets a GitHub **Check Run** named
  `opencode / review` with conclusion `success` or `failure`.
- **`@spiral-fixer` (internal, auto-trigger).** Runs on the
  failure of `opencode / review`. Auto-pushes fix commits to
  the PR branch. Uses **dual-model routing** based on diff
  size: ≤3 files ∧ ≤150 lines ∧ no `pub` API change →
  `opencode-go/mimo-v2.5`; else → `opencode-go/mimo-v2.5-pro`.
  Capped at **5 iterations** per PR — after that the bot
  posts a final summary and stops; humans take over.
- **`@spiral-reviewer-external` (fork PRs).** Runs in a
  **Cloudflare Worker** triggered by a GitHub App webhook on
  `pull_request` events from forks. Uses the same
  `MiniMax-M3` model and the
  `./.opencode/agents/spiral-reviewer-external.md` prompt.
  The Worker reads the diff via the public GitHub API, calls
  OpenCode Go with the `OPENCODE_GO_KEY` Worker secret, and
  posts a conversational review comment + Check Run via the
  GitHub App's installation token (no write to code).
  **Fixer is opt-in on forks:** the reviewer comment ends
  with `@spiral-fixer Fix my code`; when the maintainer or
  contributor types the phrase, a `pull_request_target`
  workflow with the maintainer's `GITHUB_TOKEN` pushes fix
  commits to the fork branch.

**Merge gate.** A branch-protection rule on `master` requires
the `opencode / review` Check Run to be `success` before merge
is allowed. Admins can bypass (for genuine emergencies). A
`spiral:hotfix` label short-circuits the iterative loop
(reviewer + fixer still comment, but the bot does not push
fix commits). The 5-iteration cap is enforced via a
`spiral/fix-iteration` counter label on the PR.

**Tone.** All reviewer comments are conversational,
peer-developer register: high-level observation first, then
specific technical callouts, then a closing offer. Internal
comments end with a "what I'd suggest" closing; external
comments end with `@spiral-fixer Fix my code` opt-in. No
bot-speak, no emoji, no lecture. Tone template lives at
`.opencode/prompts/review.md` and
`.opencode/prompts/review-external.md`.

**Cost.** Expected ~$2-5/month on the OpenCode Go $10 plan
at moderate PR cadence (10 PRs/month × ~3 review runs + 2
fix runs each). Well within the $30/week soft cap. Cloudflare
Worker free tier covers the expected ~100 webhook invocations
per day.

**Auth.**
- `secrets.OPENCODE_API_KEY` — repo secret, used by both
  internal Actions workflows. Single key, no per-bot scoping.
- `secrets.FORK_REVIEWER_WEBHOOK` — repo secret, the URL of
  the Cloudflare Worker (e.g.
  `https://spiral-fork-reviewer.<account>.workers.dev`).
- Worker secrets: `OPENCODE_GO_KEY` (the same key as the
  repo secret), `GITHUB_APP_ID`, `GITHUB_APP_PRIVATE_KEY`,
  `GITHUB_APP_INSTALLATION_ID`. The Worker uses the App's
  installation token to post comments and Check Runs; the
  token has no code-write scope.

**Security posture.**
- The `anomalyco/opencode/github@<SHA>` action is pinned to a
  full commit SHA, not a tag, per the GitHub Actions security
  hardening guide.
- Internal workflows declare `permissions: { contents: read,
  pull-requests: write, checks: write }` as the explicit
  minimum; the bot cannot push to other branches.
- The fork-PR workflow uses `pull_request_target` with the
  maintainer token only for the **fix** path; the review path
  runs in the Cloudflare Worker with no repo secrets in scope.
- The bot's review prompt explicitly forbids `git push` on
  fork branches in the external-fix workflow; the fix commit
  is built and pushed by the maintainer-token workflow, not
  the bot.
- A `CODEOWNERS` rule requires human review for changes
  under `.github/workflows/spiral-*` and `cloudflare/`, so
  the bot cannot edit its own config without human oversight.

## Why this is the right bet

The Spiral bet (`docs/system_architecture.md:56-189`) rests on
five architectural choices: shared-everything, JIT-optional,
L1-L5 filter, persistent renderer, GPU chrome. The bet is
*demonstrated* by the engine landing first and the workflow
landing second. The bet is **not** demonstrated by the
human-driven review loop continuing to bottleneck every PR.
The three-bot architecture closes that loop with the same
discipline the rest of the project uses: directive contracts
(`MUST` / `MUST NOT` in `.spiral/rules/`), per-role
subagents (the human `reviewer.md` + the three bot role
contracts), and a branch-protection gate that is enforced by
the platform, not by humans remembering to check.

The user's specific constraint — "I want the reviews on the
repo to be in natural, conversational, human language mixed
with just the right amount of technical information" — is
encoded in the `review.md` and `review-external.md` prompts,
not in the model choice. M3 is more than capable of the
register; the prompt is what guarantees it.

The user's other constraint — "for external PRs, fixer bot
optional" — is encoded by splitting the external review path
into a Cloudflare Worker (no secrets, no write) plus a
`pull_request_target` fix workflow (maintainer-token, gated on
the explicit `@spiral-fixer Fix my code` comment). The
contributor retains agency.

## Alternatives considered

### Option A — Run all three bots on GitHub Actions, no Cloudflare

Reject the Cloudflare Worker for external PRs. Use the
maintainer-token `pull_request_target` workflow for both
review and fix on forks. Pros: simpler architecture, no
Worker code to maintain. Cons: the review prompt on a fork
PR has access to the maintainer's repo secrets, which is
broader than necessary. **Rejected** because the user
explicitly asked for a separate solution for external PRs
and a Worker constrains the secret blast radius.

### Option B — Use DeepSource or a third-party PR review service

Tools like DeepSource, Codacy, or SonarCloud already review
PRs and could be configured with the Spiral rule set. Pros:
no bot code to maintain. Cons: cannot apply fixes, cannot
match the conversational tone, and the user has a DeepSource
test branch (`test/deepsource-green-button`) already that
suggests they want more, not less, in-house control over
the review voice. **Rejected** because the user explicitly
asked for "review bot" + "fixer bot" in-house.

### Option C — Use OpenCode Zen instead of Go

OpenCode Zen is a pay-as-you-go alternative with the same
model set. Pros: no subscription lock-in. Cons: per-token
pricing is higher than Go's subscription-bundled rates,
and the Go plan's $30/week soft cap is well within our
expected spend. **Rejected** because the user has the Go
subscription and the cost posture is identical or better.

### Option D — One bot, two modes

Use a single `@spiral-reviewer` bot that branches on
`github.event.pull_request.head.repo.fork` to decide between
"internal mode" (full review + auto-fix) and "external mode"
(review only + opt-in fix). Pros: simpler workflow file
count. Cons: harder to reason about permissions, harder to
attribution, harder to revoke one mode without breaking the
other. **Rejected** in favour of three named bots for clean
attribution and per-bot permission scoping.

### Option E — Auto-trigger fixer on external PRs

Mirror the internal auto-fix behaviour for forks. Pros:
faster turnaround. Cons: violates the user's explicit
"fixer bot optional for external PRs" constraint.
**Rejected** because the constraint is non-negotiable.

## Novelty check (per `AGENTS.md` §"Novelty Claims")

The three-bot architecture is **not novel**. Every
component is configuration of well-known tooling:
GitHub Actions (since 2019), OpenCode (since 2025), Cloudflare
Workers (since 2017). The novel-tending aspects of Spiral
(brand types, shared Vortex heap, L1-L5 filter, GPU chrome)
are unchanged. The claim survives: "three-bot infrastructure"
is a workflow bet, not a technical novelty, and belongs
alongside other workflow tooling (DeepSource, Renovate,
Dependabot) rather than alongside the engine architecture.

## Wiring & Integration

- **Crates affected:** none. Zero Rust code touched.
- **Touched trees:** `.github/workflows/` (5 new files),
  `.opencode/agents/` (3 new subagent defs),
  `.opencode/prompts/` (3 new prompt files),
  `cloudflare/spiral-fork-reviewer/` (new Worker source),
  `bin/` (2 new helper scripts),
  `docs/agents/` (3 new bot role contracts + README
  update), `docs/decisions/0010-bot-architecture.md`
  (this ADR), `docs/plans/bot-rollout.md` (new rollout
  plan), `AGENTS.md` (workflow-table update),
  `docs/active_context.md` (status),
  `docs/implementation_tracker.md` (Step 2.13 packet).
- **Call sites:**
  - `.github/workflows/spiral-review.yml` (new) — calls the
    opencode action with model `opencode-go/minimax-m3`
    and agent `./.opencode/agents/spiral-reviewer.md`.
  - `.github/workflows/spiral-fix.yml` (new) — calls the
    same action with model `opencode-go/mimo-v2.5`
    (routed to `-pro` per the diff heuristic) and
    agent `./.opencode/agents/spiral-fixer.md`.
  - `.github/workflows/spiral-external-gate.yml` (new) —
    `pull_request_target` on fork PRs; calls
    `https://<worker>.workers.dev/review` with the diff
    payload.
  - `.github/workflows/spiral-external-fix.yml` (new) —
    `issue_comment` matching `@spiral-fixer Fix my code`
    on a fork PR; uses `pull_request_target` with the
    maintainer token to push fix commits to the fork
    branch.
  - `cloudflare/spiral-fork-reviewer/src/index.ts` (new)
    — Worker fetch handler. Receives GitHub App webhooks,
    calls OpenCode Go, posts Check Runs + comments.
  - `bin/install-bot-secrets.sh` (new) — `gh secret set`
    wrapper.
  - `bin/spiral-bot-status.sh` (new) — last-5-runs
    status script.
- **Test coverage:** No new Rust unit tests. Verification
  is workflow-side: a deliberately-broken dry-run PR
  triggers (a) a `failure` Check Run, (b) a fix commit
  pushed within 5 min, (c) a `success` Check Run on
  re-review, (d) branch-protection merge-button state
  flips green.
- **End-to-end surface:** A maintainer opens a PR with a
  known defect (orphan `pub` symbol). Within 5 min:
  reviewer posts comment + sets `opencode / review` to
  `failure`. Within 5 min: fixer auto-pushes a fix
  commit. Re-review sets the Check Run to `success`.
  Merge button enables. Loop repeats until either
  review is `success` or the iteration cap (5) trips.
- **Audit expectation:** `./scripts/audit-orphan-exports.sh`
  and `./scripts/audit-doc-drift.sh` continue to exit 0.
  The bot review workflow itself calls both audits; a
  bot-suggested change that introduces orphans will be
  flagged on the next review pass.
- **Manual one-time steps (cannot be in repo):**
  1. Install the OpenCode GitHub App on the repo.
  2. Create the `spiral-reviewer-external` GitHub App
     (or reuse the OpenCode one with restricted scope);
     install on the repo; note the App ID.
  3. Deploy the Cloudflare Worker (`wrangler deploy`);
     set Worker secrets `OPENCODE_GO_KEY`, `GITHUB_APP_ID`,
     `GITHUB_APP_PRIVATE_KEY`, `GITHUB_APP_INSTALLATION_ID`.
  4. Configure branch protection on `master`: required
     check `opencode / review`, admin bypass allowed,
     require linear history, do not require admin to
     bypass the linear history rule.
  5. Set repo secrets `OPENCODE_API_KEY` and
     `FORK_REVIEWER_WEBHOOK` via
     `bin/install-bot-secrets.sh`.
- **Follow-on (R8, deferred):** if the Cloudflare Worker
  is over-budget or the latency is unacceptable, the
  external-reviewer can move to a Cloudflare Container
  that runs a persistent OpenCode runtime. ADR stays
  valid; only the Worker source changes.
