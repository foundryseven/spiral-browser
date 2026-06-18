#!/usr/bin/env bash
#
# bin/spiral-pr.sh
#
# PR workflow for Spiral Browser. Wraps the
# "I shipped a packet, now I want a PR open and reviewed"
# flow so an implementer does not have to remember the
# `gh pr create` flags or forget to run the pre-flight
# checks.
#
# Usage:
#   bin/spiral-pr.sh <packet-id>
#   bin/spiral-pr.sh --dry-run <packet-id>   # show what would happen
#   bin/spiral-pr.sh --skip-tests <packet-id> # for hot-fix scenarios
#   bin/spiral-pr.sh --codacy-timeout <seconds> <packet-id>
#   bin/spiral-pr.sh --no-wait-codacy <packet-id>  # push and open, don't poll
#
# Pre-flight checks (run unless --skip-tests is set):
#   1. cargo fmt --all -- --check
#   2. cargo clippy --workspace --all-targets -- -D warnings
#   3. cargo test --workspace
#   4. scripts/audit-orphan-exports.sh
#   5. scripts/audit-doc-drift.sh
#
# After the PR is open:
#   6. Poll `gh pr checks <branch>` until Codacy's conclusion is
#      `success` or `failure`. Default timeout: 15 minutes.
#   7. If `success`: enable auto-merge (`gh pr merge --auto --squash`)
#      and post a "click the green merge button" PR comment.
#   8. If `failure`: post a "Codacy found issues" PR comment with
#      the failure summary and exit non-zero so the next agent
#      session can fix-and-retry.
#
# The agent does NOT have merge authority on `main` — the actual
# merge click is a human action. The script enables auto-merge
# so GitHub merges automatically once the human clicks; the
# human always has the final say.
#
# Adopted 2026-06-17 per the implementer-agent ergonomics
# review. Codacy wait/auto-merge behaviour adopted 2026-06-18
# per the auto-merge-on-codacy packet. See
# `docs/methodology.md` §5 and `AGENTS.md` § Codacy merge gate.

set -o pipefail

cd "$(dirname "$0")/.." || exit 1

usage() {
    sed -n '2,40p' "$0"
    exit "${1:-1}"
}

DRY_RUN=0
SKIP_TESTS=0
NO_WAIT_CODACY=0
CODACY_TIMEOUT=900   # 15 minutes default
PKT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)         DRY_RUN=1; shift ;;
        --skip-tests)      SKIP_TESTS=1; shift ;;
        --no-wait-codacy)  NO_WAIT_CODACY=1; shift ;;
        --codacy-timeout)  CODACY_TIMEOUT="$2"; shift 2 ;;
        --help|-h)         usage 0 ;;
        -*)                printf "Unknown flag: %s\n\n" "$1" >&2; usage 1 ;;
        *)                 PKT="$1"; shift ;;
    esac
done

if [[ -z "$PKT" ]]; then
    usage 1
fi

hr() { printf '\n%s\n' "------------------------------------------------------------"; }

run_step() {
    local desc="$1"
    shift
    hr
    printf ">>> %s\n" "$desc"
    hr
    if [[ $DRY_RUN -eq 1 ]]; then
        printf "[dry-run] would run: %s\n" "$*"
        return 0
    fi
    "$@"
    local rc=$?
    if [[ $rc -ne 0 ]]; then
        printf "\n[FAIL] %s exited %d. Fix and re-run.\n" "$desc" "$rc" >&2
        exit $rc
    fi
}

# ----- 1. Pre-flight checks --------------------------------------------

if [[ $SKIP_TESTS -eq 0 ]]; then
    run_step "cargo fmt --all -- --check" cargo fmt --all -- --check
    run_step "cargo clippy --workspace --all-targets -- -D warnings" \
        cargo clippy --workspace --all-targets -- -D warnings
    run_step "cargo test --workspace" cargo test --workspace
    run_step "scripts/audit-orphan-exports.sh" bash scripts/audit-orphan-exports.sh
    run_step "scripts/audit-doc-drift.sh" bash scripts/audit-doc-drift.sh
