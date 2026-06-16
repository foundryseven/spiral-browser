# ADR 0003: Rename `spiral-layout` to `spiral-gyre`; Gyre is Spiral's in-house layout engine

**Status:** Accepted
**Date:** 2026-06-14
**Deciders:** James Pinnell
**Related:** `docs/progress_ledger.md` (2026-06-14 "Engine branding decided" entry), `crates/spiral-gyre/Cargo.toml`, `crates/spiral-gyre/src/lib.rs`, `docs/audits/2026-06-15-baseline.md`

---

## Context

The M0 plan was for Spiral's layout engine to live in
`spiral-layout` and to be implemented on top of Taffy,
a Rust layout library. The reasoning at the time: layout
is 18 months of work, Taffy is a maintained library, and
"using Taffy" is faster than "writing a layout engine."

The M0 plan was wrong for the same reason the rquickjs
plan was wrong (see ADR 0002). Layout is **the**
core-engine moat for a browser — it is what differentiates
one browser from another at the level the user feels.
A Taffy wrapper is a thin veneer on someone else's
implementation. It is not Spiral's tech.

In addition, the codebase had a naming inconsistency:
every other engine (Vortex, Forge) had a brand name, but
the layout engine was just `spiral-layout`. The brand
names are the project's internal language (see
`docs/glossary.md`); layout was a gap.

---

## Decision

1. **Rename the crate:** `crates/spiral-layout/` →
   `crates/spiral-gyre/`. The package name becomes
   `spiral-gyre`. The brand name is **Gyre**.
2. **Drop Taffy from the workspace dependencies.** Gyre
   is in-house from day one; no Taffy in `Cargo.toml`,
   no Month-18 "remove Taffy" milestone.
3. **Implement Gyre from scratch** following the CSS
   spec. The implementation surface is:
   - Box model (foundational — get this right first)
   - Block layout: vertical stacking, margin collapse
   - Flexbox: custom implementation (M10–M11)
   - Grid: custom implementation (M13–M14)
4. The crate's `lib.rs` documents this posture at the
   top: "Gyre is Spiral's custom layout engine. It
   computes the box model, block flow, flex layout,
   and grid layout for every element in the DOM. Gyre
   is fully in-house — no Taffy, no Servo layout code —
   and is the only piece of the engine that is
   genuinely *ours*."

---

## Consequences

- **Positive:**
  - The brand-name pattern is complete (Gyre, Vortex,
    Forge — see `docs/glossary.md`).
  - Gyre is Spiral's. The M0 plan's Taffy dependency
    is gone; we own the implementation, the test
    surface, and the optimisation path.
  - The "Our tech where it matters" mandate (user,
    2026-06-15) is honoured.
  - Box-model work (M4.6) is the foundation for the
    rest of the layout pipeline; a custom
    implementation lets us make the right tradeoffs
    (e.g. margin collapse, table layout, line layout)
    for our use case, not Taffy's.
- **Negative:**
  - 18 months of layout work is now 18 months of
    layout work. The Taffy shortcut is gone.
  - The M4.4.1 sprint does not yet have any layout
    code; the M4.6 sprint delivers the first slice
    (box model + margins). The renaming is
    pre-emptive; the engine is not yet functional.
- **Migration:** the file-level rename was a
  `git mv crates/spiral-layout crates/spiral-gyre`
  preserving history. The package name in
  `Cargo.toml` changed. No other crate imported
  `spiral_layout` at the time of the rename (the
  rename was done before `spiral-gyre` had any
  consumers), so no call-site updates were needed.

---

## Alternatives considered

### Option A: Keep `spiral-layout` on Taffy

Rejected. The "our tech where it matters" posture
applies to layout as much as it does to the JS
engine. A Taffy wrapper has no engine layer.

### Option B: Use Servo's layout code (vendored)

Rejected because Servo's layout code is a 15-year-old
C++ project with its own assumptions about the DOM,
the renderer, and the IPC layer. The cost of vendoring
and adapting Servo layout is comparable to the cost of
writing Gyre from scratch, and the result would not
be Spiral's tech — it would be Servo's tech, adapted.

### Option C: Adopt a different layout library (stretch, morphorm)

Rejected on the same grounds as Option A. The moat
is the implementation.

### Option D: Rename to `spiral-gyre` but defer the implementation rewrite

Accepted (de facto). The rename happened on
2026-06-14; the first slice of Gyre (box model +
margins) lands in M4.6. The M4.4.1 sprint does
not yet contain any layout code; the renamed crate
contains only the crate skeleton.

---

## Wiring & Integration

- **Crates affected:** `spiral-gyre` (renamed from
  `spiral-layout`). The rename is the only change
  in this ADR; the implementation lands in M4.6.
- **Call sites:** `spiral-gyre` does not yet have
  any external consumers in M4.4.1. The first
  consumer is itself: the layout pipeline that
  takes a `Dom` + a `Stylesheet` and produces a
  `LayoutNode` (see `crates/spiral-gyre/src/lib.rs`).
  The current `layout(&self, dom: &Dom, _stylesheet
  : &Stylesheet)` signature takes a `&Stylesheet`
  but does not yet use it — the layout pipeline is
  not connected to the new CSS parser (ADR 0001)
  until M4.6.
- **Test coverage:** 3 lib tests cover the box-model
  types and the empty-stylesheet layout pass. The
  layout semantics (margin collapse, flex, grid)
  land with M4.6.
- **Reachable from a real surface:** the crate
  compiles; the type-level surface is reachable.
  The *engine* is not yet executed. This is
  documented in the M4.4.1 Item 4 / Item 8 ledger
  entries as a known staged rollout.

---

## Notes

The M4 baseline audit (`docs/audits/2026-06-15-
baseline.md:85–96`) lists 6 Gyre defects in
priority order for M4.4–M4.6. The first slice —
box model + margins — is M4.6 Item 13. The
other 5 items (block layout, flex container,
flex item, grid container, grid item) follow.
