# Spiral Browser — Quick Reference for LLMs

## Project Overview
- **Name:** Spiral Browser
- **Language:** Rust (edition 2021)
- **License:** MPL-2.0
- **Platforms:** Windows, macOS, Linux
- **Scope:** Independent browser (not Chromium/WebKit/Gecko)
- **Status (SSOT):** [`docs/implementation_tracker.md`](docs/implementation_tracker.md) (Group → Phase → Step → Packet)
- **Active state:** [`docs/active_context.md`](docs/active_context.md)
- **Workflow contract:** [`AGENTS.md`](AGENTS.md) § Workflow Discipline + [`.spiral/rules/`](.spiral/rules/)

## Workflow Tools (canonical surface)
- **Session start:** `bin/spiral-context.sh` (or `just context`) — prints the 5–7 most relevant files. Replaces 6 manual reads.
- **Pre-commit gate:** `just verify-fast` (fmt + clippy + test + build).
- **Pre-merge + CI nightly gate:** `just verify-rules` (nightly clippy + both audit scripts).
- **End-to-end canonical gate:** `just verify` (= `verify-fast` + `verify-rules`).
- **PR workflow:** `bin/spiral-pr.sh <packet-id>` (runs pre-flight checks, pushes, opens PR). Do not invoke `gh pr create` directly.
- **Audit scripts:**
  - `./scripts/audit-orphan-exports.sh` — `pub` symbols with no external consumer (exit 1 = blocker).
  - `./scripts/audit-orphan-exports.sh --tool-coverage` — `bin/` and `scripts/` tools not named in a rule file (exit 1 = blocker).
  - `./scripts/audit-doc-drift.sh` — SSOT doc inconsistencies (stale crate refs, retired vocabulary, R5 rule-file contract).

See [`AGENTS.md`](AGENTS.md) § Workflow Discipline for the full list of mandatory gates.

