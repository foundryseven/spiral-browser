# Spiral Innovations II — Eleven More Genuinely New Ideas

**Status:** design stubs (2026-06-15), user-approved
**Author:** implementer agent
**Phase context:** M4 first sprint complete; companion to
[`docs/innovations-stubs.md`](innovations-stubs.md).
**Purpose:** another round of ten-eleven novel ideas for
Spiral. Same structure as the first batch.

---

## How to read this doc

Each idea has:

1. **Pitch** — one paragraph, what the user/developer gets.
2. **What's novel** — the specific thing with no prior art.
3. **What's prior art** — honest assessment of components that
   already exist.
4. **Build cost** — engineer-months estimate.
5. **M-month target** — which phase this would land in.
6. **Open questions** — what we don't know yet.
7. **Depends on** — which existing Spiral crate(s) it builds on.

Novelty classification (same as the M4 audit):

- **Truly novel** = no shipped browser does this combination.
- **Partially novel** = the components exist; Spiral's
  combination is new.
- **Configuration** = a sound engineering choice, not a
  uniqueness claim.

---

## 1. "Compute Credits" — A Browser-Native Token Economy for Local Resources

**Pitch.** Sites can request compute credits from the browser —
"this page would like to use 5 seconds of CPU and 50 MB of RAM
in the next 10 seconds for a background ML inference, an
encryption job, or a layout pre-compute." The browser maintains
a per-site credit budget, the user can see and adjust the
budget, and the browser enforces that no site exceeds its
allotment. Sites that want more compute can ask the user; sites
that abuse their credit get throttled automatically.

**What's novel.** Browsers have network bandwidth caps (Chrome
Data Saver, Firefox's "limit data usage"). No browser has a
general *local compute* budget. The closest prior art is
WASM's fuel meter (Interpreter::set_fuel) — that's per-module,
per-execution, not a per-site policy. Spiral's per-origin
isolation gives us the natural unit: a site has N credits per
minute, the browser enforces it.

**What's prior art.**
- **WASM fuel metering** (WebAssembly CG, 2023) — count-down
  fuel for a single execution.
- **Chrome's "heavy ad intervention"** (2020) — Chrome throttles
  ads that use too much CPU/network.
- **iOS Background App Refresh quotas** — per-app time budget.
- **Linux cgroups** — per-process resource control.

**Build cost.** 4–6 engineer-months. Includes: the per-origin
credit store; the per-frame budget enforcement in Vortex; the
exposure to JS (`navigator.computeBudget.remaining`); the user
slider; the "this site used its budget" UI; the back-pressure
mechanism when a site hits its limit.

**M-month target.** M30+ (needs mature Vortex with fuel metering).

**Open questions.**
- What's the default credit? Per-minute? Per-page-load? Per
  -tab?
- Does the user want to set it site-by-site or globally?
- What counts as "compute"? CPU time only? Memory? GPU?
- Does this create an adversarial arms race (sites that claim
  to need 1s of compute but actually use 5s)?

**Depends on.** `spiral-vortex` (fuel metering in the bytecode
VM), `spiral-gyre` (per-frame layout budget), `spiral-ui` (the
budget UI).

---

## 2. "URL Time-Travel" — View Any URL as It Was at Any Point in History

**Pitch.** A browser-level diff tool for any URL. The user can
say "show me nytimes.com on 2024-09-12" or "show me example.com
as the Wayback Machine saw it in 2017" or "show me the page that
was served 3 minutes ago when the bug report came in." Spiral
ships with a *frozen DOM diff* view: the page renders in a
sandboxed iframe, the user can flip between snapshots, and the
browser highlights what changed.

**What's novel.** The Wayback Machine has the *archive* but not
the *view*. The browser has the *view* but not the *archive*.
Spiral could bridge them: every page the user visits is
automatically snapshotted to a local archive (configurable
retention), and the user can rewind any page they ever visited.
For pages they didn't visit, Spiral can fetch from external
archives (Wayback Machine, Common Crawl) and render in a
sandboxed iframe.

**What's prior art.**
- **Wayback Machine** (2001) — the archive, but a separate
  site, not integrated.
- **Chrome's "view cached page"** — a single snapshot, not a
  timeline.
