# Spiral Browser — Multi-Model Workspace Instructions

This document tells LLM agents how to work on the Spiral Browser codebase.
Global developer instructions are in `~/.config/opencode/AGENTS.md` (read-only,
do not override).

---

## Workflow Discipline (Compulsory)

This file and the seven rule files under [`.spiral/rules/`](.spiral/rules/) are the
agent's operating contract. The user is **no-code-agentic**: they will not
remember commands, and they do not want to. The agent drives the workflow
end-to-end and reports back. The rules are not advisory; the directive verbs
(`MUST`, `SHALL`, `REQUIRED`) are gating.

**The single source of truth for "what tool, when" is
[`.spiral/rules/workflow.md`](.spiral/rules/workflow.md).** This section is a
checklist that mirrors it; if the two ever disagree, the rule file wins and
the discrepancy is a bug.

**Origin of the model:** [`docs/plans/no-code-agentic-refactor.md`](docs/plans/no-code-agentic-refactor.md).
The plan describes why the agent drives the workflow end-to-end and how the
global↔project rule split works. Packet R2 (this rewrite) is one of six atomic
packets shipping on `refactor/no-code-agentic`.

### Mandatory gates (every session)

The agent MUST satisfy each gate below; failure to do so is a blocking issue.

| Moment | MUST run | Why |
|--------|----------|-----|
| Session start (no packet) | `bin/spiral-context.sh` | Surfaces the 6 always-relevant files in one command; replaces 6 manual reads. |
| Session start (with packet) | `bin/spiral-context.sh <packet-id>` | Adds the packet's pre-expanded block (spec, crates affected, call sites, tests expected, end-to-end surface). |
| Mid-cycle (one crate) | `just test-fast <crate> [pattern]` | Skips the 30–60s `cargo test --workspace` overhead. |
| After a `pub` API change | `just test-with-deps <crate>` | Computes reverse-dep fan-out and runs all impacted crates. |
| Before claiming complete | `just verify-packet <crate>` | Wraps `fmt + clippy + test + audit-orphan-exports` into one scoped command. |
| Pre-commit / pre-merge | `./scripts/audit-orphan-exports.sh` AND `./scripts/audit-doc-drift.sh` | Both audits exit 0 on success; exit 1 = blocking. |
| End of session (PR wanted) | `bin/spiral-pr.sh <packet-id>` | Runs all pre-flight checks, pushes, opens PR. Do not call `gh pr create` manually. Codacy merge gate is enforced by **Spiral-Bot** (`.github/workflows/codacy-bot.yml`) — the bot auto-fixes Codacy findings on PRs. See `docs/active_context.md` for Spiral-Bot details. |

### Prohibited behaviour

- The agent MUST NOT manually re-load the 5–7 SSOT files that `bin/spiral-context.sh`
  already surfaces. That is the 15–20 minute per-session cost the script exists
  to eliminate.
- The agent MUST NOT skip `just verify-packet` "because clippy passed locally" —
  the audit scripts catch wiring and doc-drift regressions that clippy does not.
- The agent MUST NOT invoke `gh pr create` directly. `bin/spiral-pr.sh` is the
  entry point and runs the pre-flight checks automatically.
- The agent MUST NOT bypass Codacy. Spiral-Bot (`.github/workflows/codacy-bot.yml`)
  drives Codacy to green automatically on PRs. If Spiral-Bot is blocked (missing
  secrets, OpenCode Go cap), the agent MUST surface the situation to the human.
- The agent MUST NOT add a directive that contradicts the rules in
  `.spiral/rules/`. When in doubt, the rule file wins; file a ledger note to
  propose an amendment.

### Cross-references

- Rule file for the workflow gates: [`.spiral/rules/workflow.md`](.spiral/rules/workflow.md)
- Role contracts (implementer / reviewer / etc.): [`docs/agents/`](docs/agents/)
- Origin plan (six atomic packets): [`docs/plans/no-code-agentic-refactor.md`](docs/plans/no-code-agentic-refactor.md)

---

## Current Status

