# Chunk 0 — Methodology, Source Ladder, and Scoring Rules

**Chunk 0 of 14.** This file is the contract every later chunk binds to.
**Read it first, then read `citations/sources.md`.** If a research
chunk's row or claim does not satisfy the rules here, it does not ship.

---

## 1. The five-tier source ladder

Every factual claim in this research subset must be backed by a source
from at least one of these tiers. Tier matters: a Tier-1 claim is
stronger than a Tier-5 claim, and the synthesis in chunk 13 weights
accordingly.

| Tier | Examples | Strength | Use for |
|------|----------|----------|---------|
| **1** | WHATWG specs (HTML, DOM, Fetch, URL, Infra, Encoding, Storage, Streams, Permissions, Infra); W3C Recommendations (CSS, ARIA, SVG, WebRTC, EME, WebAuthn, Payment Request, WebGPU, WebTransport); IETF RFCs; ECMA-262/402/404; Unicode (UAX, UTS); WPT; WAI | Highest | "This is the standard behaviour." |
| **2** | MDN ("Baseline" / "Limited availability" / browser support tables), Can I Use, web.dev, W3C Working Drafts with broad implementation | High | "This is widely deployed; here's the support matrix." |
| **3** | Engine release notes (chromium.googlesource.com, developer.mozilla.org/en-US/blog/, webkit.org/blog/, hacks.mozilla.org), platform vendor docs (Apple Developer, Microsoft Learn, developer.android.com), Mozilla platform docs, Chrome Platform Status, WebKit feature status | High | "This is what engine X ships in version Y." |
| **4** | Third-party audits (`privacytests.org`, EFF Cover Your Tracks, GDPR cookie consent audits, accessibility audits), regulator guidance (ICO, CNIL, FTC, EU DSA, EU CRA) | High | "This is what users/regulators actually measure." |
| **5** | Independent benchmarks, blog posts, conference talks (TPAC, BlinkOn, JSCamp, GopherCon, LF/LPC), news reporting | Medium | "This is what the community is saying." Treated as supporting evidence only. |

**Rule:** A claim at Tier 5 cannot be the sole evidence for any priority
recommendation. It must be re-anchored to Tier 1–4 before chunk 13.

---

## 2. Six reference engines

The competitive matrix (chunk 12) is scored across six engines. This is
deliberately wider than the M4 audit's seven (which included Brave) —
Brave is a Chromium fork, not a distinct engine, and is tracked under
"Chromium" with a fork-specific note where relevant.

| Engine | Vendor | What it ships | Why in the matrix |
|--------|--------|---------------|-------------------|
| **Chromium** | Google (open source) | Blink + V8 + Skia + ANGLE + V8's V8 | The de facto baseline; ~70% browser share. The "must be at parity" floor. |
| **Gecko (Firefox)** | Mozilla | SpiderMonkey + Gecko + Servo-derived CSS engine | Independent engine, divergent on privacy/storage partitioning. Strong on standards. |
| **WebKit (Safari)** | Apple | JavaScriptCore + WebCore + CoreGraphics | Independent engine, divergent on PWA, EME, certain APIs. Default iOS engine. |
| **Servo** | Linux Foundation / former Mozilla Research | Rust-based, fragmented — parts in Firefox, parts in research | The only "research engine" that ships real consumer builds (via the Servo-Starter-Fork / Nyxt integration). Cited where it diverges from Chromium/Gecko. |
| **Ladybird** | The SerenityOS project | LibJS + LibWeb + LibGfx | Independent from-scratch C++ engine. Not in production use but the most credible fresh-engine comparator for Spiral. |
| **Flow** | Huawei / The Browser Company (formerly) — *verify in chunk 1* | Stack: dual-engine (previously used a fork of Chromium with a custom shell) | The only independent engine with backing by a top-tier vendor outside the US/EU bloc. Treated with caution. **Novelty-gate check required: re-verify the engine identity and current status before chunk 12 is written; if the engine has changed or been retired, the row is replaced with the next credible candidate (e.g. Thunderbird's rendering surface, or Ecosia/Brave's Gecko-derived work).** |

