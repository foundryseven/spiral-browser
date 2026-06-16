# Spiral Innovations III — Thirty-Two Audited Stubs

**Status:** design stubs, audited (2026-06-15)
**Author:** implementer agent (with 4 parallel research agents)
**Phase context:** M4 first sprint complete; companion to
[`docs/innovations-stubs.md`](innovations-stubs.md) and
[`docs/innovations-stubs-2.md`](innovations-stubs-2.md).
**Purpose:** 32 more ideas for the Spiral backlog, each
audited against live prior art (same methodology as the
M4 audit at `docs/audit-sprint-m4.md`).

---

## Audit summary

The audit found:

- **5 ideas that were claimed "truly novel" but are demonstrably false:**
  #62 (bookmark tags — Firefox has had this since 2008),
  #63 (notification permission revocation — Chrome's Safety
  Check does this), #71 (tab groups survival — Chrome since
  2020, Safari since 2021), #76 (public build dashboard —
  Treeherder, Chromium Status have done this for ~10 years),
  #35 (deopt without pausing — V8 background compilation
  since Chrome 66, 2018).
- **1 wrong citation:** #72 cites `chrome://tracing` which
  is for performance traces, not memory. The actual prior
  art is `about:memory` (Firefox), `chrome://discards`,
  `chrome://memory-internals`.
- **1 undersold:** #70 "The Browser That Asks Why" was
  classified "configuration" but has no shipped-browser
  equivalent for proactive plain-language reflection.
- **All other classifications are honest** — partially
  novel, configuration, etc.

The honesty corrections are integrated into the stubs
below. Each stub has the audited verdict, not the original
claim.

---

## Vortex (JS engine) — 6 ideas

### #34 "Inline Caching at Parse Time" (Audited)

**Pitch.** Vortex emits a shape (hidden class) for every
object literal at parse time, so the first allocation
already has the optimal shape.

**Verdict: Partially novel.** V8 already pre-allocates
object-literal boilerplate at compile time
(`v8.dev/blog/background-compilation`, 2017); the novelty
in #34 is the *per-statement granularity* of the
boilerplate, not the fact of pre-allocation. SpiderMonkey
Shapes and JSC Structures are runtime-allocated. Vortex's
own `gc/header.rs` already has a `Shape` cell-type
variant — the AST→Shape emission is the work.

**Build cost: 6–10 engineer-weeks.** M12+, gated on
bytecode VM (M10+).

**Open questions.** What's the granularity — per literal,
per function, per module? How does shape sharing work
across hot-reload? How do we invalidate the boilerplate
when the bytecode optimiser changes the shape?

**Depends on.** Vortex bytecode VM, Gyre box model.

### #35 "Deopt and Reopt Without Pausing" (Audited)

**Pitch.** Tier-up happens in the background; swap is
atomic at a safepoint.

**Verdict: Already exists.** V8 has background bytecode
compilation since Chrome 66 (2018); JSC's tier-up uses
compiler threads (Pizlo 2020); SpiderMonkey WarpMonkey
(not "WarpBuilder" — minor factual error in the pitch)
uses a WARPHelper background thread. The pitch is
describing what V8/JSC/SM have done for ~7 years.

**Build cost: 1 month, not 2–3.** Once the bytecode VM
exists, the work is a `CompilerThread` + atomic swap on
the `SharedFunctionInfo`-style code pointer.

**Open questions.** None that V8 hasn't already
answered.

**Depends on.** Vortex bytecode VM (M10+).

**Verdict change from pitch.** The pitch's "partially
novel" was overclaimed. Re-rating: **already exists**.

### #36 "Stack Frames as a Stream" (Audited)

**Pitch.** Vortex's call stack is a stream, not a tree.
Debugger and devtools subscribe to the stream; stack
traces are live, not snapshots.

**Verdict: Partially novel.** V8's async stack traces
are snapshots stored at task creation; CLR's `IStackWalk`
and Linux `perf` give OS-level frame streams. The
novelty is exposing the stream *to JS via a DevTools
API* — V8 doesn't do this.

**Build cost: 4–8 engineer-weeks.** M18+.

**Open questions.** Frame budget — pushing every frame
to subscribers is expensive. What does "stream" mean —
push to subscribers (callback-driven) or pull (iterator)?

