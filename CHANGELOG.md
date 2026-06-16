# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.0] - 2026-06-16

The bootstrap release. Documentation-only; no public-API
changes. Establishes the SSOT hierarchy that all future
releases will follow. **Post-bootstrap follow-on
packets** (1.6.1, 1.6.3, 1.6.4) shipped later the
same day and will appear in the next tagged release.
Full release notes: [docs/releases/0.0.0-bootstrap.md](docs/releases/0.0.0-bootstrap.md).

## [Unreleased]

### Added
- **Phase 1.5 — SSOT Restructure (2026-06-16).** New canonical documents and a
  Group → Phase → Step → Packet vocabulary. The time-based Month / Sprint /
  Chunk / Item vocabulary is **retired**.
  - `docs/implementation_tracker.md` — Group → Phase → Step → Packet status
    SSOT (replaces the old `specs/SPRINT_*.md` series and the per-Sprint
    item tables in `docs/archives/phase1-tasks.md`).
  - `docs/active_context.md` — live pointer to current phase, blockers, and
    "do-not-touch" zones.
  - `docs/glossary.md` — engine brand names (Gyre, Vortex, Fmt, Forge).
  - `docs/decisions/0000-0004` ADRs — cross-cutting decisions, immutable
    once Accepted.
  - `docs/agents/` — role contracts (implementer, reviewer, architect,
    tester, security, release, onboarding, PROMPT_LIBRARY).
  - `docs/architecture/` — per-subsystem architecture stubs
    (context, design, filter, fmt, gyre, net, vortex).
  - `docs/audits/2026-06-15-baseline.md` — functional baseline.
  - `docs/baseline-warnings.md` — `cargo check --workspace` warning
    drift baseline.
  - `docs/audit-sprint-m4.md` — M4 novelty / licence / originality
    audit (now archived methodology; novelty claims use it as the
    canonical standard).
  - `docs/innovations/backlog.md` — 70-idea backlog (the "bets").
  - `docs/plans/iteration-options.md` — iteration strategy
    (scheduling lives in the tracker).
  - `docs/releases/0.0.0-bootstrap.md` — first release notes.
  - `docs/security/post-mortems/` — security post-mortem log.
  - `docs/archives/phase1-tasks.md` — Phase 1 task archive (moved
    from the live tracker).
  - `scripts/audit-orphan-exports.sh` — orphan-export wiring audit.
  - `.spiral/rules/architecture.md`, `.spiral/rules/coding-standards.md`,
    `.spiral/rules/testing.md` — repository rule files.
  - `AGENTS.md` — Decision Protocol and Wiring & Integration rule
    (adopted from the Zeus repo's `docs/decisions/0006-cross-cutting-features.md`).
- **Phase 1.6 Packet 1.6.1 — Vortex GC rewrite (2026-06-16, SHIPPED).**
  Mark-sweep GC rewritten; tracking of free list, roots, and mark phase
  is now test-covered. See
  `docs/progress_ledger.md` (2026-06-16 Vortex GC entry) and
  `docs/implementation_tracker.md` (Phase 1 Step 1.6).

### Changed
- **Crate rename:** `crates/spiral-js/` → `crates/spiral-vortex/`.
  Package name: `spiral-js` → `spiral-vortex`. Backing engine strategy
  is from-scratch Rust; `rusty_v8` is behind the `v8` Cargo feature for
  CI compliance testing only (the "V8 oracle"). See
  `docs/decisions/0002-vortex-from-scratch.md`.
- **Crate rename:** `crates/spiral-layout/` → `crates/spiral-gyre/`.
  Package name: `spiral-layout` → `spiral-gyre`. `taffy` was never
  added; Gyre is in-house from day one. See
  `docs/decisions/0003-gyre-rename.md`.
- **Crate retirement:** `crates/spiral-html/` removed from workspace
  (2026-06-15). All HTML parsing lives in `spiral-fmt`.
- **Crate deprecation:** `crates/spiral-css/` is a deprecated shim
  (2026-06-16). Forwards to `spiral_fmt::css::*` and provides a
  `CssParser` adapter that calls `spiral_fmt::parse_css`. New code
  should depend on `spiral-fmt` directly. The `deprecation` lint is
  set on the crate. See
  `docs/decisions/0001-css-parser-spiral-fmt.md`.
- **New crates:** `crates/spiral-fmt/` (from-spec HTML5 + CSS),
  `crates/spiral-context/` (capability-typed API, Bet 1),
  `crates/spiral-filter/` (compile-time HTML/CSS policy, Bet 3),
  `crates/spiral-imagedecoder/` (PNG / JPEG / WebP / AVIF).
- **Workspace now 20 members.** Updated `README.md`, `CODEX.md`,
  `ARCHITECTURE.md`, `PLAN.md`, `AGENTS.md`, `CODEOWNERS`, and
  `Cargo.toml` member list to match.
- **Workspace dependency cleanup:** `html5ever`, `markup5ever`,
  `tendril`, `cssparser`, `selectors`, `cssparser-macros` are not
  vendored and not in the workspace dependency graph. The earlier
  "0.29→0.39 / 0.33→0.37 / 0.25→0.38" version-bump notes in the
  changelog were forward-looking and never applied; the decision
  (ADR-0001) is from-spec in `spiral-fmt`.
- **Workspace dependency versions:** `vello` 0.3→0.9, `wgpu` 23→29,
  `harfrust` 0.1→0.8, `cosmic-text` 0.12→0.19, `png` 0.17→0.18,
  `webp` 0.4→0.3, `ravif` 0.11→0.13, `caps` 0.3→0.5, `rusty_v8`
  0.32.1 (new). Hyper, hyper-util, http-body-util, hickory-resolver,
  hickory-proto, rustls, rustls-pemfile, webpki-roots, sha2, and
  getrandom are unchanged.
- `AGENTS.md` updated to point at the new SSOT surface and to
  document the Decision Protocol, the Wiring & Integration rule, and
  the Novelty Claims rule.
- `ROADMAP.md` and `PLAN.md` rewritten as one-page and strategy
  documents; calendar and packet detail moved to the tracker.
- `README.md` rewritten with current status pointer, full crate
  list, and an "important removals" note.
- `ARCHITECTURE.md` updated to reflect the from-spec HTML+CSS
  pipeline; the old `spiral-html (vendored html5ever)` line is
  replaced by `spiral-fmt` (from-spec tokeniser + tree builder).
- `CODEX.md` updated to be a one-shot reference for LLMs; it
  cross-references the new SSOT.
- `CODEOWNERS` updated: `spiral-html/` removed; `spiral-fmt/`,
  `spiral-context/`, `spiral-filter/` added; ownership moved to
  `@spiral/dom-maintainers` for the parsing/capability area.
- `TESTING.md` integration test example now uses
  `cargo test --package spiral-fmt` and `spiral-fmt` (the retired
  `spiral-html` example is gone).
- `cargo fmt` and `cargo clippy -D warnings` applied across the
  workspace.

### Fixed
- `Cargo.toml` workspace dependency list no longer declares unused
  `cssparser` and `selectors` entries; they were dead since the
  `spiral-html` retirement.
- `specs/GAP_ANALYSIS.md` is now spec-only; status moved to
  `docs/implementation_tracker.md` per the SSOT restructure.
- `docs/progress_ledger.md` remains append-only; the docdrift-sync
  pass (this entry) is logged at the top of the append.

## [0.0.0-bootstrap] — 2026-06-16

Initial bootstrap release. See
[`docs/releases/0.0.0-bootstrap.md`](docs/releases/0.0.0-bootstrap.md)
for the full release notes. Tagged at the commit that closes
Phase 0 and Phase 1.5.

### Added
- 20-crate Cargo workspace with a hello-world end-to-end render
  target (Phase 0 complete).
- IPC shell: `spiral-ipc` with Unix domain socket transport
  (Linux/macOS), Win32 named-pipe transport, `MockTransport` for
  tests, and u32-LE length-prefixed bincode framing.
- `spiral-core` shared types: `BrowserConfig`, `TabId`,
  `IPCMessage` (13 variants), `Error`, with comprehensive tests.
- `spiral-dom` arena-allocated DOM tree.
- `spiral-fmt` from-spec HTML5 tokeniser + tree builder (8 insertion
  modes) and CSS parser (8 modules: tokeniser, parser, selectors,
  specificity, values, at-rules, declarations, attribute matchers).
- `spiral-css` deprecated shim forwarding to `spiral-fmt`.
- `spiral-gyre` (Gyre) custom layout engine — box model + block
  layout in Phase 1; flex / grid in Phase 2.
- `spiral-render` + `spiral-paint` + `spiral-gpu` — Vello + wgpu
  GPU render path.
- `spiral-vortex` (Vortex) from-scratch JS engine — lexer, parser
  (Pratt), AST, tree-walking interpreter, mark-sweep GC, builtins
  (console, math, object, array), event loop, DOM binding stubs.
- `spiral-context` capability-typed API surface (Bet 1, in progress).
- `spiral-filter` compile-time HTML/CSS policy engine (Bet 3, in
  progress).
- `spiral-network` + `spiral-net` + `spiral-crypto` HTTP/TLS/DNS
  stack (hyper + hickory-dns + rustls).
- `spiral-imagedecoder` PNG / JPEG / WebP / AVIF.
- `spiral-sandbox` OS-level sandboxing (Linux Landlock + seccomp,
  macOS Seatbelt, Win32 restricted tokens — scaffolded in Phase 1,
  to be hardened in Phase 4).
- `spiral-ui` + `spiral-theme` Zen-style browser chrome
  (sidebar tabs, floating URL bar, single-accent-colour theming).
- `spiral-browser` binary entry point.
- GitHub CI: 4-job pipeline (fmt, clippy, test, build) across
  Linux, macOS, Windows; clippy runs with `-D warnings`.
- Multi-model agent definitions under `~/.config/opencode/agents/`:
  architect, implementer, reviewer, test-writer.
- Foundation documents: `README.md`, `LICENSE`, `CHANGELOG.md`,
  `SECURITY.md`, `CODEOWNERS`, `BUILD.md`, `TESTING.md`, `ERRORS.md`,
  `CONTRIBUTING.md`.

[Unreleased]: https://github.com/your-org/spiral-browser/compare/v0.0.0-bootstrap...HEAD
[0.0.0-bootstrap]: https://github.com/your-org/spiral-browser/releases/tag/v0.0.0-bootstrap
