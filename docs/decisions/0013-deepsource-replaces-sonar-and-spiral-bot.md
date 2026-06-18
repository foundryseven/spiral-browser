# ADR 0013: Adopt DeepSource as the Rust quality gate; retire SonarCloud and Spiral-Bot

**Status:** Accepted
**Date:** 2026-06-18
**Deciders:** the team (no-code-agentic session 2026-06-18)
**Related:** [`AGENTS.md`](../AGENTS.md) §Workflow Discipline; [`docs/progress_ledger.md`](../progress_ledger.md) 2026-06-18 entries; ledger line 4783 (Spiral-Bot registration) and 4803 (Codacy → SonarCloud switch); ADR superseded by this one is *implicit* — there is no prior ADR for SonarCloud or Spiral-Bot; this ADR supersedes those operational decisions in prose only.

---

## Context

The repository has accumulated three overlapping code-quality tools over the past week:

1. **Codacy** — registered 2026-06-18 as a Code Quality gate. Bot tried to read its API v3 to drive auto-fixes; every endpoint shape returned 404 (see ledger line 4805).
2. **SonarCloud** — registered 2026-06-18 as a replacement for Codacy. Free tier has **zero Rust rules** (SonarQube supports Rust only on Developer Edition and above, which is paid). The "switch to SonarCloud" commit (4346f13) shipped a working scanner client but never produced a working scan, because no `sonar-project.properties` was committed and no `SONAR_TOKEN` was provisioned.
3. **Spiral-Bot** — a GitHub Actions CI fix-bot (Bun/TypeScript, ~600 LoC) that polls a code-quality API on a 5-min cron, calls OpenCode Go to draft fixes, and commits via the GitHub Contents API. Built for Codacy, refactored to SonarCloud. PR #4 is still open, blocked on `OPENCODE_GO_API_KEY` and `SONAR_TOKEN` secrets.

The original user goal was simple: "make sure PRs are checked for code quality before humans merge." Three constraints made the existing stack fail to deliver that:

- **SonarCloud free tier cannot scan Rust.** The compatibility error the user observed in the SonarCloud dashboard ("didn't meet the compatibility criteria") is a known limitation: SonarQube's Rust analyzer is gated behind a paid tier. No amount of config fixes that.
- **Codacy's API is not programmatically usable.** The AI Reviewer is gated behind a manual human click in the dashboard, which means there is no way to drive Codacy to green automatically.
- **Autofix is not available on the user's account.** GitHub Copilot Autofix for code scanning is not visible in the user's personal-account repo settings. The original plan to use Copilot Autofix + Dependabot to fill the gap is not actionable on this account.

What the user actually wants is a **green button**: PR is mergeable only when a code-quality check passes. The check needs to support Rust on a free public repo / personal account.

---

## Decision

**Adopt DeepSource as the code-quality gate; retire SonarCloud and Spiral-Bot.**

Concretely:

1. **Install the DeepSource GitHub App** on the personal repo (`https://github.com/foundryseven/spiral-browser`). DeepSource is free for OSS public repos. Rust is a first-class supported language.
2. **Commit a `deepsource.toml`** at the repo root enabling the `rust` and `secrets` analyzers. `test-coverage` is left disabled — coverage tracking is a separate decision and not part of this packet.
3. **Configure the DeepSource quality gate** in the DeepSource dashboard to block PRs with failing `rust` or `secrets` checks. This produces the green button the user described.
4. **Enable Dependabot security updates** for `cargo` (already on per repo security settings). Dependabot handles dependency vulnerability fixes; DeepSource does not.
5. **Configure GitHub branch protection** on `main` to require the `DeepSource` status check. The merge button is greyed out until the check is green. The human still approves the merge.
6. **Retire Spiral-Bot** entirely. Delete `.github/workflows/spiral-bot.yml` and the `bin/spiral-bot/` directory tree (10 files: `index.ts`, `ai.ts`, `github.ts`, `sonarqube.ts`, `package.json`, `.gitignore`, 3 test files, 1 prompt). Net deletion: ~600 LoC of Bun/TypeScript, plus the cron workflow and two CI secrets (`OPENCODE_GO_API_KEY`, `SONAR_TOKEN`) that were blocked on PR #4.
7. **Remove all SonarCloud and Codacy references** from `AGENTS.md`, `bin/README.md`, `docs/active_context.md`, `docs/progress_ledger.md` (existing entries stay as historical record), `CHANGELOG.md`, and the implementation tracker. The 2026-06-18 ledger entries on Spiral-Bot remain — they document a real, reversible experiment; deleting them would falsify the history.

