# Spiral Innovations Backlog

**Status:** 2026-06-15, fully audited, consolidated
**Source:** all 5 brainstorm batches in [`docs/innovations-stubs-archive/`](innovations-stubs-archive/),
plus the 2026-06-15 baseline audit methodology at
[`docs/audits/2026-06-15-baseline.md`](audits/2026-06-15-baseline.md)
(the M4 audit
[`docs/audit-sprint-m4.md`](audit-sprint-m4.md) is the
novelty-claim research input to the baseline audit; the
baseline audit is the consolidated product).
**Replaces:** `innovations-index.md`, `innovations-routing.md`,
`innovations-top-10.md`, `innovations-quality-novelty-summary.md`
(consolidated 2026-06-15; the four originals are deleted).

**Total ideas: 70** (5 truly novel, 45 partially novel,
15 configuration, 4 dropped, 1 do-not-build, 1 skip,
plus 5 outside-the-box provocations).

---

## Quick legend

- **Novelty class:** TN = truly novel, PN = partially novel,
  C = configuration (sound engineering, not a uniqueness
  claim), S = skip / do not build, D = drop from backlog
  (catch-up work).
- **Routing:** W = wave development (touches the core
  engine per the user's criterion: `spiral-core`, `dom`,
  `network`, `ipc`, `vortex`, `gyre`, `paint`); M = main
  development.
- **M-month:** target phase (matches `ROADMAP.md`).
- **Cost:** engineer-months estimate.
- **Status:** ✓ built, ☆ strong candidate, ✗ dropped,
  ⚠ flag, — = backlog.

---

## Section 1 — All 70 ideas, sorted by build cost

The full backlog, cheapest first.

