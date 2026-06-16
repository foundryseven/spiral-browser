#!/usr/bin/env bash
#
# scripts/audit-orphan-exports.sh
#
# Detect "orphan" public symbols in the workspace: items
# declared `pub` in a crate's `lib.rs` that are not
# imported anywhere outside that crate. A symbol that
# nobody imports is a wiring gap — the type or function
# is built but never used, which usually means the
# feature it backs is not actually wired into a real
# surface.
#
# Borrowed from the Zeus repo's
# `scripts/audit-orphan-exports.mjs` pattern (2026-06-16,
# full Tier 1+2+3 restructure).
#
# Usage:
#   ./scripts/audit-orphan-exports.sh            # audit all crates
#   ./scripts/audit-orphan-exports.sh spiral-fmt # audit one crate
#
# Output format (per crate, then a summary line):
#   <crate>          OK (N symbols, all wired)
#   <crate>          ORPHANS (K / N):
#                      - SymbolName
#                      - SymbolName
#   <crate>          (no candidates) OK
#   <crate>          (no lib.rs; skipped)
#
#   OK: 0 orphan exports across M crate(s) audited (P with public symbols).
#   FAIL: K orphan export(s) across M crate(s):
#     - <crate>
#     - <crate>
#
# Exit code:
#   0  no orphans (caller may claim "wired")
#   1  one or more orphans found (treat as build break per
#      AGENTS.md § Wiring & Integration)
#   2  usage error (bad crate name, missing tool)
#
# Implementation notes:
#   - Written for portability across macOS bash 3.2
#     (no `mapfile`, no `declare -a`).
#   - Prefers ripgrep (`rg`) if installed; falls back
#     to `grep -R` + `find` otherwise.
#   - Integration tests in `tests/`, examples in
#     `examples/`, and benchmarks in `benches/` count as
#     consumers (separate compilation units). Only the
#     lib's `src/` is excluded from the consumer search.

set -e

# -- arg parsing ----------------------------------------------------------

if [[ $# -gt 1 ]]; then
    echo "usage: $0 [crate-name]" >&2
    exit 2
fi

# -- locate repo root -----------------------------------------------------

# Walk up to the Cargo.toml that defines the workspace.
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ ! -f "$repo_root/Cargo.toml" ]]; then
    echo "error: cannot find workspace Cargo.toml above $repo_root" >&2
    exit 2
fi

cd "$repo_root"

crates_dir="crates"

# -- list of crates to audit ---------------------------------------------

all_crates=$(ls -1 "$crates_dir" 2>/dev/null | while read d; do
    if [[ -f "$crates_dir/$d/Cargo.toml" ]]; then
        echo "$d"
    fi
done)

if [[ -z "$all_crates" ]]; then
    echo "error: no crates found under $crates_dir" >&2
    exit 2
fi

