# Competitive Parity Synthesis

**Date:** 2026-06-16
**Worktree:** `research/competitive-parity` (base: `audit/m4-window` @ `5f7b6a4`)
**Methodology:** `00-methodology.md`
**Matrix index:** `02-competitive-matrix-index.md`

---

## 1. Executive summary

The competitive-parity research identified **1,571 capabilities** that
modern browsers ship across 11 domains. Of these, **89.6% are
not-started** in Spiral, 5.3% are partial, and 4.2% are shipped. The
shipped capabilities are concentrated in Forge (HTML/CSS parsing) and
Vortex (JS lexer/parser/interpreter).

**The single most important finding:** the top-20 competitive gaps are
concentrated in two areas — the HTML tree-builder depth (adoption agency,
active formatting elements, foster parenting, fragment parsing) and the
DOM IDL surface (NodeList, HTMLCollection, DOMTokenList, Attr, dataset,
structuredClone, URL). Both areas are tagged Phase 2 and block the
largest number of downstream capabilities.

**The single most important action:** pull the HTML tree-builder depth
items (adoption agency, active formatting elements, foster parenting)
into the current M4.5/M5 sprint. These are the foundational algorithms
that make the tree builder produce correct DOM from real-world HTML.
Without them, Spiral cannot render any non-trivial page.

---

## 2. Methodology applied

- **Source ladder:** 5 tiers (WHATWG/W3C/IETF/ECMA/Unicode → MDN/Can
  I Use → Engine release notes → Third-party audits → Community)
- **Scoring:** prevalence (ubiquitous=5 … legacy=0) × Spiral gap
  (not-started=5 … do-not-touch=0) × phase urgency (0.1-blocker=8 …
  P6=2)
- **Novelty gate:** No novelty claims made in this synthesis. The
  research is descriptive (what engines ship), not prescriptive (what
  Spiral should invent). Per methodology §8, the novelty gate is
  satisfied by absence of claims.

---

## 3. Distribution summary

| Dimension | Key finding |
|-----------|-------------|
| Total rows | 1,571 |
| Prevalence | 60.5% ubiquitous, 22.0% widespread, 8.0% mixed |
| Spiral status | 89.6% not-started, 5.3% partial, 4.2% shipped |
| Phase | 35.1% P3 (months 10–24), 18.2% P4, 18.5% P6, 8.9% P2 |
| Complexity | 43.3% M, 31.6% S, 19.0% L, 5.4% XL |

---

## 4. Top-20 gaps (with synthesis actions)

The top-20 list from `02-competitive-matrix-index.md` §6 is reproduced
below with a "Synthesis action" column. All 20 share score 150
(ubiquitous × not-started × P2).

