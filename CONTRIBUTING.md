# Spiral Browser — Contributing Guide

> **The workflow contract is the contributing contract.** Spiral is an agent-driven project: the same rules apply to a human contributor and an AI agent. The rules are in [`AGENTS.md`](AGENTS.md); this file is the human-friendly entry point.
>
> If you are an AI agent landing work, read [`AGENTS.md`](AGENTS.md) and the relevant role contract in [`docs/agents/`](docs/agents/) instead.

---

## 1. The shape of the work

Spiral is built around three in-house engines that bear the Spiral brand:

- **Gyre** (`spiral-gyre`) — layout
- **Vortex** (`spiral-vortex`) — JavaScript
- **Fmt** (`spiral-fmt`) — HTML5 + CSS parsers

Plus two in-design branded subsystems (**Filter** and **Context**), plus plumbing (the other 14 crates). The brand belongs to the engine, not the wire. See [`docs/glossary.md`](docs/glossary.md) for the canonical mapping.

Most contribution paths fall into one of three buckets:

1. **Landing a packet.** The unit of work. Branches, commits, gates, PR. See § 4 below.
2. **Filing an issue.** Bug report, feature request, spec ambiguity. See § 6 below.
3. **Triaging a security report.** Private disclosure. See [`SECURITY.md`](SECURITY.md).

---

## 2. Prerequisites

### Toolchain

- **Rust 1.75+** (stable). `rustup install stable`.
- **Cargo** (included with Rust).
- **`just`** — the command runner. `brew install just` (macOS), `apt install just` (Debian/Ubuntu), or `cargo install just`.
- **`gh`** — the GitHub CLI, for the PR workflow. `brew install gh` / `apt install gh` / `winget install GitHub.cli`.
- **Platform-specific dependencies:**
  - **Linux:** `libwayland-dev`, `libxkbcommon-dev`, `libfontconfig-dev`, `pkg-config`.
  - **macOS:** Xcode Command Line Tools (`xcode-select --install`).
  - **Windows:** Visual Studio Build Tools (Desktop development with C++ workload).

### Verify the environment

```bash
cargo build                    # Build all 20 crates (first time takes ~5 min)
cargo test --workspace         # Run the full test suite
just verify                    # Run the canonical end-to-end gate
```

If `just verify` returns exit 0, your environment is ready. CI runs the same gate.

---

## 3. The workflow contract

The full contract is in [`AGENTS.md`](AGENTS.md) § Workflow Discipline. The short version:

| Moment | MUST run |
|--------|----------|
| Session start | `bin/spiral-context.sh` (or `bin/spiral-context.sh <packet-id>` with a packet) |
| Mid-cycle (one crate) | `just test-fast <crate> [pattern]` |
| After a `pub` API change | `just test-with-deps <crate>` |
| Before claiming complete | `just verify-packet <crate>` |
| Pre-commit / pre-merge | `./scripts/audit-orphan-exports.sh` AND `./scripts/audit-doc-drift.sh` (both must exit 0) |
| End of session (PR wanted) | `bin/spiral-pr.sh <packet-id>` (do not invoke `gh pr create` directly) |

The canonical end-to-end gate is `just verify` (= `just verify-fast` + `just verify-rules`). It is the same gate CI runs. A green local gate is a green CI gate.

---

## 4. Landing a packet

A *packet* is the unit of work in Spiral. The packet list is at [`docs/implementation_tracker.md`](docs/implementation_tracker.md). Pick an unchecked packet and read its pre-expanded block via `bin/spiral-context.sh <packet-id>`.

### 4.1 Branch

```bash
git checkout main
git pull
git checkout -b feat/short-description   # or fix/, refactor/, docs/, chore/
```

Branch scopes match the commit scopes in `AGENTS.md` § Commit Messages:

- `feat(scope)` — new feature
- `fix(scope)` — bug fix
- `refactor(scope)` — refactor
- `test(scope)` — test changes only
- `docs(scope)` — doc changes only
- `chore(scope)` — tooling, build, deps

Examples: `feat/gyre-flex`, `fix/vortex-closures`, `docs/no-code-agentic-refactor`.

### 4.2 Code

- Read the file before editing it (mandatory).
- Follow the conventions in [`AGENTS.md`](AGENTS.md) § File Editing and the rule files in [`.spiral/rules/`](.spiral/rules/).
- For a new engine or subsystem, file an ADR in `docs/decisions/` first.
- For any claim of "novel", "first", "unique", or "no prior art", run the Novelty Claims rule in `AGENTS.md` first. The M4 audit methodology (`docs/audit-sprint-m4.md`) is the canonical standard.

### 4.3 Test