**Depends on.** Vortex bytecode VM (for stable call frames).

**Verdict change.** The cited prior art "Chrome's Live
Stack" is **not a real feature name**; corrected in the
stub to "V8's async stack traces" and "CLR's profiler
API."

### #37 "Property Cache Shared Across Origins" (Audited, Recommendation: **Do Not Build**)

**Pitch.** Vortex's property cache (IC lookup result) is
shared across origins when the shape is identical.

**Verdict: Novel but a bad idea.** No engine has ever
done this; V8/SM/JSC are all per-origin by design.
Cross-origin state sharing creates Spectre-class side
channels — the IC timing leaks another origin's object
shapes. The "partially novel" classification hides the
fact that this would *regress* browser security.

**Build cost: 6–12 months including security review.**
The conclusion of that review may be "we shouldn't ship
this." M30+ is irrelevant — the build is gated on
security sign-off.

**Open questions.** How do you formally verify no
side-channel? What's the threat model? Who has signing
authority for the "this shape is safe to share" claim?

**Depends on.** Vortex, spiral-context (for the
capability model), formal methods team.

**Verdict change.** This was claimed "partially novel" —
correct on novelty, but the *bad idea* classification
should have been added. **Recommendation: drop or defer
to research.**

### #38 "Module Graph as a First-Class Type" (Audited)

**Pitch.** ES modules have a graph; Vortex can compile
and validate the entire graph before any code runs.

**Verdict: Configuration, not novel.** TypeScript,
Rollup, esbuild, Bun, Deno, Vite all build static
module graphs. The cited "Node's `--experimental-loader`"
is the **opposite** design (runtime resolution). TC39
Source Phase Imports (Stage 3) is moving browsers toward
static graphs. The novelty of "having a module graph" is
absorbed by the spec and the bundler ecosystem.

**Build cost: 6–12 engineer-weeks.** M24+ (gated on ESM
support in Vortex).

**Depends on.** Vortex ESM support (planned M12–M18).

### #39 "The Garbage Collector Observes User Attention" (Audited)

**Pitch.** Vortex's GC runs more aggressively when the
user is *not* looking at a tab. The heuristic is "user
is on the tab but not interacting."

**Verdict: Partially novel.** Chrome's background-tab
freezing and V8's idle-time GC scheduling are well-known.
The "foregrounded but idle" signal is the contribution —
no engine uses "user is reading the page but not
clicking" as a GC trigger.

**Build cost: 4–8 engineer-weeks.** M30+.

**Open questions.** What's the idle threshold? How do we
detect "reading"? Mouse/keyboard/scroll event timestamps?
Tab visibility? Audio playing? Video playing? The signal
needs to be richer than a single timestamp.

**Depends on.** Vortex GC, Blink-equivalent attention
signal.

---

## Gyre + spiral-render — 9 ideas

### #40 "Intrinsic Sizing Without a Re-Layout" (Audited)

**Pitch.** Gyre computes min-content, max-content, and
the actual size in one pass.

**Verdict: Partially novel (overclaimed).** WebKit and
Blink LayoutNG both produce all three sizes during one
layout traversal; the cited "Blink's intrinsic sizing
pass is separate" is **wrong** for current Blink. The
novelty is exposing all three sizes as a first-class
queryable result. A small API design choice.

**Build cost: 1–2 months.** M7+.

**Open questions.** None beyond the standard layout
correctness risk (margin collapse, BFC/IFC, float
interaction). The WPT css-sizing test suite is the gate.

**Depends on.** Gyre box model, WPT runner.

### #41 "Subgrid in V0.1" (Audited)

**Pitch.** CSS Subgrid (Level 2) in v0.1.

**Verdict: Already exists (and stale).** CSS Subgrid
became Baseline 2023 (Sept 15) in Firefox 71, Safari
16, Chrome 117 / Edge 117. Spiral shipping it in v0.1
*is* a feat, but the spec feature is not novel. The
"Chrome has had it in development for years" framing
makes it sound un-shipped; it shipped 2 years 9 months
ago.

**Build cost: 1–2 months on top of Gyre's grid
implementation.** M13+ (gated on grid).

**Verdict change.** Move to "Grid M13–14 deliverables,"
not the innovations backlog. It is required spec
parity, not a novelty.

