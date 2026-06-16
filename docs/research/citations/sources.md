# Sources — Master Index

> **Per chunk 0 §1, every claim in the research subset is backed by a
> source from one of five tiers.** This file is the master index. Each
> chunk adds to a chunk-local `## Sources` table; this file is the
> canonical list of domains and where they sit on the tier ladder.

---

## Tier 1 — Standards bodies and conformance suites

These are the primary sources. Where a row in a research chunk says
"the standard defines X", it is backed by a URL in this section.

| Source | Domain | Use for |
|--------|--------|---------|
| WHATWG | `html.spec.whatwg.org`, `dom.spec.whatwg.org`, `fetch.spec.whatwg.org`, `url.spec.whatwg.org`, `infra.spec.whatwg.org`, `storage.spec.whatwg.org`, `streams.spec.whatwg.org`, `xhr.spec.whatwg.org`, `mimesniff.spec.whatwg.org`, `encoding.spec.whatwg.org`, `notifications.spec.whatwg.org`, `permissions.spec.whatwg.org` | HTML, DOM, Fetch, URL parsing, web platform infra |
| W3C Recommendations | `w3.org/TR/` | CSS, ARIA, SVG, WebRTC, EME, WebAuthn, Payment Request, WebGPU, WebTransport, WebNN, IndexedDB, WAI, Web Annotation, WICG drafts that have reached REC |
| IETF | `datatracker.ietf.org` | HTTP (RFC 9110, 9111, 9112, 9113, 9114), QUIC (RFC 9000, 9301, 9302, 9303, 9304, 9305, 9308, 9310, 9311, 9412, 9413, 9439), DNS (RFC 1034/1035, 8484 DoH, 9250 DoQ, 6698 TLSA), TLS (RFC 8446, 8447, 8879, 9325), DNSSEC (RFC 4033+), Cookies (RFC 6265), WebSocket (RFC 6455), BCPs, RFC 9211 (HTTP caching) |
| ECMA International | `tc39.es/ecma262/`, `tc39.es/ecma402/`, `tc39.es/ecma404/` | ECMAScript, Intl, JSON |
| Unicode | `unicode.org/reports/`, `unicode.org/versions/Unicode-16.0.0/` | UAX #9 bidi, UAX #11 East Asian Width, UAX #14 line breaking, UAX #15 normalization, UAX #24 script property, UAX #29 text segmentation, UAX #31 identifier syntax, ICU behaviour |
| WPT | `web-platform-tests.org` | The cross-engine conformance suite. Used to verify "is this interoperable?" rather than "does one engine do it?" |
| WAI | `w3.org/WAI/`, `w3.org/WAI/ARIA/`, `w3.org/WAI/standards-guidelines/wcag/` | ARIA Authoring Practices, WCAG 2.2 / 3.0, AT-SPI, UIA, AX API |

---

## Tier 2 — Aggregator and documentation sites

| Source | Domain | Use for |
|--------|--------|---------|
| MDN Web Docs | `developer.mozilla.org` | "Baseline" table, browser support columns, reference documentation. **The default citation for "which engines support this."** |
| Can I Use | `caniuse.com` | Coverage by version, region splits, "is it on the roadmap" |
| Chrome Platform Status | `chromestatus.com` | Origin trials, shipping intent, intent-to-deprecate, intent-to-experiment |
| web.dev | `web.dev` | Patterns and "this is the right way to use it" |
| webxr.dev | `webxr.dev` (archived) and Immersive Web working group | WebXR device APIs, AR |
| Khronos | `registry.khronos.org` | WebGL 1.0/2.0/3.0 specs, WebGPU binding to native APIs (Vulkan, Metal, DX12) |
| WHATWG wiki | `wiki.whatwg.org` | Open issues per spec — useful when the spec text is unsettled |
| IETF WG drafts | `datatracker.ietf.org/wg/` | Live drafts, in-progress work, mailing list archives |

---

## Tier 3 — Engine release notes, vendor documentation, status pages

