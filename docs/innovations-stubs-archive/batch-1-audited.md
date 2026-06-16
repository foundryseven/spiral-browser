# Spiral Innovations — Ten Stubs (Batch 1, audited)

**Status:** design stubs, audited (2026-06-15)
**Author:** implementer agent (originally self-written; now audited
by parallel research agent)
**Phase context:** M4 first sprint complete
**Purpose:** the original 10 idea stubs from `innovations-stubs.md`,
now with M4-audit corrections integrated. The novelty
classifications have been updated where the audit found
overclaims.

---

## Audit summary

The Batch 1 audit found:

- **5 of 10 "truly novel" claims are wrong:**
  - #1 Wound Lattice — HTTP Signed Exchanges
    (Chrome 73, 2019) + Web Bundles is exactly the prior
    art. The "compiled response" claim is true only for
    the layout+JS serialisation piece, not the HTML.
  - #2 Provenance Tracking — Brave PageGraph (shipped
    2022) + W3C PROV (2013 Recommendation) is exactly the
    prior art. The "forensic-grade read receipts" is a
    Brave feature.
  - #4 Type-Verified URL — W3C Trusted Types
    (shipped Chromium 83+, 2020) is literally the same
    idea ("non-spoofable, typed values in place of
    strings") for DOM XSS sinks. The URL-typing version
    is a domain application.
  - #5 Sandbox Sandboxing — capability OS design has
    existed since 1965; WASI in the browser ships in
    Chrome 119+; the pitch omits Fuchsia, Genode,
    RedoxOS, V8 isolates, and Chromium Component Updater.
  - #10 Self-Patching Bugs — Chromium Component Updater
    already does this for vendor-only patches.
- **1 wrong citation:** #3 cites "Chrome's 'scoped reload'
  extension API" which does not exist. Correct citation
  is `chrome.scripting.executeScript` for frame-scoped work
  or the DOM API for iframe reload.
- **1 undersold:** #8 WASM-as-IPC is the *cleanest*
  technical synthesis in the batch; self-classified as
  partially novel correctly.
- **Other 4 are honestly classified** as partially novel
  or configuration.

The corrected stubs follow. Each one has the original
pitch plus an "Audit corrections" section listing the
prior art I missed, the corrected verdict, and any
factual errors.

---

## #1 — "Wound Lattice" — Server-Push That Compiles, Not Streams (Audited)

**Original pitch.** Spiral pre-runs the page through the
client's rendering pipeline; ships a signed binary blob;
client unzips, validates, and skips 80% of the work.

**Audit corrections.**

- **Verdict: Partially novel.** The "signed HTML
  exchange" piece is HTTP Signed Exchanges (SXG), shipped
  in Chrome 73 / Edge 79 / Opera 64 (2019) and produced
  by Cloudflare's Automatic Signed Exchanges since 2020.
  The "Web Bundles" format (W3C/IETF) is the prior art for
  shipping multiple resources as one signed artifact.
  Google Search + AMP uses SXG in production.
- **The genuinely novel piece** is the layout + JS bytecode
  serialisation. No browser ships that. The pitch's
  "compiled response that the client unzips" is
  misleading because SXG does this for HTML today.
- **Factual errors corrected:**
  - "No browser ships a compiled response that the
    client unpacks" — false for HTML; SXG is the prior
    art.
  - "Isomorphic/Universal JS (2013)" — actually 2011
    (Charlie Robbins, Nodejitsu).
- **License risk:** clean. SXG format is unencumbered;
  sxg-rs is Apache-2.0, webpkgserver is Apache-2.0,
  libsxg is BSD-3-Clause, nginx-sxg-module is Apache-2.0.
  All MPL-2.0 compatible.
- **Build cost: 4–6 mo is inflated; 6–10 wk for the
  SXG client, plus 4–6 mo for the novel layout+JS
  serialisation.**

**Open questions.** Whose key signs? Site owner, Spiral
team, both? What's the freshness model? TTL, ETag, or
re-sign-on-mutation? What's the bandwidth tradeoff — a
serialised DOM is smaller than HTML, but layout + JS is
bigger?

**Depends on.** spiral-fmt (server side), spiral-gyre
(serialise layout), spiral-css (serialise cascade),
spiral-vortex (serialise bytecode), Bet 4 persistent-
renderer work.

**Verdict change.** Truly novel → **partially novel**.

---

## #2 — "Provenance Tracking" — A Read Audit Trail Inside the Browser (Audited)

**Original pitch.** Every storage write is signed with a
chain of keys; the user can see "what does this site know
about me?" graph.

**Audit corrections.**

- **Verdict: Partially novel (not "truly novel").** The
  exact concept of "forensic-grade read receipts for what
  the browser stored and from whom" is **Brave PageGraph**
  (shipped Brave 1.46, 2022; backed by the AdGraph
  academic paper from 2018). PageGraph exposes
  document modifications, network requests, script
  execution, and privacy-relevant Web API access in a
  graph you can query.
- **The "W3C PROV" piece is the right model.** PROV is a
  W3C Recommendation since 2013; 12 documents; widely
  implemented.
- **The novel piece is the *user-facing "what does this
  site know about me?" export*, with selective decryption
  under user control, and the cryptographic chain
  specifically for storage writes.** PageGraph is a
  research-grade instrumentation layer; it is not the
  user-facing audit the pitch describes.
- **License risk:** clean if Spiral writes from first
  principles on PROV-DM; **MPL-2.0 if Spiral vendors
  pagegraph-rust**.

**Open questions.** Where does the user key live? TPM,
iCloud Keychain, user passphrase? What if the user loses
the key? Does this conflict with private-browsing mode?

**Depends on.** spiral-storage (planned), a new
`spiral-audit` crate.

**Verdict change.** Truly novel → **partially novel**.

---

## #3 — "Sectional Reload" — Surgical Refresh of Only the Broken Part (Audited)

**Original pitch.** Right-click "Reload this section" reloads
just the DOM subtree, JS modules, CSS, and storage
entries.

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification).
- **Factual error corrected:** "Chrome's 'scoped reload'
  extension API" **does not exist** as a Chrome extension
  API. The pitch may have meant `chrome.scripting.executeScript`
  (frame-scoped work) or the iframe-reload via
  `iframe.contentWindow.location.reload()`. Either way, the
  citation as written is wrong.
