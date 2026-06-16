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
