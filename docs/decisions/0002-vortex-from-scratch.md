# ADR 0002: Vortex is a from-scratch JavaScript engine, not a V8 wrapper

**Status:** Accepted (posture change, supersedes the rquickjs → rusty_v8 two-step plan)
**Date:** 2026-06-14
**Deciders:** James Pinnell
**Related:** `docs/progress_ledger.md` (2026-06-14 entry "Vortex posture change"), `crates/spiral-vortex/src/lib.rs`, `docs/audits/2026-06-15-baseline.md`

---

## Context

The original Vortex plan was a **two-step** migration:

1. **Step 1 (M2–M3):** Vortex wraps `rquickjs`, a
   Rust binding to QuickJS. Gets the engine into the
   browser quickly, gets the team familiar with the JS
   runtime surface.
2. **Step 2 (M4+):** Vortex switches from `rquickjs` to
   `rusty_v8` (V8 via the official V8 Rust bindings).
   Vortex becomes V8-backed, the same engine as
   Chrome and Node.

This was the M0 plan. The reasoning at the time: the JS
engine is a 10-year project, and the user has a browser
to ship in 24 months. Use QuickJS to get unblocked, then
swap in V8 once the surface is stable.

The posture was wrong. Three reasons:

1. **V8 via Rust is not "the V8 path that V8 itself
   uses."** The whole point of choosing V8 was
   correctness and JS-compliance. `rusty_v8` re-exposes
   V8's API in Rust, but the API surface that you build
   against is a thin object system on top of V8 handles.
   To get value from V8 you have to write *V8-shaped*
   Rust code; that's not a small lift and it's not a
   value-add. The cost-benefit was inverted: pay the V8
   complexity tax but get a sandboxed, embeddable V8
   you can't actually customize the way you would
   customize your own engine.

2. **"Our tech where it matters" (user mandate,
   2026-06-15).** A V8 wrapper is a thin veneer on
   someone else's engine. It is not Spiral's tech. The
   reason to build a browser from scratch — the
   posture that runs through the entire M4 audit — is
   that the **engine layer is the moat**. A V8 wrapper
   has no engine layer.

3. **The M4 baseline audit found a different path:**
   keep `rusty_v8` in the workspace, but gate it behind
   a `v8` Cargo feature flag, and use it **only as a CI
   compliance oracle**. The Vortex test harness runs JS
   snippets through both Vortex and V8 and compares
   outputs. That gives correctness validation without
   coupling the production engine to V8.

---

## Decision

Vortex is a **from-scratch** JavaScript engine, written
entirely in safe Rust. No QuickJS, no V8, no Boa.

Concretely:

1. The Vortex implementation is a hand-written
   lexer → parser → AST → bytecode compiler → stack-based
   interpreter pipeline, with a mark-sweep GC.
2. `rusty_v8` is kept as a **dev-dependency under the
   `v8` feature flag** (off by default). When the flag
   is on, the CI harness runs a corpus of JS snippets
   through both Vortex and V8 and compares outputs.
   V8 is the oracle, not the engine.
3. The crate's `lib.rs` documents this posture at the
   top: "**not the production engine — it exists only
   as a CI compliance oracle**."
4. The M2–M3 rquickjs intermediate step is removed
   from the roadmap entirely.

The implementation surface is the standard
from-scratch JS engine pipeline:

```
source ──▶ Lexer ──▶ Tokens ──▶ Parser ──▶ AST
                                            │
                                      BytecodeCompiler
                                            │
                                          Bytecode
                                            │
                                       VM (interpreter)
                                            │
                                       Builtins / GC
                                            │
                                  DOM bindings / Event loop
```

A future baseline JIT (Cranelift) is on the roadmap
post-M4.5; the interpreter is the M4.5 minimum.

---

## Consequences

- **Positive:**
  - The engine layer is Spiral's. A V8 wrapper would
    never give us that.
  - `rusty_v8` still validates correctness: any
    discrepancy in the oracle harness is a Vortex bug.
  - The interpreter is small (M4.5 minimum) and we
    ship incremental features (lexer → parser →
    interpreter → GC → DOM bindings) without dragging
    V8's build system into CI.
  - The M4 audit's "M4 Vortex bugs" finding (the
    `gc_live_count` returns 0 bug noted in
    `docs/audits/2026-06-15-baseline.md:76`) is a bug
    in *our* code, not in V8, so we own the fix path.
- **Negative:**
  - We are now building a JavaScript engine. The
    10-year project. The work is real and large.
  - The compliance oracle only catches *output
    differences*; it does not validate performance,
    and it does not catch Vortex bugs that V8 also
    has (e.g. semantics around edge cases in the
    spec).
  - Adding a `v8` feature flag adds CI complexity.
    We have to build Vortex twice — once with, once
    without — to be sure the gated path is sound.
- **Migration:** None required. Vortex is a new
  crate; nothing exists in the M0–M3 codebase that
  depends on `rquickjs`.

---

## Alternatives considered

### Option A: Keep the rquickjs → rusty_v8 two-step plan

Rejected. The "engine layer is the moat" posture makes
a wrapper-engine approach self-defeating. The user
mandate of 2026-06-15 ("our tech where it matters")
makes the case explicit.

### Option B: Drop JS from the browser entirely

Rejected. JS is the user-facing surface of the modern
web. A browser without JS is not a browser. A
JS-optional browser (M4.5+) is fine, but the engine
must exist.

### Option C: Adopt an existing Rust JS engine (Boa, Kiesel)

Rejected on the same grounds as Option A. The moat
is in *our* implementation, not in a third-party
Rust binding. The work is the work.

### Option D: Use V8 directly via FFI (not via rusty_v8)

Rejected because the FFI surface is the same shape
of problem as `rusty_v8` (a thin wrapper around V8
handles), and `rusty_v8` is a maintained binding
that is closer to the upstream V8 API. If we want
V8-shaped Rust code, `rusty_v8` is the right binding.

---

## Wiring & Integration

- **Crates affected:** `spiral-vortex` (the new crate,
  not the old `spiral-js` which was renamed in
  ADR 0003).
- **Call sites:** `spiral-vortex::Lexer`,
  `spiral-vortex::Parser`, `spiral-vortex::Bytecode
  Compiler`, `spiral-vortex::VM`, `spiral-vortex::GC`
  are the new public entry points.
- **Test coverage:** the M4.5 sprint will add
  interpreter tests + a V8 oracle harness. As of
  M4.4 the skeleton is in place but the engine is
  not yet functional; tests land with M4.5 Item 9.
- **Reachable from a real surface:** the Vortex
  skeleton is wired (it compiles, the modules are
  reachable, the lib.rs documents the posture), but
  the engine itself is not yet executed. The
  "wired" rule applies to the *crate*, not the
  *engine* — the engine lands in M4.5. This is
  documented in the M4.4.1 Item 4 / Item 8 ledger
  entries and is a known staged rollout.

---

## Notes

The M4 baseline audit lists 13 open Vortex defects
(M4.4, 4.5, 4.6 priority). They are owned by Vortex
and are in scope for the M4.5 sprint.