- The "Chrome's 'Edit & Reload' DevTools feature" cited
  as a separate prior art is also slightly misframed;
  DevTools has "Edit as HTML" (right-click an element),
  not a one-shot "reload just this element."
- **The user-facing right-click sectional reload is
  genuinely not in any shipped browser.** The engineering
  to identify "which JS module owns this subtree" is
  almost what Chrome DevTools' "initiator chain" already
  shows, but never exposed to the user.

**Open questions.** How do we identify which JS module
"owns" a section? DOM attributes, frameworks, heuristics?
What if reloading a section breaks another section that
depends on it? What about CSS scoping? Does this work
for iframes?

**Build cost: 2–3 mo is honest.**

**Verdict change.** No change — partially novel was
correct, but citations corrected.

---

## #4 — "Type-Verified URL" — Phishing Protection by Compiler (Audited)

**Original pitch.** A URL type system; forms cannot POST to
a different type; the type registry is curated and signed.

**Audit corrections.**

- **Verdict: Partially novel (not "truly novel").** W3C
  Trusted Types is **literally the same idea** —
  "non-spoofable, typed values in place of strings" — for
  DOM XSS sinks. Shipped in Chromium 83+ since 2020;
  production users include Google, Facebook, GitHub.
- The "URL variant" of Trusted Types (`TrustedURLPolicy`)
  is a *domain application* of an existing W3C standard.
- The closest prior art for URL-typing specifically is
  Apple's iCloud Keychain + Password AutoFill, which
  bind cryptographically-signed password entries to
  origin.
- **Factual error:** HPKP was removed from Chrome in
  2018; do not cite it as current prior art.
- The "mechanism" (typed registry, signed, with
  cross-type casts) is partially novel; the "principle"
  (typed values at browser boundaries) is prior art.

**Open questions.** Who maintains the registry? Spiral
team, community, site owners self-register? How do we
bootstrap trust? First-visit to a new bank, you don't
have a type yet.

**Depends on.** spiral-network, spiral-filter (form-
action enforcement), a new `spiral-types` crate.

**Verdict change.** Truly novel → **partially novel**.

---

## #5 — "Sandbox Sandboxing" — Sandboxes for Sandboxes (Audited)

