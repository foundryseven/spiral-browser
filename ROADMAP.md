# Spiral Browser — Roadmap

This document is a **one-page index** of the implementation phases.
It does not contain tasks, packets, or status. All of that lives in
[`docs/implementation_tracker.md`](docs/implementation_tracker.md).

The roadmap vocabulary is **Group → Phase → Step → Packet**. The
time-based `Month` / `Sprint` / `Chunk` / `Item` vocabulary is
**retired** as of 2026-06-16. There are no calendar estimates.

---

## Groups

A Group is a capability area of the browser. Phases sit underneath
Groups.

| Group | Subsystems |
|-------|------------|
| **Engines** | Vortex (JS), Gyre (layout), Fmt (HTML/CSS), Filter, Context |
| **Networking** | spiral-network, spiral-crypto, spiral-ipc, spiral-sandbox |
| **Presentation** | spiral-render, spiral-ui, spiral-theme |
| **Cross-cutting** | spiral-core, spiral-browser (binary), `docs/decisions/`, `docs/innovations/` |

See [`docs/architecture/`](docs/architecture/) for the per-subsystem
design and [`docs/glossary.md`](docs/glossary.md) for the brand names.

---

## Phases

A Phase is a major delivery milestone. The Phase content (Steps,
Packets, status) lives in
[`docs/implementation_tracker.md`](docs/implementation_tracker.md).

| # | Title | What ships |
|---|-------|------------|
| **0** | [Foundation](docs/implementation_tracker.md#phase-0--foundation-) | IPC shell, hello-world render, 20-crate workspace. **Done.** |
| **1** | [Engines Foundation](docs/implementation_tracker.md#phase-1--engines-foundation-) | Vortex first slice (`console.log`), spiral-fmt HTML+CSS parsers (from-spec), Gyre box model, filter runtime. **In flight.** |
| **1.5** | [SSOT Restructure](docs/implementation_tracker.md#phase-15--ssot-restructure-) | Group → Phase → Step → Packet hierarchy; rule files; role docs; CI supply-chain baseline. **Shipped at `v0.0.0-bootstrap` (2026-06-16).** |
| **2** | [Engines Depth](docs/implementation_tracker.md#phase-2--engines-depth-) | Fragment parsing, DOM collection types, global attributes, `dataset`, `globalThis`, `structuredClone`, `URL` + `URLSearchParams`, adoption agency, AFE, foster parenting. |
| **3** | [Networking](docs/implementation_tracker.md#phase-3--networking-) | HTTP/1.1 client, cookie jar, DNS resolver, sandbox profile, IPC hardening. |
| **4** | [Presentation](docs/implementation_tracker.md#phase-4--presentation-) | Vello integration, browser chrome, theme system. |
| **5** | [Capability Types Runtime](docs/implementation_tracker.md#phase-5--capability-types-runtime-) | Per-origin isolate, capability tokens in production paths. (Bet 1 runtime.) |
| **6** | [Bytecode VM](docs/implementation_tracker.md#phase-6--bytecode-vm-) | Vortex tree-walker → bytecode VM, ICs, real-world profile gate. |
| **7** | [Media + DRM](docs/implementation_tracker.md#phase-7--media--drm-) | MSE demuxers, codecs, ClearKey EME, Widevine (gated). |
| **8** | [Persistent Renderer](docs/implementation_tracker.md#phase-8--persistent-renderer-) | Vortex heap + layout tree + document checkpoints. (Bet 4.) |
| **9** | [Hardening](docs/implementation_tracker.md#phase-9--hardening-) | Memory budget CI gate, WPT coverage, fuzz harnesses, supply-chain review. |

---

## Where to look for what

| Question | File |
|----------|------|
| What is in flight? | [`docs/implementation_tracker.md`](docs/implementation_tracker.md) |
| What just shipped? | [`docs/progress_ledger.md`](docs/progress_ledger.md) (last 3 entries) |
| What's the live Phase state? | [`docs/active_context.md`](docs/active_context.md) |
| What does the browser need to do? | [`specs/GAP_ANALYSIS.md`](specs/GAP_ANALYSIS.md) (spec-only) |
| Why was X decided? | [`docs/decisions/`](docs/decisions/) (ADRs) |
| What is the architecture? | [`docs/system_architecture.md`](docs/system_architecture.md) |
| How do I work on Y? | [`docs/agents/<role>.md`](docs/agents/) + [`.spiral/rules/`](.spiral/rules/) |

---

If you are an agent (human or AI) reading this for the first time,
start at [`docs/agents/onboarding.md`](docs/agents/onboarding.md).
