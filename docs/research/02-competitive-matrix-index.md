# Competitive Matrix — Index and Summary

**Date:** 2026-06-16
**Worktree:** `research/competitive-parity`
**Methodology:** `00-methodology.md`
**Sources:** `citations/sources.md`

---

## 1. Domain files

| Domain | File | Lines | Rows |
|--------|------|------:|-----:|
| HTML elements | `02-competitive-matrix-html.md` | 166 | 83 |
| DOM / JS / ECMAScript | `02-competitive-matrix-dom-css.md` | 560 | 376 |
| Networking & protocols | `02-competitive-matrix-network.md` | 211 | 94 |
| Security & privacy | `02-competitive-matrix-security.md` | 350 | 128 |
| Storage & state | `02-competitive-matrix-storage.md` | 212 | 89 |
| Media, codecs, EME | `02-competitive-matrix-media.md` | 219 | 90 |
| Web platform APIs & runtime | `02-competitive-matrix-apis.md` | 189 | 162 |
| User-facing UX | `02-competitive-matrix-ux.md` | 279 | 126 |
| Developer / power-user | `02-competitive-matrix-developer.md` | 258 | 113 |
| Accessibility & i18n | `02-competitive-matrix-a11y.md` | 180 | 90 |
| Extensions & customisation | `02-competitive-matrix-extensions.md` | 140 | 70 |
| Distribution & platform | `02-competitive-matrix-distribution.md` | 235 | 150 |
| **Total** | | **2,999** | **1,571** |

---

## 2. Distribution by prevalence

| Bucket | Count | % |
|--------|------:|--:|
| `ubiquitous` | 950 | 60.5% |
| `widespread` | 345 | 22.0% |
| `mixed` | 125 | 8.0% |
| `niche` | 111 | 7.1% |
| `experimental` | 22 | 1.4% |
| `legacy` / `deprecated` | 11 | 0.7% |
| (unclassified / qualifier) | 7 | 0.4% |
| **Total** | **1,571** | |

---

## 3. Distribution by Spiral status

| Status | Count | % |
|--------|------:|--:|
| `not-started` | 1,407 | 89.6% |
| `partial` | 83 | 5.3% |
| `shipped` | 66 | 4.2% |
| `designed` | 5 | 0.3% |
| `do-not-touch` | 3 | 0.2% |
| (qualifier variants) | 7 | 0.4% |
| **Total** | **1,571** | |

**Interpretation:** 89.6% of modern-browser capabilities are not started.
4.2% (66 rows) are "shipped" — these are concentrated in `spiral-fmt`
(Forge: HTML tokeniser + tree builder, CSS tokeniser + parser) and
`spiral-vortex` (Vortex: lexer, parser, interpreter, builtins). 5.3%
(83 rows) are "partial" — also concentrated in Forge and Vortex.

---

## 4. Distribution by phase

| Phase | Count | % |
|-------|------:|--:|
| P2 (months 4–9, current) | 140 | 8.9% |
| P3 (months 10–24) | 552 | 35.1% |
| P4 (months 25–42) | 286 | 18.2% |
| P5 (months 43–60) | 129 | 8.2% |
| P6 (months 61–84) | 291 | 18.5% |
| 0.1-blocker | 0 | 0% |
| 1.0-blocker | 3 | 0.2% |
| (compound / unclassified) | 170 | 10.8% |
| **Total** | **1,571** | |