| Field | Value |
|-------|-------|
| **Phase** | Phase 1 — Engines Foundation 🔄 IN FLIGHT (Step 1.6 Vortex GC rewrite; packets 1.6.1-1.6.5 SHIPPED, packets 1.6.6-1.6.8 retired to Step 2.8; packets 2.8.1 SHIPPED, 2.8.2 SHIPPED, 2.8.3 SHIPPED; Packet 2.1.1 SHIPPED, Packet 2.1.2 SHIPPED, Packet 2.1.3 SHIPPED); **Phase 2 Steps 2.9–2.12 added 2026-06-18 (table-stakes i18n, ADR-0007)** |
| **Phase 1.5 SSOT Restructure** | ✅ SHIPPED at `v0.0.0-bootstrap` (2026-06-16) |
| **Active state** | [`docs/active_context.md`](docs/active_context.md) (live pointer) |
| **Status SSOT** | [`docs/implementation_tracker.md`](docs/implementation_tracker.md) (Group → Phase → Step → Packet) |
| **Architecture SSOT** | [`docs/system_architecture.md`](docs/system_architecture.md) |
| **Glossary** | [`docs/glossary.md`](docs/glossary.md) (engine brand names) |
| **ADRs** | [`docs/decisions/`](docs/decisions/) (cross-cutting decisions; link from tracker) |
| **Role contracts** | [`docs/agents/`](docs/agents/) (implementer / reviewer / architect / tester / security / release / onboarding / PROMPT_LIBRARY) |
| **Per-subsystem architecture** | [`docs/architecture/`](docs/architecture/) (vortex, gyre, fmt, net, filter, context, design/) |
| **Rule files** | [`.spiral/rules/`](.spiral/rules/) (architecture, coding-standards, testing) |
| **Change log** | [`docs/progress_ledger.md`](docs/progress_ledger.md) (append-only) |
| **Spec** | [`specs/GAP_ANALYSIS.md`](specs/GAP_ANALYSIS.md) (spec-only; status moved to tracker) |
| **Roadmap** | [`ROADMAP.md`](ROADMAP.md) (one-page Group → Phase index) |
| **Wiring audit script** | [`scripts/audit-orphan-exports.sh`](scripts/audit-orphan-exports.sh) |
| **Doc-drift audit script** | [`scripts/audit-doc-drift.sh`](scripts/audit-doc-drift.sh) |
| **Workflow scripts** | [`bin/`](bin/) (context primer, PR workflow — see `bin/README.md`) |
| **Build recipes** | [`justfile`](justfile) (`just verify`, `just test-fast`, `just context`, etc.) |

Read [`docs/implementation_tracker.md`](docs/implementation_tracker.md) **before
starting any task**. It is the single source of truth for what is in flight,
what is blocked, and what you must not touch. The tracker is grouped by
**Group → Phase → Step → Packet**; the time-based `Month` / `Sprint` / `Chunk` /
`Item` vocabulary is **retired** as of 2026-06-16.

---

## Model Routing

All agents in this repository use `ozore/ozore/minimax-m3` (per the
system prompt). No model switching is configured at the repo level; the
global config handles that. If you need a different model for a
specific role, update `~/.config/opencode/opencode.jsonc`.

---

## SSOT Update Protocol

After completing any task loop, the **implementer agent** must:

1. Append an entry to `docs/progress_ledger.md`.
2. Update `docs/active_context.md` if Phase state, blockers, or "do not touch"
   zones changed.
3. Tick the corresponding packet in `docs/implementation_tracker.md`
   (change `[ ]` to `[x]`).
4. Create an ADR under `docs/decisions/` if the task took a cross-cutting
   design choice; link the ADR from the relevant Step in the tracker.
5. If a tracked gap from `specs/GAP_ANALYSIS.md` was fixed, the spec stays
   unchanged (it is spec-only now); only the tracker packet ticks.

The **reviewer agent** must flag a stale `docs/implementation_tracker.md` or
`docs/active_context.md` (last update older than the current task) as a
blocking issue.

