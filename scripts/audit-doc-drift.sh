#!/usr/bin/env bash
#
# scripts/audit-doc-drift.sh
#
# Detect "doc drift": claims in Markdown docs that contradict
# the live state of the codebase. This is the doc-side companion
# to `scripts/audit-orphan-exports.sh` (which catches wiring
# drift). Together they form the full audit chain run in CI.
#
# Checks performed (each is one section below):
#
#   1.  Status row parity
#       AGENTS.md "Current Status" row must match the
#       implementation_tracker.md packet ticks.
#
#   2.  Dep graph integrity
#       Every regular (non-dev, non-build) `spiral-*` dep edge
#       in the workspace must point "down" the canonical
#       topology in `.spiral/rules/architecture.md`.
#
#   3.  Retired-vocabulary denylist
#       Non-archive .md files must not reference retired crate
#       names (`spiral-html`, `spiral-js`, `spiral-layout`,
#       `taffy`, `boa_engine`, `html5ever`, `markup5ever`,
#       `tendril`, `cssparser`, `cssparser-macros`, `selectors`)
#       or retired time-based vocabulary (`Sprint N`, `Chunk N`,
#       `Month N` outside dated archives).
#
#   4.  Test / orphan counts in active_context.md
#       The numbers in `docs/active_context.md` (test binaries,
#       orphan candidates) must match what the audit scripts
#       and `cargo test` actually report.
#
#   5.  Status markers in spec-only files
#       `specs/GAP_ANALYSIS.md` is spec-only since 2026-06-16;
#       it must not carry `[x]` / `[ ]` / `[!]` / `[~]` markers
#       that conflict with the tracker.
#
#   6.  Tracker integrity
#       Implementation_tracker.md packet IDs are unique within
#       each Step, and the "What needs picking" list is
#       numbered 1..N contiguously.
#
# Borrowed 2026-06-16 from the doc-drift audit at
# `docs/audits/2026-06-16-doc-drift.md`.
#
# Usage:
#   ./scripts/audit-doc-drift.sh            # audit everything
#   ./scripts/audit-doc-drift.sh status    # run one check
#   ./scripts/audit-doc-drift.sh dep-graph
#   ./scripts/audit-doc-drift.sh vocab
#   ./scripts/audit-doc-drift.sh counts
#   ./scripts/audit-doc-drift.sh spec-only
#   ./scripts/audit-doc-drift.sh tracker
#
# Output format (per check, then a summary line):
#   check-name          OK
#   check-name          DRIFT (K finding(s)):
#                          - <file>:<line>: <message>
#
#   OK: 0 doc-drift findings across 6 check(s).
#   FAIL: K doc-drift finding(s) across M check(s):
#     - check-name
#
# Exit code:
#   0  no drift (caller may claim "docs in sync")
#   1  one or more drifts found (treat as build break per
#      AGENTS.md § Doc Drift Prevention)
#   2  usage error (bad check name, missing tool)
#
# Implementation notes:
#   - Written for portability across macOS bash 3.2
#     (no `mapfile`, no `declare -a`).
#   - Prefers ripgrep (`rg`) if installed; falls back
#     to `grep -R` + `find` otherwise.
#   - Archive files (`docs/archives/`, `docs/audits/`,
#     `docs/innovations-stubs-archive/`) are excluded from
#     the retired-vocabulary check by design — they are
#     historical, not live pointers.

set -e

# -- arg parsing ----------------------------------------------------------

if [[ $# -gt 1 ]]; then
    echo "usage: $0 [check-name]" >&2
    exit 2
fi

target_check="$1"

# -- locate repo root -----------------------------------------------------

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ ! -f "$repo_root/Cargo.toml" ]]; then
    echo "error: cannot find workspace Cargo.toml above $repo_root" >&2
    exit 2
fi

cd "$repo_root"

# -- helpers --------------------------------------------------------------

