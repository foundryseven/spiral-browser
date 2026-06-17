---
paths:
  - "**/*.rs"
---

# Unsafe Code Standards

> **Read first.** This file is the operative contract for any
> `unsafe` block or `unsafe fn` in the workspace. The companion
> workflow gate table lives in [`AGENTS.md`](../AGENTS.md) and
> the gate-level detail lives in
> [`.spiral/rules/workflow.md`](workflow.md). Where this file and
> `AGENTS.md` disagree, this file wins for unsafe-specific
> questions; `workflow.md` wins for "what tool, when".

## Workflow Tools (mandatory)

| Moment | MUST run | Why |
|--------|----------|-----|
| Before adding any `unsafe` block or `unsafe fn` | `cargo miri test -p <crate>` (or `cargo miri setup && cargo miri test`) locally | Catches UB at write-time, not in CI. |
| Before claiming an `unsafe`-touching packet complete | `./scripts/audit-orphan-exports.sh` | Confirms the unsafe surface has an external consumer (Wiring & Integration). |
| Before merging | `just verify-packet <crate>` | Wraps fmt + clippy + test + audit into one scoped command. |
| After any new `unsafe` block is added | `bin/spiral-context.sh` to surface `docs/security/unsafe_registry.md` | The registry MUST be updated in the same commit as the `unsafe` addition. |

Safety is a core pillar of Spiral. As a general rule, **Safe Rust
is the default**. Unsafe Rust MUST only be introduced when
strictly necessary, with rigorous documentation and peer-review
guarantees.

## 1. Safety Proofs (The `// SAFETY:` Comment)

Every single `unsafe` block or `unsafe fn` declaration MUST be
preceded by a detailed `// SAFETY:` comment. A missing safety
comment is treated as a build break and rejected by `cargo clippy`
(via `#![deny(missing_safety_doc)]` at the crate root for
unsafe-allowed crates; see §2).

The safety comment MUST:
* Document the specific invariants that the compiler cannot
  verify.
* Explain why the caller or block ensures those invariants are
  upheld.
* Prove that Undefined Behaviour (UB) is impossible under all
  execution paths.

A reviewer MUST reject any `unsafe` block whose `// SAFETY:`
comment is generic ("we trust the caller"), restates the function
signature, or names invariants without naming the
caller/callee/bounds that uphold them.

### Standard Format

```rust
// SAFETY: The pointer `ptr` is checked to be non-null and
// properly aligned before dereferencing (see
// `validate_alignment` at line N). The backing buffer has a
// verified lifetime that outlives the current stack frame
// (see `'buffer` borrow in `with_buffer`).
let value = unsafe { *ptr };
```

## 2. Unsafe Code Restrictions

Unsafe blocks are strictly prohibited in the following crates:
* `spiral-context` (variance and lifetime branding relies on
  strict type system isolation).
* `spiral-filter` (pure logic, no performance-driven need for
  unsafe memory access).
* `spiral-fmt` (parser recoverability and safety MUST be
  guaranteed).
* `spiral-dom` (arena indexes are managed safely).
* `spiral-network` (network logic is safe).

An `unsafe` block introduced into a forbidden crate is a build
break and MUST be removed in review before merge.

Unsafe is permitted only with benchmark-backed justification
(proving significant performance gains or for FFI/OS bounds) in:
* `spiral-vortex` (bytecode VM interpreter and garbage collector).
* `spiral-sandbox` (platform-specific OS boundaries and
  sandboxing profiles).
* `spiral-ipc` (low-level named pipe transport framing
  optimisation).

A `pub unsafe fn` in any other crate MUST be accompanied by an
ADR under `docs/decisions/` justifying the addition.

## 3. Review and Verification

* **Central Registry:** All unsafe blocks MUST be registered in
  [`docs/security/unsafe_registry.md`](../docs/security/unsafe_registry.md).
  An addition to the registry MUST ship in the same commit as
  the `unsafe` block it documents. A reviewer MUST treat a
  registry lag as a build break.
* **Miri Invariant:** Any crate containing `unsafe` code MUST run
  cleanly under Miri. Undefined behaviour caught by Miri is a
  blocker; the implementer MUST root-cause the UB and either
  rewrite in safe Rust or tighten the `// SAFETY:` proof before
  re-running.
