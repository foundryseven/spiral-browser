# Public Failure Log

> **The running record of LLM-specific mistakes the Spiral project
> has caught, fixed, and documented.**
>
> The failure log is a working tool, not a confession. Each entry
> is a small, specific, reproducible record of an LLM failure mode
> the project has encountered and addressed. The methodology
> ([`docs/methodology.md`](../methodology.md) §7) is the
> commitment to keep the log current.

---

## Schema

Each entry is a Markdown file in this directory with the naming
convention:

```
YYYY-MM-DD-NNN-<short-slug>.md
```

Where:

- `YYYY-MM-DD` is the date the failure was recorded.
- `NNN` is a zero-padded sequence number (`001`, `002`, …).
- `<short-slug>` is a kebab-case short name of the failure.

The body of the file follows this schema:

```markdown
# NNN — <short-title>

- **Date:** YYYY-MM-DD
- **Commit introduced:** <short-sha>
- **Commit fixed:** <short-sha>
- **Category:** <one of: wiring-leaks | third-party-crate | vocabulary-drift | other>

## What went wrong

<one or two paragraphs>

## How it was caught

<one paragraph: which audit, which test, which reviewer>

## How it was fixed

<one paragraph: which commit, which test added>

## Lesson learned

<one sentence>
```

---

## Categories

The project has encountered three recurring categories of LLM
failure. New categories are added here as they emerge.

1. **Wiring leaks.** Over-published `pub` symbols. The AI tends
   to expose APIs "just in case"; the audit catches it. The
   standing defence is
   [`scripts/audit-orphan-exports.sh`](../scripts/audit-orphan-exports.sh).
2. **Dependence on stale third-party crates.** The AI defaults
   to wrapping or vendoring mature third-party crates
   (html5ever, rquickjs, taffy, etc.) and the project later has
   to retire the wrapper and write the equivalent from-spec. The
   relevant ADRs are
   [`docs/decisions/0001-css-parser-spiral-fmt.md`](../decisions/0001-css-parser-spiral-fmt.md),
   [`0002-vortex-from-scratch.md`](../decisions/0002-vortex-from-scratch.md),
   and [`0003-gyre-rename.md`](../decisions/0003-gyre-rename.md).
3. **Vocabulary drift.** The AI uses time-based words (`Sprint`,
   `Chunk`, `Month`) that don't match the actual `Group` /
   `Phase` / `Step` / `Packet` structure. The standing defence
   is the retired-vocabulary denylist in
   [`scripts/audit-doc-drift.sh`](../scripts/audit-doc-drift.sh).

---

## Entries

- [`2026-06-18-001-render-node-id.md`](2026-06-18-001-render-node-id.md) —
  `spiral-core::RenderNodeId` was declared `pub` with no
  external consumer. Caught by `audit-orphan-exports.sh` on
  its first run.
