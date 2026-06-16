# Post-Mortem NNNN: <Title>

**Status:** Draft | In Review | Closed
**Date:** YYYY-MM-DD
**Severity:** Critical | High | Medium | Low
**Author:** <name, role>

---

## Summary

<1-2 sentences. What happened, in plain language.>

## Timeline (UTC)

- **YYYY-MM-DD HH:MM** — <event>
- **YYYY-MM-DD HH:MM** — <event>
- **YYYY-MM-DD HH:MM** — <event>
- **YYYY-MM-DD HH:MM** — <resolution>

## Impact

- **Users affected:** <number or "all">
- **Data exposed:** <yes / no / partial — what?>
- **Duration:** <start → end>
- **Detection:** <how was it detected?>

## Root Cause

<1-2 paragraphs. Not "human error" — be specific. What went wrong, and
why did the system allow it?>

## What Went Well

- <one bullet — e.g. "Detection was automatic, no user reports">

## What Went Wrong

- <one bullet — e.g. "Audit did not catch the dep CVE">

## Action Items

| # | Action | Owner | Status |
|---|--------|-------|--------|
| 1 | <e.g. add cargo-audit to CI> | <role> | Open |
| 2 | <e.g. rotate the leaked key> | <role> | Done |

## Lessons Learned

<1-2 paragraphs. The generalisable lesson, not the specific bug.>

## References

- <link to the relevant ADR>
- <link to the relevant Phase in implementation_tracker>
- <link to the hotfix commit / PR>
- <link to the disclosure, if any>
