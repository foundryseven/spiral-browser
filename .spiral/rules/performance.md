---
paths:
  - "benches/**"
  - "crates/**"
---

# Performance Standards

> **Read first.** This file is the operative contract for
> performance verification on the hot engines (Vortex, Gyre, Fmt).
> The companion workflow gate table lives in
> [`AGENTS.md`](../AGENTS.md) and the gate-level detail lives in
> [`.spiral/rules/workflow.md`](workflow.md). Where this file and
> `AGENTS.md` disagree, this file wins for perf-specific questions;
> `workflow.md` wins for "what tool, when".

## Workflow Tools (mandatory)

| Moment | MUST run | Why |
|--------|----------|-----|
| Before claiming a perf-related packet complete | `cargo bench --workspace` | Confirms no regression > 5% on the touched crate's microbenchmarks. |
| When claiming an optimisation | `cargo bench --bench <name> -- --save-baseline before` → apply change → `cargo bench --bench <name> -- --baseline before` | Produces the Criterion HTML report / terminal diff the §3 gate requires. |
| When touching Vortex, Gyre, or Fmt hot paths | `just verify-packet <crate>` after the benchmark | Catches lint and test regressions introduced by the same change. |

To keep custom engines like **Vortex** (JavaScript VM) and **Gyre**
(layout) responsive and fast, performance verification MUST be
treated as a first-class citizen. Performance regressions are
build breaks.

## 1. Benchmarking Scope

Microbenchmarks MUST be created when implementing or refactoring
performance-critical code pathways, particularly:

* **Vortex VM:** New bytecode compiler rules, VM opcodes, GC
  trace/sweep passes, and runtime builtins.
* **Gyre Layout:** Box model computations, block/flex/grid
  formatting passes, and display-list generation.
* **Fmt Parser:** HTML/CSS tokenisation and tree-construction hot
  paths.

A claim that "this change is perf-related" without a Criterion
target under `benches/` or `crates/<name>/benches/` MUST be
treated as a hollow claim by the reviewer.

## 2. Framework Standards

The workspace uses [Criterion](https://crates.io/crates/criterion)
for stable microbenchmarking:

* Benchmark suites are located in `benches/` at the root of the
  workspace or in crate-local `benches/` directories.
* Benchmarks MUST measure operations on varying input sizes (e.g.,
  small, typical, and large payloads) to capture algorithmic
  complexity behaviour.
* Run benchmarks locally before submit:
  ```bash
  cargo bench
  ```

## 3. Performance Regression CI Gates

* **Regression Tolerance:** Any pull request introducing a
  degradation greater than **5%** on core engine microbenchmarks
  MUST be investigated. "Investigated" means a root-cause note
  in the PR description; the implementer MUST NOT merge a > 5%
  regression without one.
* **Baseline Comparison:** Automated PR workflows run `cargo bench`
  against the target branch (`main`) and compare statistical
  deviation.
* **Optimisation Proofs:** When claiming a performance improvement,
  the implementer MUST provide a Criterion HTML report or
  terminal diff showing the percentage gain. "Feels faster" is not
  a proof.

## 4. Writing Conventions

* All benchmark code and documentation MUST use **Australian
  spelling** (e.g., `optimise`, `analyse`, `programme`,
  `organisation`, `realise`).
