#!/usr/bin/env bash
#
# bin/spiral-context.sh
#
# Context primer for Spiral Browser sessions. Prints the
# 5-10 files most relevant to picking up a given packet, so
# the next implementer (or a fresh LLM session) does not have
# to re-load the codebase into working memory from scratch.
#
# Usage:
#   bin/spiral-context.sh              # session start (no packet)
#   bin/spiral-context.sh <packet-id>  # packet context
#   bin/spiral-context.sh --quick      # only the always-relevant files
#   bin/spiral-context.sh --rules-check # full rules enforcement (nightly clippy + audits)
#
# Output is plain text suitable for piping to `less` or
# copy-pasting into a chat window. Exit code is always 0.
#
# Adopted 2026-06-17 per the implementer-agent ergonomics
# review. See `docs/active_context.md` for the rationale.

set -o pipefail

cd "$(dirname "$0")/.." || exit 1

# ----- 1. Always-relevant files (session start) ------------------------

ALWAYS_RELEVANT=(
    "AGENTS.md"
    "docs/active_context.md"
    "docs/implementation_tracker.md"
    "docs/progress_ledger.md"
    "docs/glossary.md"
    "docs/agents/implementer.md"
)

# ----- 2. Architecture / ADR lookup tables -----------------------------
#
# Map packet-id prefix to its primary architecture doc and
# the most relevant ledger / ADR for that subsystem.
#
# macOS ships bash 3.2 (no associative arrays), so we use
# a function that switches on the major version instead.

arch_for_major() {
    case "$1" in
        1) printf "docs/architecture/vortex.md" ;;  # Phase 1 = Vortex
        2) printf "docs/architecture/fmt.md"   ;;  # Phase 2 = Fmt (until other steps land)
        3) printf "docs/architecture/net.md"   ;;  # Phase 3 = Net
        4) printf ""                            ;;  # Phase 4 = architecture infra
        *) printf ""                            ;;
    esac
}

# ----- 3. Print a header -----------------------------------------------

hr() { printf '\n%s\n' "------------------------------------------------------------"; }

print_always_relevant() {
    hr
    printf "%s\n" "ALWAYS RELEVANT (read first, every session):"
    hr
    for f in "${ALWAYS_RELEVANT[@]}"; do
        if [[ -f "$f" ]]; then
            printf "  %s\n" "$f"
        fi
    done
}

# ----- 4. Packet-specific context --------------------------------------

print_packet_context() {
    local pkt="$1"

    hr
    printf "PACKET: %s\n" "$pkt"
    hr

    # Pull the packet line out of the tracker. The tracker uses
    # markdown bold: `- [ ] **Packet 2.1.2** — Title.` so we grep
    # for the package-prefixed marker.
    local tracker_line
    tracker_line=$(grep -m1 "Packet ${pkt}\*\*" docs/implementation_tracker.md 2>/dev/null || true)
    if [[ -n "$tracker_line" ]]; then
        printf "\nTracker line:\n  %s\n" "$tracker_line"
    else
        printf "\n[warn] Packet %s not found in implementation_tracker.md\n" "$pkt" >&2
    fi

    # Find the leading numeric prefix (e.g. "2.1" from "2.1.2") to look
    # up the Step header.
    local step_prefix
    step_prefix=$(printf "%s" "$pkt" | grep -oE '^[0-9]+\.[0-9]+' || true)
    if [[ -n "$step_prefix" ]]; then
        local step_line
        step_line=$(grep -m1 "^### Step ${step_prefix} " docs/implementation_tracker.md 2>/dev/null || true)
        if [[ -n "$step_line" ]]; then
            printf "\nStep header:\n  %s\n" "$step_line"
        fi
    fi

    # Map the leading major version to its crate's architecture doc.
    local major
    major=$(printf "%s" "$pkt" | cut -d. -f1)
    local pkg_arch
    pkg_arch=$(arch_for_major "$major")

    if [[ -n "$pkg_arch" && -f "$pkg_arch" ]]; then
        printf "\nArchitecture doc (read second):\n  %s\n" "$pkg_arch"
    fi

    # Pull the most recent ledger entry that mentions the packet.
    local ledger_ctx
    ledger_ctx=$(grep -B1 -A8 "Packet ${pkt} " docs/progress_ledger.md 2>/dev/null \
        | head -20 || true)
    if [[ -n "$ledger_ctx" ]]; then
        printf "\nRecent ledger context (most recent match):\n"
        printf "%s\n" "$ledger_ctx" | sed 's/^/  /'
    fi

    # If the packet is unchecked, surface the expansion block
    # (added by Suggestion 1) right after the tracker line.
    if [[ "$tracker_line" == *"[ ]"* ]]; then
        printf "\nExpansion (read this if present):\n"
        local in_block=0
        local line
        while IFS= read -r line; do
            if [[ $in_block -eq 0 ]] && [[ "$line" == *"Packet ${pkt}"*"**"* ]]; then
                in_block=1
                printf "  %s\n" "$line"
                continue
            fi
            if [[ $in_block -eq 1 ]]; then
                # End at the next top-level packet bullet or non-indented line.
                if [[ "$line" == "- [ ] **Packet "* ]] || [[ "$line" == "- [x] **Packet "* ]]; then
                    break
                fi
                # Skip the blank line that follows the bullet (cosmetic).
                [[ -z "$line" ]] && continue
                printf "  %s\n" "$line"
            fi
        done < docs/implementation_tracker.md
    fi
}