---

## Decision Protocol

When you hit a fork, the answer is usually one of these four. Pick one and
proceed; do not invent a fifth category.

| Situation | Action |
|-----------|--------|
| The change fits the existing plan and uses the existing toolchain. | Proceed. Mention the work in the next ledger entry. |
| The change is a bug fix, a small refactor, or a docs tweak in a single crate. | Proceed. Mention the work in the next ledger entry. |
| The change renames a crate, swaps a dependency, changes a public type, or alters the build graph. | Stop. Write an ADR (`docs/decisions/NNNN-…md` from `0000-template.md`). The ADR goes in *this* commit; the implementation may follow. |
| The change is novel, unique, or claims "first", "no prior art", or "no shipped browser does this". | Stop. Run the Novelty Claims rule (below) *before* writing code. |

When in doubt: prefer the smaller change. The fork is rarely as wide as it
looks. A rename to a single crate is a one-line `Cargo.toml` edit and an ADR
forward-reference, not a multi-day refactor.

---

## Workflow Tools (every session)

These tools exist because the SSOT is large and re-loading it
into working memory costs 15-20 minutes per fresh session.
Adopted 2026-06-17; see `bin/README.md` for the design rationale.
The table below is **directive**: each "MUST run" column is a
compulsory gate enforced by the rules under
[`.spiral/rules/workflow.md`](.spiral/rules/workflow.md).

| Tool | When (MUST run) | Why |
|------|-----------------|-----|
| `bin/spiral-context.sh` (or `just context`) | **MUST run at the start of every session,** before reading any docs. | Prints the 5-7 most relevant files for the current packet (or the 6 always-relevant files if no packet). Replaces 6 manual file reads with one command. |
| `just test-fast <crate> [pattern]` | **MUST run mid-cycle** while writing tests and code for a single packet. | Skips the 30-60s `cargo test --workspace` overhead. Use the full workspace test for pre-commit only. |
| `just test-with-deps <crate>` | **MUST run after any `pub` API change** in `spiral-foo`. | Computes the reverse-dep fan-out and runs all impacted crates. Catches breakage the unit test missed. |
| `just verify-packet <crate>` | **MUST run before claiming a packet complete.** | Wraps `fmt + clippy + test + audit-orphan-exports` into one command, scoped to the crate you touched. |
| `bin/spiral-pr.sh <packet-id>` | **MUST run at end of session, when a PR is wanted.** Replaces manual `gh pr create` invocations. | Runs all pre-flight checks (fmt, clippy, test, both audits), pushes, opens a PR with a standardised body and reviewer checklist. `--dry-run` for preview, `--skip-tests` for hot fixes. |
| `scripts/audit-orphan-exports.sh` | **MUST run pre-commit and pre-merge.** | Catches `pub` symbols with no external consumer (wiring gaps). Exit 0 = clean; exit 1 is a build break. |
| `scripts/audit-doc-drift.sh` | **MUST run pre-commit and pre-merge.** | Catches SSOT inconsistencies (tracker out of sync with active context, AGENTS.md status row missing shipped packets, retired vocabulary). Exit 0 = clean; exit 1 is a build break. |

The role contracts in `docs/agents/` (especially `implementer.md`)
encode WHEN to invoke each tool. The `.spiral/rules/` directory
holds the cross-cutting rules. Both pull from this table as the
single source of truth for "what tool, when."

---

## Quick Start

In the no-code-agentic model, the agent reads the SSOT — not the user.
The 3-step sequence below is what the agent MUST follow on every fresh
session. The 11-step "Manual fallback" list is preserved below for the
rare case where `bin/spiral-context.sh` is unavailable.

1. **MUST run `bin/spiral-context.sh [<packet-id>]`** (or `just context`).
   The script prints the 5–7 most relevant files for the current task and,
   when given a packet id, the packet's pre-expanded block (spec, crates
   affected, call sites, tests expected, end-to-end surface). This replaces
   manual file reads.
2. **MUST read the 5–7 files the script surfaced.** Do not re-load the
   always-relevant six by hand — the script has already done the routing.
