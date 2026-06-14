# Spiral Browser — Multi-Model Workspace Instructions

This document tells LLM agents how to work on the Spiral Browser codebase.
Global developer instructions are in `~/.config/opencode/AGENTS.md` (read-only,
do not override).

---

## Current Status

| Field | Value |
|-------|-------|
| **Phase** | Phase 1 — Foundation (Months 1–3) |
| **Sprint** | Sprint 0 — Repo scaffolding and documentation baseline |
| **Active sprint state** | [`docs/active_context.md`](docs/active_context.md) |
| **Task breakdown** | [`docs/phase1-tasks.md`](docs/phase1-tasks.md) |
| **Architecture deltas** | [`docs/system_architecture.md`](docs/system_architecture.md) |
| **Change log** | [`docs/progress_ledger.md`](docs/progress_ledger.md) |
| **Full roadmap** | [`ROADMAP.md`](ROADMAP.md) |

Read `docs/active_context.md` **before starting any task**. It is the single
source of truth for what is in flight, what is blocked, and what you must not
touch.

---

## Model Routing

All agents in this repository use `ozore/custom` (1M context, 16k output).
No model switching is configured at the repo level; the global config handles
that. If you need a different model for a specific role, update
`~/.config/opencode/opencode.jsonc`.

---

## SSOT Update Protocol

After completing any task loop, the **implementer agent** must:

1. Append an entry to `docs/progress_ledger.md`.
2. Update `docs/active_context.md` if sprint state, blockers, or "do not touch"
   zones changed.

The **reviewer agent** must flag a stale `active_context.md` (last update older
than the current task) as a blocking issue.

---

## Quick Start

1. Read `CODEX.md` for project overview
2. Read `ARCHITECTURE.md` for system design
3. Read `PLAN.md` for current phase and tasks
4. Read `docs/active_context.md` for live sprint state
5. Run `cargo build` to verify your environment
6. Run `cargo test` to verify tests pass

---

## Project Rules

### File Editing
- Always read a file before editing it
- Never modify `Cargo.lock` manually (let `cargo` handle it)
- Keep imports sorted: std, external crates, internal crates
- Use `snake_case` for functions/variables, `PascalCase` for types
- Prefer `?` operator over `.unwrap()` in library code
- Add `#[must_use]` to functions that return important values

### Crate Boundaries
- Never depend on a crate "up" the dependency graph
- Example: `spiral-core` cannot depend on `spiral-browser`
- Check `Cargo.toml` before adding a dependency
- Prefer re-exporting from `spiral-core` for shared types

### Commit Messages
```
type(scope): description

[optional body]

[optional footer]
```
Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`
Scopes: core, ipc, html, css, layout, render, dom, js, network, ui, theme, browser, sandbox

Example:
```
feat(layout): implement flexbox container alignment

Added Taffy integration for flex-start, center, flex-end alignment.
Works for both horizontal and vertical axes.
```

### Testing
- Every public function needs a unit test
- Use `#[cfg(test)] mod tests` in the same file
- Integration tests go in `tests/` per crate
- Run `cargo test` before committing
- Run `cargo clippy` for lint checks

---

## Working on Specific Crates

### spiral-core
- Shared types only, no business logic
- All other crates depend on this
- Changes here affect everything — be careful
- Run full workspace tests after changes

### spiral-ipc
- Platform-specific code uses `#[cfg(target_os)]`
- Test on current platform only
- Ensure message framing is correct
- Check for buffer overflow in deserialization

### spiral-html
- Wraps html5ever — read html5ever docs first
- Output is spiral-dom Document
- Handle encoding detection carefully
- Test with malformed HTML (html5ever is lenient)

### spiral-css
- Wraps cssparser + selectors — read Servo docs
- Cascade order: user agent < user < author < author!important
- Specificity: inline > ID > class > element
- Test with complex selector chains

### spiral-layout
- Box model is foundation — get this right first
- Block layout: vertical stacking, margin collapse
- Flexbox: main axis, cross axis, wrap, gap
- Grid: template columns/rows, area placement
- Use Taffy for flex/grid, custom code for block

### spiral-render
- Vello for GPU rendering
- Display list is intermediate representation
- Keep render ops simple and composable
- Test with simple shapes first, text later

### spiral-dom
- DOM tree is central data structure
- Nodes are arena-allocated (Vec<Node> + indices)
- Parent/child relationships via indices
- Attribute storage: `Vec<(String, String)>`

### spiral-js
- Boa engine integration
- Start with console.log only
- DOM manipulation comes later
- Test with simple scripts first

### spiral-network
- hyper for HTTP — read hyper docs
- hickory-dns for DNS resolution
- TLS via rustls
- Handle redirects, timeouts, errors

### spiral-ui
- GPU-rendered browser chrome
- Zen-style design tokens from spiral-theme
- Sidebar tabs, floating URL bar
- Test with winit window directly

### spiral-theme
- CSS custom properties approach
- Light/dark mode
- Single accent color
- System preference detection

### spiral-sandbox
- Platform-specific — only compile for target OS
- Test sandbox profile creation
- Test that blocked operations fail
- Linux: Landlock + seccomp-bpf
- macOS: Seatbelt profiles
- Windows: Restricted Token

---

## Debugging

### Build Issues
```bash
cargo build 2>&1 | head -50    # see first errors
cargo check                     # type checking only
cargo clippy                    # lint warnings
```

### Test Failures
```bash
cargo test                      # all tests
cargo test spiral-core          # specific crate
cargo test test_name            # specific test
```

### IPC Debugging
- Enable RUST_LOG=debug for verbose IPC messages
- Check socket/pipe permissions on Linux/macOS
- Verify named pipe path on Windows

### Rendering Debugging
- Enable RUST_LOG=trace for render pipeline
- Check wgpu adapter selection
- Verify GPU driver compatibility

---

## Common Pitfalls

1. **Forgetting `#[cfg]` guards** — Platform-specific code must be guarded
2. **Breaking IPC serialization** — Changing `IPCMessage` enum breaks bincode
3. **Circular dependencies** — Check dependency graph before adding imports
4. **Unwrap in library code** — Always use `?` or proper error handling
5. **Missing tests** — Every public function needs at least one test
6. **Blocking in async** — Never use `std::thread::sleep` in tokio context

---

## File Templates

### New Crate
```
crates/spiral-{name}/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── error.rs
│   └── {module}.rs
└── tests/
    └── {name}_test.rs
```

### New Module
```rust
// src/{module}.rs

//! Module description

use crate::error::Error;

/// Public type documentation
pub struct TypeName {
    field: Type,
}

impl TypeName {
    /// Constructor documentation
    pub fn new(field: Type) -> Self {
        Self { field }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let t = TypeName::new(value);
        assert_eq!(t.field, value);
    }
}
```

---

## Communication Between Models

When working in a multi-model environment:
- Reference files by path and line number
- Quote exact code snippets when discussing changes
- State which crate and module you're modifying
- Mention any breaking changes to IPC protocol
- Note platform-specific behavior

Example:
> I modified `crates/spiral-layout/src/block.rs:45` to fix margin collapse.
> The change affects `BlockLayout::compute()` which is called from `LayoutTree::layout()`.
> This only affects Linux builds due to `#[cfg(target_os = "linux")]` in the test.