**Interpretation:** P3 (months 10–24, "Flex layout, text rendering,
Vortex bytecode VM, basic DOM from JS") carries the most weight at 35%.
P6 (WPT compliance, performance, cross-platform packaging) carries 18.5%.
The current phase (P2) accounts for 8.9% — these are the features
Spiral should be actively working on right now.

---

## 5. Distribution by complexity

| Size | Count | % |
|------|------:|--:|
| S (< 1 day) | 497 | 31.6% |
| M (1–5 days) | 680 | 43.3% |
| L (1–4 weeks) | 298 | 19.0% |
| XL (1+ month) | 85 | 5.4% |
| (unclassified) | 11 | 0.7% |
| **Total** | **1,571** | |

---

## 6. Top 20 competitive gaps

Scoring: `prevalence_weight × spiral_gap_weight × phase_urgency`
(max score = 5 × 5 × 8 = 200)

Prevalence weights: ubiquitous=5, widespread=4, mixed=3, niche=2, experimental=1, legacy=0.
Spiral gap weights: not-started=5, designed=4, partial=3, shipped=1, do-not-touch=0.
Phase urgency: 0.1-blocker=8, 1.0-blocker=7, P2=6, P3=5, P4=4, P5=3, P6=2.

The top-20 list is **heavily tied at score 150** (ubiquitous × not-started × P2).
These are all foundational HTML/DOM/JS features that Spiral's parser accepts
but whose IDL surfaces, behavioural algorithms, or integration points have
not been implemented. The list below picks the 20 most impactful from the
tie, prioritising items that block the most downstream work.

| Rank | Capability | Domain | Prevalence | Spiral Status | Phase | Score | Priority recommendation |
|------|-----------|--------|-----------|---------------|-------|------:|------------------------|
| 1 | Global attributes (`id`, `class`, `style`, `title`, `lang`, `dir`, `hidden`, `tabindex`, `contenteditable`, `inert`, `popover`, etc.) | html | ubiquitous | not-started | P2 | 150 | P2: foundational — blocks all DOM IDL and accessibility |
| 2 | Adoption agency algorithm (misnested formatting) | html | ubiquitous | not-started | P2 | 150 | P2: blocks correct rendering of real-world HTML |
| 3 | Active formatting elements list | html | ubiquitous | not-started | P2 | 150 | P2: required by adoption agency |
| 4 | Foster parenting (out-of-table / in-table placement) | html | ubiquitous | not-started | P2 | 150 | P2: blocks correct table parsing |
| 5 | `<template>` content document-fragment construction | html | ubiquitous | not-started | P2 | 150 | P2: blocks Web Components, Shadow DOM, declarative templates |
| 6 | Fragment parsing algorithm (`DOMParser.parseFragment`) | html | ubiquitous | not-started | P2 | 150 | P2: blocks innerHTML, insertAdjacentHTML, template content |
| 7 | `<noscript>` element | html | ubiquitous | not-started | P2 | 150 | P2: blocks correct rendering with JS enabled |
| 8 | Quirk mode classifier | html | ubiquitous | not-started | P2 | 150 | P2: blocks correct CSS behaviour on legacy sites |
| 9 | `DOMTokenList` (`classList`, `relList`, `sandbox`, etc.) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks class manipulation from JS |
| 10 | `NodeList` (static or live) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks querySelectorAll result handling |
| 11 | `HTMLCollection` (live ordered collection) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks getElementsByTagName result handling |
| 12 | `Attr` interface (`name`, `value`, `namespaceURI`, etc.) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks getAttributeNode and attribute iteration |
| 13 | `NamedNodeMap` (attribute collection) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks `.attributes` on Element |
| 14 | `DocumentType` (`name`, `publicId`, `systemId`) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks `document.doctype` |
| 15 | `data-*` custom data attributes (`dataset` IDL) | html | ubiquitous | not-started | P2 | 150 | P2: blocks `element.dataset` from JS |
| 16 | `globalThis` | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks universal global reference |
| 17 | `structuredClone` | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks postMessage structured data, Workers |
| 18 | `Proxy` (handler traps) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks transparent object interception |
| 19 | `Reflect` (static reflection namespace) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks Proxy + Reflect idiom |
| 20 | `URL` and `URLSearchParams` (WHATWG URL parser) | dom-css | ubiquitous | not-started | P2 | 150 | P2: blocks URL manipulation everywhere |

**Note on the tie:** 150+ capabilities share the score 150. The 20 above
were selected because they are **foundational blockers** — each one
blocks a cluster of downstream capabilities. The full list of 150+ tied
items can be derived from the domain matrix files by filtering for
`prevalence=ubiquitous, status=not-started, phase=P2`.

### Key observation

The top-20 list is concentrated in two areas:

1. **HTML tree-builder depth** (rows 1–8): Spiral's HTML parser (`spiral-fmt`)
   tokenises correctly and has 8 insertion modes, but the adoption agency
   algorithm, active formatting elements, foster parenting, and fragment
   parsing are not yet implemented. These are the core algorithms that
   make the tree builder produce correct DOM from real-world HTML.

2. **DOM IDL surfaces** (rows 9–20): Spiral's DOM (`spiral-dom`) has
   `Node`, `Element`, `Document`, `Text`, `Comment` variants but lacks
   the collection types (`NodeList`, `HTMLCollection`, `NamedNodeMap`,
   `DOMTokenList`), the `Attr` interface, `DocumentType`, `dataset`,
   and the JS builtins that web pages use constantly (`globalThis`,
   `structuredClone`, `Proxy`, `Reflect`, `URL`).

Both areas are Phase 2 work. Pulling the tree-builder depth items into
the current M4.5/M5 sprint would unblock the most downstream capability.

---

## 7. Open questions for the user

1. **Phase 2 backlog overflow:** 140 capabilities tagged P2 (months 4–9).
   At ~40 working days per sprint, M4.5 through M9 is ~6 sprints (~240
   working days). 140 capabilities at an average of 2 days each is 280
   days. This exceeds the sprint window. Should some P2 items be
   re-tagged P3 to fit the timeline?

2. **Top-20 bias toward HTML/DOM:** The scoring formula weights prevalence
   heavily. HTML/DOM items dominate because they are "ubiquitous." Should
   the scoring include a "Spiral-specific urgency" weight (e.g. items
   that block the next milestone vs items that can wait)?

3. **Flow engine verification:** Per methodology §11.1, the Flow row was
   to be re-verified at chunk 12 time. The matrix files use "no" for Flow
   on most rows. If Flow's scope has shifted (retired, merged, or
   changed engine family), the Flow column should be updated.

4. **"Do-not-touch" items:** Only 3 rows are marked `do-not-touch`. This
   seems low. Is the architecture-bet exclusion working as intended, or
   are there capabilities that should be explicitly excluded?