3. **MUST follow the relevant rule in `.spiral/rules/`.** The seven rule
   files are the operative contract; the role doc in `docs/agents/<role>.md`
   points to the rules that apply to that role.

### Manual fallback (only if `bin/spiral-context.sh` is broken)

The original 11-step on-ramp follows. Use it only when the script is
unavailable; otherwise it is wasteful repetition.

1. Read `CODEX.md` for project overview
2. Read `docs/system_architecture.md` for system design
3. Read `docs/implementation_tracker.md` for current Phase and packets
4. Read `docs/active_context.md` for live Phase state
5. Skim `docs/glossary.md` so engine names make sense
6. If picking up a numbered item, read the relevant ADR in `docs/decisions/`
7. If the task is your first in the codebase, skim `docs/agents/onboarding.md`
   then `docs/agents/<your-role>.md`
8. Skim the relevant rule file in `.spiral/rules/`
9. Run `cargo build` to verify your environment
10. Run `cargo test` to verify tests pass

---

## Project Rules

### File Editing
- Always read a file before editing it
- Never modify `Cargo.lock` manually (let `cargo` handle it)
- Keep imports sorted: std, external crates, internal crates
- Use `snake_case` for functions/variables, `PascalCase` for types
- Prefer `?` operator over `.unwrap()` in library code
- Add `#[must_use]` to functions that return important values

### Crate Boundaries
- Never depend on a crate "up" the dependency graph
- Example: `spiral-core` cannot depend on `spiral-browser`
- Check `Cargo.toml` before adding a dependency
- Prefer re-exporting from `spiral-core` for shared types

### Commit Messages
```
type(scope): description

[optional body]

[optional footer]
```
Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`
Scopes: core, ipc, fmt, css, gyre, render, dom, vortex, net, network, ui, theme, browser, sandbox, filter, context, crypto

> Note: scopes `js` and `layout` are deprecated as of Phase 1 Step 1.2 — they
> were the pre-rename names. Use `vortex` and `gyre` instead. Old commits
> stay unchanged for traceability.

Example:
```
feat(gyre): implement flex container alignment

Added flex-start, center, flex-end alignment for the main axis.
Works for both horizontal and vertical axes.
```

### Wiring & Integration

A change is not "done" until its outcome is reachable from a real surface.
Concretely:

1. **A `pub` symbol is not done when it compiles.** It is done when at least
   one consumer outside the symbol's home crate imports it. A symbol with
   no external consumer is an **orphan export** — a wiring gap.
2. **A library function is not done when it has unit tests.** It is done
   when at least one call site in another crate, or a public entry point
   in the same crate's binary surface, exercises it.
3. **A new crate is not done when it has a `Cargo.toml`.** It is done when
   the binary surface (`spiral-browser` for the end-user app, the unit-test
   entry for a pure-logic crate) actually imports it.
4. **The audit script is the ground truth.** Run
   `./scripts/audit-orphan-exports.sh` before any "this is done" claim. The
   script exits 1 on orphans; treat exit 1 as a build break.

Every ledger entry's "Wiring & Integration" section must name:
- The crates affected
- The call sites (file:line if specific)
- The test coverage that exercises the path
- The end-to-end surface that proves the wiring is real (a CLI command,
  a `#[test]`, a fixture run, a render output — something a human can
  verify)

Adopted 2026-06-16 from the Zeus repo's `docs/decisions/0006-cross-
cutting-features.md` rule. See `docs/decisions/0000-template.md` for the
ADR structure; the "Wiring & Integration" section is required.

### Testing
- Every public function needs a unit test
- Use `#[cfg(test)] mod tests` in the same file
- Integration tests go in `tests/` per crate
- Run `cargo test` before committing
- Run `cargo clippy` for lint checks
- Run `./scripts/audit-orphan-exports.sh` before any "wiring complete" claim

### Novelty Claims
- Any claim of "novel", "first", "unique", "no prior art", or "no shipped
  browser does this" **must** be verified by a research agent before committing.
