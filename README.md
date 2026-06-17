# Spiral Browser

![Spiral Browser Logo](resources/icons/logo.png)

A fully independent web browser built from scratch in Rust. Not based on Chromium, WebKit, or Gecko. Features a custom rendering engine, custom JavaScript engine, and a Zen-browser-inspired UI with vertical sidebar tabs, a floating URL bar, and single-accent-colour theming.

## Status

**Alpha — Phase 1 (Engines Foundation) in flight; Phase 1.5 SSOT Restructure shipped at v0.0.0-bootstrap.** Phase 0 (workspace + IPC shell + hello-world render) is done. Current work is in Phase 1 Step 1.6 (Vortex GC rewrite + follow-on packets). The single source of truth for what is built, in flight, and missing is [`docs/implementation_tracker.md`](docs/implementation_tracker.md) (Group → Phase → Step → Packet). Time-based Month / Sprint / Chunk / Item vocabulary is retired as of 2026-06-16.

## Quickstart

```bash
cargo build                # Build all crates
cargo test --workspace     # Run all tests
cargo clippy --workspace   # Lint check
cargo fmt --check          # Format check
```

Detailed build instructions: see `BUILD.md`.
Test strategy and conventions: see `TESTING.md`.

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
| `AGENTS.md` | AI agent instructions for this repository |
| `docs/active_context.md` | Live state: current phase, blockers |
| `docs/implementation_tracker.md` | Group → Phase → Step → Packet status (SSOT) |
| `docs/progress_ledger.md` | Append-only change log |
| `docs/archives/phase1-tasks.md` | Granular Phase 1 task breakdown (archived) |
| `docs/glossary.md` | Engine brand names (Gyre, Vortex, Fmt, Forge) |
| `docs/decisions/` | ADRs (cross-cutting decisions) |
| `docs/agents/` | Role contracts (implementer, reviewer, architect, …) |
| `docs/architecture/` | Per-subsystem architecture stubs |
| `specs/GAP_ANALYSIS.md` | P0/P1/P2/P3 gap tracker across 4 engine sub-domains (spec-only) |
| `CHANGELOG.md` | Release history |
| `SECURITY.md` | Vulnerability disclosure process |
| `LICENSE` | MPL-2.0 licence terms |

## License

MPL-2.0. See `LICENSE`.