Brave is not its own row. Brave is Chromium + privacy features. Brave-
specific behaviour is recorded in the Chromium "Engine notes" column
with `(Brave fork)` prefix.

---

## 3. Status in Spiral

Each capability row carries one of these values:

| Status | Definition |
|--------|-----------|
| `not-started` | No `Cargo.toml` reference, no design doc, no discussion. |
| `designed` | Has a design doc / ADR / `docs/design-*` file but no code. |
| `partial` | Some code, but not exercisable from a real surface (the "wiring gap" rule from `AGENTS.md` §"Wiring & Integration"). |
| `shipped` | Exists in code **and** is reachable from a real surface, and at least one integration test exercises it. (Same gate as `AGENTS.md`.) |
| `do-not-touch` | Explicitly out of scope. Reserved for capabilities that contradict the architecture bets (e.g. "tracking pixels" — no, that's not a capability; example: a user-facing recommender that conflicts with `Bet 1`'s local-first stance). Use sparingly and cite the bet. |

---

## 4. Browser prevalence

Per MDN Baseline-style scoring, plus a fifth bucket for the things the
Baseline table does not cover.

| Bucket | Definition | Examples |
|--------|-----------|----------|
| `ubiquitous` | >95% of users on a current browser can use it; Tier-2 "Baseline widely available." | CSS Grid, Fetch, Service Workers (HTTPS), WebAssembly. |
| `widespread` | 70–95%; Baseline "newly available" or recently "widely available." | View Transitions API, WebGPU (1.0), OPFS, Compute Pressure. |
| `mixed` | Two or more engines ship it, at least one does not. | WebTransport, WebCodecs, Payment Request, WebNN. |
| `niche` | One engine ships it; others have not. | Firefox's Total Cookie Protection edge cases, Safari's WebGPU quirks, Chromium's Speculation Rules. |
| `experimental` | Flag-only or origin trial; not in any stable release. | WebNN (full), WebXR Depth, Speculation Rules (origin trial). |
| `legacy` | W3C "deprecated" or marked for removal. | AppCache, Plugin API (`<embed>`, `<object>`), showModalDialog, `document.all`. Tracked because of old sites, not because we want to ship them. |

---

## 5. Phase impact

Mapping a capability to a Spiral phase uses the existing `ROADMAP.md`
phases plus two special markers. Phases are:

| Phase | Months | Theme |
|-------|--------|-------|
| 1 | 1–3 | Foundation (Cargo, IPC, hello-world render) — **complete on `audit/m4-window`** |
| 2 | 4–9 | Vendored parsers, Gyre block layout, Vortex lexer/parser/interpreter — **current** |
| 3 | 10–24 | Flex layout, text rendering, Vortex bytecode VM, basic DOM-from-JS |
| 4 | 25–42 | Grid layout, networking, HTTP/HTTPS, DOM manipulation, image decoding |
| 5 | 43–60 | Zen UI, GPU rendering, Vello optimisation, sandbox, Vortex baseline JIT |
| 6 | 61–84 | WPT compliance, performance tuning, cross-platform packaging, v0.1.0 |
| **0.1-blocker** | n/a | Must ship before the 0.1.0 public release. Distinct from phase 6. |
| **1.0-blocker** | n/a | Must ship before the 1.0 "ready for daily use" release. |

A capability can carry two phase tags (`P2 / P3`) if it is foundational in
phase 2 and gets depth in phase 3. The earlier tag drives the priority.

---

## 6. Complexity

| Size | Definition |
|------|-----------|
| `S` | < 1 day, single crate, no design. |
| `M` | 1–5 days, single crate or two, design doc required. |
| `L` | 1–4 weeks, multi-crate, ADR required, integration tests. |
| `XL` | 1+ month, new crate or major subsystem, ADR + RFC, multi-quarter. |

