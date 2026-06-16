---
paths:
  - "benches/**"
  - "crates/**"
---

# Performance Standards

To ensure that custom engines like **Vortex** (JavaScript VM) and **Gyre** (layout) remain responsive and fast, performance verification must be treated as a first-class citizen. 

Performance regressions are considered build breaks.

## 1. Benchmarking Scope

Microbenchmarks must be created when implementing or refactoring performance-critical code pathways, particularly:

* **Vortex VM:** New bytecode compiler rules, VM opcodes, GC trace/sweep passes, and runtime builtins.
* **Gyre Layout:** Box model computations, block/flex/grid formatting passes, and display-list generation.
* **Fmt Parser:** HTML/CSS tokenisation and tree-construction hot paths.

## 2. Framework Standards

The workspace uses [Criterion](https://crates.io/crates/criterion) for stable microbenchmarking:

* Benchmark suites are located in `benches/` at the root of the workspace or in crate-local `benches/` directories.
* Benchmarks must measure operations on varying input sizes (e.g., small, typical, and large payloads) to capture algorithmic complexity behaviour.
* Run benchmarks locally before submit:
  ```bash
  cargo bench
  ```

## 3. Performance Regression CI Gates

* **Regression Tolerance:** Any pull request introducing a degradation greater than **5%** on core engine microbenchmarks must be investigated.
* **Baseline Comparison:** Automated PR workflows run `cargo bench` against the target branch (`main`) and compare statistical deviation.
* **Optimisation Proofs:** When claiming a performance improvement, the implementer must provide a Criterion HTML report or terminal diff showing the percentage gain.

## 4. Writing Conventions

* All benchmark code and documentation must use **Australian spelling** (e.g., `optimise`, `analyse`, `programme`, `organisation`, `realise`).