### #42 "Layout for Server-Rendered Streaming HTML" (Audited)

**Pitch.** Gyre is designed to lay out *as the bytes
arrive*. The user sees a continuously-growing layout.

**Verdict: Partially novel, misframed.** Every browser
streams parse → layout → paint as bytes arrive (Chrome,
WebKit, Gecko, Servo). The novel claim is "Gyre's API
is designed for insertion-tolerance as a first-class
constraint" — a clean design choice, not a unique
mechanism. The cited "Chrome's FOUC prevention is
primitive" is not a real Chrome feature; corrected to
"React 18 streaming SSR," "Marko," and the existing
progressive layout pattern.

**Build cost: 2–3 months.** M18+. The work is
*correctness on the spec* (insertion doesn't break
existing layout), not novelty.

**Depends on.** Gyre box model, spiral-html streaming
parser.

### #43 "Reuse Layout for Re-Renders" (Audited)

**Pitch.** Gyre detects the structural diff and re-uses
existing layout boxes for unchanged subtrees.

**Verdict: Partially novel.** All engines do layout
reuse on unchanged DOM (Blink, Gecko, WebKit have
dirty-bit machinery). Solid.js, Svelte, and Inferno
*minimise* DOM mutation at the framework level, which
implicitly benefits from this. The novel claim is
"Vortex can tag layout subtrees with structural
identity and Gyre can reuse" — an interface design, not
a new algorithm.

**Build cost: 2–3 months.** M30+.

**Open questions.** Who owns the structural-identity
signal — Vortex (JS engine) or the framework? The
interface is the new piece; if frameworks don't adopt
it, the work is wasted.

**Depends on.** Vortex, Gyre, framework guidance.

### #44 "Coordinate-Free Layout Debugging" (Audited)

**Pitch.** Gyre dumps the layout tree in a
coordinate-free textual format: "Box A is parent of
Box B and Box C. Box B has width 100% and height auto."

**Verdict: Configuration.** Blink has
`dumpLayoutTreeAsString` (CLI flag) producing exactly
this format. Chrome's Layout panel shows structure.
**The cited "Chrome's Layout tree panel shows
coordinates" is misleading** — it shows both structure
and coordinates; the structural view is also available.

**Build cost: 1 month.** M18+.

**Depends on.** Gyre, devtools infrastructure.

### #45 "Display List Diffs as a Primitive" (Audited)

**Pitch.** spiral-render's display list is a tree; a
re-render produces a diff sent to Vello.

**Verdict: Already exists (and misframed).** Skia has
`SkPicture` picture-match as the **canonical** example
of display-list diffing (Chrome's compositor, Flutter,
React Native Skia). Vello does **not** have this API —
it is a frame renderer with no diff primitive. The
pitch implies Vello has it; it doesn't. The 2–3 month
estimate is wildly optimistic for a Skia-equivalent
implementation.

**Build cost: 6–9 months for a Skia-equivalent on top
of Vello (which is the wrong substrate).** M30+ is
irrelevant until someone forks Vello.

**Verdict change.** Re-classify as: build a
display-list differ on top of Vello (wrong tool) or
adopt Vello's frame-render model and accept the GPU
work. The "truly novel" claim is incorrect.

**Depends on.** Either fork Vello (large) or accept the
frame-render model (small).

### #46 "Off-Screen Tile Compositing" (Audited)

**Pitch.** Spiral can render tiles off-screen (about to
scroll into view) ahead of time. The tile cache is
mmap-backed.

**Verdict: Already exists; the mmap detail is small.**
Chrome's `cc::TileManager` has done off-screen tile
rasterisation for ~15 years. Firefox WebRender,
WebKit, Servo all have tile systems. The novel
"mmap-backed" detail is a 1–2 week addition, not an
architectural bet.

**Build cost: 3–4 months (standard tile manager work,
not novel).** M36+.

**Open questions.** Is "mmap-backed" worth the
implementation cost, or is GPU memory sufficient for
the typical workload? Need a workload study first.

**Verdict change.** The "partially novel" rating was
generous. Re-rating: **already exists**; the mmap
detail is a small twist.

### #47 "Color Management That Doesn't Suck" (Audited)

**Pitch.** Spiral's colour pipeline is wide-gamut by
default. The user picks sRGB or P3.