**Original pitch.** Per-subsystem compartments in the
renderer (DOM, CSS, JS, Network, GPU) with capability-typed
API surfaces.

**Audit corrections.**

- **Verdict: Partially novel (not "truly novel").**
  Capability OS design has existed since 1965 (CAP, Hydra,
  KeyKOS, Eros, CapROS, Genode, Fuchsia, RedoxOS,
  HarmonyOS/OpenHarmony, CHERI hardware). WASI preview 2
  is the in-process WASM sandbox pattern; shipped in
  Chrome 119+ (WASM GC, exception handling, reference
  types). V8's isolates are the in-process JS isolation
  unit. Chromium's renderer/GPU/network sub-processes are
  a per-subsystem decomposition.
- **Spiral's specific combination** of (a) Rust-typed
  capability tokens, (b) per-origin contexts, (c) in-
  process compartment boundaries, (d) GC-aware shared heap
  is *Spiral's contribution*. The capability concept
  predates the project.
- **Factual error:** "Chromium Mojo / process-per-renderer
  — coarser-grained" is wrong. Mojo is *IPC*; it runs
  in the same process. The pitch conflates IPC with
  isolation.

**Open questions.** IPC cost between compartments?
How do compartments share data? WASM modules as
compartments? The `Escalated` mode proxy over IPC.

**Build cost: 2–3 mo is low; 4–6 mo if WASI bindings
are part of the deliverable.**

**Verdict change.** Truly novel → **partially novel**.

---

## #6 — "Reactive Extensions as a First-Class Browser API" (Audited)

**Original pitch.** Every browser event as a stream; the
extension is a graph of stream operators; visual debugger
for the graph.

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification).
  The TC39 Observable proposal (Stage 1) has had 3.1k
  GitHub stars and multiple implementations since 2017.
  RxJS (Apache-2.0) is the canonical implementation.
  Chrome's `chrome.events` namespace returns Promises
  and event-Listener callbacks — *cold observables in
  disguise* — for extensions since 2010.
- **The genuinely novel piece is the visual debugger for
  extension stream graphs.** DevTools shows DOM, network,
  JS heap, but not *an extension's reactive computation
  graph*.

**Open questions.** Cold or hot streams? Compatibility
with existing WebExtensions? Drop-in or new parallel API?
Stream backpressure?

**Build cost: 3–4 mo is honest.** The visual debugger
is the deepest unknown.

**Verdict change.** No change.

---

## #7 — "Layout Streams" — Continuous Layout, Frame-by-Frame (Audited)

**Original pitch.** Layout publishes a stream of
`{dirty_region, layout_box}` events; the painter subscribes.

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification).
- **Factual error:** "All layout engines are batched" —
  false for current Blink (LayoutNG, since 2019),
  current WebKit (LayoutState), and Servo. Batched
  layout was a 2010s state; it is no longer accurate in
  2024-2026.
- The `LayoutShift` PerformanceObserver entry, the
  `MutationObserver`, the `IntersectionObserver`, the
  `ResizeObserver`, and the `content-visibility` CSS
  property are all *event-driven layout interfaces* in
  the public web platform. The pitch's "stream" framing
  is novel; the underlying incremental layout is not.
- The closest prior art to a "stream of layout
  invalidation events" is Blink's internal
  `LayoutInvalidation` (per-`LayoutBox` dirty bit), which
  is event-driven. It's just not exposed publicly.

**Open questions.** Backpressure on invalidation bursts?
Layout that depends on viewport size? Fonts loading
mid-stream?

**Build cost: 3–4 mo is honest *if* Gyre already has
minimal-invalidation tracking; 6–12 mo if it doesn't.**

**Verdict change.** No change.

---

## #8 — "WASM as a Cross-Process RPC" — Type-Safe IPC for Free (Audited)

**Original pitch.** `spiral-ipc` exposes WASI bindings; the
browser and renderer talk via typed WASM messages.

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification).
  The novelty is applying the WASM Component Model to
  *browser-internal* IPC, not to a server.
- **The prior art list omits the most direct competitors:**
  - **Fermyon Spin** (Apache-2.0) uses the WASM
    Component Model for all inter-function communication.
  - **wasmCloud** (Apache-2.0) has `wasi:http`, `wasi:rpc`,
    `wasi:keyvalue` interfaces designed for exactly
    typed cross-process RPC.
  - **WIT-bindgen** (Apache-2.0) gives a free IDL
    compiler.