| # | Title | Routing | Cost | M | Novelty | Status |
|---|-------|---------|------|---|---------|--------|
| 58 | Storage Quota That You Can See | W | 2–3 wk | M18+ | C | — |
| 48 | Sub-Pixel Text Positioning | M | 2–3 wk | M12+ | C | — |
| 41 | Subgrid in V0.1 | W | 1–2 mo | M13+ | C | ✗ |
| 50 | DOM Mutations as SQL | M | 1–2 mo | M36+ | — | ✗ SKIP |
| 73 | Per-Phase Memory Budget CI Gate | M | 1–2 mo | M12+ | C | — |
| 66 | Crash-Loop Detection | W | 1 mo | M21+ | C | — |
| 51 | HTML Streaming Parser | W | 2–3 mo | M18+ | C | — |
| 67 | Power-User Profile Migration | M | 2–3 mo | M24+ | C | — |
| 60 | Honest Command Palette | M | 2–3 mo | M18+ | PN | — |
| 12 | Tab Provenance Graph | W | 1–2 mo | M12+ | PN | — |
| 20 | Cookie Affinity | W | 1 mo | M12+ | PN | — |
| 62 | Bookmark Tag Bar | M | 1–2 mo | M18+ | C | ✗ |
| 53 | Friendly-Format HTML | M | 1.5–2.5 mo | M12+ | TN | ☆ |
| 63 | Notification Permission Revoke | W | 1 mo | M24+ | C | ✗ |
| 71 | Tab Groups That Survive | M | 1–2 mo | M18+ | C | ✗ |
| 76 | Public Build Dashboard | M | 1–3 mo | M30+ | C | — |
| 47 | Color Management | W | 1–2 mo | M30+ | PN | — |
| 14 | Anti-Doom Scroll | M | 2–3 mo | M18+ | PN | — |
| 22 | The Browser That Asks Why | M | 1–2 mo | M30+ | PN | ☆ |
| 68 | Spaced-Repetition for Features | M | 1–2 mo | M30+ | PN | — |
| 16 | URL Time-Travel | M | 5–7 mo | M36+ | PN | — |
| 11 | Browseable Browser | M | 3–4 mo | M30+ | C | — |
| 17 | Visual Search | M | 6–24 mo | M42+ | TN | — |
| 65 | Profile as a Workspace | M | 3–4 mo | M24+ | PN | — |
| 44 | Coordinate-Free Layout Debug | W | 1 mo | M18+ | C | — |
| 74 | Property-Based Fuzzing for Layout | W | 2–3 mo | M18+ | C | — |
| 56 | Zero-Configuration HTTP/3 | W | 3–6 wk | M21+ | C | Required |
| 75 | Spiral's Benchmark Suite | M | 1–2 mo | M24+ | C | — |
| 72 | Power-User Diagnostics | M | 1–2 mo | M30+ | C | — |
| 32 | Performance Regression Tests | M | 2–3 mo | M36+ | PN | — |
| 57 | Storage Self-Description | W | 2–3 mo | M18+ | C | — |
| 34 | Inline Caching at Parse Time | W | 6–10 wk | M12+ | PN | — |
| 35 | Deopt and Reopt Without Pausing | W | 1 mo | M30+ | C (existing) | — |
| 36 | Stack Frames as a Stream | W | 4–8 wk | M18+ | PN | — |
| 59 | Cross-Origin Storage Quota | W | 1–2 mo | M18+ | PN | — |
| 38 | Module Graph as First-Class Type | W | 6–12 wk | M24+ | C | — |
| 39 | GC Observes User Attention | W | 4–8 wk | M30+ | PN | ☆ |
| 40 | Intrinsic Sizing Without Re-Layout | W | 1–2 mo | M7+ | PN | — |
| 61 | Tab Search That Understands Intent | M | 1.5–2 mo | M30+ | PN | — |
| 54 | ALPN Negotiation Made Visible | W | 5–7 wk | M12+ | PN | — |
| 55 | Connection Pool — Per-Origin Reservations | W | 2–4 mo | M21+ | PN | — |
| 6 | Permission Budget | W | 5–7 mo | M24+ | TN | — |
| 8 | Inter-Tab Messaging Bus | W | 1–2 mo | M18+ | PN | — |
| 42 | Layout for Streaming HTML | W | 2–3 mo | M18+ | PN | — |
| 30 | Screen Reader Latency | W | 1–2 mo | M30+ | PN | — |
| 64 | Session Restore — Live State | W | 2–3 mo | M30+ | PN | — |
| 31 | Time-Travel Devtools | W | 3–4 mo | M36+ | PN | — |
| 19 | Compute Credits | W | 4–6 mo | M30+ | TN | — |
| 15 | Form Memory | M | 6–9 mo | M30+ | PN | — |
| 7 | Layout Streams | W | 3–4 mo | M25+ | PN | — |
| 45 | Display List Diffs as Primitive | W | 6–9 mo | M30+ | C (existing) | ⚠ |
| 46 | Off-Screen Tile Compositing | W | 3–4 mo | M36+ | C (existing) | — |
| 9 | Reactive Extensions API | M | 3–4 mo | M42+ | PN | — |
| 28 | Real-Time Collaborative Tabs | M | 12–18 mo | M48+ | PN (cost corrected) | — |
| 1 | Wound Lattice | W | 4–6 mo | M30+ | **PN** (audited) | — |
| 4 | Type-Verified URL | W | 4–5 mo | M36+ | **PN** (audited) | — |
| 33 | Reader-Mode-That-Works | M | 1 mo | M24+ | C | — |
| 2 | Provenance Tracking | W | 3–4 mo | M18+ | **PN** (audited) | — |
| 3 | Sectional Reload | W | 2–3 mo | M36+ | **PN** (audited, citations fixed) | — |
| 5 | Sandbox Sandboxing | W ⚠ | 2–3 mo | M25+ | **PN** (audited) | ⚠ |
| 10 | Self-Patching Bugs | W | 5–7 mo | M48+ | **PN** (audited) | — |
| 37 | Property Cache Shared Across Origins | W ⚠ | 6–12 mo | M30+ | TN | ⚠ DO NOT BUILD |
| 49 | DOM as a Streamed CRDT | W | 9–18 mo | M48+ | PN | Deferred |
| 12 (B3) | Local-First Sync | M | TBD | TBD | PN | — |
| 6 (B3) | Privacy by Default | M | TBD | TBD | **PN** (audited) | — |
| 4 (B3) | Time as First-Class Dimension | M | TBD | TBD | **PN** (audited) | — |
| 9 (B3) | URL Health Check | M | TBD | TBD | PN | — |
| 7 (B3) | Tab Groups as Workspaces | M | TBD | TBD | PN | — |
| 1 (B3) | Browser as a Compiler | M | TBD | TBD | **PN** (audited) | — |
| 2 (B3) | Web as Single Address Space | M | TBD | TBD | **PN** (audited) | — |
| 8 (B3) | Notification Quiet Hours | M | TBD | TBD | C | — |
| 5 (B3) | Profile as Code | M | TBD | TBD | C | — |
| 10 (B3) | Sticky Tabs | M | TBD | TBD | C | — |
| 11 (B3) | Power User Settings | M | TBD | TBD | C | — |
| 3 (B3) | Crash Recovery Restores Layout | W | TBD | TBD | PN | — |
| 77 | Browser-as-a-Compiler (B5 provocation) | M | — | — | Provocation | — |
| 78 | Web as Single Address Space (B5) | M | — | — | Provocation | — |
| 79 | Time as First-Class Dimension (B5) | M | — | — | Provocation | — |
| 80 | The Anti-Browser (B5) | M | — | — | Provocation | — |
| 81 | The Browser That Forgets (B5) | M | — | — | Provocation | — |

---

## Section 2 — Routing summary (wave vs main)

**Criterion (user-chosen, 2026-06-15):**
**Wave** = touches one or more of `spiral-core`, `spiral-dom`,
`spiral-network`, `spiral-ipc`, `spiral-vortex`, `spiral-gyre`,
`spiral-paint`. **Main** = everything else.

