# CODEX — Spiral Browser

> **Quick-reference for LLM agents and human contributors. NOT a re-statement of the README.**
> If you are looking for project vision, brand identity, or quickstart: read [`README.md`](README.md) first.
> If you are looking for system design and per-crate responsibilities: read [`ARCHITECTURE.md`](ARCHITECTURE.md).
> If you are looking for the implementation plan: read [`PLAN.md`](PLAN.md).
> If you are looking for the phase index: read [`ROADMAP.md`](ROADMAP.md).
> This file is the **session-start cheatsheet** for an agent that has already read those four.

---

## Project at a glance

- **Name:** Spiral Browser
- **Language:** Rust (edition 2021)
- **License:** MPL-2.0
- **Platforms:** Windows, macOS, Linux
- **Scope:** Independent browser. Not Chromium, not WebKit, not Gecko. Three in-house engines: **Gyre** (layout), **Vortex** (JavaScript), **Fmt** (HTML+CSS parsers).
- **Workspace:** 20 crates
- **CI:** 11 jobs on every push to `main` and on every PR (`.github/workflows/ci.yml`)
- **Methodology:** LLM-assisted, human-directed, adversarially reviewed. See [`docs/methodology.md`](docs/methodology.md).

| Status (SSOT) | File |
|---------------|------|
| What is in flight? | [`docs/implementation_tracker.md`](docs/implementation_tracker.md) (Group → Phase → Step → Packet) |
| What just shipped? | [`docs/progress_ledger.md`](docs/progress_ledger.md) (last 3 entries) |
| Live phase state | [`docs/active_context.md`](docs/active_context.md) |
| Why was X decided? | [`docs/decisions/`](docs/decisions/) (ADRs) |
| Engine brand identity | [`docs/glossary.md`](docs/glossary.md) |
| Methodology | [`docs/methodology.md`](docs/methodology.md) |
| Failure log | [`docs/failures/`](docs/failures/) |
| Workflow contract | [`AGENTS.md`](AGENTS.md) |
| Rule files | [`.spiral/rules/`](.spiral/rules/) |
| Role contracts | [`docs/agents/`](docs/agents/) |

---

## Workflow tools (canonical surface)

The full contract is in [`AGENTS.md`](AGENTS.md) § Workflow Discipline. The minimum a session must do:

1. **Session start:** `bin/spiral-context.sh` (or `bin/spiral-context.sh <packet-id>` if a packet is named). Add `--rules-check` to also run the R5 audit fast-scan.
2. **Mid-cycle (one crate):** `just test-fast <crate> [pattern]` to skip the full-workspace overhead.
3. **After a `pub` API change:** `just test-with-deps <crate>` (reverse-dep fan-out).
4. **Before claiming complete:** `just verify-packet <crate>`.
5. **Pre-commit / pre-merge:** `./scripts/audit-orphan-exports.sh` AND `./scripts/audit-doc-drift.sh` (both must exit 0).
6. **End of session (PR wanted):** `bin/spiral-pr.sh <packet-id>`. Do not invoke `gh pr create` directly.

End-to-end canonical gate: `just verify` (= `just verify-fast` + `just verify-rules`).

Audit scripts:

- `./scripts/audit-orphan-exports.sh` — `pub` symbols with no external consumer (exit 1 = blocker).
- `./scripts/audit-orphan-exports.sh --tool-coverage` — `bin/` and `scripts/` tools not named in a rule file (exit 1 = blocker).
- `./scripts/audit-doc-drift.sh` — SSOT doc inconsistencies (stale crate refs, retired vocabulary, R5 rule-file contract).

---

## Brand identity — quick map

The brand belongs to the engines, not the wire. New engine = new brand. Wire protocol = no brand.

| Brand | Crate | What it does | Replaces |
|-------|-------|--------------|----------|
| **Gyre** | `spiral-gyre` | Layout — box model, block, flex, grid | Taffy, Servo layout |
| **Vortex** | `spiral-vortex` | JavaScript — lexer, parser, AST, bytecode VM, GC, future JIT | V8, JSC, SpiderMonkey |
| **Fmt** *(the Forge)* | `spiral-fmt` | From-spec HTML5 + CSS Syntax 3 parsers | html5ever, cssparser, selectors |
| **Filter** | `spiral-filter` | Compile-time HTML/CSS policy | (research-grade; no upstream equivalent) |
| **Context** | `spiral-context` | Capability-typed page context (`Context<'brand, Mode>`, `CapabilitySet<'brand>`) | (research-grade; "Bet 1" runtime) |