| Rank | Capability | Domain | Synthesis action |
|------|-----------|--------|-----------------|
| 1 | Global attributes (`id`, `class`, `style`, `title`, `lang`, `dir`, `hidden`, `tabindex`, `contenteditable`, `inert`, `popover`, etc.) | html | **Add to GAP_ANALYSIS** — no row exists for global attribute IDL |
| 2 | Adoption agency algorithm (misnested formatting) | html | **Add to GAP_ANALYSIS** — no row exists; blocks correct rendering |
| 3 | Active formatting elements list | html | **Add to GAP_ANALYSIS** — required by adoption agency |
| 4 | Foster parenting (out-of-table / in-table placement) | html | **Add to GAP_ANALYSIS** — blocks correct table parsing |
| 5 | `<template>` content document-fragment construction | html | **Bump priority in GAP_ANALYSIS** — exists as priority #10 but tagged P2; should be P2 sprint item |
| 6 | Fragment parsing algorithm (`DOMParser.parseFragment`) | html | **Add to GAP_ANALYSIS** — blocks innerHTML, insertAdjacentHTML |
| 7 | `<noscript>` element | html | **Add to GAP_ANALYSIS** — blocks correct rendering with JS enabled |
| 8 | Quirk mode classifier | html | **Add to GAP_ANALYSIS** — blocks correct CSS behaviour on legacy sites |
| 9 | `DOMTokenList` (`classList`, `relList`, `sandbox`, etc.) | dom-css | **Add to GAP_ANALYSIS** — blocks class manipulation from JS |
| 10 | `NodeList` (static or live) | dom-css | **Add to GAP_ANALYSIS** — blocks querySelectorAll result handling |
| 11 | `HTMLCollection` (live ordered collection) | dom-css | **Add to GAP_ANALYSIS** — blocks getElementsByTagName result handling |
| 12 | `Attr` interface (`name`, `value`, `namespaceURI`, etc.) | dom-css | **Add to GAP_ANALYSIS** — blocks getAttributeNode and attribute iteration |
| 13 | `NamedNodeMap` (attribute collection) | dom-css | **Add to GAP_ANALYSIS** — blocks `.attributes` on Element |
| 14 | `DocumentType` (`name`, `publicId`, `systemId`) | dom-css | **Add to GAP_ANALYSIS** — blocks `document.doctype` |
| 15 | `data-*` custom data attributes (`dataset` IDL) | html | **Add to GAP_ANALYSIS** — blocks `element.dataset` from JS |
| 16 | `globalThis` | dom-css | **Add to GAP_ANALYSIS** — blocks universal global reference |
| 17 | `structuredClone` | dom-css | **Add to GAP_ANALYSIS** — blocks postMessage structured data, Workers |
| 18 | `Proxy` (handler traps) | dom-css | **Add to GAP_ANALYSIS** — blocks transparent object interception |
| 19 | `Reflect` (static reflection namespace) | dom-css | **Add to GAP_ANALYSIS** — blocks Proxy + Reflect idiom |
| 20 | `URL` and `URLSearchParams` (WHATWG URL parser) | dom-css | **Add to GAP_ANALYSIS** — blocks URL manipulation everywhere |

---

## 5. Domain-by-domain synthesis

### 5.1 HTML / DOM / CSS / JS (core engines)

**What Spiral ships today:**
- HTML tokeniser + tree builder (8 insertion modes) — `spiral-fmt` [shipped]
- CSS tokeniser + parser (8 modules) — `spiral-fmt` [shipped]
- DOM: Node, Element, Document, Text, Comment — `spiral-dom` [partial]
- Vortex: lexer, parser, interpreter, builtins — `spiral-vortex` [partial]

**What's missing and why it matters:**
- **Tree-builder depth:** adoption agency, active formatting elements,
  foster parenting, fragment parsing. Without these, the tree builder
  produces incorrect DOM for any non-trivial HTML. These are the #2–#4
  and #6–#8 gaps.
- **DOM IDL surfaces:** NodeList, HTMLCollection, DOMTokenList, Attr,
  NamedNodeMap, DocumentType, dataset. Without these, JS code that
  manipulates DOM collections cannot run. These are the #9–#15 gaps.
- **JS builtins:** globalThis, structuredClone, Proxy, Reflect, URL.
  Without these, modern JS patterns (Proxy-based reactivity, structured
  cloning, URL manipulation) cannot work. These are the #16–#20 gaps.
- **CSS depth:** Container Queries, View Transitions, Anchor Positioning,
  subgrid, masonry, :has(), :focus-visible, color-mix, oklch, relative
  colour, light-dark(), text-wrap: balance/pretty. These are all
  `ubiquitous` or `widespread` and `not-started`.

**Recommended actions:**
1. **M4.5/M5 sprint:** adoption agency + active formatting elements +
   foster parenting. Estimated: 2–3 weeks, L complexity.
2. **M5 sprint:** DOM collection types (NodeList, HTMLCollection,
   DOMTokenList, Attr, NamedNodeMap, DocumentType). Estimated: 1–2
   weeks, M complexity each.
3. **M5 sprint:** fragment parsing + `<template>` content. Estimated:
   1 week, M complexity.
4. **M5 sprint:** globalThis, structuredClone, URL/URLSearchParams.
   Estimated: 1 week, S–M complexity each.
5. **M6 sprint:** Proxy + Reflect. Estimated: 1–2 weeks, L complexity.
6. **M6+ sprint:** Container Queries, :has(), :focus-visible. Estimated:
   2–4 weeks, L complexity each.

**Cross-refs to GAP_ANALYSIS:**
- §1.1 (HTML parser) — shipped, but tree-builder depth missing
- §1.3 (DOM) — partial, collection types missing
- §1.6 (Vortex) — partial, builtins missing
- Priority #10 (`<template>`, DOCTYPE, insertBefore) — exists, should
  be bumped from P2 to P2 sprint item

