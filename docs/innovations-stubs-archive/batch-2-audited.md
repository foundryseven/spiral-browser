# Spiral Innovations — Eleven Stubs (Batch 2, audited)

**Status:** design stubs, audited (2026-06-15)
**Author:** implementer agent (originally self-written; now
audited by parallel research agent)
**Phase context:** M4 first sprint complete
**Purpose:** the original 11 idea stubs from
`innovations-stubs-2.md`, now with M4-audit corrections
integrated.

---

## Audit summary

The Batch 2 audit found:

- **3 of the 11 ideas that were claimed "truly novel" are
  partially novel** (#1 Compute Credits, #6 Permission
  Budget, #9 Visual Search). The classifications were
  honest.
- **1 idea is a duplicate of a Batch 4 idea** (#3
  Anti-Doom Scroll ↔ #22 The Browser That Asks Why). The
  Batch 4 audit's verdict ("partially novel") stands.
- **2 citations were corrected:** #2 URL Time-Travel
  references Chrome's "view cached page" which was
  removed in Chrome 124 (June 2024); #5 Form Memory
  undersells 1Password's learning fill engine (since
  2019) and Bitwarden's (since 2022).
- **1 corrected:** #3 Anti-Doom Scroll is "partially novel,
  but the gap is narrower than the pitch presents" —
  iOS 16+ Safari Website Limit (Sept 2022) is the
  closest shipped prior art.
- **1 specific build-cost adjustment:** #4 Real-Time
  Collaborative Tabs is rated "4–6 mo" in the pitch, but
  the audit finds 12–18 mo is realistic for v1 because
  the JS execution state problem (which the pitch is
  silent on) is the hard part.

The corrected stubs follow. Each one has the original
pitch plus an "Audit corrections" section.

---

## #1 — "Compute Credits" — Per-Site Budget for Local Resources (Audited)

**Original pitch.** Sites can request compute credits from
the browser; a per-site budget, user-visible, throttled
when exceeded.

**Audit corrections.**

- **Verdict: Truly novel — accepted, with a caveat.** A
  per-origin, user-visible, JS-queryable budget of
  CPU/RAM, enforced by the browser, with automatic
  throttling, is not in any shipped browser. Bandwidth
  caps (Chrome Data Saver, Firefox limit, Safari Low Data
  Mode) are the closest prior art but apply only to
  *network*. The closest local-compute prior art is
  Chrome's heavy ad intervention (2020, expanded 2024),
  which throttles ad-frame CPU and network — but it is
  *implicit* and *non-configurable*, not a user-visible
  per-origin budget.
- **Factual errors corrected:**
  - "Chrome Data Saver" is historical; was deprecated
    in Chrome 87 (Nov 2020). The current Chrome bandwidth
    feature is "Lite mode."
  - "WASM fuel metering (2023)" — the underlying
    proposal goes back to 2019.
  - "iOS Background App Refresh quotas" is per-app, not
    per-Safari-tab.
- **W3C Cooperative Scheduling of Background Tasks**
  (WICG, 2022) was the explicit ancestor of the "site
  asks" piece; it is now abandoned. Spiral's contribution
  is folding the per-site budget into the browser's
  existing resource model.

**Open questions.** Default credit — per-minute,
per-page-load, per-tab? User set site-by-site or
globally? What counts as "compute" — CPU, memory, GPU?
Adversarial arms race?

**Build cost: 4–6 mo for MVP; 8–12 mo for production.**

**Verdict change.** No change — the audit accepted "truly
novel" with caveats.

---

## #2 — "URL Time-Travel" — View Any URL as It Was at Any Point (Audited)

**Original pitch.** Every URL has a local timeline; rewind
to any past snapshot.

**Audit corrections.**

- **Verdict: Partially novel, with the gap narrower than
  the pitch suggests.** ArchiveBox (MIT, 2017+),
  SingleFile, Webrecorder/Conifer, Memento Tracer are
  prior art for "personal archive of URLs." Brave's
  Wayback Machine integration (Brave 1.50, Nov 2022) is
  the closest shipped feature, but it triggers only when
  the original page is missing.
- **The Memento Project (2009, RFC 7089)** is the academic
  prior art for "the web as a time dimension." Not cited.
- **Factual error:** "Chrome's 'view cached page'" was
  removed in Chrome 124 (June 2024). Replaced by the
  "Web Cache" DevTools tab.
- **Factual error:** "Common Crawl" does not have a
  per-URL fetch API; only bulk S3 dumps. Drop the
  reference.
- The genuinely novel piece: *integrated*, *default-on*,
  per-origin timeline UI. ArchiveBox is integrated
  (self-hosted) but not browser-level. Brave is
  browser-level but not default-on.

**Open questions.** Where does storage live? What's
the default retention? GDPR right-to-be-forgotten
handling?

**Build cost: 3–4 mo optimistic; 5–7 mo realistic.**
DOM snapshot serialisation is non-trivial; the diff
renderer is its own project.

**Verdict change.** No change.

---

## #3 — "Anti-Doom Scroll" — A Browser That Pushes Back (Audited)

**Original pitch.** The browser detects doom-scrolling and
gently intervenes.

**Audit corrections.**

- **Verdict: Partially novel, with the gap narrower than
  the pitch presents.** iOS Screen Time + Safari Website
  Limit (iOS 16, Sept 2022) is the closest shipped
  feature — it lets the user set a per-domain daily time
  limit on Safari, with the same intervention pattern
  the pitch describes. This is a **shipped
  browser-level intervention**.
- LeechBlock (2009), StayFocusd, WasteNoTime, Cold
  Turkey, Freedom, Forest are mature prior art.
- **This is a duplicate of Batch 4 #22 "The Browser
  That Asks Why."** The Batch 4 audit's verdict
  ("partially novel — proactive plain-language
  reflection") applies here too.

**Open questions.** Doom-scroll heuristic definition.
Default-on vs opt-in. Productivity enablement
(blocking) vs reflection (asking).

**Build cost: 2–3 mo is plausible.**

**Verdict change.** No change (PN), but noted as
duplicate of #22.

---

## #4 — "Real-Time Collaborative Tabs" — Tabs as CRDTs (Audited)

**Original pitch.** Multiple devices share one tab via
CRDTs; scroll on phone, laptop scrolls.

**Audit corrections.**

- **Verdict: Partially novel — but the cost estimate is
  wildly optimistic. The pitch undercounts the hardest
  part by a factor of 2–3×.**
- The "live tab state across devices" idea is partially
  novel (Firefox Sync / Chrome Sync sync metadata only;
  Apple Handoff is one-shot).
- **The hard part the pitch is silent on: JS execution
  state.** 99% of DOM mutations on the web come from JS,
  not user input. Syncing those is "share my entire JS
  execution state" — a different and far more expensive
  problem. **The M4 audit's #49 "DOM as Streamed CRDT"
  audit concluded 9–18 mo for a similar-scope MVP.** That
  verdict applies here.
- **The cost is wrong by a factor of 2–4×.** Yjs and
  Automerge are the right models, but the CRDT for the
  DOM (tree CRDT, not sequence CRDT) is research-level.
  Vortex would need to expose the JS heap to a CRDT
  substrate.

**Open questions.** Auth model. PII handling. Transport
(WebRTC + relay). E2E encryption.

**Build cost: 4–6 mo is wildly optimistic. 12–18 mo is
realistic.**

**Verdict change.** No change (PN), but cost corrected
to 12–18 mo.

---

## #5 — "Form Memory" — Every Form Field Is Remembered (Audited)

**Original pitch.** Every form field is remembered, learned,
suggested contextually.

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification;
  the "truly novel" was overclaimed).
- **Chrome Autofill does learn.** Chrome has had
  custom-field mapping since 2014 (Mac) and 2017
  (Windows/Linux). Chrome learns from past entries —
  if the user typed "Acme Corp" into "Company" once,
  Chrome suggests it next time. **The "no browser has a
  learning form memory" claim is wrong.**
- **1Password 7.4+ (2019) and Bitwarden (2022) have
  learning behaviour.** Roboform has done this for 25
  years. The "knowledge-based" framing for 1Password is
  out of date.
- The genuinely novel piece: *contextual free-text
  answer suggestion from past free-text* (the "answer
  to that survey question from last week"). The
  *learning* part is prior art.

**Open questions.** Cross-site leakage. Privacy
guarantees. Field-type inference for arbitrary fields
(not just HTML5 types).

**Build cost: 4–5 mo optimistic; 6–9 mo realistic.**
Field-type inference is the hard part; Chrome's
classifier has had 13 years of training.

**Verdict change.** Truly novel → **partially novel**.

---

## #6 — "The Permission Budget" — Permissions That Cost Something (Audited)

**Original pitch.** Permissions cost from a per-site budget;
sites earn back over time.

**Audit corrections.**

- **Verdict: Truly novel — accepted.** No shipped browser
  has a permission budget with earn-back. Chrome's
  Safety Check (Chrome 107, Oct 2022) revokes unused
  permissions but is a *one-time revoke*, not a
  continuous budget.
- Brave's Shields is the closest UX (per-site slider
  for tracker blocking) but it's a single dimension.
- Safari's ITP has a *storage* budget (not permissions).
- Apple's "Privacy Budget" proposal (WebKit, 2020) was
  for fingerprinting, not permissions, and was abandoned.
- W3C Privacy Budget CG (2023) is the active version.
- The novel piece is the *earn-back mechanic for browser
  permissions*; that is genuinely novel.

**Open questions.** First-party vs third-party iframe.
Interaction with the user slider. Bulk-ask detection.

**Build cost: 3–4 mo optimistic; 5–7 mo realistic.**
The "earn back" UX is the hard part.

**Verdict change.** No change.

---

## #7 — "DOM Tracer" — Hot-Path Visualisation of DOM Mutations (Audited)

**Original pitch.** A devtools panel showing every DOM
mutation with its causal JS stack.

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification).
- **Chrome DevTools has caught up.** "Why was this
  element removed?" (Chrome 109, Jan 2023) is the
  exact use case. Performance Insights (Chrome 124,
  May 2024) has attribution for long tasks. Animations
  panel (Chrome 78, 2019) has full keyframe + effect
  chain visualisation.