- **Chrome's local storage / cache** — pages are cached
  implicitly, but not as a queryable archive.
- **Brave's "Wayback Machine" integration** (2022) — adds
  archive access, but only when the original page is missing.

**Build cost.** 3–4 engineer-months. Includes: the local archive
store (compressed DOM snapshots, configurable retention); the
sandboxed iframe viewer; the diff renderer; the Wayback Machine
fetch integration; the UI for browsing a URL's history.

**M-month target.** M36+ (needs stable DOM serialisation and
rendering sandbox).

**Open questions.**
- Where does the storage live? Local disk? Encrypted cloud?
  Both?
- What's the default retention? Last 30 days? Last 1000 pages?
  Per-site user-configurable?
- How do we handle pages with authentication? Snapshots may
  contain personal data.
- What about the GDPR "right to be forgotten" — does the
  archive need a delete button?

**Depends on.** `spiral-dom` (serialise), `spiral-storage`
(planned), `spiral-ui` (timeline viewer).

---

## 3. "Anti-Doom Scroll" — A Browser That Pushes Back

**Pitch.** The browser detects *doom-scrolling patterns* (rapid
scroll, long session, repeated check-ins on social media) and
gently intervenes. The intervention is configurable: dim the
screen, add a 5-second delay before loading the next page, show
the user a "you've been scrolling for 47 minutes" message,
nudge them toward a healthier page. The user can dismiss any
intervention, set per-site rules, and disable it entirely. The
default is *off* — this is a feature for users who want it, not
a moral judgement.

**What's novel.** iOS Screen Time, Android Digital Wellbeing, and
the macOS Screen Time API all do *measurement*. None of them
*intervene in the browser itself*. Spiral can intervene at the
exact moment of doom-scrolling because the browser sees the
scroll, the time, and the URL.

**What's prior art.**
- **iOS Screen Time / Android Digital Wellbeing** — measurement
  and app-level blocking, not browser-level.
- **Chrome's "site engagement"** — Chrome knows how often you
  visit a site, but doesn't surface it.
- **Firefox's "procrastination" extensions** — third-party,
  fragile.
- **Headspace / Calm** — meditation apps, not browser features.

**Build cost.** 2–3 engineer-months. Includes: the doom-scroll
detection (heuristics on scroll velocity, session length,
re-visit frequency); the intervention system (dim, delay,
message); the per-site opt-out; the user-configurable patterns
("warn me if I visit Twitter more than 5 times in 10 minutes").

**M-month target.** M18+ (when session/tab management is real).

**Open questions.**
- What's a doom-scroll pattern? A formal definition is hard;
  heuristics will be approximate.
- Will users feel judged? The default-off posture helps, but
  the feature is inherently paternalistic.
- Can this be used for productivity *enablement* too? "Block
  twitter.com from 9am to 5pm on weekdays."
- Is the browser the right layer, or should this be a separate
  app?

**Depends on.** `spiral-browser` (session/tab tracking),
`spiral-ui` (intervention UI).

---

## 4. "Real-Time Collaborative Tabs" — Tabs as CRDTs

**Pitch.** Open a tab on your phone, your laptop, and your
desktop — they're all viewing the same URL, in sync. Scroll on
the phone, the laptop scrolls. Click a link on the laptop, the
phone navigates. Or pick any subset: the phone is read-only,
the laptop is interactive. The browser uses CRDTs (Conflict-free
Replicated Data Types) to merge state across devices without
needing a server.

**What's novel.** Firefox Sync syncs bookmarks, history, and
*tabs* (the metadata) but not the *state* (scroll position,
form values, video playhead). Chrome Sync similar. No browser
syncs *live tab state* — they sync at-rest snapshots of tabs
that have been closed. Apple's "Handoff" syncs a *handoff point*
(you can move an activity to another device), not continuous
state.

**What's prior art.**
- **Firefox Sync / Chrome Sync** — at-rest tab metadata sync.
- **Apple Handoff** — single-action hand-off, not continuous
  state.
- **CRDT-based collaborative editing** (Yjs, Automerge) — the
  same data structures we'd use, but for documents, not tabs.
