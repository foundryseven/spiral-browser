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
#
# Pre-flight checks (run unless --skip-tests is set):
#   1. cargo fmt --all -- --check
#   2. cargo clippy --workspace --all-targets -- -D warnings
#   3. cargo test --workspace
#   4. scripts/audit-orphan-exports.sh
#   5. scripts/audit-doc-drift.sh
#
# Adopted 2026-06-17 per the implementer-agent ergonomics
# review. See `docs/active_context.md` for the rationale.

set -o pipefail

cd "$(dirname "$0")/.." || exit 1

usage() {
    sed -n '2,30p' "$0"
    exit "${1:-1}"
}

DRY_RUN=0
SKIP_TESTS=0
PKT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)  DRY_RUN=1; shift ;;
        --skip-tests) SKIP_TESTS=1; shift ;;
        --help|-h)  usage 0 ;;
        -*)         printf "Unknown flag: %s\n\n" "$1" >&2; usage 1 ;;
        *)          PKT="$1"; shift ;;
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

# ----- 4. Print the PR URL ----------------------------------------------

PR_URL=$(gh pr view "$CURRENT_BRANCH" --json url -q .url 2>/dev/null || echo "")
if [[ -n "$PR_URL" ]]; then
    printf "\n[ok] PR ready: %s\n" "$PR_URL"
else
    printf "\n[warn] Could not retrieve PR URL; check 'gh pr list'.\n" >&2
fi