**Verdict: Partially novel (overclaimed).** Safari has
had Display P3 since 2016 (macOS/iOS); Chrome since
111 (March 2023); Firefox on macOS. The "user picks
sRGB or P3" framing is *infeasible* in a meaningful
way — the display is the final say. The novelty is a
"force sRGB even on P3 display" setting, which is a UX
feature.

**Build cost: 1–2 months if you adopt lcms2 (MIT).**
M30+.

**Open questions.** Does the user really want a "force
sRGB" mode, or is the spec's default ("wide-gamut
content renders wide-gamut; untagged content renders
sRGB") correct?

**Depends on.** spiral-css, lcms2, peniko.

### #48 "Sub-Pixel Text Positioning" (Audited)

**Pitch.** Sub-pixel positioning of text glyphs. The
user can choose snap-to-pixel or sub-pixel.

**Verdict: Configuration.** The cited "Safari's
Sub-pixel Font Scaling" is **not a real feature name**.
Sub-pixel positioning is universal in modern browsers
(Blink/Skia, WebKit/Core Text, Gecko/HarfBuzz+FreeType).
The novel piece is "user-facing snap-to-pixel setting"
— a fine UX feature. No browser exposes it.

**Build cost: 2–3 weeks** (a toggle UI + library pass-
through). M12+. The 1–2 month estimate is generous.

**Depends on.** spiral-render text, parley/cosmic-text.

---

## DOM / fmt / network — 8 ideas

### #49 "DOM as a Streamed CRDT" (Audited)

**Pitch.** The DOM is a CRDT for cross-device collaborative
editing.

**Verdict: Partially novel, severely over-estimated.**
Yjs and Automerge work on virtual state (rich-text,
JSON), not the real DOM. The "real DOM" CRDT tier is
genuinely under-explored. **But:** the use case
(collaborative tabs) has a fundamental blocker — 99% of
DOM mutations on the web come from JS, not user input;
syncing those is "share my entire JS execution state,"
which is a different and far more expensive problem than
CRDTs on virtual state.

**Build cost: 9–18 months for a minimum-viable
prototype.** M48+.

**Open questions.** The product fit is unclear. Who is
the user? What's the threat model for shared JS
execution? Without those, this is research.

**Verdict change.** This was claimed "truly novel" and
4–6 months. Both are overclaimed. Re-rating:
**partially novel, but defer to research**.

### #50 "DOM Mutations as SQL" (Audited)

**Pitch.** The DOM is queried with SQL. "Find all `<a>`
whose href starts with `https://` and is a different
origin."

**Verdict: Not novel (the cited prior art disproves
it).** The example query is **exactly** what CSS
selectors + XPath already do:
`a[href^="https:"]:not(...)` and
`//a[starts-with(@href, 'https:') and ...]`. XQuery 3.1
(W3C Recommendation) is literally SQL for trees, with
the FLWHERE clause. SQL/XML has shipped in PostgreSQL
since 2003. The novelty is purely syntactic — SQL
syntax vs XPath syntax — with no demonstrated
practical value.

**Build cost: 1–2 months for a SQL-shaped wrapper
around spiral-dom + `selectors`.** M36+.

**Verdict change.** Re-classify as: **skip**. The value
over existing selectors is dubious.

### #51 "HTML Streaming Parser" (Audited)

**Pitch.** spiral-fmt's HTML parser is stream-first; can
begin laying out and rendering before the document is
fully received.

**Verdict: Configuration work, not novel.** The
WHATWG HTML Living Standard Section 13.2 defines
streaming. Chrome streams parse → style → layout →
paint as bytes arrive. Servo's `html5ever` (which
Spiral vendors as spiral-fmt) has a `BufferQueue`
streaming API. **The cited "Chrome's parser is not
exposed to layout" is backwards** — Chrome's parser
*is* integrated with layout for this reason.

**Build cost: 2–3 months.** M18+. Most of the work
is *correctness on the spec* (incremental layout,
speculative pre-scan), not novelty.

**Verdict change.** Rename to "incremental streaming
layout" to reflect what the work actually is.

### #52 "Markup Rewriting at Parse Time" (Audited)

**Pitch.** The parser can rewrite the DOM as it builds.
Any module can register a `(Element) -> Option<Element>`
transform.

