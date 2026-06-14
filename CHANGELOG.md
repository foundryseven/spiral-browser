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

### Changed
- `AGENTS.md` updated with current phase pointer, model routing, and SSOT references.
- `opencode.jsonc` default model switched to `ozore/custom`.
- `CODEX.md` crate count corrected from 17 to 18 to match workspace.
- `ROADMAP.md` and `PLAN.md` crate count corrected from 17 to 18.
- Workspace `Cargo.toml`: removed invalid `[target]` section (not allowed in virtual workspace manifests).
- Workspace `Cargo.toml`: added all 18 internal crate path-dependencies to `[workspace.dependencies]`.
- Workspace dependency versions updated to latest compatible: `html5ever` 0.29→0.39, `cssparser` 0.33→0.37, `selectors` 0.25→0.38, `taffy` 0.5→0.11, `vello` 0.3→0.9, `wgpu` 23→29, `boa_engine` 0.20→0.21, `harfrust` 0.1→0.8, `cosmic-text` 0.12→0.19, `png` 0.17→0.18, `webp` 0.4→0.3, `ravif` 0.11→0.13, `caps` 0.3→0.5.

### Fixed
- `tests/` and `benches/` directories created with `.gitkeep` so paths in `PLAN.md` and `CODEX.md` resolve.
- `spiral-css` compile error: `if`-`else` expression type mismatch in `parse_value()` (`.rs` line ~206).
- `spiral-sandbox` compile error on macOS/Windows: `caps` crate (Linux-only) was an unconditional dependency; moved behind `cfg(target_os = "linux")`.
- `spiral-css`: removed unused `Error` import (not used by any function in the crate).

[Unreleased]: https://github.com/your-org/spiral-browser/compare/main...HEAD