Plumbing (no brand): `spiral-core`, `spiral-ipc`, `spiral-dom`, `spiral-render`, `spiral-paint`, `spiral-gpu`, `spiral-network`, `spiral-net`, `spiral-crypto`, `spiral-imagedecoder`, `spiral-sandbox`, `spiral-ui`, `spiral-theme`, `spiral-browser`, and the deprecated `spiral-css` shim (forwards to `spiral-fmt`).

Full mapping: [`docs/glossary.md`](docs/glossary.md).

---

## Repository structure (20-crate workspace)

```
spiral-core         # shared types (BrowserConfig, TabId, IPCMessage, Error)
spiral-ipc          # cross-process messaging (UDS / named pipes, bincode, tokio)
spiral-dom          # DOM tree (Node, Element, Document; arena-allocated)
spiral-fmt          # **Fmt** — HTML5 tokeniser + tree builder, CSS parser (from-spec)
spiral-css          # Deprecated shim → spiral-fmt
spiral-gyre         # **Gyre** — custom layout engine (block, flex, grid)
spiral-vortex       # **Vortex** — from-scratch JavaScript engine
spiral-context      # **Context** — capability-typed page context (Bet 1)
spiral-filter       # **Filter** — compile-time HTML/CSS policy (Bet 3)
spiral-network      # HTTP client (hyper + hickory-dns)
spiral-net          # TLS + DNS resolution wrappers (rustls, hickory-dns)
spiral-crypto       # TLS primitives wrapper
spiral-render       # Vello + wgpu render path
spiral-paint        # Display list construction
spiral-gpu          # wgpu integration
spiral-imagedecoder # PNG / JPEG / WebP / AVIF
spiral-sandbox      # OS sandbox profiles (Landlock / Seatbelt / Job Object)
spiral-ui           # Browser chrome (sidebar tabs, floating URL bar)
spiral-theme        # Zen-style design tokens
spiral-browser      # Binary entry point (the actual product)
```

The dependency hierarchy (no crate depends "up"):

```
spiral-core  →  spiral-ipc  →  spiral-dom  →  spiral-fmt  (Fmt)
                                     │        spiral-gyre  (Gyre)
                                     │        spiral-vortex (Vortex)
                                     │        spiral-context (Context)
                                     │        spiral-filter  (Filter)
                                     │
                spiral-browser  ←  spiral-ui  ←  spiral-theme
                                  spiral-network, spiral-net, spiral-crypto
                                  spiral-render, spiral-paint, spiral-gpu
                                  spiral-imagedecoder
                                  spiral-sandbox
```

---

## Phase 1 — Engines Foundation (in flight)

| Context | Crate | Capability | Status |
|---------|-------|------------|--------|
| HTML+CSS | `spiral-fmt` | Fmt — from-spec tokeniser + tree builder (8 insertion modes), CSS parser (8 modules) | ✅ |
| JS | `spiral-vortex` | Vortex — tree-walking interpreter (lex → parse → AST → walk), mark-sweep GC, console.log | ✅ first slice (Packet 1.6.x) |
| Layout | `spiral-gyre` | Gyre — box model + block layout | ✅ Phase 1; flex/grid in Phase 2 |
| Policy | `spiral-filter` | Filter — runtime policy engine | ✅ Packet 1.6.4 |
| Capability types | `spiral-context` | Context — capability-typed API surface | ✅ skeleton + types (Packet 1.6.2 = Vortex wiring) |

---

## Important removals (2026-06-15)

- `spiral-html` retired — all HTML parsing is in `spiral-fmt` (Fmt).
- `html5ever` / `markup5ever` / `tendril` not vendored. See `docs/decisions/0001-css-parser-spiral-fmt.md`.
- `cssparser` / `selectors` not vendored.
- `taffy` was never added. Gyre is in-house from day one.
- `boa_engine` removed from workspace deps.