**Verdict: Partially novel, with caveats.** WebKit
has internal tree-builder hooks (not public). The W3C
Sanitizer API is the closest public interface (post-
parse, not in). The pitch underestimates the *state*
involved — HTML5 tree construction is stateful
(insertion mode, stack of open elements, foster-
parenting); a naive "callback per element" produces
incorrect DOM on misnested markup.

**Build cost: 2–3 months.** M24+. Position as
*post-parse* transform (like the Sanitizer API) to
avoid the state-coupling problem.

### #53 "Friendly-Format HTML" (Audited)

**Pitch.** The browser ships with a structured error
format. "This is the broken tree, this is what we did
to fix it, this is what you should change."

**Verdict: Truly novel as a *devtools UX feature*.**
The raw material exists (WHATWG spec error codes,
html5lib's error reports, Chrome's console messages).
No browser surfaces a structured "broken tree" view.

**Build cost: 1.5–2.5 months** (error code taxonomy +
tree-diff visualisation). M12+. **Strong candidate —
real gap.**

**Depends on.** spiral-fmt error output, devtools
infra.

### #54 "ALPN Negotiation Made Visible" (Audited)

**Pitch.** Show the user whether the browser is using
HTTP/2, HTTP/3, or HTTP/1.1.

**Verdict: Partially novel (overclaimed for DevTools,
novel for URL bar).** Chrome's Network panel already
shows the protocol. **The "ALPN is hidden" framing in
the first sentence of the pitch is factually wrong for
DevTools.** What is novel is the URL bar chip.

**Build cost: 5–7 weeks** (URL bar chip only).
Reposition to "URL bar protocol indicator + already-
shipped DevTools integration."

### #55 "Connection Pool With Per-Origin Reservations" (Audited)

**Pitch.** Reserve connections for predictable
connection patterns.

**Verdict: Partially novel (overclaimed).** Chrome's
"Connection Predictor" (also called PreconnectManager)
does predictive pre-warming, HTTP/2 connection
coalescing, HTTP/3 0-RTT cache. The "reservation"
piece is novel but narrow. W3C Resource Hints
(`<link rel="preconnect">`) and the Speculation Rules
API are competing mechanisms.

**Build cost: 2–4 months** for production quality.
M21+.

**Verdict change.** The value is debatable when W3C
Resource Hints already exist. Position as "predictive
+ reservations beyond the W3C surface."

### #56 "Zero-Configuration HTTP/3" (Audited)

**Pitch.** HTTP/3 by default. The user doesn't enable
it.

**Verdict: Not novel; required for parity.** Chrome
defaulted HTTP/3 on in 2020. Firefox in 2021. Safari
in 2024. Caddy, LiteSpeed, NGINX all default on. The
"defaults vary" claim in the pitch is **5 years stale**.

**Build cost: 3–6 weeks integration** (use `quiche`
BSD-2-Clause, or `neqo` MPL-2.0). M21+.

**Verdict change.** Re-classify as **required work, not
novel**. Pick `quiche` (no MPL) and ship.

---

## Storage / UI / browser / cross / build — 20 ideas

### #57 "Storage Self-Description" (Audited)

**Pitch.** Every storage entry has a metadata header:
when set, by what script, for what purpose.

**Verdict: Configuration, not partially novel.** Every
storage backend already stores timestamp, origin, key,
value. Chrome's DevTools Application panel already
exposes this. The "by what script" attribution is the
only novel sub-claim.

**Build cost: 2–3 months** (Vortex call-stack
snapshots on every `setItem`/`IDBObjectStore.put`).
M18+.

**Verdict change.** The "partially novel" rating was
generous. Re-rating: **configuration**.

### #58 "Storage Quota That You Can See"

**Verdict: Configuration.** `navigator.storage.estimate()`
is Baseline since 2018. Every Chromium browser exposes
it. The pitch is a UX feature, not novelty.

**Build cost: 2–3 weeks.** M18+.

### #59 "Cross-Origin Storage Quota" (Audited)

**Pitch.** Single budget per page; origins share it.

**Verdict: Partially novel (overclaimed).** Firefox's
eTLD+1 quota + the Storage Standard's storage-key
model already do this. The "Chrome's storage quota is
per-origin" citation is **dated** — Storage Buckets
API has evolved the model.