A future agent searching for "SonarCloud", "Spiral-Bot", "Codacy", or "code-quality gate" should find this ADR.

---

## Consequences

- **Positive:**
  - One tool, one dashboard, one status check. The "green button" the user asked for is real: a passing DeepSource check unblocks the merge button on `main`.
  - Rust is a first-class language on DeepSource's free OSS tier. Hundreds of static-analysis rules apply to Spiral's 20 crates out of the box.
  - The build graph is simpler: no Bun runtime in CI, no second secret to provision, no second API client to maintain. `ci.yml` is unchanged; only the `spiral-bot.yml` workflow and `bin/spiral-bot/` are removed.
  - DeepSource's Autofix™ generates pre-built patches for many findings, surfaced as commit suggestions on the PR. Authors can accept them in one click. This is the closest available analog to Copilot Autofix on a personal account.
  - The 5-min cron that drove Spiral-Bot is gone. CI minutes drop accordingly.
- **Negative:**
  - A third-party processor now has access to Spiral's source code on every push. DeepSource is SOC 2 Type II and processes the code only for static analysis; the user accepts this trade-off because the alternative is a no-op scanner.
  - The custom retry / circuit-breaker logic that Spiral-Bot had (3 iterations, 10-min gap, GitHub Issue on exhaustion) is gone. DeepSource's quality gate is fail-fast: a red check blocks merge; the author fixes it. There is no automatic "open issue on persistent failure" fallback.
  - The OpenCode Go call path that was the original raison d'être of Spiral-Bot is gone. If a future packet needs LLM-driven multi-file refactors, it will be re-introduced as a separate, packet-scoped workflow (not a 24/7 cron).
- **Migration:**
  - PR #4 (the open Spiral-Bot PR) is now stale. Close it as superseded by this ADR.
  - The `OPENCODE_GO_API_KEY` and `SONAR_TOKEN` GitHub secrets were never set; do not set them. If they exist as unconfigured placeholders, leave them — they do no harm.
  - The Codacy GitHub App is still installed on the repo (per ledger line 4805). The SonarCloud GitHub App was never installed. Leave Codacy in place for now; removing it is a separate housekeeping decision and not blocking.
  - The `deepsource.toml` schema is the conservative subset: analyzer list only. Quality-gate behaviour is configured in the DeepSource dashboard UI, not in the file. This avoids guessing at fields whose exact names we could not verify from the public docs.

---

## Alternatives considered

### Option A: Keep SonarCloud, pay for Developer Edition

Rejected. SonarQube's Rust analyzer unlocks at the Developer Edition tier, ~$150/yr/dev-seat minimum. The free alternative (DeepSource) covers the same need without spend, and DeepSource's Rust rule set is comparable. Paying for a second scanner when the first one is free and well-regarded is not justified for an open-source public repo at this stage.

### Option B: Keep SonarCloud free tier, accept that it doesn't scan Rust

Rejected. The free tier produces zero Rust findings. It is functionally a no-op for Spiral. Continuing to maintain `bin/spiral-bot/sonarqube.ts` against a no-op scanner is busywork.

### Option C: Adopt CodeQL default setup instead of DeepSource