# ----- 5. Recent test files (heuristic) --------------------------------

print_recent_tests() {
    hr
    printf "RECENT TEST FILES (last 10 modified under crates/*/tests):\n"
    hr
    # macOS find has no -printf, so use stat. Linux works too.
    find crates -path '*/tests/*.rs' -type f -name '*.rs' 2>/dev/null \
        | while read -r f; do
            # GNU stat: -c %Y; macOS stat: -f %m
            mtime=$(stat -c %Y "$f" 2>/dev/null || stat -f %m "$f" 2>/dev/null || echo 0)
            printf "%s %s\n" "$mtime" "$f"
        done \
        | sort -rn \
        | head -10 \
        | awk '{ printf "  %s\n", $2 }' \
        || true
}

# ----- 6. Rules audit (fast scan, ~200ms) -------------------------------

run_rules_audit() {
    hr
    printf "RULES ENFORCEMENT AUDIT (fast scan):\n"
    hr

    local ok=0

    # 1. orphan exports — run silently, capture exit, show summary only.
    local _orphan_out
    _orphan_out=$(./scripts/audit-orphan-exports.sh 2>&1)
    local _orphan_rc=$?
    local _orphan_summary
    _orphan_summary=$(printf '%s\n' "$_orphan_out" | tail -3)
    printf "  audit-orphan-exports: "
    if [[ $_orphan_rc -eq 0 ]]; then
        printf "PASS\n"
    else
        printf "FAIL\n"
        printf '%s\n' "$_orphan_summary" | sed 's/^/    /'
        ok=1
    fi

    # 2. doc drift — show summary line only.
    local _drift_out
    _drift_out=$(./scripts/audit-doc-drift.sh 2>&1)
    local _drift_rc=$?
    printf "  audit-doc-drift:     "
    if [[ $_drift_rc -eq 0 ]]; then
        local _drift_summary
        _drift_summary=$(printf '%s\n' "$_drift_out" | tail -1)
        printf "PASS — %s\n" "$_drift_summary"
    else
        printf "FAIL\n"
        printf '%s\n' "$_drift_out" | tail -10 | sed 's/^/    /'
        ok=1
    fi

    # 3. tool coverage — every bin/scripts tool referenced in rules.
    local _tc_out
    _tc_out=$(./scripts/audit-orphan-exports.sh --tool-coverage 2>&1)
    local _tc_rc=$?
    printf "  tool-coverage:       "
    if [[ $_tc_rc -eq 0 ]]; then
        printf "PASS — every tool is referenced in .spiral/rules/\n"
    else
        printf "FAIL\n"
        printf '%s\n' "$_tc_out" | sed 's/^/    /'
        ok=1
    fi

    hr
    if [[ $ok -eq 0 ]]; then
        printf "RULES AUDIT: PASS (all enforcement gates green)\n"
    else
        printf "RULES AUDIT: ISSUES FOUND — review the FAIL lines above\n"
    fi
}

# ----- 7. Main ---------------------------------------------------------

case "${1:-}" in
    --quick|-q)
        print_always_relevant
        ;;
    --rules-check)
        print_always_relevant
        run_rules_audit
        printf "\nFor full rules enforcement including nightly clippy:\n  just verify-rules\n\n"
        ;;
    --help|-h)
        sed -n '2,34p' "$0"
        ;;
    "")
        print_always_relevant
        print_recent_tests
        printf "\nTo get packet-specific context:\n  %s <packet-id>\n" "$0"
        printf "Example: %s 2.1.2\n" "$0"
        printf "To run the rules audit:           %s --rules-check\n\n" "$0"
        ;;
    *)
        print_always_relevant
        print_packet_context "$1"
        print_recent_tests
        ;;
esac

printf "\n"