### 5.2 Networking & protocols

**What Spiral ships today:**
- `spiral-net`: Resolver trait with async native resolver [partial]
- `spiral-network`: Client trait [designed]

**What's missing and why it matters:**
- HTTP/1.1, HTTP/2, HTTP/3, QUIC, WebSocket, WebTransport — all
  `not-started`. These are Phase 4 (months 25–42) items.
- TLS, DNS (DoH/DoT/DoQ), DNSSEC — `not-started`. Phase 4.
- HTTP caching, service workers, proxy support — `not-started`. Phase 4.

**Recommended actions:**
1. No change to current plan. Networking is correctly tagged Phase 4.
2. **Consider pulling forward:** HTTP/1.1 client (for basic page
   fetching) into Phase 3 (months 10–24). Without it, Spiral cannot
   load any remote page.

**Cross-refs to GAP_ANALYSIS:**
- §2.1 (HTTP/TLS/DNS) — not-started, correct priority
- §2.2 (SRI) — not-started, correct priority

### 5.3 Security & privacy

**What Spiral ships today:**
- `spiral-sandbox`: OS-level sandboxing (Landlock, seccomp-bpf, Seatbelt,
  Restricted Token) [partial]
- `spiral-filter`: CSP injection in rule AST [partial]
- `spiral-crypto`: CSPRNG + SHA-256 [partial]

**What's missing and why it matters:**
- CSP, CORS, CORP, COOP, COEP — all `not-started`. Without these,
  Spiral cannot enforce any web security policy.
- HSTS, CT, OCSP stapling — `not-started`. Phase 4.
- Permissions Policy, Storage Access API, FedCM — `not-started`. Phase 4+.
- Cookie security (SameSite, Secure, HttpOnly, Partitioned) — `not-started`.
- Fingerprinting resistance — `not-started`.

**Recommended actions:**
1. **Phase 3:** SOP enforcement (origin checks, same-origin policy).
   Without this, Spiral cannot safely run any JS.
2. **Phase 3:** CORS (at least simple requests + preflight). Without
   this, Spiral cannot fetch cross-origin resources.
3. **Phase 4:** CSP, HSTS, CT, Permissions Policy.
4. **Phase 4:** Cookie security flags (SameSite, Secure, HttpOnly).

**Cross-refs to GAP_ANALYSIS:**
- §4.1 (Security/Privacy) — not-started, correct priority
- §4.2 (Filter/Bet 3) — partial, CSP injection exists but no enforcement
- Priority #13 (SOP/CSP/HSTS) — exists, correct priority

### 5.4 Storage & state

**What Spiral ships today:**
- Nothing. All 89 storage rows are `not-started`.

**What's missing and why it matters:**
- Cookies, localStorage, sessionStorage, IndexedDB, OPFS, CacheStorage —
  all `not-started`. These are Phase 4 (months 25–42) items.
- Storage partitioning — `not-started`. Phase 4.

**Recommended actions:**
1. No change to current plan. Storage is correctly tagged Phase 4.
2. **Consider pulling forward:** cookie jar (for basic session
   management) into Phase 3.

**Cross-refs to GAP_ANALYSIS:**
- §2.3 (Storage) — not-started, correct priority
- Priority #11 (Cookies/LocalStorage/IndexedDB/OPFS) — exists, correct priority

### 5.5 Media, codecs, EME

**What Spiral ships today:**
- Nothing. All 90 media rows are `not-started`.

**What's missing and why it matters:**
- Video codecs (H.264, VP9, AV1), audio codecs (Opus, AAC, MP3),
  container formats (MP4, WebM), MSE, EME — all `not-started`.
- WebRTC, WebCodecs — `not-started`.
- Media playback APIs (HTMLMediaElement, MediaSession, PiP) — `not-started`.

**Recommended actions:**
1. No change to current plan. Media is correctly tagged Phase 4+.
2. **Phase 5:** GPU rendering (Vello) must precede media playback UI.

