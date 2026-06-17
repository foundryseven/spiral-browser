# Architecture Decision Records

This directory contains the **ADRs** — Architecture Decision Records —
for Spiral. An ADR is a short, structured document that captures a
significant decision, the context that drove it, the alternatives that
were considered, and the consequences (positive and negative).

ADRs are **immutable once Accepted**. If a decision changes, write a
new ADR that supersedes the old one. Do not edit history.

The structure is fixed by [`0000-template.md`](0000-template.md).

---

## How to Read

1. Skim the index below.
2. For a decision that affects your work, read the full ADR.
3. If the decision is **Proposed**, it is not yet binding. Treat the
   current implementation as the source of truth until the ADR is
   Accepted.
4. If the decision affects an active Step, look at the **Wiring &
   Integration** section in the ADR; it names the call sites.

---

## The Index

| # | Title | Status | Date |
|---|-------|--------|------|
| [0001](0001-css-parser-spiral-fmt.md) | CSS Syntax 3 parser moves to `spiral-fmt::css`; `spiral-css` becomes a deprecated shim (Fork 1-B) | Accepted | 2026-06-16 |
| [0002](0002-vortex-from-scratch.md) | Vortex is a from-scratch Rust JavaScript engine; `rusty_v8` is CI oracle only | Accepted | 2026-06-14 |
| [0003](0003-gyre-rename.md) | Rename `spiral-layout` to `spiral-gyre` (post-Fork 1). The Vortex rename (`spiral-js` → `spiral-vortex`) is documented in the body of ADR 0002. | Accepted | 2026-06-14 |
| [0004](0004-resolver-trait-async-design.md) | `Resolver` trait is `async`-native, with `R: Resolver` generic bound on the client | Accepted | 2026-06-16 |
| [0005](0005-filter-hook-architecture.md) | `FilterHook`, `Decision`, and `Party` live in `spiral-core` (not `spiral-filter`); resolves the packet 1.6.4 dep-arrow violation | Accepted | 2026-06-16 |
| [0006](0006-browser-image-decoder-dep.md) | Add `spiral-imagedecoder` dependency to `spiral-browser` to render startup logo | Accepted | 2026-06-17 |

---

## How to Add an ADR

1. Find the next number (max + 1).
2. Copy [`0000-template.md`](0000-template.md) to
   `docs/decisions/NNNN-<slug>.md`.
3. Fill in the required sections. **No `TBD` placeholders.**
4. Add the index entry to the table above.
5. **Link the ADR from the relevant Step** in
   [`docs/implementation_tracker.md`](../implementation_tracker.md).
   The link is required before the ADR moves to **Status: Accepted**.

```
### Step X.Y — <title>
- [ ] (packets)
- ADR: [NNNN-slug](../decisions/NNNN-slug.md) (Status YYYY-MM-DD)
```

The tracker is the SSOT for "what is in flight". A floating ADR (not
linked from a Step) is harder to find and easier to forget.

---

## How to Supersede an ADR

When a decision changes:

1. Write a new ADR (the superseding one).
2. Update the new ADR's `Related:` field to point to the superseded one.
3. Update the superseded ADR's status: `Superseded by ADR-NNNN`.
4. Add both to the index (the new one Accepted, the old one Superseded).
5. Link the new ADR from the relevant Step in the tracker.

Never edit an Accepted ADR's decision text.