Complexity is **not** the same as prevalence. A `mixed` capability can
be `S` (e.g. a CSS `::backdrop` pseudo-class) and a `niche` capability
can be `XL` (e.g. a brand-new layout engine in Gyre).

---

## 7. Capability naming rules

Per `AGENTS.md` §"Novelty Claims" and the implicit brand rule in
`docs/glossary.md`, capabilities are named **plainly** — by what they
**do**, not by what any single product **calls** them.

| Avoid | Use |
|-------|-----|
| "Chrome's site permissions UI" | "URL-pattern permission scopes" |
| "Firefox Total Cookie Protection" | "third-party storage partitioning" |
| "Safari Intelligent Tracking Prevention" | "cross-site tracking mitigation policy" |
| "Edge Collections" | "scratch-pad web clippings" |
| "Chrome's Reading List" | "saved-for-later article list" |
| "Arc Workspaces" | "named tab set / workspace" |
| "Vivaldi Tab Tiling" | "tiled tab layout" |
| "Brave Shields" | "per-site content filter" |

The synthesis in chunk 13 will keep an eye out for accidental brand
leakage. If a name is in MDN or the spec, it's fine — the rule is about
product names.

---

## 8. The novelty claim gate (run in chunk 13 only)

If chunk 13 (synthesis) wants to recommend a capability with the word
**novel**, **first**, **unique**, **no prior art**, or **no shipped
browser does this**, the recommendation must be backed by a
**prior-art check** that names what was found in **each** of:

1. V8 (Chromium's JS engine)
2. SpiderMonkey (Firefox)
3. JavaScriptCore (WebKit)
4. Servo
5. Ladybird
6. Flow *(see §2 — the engine identity must be re-verified at the time
   the gate runs; the gate cannot pass if "Flow" has changed scope.)*
7. Brave *(as a Chromium fork, tracked separately for fork-specific
   behaviour, not for engine novelty)*

**Wikipedia is a starting point, not a conclusion.** Following the M4
audit, the default failure mode is overclaiming. If a novelty claim
cannot be backed, downgrade it to **"partially novel (combination is
new)"** or **"configuration choice"** before it enters the synthesis.

The prior-art check is recorded in `12-gap-synthesis.md` §"Novelty
Audit Log."

---

## 9. What is **not** in scope for this research

The user said "modern browser" — that is the surface area. The
following are explicitly **out** of scope for this research subset and
will be noted in `12-gap-synthesis.md` as "out of scope" rather than
omitted silently:

- **Email/calendar/news clients.** Browsers do not ship these; they
  may integrate with them, but it's a partner-relationship surface, not
  a browser feature.
- **AI assistants** (e.g. integrated chatbot). These are a 2025–2026
  product differentiator, not a "modern browser" baseline. Spiral's
  stance is covered in `docs/architecture-shared-everything.md`; this
  research does not score them.
- **Cryptocurrency wallet features.** Same reason. Not a baseline.
- **Marketing/upsell surfaces** (e.g. built-in shopping portal, news
  feed on new-tab). Not a baseline. Out of scope.
- **Cloud sync** in the OS-vendor sense (e.g. iCloud Tabs, Chrome
  Sync). Tracked as "opt-in cross-device data sync" without binding
  to a vendor's cloud. The capability is in scope; the cloud is the
  user's choice.

---

## 10. Edit protocol

- Every research chunk is one commit.
- Chunk 13 is the **only** chunk that writes to files outside
  `docs/research/`.
- Outside of chunk 13, the only permitted commit messages are
  `docs(research): chunk N — <title>`.
- Chunk 13's commits follow the existing project convention
  (`docs(ssot):`, `specs(gap):`, `docs(plans):`).
- No force-push, no rebase after the PR is open.

---

## 11. Open questions for the user (to be resolved before chunk 1)

These are the methodology-level questions I have flagged but not yet
asked. They are not blockers for chunk 0 itself; they are blockers for
chunks 1–11. I will surface them in order of dependency.

