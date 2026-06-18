# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- **Public-facing doc overhaul (2026-06-18).** All eight public-facing root docs — `README.md`, `CODEX.md`, `PLAN.md`, `ROADMAP.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `SECURITY.md`, `CONTRIBUTING.md` — received full rewrites. The rewrites:
  - Add a unified brand voice (Gyre, Vortex, Fmt, Filter, Context) throughout.
  - Add explicit project vision, goals, non-goals, and core principles.
  - De-duplicate the eight docs: `README.md` is the front door; `CODEX.md` is the LLM cheatsheet; `PLAN.md` is the strategic plan; `ROADMAP.md` is the one-page phase index; `ARCHITECTURE.md` is the design canon.
  - Document the engine brand identity, the 20-crate workspace structure, the IPC protocol, the render pipeline, the capability-typed context, and the security model in one place.
- **GitHub Actions: 11-job CI pipeline (2026-06-18).** Added `tool-coverage` and `nightly-clippy` jobs to `.github/workflows/ci.yml`. The R5 audit enforcement contract is now uniformly enforced across local (`just verify-rules`) and CI.
- **No-code-agentic workflow refactor (R1–R8, 2026-06-18).** Six-packet refactor of the agent-driven workflow contract:
  - **R1** — Global `~/.config/opencode/AGENTS.md` rewrite (Spiral-specific content stripped into the project tree).
  - **R2** — Project `AGENTS.md` rewrite with the `## Workflow Discipline (Compulsory)` section and `MUST` directive verbs.
  - **R3** — Five rule files self-stand with `MUST` / `MUST NOT` / `MUST RUN` gates.
  - **R4** — Role contracts (`docs/agents/*.md`) cross-reference the rule files.
  - **R5** — `audit-orphan-exports.sh --tool-coverage` and `audit-doc-drift.sh check_stale_rules` enforce the R1–R4 contract.
  - **R6** — Stale crate reference sweep in `docs/agents/architect.md`.
  - **R7** — `tool-coverage` and `nightly-clippy` CI jobs.
  - **R8** — Plan §4 acceptance verified; all 8 items ticked.

### Changed
- **`AGENTS.md`** rewritten per R2. Adds the `## Workflow Discipline (Compulsory)` section with `MUST` directive verbs and the workflow tools table.
- **`bin/spiral-context.sh`** extended with `--quick` and `--rules-check` flags.
- **`bin/spiral-pr.sh`** new entry point for the PR workflow (replaces manual `gh pr create` invocations).
- **`justfile`** split `verify` into `verify-fast` (pre-commit) and `verify-rules` (pre-merge / nightly).

### Removed
- **SonarCloud + Spiral-Bot replaced by DeepSource (2026-06-18).** Per [`docs/decisions/0013-deepsource-replaces-sonar-and-spiral-bot.md`](docs/decisions/0013-deepsource-replaces-sonar-and-spiral-bot.md), the SonarCloud free tier does not support Rust and Copilot Autofix is not available on the personal account. Retired: `.github/workflows/spiral-bot.yml` and the entire `bin/spiral-bot/` directory (10 files, ~600 LoC of Bun/TypeScript). The DeepSource quality gate on `main` is the new "green button" — the merge button is greyed out until the DeepSource check is green. Humans still approve the merge. Dependabot security updates (already on) handle dependency vulnerability PRs.

---

## [0.0.0] - 2026-06-16

The bootstrap release. Documentation-only; no public-API changes. Establishes the SSOT hierarchy that all future releases will follow. **Post-bootstrap follow-on packets** (1.6.1, 1.6.3, 1.6.4) shipped later the same day and will appear in the next tagged release.

Full release notes: [`docs/releases/0.0.0-bootstrap.md`](docs/releases/0.0.0-bootstrap.md).

