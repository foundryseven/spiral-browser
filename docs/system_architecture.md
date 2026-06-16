# System Architecture

This document is the **delta file** for `ARCHITECTURE.md`. It records only
changes, clarifications, and in-flight decisions that have not yet been
reflected in the canonical `ARCHITECTURE.md`. If a detail exists in both files,
`ARCHITECTURE.md` is authoritative.

---

## Current Architecture (as of June 2026)

See `ARCHITECTURE.md` at the repository root for the full, stable system
design: process model, IPC layout, rendering pipeline, and data flows.

The **June 2026 design pass** introduces a new top-level architectural bet
that reframes the process model. It is documented in
[`architecture/design/shared-everything.md`](architecture/design/shared-everything.md)
and is the in-flight decision. The canonical `ARCHITECTURE.md` does not yet
reflect it; this delta file is authoritative for the new design until the
canonical doc is updated.

---

## Per-subsystem architecture

The detailed per-subsystem design lives in
[`docs/architecture/`](architecture/):

- [`vortex.md`](architecture/vortex.md) — JavaScript engine
- [`gyre.md`](architecture/gyre.md) — Layout engine
- [`fmt.md`](architecture/fmt.md) — HTML+CSS parser
- [`net.md`](architecture/net.md) — Networking
- [`filter.md`](architecture/filter.md) — Content policy
- [`context.md`](architecture/context.md) — Per-tab state

## Design documents

Design-time documents (architectural bets, exploration, design-time
proposals) live in [`docs/architecture/design/`](architecture/design/).
They are not SSOT; the implementation tracker and ADRs are. The design/
folder is a working space for proposals.

- [`shared-everything.md`](architecture/design/shared-everything.md) — The
  shared-everything multi-process bet (June 2026 design pass).
- [`capability-types.md`](architecture/design/capability-types.md) — The
  capability-type-system design (Bet 1).
- [`filter-rule-model.md`](architecture/design/filter-rule-model.md) — The
  filter rule model (Bet 3).
- [`vortex-heap.md`](architecture/design/vortex-heap.md) — The Vortex heap
  and GC design.

---

## Deltas

### June 2026 — Design pass: four architectural bets (ACTIVE)

**Decision date:** 2026-06-14
**Decided by:** user (engine thesis sign-off)
**Documented by:** implementer agent
**Status:** signed off; awaiting root-canonical doc update + first M4 sprint build

#### Bet 1 — Shared-Everything Multi-Process (SEM)

The default process model is **single renderer process per browser instance,
N typed-isolated contexts inside it**. Per-origin state is the DOM, CSSOM,
JS globals, and layout tree. Per-process shared state is the Vortex heap,
Gyre layout engine, parser, font system, and standard library.

**Implications for existing crates:**

- `spiral-context` (NEW): capability-typed API surface. Per-origin
  `Context` handle. Brand types for `File`, `Socket`, `Time`, `Rand`
  capabilities. Skeleton lands in M4; runtime lands in M25–M36.
- `spiral-sandbox`: shifts from "OS-level sandbox per renderer process"
  to "capability-typed API surface, with OS-level sandbox as the
  *optional* per-origin escalation path." OS-level sandbox becomes a
  per-origin toggle, not the default.
- `spiral-vortex`: Vortex `Isolate` abstraction (logical JS context with
  its own globals, stack, and GC roots) is designed in M4–M6 to be
  runtime-cheap. Multiple isolates share one Vortex heap.

**Implications for the IPC layer:**

- `spiral-ipc` continues to provide the browser↔renderer transport,
  but the renderer is now single-process. IPC becomes the channel to
  the *browser process* (UI chrome, networking, GPU), not between
  renderers. The `IpcTransport` trait is unchanged.

**Security argument:** weaker than Chromium's process walls; stronger
than Ladybird's flat address space. The capability-typed API surface
makes "you can't open /etc/passwd" a *compile-time* property, not a
runtime syscall check.