- **The genuinely novel piece:** every DOM mutation, with
  the full causal chain, real-time, panel UI. This is
  closer to **Time-Travel Devtools** (Batch 4 #31) than
  to a mutation panel.
- **Performance overhead** is the real concern.
  `MutationObserver` exists as a batching API because
  per-mutation recording is too expensive.

**Open questions.** Performance overhead.
Minified-bundle support. Stack capture in the bytecode
VM.

**Build cost: 2–3 mo for MVP; 4–6 mo for production.**

**Verdict change.** No change.

---

## #8 — "Inter-Tab Messaging Bus" — Tabs as Microservices (Audited)

**Original pitch.** A typed, secure bus between tabs.

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification).
  The "typed" piece is the novel contribution. The
  untyped pieces (`BroadcastChannel`, `SharedWorker`)
  exist.
- **Comlink** (Google, 2018, Apache-2.0) is the closest
  prior art for the "typed" piece. Cap'n Web
  (Cloudflare, 2024) is also relevant.
- The "per-tab opt-in" feature is not in the pitch's
  prior-art list but is a real requirement. Chrome's
  extension messaging has a similar pattern.

**Open questions.** Cold vs hot streams. Message format
(JSON, WASM-typed, both?). Scoping. Permission model.

**Build cost: 1–2 mo for MVP; 3–4 mo for production.**

