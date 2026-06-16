---
paths:
  - "**/*.rs"
---

# Unsafe Code Standards

Safety is a core pillar of Spiral. As a general rule, **Safe Rust is the default**. Unsafe Rust must only be introduced when strictly necessary, with rigorous documentation and peer-review guarantees.

## 1. Safety Proofs (The `// SAFETY:` Comment)

Every single `unsafe` block or `unsafe fn` declaration MUST be preceded by a detailed `// SAFETY:` comment. A missing safety comment is treated as a build break.

The safety comment must:
* Document the specific invariants that the compiler cannot verify.
* Explain why the caller or block ensures those invariants are upheld.
* Prove that Undefined Behaviour (UB) is impossible under all execution paths.

### Standard Format

```rust
// SAFETY: The pointer `ptr` is checked to be non-null and properly aligned
// before dereferencing. The backing buffer has a verified lifetime that
// outlives the current stack frame.
let value = unsafe { *ptr };
```

## 2. Unsafe Code Restrictions

Unsafe blocks are strictly prohibited in the following crates:
* `spiral-context` (variance and lifetime branding relies on strict type system isolation).
* `spiral-filter` (pure logic, no performance-driven need for unsafe memory access).
* `spiral-fmt` (parser recoverability and safety must be guaranteed).
* `spiral-dom` (arena indexes are managed safely).
* `spiral-net` (network logic is safe).

Unsafe is permitted only with benchmark-backed justification (proving significant performance gains or for FFI/OS bounds) in:
* `spiral-vortex` (bytecode VM interpreter and garbage collector).
* `spiral-sandbox` (platform-specific OS boundaries and sandboxing profiles).
* `spiral-ipc` (low-level named pipe transport framing optimization).

## 3. Review and Verification

* **Central Registry:** All unsafe blocks must be registered in [unsafe_registry.md](file:///Users/james/spiral-browser/docs/security/unsafe_registry.md).
* **Miri Invariant:** Any crate containing `unsafe` code must run cleanly under Miri. Undefined behavior caught by Miri is a blocker.
