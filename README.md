# Spiral Browser

A fully independent web browser built from scratch in Rust. Not based on Chromium, WebKit, or Gecko. Features a custom rendering engine, custom JavaScript engine, and a Zen-browser-inspired UI with vertical sidebar tabs, a floating URL bar, and single-accent-colour theming.

## Status

**Alpha — Phase 1 (Foundation).** Cargo workspace, IPC shell, and a "Hello World" end-to-end render target. See `PLAN.md` for the full roadmap and `ROADMAP.md` for the current month-by-month plan.

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

Multi-process design: browser process, per-tab renderer processes, dedicated network and GPU processes. IPC over Unix domain sockets (Linux/macOS) and named pipes (Windows). Full architecture in `ARCHITECTURE.md`.

```
spiral-core  →  spiral-ipc  →  spiral-dom  →  spiral-html
                     │            │              spiral-css
                      │            │              spiral-gyre
                      │            │              spiral-vortex
                     │            │
                     └─→  spiral-browser  ←  spiral-ui  ←  spiral-theme
                                    spiral-network, spiral-net, spiral-crypto
                                    spiral-render, spiral-paint, spiral-gpu
                                    spiral-imagedecoder
                                    spiral-sandbox
```

## Project Documents

| Document | Purpose |
|----------|---------|
| `CODEX.md` | Quick reference for LLMs and contributors |
| `ARCHITECTURE.md` | System design and data flows |
| `PLAN.md` | Implementation plan and crate structure |
| `ROADMAP.md` | Phase-by-phase development timeline |
| `BUILD.md` | Platform-specific build instructions |
| `TESTING.md` | Test strategy and commands |
| `ERRORS.md` | Common errors and fixes |
| `CONTRIBUTING.md` | Contribution process |
| `AGENTS.md` | AI agent instructions for this repository |
| `docs/active_context.md` | Live state: current sprint, blockers |
| `docs/progress_ledger.md` | Append-only change log |
| `docs/phase1-tasks.md` | Granular Phase 1 task breakdown |
| `docs/system_architecture.md` | Architecture pointer + deltas |
| `CHANGELOG.md` | Release history |
| `SECURITY.md` | Vulnerability disclosure process |
| `LICENSE` | MPL-2.0 licence terms |

## License

MPL-2.0. See `LICENSE`.