- The verification must check: V8, SpiderMonkey, JSC, Servo, Ladybird, Flow,
  Brave, and relevant academic literature. Wikipedia is a starting point, not
  a conclusion.
- If verification finds prior art, downgrade the claim honestly. "Partially
  novel (combination is new)" or "configuration choice" are valid categories.
- The M4 audit methodology (`docs/audit-sprint-m4.md`) is the canonical
  standard. Four rounds of retrospective correction taught us that overclaiming
  is the default failure mode — gate it proactively. References to "M4" /
  "M4.5" / "Item 8" in the historical record map to Phase 1.6 packets in
  [`docs/implementation_tracker.md`](docs/implementation_tracker.md).
- Design docs, progress ledger entries, and active_context updates are all
  in scope. Commit messages are not (too noisy).

---

## Working on Specific Crates

### spiral-core
- Shared types only, no business logic
- All other crates depend on this
- Changes here affect everything — be careful
- Run full workspace tests after changes

### spiral-ipc
- Platform-specific code uses `#[cfg(target_os)]`
- Test on current platform only
- Ensure message framing is correct
- Check for buffer overflow in deserialization

### spiral-fmt
- Spiral's from-spec HTML5 tokeniser and tree builder — no html5ever, no
  markup5ever, no tendril. Pure Spiral-native Rust.
- From-spec CSS Syntax Level 3 tokeniser and parser in `src/css/`. No
  `cssparser`, no `selectors`, no `cssparser-macros`.
- Output is `spiral-dom::Dom` (HTML) or `spiral_fmt::css::Stylesheet` (CSS).
- Public entry points: `spiral_fmt::parse_html` and `spiral_fmt::parse_css`.
- `spiral-html` is **retired** (removed from workspace 2026-06-15). All
  references to html5ever-based parsing are historical.
- HTML parser covers 8 insertion modes. CSS parser covers 8 modules:
  tokeniser, parser, selectors, specificity, values, at-rules, declarations,
  attribute matchers.
- Handle encoding detection carefully (UTF-8 only for now)
- Test with malformed HTML (the tree builder is lenient by design) and
  malformed CSS (the parser recovers from errors per CSS Syntax 3 §5).
- See `docs/decisions/0001-css-parser-spiral-fmt.md` for the Fork 1-B
  decision context.

### spiral-css
- **Deprecated shim** (Phase 1 Step 1.5, 2026-06-16). Forwards to
  `spiral_fmt::css::*` and provides a `CssParser` adapter that calls
  `spiral_fmt::parse_css`. New code should depend on `spiral-fmt` directly.
- Cascade order: user agent < user < author < author!important
- Specificity: inline > ID > class > element
- Test with complex selector chains
- See `docs/decisions/0001-css-parser-spiral-fmt.md` and the
  `deprecation` lint on the crate.

### spiral-gyre (Gyre)
- Gyre is Spiral's in-house layout engine — fully custom, no Taffy
- Box model is foundation — get this right first
- Block layout: vertical stacking, margin collapse
- Flexbox: custom implementation (Phase 2)
- Grid: custom implementation (Phase 2)

### spiral-render
- Vello for GPU rendering
- Display list is intermediate representation
- Keep render ops simple and composable
- Test with simple shapes first, text later

### spiral-dom
- DOM tree is central data structure
- Nodes are arena-allocated (Vec<Node> + indices)
- Parent/child relationships via indices
- Attribute storage: `Vec<(String, String)>`

### spiral-vortex (Vortex)
- Vortex is Spiral's from-scratch JavaScript engine, written entirely in safe Rust
- Implements ECMAScript from the ground up: lexer, parser, AST, bytecode compiler, interpreter, mark-sweep GC
- Phase 1: tree-walking interpreter (lex → parse → AST → walk)
- Phase 2: bytecode VM (AST → bytecode → stack-based interpreter, ~5-10× faster)
- Phase 3: baseline JIT (Cranelift for hot functions)
- `rusty_v8` available behind `v8` feature flag for CI compliance testing only — NOT the production engine
- `trait JSRuntime` abstraction enables future engine swapping via feature flag
- Start with console.log only
- DOM manipulation comes later
- Test with simple scripts first

