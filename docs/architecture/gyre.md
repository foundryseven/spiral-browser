# `spiral-gyre` (Gyre) — Layout Engine

> **Brand:** Gyre. **Crate:** `spiral-gyre`. **Scope:**
> block, flex, grid layout. **Status:** M4.4 type-level
> surface in place; first layout slice (box model +
> margins) is M4.6 Item 13.

Gyre is Spiral's in-house layout engine. It computes
the box model, block flow, flex layout, and grid
layout for every element in the DOM. Gyre is fully
in-house — no Taffy, no Servo layout code — and is
the only piece of the engine that is genuinely
*Spiral's tech*.

See `docs/decisions/0003-gyre-rename.md` for the
Taffy-drop decision and the brand-name rationale.

---

## Public surface

```rust
pub struct LayoutDimensions { … }   // x, y, width, height
pub struct BoxModel { … }            // margin, border, padding, content
pub struct EdgeSizes { … }           // top, right, bottom, left
pub struct LayoutNode { … }          // node_id, box_model, children
pub struct LayoutEngine { … }        // viewport_width, viewport_height

impl LayoutEngine {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self;
    pub fn layout(&self, dom: &Dom, _stylesheet: &Stylesheet) -> Result<LayoutNode>;
}
```

The current `layout()` signature takes a `&Stylesheet`
that is not yet read — the layout pipeline is not yet
connected to the new CSS parser (ADR 0001). The first
slice that reads a stylesheet is M4.6 (box model +
margins).

---

## Internal layout

```
spiral-gyre/src/
└── lib.rs           — types (LayoutDimensions, BoxModel,
                       EdgeSizes, LayoutNode) + LayoutEngine
                       + the recursive layout_node() walker
```

M4.4 has a single-file layout. The M4.6 sprint will
split this into:

```
spiral-gyre/src/
├── lib.rs           — public surface
├── box.rs           — BoxModel, EdgeSizes (M4.6)
├── block.rs         — block layout (M4.6)
├── flex.rs          — flex container + flex item (M6)
└── grid.rs          — grid container + grid item (M7)
```

The split is **not** done in M4.4. A single 209-line
file is fine until it isn't; M4.6 will hit the split
point.

---

## Constraints

- **No Taffy.** The M0 plan to use Taffy was reversed
  in 2026-06-14. See ADR 0003.
- **No Servo layout code.** The Servo layout code is
  a 15-year-old C++ project with its own assumptions;
  vendoring it would be a multi-month port with no
  Spiral ownership of the result. ADR 0003 §
  Alternatives.
- **Spec-faithful.** Gyre follows the CSS Display
  spec, the CSS Box Model spec, the CSS Flexbox
  Container spec, the CSS Grid Container spec.
  Deviations from spec require an ADR.
- **Idempotent.** `layout(&dom, &stylesheet)` is
  pure: same input → same output. No mutation of
  inputs, no hidden state.

---

## Test posture

- 3 lib tests in M4.4 cover the type-level surface
  and the empty-stylesheet layout pass.
- M4.6 will add the box-model test suite
  (~25 tests for margin collapse, padding,
  border, content box, percentage resolution).
- M6 will add the flex suite (~40 tests).
- M7 will add the grid suite (~50 tests).

Total projected: ~120 lib tests for Gyre, all
spec-anchored.

---

## Do-not-touch zones (M4.4)

- `LayoutNode.children` is the canonical tree
  representation. Adding fields is a breaking change.
- `BoxModel` is the canonical box-model struct.
  Adding fields is a breaking change.
- `LayoutEngine::layout` is the public entry point.
  Renaming is a breaking change.

---

## Related

- `docs/decisions/0003-gyre-rename.md` — the
  `spiral-layout` → `spiral-gyre` rename + Taffy drop.
- `docs/glossary.md` — the Gyre brand entry.
- `AGENTS.md` § `spiral-gyre (Gyre)` — working rules.
- `docs/audits/2026-06-15-baseline.md` §1.4 — the
  M4.4 / M4.5 / M4.6 priority list for Gyre.