## Repository Structure
```
├── Cargo.toml              # Workspace root (20 members)
├── AGENTS.md               # LLM instructions (canonical workflow contract)
├── ARCHITECTURE.md         # System design (canonical)
├── PLAN.md                 # Implementation plan
├── ROADMAP.md              # Phase index
├── TESTING.md              # Test guide
├── BUILD.md                # Build instructions
├── ERRORS.md               # Common errors and fixes
├── CONTRIBUTING.md         # Contribution process
├── CHANGELOG.md            # Release history
├── SECURITY.md             # Vulnerability disclosure
├── LICENSE                 # MPL-2.0 licence terms
├── .spiral/rules/          # Rule files (architecture, coding-standards, performance, testing, unsafe-standards, workflow, doc-drift-prevention)
├── bin/                    # Workflow scripts (spiral-context.sh, spiral-pr.sh)
├── scripts/                # Build-time tooling (audit scripts)
├── docs/                   # SSOT (tracker, ledger, active context, plans, ADRs, role contracts)
│   ├── active_context.md
│   ├── implementation_tracker.md
│   ├── progress_ledger.md
│   ├── agents/             # Role contracts
│   ├── decisions/          # ADRs
│   ├── architecture/       # Per-subsystem stubs
│   ├── plans/              # Multi-step refactor plans
│   ├── glossary.md
│   ├── system_architecture.md
│   └── archives/           # Historical artefacts (do not edit)
├── specs/                  # Spec-only documents (status moved to tracker)
├── resources/              # Static assets (icons, fonts)
├── crates/                 # 20-crate workspace
│   ├── spiral-core/        # Shared types
│   ├── spiral-ipc/         # Cross-process messaging
│   ├── spiral-dom/         # DOM tree
│   ├── spiral-fmt/         # HTML5 tokeniser + tree builder, CSS parser (from-spec; no html5ever, no cssparser)
│   ├── spiral-css/         # Deprecated shim → spiral-fmt
│   ├── spiral-gyre/        # Gyre — custom layout
│   ├── spiral-vortex/      # Vortex — from-scratch JS
│   ├── spiral-context/     # Capability types
│   ├── spiral-filter/      # Compile-time policy
│   ├── spiral-network/     # HTTP / DNS / TLS
│   ├── spiral-net/         # Low-level networking
│   ├── spiral-crypto/      # Crypto primitives
│   ├── spiral-render/      # Vello-based renderer
│   ├── spiral-paint/       # Display list construction
│   ├── spiral-gpu/         # wgpu integration
│   ├── spiral-imagedecoder/# Image codecs
│   ├── spiral-sandbox/     # OS sandbox profiles
│   ├── spiral-ui/          # Browser chrome
│   ├── spiral-theme/       # Design tokens
│   └── spiral-browser/     # Binary surface (entry point)
└── .github/workflows/      # 11-job CI pipeline (ci.yml)
```mon errors
├── CODEX.md                # This file
├── CONTRIBUTING.md         # Contribution guide
├── crates/                 # 20 Rust crates
│   ├── spiral-core/        # Shared types
│   ├── spiral-ipc/         # IPC transport (Unix / Win32)
│   ├── spiral-dom/         # DOM tree
│   ├── spiral-fmt/         # HTML5 tokeniser + tree builder, CSS parser (from-spec; no html5ever, no cssparser)
│   ├── spiral-css/         # Deprecated shim → spiral-fmt
│   ├── spiral-gyre/        # Gyre — custom layout (block, flex, grid)
│   ├── spiral-paint/       # Display list
│   ├── spiral-render/      # Vello + wgpu
│   ├── spiral-gpu/         # GPU abstraction
│   ├── spiral-vortex/      # Vortex — from-scratch JS engine
│   ├── spiral-context/     # Capability-typed API (Bet 1)
│   ├── spiral-filter/      # Compile-time ad/tracker policy (Bet 3)
│   ├── spiral-network/     # HTTP (hyper + hickory-dns + rustls)
│   ├── spiral-net/         # Thin wrapper over spiral-network
│   ├── spiral-crypto/      # SHA-2, getrandom
│   ├── spiral-imagedecoder/# PNG / JPEG / WebP / AVIF
│   ├── spiral-sandbox/     # OS sandbox profiles
│   ├── spiral-ui/          # GPU-rendered browser chrome
│   ├── spiral-theme/       # Design tokens
│   └── spiral-browser/     # Binary entry point
├── docs/                   # SSOT documents
│   ├── active_context.md       # Live phase state
│   ├── implementation_tracker.md# Group → Phase → Step → Packet (SSOT)
│   ├── progress_ledger.md      # Append-only change log
│   ├── system_architecture.md  # Architecture delta file
│   ├── glossary.md             # Engine brand names
│   ├── decisions/              # ADRs
│   ├── agents/                 # Role contracts
│   ├── architecture/           # Per-subsystem stubs
│   ├── audit-sprint-m4.md      # M4 novelty audit
│   ├── audits/                 # Functional baseline audits
│   ├── baseline-warnings.md    # cargo check warning drift
│   ├── innovations/            # 70-idea backlog
│   ├── plans/                  # Iteration strategy
│   ├── releases/               # Release notes
│   ├── security/               # Post-mortems
│   └── archives/               # Historical task lists
├── specs/                  # Spec-only documents (status lives in tracker)
│   └── GAP_ANALYSIS.md     # P0/P1/P2/P3 gap tracker
├── scripts/                # Audits and helpers (e.g. audit-orphan-exports.sh)
└── .spiral/rules/          # Architecture, coding-standards, testing rules
```

## Architecture at a glance

| Subsystem | Crate | Engine Brand | Notes |
|-----------|-------|--------------|-------|
| Layout | `spiral-gyre` | **Gyre** | Custom block / flex / grid. No Taffy. |
| JavaScript | `spiral-vortex` | **Vortex** | From-scratch Rust. `rusty_v8` behind `v8` feature for CI oracle only. |
| Parsing | `spiral-fmt` | **Fmt** | From-spec HTML5 tokeniser + tree builder + CSS parser. |
| Cascade | `spiral-css` | (deprecated) | Shim that forwards to `spiral-fmt`. New code uses `spiral-fmt` directly. |
| Network | `spiral-network` / `spiral-net` | — | hyper + hickory-dns + rustls. |
| Crypto | `spiral-crypto` | — | SHA-2, getrandom. |
| IPC | `spiral-ipc` | — | bincode-framed; Unix sockets / Win32 pipes. |
| Render | `spiral-render` + `spiral-paint` + `spiral-gpu` | — | Vello + wgpu. |
| Image | `spiral-imagedecoder` | — | PNG / JPEG / WebP / AVIF. |
| Sandbox | `spiral-sandbox` | — | Linux Landlock + seccomp, macOS Seatbelt, Win32 restricted tokens. |
| UI | `spiral-ui` + `spiral-theme` | — | Zen-style: sidebar tabs, floating URL bar. |
| Cross-cutting | `spiral-context` | — | Capability-typed API surface (Bet 1). |
| Cross-cutting | `spiral-filter` | — | Compile-time HTML/CSS policy engine (Bet 3). |

## Phase 1 — Engines Foundation (in flight)

See `docs/implementation_tracker.md` for the live Group → Phase → Step → Packet checklist. Phase 1 Steps 1.1–1.5 and Step 1.6 packets 1.6.1, 1.6.3, 1.6.4 are shipped. Step 1.6 packets 1.6.2 (Vortex), 1.6.5 (Gyre box model) are the next-up packets. Packets 1.6.6–1.6.8 retired to Step 2.8 (2.8.1 adoption agency, 2.8.2 AFE, 2.8.3 foster parenting). Phase 2 (Core Engine) is the next phase.

| Subsystem | Crate | Parser / Engine | Status |
|-----------|-------|------------------|--------|
| HTML | `spiral-fmt` (from-spec tokeniser + tree builder) | HTML5 (owned) | ✅ |
| CSS | `spiral-fmt` (from-spec parser) | CSS Syntax Level 3 (owned) | ✅ |
| Layout | `spiral-gyre` (Gyre) | Custom block / flex / grid | 🚧 in flight |
| JavaScript | `spiral-vortex` (Vortex) | From-scratch Rust | 🚧 tree-walker → bytecode VM |
| Network | `spiral-network` | hyper + hickory-dns + rustls | 🚧 Step 3.1 |
| Crypto | `spiral-crypto` | SHA-2, getrandom | ✅ |
| IPC | `spiral-ipc` | bincode, Unix / Win32 | ✅ |
| Render | `spiral-render` + `spiral-paint` + `spiral-gpu` | Vello + wgpu | ✅ minimal |
| Image | `spiral-imagedecoder` | PNG / JPEG / WebP / AVIF | 🚧 |
| UI | `spiral-ui` + `spiral-theme` | GPU chrome | 🚧 |
| Sandbox | `spiral-sandbox` | OS-level (Phase 4) | ☐ |
| Context | `spiral-context` | Capability types | ✅ skeleton + types (Packet 1.6.2 = Vortex wiring) |
| Filter | `spiral-filter` | Compile-time policy | ✅ runtime shipped (Packet 1.6.4; ADR 0005) |

## Important removals (2026-06-15)

- `spiral-html` retired — all HTML parsing is in `spiral-fmt`.
- `html5ever` / `markup5ever` / `tendril` not vendored. See `docs/decisions/0001-css-parser-spiral-fmt.md`.
- `cssparser` / `selectors` not vendored.
- `boa_engine` removed from workspace deps. `taffy` was never added.

## Key ADRs (read before changing architecture)

- `0001-css-parser-spiral-fmt.md` — from-spec parser in `spiral-fmt`.
- `0002-vortex-from-scratch.md` — Vortex is a from-scratch JS engine.
- `0003-gyre-rename.md` — `spiral-layout` → `spiral-gyre`.
- `0004-resolver-trait-async-design.md` — async `Resolver` trait shape.

## Conventions

- Be precise. Quote `file:line` for code references.
- Follow the `AGENTS.md` Decision Protocol and Wiring & Integration rule.
- Run `./scripts/audit-orphan-exports.sh` before claiming "wiring complete."
- Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` before any "done" claim.
- Australian English spelling in prose, docstrings, and comments (`initialise`, `optimise`, `colour`, `behaviour`).
