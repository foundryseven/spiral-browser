# Spiral Browser — Roadmap

> **One-page index of the implementation phases. NO tasks, NO packets, NO status.**
> All of that lives in [`docs/implementation_tracker.md`](docs/implementation_tracker.md) (Group → Phase → Step → Packet).
> This file is the *narrative overview* of where the project is going and why.

The roadmap vocabulary is **Group → Phase → Step → Packet**. The time-based `Month` / `Sprint` / `Chunk` / `Item` vocabulary is **retired** as of 2026-06-16. There are no calendar estimates.

---

## The shape of the project

Spiral is built around three engines that bear the Spiral brand, plus two in-design branded subsystems, plus plumbing. The brand belongs to the engines, not the wire.

| Brand | Crate | What it does |
|-------|-------|--------------|
| **Gyre** | `spiral-gyre` | Layout — box model, block, flex, grid |
| **Vortex** | `spiral-vortex` | JavaScript — from-scratch lexer, parser, AST, bytecode VM, GC, future JIT |
| **Fmt** *(the Forge)* | `spiral-fmt` | From-spec HTML5 + CSS parsers |
| **Filter** | `spiral-filter` | Compile-time HTML/CSS policy |
| **Context** | `spiral-context` | Capability-typed page context |

Everything else (`spiral-core`, `spiral-ipc`, `spiral-dom`, `spiral-render`, `spiral-paint`, `spiral-gpu`, `spiral-network`, `spiral-net`, `spiral-crypto`, `spiral-imagedecoder`, `spiral-sandbox`, `spiral-ui`, `spiral-theme`, `spiral-browser`, the deprecated `spiral-css` shim) is plumbing — it wires the engines together.

See [`docs/glossary.md`](docs/glossary.md) for the canonical mapping, and [`PLAN.md`](PLAN.md) § 3 for the per-engine design.

---

## The groups

A *Group* is a capability area of the browser. Phases sit underneath groups. Steps sit underneath phases. Packets sit underneath steps.

| Group | Subsystems | What it ships |
|-------|------------|---------------|
| **Engines** | Vortex, Gyre, Fmt, Filter, Context | The engines that bear the Spiral brand. The hard part. |
| **Networking** | spiral-network, spiral-crypto, spiral-ipc, spiral-sandbox | The wire-level plumbing. HTTP/1.1, HTTP/2, DNS, TLS, sandbox. |
| **Presentation** | spiral-render, spiral-ui, spiral-theme | The visual layer. Vello + wgpu, Zen-style browser chrome. |
| **Cross-cutting** | spiral-core, spiral-browser | Foundation. Shared types, binary entry point. |

---

## The phases

A *Phase* is a major delivery milestone. One Phase = one shipped user-facing capability. Phases are **not** date-bound.

| # | Phase | Status | One-liner |
|---|-------|--------|-----------|
| **0** | Foundation | ✅ Complete | Workspace, IPC shell, hello-world render. |
| **0.5** | SSOT Restructure | ✅ Shipped (`v0.0.0-bootstrap`) | Canonical docs, Group → Phase → Step → Packet vocabulary, no-code-agentic workflow contract. |
| **1** | Engines Foundation | 🔄 In flight | First slices of Gyre, Vortex, Fmt, Filter, Context. |
| **2** | Engines Depth | 🔄 In flight | DOM depth (quirks mode, fragments, adoption agency, active formatting, foster parenting). |
| **3** | Standards Compliance | 📋 Planned | CSS cascade refinement, the CSS Selectors spec, more HTML5 insertion modes. |
| **4** | Layout Depth | 📋 Planned | Gyre flex + grid, writing modes, Bidi. |
| **5** | Capability Runtime | 📋 Planned | Filter + Context integrated with Vortex and DOM. Compile-time policy. |
| **6** | Network Hardening | 📋 Planned | HTTP/2, HTTP/3, content-security-policy integration, same-origin tracking. |
| **7** | Performance | 📋 Planned | Vortex bytecode VM, baseline JIT, Gyre incremental layout, paint caching. |
| **8** | Production Readiness | 📋 Planned | Telemetry (opt-in), crash reporting, update path, packaging. |
| **9** | Production Browser | 📋 Planned | The shipped product. |

Phase 1.6 and Phase 2.8 are the current in-flight steps. Packet-level status is at [`docs/implementation_tracker.md`](docs/implementation_tracker.md).

---

## Project goals — narrative

### Goal 1 — Engine independence

**The web runs on a three-engine monoculture: Blink (Chromium), WebKit (Safari), Gecko (Firefox).** Shared vulnerabilities, shared standards drift, shared design assumptions. A serious bug in one is a serious bug in all.