else
    printf "[skip] Pre-flight checks skipped (--skip-tests).\n"
fi

# ----- 2. Build PR title and body --------------------------------------

# Title: derive from the most recent commit subject, or fall back
# to a packet-id-based title.
TITLE=$(git log -1 --pretty=%s 2>/dev/null || echo "feat: Packet ${PKT}")

# Body: build a standard template that pulls in the ledger context.
BODY_FILE=$(mktemp -t spiral-pr-body.XXXXXX)
trap 'rm -f "$BODY_FILE"' EXIT

cat > "$BODY_FILE" <<EOF
## Packet ${PKT}

### What

<!-- One-paragraph summary of what this PR ships. -->

### Verification

<!-- Paste the output of: cargo test --workspace | tail -5 -->

### SSOT updates

<!-- Which docs/ files changed. tracker.md ticked, ledger.md appended, etc. -->

### Reviewer checklist

- [ ] Reads the ledger entry for this packet.
- [ ] Runs \`just verify-packet <crate>\` and confirms pass.
- [ ] Confirms the orphan-export audit is clean.
- [ ] Confirms the doc-drift audit is clean.
EOF

if [[ -f docs/progress_ledger.md ]]; then
    # Append the most recent ledger entry that mentions the packet id.
    LEDGER_CTX=$(grep -B1 -A20 "Packet ${PKT} " docs/progress_ledger.md 2>/dev/null \
        | head -30 || true)
    if [[ -n "$LEDGER_CTX" ]]; then
        {
            echo ""
            echo "### Recent ledger context"
            echo ""
            echo '```'
            echo "$LEDGER_CTX"
            echo '```'
        } >> "$BODY_FILE"
    fi
fi

# ----- 3. Push + create PR ---------------------------------------------

CURRENT_BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null || echo "")
if [[ -z "$CURRENT_BRANCH" ]]; then
    printf "[error] Not on a branch (detached HEAD?). Aborting.\n" >&2
    exit 1
fi

if [[ "$CURRENT_BRANCH" == "main" ]]; then
    printf "[warn] You are on main. Spiral uses main as the integration branch;\n" >&2
    printf "       PRs from main into main are unusual. Continue? [y/N] " >&2
    if [[ $DRY_RUN -eq 1 ]]; then
        printf "[dry-run] assuming yes.\n"
    else
        read -r ans
        [[ "$ans" =~ ^[Yy]$ ]] || { printf "Aborted.\n"; exit 1; }
    fi
fi

run_step "git push origin $CURRENT_BRANCH" git push origin "$CURRENT_BRANCH"

if [[ $DRY_RUN -eq 1 ]]; then
    printf "\n[dry-run] would run: gh pr create --title %q --body-file %q\n" \
        "$TITLE" "$BODY_FILE"
    printf "[dry-run] PR body preview:\n"
    sed 's/^/  /' "$BODY_FILE"
    exit 0
fi

if gh pr view "$CURRENT_BRANCH" >/dev/null 2>&1; then
    printf "[ok] PR already exists for %s. Updating body and pushing.\n" "$CURRENT_BRANCH"
    gh pr edit "$CURRENT_BRANCH" --body-file "$BODY_FILE" >/dev/null
else
    printf "[info] Creating PR for %s...\n" "$CURRENT_BRANCH"
    gh pr create \
        --title "$TITLE" \
        --body-file "$BODY_FILE" \
        --base main \
        --head "$CURRENT_BRANCH" \
        --label "agent-implemented"
fi

PR_URL=$(gh pr view "$CURRENT_BRANCH" --json url -q .url 2>/dev/null || echo "")
if [[ -n "$PR_URL" ]]; then
    printf "\n[ok] PR ready: %s\n" "$PR_URL"
else
    printf "\n[warn] Could not retrieve PR URL; check 'gh pr list'.\n" >&2
fi

# ----- 4. Wait for Codacy + handle the merge gate -----------------------

# The auto-merge behaviour is skipped in --no-wait-codacy mode (the
# caller will handle it later) and in --dry-run mode.

