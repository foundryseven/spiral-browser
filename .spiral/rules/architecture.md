---
paths:
  - "crates/**/Cargo.toml"
  - "crates/**/src/lib.rs"
---

# Architecture Rules

## Crate Boundaries

A crate may not depend on a crate "up" the dependency graph. The
topology is a DAG, not a free-for-all. If you are about to write
`spiral-fmt` → `spiral-browser` (or any other "downward" edge), stop
and write an ADR.

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
- No crate may re-export from `spiral-core` to "wrap" the type.
  If a wrapper is needed, that wrapper is its own type and lives
  in the crate that needs it.

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
  crate-internal and may be re-exported as the public surface evolves.

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
