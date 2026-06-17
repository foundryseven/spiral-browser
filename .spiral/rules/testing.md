---
paths:
  - "crates/**"
---

# Testing Standards

## Test-Driven Flow (TDFlow)

When implementing a new feature, layout block, parser, or
background service, the test is co-generated:

1. **Write the failing test first.** The test must assert real
   behaviour, not a placeholder.
2. **Run the test** and confirm it fails for the right reason
   (the API does not exist yet, or the assertion is unmet).
3. **Write the minimum code to make the test pass.**
4. **Run the test again.** Confirm green.
5. **Refactor** while keeping the test green.
6. **Add more tests** for edge cases the first test missed.

If a test passes without any code change, the test is hollow
and must be rewritten.

## Iteration speed

During a single packet's TDFlow loop, do NOT run the full
`cargo test --workspace` after every test edit. It costs
30-60 seconds per run and most of the work is in one crate.

Use `just test-fast <crate> [pattern]` for in-cycle work:

```bash
just test-fast spiral-fmt                  # all tests in one crate
just test-fast spiral-fmt parse_fragment   # filtered by test name
```

For API-surface changes that fan out across reverse-dependencies,
use `just test-with-deps <crate>` — it computes the
reverse-dep set via `cargo metadata | jq` and runs each.

The full `cargo test --workspace` is reserved for pre-commit
verification and is enforced by `just verify` and `bin/spiral-pr.sh`.

## Test layout

- **Unit tests** in the same file as the implementation, in a
  `#[cfg(test)] mod tests` block at the bottom. Use
  `#[test]`, `#[test_case(...)]`, and `#[should_panic]`.
- **Integration tests** in `tests/<crate>_surface.rs` at the
  crate root. These exercise the public surface from outside
  the crate.
- **Fuzz harnesses** in `fuzz/` per crate (when applicable).
- **WPT** at the workspace root under `tests/wpt/` (see WPT integration blueprint).
- **Benchmarks** in `benches/` (see performance standards).

## Test naming

- Function names: `test_<unit>_<behaviour>` or
  `test_<behaviour>` for trivial cases.
- One `#[test]` per assertion concept. Don't bundle
  "it parses and renders and serialises" into one test.

## Test coverage

- Every `pub` function in `lib.rs` must have at least one
  external test (unit or integration).
- Branch coverage target: ≥80% on core modules
  (`spiral-fmt`, `spiral-vortex`, `spiral-dom`).
- The audit script (`./scripts/audit-orphan-exports.sh`) is
  the wiring gate: a `pub` symbol with no external consumer
  is an orphan and a test gap. See
  [`docs/implementation_tracker.md`](../docs/implementation_tracker.md)
  § Wiring & Integration Rule.

## Isolation

- No real network calls in unit or integration tests. Use
  a `MockTransport` or a `wiremock`-style local fixture.
- No real filesystem writes. Use `tempfile::TempDir` for
  filesystem tests.
- No real time. Use `tokio::time::pause()` and advance
  manually. Or inject a `Clock` trait.
- No shared state between tests. Each test is its own
  universe.

## Test data

- Synthetic fixtures live next to the test in `tests/fixtures/`
  or in the test file itself for small data.
- Real-world data (HTML, CSS, JS corpora) lives in
  `tests/wpt/`, `tests/html5lib-tests/`, or similar.
- Never commit secrets, real user data, or production keys
  to a test fixture.

## Advanced Verification (Miri, Sanitizers)

For performance-critical or security-sensitive crates (such as `spiral-vortex` and `spiral-sandbox`):
* **Miri:** Run `cargo miri test` to check for Undefined Behaviour (UB), memory leaks, and pointer alignment issues in unsafe paths.
* **Sanitizers:** Compile and run the test suite with AddressSanitizer (ASan) or ThreadSanitizer (TSan) enabled locally to catch data races and memory corruption:
  ```bash
  RUSTFLAGS="-Zsanitizer=address" cargo test --target x86_64-apple-darwin
  ```
  *(Note: Adjust target for your local platform as required; requires nightly compiler).*

## What NOT to do

- `assert!(true)` — hollow.
- `#[ignore]` without a reason. If the test is broken, fix it.
- `tokio::time::sleep` for synchronisation. Use channels
  or `tokio::time::pause` + advance.
- `unwrap()` in test setup. Use `?` and `anyhow::Result`.
- Test names that don't describe behaviour
  (`test_1`, `test_2`, `test_it_works`).

## Test posturing verification

Run before claiming "tests pass":

```bash
cargo test --workspace
cargo test --doc --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

For crates containing `unsafe` code:
```bash
cargo miri test -p <crate_name>
```

The CI runs all three standard checks. The "verification protocol" in
[`docs/agents/implementer.md`](../docs/agents/implementer.md)
§ Verification Protocol is the ground truth.

Borrowed 2026-06-16 from the Zeus repo's `.zeus/rules/testing.md`.