### spiral-network
- hyper for HTTP — read hyper docs
- hickory-dns for DNS resolution
- TLS via rustls
- Handle redirects, timeouts, errors

### spiral-ui
- GPU-rendered browser chrome
- Zen-style design tokens from spiral-theme
- Sidebar tabs, floating URL bar
- Test with winit window directly

### spiral-theme
- CSS custom properties approach
- Light/dark mode
- Single accent color
- System preference detection

### spiral-sandbox
- Platform-specific — only compile for target OS
- Test sandbox profile creation
- Test that blocked operations fail
- Linux: Landlock + seccomp-bpf
- macOS: Seatbelt profiles
- Windows: Restricted Token

---

## Debugging

### Build Issues
```bash
cargo build 2>&1 | head -50    # see first errors
cargo check                     # type checking only
cargo clippy                    # lint warnings
```

### Test Failures
```bash
cargo test                      # all tests
cargo test spiral-core          # specific crate
cargo test test_name            # specific test
```

### IPC Debugging
- Enable RUST_LOG=debug for verbose IPC messages
- Check socket/pipe permissions on Linux/macOS
- Verify named pipe path on Windows

### Rendering Debugging
- Enable RUST_LOG=trace for render pipeline
- Check wgpu adapter selection
- Verify GPU driver compatibility

---

## Common Pitfalls

1. **Forgetting `#[cfg]` guards** — Platform-specific code must be guarded
2. **Breaking IPC serialization** — Changing `IPCMessage` enum breaks bincode
3. **Circular dependencies** — Check dependency graph before adding imports
4. **Unwrap in library code** — Always use `?` or proper error handling
5. **Missing tests** — Every public function needs at least one test
6. **Blocking in async** — Never use `std::thread::sleep` in tokio context

---

## File Templates

### New Crate
```
crates/spiral-{name}/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── error.rs
│   └── {module}.rs
└── tests/
    └── {name}_test.rs
```

### New Module
```rust
// src/{module}.rs

//! Module description

use crate::error::Error;

/// Public type documentation
pub struct TypeName {
    field: Type,
}

impl TypeName {
    /// Constructor documentation
    pub fn new(field: Type) -> Self {
        Self { field }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let t = TypeName::new(value);
        assert_eq!(t.field, value);
    }
}
```

---

## Communication Between Models

When working in a multi-model environment:
- Reference files by path and line number
- Quote exact code snippets when discussing changes
- State which crate and module you're modifying
- Mention any breaking changes to IPC protocol
- Note platform-specific behavior

Example:
> I modified `crates/spiral-gyre/src/block.rs:45` to fix margin collapse.
> The change affects `BlockLayout::compute()` which is called from `LayoutTree::layout()`.
> This only affects Linux builds due to `#[cfg(target_os = "linux")]` in the test.

---

## R2 Cross-Reference

This file was rewritten under **Packet R2** of
[`docs/plans/no-code-agentic-refactor.md`](docs/plans/no-code-agentic-refactor.md)
(2026-06-17). R2 inserts the "Workflow Discipline (Compulsory)" section at the
top, adds `MUST` directives to the Workflow Tools table, and demotes the
11-step Quick Start to a Manual fallback. Companion packets:

- **R1** — global config rewrite (`~/.config/opencode/*`).
- **R3** — reword `.spiral/rules/*.md` so each file is self-standing.
- **R4** — update `docs/agents/*.md` role contracts to cross-reference the rule files.
- **R5** — extend `scripts/audit-orphan-exports.sh` and `scripts/audit-doc-drift.sh`
  to enforce R1–R4 (e.g. gate on "MUST" verb presence, reject stale rule copies).
- **R6** — fix any stale crate references discovered during R3/R4 (currently
  flagged in `docs/agents/test-writer.md`).

All six packets ship as a single batch on branch `refactor/no-code-agentic`.