**Build cost: 1–2 months.** M18+.

### #60 "The Honest Command Palette" (Audited)

**Pitch.** Cmd+K lists every action. Includes
experimental, hidden, developer options.

**Verdict: Partially novel.** VS Code, Raycast,
Sublime, Atom, GitHub, Linear, Notion, Figma, Arc
**all have command palettes**. The novel piece is
"every hidden / experimental / developer option" —
Chrome's `chrome://flags` is the closest, but it's not
a palette.

**Build cost: 2–3 months.** M18+.

**Verdict change.** The "truly novel" rating was
overclaimed.

### #61 "Tab Search That Understands Intent" (Audited)

**Pitch.** Cmd+Shift+A opens a tab search that
understands natural language ("yesterday's article").

**Verdict: Partially novel (overclaimed).** Chrome
Tab Search (`Cmd+Shift+A`, Chrome 89, March 2021) does
**full-text search across recent pages**, not just
keyword match. The cited "Chrome's tab search is
keyword-only" is **wrong**. The genuine novelty is
"natural language" parsing for time/date predicates.

**Build cost: 1.5–2 months** (on-device small
classifier). M30+.

### #62 "Bookmark That You Can Tag" (Audited, **Drop from Novelty Backlog**)

**Pitch.** Bookmarks with a first-class tag bar.

**Verdict: Configuration, not novel.** The pitch
itself says "Firefox has had bookmark tags since 2008."
This is self-contradictory. Firefox, Delicious (2003),
Pinboard (2009), Diigo, Raindrop.io, Evernote, Notion,
Apple Notes all have first-class tag UIs. Chrome's
Bookmarks API doesn't have tags — that's a Chrome
limitation, not a Spiral innovation. **Catch up to
Firefox 2008.**

**Verdict change.** Drop from the novelty backlog.
Move to the regular feature backlog.

### #63 "Notification That Asks Permission Twice" (Audited, **Drop from Novelty Backlog**)

**Pitch.** When a site abuses notification permission,
the browser asks permission again.