| Source | Domain | Use for |
|--------|--------|---------|
| Chromium blog / release notes | `blog.chromium.org`, `chromium.googlesource.com/chromium/src/+/refs/heads/main/RELEASES`, `chromiumdash.appspot.com` | What ships in which Chromium version |
| Chrome Platform Status (also tier 2) | see above | Implementation status, intent-to-ship, intent-to-deprecate |
| Firefox release notes | `mozilla.org/en-US/firefox/releases/`, `wiki.mozilla.org/Release_Management/Calendar` | What ships in which Firefox version |
| Mozilla Hacks blog | `hacks.mozilla.org` | Mozilla's engineering writeups (often Tier 1-equivalent for Gecko internals) |
| MDN blog | `developer.mozilla.org/en-US/blog/` | Mozilla's broader web-platform commentary |
| WebKit blog | `webkit.org/blog/` | WebKit release notes, engineering posts |
| Apple Developer | `developer.apple.com/documentation`, `developer.apple.com/safari/`, `developer.apple.com/wwdc/` | Safari/WebKit internals, deprecation announcements, WWDC sessions |
| Microsoft Edge docs | `learn.microsoft.com/en-us/microsoft-edge/`, `learn.microsoft.com/en-us/deployedge/` | Edge-specific behaviour, enterprise policy schema |
| Servo | `servo.org`, `github.com/servo/servo/blob/main/RELEASES.md`, `github.com/servo/servo/wiki` | Servo release notes, what's shipping in the consumer builds |
| Ladybird | `ladybird.org`, `github.com/LadybirdBrowser/ladybird/`, Andreas Kling's YouTube | Ladybird release notes, progress reports |
| Brave | `brave.com/privacy-features/`, `brave.com/developer/` | Brave-specific behaviour on top of Chromium |
| Flow | (re-verify in chunk 12) | Status, stack, browser family — see `00-methodology.md` §2 |
| Developer Changelog, blogs of vendors | various | Per-vendor change logs |

---

## Tier 4 — Third-party audits and regulator guidance

| Source | Domain | Use for |
|--------|--------|---------|
| privacytests.org | `privacytests.org` | Open-source, automated cross-engine privacy test suite. **The default citation for "is this privacy feature actually on by default?"** |
| EFF Cover Your Tracks | `coveryourtracks.eff.org` | Trackers, fingerprinting, header behaviour |
| EU DSA (Digital Services Act) | `digital-strategy.ec.europa.eu/en/policies/digital-services-act` | Notice-and-action, transparency |
| EU CRA (Cyber Resilience Act) | `digital-strategy.ec.europa.eu/en/policies/cyber-resilience-act` | Vulnerability handling, SBOM, default security posture |
| EU AI Act | `artificialintelligenceact.eu` and the official OJ text | Out of scope for this research (per `00-methodology.md` §9) but cross-referenced in chunk 11 because the Act applies to browsers as "high-risk" in certain configurations |
| GDPR (Regulation 2016/679) | `gdpr-info.eu` | Cookie consent, lawful basis for processing |
| ICO | `ico.org.uk` | UK regulator guidance (cookies, PECR) |
| CNIL | `cnil.fr` | French DPA — strong on cookies, fingerprinting |
| FTC | `ftc.gov` | US consumer protection guidance |
| NDSS / IEEE S&P / USENIX Security | `ndss-symposium.org`, `ieee-security.org`, `usenix.org/usenixsecurity24` | Academic papers, formal analyses of web security mechanisms (SOP, SRI, CORS, cache partitioning) |
| W3C TAG | `w3.org/2001/tag/` | Architectural findings and design reviews |
| WCAG-EM | `w3.org/WAI/eval/`, `w3.org/WAI/test-evaluate/` | Accessibility evaluation methodology |

---

## Tier 5 — Supporting material (re-anchor to Tier 1–4 before synthesis)

| Source | Domain | Use for |
|--------|--------|---------|
| Conference talks (TPAC, BlinkOn, GopherCon, LF/LPC, JSCamp, WWDC sessions) | various | Engineering context, not a primary source |
| Wikipedia | `wikipedia.org` | Background reading, link harvesting to Tier 1–3 |
| Personal blogs, Twitter/X, Mastodon, Hacker News threads | various | Community signal only |
| The Verge, Ars Technica, Hacker News front page | various | Industry commentary only |
| MDN "Learn" tutorials | `developer.mozilla.org/en-US/docs/Learn/` | Educational context |

---

## Engines tracked

The competitive matrix (chunk 12) tracks six engines. See
`00-methodology.md` §2 for the full rationale, including the
"Flow" identity flag.

| Engine | Primary source | Backup source |
|--------|----------------|---------------|
| Chromium | `chromestatus.com`, `chromiumdash.appspot.com` | `blog.chromium.org`, WPT |
| Firefox | `developer.mozilla.org/en-US/blog/`, `wiki.mozilla.org/Release_Management/Calendar` | WPT, MDN |
| WebKit | `webkit.org/blog/`, `developer.apple.com/documentation/safari-release-notes` | WPT, MDN |
| Servo | `servo.org`, GitHub release notes | `github.com/servo/servo/wiki` |
| Ladybird | `ladybird.org`, GitHub release notes | Andreas Kling's YouTube, SerenityOS blog |
| Flow | (re-verify — see `00-methodology.md` §2) | (re-verify) |

---

## Notes on currency

The research snapshot is **2026-06-16**. Every claim carries the date
of the underlying source where it matters. Where the source is a "live"
status (e.g. "shipping", "in development"), the most recent date the
author could verify is recorded in the row.

If a row in a research chunk reads "as of 2026-06" without a primary-
source date, that is a **bug** — open a `docs(research): chunk N —
date missing` follow-up.
