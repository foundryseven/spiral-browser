---
paths:
  - "crates/**/Cargo.toml"
  - "crates/**/src/lib.rs"
---

# Architecture Rules

> **Read first.** This file is the operative contract for crate
> boundaries and dep graph changes. The companion workflow gate
> table lives in [`AGENTS.md`](../AGENTS.md) and the gate-level
> detail lives in [`.spiral/rules/workflow.md`](workflow.md).
> Where this file and `AGENTS.md` disagree, this file wins for
> architecture-specific questions; `workflow.md` wins for
> "what tool, when".

## Workflow Tools (mandatory)

| Moment | MUST run | Why |
|--------|----------|-----|
| Before adding a new `Cargo.toml` dep | `cargo tree --workspace --edges normal -i <this-crate>` | Confirms the new edge points "down" the canonical graph below; no upward edge is permitted. |
| After promoting a `pub(crate)` item to `pub` | `./scripts/audit-orphan-exports.sh` | Verifies the new `pub` symbol has an external consumer (Wiring & Integration). |
| After writing an ADR for a boundary change | `bin/spiral-context.sh` to re-surface SSOT | Confirms the ADR is referenced from `docs/implementation_tracker.md`. |



## Crate Boundaries

A crate MUST NOT depend on a crate "up" the dependency graph. The
topology is a DAG, not a free-for-all. An implementer MUST run
`cargo tree --workspace --edges normal -i <this-crate>` (or
`cargo metadata | jq` for programmatic use) before adding a new
dependency edge, to confirm the edge goes "down" the graph and
does not cross a boundary already declared in `### The canonical
dependency graph` below. If a candidate edge crosses an
existing boundary (e.g. `spiral-fmt` → `spiral-browser`, or any
other "downward" arrow), the implementer MUST stop and write an
ADR under `docs/decisions/` per the Decision Protocol in `AGENTS.md`
before writing any code. The "MUST run" verb is gating; an
untracked dep edge is a build break treated by
`scripts/audit-orphan-exports.sh` and a peer reviewer.

### The canonical dependency graph

```
spiral-core                    (shared types, no business logic)
    │
    ▼
spiral-ipc                     (no business logic)
    │
    ▼
spiral-crypto  ───────────────┐
    │                         │
    ▼                         ▼
spiral-fmt  ─────►  spiral-dom  ◄────  spiral-vortex
                       ▲
                       │
                  spiral-filter
                       │
                       ▼
                  spiral-context
                       │
                       ▼
              spiral-gyre  ─────►  spiral-render
                       │
                       ▼
                  spiral-theme
                       │
                       ▼
                  spiral-ui
                       │
                       ▼
            spiral-sandbox
                       │
                       ▼
            spiral-network
                       │
                       ▼
            spiral-browser   (the binary; the only end-user surface)
```

A "down-only" rule means an arrow can go `core → fmt`, but never
`fmt → core`, never `browser → render`, never `vortex → fmt`.

### Re-exporting from `spiral-core`

If a type is needed by 3+ crates, **promote it to `spiral-core`**.
The rule for re-exports:

- `spiral-core` defines the type.
- All other crates `use spiral_core::TheType`.
- No crate MUST re-export from `spiral-core` to "wrap" the type.
  A wrapper is its own type and lives in the crate that needs it.

### Shared types

- `BrowserConfig`, `TabId`, `Origin`, `IPCMessage`, `Error`,
  `JsValue`, `VortexError`, `FormatError` all live in `spiral-core`
  (or the most-downstream crate that owns the concept).
- `RenderNodeId` lives in `spiral-core`. `DisplayList` lives in
  `spiral-render`. Don't cross-pollenate.

### Naming

- Every subsystem has a brand name. See
  [`docs/glossary.md`](../docs/glossary.md) for the list.
- Never use generic names ("agent", "service", "handler") when a
  subsystem name exists.
- A new subsystem is a `Decision Protocol` event (write an ADR).

## Cargo.toml conventions

- Workspace deps in the root `Cargo.toml` `workspace.dependencies` block.
- Crate-level `Cargo.toml` references them with `dep.workspace = true`.
- Feature flags for optional deps: default off, named with the
  crate's own name (e.g. `v8` on `spiral-vortex`, not `enable_v8`).
- Public surface is the `pub` items in `lib.rs`. Everything else is
  crate-internal and MUST remain private until promotion to `pub`.
  A `pub(crate)` item MUST NOT be widened to `pub` except when an external
  consumer requires it, and the change MUST be verified by
  `./scripts/audit-orphan-exports.sh` after the next build.

## When to split a crate

- A crate has more than ~5,000 lines of source.
- A crate has two distinct audiences (e.g. a parser and a VM).
- A subsystem is moving from "stub" to "real" and the audit
  (see Wiring & Integration rule in
  [`docs/implementation_tracker.md`](../docs/implementation_tracker.md))
  flags it as not crossing crate boundaries.

When in doubt, prefer the smaller crate. The fork is rarely as wide
as it looks.

## When NOT to split a crate

- The two halves share a public type.
- The two halves are at the same architectural depth.
- The split would create a circular dep.

Borrowed 2026-06-16 from the Zeus repo's `.zeus/rules/architecture.md`.