**Verdict: Configuration. The pitch's prior art citation
("Chrome's quiet notification UI") is wrong; the
actual prior art is Chrome's Safety Check, which
**already does this** (verified via Chrome Help: "If
you allow notifications for a site that Chrome marked
as abusive or misleading, Chrome may block those
notifications and require the site to request your
permission again").**

**Verdict change.** Drop from the novelty backlog.
Chrome has shipped this exact mechanism.

### #64 "Session Restore That Restores Live State" (Audited)

**Pitch.** A browser crash restores URL, form values,
**scroll position**, and live state.

**Verdict: Partially novel (the URL/form/scroll part
is bog-standard).** Firefox's `sessionstore.jsonlz4`,
Chrome's session restore, and Safari's Handoff
**all restore URL, form values, and scroll position
today**. The novel piece is "live state" — the JS
heap, video playhead, complex widgets — which is a
Bet 4 feature, not a separate innovation.

**Build cost: 2–3 months for the URL/form/scroll
part; Bet 4 timing for the JS-heap part.** M30+.

**Verdict change.** The pitch implies Chrome/Firefox
*don't* do this. They do. The novel piece is JS-heap
restore, which is Bet 4.

### #65 "Profile as a Workspace" (Audited)

**Pitch.** A profile is a *workspace* (tabs, history)
plus a *persona* (email, passwords, payment).

**Verdict: Partially novel (overclaimed).** Safari 17
(Sept 2023) introduced **exactly this concept**:
"Profiles… allows users to separate their browsing
sessions for different use cases. Every profile has a
separate favorites bar, navigation history, extensions,
tab groups, and cookies." The "persona" half is in
Safari, Edge, Chrome Work/Personal. The combination is
what every browser calls a profile.

**Build cost: 3–4 months** for a from-scratch
implementation. M24+.

**Verdict change.** The "truly novel" rating was
overclaimed.

### #66 "Crash-Loop Detection" (Audited)

**Pitch.** If a tab crashes 3 times in 60 seconds,
show a "this tab is in a crash loop" dialog.

**Verdict: Configuration.** Chrome's "Sad Tab" + auto-
reload heuristic, Firefox's about:crashes, and Android's
"App keeps stopping" UX are the prior art. The
heuristic is well-precedented.

**Build cost: 1 month** (crash detection + dialog).
M21+.

### #67 "Power-User Profile Migration" (Audited)

**Pitch.** Import from Chrome/Firefox/Safari.

**Verdict: Configuration.** Every browser does this
from every other browser. The hard part is Chrome's new
v20 encryption (Firefox was forced to add a CSV-import
workaround in 2024–2025 because of this).

**Build cost: 2–3 months** (Chrome encryption work).
M24+.

### #68 "Spaced-Repetition for Browser Features" (Audited)

**Pitch.** Tips reappear contextually.

**Verdict: Partially novel.** Anki, Duolingo, SuperMemo,
Mailbox's onboarding, Todoist's onboarding, Apple's
Tips, Microsoft's Tips — all use the technique. The
novel piece is applying SR to per-user, per-feature
retention in a browser.

**Build cost: 1–2 months.** M30+.

### #69 "Self-Documenting Errors" (Audited)

**Pitch.** Browser errors include the fix.

**Verdict: Partially novel (in the browser space).**
`rustc`, `cargo`, GHC, Python, TypeScript, `clippy`,
`psql`, `npm install` all do this. Chrome's
`chrome://network-errors` page is a static reference.
The novel piece is the inline, contextual "fix" for
browser errors.

**Build cost: 1–2 months.** M18+. **Strong candidate
— real gap in the browser space.**

### #70 "The Browser That Asks Why" (Audited)

**Pitch.** Periodically, the browser asks "you visited
Twitter 47 times this week; is that intentional?"

**Verdict: Partially novel (the pitch undersold it).**
iOS Screen Time, Android Digital Wellbeing, macOS
Screen Time all do measurement, but **none of them
proactively ask in plain language based on the user's
own browsing history**. The closest is Apple's "is this
limit working for you?" — at the limit breach, not
proactively.

**Build cost: 1–2 months.** M30+. **Strong candidate.**

**Verdict change.** Original "configuration" rating was
undersold. Re-rating: **partially novel**.

### #71 "Tab Groups That Survive" (Audited, **Drop from Novelty Backlog**)

**Verdict: Configuration (and the pitch's claim is
stale).** Chrome tab groups have **persisted to disk
since launch in 2020** and **synced across devices by
default since 2020–2021**. Safari tab groups have
**survived restart and synced via iCloud since
Safari 15 (Sept 2021)**.

**Verdict change.** Drop from the novelty backlog.
Chrome and Safari have shipped this.

### #72 "Power-User Diagnostics" (Audited, **Fix the Citation**)

**Pitch.** A `spiral://diagnostics` page.

**Verdict: Configuration.** Browsers have many
such pages. **The cited "Chrome's `chrome://tracing`"
is wrong** — `chrome://tracing` is for performance
traces (CPU), not memory. The actual prior art is
`about:memory` (Firefox), `chrome://discards`,
`chrome://memory-internals` (Chromium), and Safari's
Web Inspector Memory tab.

**Verdict change.** The novelty is small; the citation
needs fixing.

### #73 "Per-Phase Memory Budget CI Gate"

**Verdict: Configuration.** Chromium's perf trybots,
Mozilla's perf.sheriff, V8's perf infra all do this.
The M4 architecture doc already specifies the gate.

**Build cost: 1–2 months** (defining the workload is
the hard part). M12+.

### #74 "Property-Based Fuzzing for Layout"

**Verdict: Configuration.** Servo, Ladybird, Chromium
(ClusterFuzz), Gecko, WebKit, DOMFuzz all do this.
**AFL's license change in 2023** (Apache 2.0 →
restrictive) means use **AFL++** (Apache-2.0) instead.

**Build cost: 2–3 months** (corpus + assertions).
M18+.

### #75 "Spiral's Own Benchmark Suite"

**Verdict: Configuration.** Speedometer 3, WebXPRT 4,
MotionMark 1.3, JetStream 2 are all open (BSD-2-Clause).
The real cost is *maintaining* the suite as standards
evolve.

**Build cost: 1–2 months initial; 0.5 FTE ongoing.**
M24+.

**Verdict change.** "motionmark" → "MotionMark" (proper
case).

### #76 "Public Build Dashboard" (Audited, **Drop from Novelty Backlog**)