| Routing | Count | Examples |
|---------|-------|----------|
| **wave** | 33 | #1, #2, #3, #4, #6, #7, #8, #10, #12, #19, #20, #30, #31, #34–#47, #49, #51, #54–#59, #63, #64, #66, #74 |
| **main** | 35 | #5 ⚠, #9, #11, #14, #15, #16, #17, #22, #27, #28, #29 ⚠, #32, #33, #48, #50 ⚠ SKIP, #52, #53, #60, #61, #62, #65, #67, #68, #69, #70, #71, #72, #73, #75, #76 |
| **drop** (catch-up) | 4 | #41 subgrid, #62 bookmark tags, #63 notification revoke, #71 tab groups |
| **do not build** | 1 | #37 cross-origin IC cache (security regression) |
| **skip** | 1 | #50 SQL DOM (no value over XPath) |

### Two flagged routing edge cases

- **#5 Sandbox Sandboxing** — strict letter = main
  (touches `spiral-context`, not in the seven wave crates).
  Spirit = wave (extends Bet 1's capability-typed system).
  **Reclassified to wave** for consistency with the intent.
- **#29 Per-User Font Override** — strict letter = main
  (touches `spiral-render`, not in the seven). Spirit = main.
  **Left as main**; the font override is a render-pipeline
  parameter, not a new rendering primitive.

### Wave-development sequencing rationale

Wave items are ordered by **engine dependency**, not by
user value or novelty:

1. **First the engine:** Vortex bytecode VM (#34, #35,
   #36, #38) lands M10–M24; Gyre box model and block
   layout (#40) lands M7–M9.
2. **Then the engine extensions:** Capabilities (#5),
   GC attention (#39), Layout streams (#7), Display-list
   diffs (#45), Subgrid (#41).
3. **Then the engine integration:** Networking (#4, #20,
   #54, #55, #56), DOM (#2, #3, #57, #58, #59), Storage
   (#15, #18), Layout (#42, #43, #46, #47).
4. **Then the long bets:** Wound Lattice (#1), CRDT DOM
   (#49), Self-Patching Bugs (#10), Persistent renderer
   extensions (#64).

Main items have no engine-blocking dependency; they can
be scheduled freely within their M-month windows.

### Wave items NOT to build

- **#37 Property Cache Shared Across Origins** — security
  regression. The pitch is in the backlog for tracking,
  but the routing reflects "where it would land" if it
  shipped. It should not ship.

---

## Section 3 — Top 10 to build first

Selected by: low cost, strong dependency, high user value,
no fatal flaw, wave when possible. This is a recommendation,
not a commitment.

### #1 — #22 The Browser That Asks Why (M30+)

**Why #1.** The audit found this is the most undersold
truly-novel idea in the batch. The pitch was "configuration"
but the audit corrected it to "partially novel" because
no shipped browser does the *proactive reflection* framing.
iOS Screen Time, Android Digital Wellbeing, and macOS
Screen Time all do *measurement* but none of them
proactively ask the user "is that intentional?" in plain
language based on the user's *own* browsing history.

- **Cost:** 1–2 mo.
- **Dependencies:** Spiral browser history, `spiral-ui`.
- **Risk:** None (UX feature, not security).
- **Wave?** No (main).
- **Unlocks:** #14 Anti-Doom Scroll, user trust in the
  brand.

### #2 — #53 Friendly-Format HTML (M12+)

**Why #2.** The audit confirmed this is the single most
genuinely novel *feature* in the entire backlog. The
material exists (WHATWG spec error codes, html5lib's error
reports) but no browser surfaces a structured "broken
tree" view.

- **Cost:** 1.5–2.5 mo.
- **Dependencies:** `spiral-fmt` error output, devtools
  infra.
- **Risk:** Low (devtools, not user-facing UX).
- **Wave?** No (main).
- **Unlocks:** A new devtools surface, useful for the
  maintainer during the long build.

### #3 — #39 GC Observes User Attention (M30+)

**Why #3.** Partially novel in a meaningful way. The
"foregrounded but idle" signal is a contribution. Every
browser freezes *background* tabs; no browser optimises
for the "reading" case. Small implementation, large user
value (long-form reading is common, stutter-free scrolling
is noticed).

- **Cost:** 4–8 wk.
- **Dependencies:** Vortex GC (which needs the per-origin
  arenas from the M4 heap rewrite — already done).
- **Risk:** Low (additive heuristic, can be disabled).
- **Wave?** Yes.
- **Unlocks:** Battery life, the "fast on a cold page"
  brand promise.

### #4 — #69 Self-Documenting Errors (M18+)

**Why #4.** Partially novel in the browser space. `rustc`,
`cargo`, GHC, and `psql` all do "error includes the fix" —
the technique is well-precedented. No browser does it for
the user's benefit.

- **Cost:** 1–2 mo.
- **Dependencies:** `spiral-ui` (error pages), network
  error codes.
- **Risk:** None.
- **Wave?** No (main).
- **Unlocks:** A "Spiral is friendly" brand moment.

### #5 — #12 Tab Provenance Graph (M12+)

**Why #5.** The cheapest wave idea (1–2 mo) that touches
the core engine (`spiral-dom`, `spiral-ipc`). The cross-tab
provenance is genuinely novel (no browser tracks it). The
"47 tabs — 12 mine, 31 JavaScript" stat is a strong brand
moment.

- **Cost:** 1–2 mo.
- **Dependencies:** `spiral-dom`, `spiral-ipc`.
- **Risk:** Low.
- **Wave?** Yes.
- **Unlocks:** A "Spiral respects your tabs" feature.

### #6 — #20 Cookie Affinity (M12+)

**Why #6.** Safari ITP and Chrome Privacy Sandbox both
deprecate third-party cookies differently. The *affinity*
framing (a cookie is only sent to the origin that set it,
by default, with explicit user override) is the cleanest
UX. Small cost, straightforward implementation.

- **Cost:** 1 mo.
- **Dependencies:** `spiral-network`, `spiral-dom`
  (cookie state).
- **Risk:** None (privacy improvement).
- **Wave?** Yes.
- **Unlocks:** The "private by default" brand promise.

### #7 — #60 The Honest Command Palette (M18+)

**Why #7.** The "honest" part (every hidden / experimental
/ developer option) is the novel piece. The "command
palette" is well-precedented (VS Code, Raycast, Arc).

- **Cost:** 2–3 mo.
- **Dependencies:** `spiral-ui`, every Spiral crate (the
  palette needs to know about every action).
- **Risk:** None.
- **Wave?** No (main).
- **Unlocks:** Discoverability, the "transparent" brand
  promise.

### #8 — #59 Cross-Origin Storage Quota (M18+)

**Why #8.** Firefox eTLD+1 model is the prior art; Spiral
can match it with a fresh, user-visible implementation.

- **Cost:** 1–2 mo.
- **Dependencies:** `spiral-dom`, `spiral-storage` (planned).
- **Risk:** None.
- **Wave?** Yes.
- **Unlocks:** The "Spiral respects your storage" brand
  promise.

### #9 — #42 Layout for Streaming HTML (M18+)

**Why #9.** The audit found the underlying behaviour
("page streams in") already happens, but Gyre's API being
*designed for insertion-tolerance* is a clean design choice
that pays off long-term.

- **Cost:** 2–3 mo.
- **Dependencies:** Gyre box model, `spiral-fmt` streaming
  parser.
- **Risk:** Low (correctness work, well-trodden territory).
- **Wave?** Yes.
- **Unlocks:** The "fast on the wire" brand promise.

### #10 — #64 Session Restore — Live State (M30+)

**Why #10.** The URL/form/scroll restore is bog-standard;
the novel piece is "live state" (JS heap, video playhead,
complex widgets). This is a Bet 4 feature (persistent
renderer). Expensive (Bet 4 timing) but the user value is
large. The "browser crashed and I lost my place" panic
is universal.

- **Cost:** 2–3 mo (URL/form/scroll); plus Bet 4 timing
  for the JS-heap part.
- **Dependencies:** `spiral-vortex`, `spiral-gyre`,
  `spiral-dom` (Bet 4).
- **Risk:** Medium (state-restoration is fragile).
- **Wave?** Yes.
- **Unlocks:** The "Spiral doesn't lose your work" brand
  promise.

### Top 10 sequencing (M-month order)

| M-month | # | Idea | Cost |
|---------|---|------|------|
| M12+ | #5 | Tab Provenance Graph | 1–2 mo |
| M12+ | #6 | Cookie Affinity | 1 mo |
| M12+ | #2 | Friendly-Format HTML | 1.5–2.5 mo |
| M18+ | #4 | Self-Documenting Errors | 1–2 mo |
| M18+ | #7 | The Honest Command Palette | 2–3 mo |
| M18+ | #8 | Cross-Origin Storage Quota | 1–2 mo |
| M18+ | #9 | Layout for Streaming HTML | 2–3 mo |
| M30+ | #1 | The Browser That Asks Why | 1–2 mo |
| M30+ | #3 | GC Observes User Attention | 4–8 wk |
| M30+ | #10 | Session Restore — Live State | 2–3 mo |

**Total:** ~14–20 engineer-months across M12+ to M30+.
1–2 engineer-years of focused work for the top 10.

### What the top 10 deliberately excludes

- Configuration-only items (HTML streaming parser without
  Gyre's integration, Coordinate-Free Layout Debugging,
  Sub-Pixel Text Positioning). Valuable but catch-up, not
  differentiation.
- Long bets (Wound Lattice, Real-Time Collaborative Tabs,
  DOM as a Streamed CRDT, Self-Patching Bugs). 4–18 mo
  bets, belong in later phases.
- Outside-the-box provocations (browser-as-a-compiler,
  anti-browser, etc.). Directions, not roadmap items.
- The 4 dropped items (subgrid, bookmark tags, notification
  revoke, tab groups). Catch-up work, not innovations;
  build, don't celebrate.
- The 1 do-not-build (#37, security regression).
- The 1 skip (#50, no value over XPath).

The top 10 is **the differentiation roadmap** — the ideas
that, when shipped, would make a user say "I can't go back
to Chrome." The other 60 are the infrastructure and
configuration that make the differentiation possible.

---

## Section 4 — Quality and novelty summary

### The honest bottom line

Of the 70 ideas in the backlog:

- **5 are truly novel** — no shipped browser does the
  combination. These are the genuine differentiators.
- **45 are partially novel** — components exist somewhere;
  Spiral's specific combination or application is new.
  This is where most of the "clever engineering" lives.
- **15 are configuration** — sound engineering choices
  that produce real wins but are not uniqueness claims.
  Every browser has analogues.
- **5 are provocations** (Batch 5) — deliberately
  unbuildable directions. Not on the build path.
- **4 are catch-up work** (drop from novelty) — bookmark
  tags, subgrid, notification revoke, tab groups.
- **1 should not be built** — cross-origin IC cache
  (security regression).
- **1 should be skipped** — SQL DOM queries (no value over
  XPath).

**Net verdict: 5/70 truly novel, 50/70 build-worthy,
15/70 hygiene.** The "truly novel" number is small; the
"build-worthy" number is large.

### Novelty distribution

| Class | Count | % |
|-------|-------|---|
| Truly novel | 5 | 7% |
| Partially novel | 45 | 64% |
| Configuration | 15 | 21% |
| Outside the box (B5) | 5 | 7% |
| Drop / skip / do-not-build | 6 | 9% (of 70) |

The "truly novel" is a small fraction. **This is the
honest picture.** The "novelty" that matters for Spiral
is mostly the *combination* — which is the partially
novel category.

### The 5 truly novel ideas (post-audit)

1. **#53 Friendly-Format HTML** (M12+, 1.5–2.5 mo) —
   devtools UX for malformed HTML.
2. **#19 Compute Credits** (M30+, 4–6 mo) — per-origin
   CPU/RAM budget with user control.
3. **#6 Permission Budget** (M24+, 5–7 mo) — permission
   costs that earn back over time.
4. **#9 Visual Search (B2)** (M42+, 6–24 mo) — box-select
   visual query.
5. **#17 Visual Search (B4)** (M42+, 4–6 mo) — partially
   overlaps with #9, distinct scope.

(Items 4 and 5 are both called "Visual Search" and are
partially overlapping. Treat them as a single category,
not two separate "truly novel" ideas. Net unique
truly-novel count: 4 distinct ideas.)

### The 45 partially novel — themes

| Theme | Ideas | Examples |
|-------|-------|----------|
| Vortex GC and engine work | 7 | #34, #36, #37 (DO NOT BUILD), #38, #39, #43, #64 |
| Gyre layout work | 6 | #40, #42, #44, #45, #46, #47 |
| Network and IPC | 5 | #1, #2, #4, #54, #55, #56 |
| Engine-internal quality | 4 | #10, #30, #31, #45 |
| DOM and rendering | 3 | #49, #51, #52 |
| Tab and session intelligence | 4 | #8, #12, #15, #16 |
| User-experience features | 8 | #14, #22, #60, #61, #68, #69, #70 |
| Performance and diagnostics | 4 | #7, #32, #72, #74 |
| Configuration and integration | 4 | #5, #11, #65, #67 |
| Controversial or expensive | 2 | #9 (Form Memory), #28 (Real-Time Collab Tabs) |

### Strong partially novel (the "near-certain" list)

| # | Title | Why strong |
|---|-------|------------|
| 39 | GC Observes User Attention | Foregrounded-but-idle signal is the contribution. 4–8 wk cost. |
| 22 | The Browser That Asks Why | Proactive plain-language reflection. The audit corrected this from "configuration" to "partially novel" because no shipped browser does the proactive-reflection framing. |
| 69 | Self-Documenting Errors | Error includes the fix. `rustc` does it; no browser does. |
| 12 | Tab Provenance Graph | Data exists in Chrome's `openerTabId`; user-facing graph is novel. 1–2 mo. |
| 20 | Cookie Affinity | Per-site cookie enforcement with explicit user override. 1 mo. |
| 60 | Honest Command Palette | "Every hidden option" coverage is the novel piece. 2–3 mo. |
| 59 | Cross-Origin Storage Quota | Single budget per page, shared across origins. 1–2 mo. |
| 42 | Layout for Streaming HTML | Gyre's API designed for insertion-tolerance. 2–3 mo. |
| 8 | Inter-Tab Messaging Bus | Typed bus. `BroadcastChannel` is untyped. 1–2 mo. |
| 64 | Session Restore — Live State | Live state (JS heap) restore. Bet 4 timing. |

### Weaker partially novel (the "configuration-leaning" list)

| # | Title | Why weaker |
|---|-------|------------|
| 5 | Sandbox Sandboxing | Capability OS design since 1965. |
| 1 | Wound Lattice | HTTP Signed Exchanges since 2019. |
| 2 | Provenance Tracking | Brave PageGraph since 2022. |
| 4 | Type-Verified URL | W3C Trusted Types since 2020. |
| 10 | Self-Patching Bugs | Chromium Component Updater; Linux kpatch. |
| 35 | Deopt Without Pausing | V8 background compilation since 2018. |
| 7 | Layout Streams | Blink's LayoutInvalidation + LayoutShift already. |
| 45 | Display List Diffs | Skia picture-match since forever. |
| 46 | Off-Screen Tile Compositing | Chrome `cc::TileManager` since 2010. |
| 56 | Zero-Config HTTP/3 | Required parity, not novel. |

### The 15 configuration ideas

Sound engineering that every browser does in some form.
The novelty is "Spiral does it deliberately, visibly, and
well." Not uniqueness claims — quality-of-life and
engineering-hygiene wins.

| # | Title | What the configuration is |
|---|-------|---------------------------|
| 48 | Sub-Pixel Text Positioning | Per-glyph snap toggle. |
| 58 | Storage Quota That You Can See | `navigator.storage.estimate()` UI. |
| 73 | Per-Phase Memory Budget CI Gate | CI gate with budget enforcement. |
| 67 | Power-User Profile Migration | Import from Chrome/FF/Safari. |
| 47 | Color Management | Wide-gamut default with user picker. |
| 11 | Browseable Browser | `browser://about` graph. |
| 65 | Profile as a Workspace | Workspace + persona split. |
| 44 | Coordinate-Free Layout Debug | Layout dump without coordinates. |
| 75 | Spiral's Benchmark Suite | Curated site set with budgets. |
| 72 | Power-User Diagnostics | `spiral://diagnostics`. |
| 57 | Storage Self-Description | Per-entry metadata. |
| 50 (SKIP) | DOM Mutations as SQL | No value over XPath. |
| 38 | Module Graph as First-Class Type | Static module graph (TS, Rollup already). |
| 56 | Zero-Config HTTP/3 | Required parity. |
| 66 | Crash-Loop Detection | Sad Tab + auto-reload heuristic. |

### The 4 dropped (catch-up work)

These are parity work, not innovations. Spiral should build
them as hygiene, but they should not be on the novelty
backlog.

| # | Title | Why dropped |
|---|-------|-------------|
| 41 | Subgrid in V0.1 | Firefox has it since 2019, Chrome since 2023. |
| 62 | Bookmark Tag Bar | Firefox has had it since 2008. |
| 63 | Notification Permission Revoke | Chrome Safety Check already does this. |
| 71 | Tab Groups That Survive | Chrome (2020), Safari (2021) both ship this. |

### The 1 do-not-build

| # | Title | Why |
|---|-------|-----|
| 37 | Property Cache Shared Across Origins | Spectre-class side channels; security regression. The novel piece is also bad. |

### The 1 skip

| # | Title | Why |
|---|-------|-----|
| 50 | DOM Mutations as SQL | XPath + CSS selectors already do this. SQL-shaped syntax is a wrapper, not an innovation. |

### The 5 outside-the-box provocations (B5)

Deliberately unbuildable directions. The methodology
doesn't apply; the doc is honest that these are provocations,
not buildable features.

| # | Title | What it's a provocation against |
|---|-------|----------------------------------|
| 77 | Browser-as-a-Compiler | The web as a *program* |
| 78 | Web as Single Address Space | The web as a *function* |
| 79 | Time as First-Class Dimension | The web as 4D spacetime |
| 80 | The Anti-Browser | The web as *curated* |
| 81 | The Browser That Forgets | The web as *ephemeral* |

All 5 have obvious reasons they might fail. All 5 are
deliberately unbuildable. They are *directions*, not
roadmap items.

### Quality assessment

Quality is a separate axis from novelty:

- **Buildable** — clear path from current state to ship.
- **Specifiable** — the idea can be reduced to a concrete
  API or data structure.
- **Testable** — WPT, Test262, or a custom test harness
  can verify it.
- **Aligned with Spiral's values** — the user's four
  values (independent, private by default, memory+speed,
  web-compliant + useful).

| Quality | Notes |
|---------|-------|
| **Buildable** | ~95% of the 70 ideas are buildable in the Spiral architecture. The 5% that aren't are Batch 5 + #37 (do-not-build). |
| **Specifiable** | ~85% can be reduced to a concrete spec. The 15% that can't (Batch 5 + a few vague B3 ideas) need a design pass first. |
| **Testable** | ~90% are testable via WPT, Test262, custom fuzz harness, or a custom test suite. The 10% that aren't testable are UX features (command palette, browser-as-why). |
| **Aligned with Spiral's values** | Most of the 70 align with at least one of the four values. The ones that conflict are the deliberate provocations (#80 anti-browser, #81 forgetting) and the strictly-utility work (#50 SQL DOM). |

### Top quality ideas (high on all four axes)

1. **#53 Friendly-Format HTML** — devtools UX, buildable
   in 1.5–2.5 mo, testable against malformed HTML
   fixtures, aligned with the "we sell nothing" value
   (transparency).
2. **#39 GC Observes User Attention** — engine work,
   buildable in 4–8 wk, testable via heap-size benchmarks,
   aligned with the "memory+speed" value.
3. **#20 Cookie Affinity** — network layer, buildable in
   1 mo, testable against cross-site cookies, aligned with
   the "private by default" value.
4. **#42 Layout for Streaming HTML** — engine work,
   buildable in 2–3 mo, testable against progressive
   layout, aligned with the "memory+speed" value.
5. **#22 The Browser That Asks Why** — UX, buildable in
   1–2 mo, testable against the user's own data, aligned
   with the "principled" value.
6. **#12 Tab Provenance Graph** — UX + IPC, buildable in
   1–2 mo, testable against the openerTabId API, aligned
   with the "transparent" value.
7. **#69 Self-Documenting Errors** — UX, buildable in
   1–2 mo, testable against error scenarios, aligned with
   the "principled" value.

### Bottom quality ideas (low on at least one axis)

- **#50 SQL DOM** — low alignment, low novelty, low value.
  Skip.
- **#37 Cross-Origin IC Cache** — novel but bad. Do-not-build.
- **#60 Honest Command Palette** — partially novel, but
  the "every hidden option" piece is engineering-heavy
  and the user-visible benefit is unclear.
- **#41 Subgrid in V0.1** — drop (Firefox has it). This is
  a *quality* issue: shipping a stale feature with a
  "novelty" badge is poor quality, even if the feature is
  well-implemented.

### Alignment with the four user-stated values

| Value | Best-aligned ideas |
|-------|---------------------|
| **Independent / principled** | #53 Friendly-Format HTML, #22 The Browser That Asks Why, #6 Permission Budget, #69 Self-Documenting Errors, all Batch 5 (provocations) |
| **Private by default** | #20 Cookie Affinity, #59 Cross-Origin Storage Quota, #63 (notification revoke, drop) |
| **Memory + speed** | #39 GC Observes User Attention, #42 Layout for Streaming HTML, #45 Display List Diffs, #46 Off-Screen Tile Compositing, #19 Compute Credits |
| **Web-compliant + useful** | #2 Provenance Tracking, #3 Sectional Reload, #9 Visual Search, #17 Visual Search, #28 Real-Time Collab Tabs |

---

## Section 5 — Sprint and overall-dev sequencing

This section is the bridge from backlog to roadmap. It
shows which items land in which phase.

### M4 (current sprint — design pass complete, build in progress)

**M4.4–M4.6 (in progress):** the foundational vendor and
core-engine work. No backlog items land in M4 — the
backlog is M5+ fuel. The four bets (shared-everything
multi-process, JIT-optional Vortex, `spiral-filter` parse-
time policy, persistent renderer) define the architecture
the backlog items will plug into.

| M4 task | Source |
|---------|--------|
| Vendor `html5ever` into `spiral-fmt`; modernise deps | ROADMAP M4 |
| Vendor `cssparser` + `selectors` into `spiral-fmt` | ROADMAP M4 |
| Unified facade: `spiral_fmt::parse_html/css()` | ROADMAP M4 |
| `spiral_net::Resolver` trait wrapping hickory-dns | ROADMAP M4 |
| Gyre block layout — first pass (no Taffy in tree) | ROADMAP M4 |
| Vortex spike — `rusty_v8` hello world, isolate lifecycle | ROADMAP M4 |

### M5+ — first backlog items land

The earliest backlog items all gate on engine primitives
that land M7–M12. They are sequenced by M-month target.

| M-month | Wave items (core engine) | Main items (UX/feature) |
|---------|--------------------------|--------------------------|
| M7+ | #40 Intrinsic Sizing Without Re-Layout (1–2 mo) | — |
| M12+ | #12 Tab Provenance Graph (1–2 mo), #20 Cookie Affinity (1 mo), #34 Inline Caching at Parse Time (6–10 wk), #54 ALPN Negotiation Made Visible (5–7 wk) | #2 Friendly-Format HTML (1.5–2.5 mo), #48 Sub-Pixel Text Positioning (2–3 wk), #73 Per-Phase Memory Budget CI Gate (1–2 mo) |
| M13+ | ~~#41 Subgrid in V0.1~~ (drop, catch-up) | — |
| M18+ | #8 Inter-Tab Messaging Bus (1–2 mo), #36 Stack Frames as a Stream (4–8 wk), #42 Layout for Streaming HTML (2–3 mo), #44 Coordinate-Free Layout Debug (1 mo), #51 HTML Streaming Parser (2–3 mo), #57 Storage Self-Description (2–3 mo), #58 Storage Quota That You Can See (2–3 wk), #59 Cross-Origin Storage Quota (1–2 mo), #74 Property-Based Fuzzing for Layout (2–3 mo) | #14 Anti-Doom Scroll (2–3 mo), #60 Honest Command Palette (2–3 mo) |
| M21+ | #55 Connection Pool — Per-Origin Reservations (2–4 mo), #56 Zero-Config HTTP/3 (3–6 wk, required), #66 Crash-Loop Detection (1 mo) | — |
| M24+ | #6 Permission Budget (5–7 mo), #38 Module Graph as First-Class Type (6–12 wk), #63 ~~Notification Permission Revoke~~ (drop) | #33 Reader-Mode-That-Works (1 mo), #65 Profile as a Workspace (3–4 mo), #67 Power-User Profile Migration (2–3 mo), #75 Spiral's Benchmark Suite (1–2 mo) |
| M25+ | #5 Sandbox Sandboxing (2–3 mo, Bet 1), #7 Layout Streams (3–4 mo) | — |
| M30+ | #19 Compute Credits (4–6 mo), #35 Deopt and Reopt Without Pausing (1 mo), #39 GC Observes User Attention (4–8 wk), #43 Reuse Layout for Re-Renders, #45 Display List Diffs as Primitive, #47 Color Management (1–2 mo), #64 Session Restore — Live State (2–3 mo), #1 Wound Lattice (4–6 mo) | #11 Browseable Browser (3–4 mo), #22 The Browser That Asks Why (1–2 mo), #61 Tab Search That Understands Intent (1.5–2 mo), #68 Spaced-Repetition for Features (1–2 mo), #72 Power-User Diagnostics (1–2 mo), #76 Public Build Dashboard (1–3 mo) |
| M36+ | #3 Sectional Reload (2–3 mo), #4 Type-Verified URL (4–5 mo), #46 Off-Screen Tile Compositing (3–4 mo), #31 Time-Travel Devtools (3–4 mo), #32 Performance Regression Tests | #16 URL Time-Travel (5–7 mo), #50 ~~DOM Mutations as SQL~~ (skip) |
| M42+ | #49 DOM as a Streamed CRDT (9–18 mo, deferred) | #9 Reactive Extensions API (3–4 mo), #17 Visual Search (6–24 mo) |
| M48+ | #10 Self-Patching Bugs (5–7 mo) | #28 Real-Time Collaborative Tabs (12–18 mo) |

### Sprint-by-sprint bridge to ROADMAP

The 12-week sprint plan in
[`docs/plans/iteration-options.md`](plans/iteration-options.md)
is the tactical view. It pulls from this backlog and
`ROADMAP.md`. The 70 backlog items don't appear in the
sprint plan until M5+, when the engine primitives (Gyre
block layout, Vortex tree-walker) are stable.

**Practical implication:** the M4 sprint is unchanged. The
backlog becomes load-bearing from M5 onwards. M5's first
backlog-flavoured tasks will likely be the cheap wave wins
(#54 ALPN Made Visible, #48 Sub-Pixel Text, #73 Memory
Budget CI Gate) which can ship without the engine being
fully complete.

### Differentiation roadmap vs infrastructure roadmap

| Track | Items | Cost | When |
|-------|-------|------|------|
| **Differentiation** (the top 10) | 10 ideas | ~14–20 eng-mo | M12+ to M30+ |
| **Configuration / hygiene** | 15 ideas | 12–18 eng-mo | M12+ to M30+, mostly 1 engineer |
| **Long bets** (post-top-10) | ~5 ideas (Wound Lattice, Real-Time Collab, CRDT DOM, Self-Patching, Compute Credits) | 4–18 mo each | M30+ to M48+ |
| **Provocations** (Batch 5) | 5 ideas | N/A | Side projects, not main path |

The 1–2 engineer-years of focused work for the top 10
fits the 6–8 year project timeline. The configuration
work happens in parallel. The long bets land after the
top 10.

---

## Section 6 — Cross-references

- [`docs/active_context.md`](active_context.md) — sprint
  state (single source of truth for in-flight work)
- [`docs/progress_ledger.md`](progress_ledger.md) — change
  log
- [`docs/audit-sprint-m4.md`](audit-sprint-m4.md) — M4
  audit methodology
- [`docs/architecture/design/shared-everything.md`](../architecture/design/shared-everything.md)
  — Bet 1 full writeup
- [`docs/architecture/design/filter-rule-model.md`](../architecture/design/filter-rule-model.md)
  — `spiral-filter` rule AST and parser approach
- [`docs/architecture/design/capability-types.md`](../architecture/design/capability-types.md)
  — branded lifetimes and capability tokens
- [`docs/architecture/design/vortex-heap.md`](../architecture/design/vortex-heap.md) —
  per-origin arenas and `TaggedCell` header
- [`docs/plans/iteration-options.md`](plans/iteration-options.md)
  — 12-week tactical plan (pulls from this backlog)
- [`../ROADMAP.md`](../ROADMAP.md) — phase plan (months
  and gates)
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md) — canonical
  architecture
- [`../CODEX.md`](../CODEX.md) — project overview
- [`docs/innovations-stubs-archive/`](innovations-stubs-archive/)
  — raw brainstorm inputs (5 batches, original and audited)

### Archive contents (raw brainstorm inputs)

- `batch-1-original.md` — first 10 ideas
- `batch-1-audited.md` — first 10 with audit corrections
- `batch-2-original.md` — next 11 ideas
- `batch-2-audited.md` — next 11 with audit corrections
- `batch-3.md` — 12 ideas
- `batch-4.md` — 32 ideas (audited)
- `batch-5-provocations.md` — 5 outside-the-box provocations

The 4 batch files (1, 2, 3, 4) plus the 2 audited
companions (1, 2) plus the provocations (5) total 7
files, archived here. The audited corrections are folded
into this backlog; the archive is for traceability.
