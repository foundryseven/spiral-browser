# Spiral Browser — Competitive Parity Research

> **Worktree:** `research/competitive-parity` (base: `audit/m4-window` @ `5f7b6a4`)
> **Sprint:** M4.5+ research, feeds back into `specs/GAP_ANALYSIS.md` and `ROADMAP.md` on synthesis.
> **Status:** Chunk 0 (methodology) — research chunks 1–11 in flight.

## Why this subset exists

`specs/GAP_ANALYSIS.md` is the **internal** gap tracker — it measures what
Spiral has shipped against the build plan. This subset measures Spiral
against the **external** market: what does a modern browser ship in 2026
that Spiral does not? The two views are complementary, not redundant.

The goal is to produce, in order:

1. A **capability inventory** that names every modern-browser feature in
   plain English (no proprietary branding).
2. A **competitive matrix** showing, per capability, the position of six
   engines: Chromium, Firefox, WebKit, Servo, Ladybird, Flow.
3. A **synthesis** that maps external findings to Spiral's priority stack
   and writes Delta entries back into `specs/GAP_ANALYSIS.md`, with
   follow-on edits to `docs/active_context.md` and `ROADMAP.md` only at
   the synthesis step.

## Chunk index

| # | File | Topic | Engine cols? |
|---|------|-------|--------------|
| 0 | `00-methodology.md` (this dir) | Source ladder, novelty gate, scoring | n/a |
| 0 | `citations/sources.md` | Master source index | n/a |
| 1 | `01-feature-inventory.md` §A | Core web platform (HTML/DOM/CSS/JS surface) | no |
| 2 | `05-protocols-network.md` | Networking & protocols | no |
| 3 | `04-privacy-security-standards.md` | Security & privacy standards | no |
| 4 | `06-web-platform-apis.md` §B | Storage & state | no |
| 5 | `07-media-codecs-eme.md` | Media, codecs, EME | no |
| 6 | `06-web-platform-apis.md` §A | Web platform APIs & runtime | no |
| 7 | `03-user-facing-ux.md` | User-facing UX (deep) | **yes** |
| 8 | `08-developer-experience.md` | Developer / power-user surface | no |
| 9 | `09-accessibility-i18n.md` | Accessibility & i18n | no |
| 10 | `11-extension-surface.md` | Extensions & customisation | no |
| 11 | `10-distribution-platforms.md` | Distribution & platform integration | no |
| 12 | `02-competitive-matrix.md` | Per-capability × per-engine matrix | derived |
| 13 | `12-gap-synthesis.md` | Synthesis + SSOT edits (the landing) | n/a |

Chunks 1–11 are descriptive. Chunk 12 is derived. Chunk 13 is the only
chunk that writes back into the live SSOT, and it does so via append-only
Delta entries.

## Per-chunk output contract

Every research chunk produces a file with the shape:

```
# Chunk N — <title>

## Scope
<what's in, what's out, why>

## Methodology for this chunk
<which sources informed the rows>

## Inventory
| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |

## Cross-refs
<links to specs/GAP_ANALYSIS.md rows that already exist>

## Open questions for the user
<bullets>
```

The `Sources` column is mandatory. The `Engine notes` column is full
(per-engine: Chromium / Firefox / WebKit / Servo / Ladybird / Flow) on
chunk 7 and a brief one-line note on every other chunk. The `Surface`
column is mandatory on every row — see `00-methodology.md` §11.1 for
the locked scope decision (desktop + mobile + embedded).

## Novelty gate

Per `AGENTS.md` §"Novelty Claims" and the M4 audit (`docs/audit-sprint-m4.md`),
**any** claim of the form "no shipped browser does X" must, before
publication, be checked against V8, SpiderMonkey, JavaScriptCore, Servo,
Ladybird, Flow, and Brave. Wikipedia is a starting point, not a
conclusion. The novelty gate runs once, in chunk 13, against the
synthesis's recommendations — not in every chunk.

## What this subset will NOT do

- No edits under `crates/**` (research product is docs-only).
- No edits to `docs/agents/**` (role contracts are out of scope).
- No edits to `scripts/audit-orphan-exports.sh`.
- No deletions of any existing doc. Only additions under `docs/research/`,
  plus append-only Deltas in chunk 13.
- No force-push to `master`. The branch is local until the user merges.

## How to audit progress at any point

```
git -C /Users/james/spiral-research log research/competitive-parity --oneline
ls -1 /Users/james/spiral-research/docs/research/
```

Each chunk is one commit. There is no work-in-progress editor.
