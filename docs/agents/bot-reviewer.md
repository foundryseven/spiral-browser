# Bot Reviewer Role — @spiral-reviewer

This document defines the operating contract for the automated reviewer bot (**@spiral-reviewer**) on internal PRs.

---

## 1. Overview
The `@spiral-reviewer` acts as the first-line automated gatekeeper for pull requests opened from branches within the main repository. Its main purpose is to catch defects early, enforce codebase health standards, and reduce the review burden on human developers.

---

## 2. Trigger Conditions
The bot is automatically triggered via the [spiral-review.yml](file:///Users/james/spiral-browser/.github/workflows/spiral-review.yml) workflow on the following pull request actions:
- `opened`
- `synchronize` (new pushes to the branch)
- `reopened`
- `ready_for_review` (when draft status is removed)
- `edited`

It is skipped for:
- Draft PRs
- Fork PRs (handled by `@spiral-reviewer-external`)
- PRs containing the `spiral:hotfix` label

---

## 3. Operating Checks & Discipline
For every run, the bot MUST perform the following checks:
1. **Orphan Exports Audit:** Run [audit-orphan-exports.sh](file:///Users/james/spiral-browser/scripts/audit-orphan-exports.sh). Every new `pub` symbol must have a cross-crate consumer (such as a unit or integration test).
2. **Doc-Drift Audit:** Run [audit-doc-drift.sh](file:///Users/james/spiral-browser/scripts/audit-doc-drift.sh). Verify that the implementation tracker, active context, progress ledger, and agents instructions remain fully consistent.
3. **Diff Analysis:** Read the diff using the OpenCode Go API with `opencode-go/minimax-m3`. Group issues into `🔴 blockers`, `🟡 should-fixes`, `🟢 nits`, and `✅ positives`.
4. **Integration Verify:** If the diff is a Rust changeset, check formatting (`cargo fmt`), lint warnings (`cargo clippy`), and tests (`cargo test`).

---

## 4. Outputs
- **PR Comment:** Posts a conversational review comment matching the tone template defined in [.opencode/agents/spiral-reviewer.md](file:///Users/james/spiral-browser/.opencode/agents/spiral-reviewer.md).
- **Check Run Status:** Sets the `opencode / review` Check Run conclusion:
  - `success` — no blockers, no should-fixes.
  - `failure` — at least one blocker OR three or more should-fixes.
  - `neutral` — docs-only changes or minor nits.
- **Auto-Fix Dispatch:** If the Check Run conclusion is a `failure`, the bot triggers the [spiral-fix.yml](file:///Users/james/spiral-browser/.github/workflows/spiral-fix.yml) workflow.
