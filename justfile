# Spiral Browser — Agent Verification Protocol
#
# Run `just verify` to execute the full pipeline:
#   1. Format check (rustfmt)
#   2. Lint check (clippy with deny warnings)
#   3. Full workspace test suite
#   4. Workspace build (smoke test)
#
# Any step failure halts the pipeline with a non-zero exit.
# Agent must report verbatim output from this script before
# marking any task complete.

# Default recipe: run the full verification pipeline.
default: verify

# Full verification pipeline.
verify:
    @echo "=== 1/4  cargo fmt --all -- --check ==="
    cargo fmt --all -- --check
    @echo "=== 2/4  cargo clippy ==="
    cargo clippy --workspace --all-targets -- -D warnings
    @echo "=== 3/4  cargo test ==="
    cargo test --workspace
    @echo "=== 4/4  cargo build ==="
    cargo build --workspace
    @echo "=== ALL GREEN ==="

# Format check only.
fmt-check:
    cargo fmt --all -- --check

# Lint check only.
clippy-workspace:
    cargo clippy --workspace --all-targets -- -D warnings

# Full workspace test suite only.
test-workspace:
    cargo test --workspace

# Smoke build only.
smoke-build:
    cargo build --workspace

# Run tests for a specific crate: just test-crate spiral-fmt
test-crate crate:
    cargo test -p {{crate}}

# Run tests matching a name: just test-name parse_simple_div
test-name name:
    cargo test --workspace {{name}}

# Fast iteration target: run tests for one crate WITHOUT
# building the full workspace. Saves 30-60 seconds during
# packet implementation. Falls back to test-crate semantics.
#
# Usage: just test-fast spiral-fmt [test_name_pattern]
test-fast crate pattern="":
    @echo "=== Fast test (crate={{crate}} pattern={{pattern}}) ==="
    cargo test -p {{crate}} {{pattern}}

# Run tests for one crate AND its direct reverse-dependents
# (other crates that import it). Useful when changing an API
# surface that fans out across the workspace.
#
# Usage: just test-with-deps spiral-dom
test-with-deps crate:
    @echo "=== Test {{crate}} + dependents ==="
    cargo test -p {{crate}}
    @echo "=== Reverse-deps for {{crate}}: ==="
    @cargo metadata --format-version 1 --no-deps 2>/dev/null | jq -r --arg target "{{crate}}" '[.packages[] | select(.dependencies | any(.name == $target)) | .name] | .[] | "  - " + .' 2>/dev/null || echo "(jq unavailable; reverse-dep scan skipped)"
    @echo "=== Re-running dependent tests: ==="
    @deps=$(cargo metadata --format-version 1 --no-deps 2>/dev/null | jq -r --arg target "{{crate}}" '[.packages[] | select(.dependencies | any(.name == $target)) | .name] | .[]' 2>/dev/null); \
    for dep in $deps; do \
        cargo test -p "$dep" || exit 1; \
    done

# Packet lifecycle helpers. These wrap common sub-commands
# so an implementer doesn't have to remember the exact flag
# combinations.
#
# Usage: just verify-packet spiral-fmt
verify-packet crate:
    @echo "=== Verify packet in {{crate}} ==="
    cargo fmt --all -- --check
    cargo clippy -p {{crate}} --all-targets -- -D warnings
    cargo test -p {{crate}}
    @bash scripts/audit-orphan-exports.sh {{crate}}

# Open the tracker and ledger in $EDITOR (or print them).
review-status:
    @echo "=== Active context ==="
    @cat docs/active_context.md | head -50
    @echo ""
    @echo "=== What needs picking (next up) ==="
    @awk '/^## What needs picking/,/^$/' docs/implementation_tracker.md | head -25

# Context primer wrapper. Same as bin/spiral-context.sh
# but callable via `just`. Args are forwarded.
#
# Usage: just context           (session start)
#        just context 2.1.2     (packet-specific context)
context packet="":
    @bash bin/spiral-context.sh {{packet}}

# Run the wiring & integration audit (orphan exports).
# Exit 0 = all wired. Exit 1 = orphans (treat as build break).
wiring:
    @echo "=== Wiring & Integration Audit ==="
    @bash scripts/audit-orphan-exports.sh

# Run cargo-audit (advisory database check).
audit:
    @echo "=== cargo audit ==="
    @command -v cargo-audit >/dev/null 2>&1 || cargo install --locked cargo-audit
    cargo audit

# Run cargo-deny (license + advisory + ban check).
deny:
    @echo "=== cargo deny check ==="
    @command -v cargo-deny >/dev/null 2>&1 || cargo install --locked cargo-deny
    cargo deny check

# Pre-release checklist.
# Runs the full verification pipeline + supply-chain + wiring.
# Use before tagging a release (see docs/agents/release.md).
release-check: verify wiring audit deny
    @echo "=== RELEASE CHECK PASSED ==="