- Every public function needs a unit test in `#[cfg(test)] mod tests`.
- Integration tests go in `tests/` per crate.
- Run `just test-fast <crate>` mid-cycle, `just test-with-deps <crate>` after a `pub` API change.

### 4.4 Verify

```bash
just verify-packet <crate>    # fmt + clippy + test + audit-orphan-exports, scoped to <crate>
```

If the packet affects a `pub` API, also run:

```bash
just test-with-deps <crate>   # reverse-dep fan-out
```

### 4.5 Pre-commit gate

```bash
just verify-fast              # the fast pre-commit gate
./scripts/audit-orphan-exports.sh    # every pub symbol has a consumer
./scripts/audit-doc-drift.sh         # SSOT consistency
```

All three must exit 0. If any fails, fix and re-run. Do not push a red gate.

### 4.6 Commit and push

```bash
git add -A
git commit -m "feat(gyre): implement flex container alignment

Added flex-start, center, flex-end alignment for the main axis.
Works for both horizontal and vertical axes."
git push -u origin feat/gyre-flex
```

Commit messages follow the convention in `AGENTS.md` § Commit Messages:

```
type(scope): description

[optional body]

[optional footer]
```

### 4.7 PR

```bash
bin/spiral-pr.sh <packet-id>
```

This runs the pre-flight checks, pushes, and opens a PR with a standardised body and reviewer checklist. **Do not invoke `gh pr create` directly.** The `bin/spiral-pr.sh` wrapper ensures the PR body has the right shape and that the reviewer's checklist is in place.

### 4.8 Review

A reviewer (human or AI agent) will run the same `just verify` gate on your PR. If the CI gate is green, the review is mostly about: does the change follow the architecture? Is the ADR in place? Is the brand voice right? Is the test coverage real?

---

## 5. Coding conventions

The full list is in [`AGENTS.md`](AGENTS.md) § File Editing and the rule files in [`.spiral/rules/`](.spiral/rules/). The minimum:

- **Read the file before editing it.** Always.
- **Never modify `Cargo.lock` manually.** Let `cargo` handle it.
- **Keep imports sorted:** std, external crates, internal crates.
- **`snake_case` for functions/variables, `PascalCase` for types.**
- **Prefer `?` over `.unwrap()` in library code.** `unwrap()` is fine in tests.
- **Add `#[must_use]` to functions that return important values.**
- **Never depend "up" the dependency graph.** Check `Cargo.toml` before adding a dependency.
- **The brand belongs to the engine.** A new engine gets a name. A wire protocol does not.
- **Wiring & Integration:** a `pub` symbol is not done when it compiles; it is done when an external consumer imports it. The `audit-orphan-exports.sh` gate enforces this.

---

## 6. Filing an issue

We track issues at `https://github.com/foundryseven/spiral-browser/issues`. Use the appropriate template:

- **Bug report:** what you did, what you expected, what happened, your environment.
- **Feature request:** the capability you want, the user story, the trade-offs you'd accept.
- **Spec ambiguity:** a citation of the spec, the section, and the question. Most useful if you have a concrete reproducer.

For security issues, see [`SECURITY.md`](SECURITY.md). **Do not** file security issues in the public tracker.

---

## 7. Reviewer checklist

Reviewing a PR is a structured activity. The full role contract is at [`docs/agents/reviewer.md`](docs/agents/reviewer.md). The minimum:

- [ ] `just verify` is green locally.
- [ ] CI is green (11 jobs).
- [ ] The PR has a corresponding packet ticked in `docs/implementation_tracker.md`.
- [ ] The PR has a corresponding ledger entry in `docs/progress_ledger.md` with a `### Wiring & Integration` section.
- [ ] No `pub` symbol is orphaned (`audit-orphan-exports.sh`).
- [ ] No SSOT doc drift (`audit-doc-drift.sh`).
- [ ] The brand voice is consistent with the rest of the codebase.
- [ ] Any cross-cutting decision has an ADR.
- [ ] Any novelty claim has been verified by a research agent.

---

## 8. Style

- **English, Australian spelling.** `initialise`, `optimise`, `colour`, `behaviour`. See `~/.config/opencode/AGENTS.md` § 1.2.
- **No emoji in source code or docs.** Emojis in commit messages and chat are fine; emojis in the rendered docs are not.
- **No "Consider…" / "You might want to…" / "It would be nice if…"** in rule files. The rule files use `MUST` / `MUST NOT` / `MUST RUN` / `SHALL` / `REQUIRED`. See `AGENTS.md` § 2.1.

---

## 9. Licence

By contributing, you agree that your contributions are licensed under the project's MPL-2.0 licence. See [`LICENSE`](LICENSE).
