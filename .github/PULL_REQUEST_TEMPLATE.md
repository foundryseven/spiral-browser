## Summary

<!-- One or two sentences. -->

## Linked Issues

<!-- Use `Fixes #123`, `Closes #456`, or `Refs #789`. -->

## Crates Touched

<!-- e.g. spiral-core, spiral-ipc, spiral-render. -->

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Refactor
- [ ] Documentation
- [ ] Chore / tooling

## Checklist

- [ ] `cargo check --workspace` passes locally
- [ ] `cargo clippy --workspace -- -D warnings` passes locally
- [ ] `cargo test --workspace` passes locally
- [ ] `cargo fmt --check` passes locally
- [ ] New public APIs have unit tests in the same file under `#[cfg(test)] mod tests`
- [ ] `docs/active_context.md` updated if sprint state changed
- [ ] `docs/progress_ledger.md` entry appended
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
- [ ] Commit message follows `type(scope): description` (see `AGENTS.md`)
- [ ] No new clippy warnings introduced
- [ ] No `unwrap()` introduced in library code

## Security

- [ ] This change touches IPC, sandbox, network, crypto, or renderer input — extra review requested
- [ ] This change introduces no new panics, OOM surfaces, or unbounded allocations

## Reviewer Notes

<!-- Anything the reviewer should know: trade-offs, follow-ups, design alternatives rejected. -->
