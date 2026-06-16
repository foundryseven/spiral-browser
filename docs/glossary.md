# Glossary — Spiral Browser

> Internal language: which crate is which, which engine is
> which. Borrowed from the Zeus repo's `docs/glossary.md`
> pattern (2026-06-16, full Tier 1+2+3 restructure).

This file is the canonical mapping. If a crate exists
without an entry here, that is a bug — add it.

---

## Engine Layer (the "in-house" pieces)

| Brand | Crate | One-liner | Status |
|-------|-------|-----------|--------|
| **Gyre** | `spiral-gyre` | Layout engine — block, flex, grid, box model. Fully in-house Rust, no Taffy. | M4.6+ in progress (renamed from `spiral-layout` 2026-06-14) |
| **Vortex** | `spiral-vortex` | JavaScript engine — lexer, parser, AST, bytecode compiler, interpreter, GC. From-scratch Rust. | M4.5+ in progress (renamed from `spiral-js` 2026-06-14) |
| **Forge** | `spiral-fmt` | Format parsers — HTML5 tokeniser + tree builder, CSS Syntax 3 tokeniser + parser. From-spec, no html5ever / no cssparser. | M4.4 Chunks 1–3 + Item 4 complete |

## Policy / Runtime Layer

> Note: "Spirit" in `docs/innovations-backlog.md:141` is a
> **priority category** (items that "wave" with the
> capability-typed system), not a brand name for a crate.
> It is listed here as a clarification, not as a brand.

| Brand | Crate | One-liner | Status |
|-------|-------|-----------|--------|
| *(unbranded)* | `spiral-filter` | Compile-time filter policy engine. Decides at parse time whether a request is allowed. The brand name (if any) is undecided. | M4.5 Item 12 — design only |
| *(unbranded)* | `spiral-context` | Capability-typed page context (brands, `Context<'brand, Mode>`, `CapabilitySet<'brand>`, `InProcess` / `Escalated`). | M4.5+ in progress |

## Plumbing Crates (utilitarian, no brand name)

These exist to wire the engines together. No brand —
the brand belongs to the engine they serve, not the wire.

| Crate | Role |
|-------|------|
| `spiral-core` | Shared types — `Color`, `TabId`, `BrowserConfig`, `IPCMessage`, `Error`, `Result`. Every other crate depends on it. |
| `spiral-ipc` | Inter-process messaging — `IpcTransport` trait, Unix/Windows transport impls, bincode framing. |
| `spiral-dom` | Document Object Model — arena-allocated nodes (`Vec<Node>` + indices), element / text / comment / document. |
| `spiral-crypto` | SHA-256 (`sha2` crate), CSPRNG (`getrandom`). From Chunk 1, G0.1 fix. |
| `spiral-net` | DNS resolver (Hickory), async TCP. |
| `spiral-network` | HTTP fetch (hyper), TLS (rustls). |
| `spiral-browser` | Browser shell — winit window, event loop, display list, theme integration. |
| `spiral-ui` | Browser chrome — sidebar tabs, floating URL bar. GPU-rendered. |
| `spiral-render` | Software + GPU rendering (Vello). Display-list intermediate representation. |
| `spiral-paint` | Paint pipeline — display list → pixel output. |
| `spiral-gpu` | GPU device selection and pipeline management. |
| `spiral-imagedecoder` | PNG / WebP / AVIF / JPEG decoders. |
| `spiral-sandbox` | Platform sandboxing — Linux (Landlock + seccomp-bpf), macOS (Seatbelt), Windows (Restricted Token). |
| `spiral-theme` | Design tokens, light/dark mode, system preference. |

## Deprecated / Retired Crates

| Crate | Status |
|-------|--------|
| `spiral-html` | **Retired 2026-06-15.** Used `html5ever 0.39` with 6 panicking tests (G0.2). Replaced by `spiral-fmt::html`. |
| `spiral-layout` | **Renamed** to `spiral-gyre` on 2026-06-14. |
| `spiral-js` | **Renamed** to `spiral-vortex` on 2026-06-14. |
| `spiral-css` | **Deprecated shim** as of M4.4.1 Item 4 (2026-06-16). Forwards to `spiral_fmt::css::*`. The `cssparser` + `selectors` deps are gone. |

---

## Brand vs. plain-English rule

Internal documentation and crate docstrings use the brand
name (`Gyre`, `Vortex`, `Forge`, `Spirit`). The plain
English expansion is the public-facing term and appears
in user-visible UI strings:

| Brand (internal) | Plain English (user-facing) |
|------------------|------------------------------|
| Gyre | "Layout engine" |
| Vortex | "JavaScript engine" |
| Forge | "Format parsers" (HTML + CSS) |
| Spirit | (no public surface yet) |

If a brand name appears in user-visible strings, that is
a bug — file an issue and switch to the plain-English
form. The "subtle subtitle, plain primary" pattern keeps
the brand identity without making the UI opaque.