**Cross-refs to GAP_ANALYSIS:**
- §4.3 (Graphics/Compositing) — not-started, correct priority
- §4.4 (Media) — not-started, correct priority
- Priority #15 (`spiral-media`) — exists, correct priority

### 5.6 Web platform APIs & runtime

**What Spiral ships today:**
- Vortex: lexer, parser, interpreter, builtins [partial]
- `spiral-crypto`: CSPRNG + SHA-256 [partial]

**What's missing and why it matters:**
- Fetch API, Streams API, Workers — all `not-started`. These are
  Phase 3/4 items.
- WebAssembly — `not-started`. Phase 3.
- WebGPU, WebGL — `not-started`. Phase 5.
- WebAuthn, Payment Request, Web Share — `not-started`. Phase 4+.

**Recommended actions:**
1. **Phase 3:** WebAssembly (at least basic Module/Instance/Memory).
   Without this, Spiral cannot run any WASM content.
2. **Phase 3:** Fetch API (at least basic `fetch()`). Without this,
   Spiral cannot make any network requests from JS.
3. **Phase 4:** Workers (DedicatedWorker, SharedWorker, ServiceWorker).
4. **Phase 5:** WebGPU (Vello backend).

**Cross-refs to GAP_ANALYSIS:**
- §1.6 (Vortex) — partial, builtins missing
- §5.3 (Plumbing) — WebGL/WebGPU bindings exist as stubs
- Priority #17 (WebGL/WebGPU) — exists, correct priority

### 5.7 User-facing UX

**What Spiral ships today:**
- `spiral-ui`: GPU-rendered browser chrome [designed]
- `spiral-theme`: CSS custom properties, light/dark [designed]
- Sidebar tabs, floating URL bar, navigation buttons — designed

**What's missing and why it matters:**
- All 126 UX rows are `not-started` or `designed`. No UX is shipped.
- Tabs, bookmarks, history, downloads, find-in-page, reader mode,
  translate, passwords, autofill, sync — all `not-started`.

**Recommended actions:**
1. **Phase 5:** Basic UX (tabs, URL bar, navigation, bookmarks). The
   current plan has "Zen UI" in Phase 5 (months 43–60). This is correct.
2. **Phase 5:** Find-in-page. Simple but essential.
3. **Phase 6:** Reader mode, translate, passwords, autofill, sync.

**Cross-refs to GAP_ANALYSIS:**
- §3 (Presentation Layer) — designed, not shipped
- Priority #12 (WebExtensions) — exists, correct priority

### 5.8 Developer / power-user surface

**What Spiral ships today:**
- Nothing. All 113 developer rows are `not-started`.

**What's missing and why it matters:**
- DevTools, view-source, error pages, headless mode, automation
  protocol, PDF viewer, RSS — all `not-started`.

**Recommended actions:**
1. **Phase 6:** DevTools (at least Elements + Console + Network panels).
   This is a 1.0-blocker.
2. **Phase 6:** Error pages (HTTP errors, cert errors, network errors).
3. **Phase 6:** View-source.
4. **Phase 6:** Headless mode (for automation/testing).

**Cross-refs to GAP_ANALYSIS:**
- §5.2 (WPT/test infrastructure) — not-started, correct priority
- Priority #16 (WPT fixtures) — exists, correct priority

### 5.9 Accessibility & i18n

**What Spiral ships today:**
- Nothing. All 90 a11y/i18n rows are `not-started`.

**What's missing and why it matters:**
- ARIA support, accessibility tree, keyboard navigation, high-contrast
  mode, screen reader integration — all `not-started`.
- Intl.* APIs, bidi, CJK shaping, locale handling — all `not-started`.

**Recommended actions:**
1. **Phase 3:** ARIA reflection (at least role, aria-*, label,
   labelledby). Without this, assistive technology cannot read Spiral's
   DOM.
2. **Phase 3:** Keyboard navigation (tab order, focus management,
   focus-visible). Without this, Spiral is not keyboard-accessible.
3. **Phase 4:** Intl.* APIs (at least Intl.NumberFormat,
   Intl.DateTimeFormat). Without this, Spiral cannot render
   locale-specific content.
4. **Phase 5:** Full a11y tree, screen reader integration, high-contrast
   mode.

**Cross-refs to GAP_ANALYSIS:**
- §4.5 (i18n/a11y/extensions) — not-started, correct priority