1. **Engine identity for the "Flow" row** — see §2. Re-verify before
   chunk 12 is written. If Flow has shifted, propose replacement.
2. **"Modern browser" baseline year** — proposed default: 2026, meaning
   "what does a top-3 browser ship in stable as of January 2026" with
   "experimental" items called out separately. Confirm or override.
## 11.1 Locked decisions (2026-06-16)

| Question | Decision |
|---|---|
| Flow engine row | **Trust the methodology.** Re-verify Flow's current scope at chunk 12 time. If Flow has shifted, replace with the next credible candidate. |
| "Modern browser" baseline year | **2026.** Score against what a top-3 browser ships in stable as of 2026-01, with "experimental" items called out separately. Snapshot date 2026-06-16. |
| Scope | **Desktop + mobile + embedded.** Every capability row carries a `surface` column with one or more of `desktop`, `mobile`, `embedded`. The matrix is the union; a row can be `desktop+mobile+embedded`, `mobile+embedded` only, etc. Spiral is desktop-first today; the surface column makes it explicit when we are scoring a future capability Spiral has not yet chosen to chase. |
| Regulator coverage | **Global (DSA + CRA + GDPR).** No US/UK/CNIL-specific rows in the matrix. The Tier-4 column of the sources index lists ICO, CNIL, FTC for **methodology completeness only** — they are not the basis of any row. |

### What "embedded" means in this subset

Embedded = in-vehicle (e.g. CarPlay's web layer, Android Automotive
web), set-top, wearables (Wear OS Web, watchOS Web — limited surfaces),
XR (WebXR on Vision Pro / Quest), and game consoles (Switch browser,
PS5 browser, Xbox Edge). The embedded row exists so that we do not
score mobile/embedded-specific capabilities (e.g. ARCore/ARKit-style
APIs, WebXR's immersive mode, share-extension surfaces) as
"non-existent" when in fact they are the canonical target surface for
that capability.

Spiral is not committing to ship to any of these surfaces. The
embedded row is a **scope** decision for the research, not a roadmap
commitment.

### Update to §9 (out of scope)

The "Mobile is not in scope" item is **removed** from §9. Mobile and
embedded surfaces are in scope per the locked decision above. The
"AI assistants" and "cryptocurrency wallet" exclusions remain.

## 11.2 Locked decisions from chunk 1 review (2026-06-16)

| # | Question | Decision |
|---|----------|----------|
| 1 | Prevalence column: MDN Baseline vs HTTP Archive vs per-engine? | **MDN Baseline** (current approach). Per-engine `yes/no/partial` is the engine notes column — no double-counting. |
| 2 | Obsolete-but-parsed HTML elements (`<acronym>`, `<big>`, `<marquee>`, etc.) | **Collapsed single row.** Expanding adds noise, not insight. |
| 3 | Masonry CSS layout feature — chunk 1 (CSS) or chunk 8 (developer)? | **Stay in chunk 1.** It's a CSS layout feature; chunk 8 is developer tools. |
| 4 | Intl: row-per-builtin or row-per-locale-sensitivity? | **Row-per-builtin** (current approach). Locale-sensitivity is a property of a builtin, not a distinct capability. |
| 5 | Early-stage engine "no" coverage — `no-with-plan` / `no-without-plan` qualifier? | **No qualifier.** Blanket "no" is honest; the matrix cannot reliably guess intent. |
| 6 | `@scope` and `::scroll-marker*` — chunk 1 or chunk 8? | **`@scope` stays in chunk 1** (CSS at-rule). **`::scroll-marker*` stays in chunk 1** if it's in a shipped spec (CSS Pseudo-Elements 4); defer to chunk 7 if draft-only. |
| 7 | Houdini worklets split — `@property` in chunk 1, worklets in chunk 6/8? | **`@property` in chunk 1** (CSS registered custom properties). **Worklets** (paint, layout, animation) go in **chunk 6** (APIs & runtime). The current split is correct. |