**Verdict change.** No change.

---

## #9 — "Visual Search" — Real-Time Visual Query of the Page (Audited)

**Original pitch.** The user can draw a box around anything;
browser dispatches to a privacy-preserving on-device model.

**Audit corrections.**

- **Verdict: Truly novel — accepted, but the build cost
  is wildly underestimated.** The "always available,
  box-select, browser-level, can interact with the page's
  existing elements" claim is correct. No browser ships
  this. The closest is Bing Visual Search (Edge sidebar,
  2018) which is OS-Edge, not cross-browser, and does
  not interact with the page.
- **Apple Visual Look Up is a *spottable-objects* model
  (art, landmarks, plants, pets, etc.), not a general
  visual search.** It is not "general on-device visual
  search" — it's a narrow catalogue.
- **A general on-device visual search model is a 5–10
  year research bet, not a 4–6 month build.** The
  closest open-source options are Tesseract (OCR only),
  CLIP (embeddings, not search), and MobileNet
  (classification, not search). Building a *spottable-
  objects* model on MobileNet is feasible (6–12 mo);
  building a *general* model is research.
- **The on-device vs remote-query decision is v1, not
  v1.5.** Privacy-respecting = on-device = years of work.

**Open questions.** On-device model (years) vs remote
query (privacy-hostile). Catalogue size.