- **Multi-device video playback sync** (Teleparty, Netflix
  Party) — the same idea, but for one app, not a general
  browser primitive.

**Build cost.** 4–6 engineer-months. Includes: the CRDT
implementation (Yjs-like); the peer-to-peer transport (WebRTC
or a relay); the per-tab sync protocol; the conflict resolution
UX ("phone scrolled past laptop, who's right?"); the
authentication.

**M-month target.** M48+ (post-v0.1).

**Open questions.**
- How do we handle network failures? CRDTs are eventually
  consistent; what does "the tab" look like while the devices
  are disconnected?
- Privacy: a CRDT over the network means the server (if any)
  sees the tab state. End-to-end encryption is mandatory.
- What's the trust model between devices? Pairing? Per-tab
  opt-in?
- What about form values that contain PII? The CRDT should
  encrypt field values, not just transmit them.

**Depends on.** `spiral-browser` (tab model), `spiral-net`
(WebRTC, E2E encryption), `spiral-ui` (sync UI).

---

## 5. "Form Memory" — Every Form Field Is Remembered

**Pitch.** Every form field on every page the user has ever
filled is remembered, locally, in a structured way. When the
user returns to a form (even on a different site, even with
different field names), the browser offers to fill it. The
browser learns the user's name, address, phone, email, common
answers ("flexible on dates", "no, I don't want the
newsletter"), preferred pronouns, and other patterns — and
offers them contextually.

**What's novel.** Browsers have built-in form fill (Chrome's
"Autofill", Safari's "AutoFill"). But they're either
hard-coded ("name, address, phone, credit card") or
extension-driven (1Password, Bitwarden). No browser has a
*learning* form memory that adapts to the user's actual
behaviour and suggests context-appropriate answers. Chrome
suggests "your email" but not "the answer to that survey
question from last week."

**What's prior art.**
- **Chrome Autofill / Safari AutoFill** — hard-coded fields.
- **1Password / Bitwarden** — extension-based, knowledge-based.
- **Google Forms / Typeform "save and continue"** — per-form
  state, not general.
- **Notion AI / spreadsheet auto-complete** — learns from
  past entries, but in a single app, not across the web.

**Build cost.** 4–5 engineer-months. Includes: the local store
of form history; the field-type inference (which fields are
"first name", "address line 1", etc.); the contextual offer
("you typed 'flexible' on three previous forms — want to
use that?"); the user-visible history ("this is what we
remember"); the per-field opt-out.

**M-month target.** M30+ (needs stable form parsing).

**Open questions.**
- Privacy: the local store is *exactly* the kind of thing
  advertisers would love. Must be encrypted, must be local,
  must be deletable.
- How do we detect "this is a survey question I want to
  remember" vs "this is a one-off value"?
- Cross-site leakage: if the user typed "I hate Company X" on
  site A, do we offer it on site B? Almost certainly not.
- What about password fields? Definitely not remembered, but
  how do we *exclude* them reliably?

**Depends on.** `spiral-form` (new, or extend `spiral-dom`),
`spiral-storage` (encrypted local store), `spiral-ui`
(suggestion UI).

---

## 6. "The Permission Budget" — Permissions That Cost Something

**Pitch.** Every site starts with a small *permission budget*.
Asking for geolocation, notifications, camera, microphone, or
clipboard access *spends* from the budget. Spent permissions
need to be earned back (e.g. by 30 days of consistent use
without a complaint from the user). Sites can't pile up
permissions and then use them all at once — the budget is
enforced. The user can see the budget in real time and grant
extras explicitly.

**What's novel.** Browsers have *permission dialogs* (request,
allow, deny, "always allow"). They don't have a *budget*. A
"always allow" is a permanent grant with no cost to the site.
Spiral's budget model creates scarcity, which forces sites to
*earn* user trust rather than assume it.

**What's prior art.**
- **Chrome's permission system** (2015+) — request/allow/deny,
  no budget.
- **iOS App Tracking Transparency** (2021) — required opt-in
  for tracking; a coarse budget ("yes" or "no" for the whole
  category).
- **FTC's "Do Not Track"** (2009) — universally ignored.
- **Game-theoretic mechanism design** — academic prior art for
  budget-based permission systems.

**Build cost.** 3–4 engineer-months. Includes: the per-origin
budget store; the per-permission cost model; the "earn back"
mechanism; the user-visible budget display; the bulk-ask
detection (a site that asks for 5 permissions at once gets
penalised).

**M-month target.** M24+ (needs permission system mature).

**Open questions.**
- What's the cost of each permission? Notifications might be
  cheap (rarely abused); camera might be expensive (always
  reviewed).
- How is the budget earned back? Time-based? User-actions-
  based? A combination?
- What about first-party use vs third-party embeds? A
  notification from a third-party iframe might cost differently.
- Does the budget interact with the user slider (#3 in
  innovations-stubs.md)?

**Depends on.** `spiral-dom` (permission origin tracking),
`spiral-ui` (budget display), `spiral-storage` (budget store).

---

## 7. "DOM Tracer" — Hot-Path Visualisation of DOM Mutations

**Pitch.** A devtools panel that shows every DOM mutation in
real time, with the JS stack that caused it. Not a generic
"this is what's changing" — a *causal* trace: "this `<div>`
was inserted at 12:34:56.789 by `eval()` called from
`renderComments()` called from the `IntersectionObserver`
callback for the previous comment." Reverse the stack:
"every observer that could have caused this, here's the
chain."

**What's novel.** Chrome DevTools has a Performance panel that
records DOM mutations. It has the Mutation Events panel that
shows live mutations. Neither ties a mutation to the *causal
chain* (the stack of JS that led to it). The closest is the
"why was this element removed" question in devtools, which
requires stepping through the code manually.

**What's prior art.**
- **Chrome DevTools Performance panel** — records but doesn't
  explain.
- **React DevTools profiler** — component-level trace, but
  framework-specific.
- **Reactive programming debuggers** (RxJS marble diagrams) —
  visual, but not DOM-specific.
- **`MutationObserver` callback traces** — useful but
  after-the-fact.

**Build cost.** 2–3 engineer-months. Includes: the trace
collector (intercepts every DOM mutation in Vortex); the causal
chain reconstruction (call stack + observer chain); the panel
UI; the export.

**M-month target.** M30+ (when Vortex has a stable call stack
trace).

**Open questions.**
- What's the performance overhead? Intercepting every
  mutation is potentially expensive.
- How do we handle third-party scripts that don't expose their
  stack? Webpack minification strips source maps.
- Can this be opt-in? Yes, but opt-in tools are often
  forgotten.

**Depends on.** `spiral-vortex` (stack traces), `spiral-dom`
(mutation interception), `spiral-ui` (devtools).

---

## 8. "Inter-Tab Messaging Bus" — Tabs as Microservices

**Pitch.** A typed, secure, optional messaging bus between
tabs. Tab A can publish events to a named channel. Tab B
subscribes. The bus enforces: same-origin tabs can talk;
cross-origin tabs need user opt-in; the bus is on the same
process, no network. A pattern for "open the same site in
multiple tabs and let them coordinate" without rolling a
websocket.

**What's novel.** `BroadcastChannel` and `SharedWorker` exist,
but they're heavyweight (BroadcastChannel is per-origin, no
typing; SharedWorker is a separate JS context). No browser
exposes a *typed* inter-tab bus with the same ergonomic quality
as a typed in-process queue.