### Added
- **Phase 1.5 — SSOT Restructure (2026-06-16).** New canonical documents and a Group → Phase → Step → Packet vocabulary. The time-based Month / Sprint / Chunk / Item vocabulary is retired.
  - `docs/implementation_tracker.md` (SSOT for status).
  - `docs/active_context.md` (live state).
  - `docs/progress_ledger.md` (append-only change log).
  - `docs/glossary.md` (engine brand names).
  - `docs/decisions/` (ADRs).
  - `docs/agents/` (role contracts).
  - `docs/architecture/` (per-subsystem stubs).
  - `docs/plans/` (multi-step refactor plans).
- **Phase 0 — Foundation.** 20-crate workspace, IPC shell, hello-world render.
- **Phase 1 Step 1.6 — Vortex GC rewrite (2026-06-15).** Mark-sweep GC for Vortex, replacing the refcount-based cycle detector. Packets 1.6.1 (smoke test), 1.6.3 (mark phase), 1.6.4 (Filter runtime), 1.6.5 (sweep phase).
- **Phase 1 Step 1.2 — Vortex tree-walking interpreter (2026-06-14).** Lexer, parser, AST, tree-walking interpreter, console.log, basic expressions.
- **Phase 1 Step 1.1 — Workspace bootstrap (2026-06-12).** 20-crate workspace, Cargo.toml, CI pipeline.
- **Phase 0 Step 0.4 — Multi-process architecture (2026-06-11).** Browser process, per-tab renderer, network process, GPU process. IPC over Unix domain sockets and named pipes.
- **Phase 0 Step 0.3 — Browser chrome (2026-06-10).** Zen-style sidebar tabs, floating URL bar, single-accent-colour theming.
- **Phase 0 Step 0.2 — Hello-world render (2026-06-09).** Vello + wgpu pipeline, "Hello, World!" output.
- **Phase 0 Step 0.1 — Workspace skeleton (2026-06-08).** Cargo.toml, 20-crate skeleton, CI pipeline.
- **Foundation documents:** `README.md`, `LICENSE`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`.
- **Multi-model agent definitions** under `~/.config/opencode/agents/`: architect, implementer, reviewer, tester.
- **GitHub CI:** 11-job pipeline (fmt, clippy, test, build, audit, deny, secrets, wiring, tool-coverage, doc-drift, nightly-clippy) across Linux, macOS, Windows. Clippy runs with `-D warnings`.

### Removed
- **`spiral-html` retired (2026-06-15).** All HTML parsing is in `spiral-fmt` (Fmt). The pre-rename `spiral-html` crate was removed from the workspace.
- **`spiral-layout` → `spiral-gyre` (2026-06-15).** Renamed for the Gyre brand. ADRs 0003 and 0006 document the rename.
- **`spiral-js` → `spiral-vortex` (2026-06-15).** Renamed for the Vortex brand. ADRs 0002 and 0003 document the rename.
- **`html5ever` / `markup5ever` / `tendril` not vendored (2026-06-15).** All HTML parsing is from-spec in `spiral-fmt`.
- **`cssparser` / `selectors` not vendored (2026-06-15).** All CSS parsing is from-spec in `spiral-fmt`.
- **`boa_engine` removed from workspace deps.** `taffy` was never added.

---

## How to read this file

- **Versions are semantic.** Major versions signal a shipped product milestone. Minor versions signal a new user-facing capability. Patch versions signal bug fixes and non-breaking changes.
- **The `[Unreleased]` section** is the canonical "what just landed" feed. Every PR adds a bullet here; the next tagged release moves the bullet into a dated version section.
- **ADRs are the canonical "why" record.** A bullet in the changelog that says "we did X" should have a corresponding ADR in `docs/decisions/` that says "we chose X because Y."
- **The append-only ledger** ([`docs/progress_ledger.md`](docs/progress_ledger.md)) has the per-packet detail that this changelog summarises.

---

## Earlier history

Pre-bootstrap development (2026-06-08 through 2026-06-15) is documented in the `docs/progress_ledger.md` entries. The bootstrap release is the first version that is publicly tagged.
