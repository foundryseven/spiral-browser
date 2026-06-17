# Spiral Browser

![Spiral Browser Logo](resources/icons/logo.png)

> **A fully independent web browser, built from scratch in Rust.**
> Not based on Chromium. Not based on WebKit. Not based on Gecko.
> Every engine we ship carries the Spiral brand.

---

## What is Spiral?

The modern web runs on three browser engines — Blink (Chromium), WebKit (Safari), Gecko (Firefox). The result is a monoculture: shared vulnerabilities, shared standards drift, shared design assumptions. A serious bug in one is a serious bug in all.

**Spiral exists to be a fourth engine, written from scratch in safe Rust, with no vendored browser-engine code anywhere in the tree.**

The bet is straightforward: building a from-scratch browser in 2026 is feasible, the engineering work is tractable, and the result is a system that the people who depend on it can actually inspect. We are not building a faster Chromium. We are building a different thing entirely.

### Goals

- **Engine independence.** No `html5ever`, no `Servo`, no Blink, no WebKit, no Gecko. Every parser, layout engine, and JavaScript engine that touches a web page is written by us in Spiral-native Rust.
- **Memory safety end-to-end.** The render path is 100% safe Rust. The only `unsafe` in the tree is in audited dependencies and in FFI shims with narrow surface area.
- **From-spec implementations.** Our HTML5 parser follows the WHATWG HTML Living Standard section-by-section. Our CSS parser follows the CSS Syntax Level 3 module. Our layout engine is the CSS Display + Box Model specs. Comments in the code cite the spec section.
- **Capability-typed page context.** Every cross-origin or sensitive operation is gated by a compile-time-checked capability. A page cannot reach the network unless it holds a `Network` capability. This is the *Filter* and *Context* work — research-grade in 2026, but the design is end-state.
- **A working browser, eventually.** Spiral is a real product, not a research project. The Phase 1 line delivers a browser that can render HTML+CSS and run trivial JavaScript. The Phase 2 line delivers DOM depth. The Phase 5 line delivers the capability-typed runtime. The Phase 9 line ships the production browser.
- **Open source, MPL-2.0.** All code is published. All decisions are ADRs. All progress is in the public ledger.

### Non-goals

- **Not a Chromium competitor on performance.** A from-scratch engine will not match V8 in 2026. The aim is *correctness* and *independence*, not benchmark leadership.
- **Not a privacy browser.** Spiral is a general-purpose browser. The capability types *enable* a strong privacy story; we are not building one in the engine itself.
- **Not a drop-in Chrome replacement.** Sites that depend on Chrome-only Web Platform features will not work. We track the standards, not the implementation shortcuts.
- **Not a proprietary codebase.** Everything is public from day one. There is no private fork.

### Core principles

1. **Independence over performance.** A slower browser you can inspect beats a faster browser you cannot.
2. **From-spec over upstream.** If the spec says so, we do what the spec says. If the spec is ambiguous, we file an issue.
3. **Memory safety as architecture, not patch.** No `unsafe` in the engines. No `unsafe` in the parser. `unsafe` only at the FFI boundary, and every `unsafe` block has a safety comment.
4. **Per-packet completeness.** A `pub` symbol is not done when it compiles; it is done when an external consumer uses it. The `audit-orphan-exports.sh` gate enforces this.
5. **The brand belongs to the engine.** A new engine gets a name. A wire protocol does not.

---

## The Spiral brand identity

Three engines in the Spiral stack bear the brand. Each is fully in-house Rust.

| Brand | Crate | One-liner | Replaces |
|-------|-------|-----------|----------|
| **Gyre** | `spiral-gyre` | Layout — box model, block, flex, grid, all in-house | Taffy, Servo layout |
| **Vortex** | `spiral-vortex` | JavaScript — lexer, parser, AST, bytecode VM, GC, JIT (future) | V8, JSC, SpiderMonkey |
| **Fmt** *(the "Forge")* | `spiral-fmt` | From-spec HTML5 + CSS Syntax Level 3 parsers | html5ever, cssparser |

Two more are in design and partial shipping:

| Brand | Crate | One-liner |
|-------|-------|-----------|
| **Filter** | `spiral-filter` | Compile-time HTML/CSS policy. Decides at parse time whether a request or selector is allowed. |
| **Context** | `spiral-context` | Capability-typed page context. `Context<'brand, Mode>`, `CapabilitySet<'brand>`. The "Bet 1" runtime. |

