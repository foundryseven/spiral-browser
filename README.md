# Spiral Browser

![Spiral Browser Logo](resources/icons/logo.png)

A fully independent web browser built from scratch in Rust. Not based on Chromium, WebKit, or Gecko. Features a custom rendering engine, custom JavaScript engine, and a Zen-browser-inspired UI with vertical sidebar tabs, a floating URL bar, and single-accent-colour theming.

## Status

**Alpha — Phase 1 (Engines Foundation) in flight; Phase 1.5 SSOT Restructure shipped at v0.0.0-bootstrap.** Phase 0 (workspace + IPC shell + hello-world render) is done. Current work is in Phase 2 Step 2.1 (DOM quirks mode propagation — Packets 2.1.1 ✅, 2.1.2 ✅). The single source of truth for what is built, in flight, and missing is [`docs/implementation_tracker.md`](docs/implementation_tracker.md) (Group → Phase → Step → Packet). Time-based Month / Sprint / Chunk / Item vocabulary is retired as of 2026-06-16.

The **no-code-agentic workflow refactor** (R1–R8) is shipped on `main` as of 2026-06-18. The agent-driven workflow is the canonical surface for the next packet: a session start runs `bin/spiral-context.sh`, work lands via `just verify-fast` + `just verify-rules`, and PRs go out via `bin/spiral-pr.sh`. See [Workflow](#workflow) below.

## Quickstart

```bash
cargo build                # Build all crates
cargo test --workspace     # Run all tests
cargo clippy --workspace   # Lint check
cargo fmt --check          # Format check
```

The canonical end-to-end gate is `just verify` (= `just verify-fast` + `just verify-rules`). See [Workflow](#workflow) below.

Detailed build instructions: see `BUILD.md`.
Test strategy and conventions: see `TESTING.md`.

## Workflow

The agent-driven workflow is enforced by `just` and `bin/` scripts.

| Command | What it runs | When |
|---------|-------------|------|
| `bin/spiral-context.sh` | Prints the 5–7 most relevant files for the current task (or the 6 always-relevant files if no packet). | **Session start.** Replaces 6 manual file reads. |
| `bin/spiral-context.sh --rules-check` | Same, plus a fast scan of the R5 audit gates. | **Session start** when you want to confirm the contract is green before reading anything. |
| `just verify-fast` | `cargo fmt --check` + `cargo clippy -D warnings` + `cargo test --workspace` + `cargo build --workspace`. | **Pre-commit** (mandatory gate per `AGENTS.md`). |
| `just verify-rules` | `cargo +nightly clippy -D warnings` + `audit-orphan-exports.sh` (default + `--tool-coverage`) + `audit-doc-drift.sh`. | **Pre-merge** and **CI nightly**. |
| `just verify` | `verify-fast` + `verify-rules`. | End-to-end canonical gate. |
| `bin/spiral-pr.sh <packet-id>` | Runs all pre-flight checks, pushes, opens a PR with a standardised body and reviewer checklist. | **End of session** when a PR is wanted. Do not invoke `gh pr create` directly. |
| `./scripts/audit-orphan-exports.sh` | Walks every `pub` symbol in every crate and confirms at least one consumer outside the home crate imports it. | **Before claiming "wiring complete."** |
| `./scripts/audit-orphan-exports.sh --tool-coverage` | Confirms every `bin/` and `scripts/` tool is named in a `.spiral/rules/*.md` file. | **Before claiming "workflow done."** |
| `./scripts/audit-doc-drift.sh` | Walks the SSOT docs and flags stale crate refs, retired vocabulary, and the R5 rule-file contract (passive verbs and missing `MUST`/`SHALL`/`REQUIRED`). | **Before claiming "docs done."** |

Full workflow contract: see [`AGENTS.md`](AGENTS.md) § Workflow Discipline and the rule files under [`.spiral/rules/`](.spiral/rules/).

## Project Documents

| Document | Purpose |
|----------|---------|
| `CODEX.md` | Quick reference for LLMs and contributors |
| `ARCHITECTURE.md` | System design and data flows (canonical) |
| `docs/system_architecture.md` | Architecture delta file (in-flight decisions) |
| `PLAN.md` | Implementation plan and crate structure |
| `ROADMAP.md` | Phase-by-phase development timeline (one-page index) |
| `BUILD.md` | Platform-specific build instructions |
| `TESTING.md` | Test strategy and commands |
| `ERRORS.md` | Common errors and fixes |
| `CONTRIBUTING.md` | Contribution process |
| `AGENTS.md` | AI agent instructions for this repository (canonical) |
| `bin/README.md` | Workflow scripts in `bin/` |
| `docs/active_context.md` | Live state: current phase, blockers |
| `docs/implementation_tracker.md` | Group → Phase → Step → Packet status (SSOT) |
| `docs/progress_ledger.md` | Append-only change log |
| `docs/archives/phase1-tasks.md` | Granular Phase 1 task breakdown (archived) |
| `docs/plans/no-code-agentic-refactor.md` | R1–R8 workflow refactor plan and §4 acceptance checklist |
| `docs/glossary.md` | Engine brand names (Gyre, Vortex, Fmt, Forge) |
| `docs/decisions/` | ADRs (cross-cutting decisions; link from tracker) |
| `docs/agents/` | Role contracts (implementer, reviewer, architect, …) |
| `docs/architecture/` | Per-subsystem architecture stubs |
| `.spiral/rules/` | Rule files (architecture, coding-standards, performance, testing, unsafe-standards, workflow, doc-drift-prevention) |
| `bin/` | Workflow scripts (`spiral-context.sh`, `spiral-pr.sh`) |
| `scripts/` | Build-time tooling (audit scripts) |
| `specs/GAP_ANALYSIS.md` | P0/P1/P2/P3 gap tracker across 4 engine sub-domains (spec-only) |
| `CHANGELOG.md` | Release history |
| `SECURITY.md` | Vulnerability disclosure process |
| `LICENSE` | MPL-2.0 licence terms |

## Architecture

Multi-process design: browser process, per-tab renderer processes, dedicated network and GPU processes. IPC over Unix domain sockets (Linux/macOS) and named pipes (Windows). Full architecture in `ARCHITECTURE.md` (canonical) and `docs/system_architecture.md` (delta file for in-flight changes).

```
spiral-core  →  spiral-ipc  →  spiral-dom  →  spiral-fmt   (HTML+CSS, from-spec)
                                       │      spiral-css    (deprecated shim → spiral-fmt)
                                       │      spiral-gyre   (Gyre — custom layout)
                                       │      spiral-vortex (Vortex — from-scratch JS)
                                       │      spiral-context (capability types)
                                       │      spiral-filter  (compile-time policy)
                  └─→  spiral-browser  ←  spiral-ui  ←  spiral-theme
                                spiral-network, spiral-net, spiral-crypto
                                spiral-render, spiral-paint, spiral-gpu
                                spiral-imagedecoder
                                spiral-sandbox
```

20-crate workspace (`crates/`): `spiral-core`, `spiral-ipc`, `spiral-dom`, `spiral-fmt`, `spiral-css` (deprecated shim → `spiral-fmt`), `spiral-gyre`, `spiral-render`, `spiral-paint`, `spiral-gpu`, `spiral-vortex`, `spiral-context`, `spiral-filter`, `spiral-network`, `spiral-net`, `spiral-crypto`, `spiral-imagedecoder`, `spiral-sandbox`, `spiral-ui`, `spiral-theme`, `spiral-browser`.

**Important removals (2026-06-15):** `spiral-html` retired, `html5ever` / `markup5ever` / `tendril` not vendored, `cssparser` / `selectors` not vendored. All HTML and CSS parsing is from-spec in `spiral-fmt`. See `docs/decisions/0001-css-parser-spiral-fmt.md`.

## License

MPL-2.0. See `LICENSE`.
