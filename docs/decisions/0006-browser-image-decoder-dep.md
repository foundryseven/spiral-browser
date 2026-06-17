# ADR 0006: Add spiral-imagedecoder Dependency to spiral-browser

**Status:** Accepted
**Date:** 2026-06-17
**Deciders:** the pair programming team
**Related:** [README.md](file:///Users/james/spiral-browser/README.md), [display_list.rs](file:///Users/james/spiral-browser/crates/spiral-browser/src/display_list.rs), [audit-doc-drift.sh](file:///Users/james/spiral-browser/scripts/audit-doc-drift.sh)

---

## Context

To incorporate the Spiral Browser logo on the headless Hello World startup page, the display list builder in `spiral-browser` must load and decode the logo's PNG bytes.

Image decoding in the codebase is encapsulated within the `spiral-imagedecoder` crate. Therefore, `spiral-browser` must depend on `spiral-imagedecoder`.

As `spiral-browser` is the top-level binary of the project, it sits at the bottom of the dependency graph (consuming other crates). Depending on `spiral-imagedecoder` is a clean downstream dependency, but it must be explicitly allowlisted in the dependency graph validation rules and in the doc-drift audit script to maintain compliance.

---

## Decision

Add `spiral-imagedecoder` as a regular dependency of `spiral-browser` in `crates/spiral-browser/Cargo.toml`. Add `"spiral-browser spiral-imagedecoder"` to the `rules_allow` array in `scripts/audit-doc-drift.sh`.

---

## Consequences

- **Positive:** `spiral-browser` is able to decode PNG assets (like the logo) to build rich hello-world display lists.
- **Negative:** None. `spiral-browser` is the top-level binary process, so downstream dependency accretion is expected.
- **Migration:** No migration required.

---

## Alternatives considered

### Option A: Read raw pixels in spiral-browser
Embedding raw RGBA8 pixels instead of PNG bytes directly into the source code was considered, but rejected because a raw RGBA8 representation of a 160x160 image is less compact than a compressed PNG, and it bypasses testing the codebase's own `spiral-imagedecoder` crate.

---

## Wiring & Integration

- **Crates affected:** `spiral-browser`, `spiral-imagedecoder`
- **Call sites:** `crates/spiral-browser/src/display_list.rs:31` calls `ImageDecoder::new()` and `decoder.decode()`.
- **Test coverage:** `display_list::tests::hello_list_has_six_ops` verifies the display list contains the decoded image operation.