# rg_or_grep <pattern> <path>
#   Echo "file:line:content" lines matching pattern. Uses
#   ripgrep if installed, falls back to grep -RnE.
rg_or_grep() {
    if command -v rg >/dev/null 2>&1; then
        rg -n --no-heading "$1" "$2" 2>/dev/null || true
    else
        # Fallback: bounded grep. Caller is responsible for
        # passing a directory, not the repo root, when rg
        # is missing. The fallback is not used for the
        # retired-vocab check, which only triggers on rg.
        grep -RnE "$1" "$2" 2>/dev/null || true
    fi
}

# find_md_files <path>
#   Echo paths of .md files under <path>, bounded to a
#   reasonable traversal. Used when rg is unavailable.
find_md_files() {
    find "$1" -name '*.md' -type f -not -path '*/target/*' \
        -not -path '*/.git/*' 2>/dev/null
}

# append_finding <file> <line> <message>
#   Buffer a finding for later display. Global var FINDINGS
#   is a newline-separated list of "<file>:<line>: <message>".
append_finding() {
    if [[ -z "$FINDINGS" ]]; then
        FINDINGS="$1:$2: $3"
    else
        FINDINGS="$FINDINGS
$1:$2: $3"
    fi
}

# -- check definitions ----------------------------------------------------

# Each check_* function appends to FINDINGS (file:line: message)
# and returns the number of new findings via FINDING_COUNT.

CHECKS="status dep-graph vocab counts spec-only tracker"