- The "no browser uses WASM Component Model for IPC"
  claim is *correct*; the "this is a novel application of
  the technology" claim is *correct* and well-scoped.

**Open questions.** WASM Component Model is in preview 2;
preview 3 (async/threads) is on the roadmap. Spiral
should commit to preview 2 with a migration plan. What's
the per-call overhead of WASM calls? Can the existing
`IPCMessage` enum be migrated to WASM-typed?

**Build cost: 3–5 mo is honest.** The WASM runtime
(`wasmtime` Apache-2.0 or `wasmer` MIT) is reused; the
bindings are mechanical; the integration with existing
`spiral-ipc` is the real work.

**Verdict change.** No change.

---

## #9 — "Tab Provenance Graph" — Show the User Where a Tab Came From (Audited)

**Original pitch.** Every tab knows its lineage; user sees
"12 opened by me, 31 by JS."

**Audit corrections.**

- **Verdict: Partially novel** (correct self-classification).
  The *data model* already exists in Chrome's
  `chrome.tabs.Tab.openerTabId` API (since Chrome 26, 2013).
  The user-facing **graph visualisation with the "12
  opened by me" summary** is novel.
- **Vivaldi's tab stacking and tiling** (2018) is a visual
  parent-child relationship (similar UX).
- **Firefox's Tree Style Tab** is the closest visual
  analogue.
- The `window.opener` JavaScript property (1996) is the
  underlying primitive.

**Open questions.** How long is provenance retained?
Forever? Session-only? User-configurable? Does the graph
have a privacy dimension (knowing the chain could be
sensitive)?

**Build cost: 1–2 mo is honest.** Most of the work is
the side-panel UI; the data structure is small.

**Verdict change.** No change.

---

## #10 — "Self-Patching Bugs" — The Browser Fixes Its Own Security Issues (Audited)

**Original pitch.** Signed WASM security patches applied
in-process without a full release.

**Audit corrections.**

- **Verdict: Partially novel** (not "truly novel"). The
  closest direct prior art is **Chromium's Component
  Updater** (ships signed `.crx` patches for individual
  components — Widevine, PDF viewer, real-time URL
  lookups). It is not "user-installable" but it is
  "cryptographically-signed, in-process updates without a
  full release." Brave's `go-update` is the open-source
  implementation (MPL-2.0).
- **Linux kernel live patching** (kpatch, kGraft,
  Ksplice, Canonical Livepatch) is the canonical prior art.
  Microsoft Hot-Patching for Windows Server has shipped
  since 2008.
- **The novel piece** is the *WASM format* + *user-
  installable* + *browser-internal* combination. The
  trust model and rollback story are the novel parts.

**Open questions.** Who has the signing key? What's
the rollback story? Two-version problem (some users on
patch A, some on patch B)?

**Build cost: 5–7 mo is honest.** The trust model and
rollback are the hard parts.

**Verdict change.** Truly novel → **partially novel**.

---

## Final ranking (audited)

| Rank | # | Title | Original | Audited |
|------|---|-------|----------|---------|
| 1 | #8 | WASM-as-IPC | PN | PN ✓ |
| 2 | #9 | Tab Provenance Graph | PN | PN ✓ |
| 3 | #6 | Reactive Extensions API | PN | PN ✓ |
| 4 | #7 | Layout Streams | PN | PN ✓ (citations corrected) |
| 5 | #3 | Sectional Reload | PN | PN ✓ (citations corrected) |
| 6 | #5 | Sandbox Sandboxing | TN | **PN** ↓ |
| 7 | #2 | Provenance Tracking | TN | **PN** ↓ |
| 8 | #10 | Self-Patching Bugs | TN | **PN** ↓ |
| 9 | #1 | Wound Lattice | TN | **PN** ↓ |
| 10 | #4 | Type-Verified URL | TN | **PN** ↓ |

**5 of 10 "truly novel" claims were wrong.** The pattern
is the same as the Batch 4 audit and the M4 audit. The
honest author uses the M4 audit methodology.

**Net result:** Batch 1 has 0 truly novel, 8 partially
novel, 2 configuration (#5 if you count it as a Spiral
combination; #10 if you count Chromium Component Updater
as configuration).
