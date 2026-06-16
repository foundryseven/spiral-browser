# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Repository skeleton: 18-crate Cargo workspace.
- Foundation documents: `README.md`, `LICENSE`, `CHANGELOG.md`, `SECURITY.md`, `CODEOWNERS`.
- Standardised `/docs` SSOT: `system_architecture.md`, `active_context.md`, `progress_ledger.md`, `phase1-tasks.md`.
- Multi-model agent definitions under `~/.config/opencode/agents/`: architect, implementer, reviewer, test-writer.
- GitHub issue and pull request templates.
- CI/CD: 4-job pipeline (`fmt`, `clippy`, `test`, `build`) across Linux/macOS/Windows.
- `spiral-core` comprehensive test suite (18 tests): `BrowserConfig` bincode round-trip, `TabId` Display/equality/hash, `IPCMessage` all 13 variants, `Error` From/Display/propagation.
- `TabId` `Display` impl (previously missing).
- `spiral-ipc`: `IpcTransport` trait (`send`/`recv`/`close`) with `Pin<Box<dyn Future>>`.
- `spiral-ipc`: `unix::UnixListener` + `unix::UnixTransport` — Unix domain socket transport (Linux/macOS).
- `spiral-ipc`: `pipe::PipeListener` + `pipe::PipeTransport` — Windows named pipe transport (`#[cfg(windows)]`).
- `spiral-ipc`: `MockTransport::pair()` — in-memory MPSC-backed transport for testing.
- `spiral-ipc`: public `encode_message`/`decode_message` — u32-LE length-prefixed bincode framing.
- `spiral-ipc`: fuzz smoke test — 11 structured malformed patterns + 256 single-byte header permutations.
- **Gyre** (codename) and **Vortex** (codename) introduced as the brand identities of Spiral's two custom engines: `spiral-gyre` (layout) and `spiral-vortex` (JavaScript).
- Vortex: full from-scratch JS engine skeleton — lexer, parser (Pratt parsing), AST, tree-walking interpreter, JsValue, object model, mark-sweep GC heap, builtins (console, math, object, array), event loop, DOM binding stubs, V8 oracle (`v8` feature).
- `rusty_v8` available behind `v8` feature flag for CI compliance testing only.

### Changed
- `AGENTS.md` updated with current phase pointer, model routing, and SSOT references.
- `opencode.jsonc` default model switched to `ozore/custom`.
- `CODEX.md` crate count corrected from 17 to 18 to match workspace.
- `ROADMAP.md` and `PLAN.md` crate count corrected from 17 to 18.
- Workspace `Cargo.toml`: removed invalid `[target]` section (not allowed in virtual workspace manifests).
- Workspace `Cargo.toml`: added all 18 internal crate path-dependencies to `[workspace.dependencies]`.
- Workspace dependency versions updated to latest compatible: `html5ever` 0.29→0.39, `cssparser` 0.33→0.37, `selectors` 0.25→0.38, `vello` 0.3→0.9, `wgpu` 23→29, `rusty_v8` (new), `harfrust` 0.1→0.8, `cosmic-text` 0.12→0.19, `png` 0.17→0.18, `webp` 0.4→0.3, `ravif` 0.11→0.13, `caps` 0.3→0.5.
- **Crate rename:** `crates/spiral-js/` → `crates/spiral-vortex/`. Package name: `spiral-js` → `spiral-vortex`. Backing engine strategy shifted from `rquickjs` → `rusty_v8` to `rusty_v8` directly (no QuickJS intermediate).
- **Crate rename:** `crates/spiral-layout/` → `crates/spiral-gyre/`. Package name: `spiral-layout` → `spiral-gyre`. `taffy` removed from workspace dependencies; Gyre is in-house from day one.
- CI/CD: rewrote `.github/workflows/ci.yml` with separate `fmt`, `clippy`, `test`, `build` jobs across Linux/macOS/Windows. Clippy runs with `-D warnings`.
- `cargo fmt` applied across entire workspace.

### Fixed
- `tests/` and `benches/` directories created with `.gitkeep` so paths in `PLAN.md` and `CODEX.md` resolve.
- `spiral-css` compile error: `if`-`else` expression type mismatch in `parse_value()` (`.rs` line ~206).
- `spiral-sandbox` compile error on macOS/Windows: `caps` crate (Linux-only) was an unconditional dependency; moved behind `cfg(target_os = "linux")`.
- `spiral-css`: removed unused `Error` import (not used by any function in the crate).
- `spiral-css`: clippy `manual_strip` — replaced `starts_with` + slicing with `strip_prefix`.
- `spiral-css`: test `test_parse_selector` assertion corrected (3 parts, not 4).
- `spiral-dom`: clippy `vec_init_then_push` — replaced `Vec::new()` + `push` with `vec![]`.
- `spiral-ipc`: unused import `RendererToBrowser` removed from test module.
- `spiral-ui`: unused variable `id` prefixed with `_` in `test_add_tab`.
- `spiral-layout`: `Document` branch in `layout_node` now assigns `content.width = available_width`, fixing `test_layout_empty_dom`.

[Unreleased]: https://github.com/your-org/spiral-browser/compare/main...HEAD
