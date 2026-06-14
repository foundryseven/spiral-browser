# Security Policy

## Supported Versions

The current alpha line is supported with security updates. Once a stable release
ships, only the latest minor version of the latest major version will receive
security patches.

| Version | Supported          |
|---------|--------------------|
| main    | :white_check_mark: |
| < 0.1.0 | :x:                |

## Reporting a Vulnerability

Please **do not** file a public GitHub issue for suspected security
vulnerabilities. Spiral is a web browser: an unpatched browser vulnerability
can put every user at risk before a fix ships.

Report privately via one of the following channels:

1. Email: security@spiral-browser.example (replace with the real address before
   publishing).
2. GitHub Security Advisories: open a draft advisory on the upstream repository.

Please include:

- A clear description of the vulnerability and its impact.
- A reproducer: minimal HTML, CSS, JS, or URL that triggers the issue.
- The commit hash or version you tested against.
- Platform and OS version (Linux distribution, macOS build, Windows build).
- Your contact details for follow-up.

We aim to acknowledge new reports within three working days and to issue a fix
or mitigation within thirty days for critical issues, or within the next
regular release for lower-severity issues.

## Scope

In scope for this policy:

- Memory unsafety, use-after-free, double-free, out-of-bounds access.
- Sandbox escapes from the renderer process.
- IPC framing or deserialisation flaws.
- TLS or certificate validation bypasses.
- Same-origin policy or content-security-policy bypasses.
- Privilege escalation between browser, network, and GPU processes.

Out of scope:

- Vulnerabilities in upstream dependencies (file against the upstream
  project; we will pull fixes on a best-effort basis).
- Denial-of-service in a single renderer tab from a malicious page (we do
  not yet enforce resource quotas; this is tracked on the Phase 5 hardening
  list).

## Coordinated Disclosure

We follow a ninety-day coordinated disclosure window. If a fix requires
longer, we will negotiate a mutually agreed extension before the deadline.