Everything else (`spiral-core`, `spiral-ipc`, `spiral-dom`, `spiral-render`, `spiral-paint`, `spiral-gpu`, `spiral-network`, `spiral-net`, `spiral-crypto`, `spiral-imagedecoder`, `spiral-sandbox`, `spiral-ui`, `spiral-theme`, `spiral-browser`, the deprecated `spiral-css` shim) is plumbing. It wires the engines together. The brand belongs to the engine, not the wire.

The full canonical mapping is at [`docs/glossary.md`](docs/glossary.md).

---

## Status

**Alpha.** Phase 0 (workspace, IPC shell, hello-world render) is complete. Phase 1.5 (SSOT Restructure) shipped at `v0.0.0-bootstrap` on 2026-06-16. **Phase 1 (Engines Foundation) is in flight** — Vortex's first slice (`console.log` and basic expressions), Fmt's HTML5 + CSS parsers (8 insertion modes + 8 CSS modules), Gyre's box model, and the Filter runtime are landing in 1.6.x packets.

**Phase 2 (Engines Depth)** is also in flight — DOM quirks mode propagation (Packets 2.1.1 ✅ and 2.1.2 ✅), fragment parsing, adoption agency algorithm, active formatting elements list, foster parenting.

The **no-code-agentic workflow refactor (R1–R8)** shipped on 2026-06-18. The agent-driven workflow is the canonical surface for the next packet: session start runs `bin/spiral-context.sh`, work lands via `just verify-fast` + `just verify-rules`, and PRs go out via `bin/spiral-pr.sh`. See the [Workflow](#workflow) section below.

The single source of truth for what is built, in flight, and missing is [`docs/implementation_tracker.md`](docs/implementation_tracker.md) (Group → Phase → Step → Packet). The time-based Month / Sprint / Chunk / Item vocabulary is retired as of 2026-06-16.

---

## Quickstart

```bash
# Clone (replace with the real URL before publishing)
git clone https://github.com/foundryseven/spiral-browser.git
cd spiral-browser

# Build all 20 crates
cargo build

# Run the full test suite
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --all -- --check
```

The canonical end-to-end gate is `just verify` (= `just verify-fast` + `just verify-rules`). It is the same gate CI runs. See the [Workflow](#workflow) section below.

Detailed build instructions: [`BUILD.md`](BUILD.md).
Test strategy and conventions: [`TESTING.md`](TESTING.md).
Common errors: [`ERRORS.md`](ERRORS.md).

---

## Architecture at a glance

Spiral uses a multi-process architecture with separate processes for browsing, rendering, networking, and GPU operations. Each renderer is isolated per-tab for security.

```
┌──────────────────────────────────────────────────────────┐
│                    Browser Process                        │
│  ┌─────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐ │
│  │ Tab Mgr │ │ IPC Rtr  │ │ UI Chrome│ │ Config Mgr   │ │
│  └─────────┘ └──────────┘ └──────────┘ └──────────────┘ │
├──────────────────────────────────────────────────────────┤
│                     IPC Layer                            │
│  Unix Domain Sockets (Linux/macOS) · Named Pipes (Win)   │
│  bincode length-prefixed framing · tokio async runtime   │
├────────┬────────┬────────────┬──────────────────────────┤
│Render-1│Render-2│  Network   │         GPU              │
│ (Tab1) │ (Tab2) │  Process   │       Process            │
│  Fmt   │  Fmt   │   HTTP/1.1 │     wgpu Device/Queue    │
│  Gyre  │  Gyre  │   HTTP/2   │     Vello Renderer       │
│  Vortex│  Vortex│   DNS/TLS  │     Texture Management   │
│  Paint │  Paint │   Cookies  │     Swap Chain           │
└────────┴────────┴────────────┴──────────────────────────┘
```

The renderer pipeline:

```
Network Response (HTML bytes)
    → Fmt (HTML tokeniser + tree builder) → DOM
    → Fmt (CSS parser) → Stylesheet
    → Style Resolution → Computed Styles
    → Gyre (layout) → Layout Tree
    → Paint (display list construction) → Display List
    → Render (Vello + wgpu) → GPU texture → screen
```

Full architecture with per-crate responsibilities, data flows, security model, and configuration: [`ARCHITECTURE.md`](ARCHITECTURE.md).

The 20-crate workspace is described in [`docs/glossary.md`](docs/glossary.md) and [`PLAN.md`](PLAN.md).

---

## Workflow

The agent-driven workflow is the canonical surface for landing work. It is enforced by `just` and `bin/` scripts, and mirrored in GitHub Actions.

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
| `./scripts/audit-doc-drift.sh` | Walks the SSOT docs and flags stale crate refs, retired vocabulary, and the R5 rule-file contract. | **Before claiming "docs done."** |

Full workflow contract: [`AGENTS.md`](AGENTS.md) § Workflow Discipline and the rule files under [`.spiral/rules/`](.spiral/rules/).

CI runs 11 jobs on every push to `main` and on every PR: `fmt`, `clippy` × 3 OSes, `test` × 3, `build` × 3, `audit`, `deny`, `secrets`, `wiring`, `tool-coverage`, `doc-drift`, `nightly-clippy`. The workflow is at [`.github/workflows/ci.yml`](.github/workflows/ci.yml).

---

## Project documents

### Front door

| Document | Purpose |
|----------|---------|
| [`README.md`](README.md) | This file. Project vision, status, quickstart, brand identity. |
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | System design, process model, per-crate responsibilities, data flows, security model, configuration. |
| [`PLAN.md`](PLAN.md) | Strategic plan: engine identity, vision, principles, crate structure, IPC protocol. |
| [`ROADMAP.md`](ROADMAP.md) | One-page Group → Phase index with project goals narrative. |
| [`CODEX.md`](CODEX.md) | Quick reference for LLM agents — a session-start cheatsheet, not a re-statement of the above. |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history. |
| [`SECURITY.md`](SECURITY.md) | Vulnerability disclosure policy and supported versions. |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | How to land a packet, run the gates, name a branch, open a PR. |
| [`LICENSE`](LICENSE) | MPL-2.0 licence terms. |

### Build and operation

| Document | Purpose |
|----------|---------|
| [`BUILD.md`](BUILD.md) | Platform-specific build instructions and dependencies. |
| [`TESTING.md`](TESTING.md) | Test strategy, commands, and conventions. |
| [`ERRORS.md`](ERRORS.md) | Common errors and their fixes. |

### Source of truth (SSOT) — for the agent workflow

| Document | Purpose |
|----------|---------|
| [`AGENTS.md`](AGENTS.md) | AI agent instructions for this repository (canonical workflow contract). |
| [`docs/active_context.md`](docs/active_context.md) | Live state: current phase, blockers, "do not touch" zones. |
| [`docs/implementation_tracker.md`](docs/implementation_tracker.md) | Group → Phase → Step → Packet status (SSOT). |
| [`docs/progress_ledger.md`](docs/progress_ledger.md) | Append-only change log. |
| [`docs/glossary.md`](docs/glossary.md) | Engine brand names (Gyre, Vortex, Fmt, Forge, Filter, Context). |
| [`docs/decisions/`](docs/decisions/) | ADRs (cross-cutting decisions; link from tracker). |
| [`docs/agents/`](docs/agents/) | Role contracts (implementer, reviewer, architect, tester, security, release, onboarding). |
| [`docs/architecture/`](docs/architecture/) | Per-subsystem architecture stubs. |
| [`docs/plans/`](docs/plans/) | Multi-step refactor plans (e.g. `no-code-agentic-refactor.md`). |
| [`.spiral/rules/`](.spiral/rules/) | Rule files (architecture, coding-standards, performance, testing, unsafe-standards, workflow, doc-drift-prevention). |
| [`bin/README.md`](bin/README.md) | Workflow scripts in `bin/`. |

### Specs and gaps

| Document | Purpose |
|----------|---------|
| [`specs/GAP_ANALYSIS.md`](specs/GAP_ANALYSIS.md) | P0/P1/P2/P3 gap tracker across 4 engine sub-domains (spec-only; status lives in the tracker). |

---

## Contributing

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) for the workflow contract. The short version: branch, code, run `just verify`, open a PR via `bin/spiral-pr.sh <packet-id>`. The CI gate runs the same `just verify` end-to-end, so a green local gate is a green CI gate.

If you are an AI agent landing work, start with [`AGENTS.md`](AGENTS.md) and then `bin/spiral-context.sh` at session start.

---

## Security

Read [`SECURITY.md`](SECURITY.md) for the supported versions and the disclosure process. Security issues go to the private channels listed there, **not** to GitHub issues.

---

## License

MPL-2.0. See [`LICENSE`](LICENSE).
