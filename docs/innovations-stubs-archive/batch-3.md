# Spiral Innovations — Twelve Stubs (Batch 3, audited)

**Status:** design stubs, audited (2026-06-15)
**Author:** implementer agent (with 3 parallel research agents)
**Phase context:** M4 first sprint complete
**Purpose:** the 12 ideas in Batch 3. This batch was
proposed but not yet written in earlier sessions. It is
written now with M4-audit corrections integrated from
the start.

---

## Audit summary

The Batch 3 audit found:

- **0 ideas are "truly novel"** in the strong sense.
  Every "truly novel" claim was at least partially
  addressed by existing prior art.
- **4 of 12 "truly novel" claims are overclaimed** and
  should be "partially novel":
  - #1 The Browser as a Compiler — partial overlap with
    Wasm Component Model + JIT compilers
  - #2 The Web as a Single Address Space — partial overlap
    with the OS process model
  - #4 Time as a First-Class Dimension — HSTS preload,
    subresource integrity, signed exchanges
  - #6 Privacy by Default — Safari ITP, Brave Shields,
    Tor Browser
- **5 of 12 are "partially novel"** with mostly-honest
  prior art citations.
- **3 of 12 are "configuration"** with prior art I
  correctly cited.
- **License risk: clean across the board.** No
  proprietary technology required.

---

## #1 — "The Browser as a Compiler" (Audited)

**Pitch.** The browser compiles the web. HTML, CSS, JSON,
SVG, all of it. A site is a *program*; the browser's job
is to *run* that program. The compiler is *always on*.

**Audit corrections.**

- **Verdict: Partially novel (not "truly novel").** The
  *concept* of a browser compiling the web predates the
  pitch by 30+ years: JavaScript is JIT-compiled since
  1996 (SpiderMonkey), HTML is parsed and compiled to a
  tree (every browser), CSS is compiled to a rule tree
  (every browser).
- **The "always on" framing is misleading.** Every
  modern browser *is* always-on compilation. The novelty
  claim is for *binary output* (the Wound Lattice from
  Batch 1) — but the pitch is *not* about that. The
  pitch is about the existing web.
- **Wasm Component Model** is the closest prior art for
  the "site as a compilable program" framing.

**Open questions.** What's the *output* format?
Compatibility with existing sites?

**Verdict change.** Truly novel → **partially novel**.

**Depends on.** Vortex (the JIT), spiral-css (the
compiler), spiral-fmt (the parser).

---

## #2 — "The Web as a Single Address Space" (Audited)

**Pitch.** Every URL is a *function* in a global namespace.

**Audit corrections.**

- **Verdict: Partially novel (not "truly novel").** The
  "address bar as function call" framing is the Unix
  process model. Every OS has had this since Multics
  (1965).
- **Web Bundles** (W3C/IETF) are the closest web-platform
  prior art: multiple resources addressed as one.
- **The cross-origin function call problem** is the
  same-origin policy. No browser has crossed it for
  security reasons.
- The web is not a single address space by design; it's
  a graph of address spaces (one per origin).

**Open questions.** Type system. Cross-origin
authorisation.

**Verdict change.** Truly novel → **partially novel**.

---

## #3 — "Crash Recovery That Restores Layout" (Audited)

**Pitch.** On crash, restore the layout tree from
persistent storage.

**Audit corrections.**

- **Verdict: Partially novel.** The "restore on crash"
  piece is bog-standard (every browser's session restore
  does this). The "layout tree, not just URL" piece is
  novel — this is the Bet 4 work (persistent renderer).
- **The novel piece is the *layout tree* checkpoint,**
  not the URL-list checkpoint. This is correctly
  classified as partially novel in the existing M4 work.

**Open questions.** Storage cost. Versioning. Hot
vs cold restore.

**Verdict change.** No change.

---

## #4 — "Time as a First-Class Dimension" (Audited)

**Pitch.** URLs have a time coordinate; the URL bar has a
slider.

**Audit corrections.**

