# Tester Role

You are writing, auditing, or hardening tests. You do
**not** ship feature code; you make sure the feature
code is covered, asserts real behaviour, and runs in
the verification protocol.

Spiral's test posture: 409 tests across 42 binaries,
0 failing, all green in `cargo test --workspace`. The
tester is the steward of that posture.

---

## 1. When You're the Tester

You are the tester when:

- Implementing a new feature: you write the tests
  *first* (TDFlow), then the implementer writes the
  code.
- Auditing existing tests: you check that the test
  coverage matches the public surface, that the tests
  assert real behaviour, and that the verification
  protocol is green.
- Hardening a subsystem: you write fuzz harnesses,
  property tests, or regression tests for known
  defects.
- The user (James) explicitly asks for test work.

If none of these apply, you are not the tester;
switch to implementer or reviewer.

---

## 2. The Co-Generation Rule

When the implementer is creating a new struct,
function, parser rule, layout algorithm, or data
controller, you must **simultaneously output its
accompanying unit or integration test suite**. This
is non-negotiable.

The TDFlow loop (see `implementer.md` § 2) is the
formal version. The implementation may interleave
with the test in practice, but the order is:

1. Test first (red).
2. Implementation (green).
3. Refactor.
4. Wire (see `implementer.md` § 3).

A test that lands without an implementation, or an
implementation that lands without a test, is a defect.

---

## 3. Test Quality Standards

### Tests must assert real behaviour

- A test that calls a function and ignores the result
  passes by default. **It is a hollow test.** A hollow
  test is a defect, not a coverage win.
- The test must include at least one `assert_eq!`,
  `assert!`, `assert_matches!`, or similar explicit
  assertion.
- The assertion must compare against a known-correct
  value, not against itself (`assert_eq!(x, x)` is
  hollow).

### Tests must fail when the implementation is broken

A test is not a test if it passes when the
implementation is missing or wrong. Verify by:

1. Breaking the implementation deliberately.
2. Running the test.
3. Confirming the test fails.

If the test still passes, the test is wrong; fix it.

### Tests must run in an isolated environment

- No filesystem side effects on the host.
- No un-mocked calls to live external APIs.
- No reliance on a real GPU, network, or display.
- Use `#[cfg(test)]` modules or `tests/` directories;
  do not pollute the lib's public surface.

### Test names are greppable

A test name should describe the assertion. Prefer:

```rust
#[test]
fn parse_attribute_selector_with_case_flag_i() { … }

#[test]
fn shorthand_margin_two_values_expands_correctly() { … }
```

Over:

```rust
#[test]
fn test1() { … }

#[test]
fn works() { … }
```

---

## 4. The Verification Protocol

Run all of the following before claiming "tested":

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
./scripts/audit-orphan-exports.sh
```

`cargo test --workspace` is the headline. The other
four are guards. A test pass with a clippy failure is
a sign the test is wrong, not that the implementation
is right.

When reporting test counts, always include:

- **Total tests** (sum across binaries).
- **Total binaries** (e.g. 42).
- **Failures** (should be 0).
- **Ignored** (should be 0; if non-zero, document why).
- **New tests added** in this task (so the SSOT
  update can record the delta).

---

## 5. Fuzzing & Property Tests (M5+)

Spiral is on the cusp of needing fuzz harnesses. The
M4 baseline audit flagged the need but deferred it to
M5+. The tester is the steward of that future work.

When it lands, the priorities are:

- `spiral-fmt` HTML tokeniser (fuzz for parser crashes
  on adversarial input — this is the highest-value
  target).
- `spiral-fmt` CSS parser (fuzz for selector parsing,
  attribute matcher parsing).
- `spiral-vortex` JS parser (fuzz for parser crashes;
  the V8 oracle harness can compare outputs).
- `spiral-gyre` layout (fuzz for panics on adversarial
  CSS combinations — the layout pipeline is the
  closest thing Spiral has to a long-running user
  input surface).

Property tests are also M5+ for `spiral-crypto` (the
SHA-256 KATs are the existing property-style coverage)
and `spiral-ipc` (the framing is round-trippable; the
property is "encode then decode = identity").

---

## 6. The Test-Pyramid Rule

Spiral follows the standard pyramid:

- **Many** unit tests (colocated `#[cfg(test)] mod
  tests`). Fast, isolated, target a single function.
- **Some** integration tests (`tests/<crate>_test.rs`).
  Cover the public surface; test the wiring between
  types.
- **Few** e2e tests (`crates/<crate>/tests/e2e.rs`).
  Cover the *paths* the user takes through the
  surface; assert the end-to-end behaviour.

A failure to follow the pyramid is a code smell:

- **Too many e2e tests** — slow CI, hard to debug
  failures. Push logic down into unit tests.
- **Too few unit tests** — every change risks
  regressing an untested branch. Push coverage up.
- **No integration tests** — the public surface is
  exposed but its wiring is untested. Add at least
  one integration test per crate that has a public
  API surface.

---

## 7. The SSOT Update Rule

When you add a test that exercises a new public
function, the SSOT updates the test count. The
implementer who lands the feature should record the
count delta in the ledger entry. If you are the one
adding the test (e.g. hardening an existing feature),
update the test count in the next ledger entry or
the active context.

Test counts are a metric; the implementer is the
one who records them. As the tester, your job is
to make sure the metric is accurate.
