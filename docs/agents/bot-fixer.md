# Bot Fixer Role — @spiral-fixer

This document defines the operating contract for the automated fixer bot (**@spiral-fixer**) on both internal and external (fork) PRs.

---

## 1. Overview
The `@spiral-fixer` is an automated developer agent designed to automatically resolve compilation failures, clippy warnings, formatting issues, or rule violations flagged by the reviewer bot. It works directly in the branch's workspace context to apply surgical fixes.

---

## 2. Trigger Conditions
- **Internal PRs:** Automatically triggered by the [spiral-fix.yml](file:///Users/james/spiral-browser/.github/workflows/spiral-fix.yml) workflow upon a failure conclusion of the `opencode / review` Check Run.
- **Fork PRs (Opt-in):** Triggered by the [spiral-external-fix.yml](file:///Users/james/spiral-browser/.github/workflows/spiral-external-fix.yml) workflow when a maintainer or contributor posts the comment `@spiral-fixer Fix my code` on a fork PR.

---

## 3. Dynamic Model Routing
To optimize token cost and capabilities, the fixer is dynamically routed to different models based on the size and complexity of the diff:
- **Cheap Routing:** If the diff touches $\le 3$ files AND contains $\le 150$ changed lines AND introduces no `pub` API changes, it routes to `opencode-go/mimo-v2.5`.
- **Pro Routing:** Otherwise, it routes to the high-reasoning `opencode-go/mimo-v2.5-pro` model.
- *Note:* External fork PR fixes always default to the Pro model to ensure maximum accuracy on untrusted changes.

---

## 4. Fixer Discipline & Verification
The bot must strictly follow this workflow loop:
1. **Read Review:** Read the reviewer bot's latest review comments in full.
2. **Surgical Fix:** Apply the smallest edit possible that resolves the blocker or should-fix.
3. **Verify Locally:** Before pushing, the bot MUST run:
   - `cargo fmt --all -- --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `./scripts/audit-orphan-exports.sh`
   - `./scripts/audit-doc-drift.sh`
4. **Escalate on Failure:** If local verification fails after 3 consecutive attempts to resolve a compile/test issue, the bot must stop and post a comment describing the failure.
5. **Standard Commit:** Commit changes with a scoped prefix (`fix(gyre): ...`) and push.

---

## 5. Iteration Cap & Stop Conditions
- **5-Iteration Cap:** The fixer is capped at 5 automated runs per PR to prevent infinite loops. The workflow tracks iterations using the `spiral/fix-iteration:<n>` label.
- **ADR Boundary:** The fixer is prohibited from adding new dependencies, renaming crates, or modifying public API types that would require an ADR. If such changes are needed, the bot stops and requests human architect input.
- **Scope Boundary:** If a fix would require modifying $> 10$ files or $> 500$ lines, the bot stops and escalates.
