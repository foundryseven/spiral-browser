# `spiral-context` (Context) — Capability-Typed Page Context

> **Brand:** *(unbranded).* **Crate:** `spiral-context`.
> **Scope:** page-origin brands, capability sets,
> context types. **Status:** M4.4 crate skeleton in
> place; first functional slice is M4.5.

`spiral-context` is Spiral's capability-typed page
context. It implements **Bet 1** (the shared-everything
multi-process architecture) at the type level. A
`Context<'brand, Mode>` is tagged with a brand (the
origin) and a mode (`InProcess` or `Escalated`); the
type checker enforces that capabilities can flow
only through compatible contexts.

See `docs/architecture-shared-everything.md` and
`docs/design-capability-types.md` for the design.

---

## Public surface (target, M5+)

```rust
// Brand — compile-time tag, no runtime representation.
pub struct Brand<'brand> { … }       // invariant-lifetime
pub struct CapabilitySet<'brand> { … }

// Context — the page-origin context.
pub struct Context<'brand, Mode> { … }
pub trait ContextOps { … }

// Mode tags.
pub struct InProcess;                // in-process JS execution
pub struct Escalated;                // sandboxed JS execution

// Capability types.
pub struct NetCap;                   // network access
pub struct FsCap;                    // filesystem access
pub struct DomCap;                   // DOM access
pub struct ClockCap;                 // clock access
pub struct RngCap;                   // RNG access
```

The M4.4 skeleton has the **module layout** but not
the implementation. M4.5 fills in the types.

---

## Internal layout

```
spiral-context/src/
├── lib.rs           — public surface, brand documentation
├── brand.rs         — Brand<'brand> (invariant lifetime) (M4.5+)
├── caps.rs          — CapabilitySet + capability types (M4.5+)
├── context.rs       — Context<'brand, Mode> (M4.5+)
└── dom.rs           — DOM context integration (M5+)
```

The skeleton has the files but they're empty
stubs; M4.5 implements the type-level surface.

---

## Constraints

- **No `unsafe`.** The brand mechanism relies on
  `PhantomData<fn(&'brand ()) -> &'brand ()>` for
  variance. Any `unsafe` in the context types is
  a red flag.
- **No `Send` / `Sync` on brand-tagged types.** A
  brand is a *compile-time* tag; transferring it
  across threads would require explicit
  threading-type machinery (M5+).
- **PhantomData only.** Brands have no runtime
  representation. Zero-cost.
- **Capability grants are explicit.** No implicit
  capabilities. A capability that is not granted
  via `CapabilitySet` is not available.

---

## Test posture

- 0 functional tests in M4.4 (skeleton only).
- M4.5 adds the brand-mechanism tests (compile-fail
  tests for variance, send/sync tests, capability
  grant tests).
- M5+ adds the integration tests with
  `spiral-filter` and `spiral-vortex`.

Total projected: ~20 compile-fail tests + ~10
integration tests.

---

## Do-not-touch zones (M4.4)

- The `Brand<'brand>` shape. Changing the variance
  is a breaking change.
- The `CapabilitySet<'brand>` API. Adding capabilities
  requires an ADR.
- The mode tags (`InProcess`, `Escalated`). Adding
  a mode requires an ADR.

---

## Related

- `docs/architecture-shared-everything.md` — Bet 1
  the architectural bet.
- `docs/design-capability-types.md` — the
  capability-typed system design.
- `AGENTS.md` § `spiral-context` (forthcoming).