**What's prior art.**
- **`BroadcastChannel`** (2015) — same-origin, untyped,
  stringly-keyed.
- **`SharedWorker`** (2009) — heavyweight, separate JS context.
- **LocalStorage events** — hack, not designed for this.
- **WebSocket to localhost** — works, but requires a server.

**Build cost.** 1–2 engineer-months. Includes: the typed
message bus; the same-origin / cross-origin policy; the
permission model; the per-tab opt-in.

**M-month target.** M18+ (when tab coordination is real).

**Open questions.**
- What is the message format? JSON? WASM-typed? Both?
- How is the bus scoped? Per-browser-instance? Per-profile?
- Does the user need to see a notification ("Tab A wants to
  message Tab B")?
- What about cross-tab SharedArrayBuffer? Could be combined.

**Depends on.** `spiral-browser` (tab management), `spiral-vortex`
(messaging primitives), `spiral-context` (capability types for
the bus).

---

## 9. "Visual Search" — Real-Time Visual Query of the Page

**Pitch.** The user can draw a box around anything on the
screen and the browser finds it. "Find this product." "What is
this icon." "Translate this text." The browser dispatches to a
visual search service (configurable; default is a privacy-
preserving on-device model). The result is a panel that
overlays the page: "this is the Adidas Ultraboost 22, $180,
available at..." or "this icon means 'share'" or "this text
translates to 'open in new tab'."

**What's novel.** Google Lens does this. Apple's Visual Look Up
does this. But both are OS-level or app-level features, not
browser-level. Spiral's visual search is *always available* on
any page the user can see, with a single gesture (the box
select). The visual search is *part of the page* — it can
interact with the page's existing elements.

**What's prior art.**
- **Google Lens** (2017) — mobile-first, OS-level.
- **Apple Visual Look Up** (2021) — iOS/macOS-level.
- **Chrome's "search images on page"** — keyword search, not
  visual.
- **TinEye / Yandex reverse image search** — image-based, but
  requires a separate flow.

**Build cost.** 4–6 engineer-months. Includes: the gesture
detection (box select); the on-device visual model (or the
  remote query protocol); the result panel; the privacy
  controls (which model? which data leaves the device?).

**M-month target.** M42+ (needs on-device ML infrastructure).

**Open questions.**
- On-device model or remote query? On-device is private but
  limited. Remote is more powerful but a privacy risk.
- How do we handle copyright? Showing "this is a copyrighted
  image" or "this image is from a known photographer"?
- Does the user see *what* the visual search sends? Full
  transparency required.
- What about accessibility? The user can already do this with
  alt text; is visual search duplicating that?

**Depends on.** `spiral-render` (the gesture), `spiral-ui` (the
result panel), new `spiral-vision` crate (model integration).

---

## 10. "Self-Cleaning Storage" — Browsers That Forgive

**Pitch.** Sites accumulate localStorage, IndexedDB, cookies,
service workers, cached responses, and permissions over time.
Most users never clean up. Spiral runs a background sweep:
storage that hasn't been accessed in 90 days is *deprecated*,
in 180 days is *quarantined* (kept but not auto-loaded), in
365 days is *deleted*. The user can see the timeline for each
site and override ("this is a long-term project, keep this
data"). The default is *aggressive* but never silent — every
deletion is logged and reversible for 30 days.

**What's novel.** Browsers have "clear browsing data" (manual,
all-or-nothing). Chrome's "Storage Access API" is per-page
permission. No browser has a *time-based* storage policy that
automatically deprecates, quarantines, and deletes based on
access patterns.

**What's prior art.**
- **Chrome's "clear browsing data"** — manual, not automated.
- **Firefox's "strict" cookie behaviour** — expires cookies
  aggressively, but only cookies.
- **iOS app offloading** — removes rarely-used apps, but
  whole-app.
- **Git's garbage collection** (`git gc`) — prunes unreachable
  objects after a time; the same idea applied to web storage.

**Build cost.** 2–3 engineer-months. Includes: the access-time
tracker; the deprecation/quarantine/delete state machine; the
per-site override; the undo log; the user UI.

**M-month target.** M18+ (when storage is real).

**Open questions.**
- Does the user have to opt in, or is the default-aggressive
  policy the right choice? Aggressive-by-default is paternalistic
  but reduces real harm.
- How do we distinguish "this site hasn't been visited because
  the user is on holiday" from "this site is forgotten"?
- What about offline-first sites? They might be unused for
  months but expected to be available.
- Does the user get a digest email ("we deleted 47 sites of
  data this month")?

**Depends on.** `spiral-storage` (planned), `spiral-ui` (storage
timeline UI).

---

## 11. "The Browseable Browser" — Self-Describing Internals

**Pitch.** The user can navigate to `browser://about` and see
not a marketing page, but a *live, navigable* description of
the browser itself. Every setting, every permission, every
running process, every loaded extension, every storage entry is
a navigable resource. The browser is introspectable in the
same way that a well-designed web app is. Click on a process,
see what tabs use it. Click on a permission, see which sites
have it. Click on a cached response, see when it was fetched
and from where.

**What's novel.** Chrome has `chrome://flags`, `chrome://settings`,
`chrome://inspect`, etc. — many separate pages, each its own
thing. Firefox similar. None of them are *navigable as a
hypertext system*. The browser's internals are a graph, and
the user should be able to explore that graph with the same
tool (the browser itself) they use to explore the web.

**What's prior art.**
- **`chrome://*` pages** — many separate URLs, not a unified
  graph.
- **`about:` pages in Firefox** — same.
- **The "small web" / "indie web" idea** — the web as a graph
  of resources, not pages.
- **Brave's `brave://*` introspection** — Brave has invested in
  this, but not at the depth of full navigability.

**Build cost.** 3–4 engineer-months. Includes: the
`browser://about` graph model; the navigation chrome (address
bar works for browser URLs too); the per-resource pages
(process, permission, cached response); the cross-linking
("this permission is held by these 3 sites, which were created
on these dates, by these mechanisms").

**M-month target.** M30+ (when there's enough browser to be
worth introspecting).

**Open questions.**
- How deep does the graph go? Top-level only, or do we
  expose every V8 object?
- Does this make the browser attackable (a misconfigured
  `browser://*` page is a security hole)?
- Performance: rendering 47 tabs as a graph is expensive.
- Is the graph web-accessible (`browser://about` is local, but
  should there be a `web://about` that's public)?

**Depends on.** `spiral-ui` (the navigation chrome), all
existing crates (introspection requires the introspection
points to exist).

---

## Cross-cutting Notes

### Build sequencing

These eleven cluster into roughly four phases:

| Phase | Ideas |
|-------|-------|
| **Phase 2 (M4–9)** | None — too ambitious for the foundation phase. |
| **Phase 3 (M10–24)** | #6 Permission Budget, #8 Inter-Tab Bus, #10 Self-Cleaning Storage |
| **Phase 4 (M25–42)** | #1 Compute Credits, #2 URL Time-Travel, #3 Anti-Doom Scroll, #4 Real-Time Collab Tabs, #5 Form Memory, #7 DOM Tracer, #9 Visual Search, #11 Browseable Browser |
| **Phase 5 (M43–60+)** | None — all are Phase 3 or 4. |

**Cheapest of the eleven:** #8 Inter-Tab Bus (1–2 months) and #3
Anti-Doom Scroll (2–3 months).

**Most ambitious:** #4 Real-Time Collaborative Tabs (4–6 months)
and #9 Visual Search (4–6 months, plus on-device ML infrastructure).

### Combined with the first batch

Together, the first batch (10 ideas) and this batch (11 ideas)
make 21 novel ideas in the backlog. They cluster into four
themes:

- **User respect:** Anti-Doom Scroll, Self-Cleaning Storage,
  Permission Budget, Form Memory (with user control)
- **Tab/session intelligence:** Tab Provenance, Inter-Tab Bus,
  Sectional Reload, URL Time-Travel
- **Computation as currency:** Compute Credits, Self-Patching
  Bugs
- **Engineering quality:** Layout Streams, DOM Tracer,
  Reactive Extensions, Browseable Browser

### What the M4 audit implies for these

Same as the first batch: research the components, design the
combination, write Spiral-native code, attribute prior art
honestly. The novelty is in the combination, not the
components. The first batch was honest about this. So is this
one.

---

## SSOT Links

- [`docs/active_context.md`](active_context.md) — live sprint
  state
- [`docs/progress_ledger.md`](progress_ledger.md) — change log
- [`docs/audit-sprint-m4.md`](audit-sprint-m4.md) — M4
  originality audit (methodology applied here)
- [`docs/innovations-stubs.md`](innovations-stubs.md) — first
  batch of ten novel ideas
- [`docs/design-filter-rule-model.md`](design-filter-rule-model.md)
  — relates to #6 (form-action enforcement)
- [`docs/design-capability-types.md`](design-capability-types.md)
  — relates to #8 (typed bus)
- [`docs/design-vortex-heap.md`](design-vortex-heap.md) —
  relates to #7 (mutation interception)
- [`ROADMAP.md`](../ROADMAP.md) — phase plan
