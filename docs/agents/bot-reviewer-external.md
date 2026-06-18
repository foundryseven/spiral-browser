# Bot Reviewer (External) Role — @spiral-reviewer-external

This document defines the operating contract for the automated external reviewer bot (**@spiral-reviewer-external**) on fork PRs.

---

## 1. Overview
The `@spiral-reviewer-external` is the reviewer gatekeeper for external pull requests opened from fork repositories. Because fork PRs do not have access to repository secrets by default, this bot uses a Cloudflare Worker webhook to run securely on the edge without exposing repo secrets to untrusted forks.

---

## 2. Trigger Conditions
- Automatically triggered by the [spiral-external-gate.yml](file:///Users/james/spiral-browser/.github/workflows/spiral-external-gate.yml) workflow on external fork PR events: `opened`, `synchronize`, `reopened`, `ready_for_review`.

---

## 3. Worker Architecture & Secrets
- **Execution Environment:** Runs on Cloudflare Workers edge runtime under the project's paid Cloudflare account.
- **Encrypted Secrets:** Configured with Worker secrets:
  - `OPENCODE_GO_KEY` — OpenCode API key.
  - `GITHUB_APP_ID`, `GITHUB_APP_PRIVATE_KEY` — GitHub App credentials for authenticating requests.
  - `GITHUB_APP_INSTALLATION_ID` — GitHub App installation context.
- **Token Auth:** Uses Web Crypto API to sign an RS256 JWT, generating a short-lived installation token for the target repository. No static GitHub tokens are ever persisted on the edge or in code.

---

## 4. Review Process & Conclusion
The worker executes the following workflow:
1. **Fetch Diff:** Downloads the raw diff of the PR from the public GitHub API.
2. **Fetch Prompts:** Downloads the external reviewer agent prompt [.opencode/agents/spiral-reviewer-external.md](file:///Users/james/spiral-browser/.opencode/agents/spiral-reviewer-external.md) and prompt template [.opencode/prompts/review-external.md](file:///Users/james/spiral-browser/.opencode/prompts/review-external.md) from the `master` branch.
3. **Reasoning:** Invokes `opencode-go/minimax-m3` on the diff and instructions.
4. **Conclusion Analysis:** Parses the response to classify the outcome:
   - `failure` if it contains a `🔴` blocker emoji.
   - `neutral` if it contains a `🟡` should-fix emoji.
   - `success` otherwise.
5. **Post Comments:** Posts the review comment directly on the PR, and creates/updates the `opencode / review` Check Run.

---

## 5. Opt-in Fixer Mechanics
- Every comment posted by the external reviewer MUST end with the opt-in offer:
  *"If you would like the bot to attempt to automatically fix the blockers and should-fixes listed above, please comment `@spiral-fixer Fix my code` on this PR."*
- Pushing fix commits to a fork branch is gated by this phrase, running inside the maintainer's token context only after the explicit user comment is posted.