**Risk:** Spectre-class in-process attacks. Mitigation: branch-prediction-
resistant layout for secret-dependent data; no raw pointers in the shared
arena; constant-time where crypto-relevant.

**Full writeup:** [`docs/architecture/design/shared-everything.md`](architecture/design/shared-everything.md)

#### Bet 2 — Vortex is JIT-optional, bytecode-first

The JIT (Cranelift-based baseline compiler) is **deferred** to a
real-world profile gate at M25. The default v0.1 ships tree-walker →
bytecode VM only. The bytecode format and IC structure are designed
JIT-friendly from day one, so the future JIT is a compiler, not a
rewrite.

**No change to the v0.1 plan in M4–M24.** The change is the *order*:
JIT is no longer a planned v0.1 deliverable.

**Implication for Test262 targets:** Test262 pass targets are revised
downward for the bytecode-VM-only world:

| Phase | Engine tier | Test262 pass target (was → is) |
|-------|-------------|--------------------------------|
| A (M4–9) | Tree-walker | 5–10% (unchanged) |
| B (M10–24) | Bytecode VM | 30–40% (unchanged) |
| C (M25–42) | Baseline JIT (IF profiling demands) | 60% (was 60%; now conditional) |
| D (M43–60) | Optimising JIT (IF JIT shipped) | 80% (was 80%; now conditional) |

The JIT-ship-or-not decision is gated on the M25 profile pass over a
representative workload (NYT, YouTube landing, Gmail, GitHub, a React
SPA, a WebGL game). The decision is *not* ideological.

#### Bet 3 — `spiral-filter` as a compile-time policy engine (NEW CRATE)

**Crate:** new — `crates/spiral-filter/`

**Position in pipeline:** between the network layer and the HTML parser.
Receives raw HTTP body bytes. Produces a *transformed* HTML+CSS document
with worst-offender ads removed or constrained. The runtime never sees
the offending markup.

**Default policy (user-approved 2026-06-14):** "Worst offenders only."

- **Block:** banner ads that break layout, popups, autoplay video and
  audio, interstitials, large sticky ads that cover content.
- **Allow:** reasonable, well-behaved ads. The page renders. Revenue
  still flows to good stewards.
- **No third-party tracking.** Period. No "acceptable ads" program
  that requires telemetry.

**Authority model:** seed with Coalition for Better Ads "Better Ads
Standards" + a curated top-100 worst-offender overlay. Community
contributions from M18+. Stewardship score per domain, opt-in for
site owners.

**User-tunable:** slider from "block nothing" to "block almost
everything." Default = "block worst offenders only."

**Why clever:** uBlock Origin blocks at runtime, after the page has
paid the cost. Spiral avoids the cost entirely. The ad-blocker is
also a *performance optimisation that happens to be a privacy
feature.* The downstream cost in Vortex, Gyre, and the renderer is
*lower* than the unfiltered equivalent.

**Phase:** M4 (crate skeleton + surgical default policy).

**Dep graph position:**

```
spiral-filter (M4)
  └─ depends on: spiral-dom (DOM types for transformation)
                 spiral-css (CSS types for cosmetic injection)
                 spiral-net (request URL inspection)
```

#### Bet 4 — Persistent renderer / warm caches

**What:** When a tab is idle, checkpoint Vortex heap + layout tree + DOM
hash to a memory-mapped file. On revisit, mmap it and lazy-fill what
changed.

**Phase:** M30+ for the Vortex heap checkpoint; M36+ for the layout
tree; M42+ for the full document checkpoint.

**No change to M4–M29 plan.** Each step is independently useful; the
later ones depend on the earlier ones landing.

**Memory accounting:** mmap'd pages count against the tab's budget
*only when touched*. 5 hot tabs + 45 warm tabs ≈ 5×hot + 0.5×warm,
not 50×hot.