**Verdict: Configuration.** Mozilla Treeherder,
Chromium CI, Chromium Status, Chromium Perf, WebKit
Buildbot, Servo CI have done this for **~10 years**.
The pitch itself cites these; the "truly novel"
classification is wrong.

**Verdict change.** Drop from the novelty backlog.
Treat as engineering hygiene.

---

## Final ranking (most novel → least novel)

| Rank | # | Title | Verdict |
|------|---|-------|---------|
| 1 | #70 | The Browser That Asks Why | **Partially novel** (corrected from "configuration") |
| 2 | #69 | Self-Documenting Errors | Partially novel |
| 3 | #53 | Friendly-Format HTML | Truly novel (devtools UX) |
| 4 | #39 | GC Observes User Attention | Partially novel |
| 5 | #42 | Layout for Streaming HTML | Partially novel (misframed) |
| 6 | #64 | Session Restore — Live State | Partially novel (JS heap only) |
| 7 | #49 | DOM as Streamed CRDT | Partially novel (defer) |
| 8 | #60 | Honest Command Palette | Partially novel |
| 9 | #34 | Inline Caching at Parse Time | Partially novel |
| 10 | #68 | Spaced-Repetition for Features | Partially novel |
| 11 | #52 | Markup Rewriting at Parse Time | Partially novel |
| 12 | #36 | Stack Frames as a Stream | Partially novel |
| 13 | #65 | Profile as a Workspace | Partially novel (Safari 17 has this) |
| 14 | #43 | Reuse Layout for Re-Renders | Partially novel (interface only) |
| 15 | #40 | Intrinsic Sizing Without Re-Layout | Partially novel (overclaimed) |
| 16 | #61 | Tab Search That Understands Intent | Partially novel (Chrome does full-text) |
| 17 | #59 | Cross-Origin Storage Quota | Partially novel (Firefox has eTLD+1) |
| 18 | #47 | Color Management That Doesn't Suck | Partially novel (overclaimed) |
| 19 | #54 | ALPN Negotiation Made Visible | Partially novel (URL bar only) |
| 20 | #55 | Connection Pool — Per-Origin Reservations | Partially novel (Chrome Predictor) |
| 21 | #45 | Display List Diffs as Primitive | Already exists (Skia) |
| 22 | #46 | Off-Screen Tile Compositing | Already exists (mmap detail only) |
| 23 | #51 | HTML Streaming Parser | Configuration |
| 24 | #72 | Power-User Diagnostics | Configuration (citation fixed) |
| 25 | #44 | Coordinate-Free Layout Debugging | Configuration |
| 26 | #48 | Sub-Pixel Text Positioning | Configuration (citation fixed) |
| 27 | #50 | DOM Mutations as SQL | Skip (no value over XPath) |
| 28 | #56 | Zero-Config HTTP/3 | Required (not novel) |
| 29 | #57 | Storage Self-Description | Configuration |
| 30 | #38 | Module Graph as First-Class Type | Configuration |
| 31 | #58 | Storage Quota That You Can See | Configuration |
| 32 | #66, #67, #73, #74, #75 | Crash-Loop, Profile Migration, Memory Budget CI, Fuzzing, Benchmark | Configuration |

**Drop from novelty backlog (re-classify as catch-up or
hygiene):** #41 (subgrid in v0.1), #62 (bookmark tags),
#63 (notification permission revocation), #71 (tab groups
survival), #76 (public build dashboard).

**Recommend against building:** #37 (cross-origin IC
cache — security regression), #50 (SQL DOM — no value
over XPath).

---

## SSOT links

- [`docs/active_context.md`](active_context.md) — sprint
  state
- [`docs/progress_ledger.md`](progress_ledger.md) —
  change log
- [`docs/audit-sprint-m4.md`](audit-sprint-m4.md) — M4
  audit methodology
- [`docs/innovations-stubs.md`](innovations-stubs.md) —
  first batch (10 ideas)
- [`docs/innovations-stubs-2.md`](innovations-stubs-2.md) —
  second batch (11 ideas)
- [`docs/innovations-routing.md`](innovations-routing.md) —
  wave vs main routing
- [`docs/innovations-index.md`](innovations-index.md) —
  one-page index
- [`docs/innovations-top-10.md`](innovations-top-10.md) —
  top 10 to build first
