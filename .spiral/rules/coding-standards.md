---
paths:
  - "**/*.rs"
  - "**/*.md"
  - "**/*.toml"
---

# Coding Standards

> **Read first.** This file is the operative contract for Rust
> style, locale, and Markdown/TOML conventions. The companion
> workflow gate table lives in [`AGENTS.md`](../AGENTS.md) and
> the gate-level detail lives in
> [`.spiral/rules/workflow.md`](workflow.md). Where this file and
> `AGENTS.md` disagree, this file wins for style-specific
> questions; `workflow.md` wins for "what tool, when".

## Workflow Tools (mandatory)

| Moment | MUST run | Why |
|--------|----------|-----|
| Before claiming any code change complete | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` | Enforces 4-space indent, line length, and lint policy in this file. |
| After editing any `.md` file (ledger, ADRs, plans, rule files) | `./scripts/audit-doc-drift.sh` | Catches Australian-vs-American spelling regressions (this file's first rule). |
| Before merging a cross-crate change | `just verify-packet <crate>` | Wraps fmt + clippy + test + audit into one scoped command. |

## Language and Locale

- All code, comments, docstrings, commit messages, and Markdown
  files MUST be in **English**, **Australian spelling**.
- Use "initialise", "optimise", "colour", "behaviour",
  "programme" (noun), "centre", "analyse", "organisation".
- Use "z" → "s" for verbs (`realise`, `organise`).
- A code or comment containing "color", "initialize", or
  "behavior" MUST be rewritten before commit; the spelling audit
  in `./scripts/audit-doc-drift.sh` exits non-zero on American
  spellings under `**/*.md`.

The single exception: third-party API schemas that require a
specific keyword (e.g. a CSS property name) keep the spec spelling.

## Rust

### Error handling

- `?` operator over `.unwrap()` in library code.
- `#[must_use]` on functions that return important values.
- `thiserror` for typed errors. `anyhow` for application-level
  top-level error wrapping.
- Never `unwrap()` a user-supplied value. Use `?` + a typed error.
- Never swallow an exception. Log it with `tracing::error!` and
  return a meaningful error.

### Imports

- Sorted: std, external crates, internal crates, then a blank
  line, then `crate::*` and `super::*`.
- Use `crate::module::Type` over `super::Type` once the file is
  more than 100 lines.
- No `use ... as ...` unless the rename is needed for a
  documented reason (e.g. shadowing a std type).

### Types

- Prefer `&str` over `String` in function signatures.
- `pub` types in `lib.rs` are the public surface. Mark them
  `#[non_exhaustive]` if the type is likely to grow fields.
- Derive `Debug` on every public type.
- Derive `Clone` only if cloning is cheap. Otherwise,
  `#[derive(Clone)]` is a footgun.

### Style

- 4-space indent. No tabs.
- `cargo fmt` and `cargo clippy -- -D warnings` are the
  ground truth. Run them before every commit.
- No trailing whitespace. LF line endings (see `.editorconfig`).
- Max line length 100 (rustfmt default).
- No dead code. Remove or `#[allow(dead_code)]` with a comment.
- No `// TODO: ...` without a linked issue / packet reference.

### Comments

- Doc comments on every `pub` item: `///` for one-line, `//!` for
  module-level. Use full sentences. Include an example where the
  API is non-obvious.
- `// This does X because Y` over `// magic`. Comments MUST be written
  for future agents to read.
- No `// ...` truncation or stub shortcuts. Write the full code
  or don't write it.

## Markdown

- LF line endings. Trailing newline at EOF.
- Heading levels: one `#` per file, `##` for top sections,
  `###` for subsections. No `#` levels deeper than `####`.
- Code blocks with language tags (` ```rust `, ` ```bash `).
- No trailing whitespace.
- No emoji unless the user explicitly asked for it.
- Internal links use relative paths (`../foo.md`), not
  GitHub-relative or absolute URLs.

## TOML

- 2-space indent.
- Workspace deps in `[workspace.dependencies]`.
- Feature flags grouped under `[features]`.
- No inline tables for non-trivial config.

## File headers (Rust)

- Module-level doc comment on every `lib.rs` and every `mod foo;`
  in `mod.rs`.
- Crate-level doc comment on the crate root with: purpose, the
  public surface, the "owns" types, the consumers.

## AI-assisted commits

Spiral is LLM-assisted (see [`docs/methodology.md`](../docs/methodology.md)).
Commits produced with LLM assistance MUST add a
`Assisted-by: <model>` footer to the commit body. The footer is a
transparency signal, not a stigma. Example:
`Assisted-by: Claude Opus 4.7`. Human-authored commits do not need
the footer.

The commit-message type/scope table in `AGENTS.md` § Commit Messages
is unchanged. The `Assisted-by:` line is added to the body, not to
the type/scope table.

Borrowed 2026-06-16 from the Zeus repo's `.zeus/rules/coding-standards.md`.