### 5.10 Extensions & customisation

**What Spiral ships today:**
- `spiral-filter`: content filter / ad-blocker [partial]

**What's missing and why it matters:**
- WebExtensions API — `not-started`. Phase 6 / 1.0-blocker.
- Content scripts, background service worker, popup, options page —
  all `not-started`.
- Custom themes, custom CSS, custom search engines — `not-started`.

**Recommended actions:**
1. **Phase 6:** WebExtensions MV3 (at least tabs, storage, content_scripts,
   action, popup). This is a 1.0-blocker.
2. **Phase 6:** Extension store / gallery.
3. **Phase 6:** Custom themes (background image, accent colour).

**Cross-refs to GAP_ANALYSIS:**
- §4.2 (Filter/Bet 3) — partial, content filter exists
- Priority #12 (WebExtensions) — exists, correct priority

### 5.11 Distribution & platform integration

**What Spiral ships today:**
- `spiral-sandbox`: OS-level sandboxing [partial]
- No installer, no auto-update, no code signing, no notarisation.

**What's missing and why it matters:**
- All 150 distribution rows are `not-started`.
- Installers (.deb, .rpm, .dmg, .exe, .msix), auto-update, code
  signing, notarisation — all `not-started`.
- Default browser registration, protocol handlers, file associations —
  all `not-started`.
- Enterprise policy (Group Policy, plist, JSON) — `not-started`.
- Crash reporting, telemetry — `not-started`.

**Recommended actions:**
1. **Phase 6:** Basic installers (Linux .deb + .rpm, macOS .dmg, Windows
   .exe). This is a 0.1-blocker.
2. **Phase 6:** Auto-update (differential, background, signed).
3. **Phase 6:** Code signing (EV cert for Windows, Apple Developer ID
   for macOS).
4. **Phase 6:** Default browser registration.
5. **Phase 6:** Crash reporting (opt-in).
6. **Phase 6:** Enterprise policy (at least ExtensionSettings,
   SafeBrowsing, proxy).

**Cross-refs to GAP_ANALYSIS:**
- §5.3 (Build/CI/docs) — not-started, correct priority
- §5.4 (Stub crates) — spiral-sandbox exists, correct priority

---

## 6. Novelty audit log

**No novelty claims made in this synthesis.**

The research is descriptive (what engines ship), not prescriptive (what
Spiral should invent). Per methodology §8, the novelty gate is satisfied
by absence of claims. If any future synthesis or ADR makes a "no shipped
browser does this" claim, it must be checked against V8, SpiderMonkey,
JavaScriptCore, Servo, Ladybird, Flow, and Brave before publication.

---

## 7. Priority re-ranking recommendations

| Capability | Current priority | Recommended | Rationale |
|-----------|-----------------|-------------|-----------|
| Adoption agency algorithm | Not in GAP_ANALYSIS | **P2 sprint item** | #2 gap, blocks correct rendering of real-world HTML |
| Active formatting elements | Not in GAP_ANALYSIS | **P2 sprint item** | #3 gap, required by adoption agency |
| Foster parenting | Not in GAP_ANALYSIS | **P2 sprint item** | #4 gap, blocks correct table parsing |
| Fragment parsing | Not in GAP_ANALYSIS | **P2 sprint item** | #6 gap, blocks innerHTML, insertAdjacentHTML |
| DOM collection types (NodeList, HTMLCollection, DOMTokenList, Attr, NamedNodeMap) | Not in GAP_ANALYSIS | **P2/M5 sprint item** | #9–#13 gaps, blocks DOM manipulation from JS |
| globalThis | Not in GAP_ANALYSIS | **P2 sprint item** | #16 gap, blocks universal global reference |
| structuredClone | Not in GAP_ANALYSIS | **P2 sprint item** | #17 gap, blocks postMessage structured data |
| URL/URLSearchParams | Not in GAP_ANALYSIS | **P2 sprint item** | #20 gap, blocks URL manipulation everywhere |
| `<template>` content | Priority #10, P2 | **P2 sprint item** (bump from general P2 to active sprint) | #5 gap, blocks Web Components, Shadow DOM |
| Quirk mode classifier | Not in GAP_ANALYSIS | **P2 sprint item** | #8 gap, blocks correct CSS behaviour on legacy sites |
| `<noscript>` element | Not in GAP_ANALYSIS | **P2 sprint item** | #7 gap, blocks correct rendering with JS enabled |