if [[ $# -eq 1 ]]; then
    target="$1"
    if [[ ! -d "$crates_dir/$target" ]]; then
        echo "error: crate '$target' not found in $crates_dir" >&2
        exit 2
    fi
    crates="$target"
else
    crates="$all_crates"
fi

# -- per-crate audit ------------------------------------------------------

# A symbol is "orphan" when:
#   1. It is declared `pub` in the crate's lib.rs (or a
#      top-level re-export line `pub use …::Foo;`).
#   2. Its name does not appear in `use` statements or
#      fully-qualified paths (`crate_name::Foo`,
#      `spiral_x::Foo`) anywhere in the workspace
#      outside the crate itself.
#
# Tests in the crate itself don't count as a "consumer"
# for the purpose of this audit — a symbol that is only
# referenced in its own test file is still orphaned from
# the rest of the workspace.

total_orphans=0
failed_crates=""
ok_crates=0

for crate in $crates; do
    lib_rs="$crates_dir/$crate/src/lib.rs"
    if [[ ! -f "$lib_rs" ]]; then
        # No lib.rs (binary crate); skip with a note.
        printf "  %-22s (no lib.rs; skipped)\n" "$crate"
        continue
    fi

    # Collect candidate names: every `pub` declaration
    # in lib.rs. We look for `pub use …::NAME`,
    # `pub fn NAME`, `pub struct NAME`, `pub enum NAME`,
    # `pub trait NAME`, `pub type NAME`.
    #
    # We also handle multi-line `pub use { … }` blocks:
    # any line inside such a block that contains an
    # identifier is treated as a re-exported symbol.
    #
    # We deliberately exclude `pub mod NAME` (those are
    # module re-exports, not value-level symbols) and
    # `pub const NAME` (constants are not part of the
    # wiring surface this audit covers).
    #
    # Use awk for portability — it handles the
    # `pub … NAME;` pattern with the second token
    # being the keyword and the third the identifier.
    symbols=$(awk '
        BEGIN { in_use_block = 0 }

        # End of a `pub use { … }` block.
        in_use_block && /};?[[:space:]]*$/ {
            in_use_block = 0
            next
        }

        # Inside a `pub use { … }` block: capture
        # identifiers on each line.
        in_use_block {
            # Match capitalised identifiers (the things
            # being re-exported). Skip lines that are
            # `as Foo` aliases where we only want the
            # alias name on the right.
            n = split($0, a, /[[:space:]:,]+/)
            for (i = 1; i <= n; i++) {
                if (a[i] ~ /^[A-Z][A-Za-z0-9_]*$/) {
                    print a[i]
                }
            }
            next
        }

        # Start of a `pub use { … }` block.
        /^[[:space:]]*pub[[:space:]]+use[[:space:]]+[A-Za-z0-9_:]*(::)?\{/ {
            in_use_block = 1
            # Some re-exports have the form
            # `pub use foo::{Bar, Baz};` — no braces
            # block. Detect that case and emit the
            # inner identifiers in one pass.
            if ($0 ~ /\{.*\}/) {
                inner = $0
                sub(/^.*\{/, "", inner)
                sub(/\}.*$/, "", inner)
                n = split(inner, a, /[[:space:]:,]+/)
                for (i = 1; i <= n; i++) {
                    if (a[i] ~ /^[A-Z][A-Za-z0-9_]*$/) {
                        print a[i]
                    }
                }
                in_use_block = 0
            }
            next
        }

        # Single-line `pub use foo::Bar;` (or
        # `pub use foo::{Bar, Baz};`).
        /^[[:space:]]*pub[[:space:]]+use[[:space:]]+[A-Za-z0-9_:]*(::)?\{/ {
            inner = $0
            sub(/^.*\{/, "", inner)
            sub(/\}.*$/, "", inner)
            n = split(inner, a, /[[:space:]:,]+/)
            for (i = 1; i <= n; i++) {
                if (a[i] ~ /^[A-Z][A-Za-z0-9_]*$/) {
                    print a[i]
                }
            }
            next
        }

        # Single-line `pub use foo::Bar;` (no braces).
        /^[[:space:]]*pub[[:space:]]+use[[:space:]]+[A-Za-z0-9_:]+/ {
            # Strip `pub use …;`.
            line = $0
            sub(/^[[:space:]]*pub[[:space:]]+use[[:space:]]+/, "", line)
            sub(/[[:space:]]*;.*$/, "", line)
            # The last segment after `::` is the
            # re-exported name.
            n = split(line, a, "::")
            name = a[n]
            sub(/[[:space:];,].*/, "", name)
            # If the line is `foo as bar`, take `bar`.
            if (line ~ /[[:space:]]as[[:space:]]/) {
                m = split(line, b, /[[:space:]]+as[[:space:]]+/)
                name = b[m]
                sub(/[[:space:];,].*/, "", name)
            }
            if (name ~ /^[A-Z][A-Za-z0-9_]*$/) {
                print name
            }
            next
        }

        # `pub fn NAME`, `pub struct NAME`, etc.
        /^[[:space:]]*pub[[:space:]]+(fn|struct|enum|trait|type)[[:space:]]+[A-Z][A-Za-z0-9_]*/ {
            sub(/^[[:space:]]*pub[[:space:]]+(fn|struct|enum|trait|type)[[:space:]]+/, "")
            sub(/[[:space:];,<({:].*/, "")
            print
        }
    ' "$lib_rs" | sort -u)

    if [[ -z "$symbols" ]]; then
        printf "  %-22s (no candidates) OK\n" "$crate"
        continue
    fi

    n_symbols=$(printf '%s\n' "$symbols" | wc -l | tr -d ' ')

    orphans=""
    while IFS= read -r sym; do
        [[ -z "$sym" ]] && continue
        # Search the rest of the workspace for any
        # reference to this symbol. We exclude the
        # crate itself (we want to know if anything
        # OUTSIDE this crate uses it).
        #
        # The simplest check: a word-boundary grep
        # for the bare symbol name in any other
        # crate's .rs file. False positives are
        # possible (e.g. a symbol with the same
        # name in a different namespace) but rare
        # in practice; the goal is to flag obvious
        # orphans, not to be a perfect static
        # analyser.
        #
        # We exclude only the lib's `src/` directory —
        # the lib is the *declaration* site. Integration
        # tests in `tests/`, examples in `examples/`,
        # and benchmarks in `benches/` are *separate
        # compilation units* that consume the lib's
        # public surface, so they count as consumers.
        hits=$(find "$crates_dir" -name '*.rs' \
                 -not -path "$crates_dir/$crate/src/*" \
                 -exec grep -lw "$sym" {} + 2>/dev/null | wc -l | tr -d ' ')

        if [[ "${hits:-0}" -eq 0 ]]; then
            if [[ -z "$orphans" ]]; then
                orphans="$sym"
            else
                orphans="$orphans
$sym"
            fi
        fi
    done <<< "$symbols"

    if [[ -z "$orphans" ]]; then
        printf "  %-22s OK (%s symbols, all wired)\n" "$crate" "$n_symbols"
        ok_crates=$((ok_crates + 1))
    else
        n_orphans=$(printf '%s\n' "$orphans" | wc -l | tr -d ' ')
        printf "  %-22s ORPHANS (%s / %s):\n" "$crate" "$n_orphans" "$n_symbols"
        while IFS= read -r o; do
            [[ -n "$o" ]] && printf "      - %s\n" "$o"
        done <<< "$orphans"
        total_orphans=$((total_orphans + n_orphans))
        failed_crates="$failed_crates
$crate"
    fi
done

# -- summary -------------------------------------------------------------

echo
n_crates=$(printf '%s\n' "$crates" | wc -l | tr -d ' ')
if [[ $total_orphans -eq 0 ]]; then
    echo "OK: 0 orphan exports across $n_crates crate(s) audited ($ok_crates with public symbols)."
    exit 0
else
    n_failed=$(printf '%s\n' "$failed_crates" | sed '/^$/d' | wc -l | tr -d ' ')
    echo "FAIL: $total_orphans orphan export(s) across $n_failed crate(s):"
    printf '%s\n' "$failed_crates" | sed '/^$/d' | sed 's/^/  - /'
    exit 1
fi