if [[ $DRY_RUN -eq 1 || $NO_WAIT_CODACY -eq 1 ]]; then
    if [[ $NO_WAIT_CODACY -eq 1 ]]; then
        printf "[info] --no-wait-codacy: skipping Codacy wait/auto-merge.\n"
    fi
    exit 0
fi

hr
printf ">>> Polling Codacy on %s (timeout %ds)\n" "$CURRENT_BRANCH" "$CODACY_TIMEOUT"
hr

# Get the latest commit SHA on the head branch so we can check
# checks for THAT commit, not stale ones.
HEAD_SHA=$(gh pr view "$CURRENT_BRANCH" --json headRefOid -q .headRefOid 2>/dev/null || echo "")
if [[ -z "$HEAD_SHA" ]]; then
    printf "[warn] Could not resolve head SHA; Codacy polling may be stale.\n" >&2
fi

deadline=$((SECONDS + CODACY_TIMEOUT))
poll_interval=20
last_conclusion=""
last_state=""

while (( SECONDS < deadline )); do
    # Pull the Codacy check specifically. Match by name to avoid
    # being confused by future checks.
    CHECK_JSON=$(gh api \
        "repos/$(git config --get remote.origin.url | sed -E 's#.*github.com[:/]([^/]+/[^/.]+)(\.git)?#\1#')/commits/${HEAD_SHA}/check-runs" 2>/dev/null \
        | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    print(""); sys.exit(0)
for c in d.get("check_runs", []):
    if c["name"].lower().startswith("codacy"):
        print(f"{c[\"status\"]}|{c[\"conclusion\"] or \"\"}|{c.get(\"details_url\",\"\")}")
        break
else:
    print("|notfound|")
' 2>/dev/null || echo "|error|")

    IFS='|' read -r status conclusion details <<< "$CHECK_JSON"

    # Codacy posts one check at PR-open time. While that check is
    # `in_progress` or `queued`, keep polling. When the conclusion
    # is `success` or `failure`, we're done.
    if [[ "$status" == "completed" ]]; then
        last_conclusion="$conclusion"
        last_state="completed"
        break
    fi

    last_state="$status"
    printf "  [poll] %s — Codacy %s, sleeping %ds...\n" \
        "$(date +%H:%M:%S)" "$status" "$poll_interval"
    sleep "$poll_interval"
done

hr
printf ">>> Codacy result: state=%s conclusion=%s\n" "$last_state" "$last_conclusion"
hr

if [[ -z "$last_conclusion" ]]; then
    # Timeout fired with no resolution.
    SUMMARY=$(gh api \
        "repos/$(git config --get remote.origin.url | sed -E 's#.*github.com[:/]([^/]+/[^/.]+)(\.git)?#\1#')/commits/${HEAD_SHA}/check-runs" 2>/dev/null \
        | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    print("Codacy check did not post a conclusion before timeout."); sys.exit(0)
for c in d.get("check_runs", []):
    if c["name"].lower().startswith("codacy"):
        out = c.get("output", {}) or {}
        print(f"state: {c[\"status\"]}\nconclusion: {c.get(\"conclusion\") or \"-\"}\nsummary: {out.get(\"summary\",\"\")}\nannotations: {len(out.get(\"annotations\", []) or [])}")
        break
else:
    print("Codacy check did not post a conclusion before timeout.")
' 2>/dev/null || echo "Could not retrieve Codacy check summary.")

    {
        echo "## Codacy timeout — agent needs help"
        echo ""
        echo "Codacy Static Code Analysis did not reach a conclusion within ${CODACY_TIMEOUT}s."
        echo ""
        echo '```'
        echo "$SUMMARY"
        echo '```'
        echo ""
        echo "Please review [the PR]($PR_URL) and either wait for Codacy to finish,"
        echo "re-run \`bin/spiral-pr.sh $CURRENT_BRANCH\` to retry, or intervene manually."
    } | gh pr comment "$CURRENT_BRANCH" --body-file - >/dev/null
    printf "[timeout] Codacy did not conclude within %ds. PR comment posted; agent aborting.\n" "$CODACY_TIMEOUT" >&2
    exit 2
fi

if [[ "$last_conclusion" != "success" ]]; then
    # Codacy failed.
    FAIL_SUMMARY=$(gh api \
        "repos/$(git config --get remote.origin.url | sed -E 's#.*github.com[:/]([^/]+/[^/.]+)(\.git)?#\1#')/commits/${HEAD_SHA}/check-runs" 2>/dev/null \
        | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    print("Codacy check failed; see PR for details."); sys.exit(0)
for c in d.get("check_runs", []):
    if c["name"].lower().startswith("codacy"):
        out = c.get("output", {}) or {}
        ann = out.get("annotations", []) or []
        print(f"summary: {out.get(\"summary\",\"\")}")
        print(f"annotations: {len(ann)}")
        for a in ann[:10]:
            print(f"  - {a.get(\"path\")}:{a.get(\"start_line\")} {a.get(\"message\",\"\")[:160]}")
        if len(ann) > 10:
            print(f"  ... and {len(ann) - 10} more")
        break
else:
    print("Codacy check did not report a failure summary.")
' 2>/dev/null || echo "Could not retrieve Codacy check summary.")

    {
        echo "## Codacy found issues — agent will retry"
        echo ""
        echo "Codacy Static Code Analysis reported a failure on commit \`${HEAD_SHA:0:7}\`."
        echo ""
        echo '```'
        echo "$FAIL_SUMMARY"
        echo '```'
        echo ""
        echo "Per \`AGENTS.md\` § Codacy merge gate, the next agent session MUST:"
        echo "1. Read the findings above."
        echo "2. Fix the issues on the branch."
        echo "3. Commit the fix and push."
        echo "4. Re-run \`bin/spiral-pr.sh $CURRENT_BRANCH\` to re-trigger Codacy."
        echo ""
        echo "The retry loop continues until Codacy passes or the timeout fires."
    } | gh pr comment "$CURRENT_BRANCH" --body-file - >/dev/null
    printf "[fail] Codacy reported issues. PR comment posted; agent aborting so the next session can fix-and-retry.\n" >&2
    exit 3
fi

# Codacy passed. Enable auto-merge and post the merge-button alert.

# Make sure auto-merge is enabled at the repo level (one-time, idempotent).
gh repo edit --enable-auto-merge >/dev/null 2>&1 || true

# Enable auto-merge on this PR. If auto-merge is rejected (e.g. branch
# protection does not allow it), fall back to a clear human-action alert.
if gh pr merge "$CURRENT_BRANCH" --auto --squash --delete-branch 2>/dev/null; then
    AUTO_MERGE_OK=1
else
    AUTO_MERGE_OK=0
fi

{
    echo "## Codacy is green — ready to merge"
    echo ""
    echo "Codacy Static Code Analysis passed on commit \`${HEAD_SHA:0:7}\`."
    echo ""
    if [[ $AUTO_MERGE_OK -eq 1 ]]; then
        echo "**Auto-merge is enabled.** GitHub will merge this PR once you click"
        echo "the green merge button."
        echo ""
        echo "**Click here to merge:** $PR_URL"
        echo ""
        echo "After clicking, GitHub squashes the commits and deletes the branch."
    else
        echo "**Auto-merge could not be enabled** (the repo may not allow it, or"
        echo "branch protection requires additional checks). Please merge manually:"
        echo ""
        echo "**Click here to merge:** $PR_URL"
        echo ""
        echo "Or via the CLI: \`gh pr merge $CURRENT_BRANCH --squash --delete-branch\`"
    fi
    echo ""
    echo "Per \`AGENTS.md\` § Codacy merge gate, the actual merge click is the"
    echo "human's call. The agent does not have merge authority on \`main\`."
} | gh pr comment "$CURRENT_BRANCH" --body-file - >/dev/null

printf "\n[ok] Codacy green. Auto-merge %s. PR comment with the merge-button URL posted.\n" \
    "$([[ $AUTO_MERGE_OK -eq 1 ]] && echo 'enabled' || echo 'unavailable; manual merge required')"
exit 0