---

## 8. ROADMAP.md adjustment recommendations

| Phase | Adjustment | Rationale |
|-------|-----------|-----------|
| P2 (months 4–9) | **Add:** adoption agency, active formatting elements, foster parenting, fragment parsing, DOM collection types, globalThis, structuredClone, URL/URLSearchParams, quirk mode, `<noscript>` | These are the top-20 gaps and are all Phase 2 work. Without them, Spiral cannot render real-world pages. |
| P3 (months 10–24) | **Add:** HTTP/1.1 client (basic page fetching), SOP enforcement, CORS (simple + preflight), cookie jar, WebAssembly (basic), Fetch API, ARIA reflection, keyboard navigation | These are the minimum viable platform for running web content. Without them, Spiral cannot load or interact with any remote page. |
| P4 (months 25–42) | **No change.** Storage, media, networking depth, security depth are correctly placed. | |
| P5 (months 43–60) | **No change.** GPU rendering, Vello optimisation, sandbox re-evaluation are correctly placed. | |
| P6 (months 61–84) | **Add:** DevTools (Elements + Console + Network), error pages, view-source, headless mode, installers (.deb/.rpm/.dmg/.exe), auto-update, code signing, crash reporting, enterprise policy, WebExtensions MV3, extension store | These are 0.1-blockers and 1.0-blockers. They must be in Phase 6. |

---

## 9. Open questions for the user

1. **Phase 2 backlog overflow:** 140 capabilities tagged P2 (months 4–9).
   At ~40 working days per sprint, M4.5 through M9 is ~6 sprints (~240
   working days). 140 capabilities at an average of 2 days each is 280
   days. This exceeds the sprint window. Should some P2 items be
   re-tagged P3 to fit the timeline?

2. **Top-20 bias toward HTML/DOM:** The scoring formula weights prevalence
   heavily. HTML/DOM items dominate because they are "ubiquitous." Should
   the scoring include a "Spiral-specific urgency" weight (e.g. items
   that block the next milestone vs items that can wait)?

3. **HTTP/1.1 pull-forward:** The research shows HTTP/1.1 is a
   prerequisite for loading any remote page. Should it be pulled forward
   from Phase 4 to Phase 3?

4. **Cookie jar pull-forward:** The research shows cookies are a
   prerequisite for session management. Should the cookie jar be pulled
   forward from Phase 4 to Phase 3?

5. **DevTools scope:** The research identifies 113 developer-surface
   capabilities. Should Phase 6 DevTools be scoped to just Elements +
   Console + Network (the minimum viable set), or should it include
   Performance, Memory, Security, and Application panels?

6. **Flow engine verification:** Per methodology §11.1, the Flow row was
   to be re-verified at chunk 12 time. The matrix files use "no" for Flow
   on most rows. If Flow's scope has shifted, the Flow column should be
   updated.

---

## 10. Delta log (for SSOT agent)

Deltas to append to `specs/GAP_ANALYSIS.md`:

### Delta 5 — 2026-06-16 (Competitive parity research: top-20 gaps)

