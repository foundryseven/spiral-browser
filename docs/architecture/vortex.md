# `spiral-vortex` (Vortex) — JavaScript Engine

> **Brand:** Vortex. **Crate:** `spiral-vortex`. **Scope:**
> lexer → parser → AST → bytecode compiler → interpreter
> → GC, with a V8 oracle harness under the `v8` feature
> flag. **Status:** Step 1.6 / Packet 1.6.1 shipped
> (GC rewrite: `VortexHeap` + per-origin `OriginArena` +
> `TaggedCell` + `GcKey` + mark-sweep; old `Heap` type
> retired). Pre-1.6.1 GC was a 4-array mark-sweep; post-1.6.1
> it is origin-tagged. See
> [`docs/architecture/design/vortex-heap.md`](design/vortex-heap.md)
> for the design rationale. All 4 of the
> `spiral-vortex` orphans closed in 1.6.1.

Vortex is Spiral's from-scratch JavaScript engine,
written entirely in safe Rust. It implements
ECMAScript (ES2015+) from the ground up: lexer,
parser, AST, bytecode compiler, interpreter,
mark-sweep GC, and (future) a baseline JIT. No
QuickJS, no V8 in the production path, no Boa.

See `docs/decisions/0002-vortex-from-scratch.md` for
the posture decision (and the abandoned rquickjs →
rusty_v8 two-step plan).

---

## Public surface (Step 1.6 / Packet 1.6.1)

```rust
// Errors + top-level entry point.
pub use error::{VortexError, VortexResult};
pub use runtime::Vortex;
pub use value::JsValue;

/// End-to-end entry point (Packet 1.6.5+).
/// Parses, compiles, and runs `source`; returns the
/// expression-statement result or `Err(VortexError)`.
pub fn vortex_eval(source: &str) -> VortexResult<JsValue>;

// Lexer → parser → AST.
pub struct Lexer { … }
pub struct Parser { … }
pub struct Program { … }              // AST root
pub enum Stmt { … }                  // statement variants
pub enum Expr { … }                  // expression variants

// Bytecode compiler.
pub struct Bytecode { … }            // instruction stream
pub struct BytecodeCompiler { … }
pub enum Op { … }                    // VM instruction

// VM.
pub struct VM { … }
pub enum Value { … }                 // JS value representation
pub struct Handle<T> { … }           // GC handle

// GC (post-1.6.1 rewrite; old `Heap` retired).
pub struct VortexHeap { … }          // owner of all arenas
pub struct OriginArena { … }         // one per origin; owns TaggedCells
pub struct TaggedCell { … }          // 4-byte header (origin tag + mark bit + size)
pub struct GcKey { … }               // versioned + branded key into the arena
```

The M4.4 skeleton has the **module layout**; the
post-1.6.1 implementation has the **GC rewrite**
(`VortexHeap` / `OriginArena` / `TaggedCell` /
`GcKey` + mark-sweep). M4.5 Item 9 (the
end-to-end slice: lexer → parser → AST →
interpreter) lands in Packet 1.6.5; it is
the next "what needs picking" item in
[`docs/implementation_tracker.md`](../implementation_tracker.md).

---

## Internal layout

```
spiral-vortex/src/
├── lib.rs           — public surface, lib-level docs
├── error.rs         — VortexError, VortexResult
├── lexer/           — ES lexer (Packet 1.6.5+)
├── parser/          — ES parser (Packet 1.6.5+)
├── ast/             — AST types (Packet 1.6.5+)
├── value/           — JS value types (number, string,
│                     object, function, …) (1.6.5+; JsValue shipped in 1.6.1)
├── gc/              — mark-sweep GC (1.6.1 SHIPPED)
│                     — VortexHeap, OriginArena, TaggedCell, GcKey
├── vm/              — interpreter (1.6.5+)
├── runtime/         — Vortex struct + builtins (Math, JSON, console, …) (1.6.5+)
├── v8/              — V8 oracle harness (gated on `v8`
│                     feature flag) (5+)
├── dom_bindings/    — DOM integration (Phase 6+)
├── event_loop/      — microtask + macrotask scheduling (Phase 6+)
└── builtins/        — JS standard library (Array, Object,
                      Promise, …) (Phase 6+)
```

The skeleton is in place. The GC is implemented
(post-1.6.1); the lexer/parser/ast/vm are stub
modules awaiting Packet 1.6.5.

---

## Constraints

- **No QuickJS, no Boa, no V8 in the production
  path.** ADR 0002 § Decision.
- **`rusty_v8` is a dev-dependency under the `v8`
  feature flag.** The flag is off by default. The
  CI oracle runs the JS test corpus through both
  Vortex and V8 and diffs outputs. ADR 0002.
- **Safe Rust only.** No `unsafe` blocks in the
  interpreter or the GC. The bytecode compiler may
  use `unsafe` for hot loops, with documented
  justification.
- **Spec-faithful.** Vortex targets the latest
  ECMAScript specification. Deviations require
  an ADR.

---

## Test posture

- 84 tests post-1.6.1 (GC went 41 → 84 across the
  rewrite): 22 new GC tests cover
  `VortexHeap` / `OriginArena` / `TaggedCell` /
  `GcKey` + mark-sweep correctness
  (allocation, trace, mark, sweep, drop-arenas,
  cross-origin isolation).
- Packet 1.6.5 (the end-to-end slice) adds the
  first functional tests: lexer tests, parser
  tests, and an end-to-end test that
  `vortex_eval("1 + 2")` returns `3`.
- Phase 5+ adds the V8 oracle harness: a corpus of
  ~200 JS snippets run through both Vortex and
  V8, with output diffing as the test.
- Total projected: ~500 tests for Vortex, of which
  ~200 are the oracle corpus.

---

## Do-not-touch zones (post-1.6.1)

- The `lib.rs` module declarations. Adding a new
  module is a public-surface change.
- The `error.rs` `VortexError` type. Changing
  variants is a breaking change.
- The `v8/` feature flag wiring. Removing the flag
  is a posture change and needs an ADR.
- The `VortexHeap` / `OriginArena` / `TaggedCell` /
  `GcKey` types and the 4-byte header layout.
  These are the wire format for the multi-origin
  isolation bet (Bet 1); changing them is a
  posture change.

---

## Related

- `docs/decisions/0002-vortex-from-scratch.md` —
  the from-scratch posture.
- `docs/glossary.md` — the Vortex brand entry.
- `AGENTS.md` § `spiral-vortex (Vortex)` — working
  rules.
- `docs/audits/2026-06-15-baseline.md` §1.6 — the
  M4.4 / M4.5 / M4.6 priority list for Vortex.
- `docs/architecture/design/vortex-heap.md` — the GC design.