If you see a pre-2026-06-15 reference to `spiral-html`, `spiral-layout`, `spiral-js`, or any of the vendored crates, it is stale. Fix it via the `audit-doc-drift.sh` findings.

---

## Key ADRs (read before changing architecture)

- `0001-css-parser-spiral-fmt.md` — from-spec parser in `spiral-fmt` (Fmt). The "Fork 1-B" decision.
- `0002-vortex-from-scratch.md` — Vortex is a from-scratch JS engine, not a V8 wrapper.
- `0003-gyre-rename.md` — `spiral-layout` → `spiral-gyre` (the Gyre brand).
- `0004-resolver-trait-async-design.md` — async `Resolver` trait shape.
- `0005-filter-runtime-design.md` — Filter policy runtime.
- `0006-cross-cutting-features.md` — Wiring & Integration rule (a `pub` symbol is not done when it compiles; it is done when an external consumer imports it).

---

## Conventions

- File editing: read the file first; never modify `Cargo.lock` manually; keep imports sorted (std, external, internal); use `snake_case` for functions, `PascalCase` for types; prefer `?` over `.unwrap()` in library code; add `#[must_use]` to functions that return important values.
- Crate boundaries: never depend "up" the dependency graph. Check `Cargo.toml` before adding a dependency. Prefer re-exporting from `spiral-core` for shared types.
- Commit messages: `type(scope): description`. Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`. Scopes: `core`, `ipc`, `fmt`, `css`, `gyre`, `render`, `dom`, `vortex`, `net`, `network`, `ui`, `theme`, `browser`, `sandbox`, `filter`, `context`, `crypto`. (Old `js` and `layout` scopes are retired — use `vortex` and `gyre`.)
- Wiring: a `pub` symbol is not done when it compiles; it is done when at least one consumer outside the home crate imports it. The `audit-orphan-exports.sh` gate enforces this.
- Testing: every public function needs a unit test; integration tests go in `tests/` per crate; run `cargo test --workspace` before committing; run `cargo clippy --workspace -- -D warnings` for lint checks.
- Novelty claims: any claim of "novel", "first", "unique", "no prior art" **must** be verified by a research agent before committing. The M4 audit methodology (`docs/audit-sprint-m4.md`) is the canonical standard. The Context and Filter work is research-grade; the Gyre and Vortex work is from-scratch and intentionally not novel-as-combination.
- Branding: a new engine gets a name. A wire protocol does not. New marketing names go through an ADR.

---

## Role contracts (point to docs/agents/<role>.md)

| Role | Contract |
|------|----------|
| Implementer | [`docs/agents/implementer.md`](docs/agents/implementer.md) |
| Reviewer | [`docs/agents/reviewer.md`](docs/agents/reviewer.md) |
| Architect | [`docs/agents/architect.md`](docs/agents/architect.md) |
| Tester | [`docs/agents/tester.md`](docs/agents/tester.md) |
| Security | [`docs/agents/security.md`](docs/agents/security.md) |
| Release | [`docs/agents/release.md`](docs/agents/release.md) |
| Onboarding | [`docs/agents/onboarding.md`](docs/agents/onboarding.md) |
| Prompt library | [`docs/agents/PROMPT_LIBRARY.md`](docs/agents/PROMPT_LIBRARY.md) |
| Ledger template | [`docs/agents/ledger-template.md`](docs/agents/ledger-template.md) |

---

## If you are a new agent reading this for the first time

1. Read [`README.md`](README.md) for project vision and brand identity.
2. Read [`ARCHITECTURE.md`](ARCHITECTURE.md) for system design.
3. Read [`docs/glossary.md`](docs/glossary.md) for engine brand names.
4. Skim [`docs/agents/onboarding.md`](docs/agents/onboarding.md).
5. Read the role contract that matches your task.
6. Skim the rule files in [`.spiral/rules/`](.spiral/rules/) that apply to your task.
7. Run `bin/spiral-context.sh` at session start. If you have a packet, run `bin/spiral-context.sh <packet-id>`.
8. Run `cargo build` and `cargo test --workspace` to verify the environment.