| Gap | Was | Now | Evidence |
|-----|-----|-----|----------|
| **G1.3a** Adoption agency algorithm (misnested formatting) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 2. WHATWG HTML §12.2.6.1. Ubiquitous across all engines. |
| **G1.3b** Active formatting elements list | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 3. WHATWG HTML §12.2.6.1. Required by adoption agency. |
| **G1.3c** Foster parenting (out-of-table / in-table) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 4. WHATWG HTML §12.2.6.1. Blocks correct table parsing. |
| **G1.3d** Fragment parsing algorithm | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 6. WHATWG HTML §12.4. Blocks innerHTML, insertAdjacentHTML. |
| **G1.3e** Quirk mode classifier | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 8. WHATWG HTML §12.1. Blocks correct CSS on legacy sites. |
| **G1.3f** `<noscript>` element | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 7. WHATWG HTML §4.6.7. Blocks correct rendering with JS enabled. |
| **G1.3g** Global attributes IDL (`id`, `class`, `style`, `title`, `lang`, `dir`, `hidden`, `tabindex`, `contenteditable`, `inert`, `popover`, etc.) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 1. WHATWG HTML §3.2.6. Blocks all DOM IDL and accessibility. |
| **G1.3h** `data-*` custom data attributes (`dataset` IDL) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-html-dom-js.md` §A.1, row 15. WHATWG HTML §3.2.6.3. Blocks `element.dataset` from JS. |
| **G1.4a** `DOMTokenList` (`classList`, `relList`, etc.) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 9. WHATWG DOM §7.1. Blocks class manipulation from JS. |
| **G1.4b** `NodeList` (static or live) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 10. WHATWG DOM §4.4. Blocks querySelectorAll result handling. |
| **G1.4c** `HTMLCollection` (live ordered collection) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 11. WHATWG DOM §4.5. Blocks getElementsByTagName result handling. |
| **G1.4d** `Attr` interface | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 12. WHATWG DOM §4.9. Blocks getAttributeNode and attribute iteration. |
| **G1.4e** `NamedNodeMap` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 13. WHATWG DOM §4.8. Blocks `.attributes` on Element. |
| **G1.4f** `DocumentType` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 14. WHATWG DOM §4.6. Blocks `document.doctype`. |
| **G1.6a** `globalThis` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 16. ECMA-262 §19.4.1. Blocks universal global reference. |
| **G1.6b** `structuredClone` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 17. WHATWG HTML §8.2.7. Blocks postMessage structured data. |
| **G1.6c** `Proxy` (handler traps) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 18. ECMA-262 §10.5. Blocks transparent object interception. |
| **G1.6d** `Reflect` (static reflection namespace) | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 19. ECMA-262 §28.1. Blocks Proxy + Reflect idiom. |
| **G1.6e** `URL` and `URLSearchParams` | (not tracked) | `[ ]` P2 sprint item | `01-feature-inventory-dom-css.md` §A.2, row 20. WHATWG URL §4. Blocks URL manipulation everywhere. |

### Delta 6 — 2026-06-16 (Competitive parity research: priority re-ranking)

| Gap | Was | Now | Evidence |
|-----|-----|-----|----------|
| **#10** `<template>` content fragment; DOCTYPE; insertBefore | P2 | **P2 sprint item** (bump from general P2 to active sprint) | `02-competitive-matrix-index.md` §6, rank 5. Ubiquitous, not-started, P2. Blocks Web Components, Shadow DOM, declarative templates. |

---

## 11. SSOT update instructions (for the SSOT agent)

The following files must be updated:

1. **`specs/GAP_ANALYSIS.md`:**
   - Append Delta 5 and Delta 6 to the end of the file (after Delta 4).
   - Add new rows to the Priority Stack (§6) for the top-20 gaps that
     are not already tracked. Use the gap IDs from Delta 5.
   - Update the "Proposed First Fill" (§7) if the top-20 gaps change
     the recommended sprint order.
   - Update "Open Questions for the User" (§8) with the questions from
     §9 of this synthesis.

2. **`docs/active_context.md`:**
   - Add a "External parity research landed" section listing:
     - Date: 2026-06-16
     - Worktree: `research/competitive-parity`
     - Docs: `docs/research/` (18 files, 1,571 rows)
     - Key finding: top-20 gaps are HTML tree-builder depth + DOM IDL
     - Priority changes: 19 new P2 sprint items, 1 re-ranked item
   - Update the "Sprint state" if the top-20 gaps change the current
     sprint focus.

3. **`ROADMAP.md`:**
   - Add the items from §8 (ROADMAP adjustment recommendations) to the
     appropriate phases. Do not remove existing items — only add.
   - The additions are all "pull-forward" or "new" items. None replace
     existing items.

---

## 12. Verification (post-SSOT update)

After the SSOT agent makes the edits, run:
- `cargo fmt --all -- --check` — clean
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo test --workspace` — all passing
- `cargo build --workspace` — clean
- Verify `specs/GAP_ANALYSIS.md` has Delta 5 and Delta 6
- Verify `docs/active_context.md` has the "External parity research" section
- Verify `ROADMAP.md` has the new items in the correct phases