- **Verdict: Partially novel (not "truly novel").** HSTS
  preload (`Strict-Transport-Security` with `preload`
  directive) is time-aware. Subresource Integrity
  (`integrity="..."`) is a content-hash. Signed HTTP
  Exchanges (SXG, see Batch 1 #1) include a `validity-url`
  and `date` field. The web already has time-aware
  primitives.
- **The "URL bar has a slider" framing is novel.** The
  underlying mechanism (snapshot at time T) is not.

**Open questions.** Who serves the historical snapshots?
Storage cost.

**Verdict change.** Truly novel → **partially novel**.

---

## #5 — "Profile as Code" (Audited)

**Pitch.** A profile is a structured file; users can share,
diff, version-control profiles.

**Audit corrections.**

- **Verdict: Configuration.** Chrome's profile is a
  directory; Firefox's `profiles.ini` is text. Both are
  diff-able. NixOS's home-manager treats user
  configuration as code. The pitch is making a
  configuration choice.
- **No novelty.** The "profile as code" idea has been
  used in dotfiles communities for a decade.

**Open questions.** Schema format. Migration.

**Verdict change.** No change.

---

## #6 — "Privacy by Default" (Audited)

**Pitch.** A browser where every privacy decision is the
default-on, conservative choice.

**Audit corrections.**

- **Verdict: Partially novel (not "truly novel").** Safari
  ITP (2017+), Brave Shields (2019+), Tor Browser
  (2008+), DuckDuckGo Privacy Essentials (2018+)
  already ship "privacy by default" configurations.
- **The novel piece is the *combination of all privacy
  decisions on, plus a user-visible explanation of each
  decision*.** Safari ITP makes decisions invisibly. Tor
  makes decisions transparently. The pitch's combination
  is partially novel.

**Open questions.** What's the user-visible explanation?
How granular?

**Verdict change.** Truly novel → **partially novel**.

---

## #7 — "Tab Groups That Are Workspaces" (Audited)

**Pitch.** A tab group is a workspace with its own set of
URLs, history, cookies, and storage; survives across
restart.

**Audit corrections.**

- **Verdict: Partially novel.** Chrome tab groups survive
  across restart (since 2020); Safari tab groups sync
  across devices (since 2021). The "cookies and storage
  per group" piece is novel. The "history per group" is
  less novel.

**Open questions.** Storage cost. Sync model.

**Verdict change.** No change.

---

## #8 — "Notification Quiet Hours" (Audited)

**Pitch.** A user-configurable quiet hours; no
notifications during sleep / work / meals.

**Audit corrections.**

- **Verdict: Configuration.** macOS Focus Modes (since
  macOS Monterey, 2021), iOS Focus Mode, Android Focus
  Mode, Windows Focus Assist (since Windows 10 1903,
  2019) all ship this. The browser-level integration is
  the Spiral-specific bit, but the concept is prior art.

**Open questions.** Per-site exceptions. Calendar
integration.

**Verdict change.** No change.

---

## #9 — "URL Health Check" (Audited)

**Pitch.** The browser checks URLs against reputation
databases; warns the user before navigating to risky
sites.

**Audit corrections.**

- **Verdict: Partially novel.** Chrome's "Dangerous site
  ahead" warning (since 2014, using Google's Safe
  Browsing API), Firefox's similar feature, Edge's
  SmartScreen, Safari's "Fraudulent Website Warning"
  (since 2015) are all prior art.
- **The novel piece is the *user-configurable whitelist
  of trusted domains*, with cryptographic verification
  of the reputation data.** Current Safe Browsing is a
  blacklist with no user control over trust.

**Open questions.** Privacy implications of reputation
lookup. User trust model.

**Verdict change.** No change.

---

## #10 — "Sticky Tabs Across Sessions" (Audited)

**Pitch.** When the user closes the browser, a snapshot
of the tab group persists; reopening restores everything.

**Audit corrections.**

- **Verdict: Configuration.** Chrome's "Continue where
  you left off" (since 2017), Firefox's session restore
  (since 2002), Safari's "Reopen All Windows from Last
  Session" (since 2010) all do this. The "tab group
  snapshot" is a small extension; not a novelty.

**Verdict change.** No change.

---

## #11 — "Power User Settings Surface" (Audited)

**Pitch.** A single page that exposes every setting, every
flag, every preference, with a fuzzy search.

**Audit corrections.**

- **Verdict: Configuration.** `chrome://flags` (since
  2008), `about:config` (since Firefox 1.0, 2004), Edge's
  `edge://flags`, Brave's `brave://flags` are all
  "power user settings surfaces." The "single page with
  fuzzy search" is a UX feature; not a novelty.

**Open questions.** Per-setting risk levels
("experimental" vs "stable" vs "dangerous").

**Verdict change.** No change.

---

## #12 — "Local-First Sync" (Audited)

**Pitch.** All user data lives locally first; sync is a
backup, not a primary store.

**Audit corrections.**

- **Verdict: Partially novel.** Local-first software is a
  movement (Ink & Switch, Martin Kleppmann et al.) since
  ~2019. The pitch applies it to browser state.
- **Firefox Sync** is "cloud-first, but the local copy
  works offline." **Local-first** is the inverse
  emphasis.
- **The novel piece is the *guaranteed local-first* with
  sync as backup, plus end-to-end encryption of the sync
  payload.** The guarantees and the encryption are the
  novel parts.

**Open questions.** Conflict resolution. Sync model
(CRDT, vector clock, manual).

**Verdict change.** No change.

---

## Final ranking

| Rank | # | Title | Verdict |
|------|---|-------|---------|
| 1 | #12 | Local-First Sync | Partially novel |
| 2 | #7 | Tab Groups as Workspaces | Partially novel |
| 3 | #3 | Crash Recovery Restores Layout | Partially novel |
| 4 | #6 | Privacy by Default | **Partially novel** (corrected from "truly novel") |
| 5 | #4 | Time as a First-Class Dimension | **Partially novel** (corrected from "truly novel") |
| 6 | #9 | URL Health Check | Partially novel |
| 7 | #1 | Browser as a Compiler | **Partially novel** (corrected from "truly novel") |
| 8 | #2 | Web as Single Address Space | **Partially novel** (corrected from "truly novel") |
| 9 | #8 | Notification Quiet Hours | Configuration |
| 10 | #5 | Profile as Code | Configuration |
| 11 | #10 | Sticky Tabs | Configuration |
| 12 | #11 | Power User Settings | Configuration |

**Net result:** Batch 3 has 0 truly novel, 8 partially
novel, 4 configuration.

---

## Cross-references

- [`innovations-stubs-1-audited.md`](innovations-stubs-1-audited.md) —
  Batch 1 with audit corrections
- [`innovations-stubs-2-audited.md`](innovations-stubs-2-audited.md) —
  Batch 2 with audit corrections
- [`innovations-stubs-4.md`](innovations-stubs-4.md) —
  Batch 4 (audited)
- [`innovations-stubs-5.md`](innovations-stubs-5.md) —
  Batch 5 (outside the box)
- [`audit-sprint-m4.md`](audit-sprint-m4.md) — M4 audit
  methodology
