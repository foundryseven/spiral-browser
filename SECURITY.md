# Security Policy

> **Spiral is a web browser.** An unpatched browser vulnerability can put every user at risk before a fix ships. We take security reports seriously, and we keep the disclosure process private until a fix is ready.

---

## Supported versions

The current alpha line is supported with security updates. Once a stable release ships, only the latest minor version of the latest major version will receive security patches.

| Version | Supported          |
|---------|--------------------|
| `main` (latest) | :white_check_mark: |
| Tagged releases at `>= 1.0.0` | :white_check_mark: (latest minor only) |
| Tagged releases at `< 1.0.0` | :x: |

> **Until Spiral ships a `1.0.0` release, security support is for the `main` branch only.** Pre-1.0 tags are reference points, not supported versions. This is documented in the [`ROADMAP.md`](ROADMAP.md) Phase 9 line: "the production browser."

---

## In-scope vulnerabilities

We are interested in reports against the following surfaces, ordered by user-impact:

### High priority

1. **Memory safety bugs in the parser path** — Fmt (`spiral-fmt`), Gyre (`spiral-gyre`), Vortex (`spiral-vortex`).
2. **Memory safety bugs in the IPC layer** — `spiral-ipc`, including deserialisation buffer overflows and the `IPCMessage` enum handling.
3. **Sandbox escape** — `spiral-sandbox` (Linux Landlock + seccomp-bpf, macOS Seatbelt, Windows restricted tokens + Job Objects).
4. **Same-origin policy bypass** — any cross-origin data leak across the Browser Process ↔ Renderer boundary.
5. **TLS / certificate validation bypass** — `spiral-crypto`, `spiral-net`, `spiral-network`.

### Medium priority

6. **Vortex correctness issues** — engine conformance failures that lead to security-relevant behaviour (e.g. prototype pollution that escapes the `Object` model).
7. **CSS selector parser correctness** — selector injection that bypasses the Filter policy.
8. **Capability-type unsoundness** — ways for a page with `Mode = NoNetwork` to call `fetch()`. Phase 5+ work; we want reports before integration.
9. **Cookie / storage handling** — leak, persistence, or scope error.
10. **Navigation race conditions** — races that let a renderer read resources it should not.

### Low priority

11. **Information disclosure in error paths** — non-crashing leaks via panic messages, log lines, or crash dumps.
12. **Performance regressions with security implications** — slow paths that suggest algorithmic weaknesses.

---

## Out-of-scope

- **Vulnerabilities in vendored dependencies** that are not reachable from Spiral's code. We use `cargo-audit` and `cargo-deny` in CI; report upstream.
- **Speculative academic attacks** with no demonstrated impact. We track the academic literature (e.g. Spectre, Rowhammer) but a paper is not a vulnerability report.
- **Denial of service** via unbounded tab count or memory consumption. The Phase 1 alpha has no DoS surface; we will add bounds in Phase 8.
- **Phishing / social engineering.** Spiral ships no anti-phishing features in the engine.

---

## Reporting a vulnerability

**Please do not file a public GitHub issue for suspected security vulnerabilities.** A public issue gives attackers a roadmap before a fix ships.

Report privately via one of the following channels:

1. **GitHub Security Advisories:** open a draft advisory on the upstream repository. This is the preferred channel; it scopes the conversation to the reporter, the maintainers, and any co-ordinated disclosure partners.
2. **Email:** `security@spiral-browser.example` (replace with the real address before publishing the public repo).

Please include:

- A clear description of the issue and the security impact.
- A reproducer (failing test, proof-of-concept, or step-by-step instructions).
- The commit hash or version tag where you observed the issue.
- Your name and affiliation (or "anonymous") for the acknowledgement in the security advisory.

---

## Response timeline

We aim to:

- **Acknowledge** new reports within **3 business days**.
- **Triage** (confirm / decline / ask for more info) within **10 business days**.
- **Ship a fix** for confirmed issues within **90 days** for high-priority reports. The fix lands on `main` and is included in the next tagged release.
- **Co-ordinate disclosure** with the reporter on the public advisory timing. The default is "ship the fix, then publish the advisory 14 days later."

We may deviate from this timeline for issues that require upstream co-ordination (e.g. a bug in rustls, hyper, or Vello). We will keep the reporter informed.

---

## Security advisories

Published security advisories live at `https://github.com/foundryseven/spiral-browser/security/advisories`. Subscribe to the repository's "Security" tab to get notifications for new advisories.

---

## Security-related ADRs and design documents

- `docs/decisions/0002-vortex-from-scratch.md` — Vortex is a from-scratch JS engine, with the safe-Rust constraint as a first-class design property.
- `docs/decisions/0005-filter-runtime-design.md` — Filter policy runtime; the design for the compile-time policy integration in Phase 5.
- `docs/decisions/0006-cross-cutting-features.md` — Wiring & Integration rule (a `pub` symbol is not done when it compiles; it is done when an external consumer imports it).
- `ARCHITECTURE.md` § 7 — Security model (process isolation, IPC framing, TLS, capability-typed context).
- `.spiral/rules/unsafe-standards.md` — The `unsafe` rule for the codebase. Every `unsafe` block has a safety comment.
- `.spiral/rules/coding-standards.md` — The general coding standards.

---

## Security tooling in CI

- `cargo audit` — RustSec advisory database. Runs on every push.
- `cargo deny` — license + advisory check. Runs on every push.
- `gitleaks` — secret scanning. Runs on every push.
- `audit-orphan-exports.sh` — every `pub` symbol has a consumer. Runs on every push.
- `audit-doc-drift.sh` — SSOT consistency. Runs on every push.
- `audit-orphan-exports.sh --tool-coverage` — every `bin/` and `scripts/` tool is named in a rule. Runs on every push.

The full pipeline is at [`.github/workflows/ci.yml`](.github/workflows/ci.yml).

---

## Acknowledgements

We thank the reporters who help us keep Spiral safe. Reporters who consent to acknowledgement are listed in the relevant security advisory.
