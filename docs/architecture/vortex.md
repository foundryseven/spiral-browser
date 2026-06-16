# `spiral-vortex` (Vortex) — JavaScript Engine

> **Brand:** Vortex. **Crate:** `spiral-vortex`. **Scope:**
> lexer → parser → AST → bytecode compiler → interpreter
> → GC, with a V8 oracle harness under the `v8` feature
> flag. **Status:** M4.4 skeleton; first functional slice
> is M4.5 Item 9.

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

## Public surface (target, M5+)

```rust
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

// GC.
pub struct Heap { … }                // mark-sweep
```

The M4.4 skeleton has the **module layout** but not
the implementation. M4.5 Item 9 delivers the first
end-to-end slice (lexer → parser → AST → console.log
interpreter).

---

## Internal layout

```
spiral-vortex/src/
├── lib.rs           — public surface, lib-level docs
├── error.rs         — VortexError, VortexResult
├── lexer/           — ES lexer (M4.5+)
├── parser/          — ES parser (M4.5+)
├── ast/             — AST types (M4.5+)
├── value/           — JS value types (number, string,
│                     object, function, …) (M4.5+)
├── gc/              — mark-sweep GC (M4.6+)
├── vm/              — interpreter (M4.5+)
├── runtime/         — builtins (Math, JSON, console, …) (M4.5+)
├── v8/              — V8 oracle harness (gated on `v8`
│                     feature flag) (M5+)
├── dom_bindings/    — DOM integration (M6+)
├── event_loop/      — microtask + macrotask scheduling (M6+)
└── builtins/        — JS standard library (Array, Object,
                      Promise, …) (M6+)
```

The skeleton is in place. Each module is currently
empty (or contains only a stub `mod.rs`); M4.5+
fills them in.

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

- 0 functional tests in M4.4 (the skeleton compiles
  but the engine is not yet implemented).
- M4.5 Item 9 adds the first slice: lexer tests,
  parser tests, and an end-to-end test that
  `console.log("hello, world!")` works.
- M5+ adds the V8 oracle harness: a corpus of
  ~200 JS snippets run through both Vortex and
  V8, with output diffing as the test.
- Total projected: ~500 tests for Vortex, of which
  ~200 are the oracle corpus.

---

## Do-not-touch zones (M4.4)

- The `lib.rs` module declarations. Adding a new
  module is a public-surface change.
- The `error.rs` `VortexError` type. Changing
  variants is a breaking change.
- The `v8/` feature flag wiring. Removing the flag
  is a posture change and needs an ADR.

---

## Related

- `docs/decisions/0002-vortex-from-scratch.md` —
  the from-scratch posture.
- `docs/glossary.md` — the Vortex brand entry.
- `AGENTS.md` § `spiral-vortex (Vortex)` — working
  rules.
- `docs/audits/2026-06-15-baseline.md` §1.6 — the
  M4.4 / M4.5 / M4.6 priority list for Vortex.
- `docs/design-vortex-heap.md` — the GC design.