#### Three new crates, one process model shift, one policy change

The four bets produce three new crates and a process model shift. They
are summarised in [`docs/active_context.md`](active_context.md) and
authorised there.

### June 2026 — Active (pre-design-pass)

Phase 1 is complete. Phase 2 first sprint (Month 4) is queued. No crate
implementations from Phase 2 have shipped yet. The design pass is the
last action before the first Phase 2 sprint begins.

---

## Memory Budgets (NEW — design pass)

Per-phase memory budgets are now part of the architecture. They are
CI-gated. Exceeding a budget fails the build.

| Scenario | M4–9 | M10–24 | M25–42 | M43–60 (v0.1) |
|----------|------|--------|--------|---------------|
| Cold start to interactive (single tab) | < 800 ms | < 400 ms | < 250 ms | < 200 ms |
| Idle tab resident (backgrounded) | < 80 MB | < 50 MB | < 30 MB | < 25 MB |
| Active tab (NYT-class) | < 300 MB | < 200 MB | < 150 MB | < 120 MB |
| 5-tab session (1 active, 4 idle) | < 700 MB | < 400 MB | < 250 MB | < 200 MB |

**Workload:** NYT front page on a clean profile, all scripts allowed,
default filter policy, fresh session.

**How measured:** a CI job opens a real headless Spiral instance, loads
the page, idles for 30 seconds, snapshots RSS. The job fails on budget
breach. The job is added in M4 (with relaxed budgets) and tightened each
phase.

---

## WPT Targets (NEW — design pass)

Per-phase Web Platform Test pass targets. These are the *gates* for each
phase's exit criteria.

| Suite | M4–9 | M10–24 | M25–42 | M43–60 (v0.1) |
|-------|------|--------|--------|---------------|
| `css/css-box/` | 25% | 40% | 50% | 60% |
| `css/css-position/` | 20% | 40% | 50% | 60% |
| `css/css-flexbox/` | 0% | 30% | 50% | 60% |
| `css/css-grid/` | 0% | 0% | 30% | 50% |
| `html/semantics/` (scripting) | 20% | 35% | 50% | 60% |
| `html/semantics/embedded-content/` | 30% | 50% | 65% | 75% |
| Test262 (Vortex) | 5–10% | 30–40% | 60% (if JIT) | 80% (if JIT) |

The Test262 row reflects Bet 2: the high percentages are conditional on
JIT shipping, which is gated on the M25 profile.

---

## Build Sign-Off Checklist (pre-M4-sprint)

Before the implementer agent begins the M4 first sprint, the following
must be true. This delta file is the SSOT until `ARCHITECTURE.md` is
updated to match.

- [ ] User has reviewed and signed off on the four architectural bets
      (✅ 2026-06-14)
- [ ] `docs/active_context.md` reflects the new thesis, four bets, and
      three new crates (✅ 2026-06-14)
- [ ] `docs/architecture/design/shared-everything.md` exists with the full
      Bet 1 writeup (✅ 2026-06-14)
- [ ] `docs/archives/phase1-tasks.md` is marked complete (✅ 2026-06-14)
- [ ] `ROADMAP.md` is updated to reflect the new memory budgets, WPT
      targets, and the three new crates (⏳ — next pass)
- [ ] `PLAN.md` is updated to reflect the new thesis and the four
      bets (⏳ — next pass)
- [ ] `ARCHITECTURE.md` is updated to reflect the shared-everything
      model, the SEM process diagram, and the new `spiral-filter` /
      `spiral-context` / `spiral-media` crates (⏳ — next pass)
- [ ] `AGENTS.md` is updated to include `spiral-filter`,
      `spiral-context`, and `spiral-media` working guidelines (⏳ —
      next pass)
- [ ] `CODEX.md` is updated for the new crate list and pipeline
      diagram (⏳ — next pass)

The next pass will land all root-canonical doc updates in one commit,
gated on user review of this delta file.