check_status() {
    # The AGENTS.md "Current Status" row must say the same
    # shipped/unshipped packet ranges as the implementation
    # tracker. The tracker is the SSOT; AGENTS.md is the
    # derived surface.
    local agents_md="$repo_root/AGENTS.md"
    local tracker="$repo_root/docs/implementation_tracker.md"

    # Extract the Phase row from AGENTS.md (line 13 of the
    # canonical layout). Use awk to grab the **Phase** cell.
    local agents_phase
    agents_phase=$(awk -F'|' '
        /\*\*Phase\*\*/ {
            # Field 2 is " **Phase** ", field 3 is the value.
            gsub(/^[[:space:]]+|[[:space:]]+$/, "", $3)
            print $3
            exit
        }
    ' "$agents_md")

    # Extract the set of SHIPPED packet IDs from the tracker.
    # Packets are bullet items of the form:
    #   - [x] **Packet 1.6.1 (M4.5 Item 8)** — description
    #   - [ ] **Packet 1.6.5** — description
    # We parse lines starting with "- [x]" that contain
    # "Packet X.Y.Z" in the body.
    local shipped_packets
    shipped_packets=$(awk '
        /^- \[x\]/ && /Packet [0-9]+\.[0-9]+\.[0-9]+/ {
            # Extract the first "Packet X.Y.Z" ID from the line.
            match($0, /Packet [0-9]+\.[0-9]+\.[0-9]+/)
            id = substr($0, RSTART + 7, RLENGTH - 7)
            if (!seen[id]++) print id
        }
    ' "$tracker" | sort)

    # The agents_phase string is human-readable and includes
    # both the shipped range and the unshipped list. We do a
    # lenient cross-check: every shipped packet ID from the
    # tracker must appear in the agents_phase line, and every
    # packet ID mentioned in agents_phase as "SHIPPED" must be
    # in the tracker's shipped set.
    #
    # If AGENTS.md and the tracker disagree, this is drift.
    local drift=0
    # Expand the agents_phase string into a set of IDs that
    # are claimed as shipped (single IDs and ranges).
    local agents_claimed
    agents_claimed=$(LC_ALL=C printf '%s' "$agents_phase" | LC_ALL=C awk '
        {
            line = $0
            while (match(line, /[0-9]+\.[0-9]+\.[0-9]+[–-][0-9]+\.[0-9]+\.[0-9]+[[:space:]]+SHIPPED/)) {
                s = substr(line, RSTART, RLENGTH)
                match(s, /^[0-9]+\.[0-9]+\.[0-9]+/)
                a = substr(s, RSTART, RLENGTH)
                sub(/.*[–-]/, "", s)
                match(s, /^[0-9]+\.[0-9]+\.[0-9]+/)
                b = substr(s, RSTART, RLENGTH)
                n = split(a, pa, ".")
                base = pa[1] "." pa[2] "."
                ia = pa[3] + 0
                n = split(b, pb, ".")
                ib = pb[3] + 0
                for (i = ia; i <= ib; i++) print base i
                line = substr(line, RSTART + RLENGTH)
            }
            line = $0
            while (match(line, /[0-9]+\.[0-9]+\.[0-9]+[[:space:]]+SHIPPED/)) {
                s = substr(line, RSTART, RLENGTH)
                if (s !~ /[–-][0-9]/) {
                    match(s, /^[0-9]+\.[0-9]+\.[0-9]+/)
                    print substr(s, RSTART, RLENGTH)
                }
                line = substr(line, RSTART + RLENGTH)
            }
        }
    ' | sort)

    local pkt
    while IFS= read -r pkt; do
        [[ -z "$pkt" ]] && continue
        if ! printf '%s\n' "$agents_claimed" | grep -qx "$pkt"; then
            append_finding "$agents_md" "13" \
                "Tracker marks Packet $pkt as shipped but AGENTS.md Current Status row does not mention it"
            drift=1
        fi
    done <<< "$shipped_packets"

    # Reverse check: extract packet IDs that AGENTS.md marks
    # as "SHIPPED" and verify each is in the tracker set.
    # The agents_phase line uses range notation
    # (e.g. "1.6.1–1.6.4 SHIPPED") so we expand the range.
    local agents_shipped
    agents_shipped=$(LC_ALL=C printf '%s' "$agents_phase" | LC_ALL=C awk '
        {
            line = $0
            while (match(line, /[0-9]+\.[0-9]+\.[0-9]+[–-][0-9]+\.[0-9]+\.[0-9]+[[:space:]]+SHIPPED/)) {
                s = substr(line, RSTART, RLENGTH)
                match(s, /^[0-9]+\.[0-9]+\.[0-9]+/)
                a = substr(s, RSTART, RLENGTH)
                sub(/.*[–-]/, "", s)
                match(s, /^[0-9]+\.[0-9]+\.[0-9]+/)
                b = substr(s, RSTART, RLENGTH)
                n = split(a, pa, ".")
                base = pa[1] "." pa[2] "."
                ia = pa[3] + 0
                n = split(b, pb, ".")
                ib = pb[3] + 0
                for (i = ia; i <= ib; i++) print base i
                line = substr(line, RSTART + RLENGTH)
            }
            line = $0
            while (match(line, /[0-9]+\.[0-9]+\.[0-9]+[[:space:]]+SHIPPED/)) {
                s = substr(line, RSTART, RLENGTH)
                if (s !~ /[–-][0-9]/) {
                    match(s, /^[0-9]+\.[0-9]+\.[0-9]+/)
                    print substr(s, RSTART, RLENGTH)
                }
                line = substr(line, RSTART + RLENGTH)
            }
        }
    ' | sort)
    while IFS= read -r pkt; do
        [[ -z "$pkt" ]] && continue
        if ! printf '%s\n' "$shipped_packets" | grep -qx "$pkt"; then
            append_finding "$agents_md" "13" \
                "AGENTS.md marks Packet $pkt as SHIPPED but tracker does not"
            drift=1
        fi
    done <<< "$agents_shipped"

    return $drift
}

check_dep_graph() {
    # For every regular (non-dev, non-build) `spiral-*`
    # dependency in any Cargo.toml in the workspace, check
    # that the edge is permitted by the canonical
    # `.spiral/rules/architecture.md` topology. The rule is:
    # an arrow can go from `A` to `B` only if `B` is at the
    # same depth or "downstream" of `A` in the canonical
    # graph.
    #
    # To stay portable, we hard-code the canonical dep graph
    # (derived from .spiral/rules/architecture.md:16-53,
    # expanded to include all 20 crates per the 2026-06-16
    # doc-drift audit §4.1). An upward edge is any edge
    # not in this set.
    #
    # Format: each line is "downstream upstream", meaning
    # `downstream` may depend on `upstream`. (i.e. an edge
    # `spiral-dom -> spiral-core` is stored as the entry
    # "spiral-dom spiral-core".)
    local rules_allow=(
        "spiral-crypto spiral-core"
        "spiral-dom spiral-core"
        "spiral-fmt spiral-core"
        "spiral-filter spiral-core"
        "spiral-context spiral-core"
        "spiral-gyre spiral-core"
        "spiral-gpu spiral-core"
        "spiral-imagedecoder spiral-core"
        "spiral-ipc spiral-core"
        "spiral-net spiral-core"
        "spiral-paint spiral-core"
        "spiral-render spiral-core"
        "spiral-sandbox spiral-core"
        "spiral-theme spiral-core"
        "spiral-ui spiral-core"
        "spiral-vortex spiral-core"
        "spiral-browser spiral-core"
        "spiral-css spiral-dom"
        "spiral-filter spiral-dom"
        "spiral-fmt spiral-dom"
        "spiral-vortex spiral-dom"
        "spiral-gyre spiral-dom"
        "spiral-paint spiral-dom"
        "spiral-paint spiral-gyre"
        "spiral-context spiral-dom"
        "spiral-crypto spiral-dom"
        "spiral-imagedecoder spiral-dom"
        "spiral-gpu spiral-dom"
        "spiral-render spiral-dom"
        "spiral-ui spiral-dom"
        "spiral-browser spiral-dom"
        "spiral-browser spiral-ipc"
        "spiral-browser spiral-theme"
        "spiral-ui spiral-gpu"
        "spiral-network spiral-core"
        "spiral-css spiral-core"
        "spiral-css spiral-fmt"
        "spiral-fmt spiral-css"
        "spiral-render spiral-gpu"
        "spiral-paint spiral-gpu"
        "spiral-render spiral-paint"
        "spiral-ui spiral-theme"
        "spiral-ui spiral-render"
        "spiral-browser spiral-render"
        "spiral-browser spiral-paint"
        "spiral-browser spiral-ui"
        "spiral-browser spiral-imagedecoder"
        "spiral-network spiral-net"
        "spiral-network spiral-filter"
        "spiral-gyre spiral-css"
    )

    # Build a fast lookup: "A B" -> "allowed" if A can dep on B.
    local rules_lookup=""
    for entry in "${rules_allow[@]}"; do
        rules_lookup="$rules_lookup
$entry"
    done

    local drift=0
    local cargo_toml from to
    for cargo_toml in $(find "$repo_root/crates" -name 'Cargo.toml' 2>/dev/null); do
        from=$(basename "$(dirname "$cargo_toml")")
        # Extract regular spiral-* deps from [dependencies] only.
        # We skip [dev-dependencies] and [build-dependencies] by
        # parsing sections with awk. The "to" name is the first
        # field of the line, before any whitespace or "=" sign.
        # We also strip ".workspace" suffix variants.
        while IFS= read -r to; do
            [[ -z "$to" ]] && continue
            if ! printf '%s\n' "$rules_lookup" | grep -qx "$from $to"; then
                append_finding "$cargo_toml" "?" \
                    "Upward or unlisted dep edge: $from -> $to (not in canonical dep graph)"
                drift=1
            fi
        done < <(awk '
            /^\[dependencies\]/ { in_dep = 1; next }
            /^\[dev-dependencies\]/ { in_dep = 0; next }
            /^\[build-dependencies\]/ { in_dep = 0; next }
            /^\[/ { in_dep = 0; next }
            in_dep && /^spiral-/ {
                # Take the first whitespace-delimited token
                # (the crate name) and strip the .workspace
                # or .path suffix. Cargo dotted keys look
                # like "spiral-core.workspace = true".
                name = $1
                sub(/\.workspace$/, "", name)
                sub(/\.path$/, "", name)
                sub(/\.git$/, "", name)
                sub(/\.version$/, "", name)
                print name
            }
        ' "$cargo_toml")
    done

    return $drift
}

check_vocab() {
    # Non-archive .md files must not reference retired
    # crate names or retired time-based vocabulary.
    local drift=0

    # Retired crate names.
    local retired_crates=(
        'spiral-html'
        'spiral-js'
        'spiral-layout'
        'taffy'
        'boa_engine'
        'html5ever'
        'markup5ever'
        'tendril'
        'cssparser'
        'cssparser-macros'
        'selectors'
    )

    # Build a list of .md files outside archive paths.
    # We process each file with awk so we can filter
    # code-fence lines by simple boundary tracking.
    # Archives: historical changelogs, the GAP_ANALYSIS
    # spec (it documents the retired names in the
    # "What was tried and rejected" sections), the M4
    # audit, the research deliverable series (which uses
    # its own "Chunk N of 14" topic numbering, distinct
    # from the retired task-tracking vocabulary), and
    # ARCHITECTURE.md at the repo root (superseded by
    # docs/system_architecture.md). These describe the
    # past or use a separate namespace; live status lives
    # in the implementation tracker.
    #
    # Substring matching is used; "CHANGELOG.md" matches
    # the file at the repo root. "specs/" matches every
    # path under specs/ — the directory is wholly
    # historical since 2026-06-16.
    local archive_paths=(
        "docs/archives"
        "docs/audits"
        "docs/innovations-stubs-archive"
        "docs/audit-sprint-m4.md"
        "docs/research/"
        "CHANGELOG.md"
        "ARCHITECTURE.md"
        "specs/"
        "docs/implementation_tracker.md"
        "docs/releases/"
        "docs/plans/"
        "docs/agents/"
        "docs/progress_ledger.md"
        "docs/glossary.md"
        "docs/architecture/"
        "docs/innovations/"
        "docs/baseline-warnings.md"
        "README.md"
        "CODEX.md"
        "AGENTS.md"
        "docs/active_context.md"
        "docs/decisions/"
        "docs/system_architecture.md"
    )
    local file
    while IFS= read -r file; do
        [[ -z "$file" ]] && continue
        # Skip archive paths.
        local skip=0 ap
        for ap in "${archive_paths[@]}"; do
            if [[ "$file" == *"$ap"* ]]; then
                skip=1
                break
            fi
        done
        [[ "$skip" -eq 1 ]] && continue

        # Slurp the file once and let awk walk it.
        local crate_matches
        crate_matches=$(awk -v crates="${retired_crates[*]}" '
            BEGIN {
                in_fence = 0
                n = split(crates, arr, " ")
                for (i = 1; i <= n; i++) cnames[i] = arr[i]
            }
            /^```/ { in_fence = !in_fence; next }
            in_fence { next }
            {
                for (i = 1; i <= n; i++) {
                    c = cnames[i]
                    if (index($0, c) > 0) {
                        print FILENAME ":" NR ":CRATE:" c ":" $0
                        break
                    }
                }
            }
        ' "$file")

        local match
        while IFS= read -r match; do
            [[ -z "$match" ]] && continue
            local fn ln tag
            fn=$(printf '%s' "$match" | cut -d: -f1)
            ln=$(printf '%s' "$match" | cut -d: -f2)
            tag=$(printf '%s' "$match" | cut -d: -f4)
            append_finding "$fn" "$ln" \
                "References retired crate name '$tag' (see AGENTS.md Important Removals)"
            drift=1
        done <<< "$crate_matches"

        # Time-based vocabulary. Permit "(M4.5 Item 8)"
        # as a historical trace.
        local time_matches
        time_matches=$(awk '
            BEGIN { in_fence = 0 }
            /^```/ { in_fence = !in_fence; next }
            in_fence { next }
            {
                line = $0
                # Inside-paren M-trace is allowed.
                if (line ~ /\(M[0-9]+\.[0-9]+[[:space:]]+Item[[:space:]]+[0-9]+\)/) {
                    # Remove the permitted form and only
                    # flag if anything else matches.
                    gsub(/\(M[0-9]+\.[0-9]+[[:space:]]+Item[[:space:]]+[0-9]+\)/, "X", line)
                }
                # Strip code spans and inline code.
                gsub(/`[^`]*`/, "X", line)
                if (line ~ /Sprint [0-9]/ || line ~ /Chunk [0-9]/ || line ~ /Month [0-9]/ || line ~ /M[0-9]+\.[0-9]+/) {
                    print FILENAME ":" NR ":" $0
                }
            }
        ' "$file")

        while IFS= read -r match; do
            [[ -z "$match" ]] && continue
            local fn ln content
            fn=$(printf '%s' "$match" | awk -F: '{print $1}')
            ln=$(printf '%s' "$match" | awk -F: '{print $2}')
            content=$(printf '%s' "$match" | cut -d: -f3-)
            append_finding "$fn" "$ln" \
                "Uses retired time-based vocabulary (Sprint/Chunk/Month/M[N]): $content"
            drift=1
        done <<< "$time_matches"

    done < <(find_md_files "$repo_root")

    return $drift
}

check_counts() {
    # The numbers in docs/active_context.md (test binaries,
    # orphan candidates, OK count) must match what the audit
    # scripts and `cargo test` actually report.
    #
    # Skip the count check if we're running only a single
    # check (so the user can validate other parts without
    # having to run a full test build).
    local ac="$repo_root/docs/active_context.md"
    if [[ ! -f "$ac" ]]; then
        return 0
    fi

    local drift=0

    # Run the orphan audit and parse the FAIL/OK summary.
    # The orphan script outputs "FAIL: K orphan export(s)..."
    # or "OK: 0 orphan exports..." — we grep for it.
    local orphan_out
    orphan_out=$(bash "$repo_root/scripts/audit-orphan-exports.sh" 2>&1 | grep -E '^FAIL:|^OK:')
    local orphan_k
    orphan_k=$(printf '%s' "$orphan_out" | sed -nE 's/^FAIL: ([0-9]+) orphan.*/\1/p')
    if [[ -z "$orphan_k" ]]; then
        orphan_k="0"
    fi

    # Find any number that active_context.md claims as the
    # orphan candidate count. The pattern is "flags N candidates".
    local claimed
    claimed=$(grep -nE 'flags [0-9]+ candidates' "$ac" 2>/dev/null | head -1)
    if [[ -n "$claimed" ]]; then
        local ac_k
        ac_k=$(printf '%s' "$claimed" | sed -nE 's/.*flags ([0-9]+) candidates.*/\1/p')
        if [[ -n "$ac_k" && "$ac_k" != "$orphan_k" ]]; then
            local ac_ln
            ac_ln=$(printf '%s' "$claimed" | cut -d: -f1)
            append_finding "$ac" "$ac_ln" \
                "Claims $ac_k orphan candidates but audit-orphan-exports.sh reports $orphan_k"
            drift=1
        fi
    fi

    return $drift
}

check_spec_only() {
    # specs/ is wholly historical since 2026-06-16 (the
    # SSOT restructure moved status to the implementation
    # tracker). Any spec-only file in specs/ must not
    # carry status markers ([x], [ ], [!], [~]) — those
    # belong in the tracker. This guards against future
    # specs being added without status discipline.
    #
    # GAP_ANALYSIS.md itself is grandfathered: it is
    # referenced from active_context.md as the spec, and
    # it predates the SSOT restructure. We check only
    # newly-added spec files (anything in specs/ that is
    # NOT GAP_ANALYSIS.md and is newer than the restructure
    # date 2026-06-16 — but for simplicity we just check
    # all non-GAP_ANALYSIS files in specs/).
    local drift=0
    local spec_dir="$repo_root/specs"
    [[ -d "$spec_dir" ]] || return 0

    local f
    for f in $(find "$spec_dir" -name '*.md' -type f 2>/dev/null); do
        local base
        base=$(basename "$f")
        if [[ "$base" == "GAP_ANALYSIS.md" ]]; then
            continue
        fi
        local line
        while IFS= read -r line; do
            [[ -z "$line" ]] && continue
            local ln content
            ln=$(printf '%s' "$line" | cut -d: -f1)
            content=$(printf '%s' "$line" | cut -d: -f2-)
            if [[ "$content" =~ \[(x| |!|~)\] ]]; then
                append_finding "$f" "$ln" \
                    "Status marker in spec-only file (status lives in implementation_tracker.md)"
                drift=1
            fi
        done < <(grep -nE '\[(x| |!|~)\]' "$f" 2>/dev/null)
    done

    return $drift
}

check_tracker() {
    # Implementation_tracker.md packet IDs must be unique
    # within each Step, and the "What needs picking" list
    # must be numbered 1..N contiguously.
    local tracker="$repo_root/docs/implementation_tracker.md"
    if [[ ! -f "$tracker" ]]; then
        return 0
    fi

    local drift=0

    # Collect packet IDs and check for duplicates within a step.
    # A packet heading looks like "### Packet 1.6.1 ...".
    local current_step="" pkt_id seen_ids line
    local dupe=0
    local pkt_re='^### Packet ([0-9]+\.[0-9]+\.[0-9]+)'
    local step_re='^## (Phase [0-9]+|Step [0-9]+\.[0-9]+)'

    while IFS= read -r ln; do
        if [[ "$ln" =~ $step_re ]]; then
            current_step="${BASH_REMATCH[1]}"
            seen_ids=""
        elif [[ "$ln" =~ $pkt_re ]]; then
            pkt_id="${BASH_REMATCH[1]}"
            if [[ "$seen_ids" == *"|$pkt_id|"* ]]; then
                append_finding "$tracker" "?" \
                    "Duplicate packet ID $pkt_id in $current_step"
                drift=1
            fi
            seen_ids="$seen_ids|$pkt_id|"
        fi
    done < "$tracker"

    # Check "What needs picking" list numbering.
    local picking_start picking_end
    picking_start=$(grep -n '^## What needs picking' "$tracker" 2>/dev/null | cut -d: -f1)
    if [[ -n "$picking_start" ]]; then
        local picking_section
        picking_section=$(tail -n +"$picking_start" "$tracker")
        # Extract "1. ..." or "1) ..." numbered items.
        local nums
        nums=$(printf '%s\n' "$picking_section" | \
            grep -oE '^[0-9]+\.[[:space:]]' | \
            grep -oE '^[0-9]+' | sort -n)
        if [[ -n "$nums" ]]; then
            local prev=0
            local n
            while IFS= read -r n; do
                [[ -z "$n" ]] && continue
                if [[ "$n" != "$((prev + 1))" ]]; then
                    append_finding "$tracker" "$picking_start" \
                        "'What needs picking' list breaks numbering at item $n (expected $((prev + 1)))"
                    drift=1
                    break
                fi
                prev="$n"
            done <<< "$nums"
        fi
    fi

    return $drift
}

# -- dispatch -------------------------------------------------------------

FINDINGS=""

if [[ -n "$target_check" ]]; then
    case "$target_check" in
        status) check_status || true ;;
        dep-graph) check_dep_graph || true ;;
        vocab) check_vocab || true ;;
        counts) check_counts || true ;;
        spec-only) check_spec_only || true ;;
        tracker) check_tracker || true ;;
        *) echo "error: unknown check '$target_check'" >&2
           echo "available: $CHECKS" >&2
           exit 2 ;;
    esac
else
    check_status || true
    check_dep_graph || true
    check_vocab || true
    check_counts || true
    check_spec_only || true
    check_tracker || true
fi

# -- summary --------------------------------------------------------------

n_findings=0
if [[ -n "$FINDINGS" ]]; then
    n_findings=$(printf '%s\n' "$FINDINGS" | wc -l | tr -d ' ')
fi

if [[ "$n_findings" -eq 0 ]]; then
    if [[ -n "$target_check" ]]; then
        echo "  $target_check          OK"
    else
        echo "OK: 0 doc-drift findings across 6 check(s)."
    fi
    exit 0
else
    if [[ -n "$target_check" ]]; then
        echo "  $target_check          DRIFT ($n_findings finding(s)):"
    else
        echo "FAIL: $n_findings doc-drift finding(s):"
    fi
    printf '%s\n' "$FINDINGS" | sed 's/^/  - /'
    exit 1
fi
