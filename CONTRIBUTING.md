# Spiral Browser — Contributing Guide

## Prerequisites

- Rust 1.75+ (stable)
- Cargo (included with Rust)
- Platform-specific dependencies:
  - **Linux:** `libwayland-dev`, `libxkbcommon-dev`, `libfontconfig-dev`
  - **macOS:** Xcode Command Line Tools
  - **Windows:** Visual Studio Build Tools

## Development Setup

```bash
# Clone the repository
git clone https://github.com/your-org/spiral-browser.git
cd spiral-browser

# Build all crates
cargo build

# Run all tests
cargo test

# Run linter
cargo clippy --workspace -- -D warnings

# Check formatting
cargo fmt --check
```

## Making Changes

### 1. Create a Branch
```bash
git checkout -b feat/your-feature
```

### 2. Make Changes
- Read `AGENTS.md` for coding conventions
- Follow existing code style
- Add tests for new functionality

### 3. Verify
```bash
cargo test
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

### 4. Commit
```bash
git commit -m "feat(scope): description"
```

### 5. Push and Create PR
```bash
git push origin feat/your-feature
```

## Branch Naming

- `feat/feature-name` — new features
- `fix/bug-description` — bug fixes
- `refactor/module-name` — refactoring
- `docs/topic` — documentation
- `test/module-name` — adding tests

## Pull Request Checklist

- [ ] Tests pass (`cargo test`)
- [ ] Linter passes (`cargo clippy`)
- [ ] Formatting is correct (`cargo fmt`)
- [ ] New code has tests
- [ ] Documentation updated if needed
- [ ] No breaking changes to IPC protocol (or discussed in PR)

## Code Review

All PRs require review before merging. Reviews check:
1. Correctness and completeness
2. Test coverage
3. Performance implications
4. Security considerations
5. Code style consistency

## Reporting Issues

Use GitHub Issues with these labels:
- `bug` — something is broken
- `enhancement` — new feature request
- `documentation` — docs improvement
- `good-first-issue` — beginner-friendly
- `help-wanted` — needs community help

## License

By contributing, you agree that your contributions will be licensed under MPL-2.0.