Considered. CodeQL is GitHub-native, free for public repos, and supports Rust with ~70+ security-and-quality queries. It is a reasonable alternative. Rejected because: (1) the Autofix-style pre-built patches that DeepSource generates are not available on CodeQL; (2) CodeQL's quality-gate configuration is more verbose than DeepSource's dashboard-driven gate; (3) DeepSource's PR report card gives a structured maintainability score that maps well to AGENTS.md's "Wiring & Integration" rules.

If a future packet reveals that DeepSource's free tier changes its terms for OSS public repos, CodeQL is the documented fallback. Reverting is a 1-file change: delete `deepsource.toml`, enable CodeQL default setup in repo security settings.

### Option D: Keep Spiral-Bot, point it at CodeQL alerts via the GitHub Code Scanning API

Considered. This was the previous research turn's "reframe" option. Rejected because: (1) the user explicitly said "we need a third-party option so Spiral Bot can just be a fix, not a review" — i.e., they want a tool that handles review natively and lets the bot do nothing, not a bot that does both; (2) the LLM-fix loop that Spiral-Bot was doing was duplicative with DeepSource's Autofix™, which is purpose-built and free.

### Option E: Build a custom CodeQL + fix-bot hybrid in-house

Rejected. The cost of maintaining a CodeQL-driven fix-bot (parse CodeQL SARIF output, prompt an LLM with file context, apply diffs, run tests, commit) exceeds the cost of using DeepSource's purpose-built Autofix™ pipeline. The "no-code-agentic" model in AGENTS.md is about not building infrastructure that the toolchain can provide.

---

## Wiring & Integration

- **Crates affected:** None. This is a tooling change; no Rust source is modified.
- **Call sites:** `deepsource.toml` (new, repo root) — consumed by the DeepSource GitHub App on every push and PR. `.github/workflows/spiral-bot.yml` and `bin/spiral-bot/` — deleted; their absence is the integration evidence.
- **Test coverage:** End-to-end verified by a deliberately-bad test commit (e.g. add `.unwrap()` in a `lib.rs` of any crate). Expected: DeepSource posts an Autofix suggestion as a commit suggestion on the PR, the status check goes red, the merge button is disabled.
- **End-to-end surface:** The DeepSource status check on `main` is the green-button signal visible to humans on every PR. The user (or any reviewer) sees a green ✓ / red ✗ next to the merge button.
- **SSOT sync (per AGENTS.md §SSOT Update Protocol):**
  - `AGENTS.md` lines 39, 50–51 — replace SonarQube/Spiral-Bot references with DeepSource.
  - `bin/README.md` line 13 — remove the `spiral-bot/` row.
  - `docs/active_context.md` lines 499–509 — remove the Spiral-Bot section; the Cloudflare Workers section above it is unchanged.
  - `docs/progress_ledger.md` — append a new entry recording the switch.
  - `docs/implementation_tracker.md` — record the change under the Workflow Refactor group (or a new "Tooling" group if one is created in a follow-up).
  - `CHANGELOG.md` — `[Unreleased]` entry under "Changed".

---

## Notes

- The 2026-06-18 ledger entries for "Spiral-Bot CI fix-bot" (line 4783) and "Spiral-Bot switched from Codacy to SonarQube Cloud" (line 4803) remain in the ledger as historical record. They document a real experiment with a real, documented reason for reversal (Autofix unavailable on a personal account; SonarCloud free tier has no Rust rules). Deleting them would falsify the project's history.
- DeepSource's `deepsource.toml` schema could not be fully verified from the public docs (which are gated behind a login wall as of 2026-06-18). The committed file uses only the most conservative schema: a `version = 1` header and an `[[analyzers]]` array with `name` and `enabled` fields. The quality gate is configured in the DeepSource dashboard UI, not the file. If the schema turns out to require additional fields, the dashboard's "Re-detect" button will surface the error.
- The plan to add `bin/spiral-bot/` is preserved on disk (`.gitignore`d `node_modules/` and `bun.lock` were the only ignored files; the source tree is committed). The `bin/spiral-bot/` directory is deleted in this commit but can be restored from git history if a future packet needs an LLM-fix loop.