**Build cost: 4–6 mo is wildly optimistic.** 6–24 mo for
a narrow model; years for a general one.

**Verdict change.** No change (TN), but cost
substantially increased.

---

## #10 — "Self-Cleaning Storage" — Browsers That Forgive (Audited)

**Original pitch.** Storage is deprecated, quarantined, and
deleted based on access time.

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification).
- The components (time-based expiry for cookies) exist;
  Brave's "Forget me" / "Shields auto-shred" (2019) is
  30-day default with per-site override. Chrome's
  Storage Buckets API (2023) lets sites declare
  eviction policies. The "Clear cookies on quit"
  feature has existed since 2010.
- The novel piece is the *time-based policy across all
  storage types* (localStorage, IndexedDB, cookies,
  service workers, cached responses, permissions) with
  the deprecation/quarantine/delete state machine.
- **The "partially novel" classification is correct:**
  the components exist for cookies; the combination
  across all storage types is novel.

**Open questions.** Default retention. Opt-in vs
default-on. Cross-store coordination.

**Build cost: 2–3 mo for v1; 4–6 mo for production.**

**Verdict change.** No change.

---

## #11 — "The Browseable Browser" — Self-Describing Internals (Audited)

**Original pitch.** `browser://about` as a hypertext graph
of the browser's internals.

**Audit corrections.**

- **Verdict: Configuration** (correct self-classification).
  `chrome://*` (since 2008, ~80 pages in modern Chrome),
  Firefox's `about:` (since 1998), Brave's `brave://*`
  (since 2019), Safari's `Develop` menu are the prior
  art. **Firefox's `about:about` (since Firefox 3, 2008)
  is the "navigate-to-the-internals" UX pattern.**
- The novelty is *unification* (one graph rather than
  80 separate pages) and *cross-linking* (process X
  references permission Y references cached response Z).
- Beaker Browser's `dat://` (2017–2023) is the
  "hypertext graph of resources" prior art, but for user
  content, not browser internals.

**Open questions.** How deep does the graph go?
"Expose every Vortex object" is a research project.

**Build cost: 3–4 mo is plausible.**

**Verdict change.** No change.

---

## Final ranking (audited)

| Rank | # | Title | Original | Audited |
|------|---|-------|----------|---------|
| 1 | #6 | Permission Budget | TN | TN ✓ |
| 2 | #1 | Compute Credits | TN | TN ✓ (cost ↑) |
| 3 | #9 | Visual Search | TN | TN ✓ (cost ↑↑) |
| 4 | #5 | Form Memory | TN | **PN** ↓ |
| 5 | #4 | Real-Time Collab Tabs | PN | PN ✓ (cost ↑↑) |
| 6 | #3 | Anti-Doom Scroll | PN | PN ✓ (duplicate of #22) |
| 7 | #2 | URL Time-Travel | PN | PN ✓ (citations fixed) |
| 8 | #10 | Self-Cleaning Storage | PN | PN ✓ |
| 9 | #7 | DOM Tracer | PN | PN ✓ (prior art fuller) |
| 10 | #8 | Inter-Tab Messaging Bus | PN | PN ✓ |
| 11 | #11 | Browseable Browser | C | C ✓ |

**0 "truly novel" downgraded this round** (the "truly
novel" claims in #1, #6, #9 were all defensible with
caveats). **1 "truly novel" downgraded to partially
novel** (#5 Form Memory — Chrome's Autofill has had
learning since 2014; 1Password has had it since 2019).

**Net result:** Batch 2 has 3 truly novel (#1, #6, #9),
7 partially novel, 1 configuration.