Spiral exists to be a fourth engine — different in architecture, auditable in code, and written in a memory-safe language. We are not building a faster Chromium. We are building a different thing entirely.

This goal drives Phase 1 and Phase 2: the engines. Until Gyre, Vortex, and Fmt can render a real page end-to-end, Spiral is not a browser. It is a research project.

### Goal 2 — Memory safety end-to-end

**A browser is the most security-critical software most users run.** It parses untrusted input from every network the user touches. It executes untrusted code. It handles untrusted graphics. A single memory-safety bug in the parser is a remote-code-execution bug in every user.

We commit to safe Rust across the render path. The only `unsafe` in the tree is in audited dependencies and in FFI shims with narrow surface area. Every `unsafe` block has a safety comment.

This goal drives the `unsafe-standards.md` rule file in [`.spiral/rules/`](.spiral/rules/) and the FFI-shim approach in `spiral-render`, `spiral-gpu`, and `spiral-sandbox`.

### Goal 3 — From-spec implementations

**If we are building a fourth engine, it has to implement the standards.** Not "what Chrome does", not "what Safari does" — what the spec says.

Our HTML5 parser follows the WHATWG HTML Living Standard section-by-section. Our CSS parser follows the CSS Syntax Level 3 module. Our layout engine is the CSS Display + Box Model specs. Comments in the code cite the spec section. In a code review, the answer to "is this right?" is "section X.Y says so."

This goal drives Fmt's design (`spiral-fmt`, see `docs/architecture/fmt/`).

### Goal 4 — Capability-typed page context

**A web page should not be able to reach a network resource unless it can prove it has the capability.** Not as a runtime check that can be bypassed — as a type-system property that the compiler enforces.

Context (`spiral-context`) is Spiral's bet on this. The shape:

```rust
pub struct Context<'brand, Mode> {
    mode: PhantomData<Mode>,
    capabilities: CapabilitySet<'brand>,
}
```

A page with `Mode = NoNetwork` cannot call `fetch()`. The compiler rejects it. Filter (`spiral-filter`) decides at parse time and request time whether an operation is allowed.

This is research-grade. The design is end-state, but the implementation is incremental. Phase 5 is when Context and Filter integrate with Vortex and DOM. Phase 6+ is when they integrate with the network layer.

### Goal 5 — A working browser, eventually

**Spiral is a real product, not a research project.** The end-state is a browser you can install, that can render real web pages, that runs real JavaScript, that you can rely on for your daily browsing.

The Phase 1 line delivers a browser that can render HTML+CSS and run trivial JavaScript. The Phase 2 line delivers DOM depth. The Phase 5 line delivers the capability-typed runtime. The Phase 9 line ships the production browser.

This goal drives the per-packet-completeness contract: a `pub` symbol is not done when it compiles; it is done when an external consumer uses it. The `audit-orphan-exports.sh` gate enforces this.

### Goal 6 — Open source, day one

**Every commit, every decision, every metric is public.** There is no private fork. There is no roadmap-only-on-Discord. The ADRs are public, the ledger is public, the implementation is public, the build is reproducible from a clean clone.

This goal drives the MPL-2.0 licence, the public ledger (`docs/progress_ledger.md`), the public ADRs (`docs/decisions/`), and the workflow contract (`.spiral/rules/` + `AGENTS.md`).

---

## What is *not* on the roadmap

- **Performance leadership.** We are not faster than V8. We are not aiming to be. We are aiming to be *correct*, *independent*, and *inspectable*.
- **A privacy browser.** Privacy is a use case the capability types *enable*; we are not building it in the engine.
- **A drop-in Chrome replacement.** Sites that depend on Chrome-only Web Platform features will not work. We track the standards, not the implementation shortcuts.
- **A proprietary codebase.** Everything is public.

---

## Where the work lives

- **SSOT for status:** [`docs/implementation_tracker.md`](docs/implementation_tracker.md) (Group → Phase → Step → Packet)
- **SSOT for live state:** [`docs/active_context.md`](docs/active_context.md)
- **Append-only change log:** [`docs/progress_ledger.md`](docs/progress_ledger.md)
- **ADRs:** [`docs/decisions/`](docs/decisions/)
- **Rule files:** [`.spiral/rules/`](.spiral/rules/)
- **Role contracts:** [`docs/agents/`](docs/agents/)
- **Architecture delta:** [`docs/system_architecture.md`](docs/system_architecture.md)
- **Architecture (canonical):** [`ARCHITECTURE.md`](ARCHITECTURE.md)
- **Implementation plan:** [`PLAN.md`](PLAN.md)
- **This file:** [`ROADMAP.md`](ROADMAP.md)
- **Front door:** [`README.md`](README.md)
- **LLM cheatsheet:** [`CODEX.md`](CODEX.md)
