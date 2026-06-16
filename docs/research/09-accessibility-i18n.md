# Chunk 9 — Accessibility & Internationalisation

> **Chunk 9 of 14.** This chunk inventories (a) the engine + chrome
> accessibility surface that lets disabled and assistive-technology
> users operate the browser, and (b) the internationalisation surface
> (locale detection, `Intl.*` APIs, text shaping, bidi, vertical text,
> line breaking, calendars, encoding, spellcheck, translation API).
>
> **Splitting:** the file would exceed the 600-line cap, so this is
> the **index** + the accessibility half. The internationalisation
> half lives in `09-i18n-engine.md`.

---

## Scope

**Accessibility (this file, §1):**

- ARIA reflection on DOM (`role`, `aria-*` attributes, IDL properties)
- Accessibility tree construction + serialisation
- Platform AT bridges (AT-SPI2, UIA/MSAA, NSAccessibility, AX iOS,
  TalkBack, ChromeVox)
- Screen-reader rendering of web content and chrome
- Keyboard navigation, focus management, focus rings, focus traps,
  focus restoration, skip links
- `forced-colors` / high-contrast, `prefers-reduced-motion`,
  `prefers-reduced-transparency`, `prefers-reduced-data`, Save-Data
- `prefers-color-scheme`, `color-scheme`, `light-dark()`
- Touch target sizing (WCAG 2.5.5, iOS HIG, Material 48dp)
- Voice control, switch control, eye tracking
- Captions / subtitles / chapters / sign language / audio description
- Zoom (full-page, text-only, reflow)
- Pointer cancellation, hit-test enlargement
- `:focus-visible` / `:focus-within`, `tabindex`, sequential focus
- Form labels, error association, live regions, landmark roles
- PDF accessibility (tagged, reflow, reading order)
- Mobile a11y (pinch-zoom, dynamic type, font scale)
- Browser chrome a11y (discoverable shortcuts, keyboard help, focus
  on URL bar)
- Drag-and-drop a11y (`aria-grabbed`, `aria-dropeffect`, HTML5 DnD)
- Reading order (`reading-flow` CSS, source vs visual)
- ARIA 1.3 / 1.4 surface

**Internationalisation (`09-i18n-engine.md`, §2):**

- Locale detection, negotiation, fallback
- `Intl.*` API surface: NumberFormat, DateTimeFormat, RelativeTimeFormat,
  ListFormat, Segmenter, DisplayNames, PluralRules, Collator, Locale
- ICU/CLDR backing, BCP 47
- Text shaping: CTL, bidi (UAX #9), joiners, ZWJ emoji, variation
  selectors
- Vertical text (`writing-mode`, `text-orientation`)
- Line breaking (UAX #14, `line-break`, `word-break`, `overflow-wrap`)
- East Asian Width (UAX #11)
- Unicode normal forms (NFC/NFD/NFKC/NFKD), IDNA, UTS #46
- Calendar systems (16+), time zones, era, day period
- Locale-aware input: date, number, currency, name
- Spellcheck, IME, dead keys, compose
- Translation API surface (page translation, Translator API)
- Character encoding detection, BOM, output encoding
- Font fallback chain, locale-specific fonts, OpenType, variable fonts
- Bidi / mirror / isolate, justification, indentation, quotation
  marks, `dir=auto`
- Case mapping (Turkish dotless i etc.), sort/collation
- Locale-specific search

## Out of scope

- User-facing translation UI (chunk 7)
- Page encoding detection internals (chunk 1)
- Engine-internal character handling (chunk 1)
- Font rendering internals (chunk 1)
- Screen reader chrome UI / chrome-level keyboard (chunk 7)
- Voice / speech APIs (chunk 6)
- Extension APIs (chunk 10)
- Distribution (chunk 11)

## Spiral grounding

`grep -r "ARIA\|a11y\|accessibility\|focus\|keyboard\|Intl\|i18n\|locale\|bidi\|RTL\|CJK" crates/ --include="*.rs" -l`
returns noise matches only (`variant` / `role` in tests, `url_bar_focused`
boolean in `spiral-ui`). No real a11y or i18n code exists. `spiral-dom`
holds only node + element + text + comment + document; no
`AriaProperties` map, no `AccessibleName` algorithm. `spiral-gyre`
computes block flow only — no RTL, no vertical text, no line breaking.
`spiral-vortex` exposes `Array` / `Console` / `Math` / `Object` only;
no `Intl` namespace. `spiral-render` is a software rasteriser with no
`prefers-color-scheme` or `forced-colors` consumer. `spiral-ui` is a
winit shell with one boolean (`url_bar_focused`); no keymap, no
shortcut registry, no a11y role. Status entries below score
`not-started` against this baseline, with phase assignments aligned
to the project roadmap (a11y / ICU on the M61–84 horizon per
`specs/GAP_ANALYSIS.md` §4.5).

## Per-file index

- This file: §1 Accessibility — rows 1–44
- `09-i18n-engine.md`: §2 Internationalisation — rows 45–90

## Row format

`# | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources`

Engine notes: one-line per engine (Chromium / Firefox / WebKit / Servo
/ Ladybird / Flow). Spec names only; no product names per methodology
§7. Australian English spelling throughout.

---

## §1 Accessibility

### §1.1 ARIA reflection on the DOM

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 1 | `role` attribute and `ARIAMixin` / `ElementInternals` reflection (`element.role`, `element.ariaRoleDescription`) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WAI-ARIA 1.2 §role-attribute](https://www.w3.org/TR/wai-aria-1.2/#role-attribute) ; [ARIA Reflection §reflection](https://w3c.github.io/aria-reflection/) |
| 2 | `aria-*` IDL properties (full ARIAMixin set: `ariaLabel`, `ariaLabelledByElements`, `ariaDescribedByElements`, `ariaCurrent`, `ariaExpanded`, `ariaHidden`, `ariaModal`, `ariaBusy`, `ariaAtomic`, `ariaRelevant`, `ariaOwns`, etc.) | desktop+mobile+embedded | not-started | >=90% | P5 | XL | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [ARIA Reflection §ARIAMixin](https://w3c.github.io/aria-reflection/#ariamixin) ; [WAI-ARIA 1.2 §state-property-definitions](https://www.w3.org/TR/wai-aria-1.2/#state_property_definitions) |
| 3 | `aria-label` / `aria-labelledby` / `aria-describedby` accessible-name computation | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [ACCName §mapping_general](https://www.w3.org/TR/accname-1.2/) ; [HTML §attr-aria-label](https://html.spec.whatwg.org/multipage/dom.html#aria-label) |
| 4 | `aria-live` polite / assertive + `atomic` + `relevant` (`additions` / `removals` / `text` / `all`) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WAI-ARIA 1.2 §aria-live](https://www.w3.org/TR/wai-aria-1.2/#aria-live) ; [ARIA in HTML §live-regions](https://www.w3.org/TR/html-aria/) |
| 5 | Implicit ARIA semantics per HTML element (`button` → `role=button`, `input[type=checkbox]` → `role=checkbox`, `nav` → `role=navigation`, etc.) | desktop+mobile+embedded | not-started | >=90% | P5 | L | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [ARIA in HTML §recommendations](https://www.w3.org/TR/html-aria/) ; [HTML §wai-aria](https://html.spec.whatwg.org/multipage/dom.html#wai-aria) |
| 6 | Landmark roles (`banner`, `main`, `navigation`, `contentinfo`, `complementary`, `region`, `search`, `form`) and the `aria-roledescription` override | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [ARIA in HTML §landmark-roles](https://www.w3.org/TR/html-aria/#document-structure-roles) |
| 7 | `role=alert` / `role=status` / `role=log` / `role=timer` / `role=marquee` (implicit `aria-live` values) | desktop+mobile+embedded | not-started | >=90% | P5 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WAI-ARIA 1.2 §live-region-roles](https://www.w3.org/TR/wai-aria-1.2/#live_region_roles) |
| 8 | `aria-invalid` / `aria-errormessage` / `aria-required` form-state plumbing | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WAI-ARIA 1.2 §aria-invalid](https://www.w3.org/TR/wai-aria-1.2/#aria-invalid) ; [ARIA in HTML §form-roles](https://www.w3.org/TR/html-aria/) |
| 9 | `aria-grabbed` / `aria-dropeffect` (legacy) and HTML5 drag-and-drop a11y (`aria-describedby` on drag, `dragover` cancel) | desktop+mobile+embedded | not-started | 50–75% (legacy ARIA DnD deprecated; modern DnD relies on live-region text) | P6 | M | Chromium: yes (both) · Firefox: legacy only · WebKit: legacy only · Servo: no · Ladybird: no · Flow: no | [WAI-ARIA 1.2 §aria-grabbed](https://www.w3.org/TR/wai-aria-1.2/#aria-grabbed) (deprecated) ; [HTML §dnd](https://html.spec.whatwg.org/multipage/dnd.html) |
| 10 | `aria-controls` / `aria-flowto` / `aria-describedby` / `aria-details` element references (IDREF + IDREF list) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WAI-ARIA 1.2 §IDL-Properties](https://www.w3.org/TR/wai-aria-1.2/#idl-properties) |

### §1.2 Accessibility tree + platform bridges

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 11 | Accessibility tree construction (parallel to render tree; platform-specific nodes with name / role / state / description) | desktop+mobile+embedded | not-started | >=90% | P5 | XL | Chromium: yes (AXTree) · Firefox: yes (xpcAccessible) · WebKit: yes (AXObject) · Servo: partial · Ladybird: no · Flow: no | [Core AAM §mapping](https://www.w3.org/TR/core-aam-1.2/) ; [ARIA 1.2 §mapping-to-platforms](https://www.w3.org/TR/wai-aria-1.2/) |
| 12 | AT-SPI2 bridge (Linux, D-Bus `org.a11y.atspi`) — exposes the tree to Orca and other Linux AT | desktop+embedded | not-started | >=90% (Linux) | P5 | L | Chromium: yes · Firefox: yes · WebKit: gtk port yes · Servo: no · Ladybird: no · Flow: no | [AT-SPI2 spec](https://gitlab.gnome.org/GNOME/at-spi2-core/-/blob/master/docs/at-spi2-spec.md) |
| 13 | UIA / MSAA bridge (Windows: `IAccessible` legacy + `IRawElementProviderSimple` UIA3) | desktop | not-started | >=90% (Windows) | P5 | L | Chromium: yes · Firefox: yes · WebKit: Win port yes · Servo: no · Ladybird: no · Flow: no | [UI Automation](https://learn.microsoft.com/en-us/windows/win32/winauto/entry-uiauto-win32) |
| 14 | NSAccessibility bridge (macOS — `NSAccessibilityElement`, `NSAccessibilityProtocol`) | desktop | not-started | >=90% (macOS) | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [Apple NSAccessibility](https://developer.apple.com/documentation/appkit/nsaccessibility) |
| 15 | AX iOS / iPadOS bridge (AccessibilityKit, `AXElement`/`AXValue`) and Android `AccessibilityNodeProvider` (TalkBack bridge) | mobile | not-started | >=90% (iOS + Android) | P6 | L | Chromium: yes · Firefox: yes (Android only) · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [Android AccessibilityNodeProvider](https://developer.android.com/reference/android/view/View#onProvideAccessibilityNodeInfo%28android.view.accessibility.AccessibilityNodeInfo%29) |
| 16 | ChromeVox-style in-process screen reader (default when no OS AT is present, e.g. on Linux server) | desktop | not-started | niche (extension in Chromium; built-in ChromeOS) | P6 | XL | Chromium: extension / ChromeOS built-in · Firefox: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [ChromeVox docs](https://chromevox.com/) |
| 17 | Accessibility event mutation queue + coalesced notifications (focus, text-changed, selection-changed, live-region updates) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [Core AAM §events](https://www.w3.org/TR/core-aam-1.2/#mapping-events) |

### §1.3 Keyboard, focus, and chrome a11y

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 18 | Sequential focus navigation (Tab / Shift+Tab, tabindex, focusable-element algorithm) | desktop+mobile+embedded | not-started (DOM has no focusable-element list; only `url_bar_focused` boolean in `spiral-ui`) | >=90% | P4 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [HTML §focusable-area](https://html.spec.whatwg.org/multipage/interaction.html#focusable-area) ; [HTML §sequential-focus-navigation](https://html.spec.whatwg.org/multipage/interaction.html#sequential-focus-navigation) |
| 19 | `:focus-visible` CSS pseudo-class (heuristic + `focus-visible` polyfill shape) | desktop+mobile+embedded | not-started | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [Selectors 4 §focus-visible-pseudo](https://www.w3.org/TR/selectors-4/#focus-visible-pseudo) |
| 20 | `:focus-within` CSS pseudo-class | desktop+mobile+embedded | not-started | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [Selectors 4 §focus-within-pseudo](https://www.w3.org/TR/selectors-4/#focus-within-pseudo) |
| 21 | Focus trap inside modal `<dialog>` (`showModal()` algorithm) and inert subtree | desktop+mobile+embedded | not-started | >=90% | P4 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [HTML §the-dialog-element](https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element) ; [HTML §inert](https://html.spec.whatwg.org/multipage/interaction.html#inert) |
| 22 | Focus restoration (back / forward navigation, dialog dismiss, route change) | desktop+mobile+embedded | not-started | >=90% | P4 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [HTML §focus-fixup-rule](https://html.spec.whatwg.org/multipage/interaction.html#focus-fixup-rule) |
| 23 | Skip-to-content links and browser-level keyboard shortcuts (`?` help, `Cmd+L` URL bar, `Cmd+T` new tab) | desktop+mobile+embedded | not-started (no shortcut registry in `spiral-ui`) | >=90% | P4 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WAI-ARIA Authoring Practices §landmarks](https://www.w3.org/WAI/ARIA/apg/practices/landmark-regions/) |
| 24 | Focus rings (user-agent default style + per-engine override) and `outline` reset for `:focus-visible` | desktop+mobile+embedded | not-started | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [CSS UI 4 §outline-props](https://www.w3.org/TR/css-ui-4/#outline-props) |
| 25 | `inert` attribute and `inert` IDL property (subtree outside tab order, screen reader, click events) | desktop+mobile+embedded | not-started | >=90% (Baseline 2022) | P4 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [HTML §inert](https://html.spec.whatwg.org/multipage/interaction.html#inert) |
| 26 | Spacing / hit-target enlargement for small controls (`input[type=checkbox|radio]`, OS-level "shape writing") | desktop+mobile+embedded | not-started | >=90% | P4 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [HTML §checkbox](https://html.spec.whatwg.org/multipage/input.html#checkbox-state-(type=checkbox)) ; [Apple HIG Accessibility](https://developer.apple.com/design/human-interface-guidelines/accessibility) |
| 27 | Pointer-cancellation semantics (`pointercancel`, `touch-action`, hit-test) | desktop+mobile+embedded | not-started | >=90% | P4 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [Pointer Events 3 §the-pointercancel-event](https://www.w3.org/TR/pointerevents3/#the-pointercancel-event) |

### §1.4 Media, captions, and audio description

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 28 | `<track kind="subtitles|captions|descriptions|chapters|metadata">` + `TextTrack` / `TextTrackCue` / `TextTrackList` / `addTextTrack()` | desktop+mobile+embedded | not-started | >=90% | P4 | L | Chromium: yes (WebVTT + TTML) · Firefox: yes (WebVTT) · WebKit: yes (WebVTT) · Servo: partial · Ladybird: no · Flow: no | [HTML §the-track-element](https://html.spec.whatwg.org/multipage/media.html#the-track-element) ; [WebVTT](https://www.w3.org/TR/webvtt1/) |
| 29 | WebVTT rendering (cue settings: line, position, size, align, vertical, region, snap-to-lines) and WebVTT CSS extension | desktop+mobile+embedded | not-started | >=90% | P5 | L | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WebVTT §css-extensions](https://www.w3.org/TR/webvtt1/#css-extensions) |
| 30 | Native caption settings UI (position, colour, font family, size, background, opacity) with OS-level override | desktop+mobile+embedded | not-started (no `spiral-ui` caption panel) | >=90% (mobile ubiquitous, desktop 75–90%) | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [WebVTT §cue-settings](https://www.w3.org/TR/webvtt1/#cue-settings) |
| 31 | Audio-description track selection (AD button on `<video>` controls; auto-detect `audio-described` MediaQuery) | desktop+mobile+embedded | not-started | 75–90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [HTML §dom-media-audiodescriptiontrack](https://html.spec.whatwg.org/multipage/media.html#dom-media-audiodescriptiontrack) |
| 32 | Sign-language overlay via pip (picture-in-picture) of a sign-language WebRTC stream | desktop+mobile | not-started | 50–75% (PiP yes, sign overlay niche) | P6 | L | Chromium: yes (PiP API + manual sign overlay) · Firefox: PiP yes · WebKit: PiP yes (iOS 15+, macOS 14+) · Servo: no · Ladybird: no · Flow: no | [Document Picture-in-Picture](https://wicg.github.io/document-picture-in-picture/) ; [W3C Media & Captioning BG](https://www.w3.org/community/captioning/) |

### §1.5 User-preference media queries (engine + chrome)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 33 | `@media (prefers-color-scheme: light\|dark)` and `color-scheme` CSS property (affects form control + scrollbar UA styling) | desktop+mobile+embedded | not-started (no media-query matcher in `spiral-fmt`; no consumer in `spiral-render`) | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [Media Queries 5 §prefers-color-scheme](https://www.w3.org/TR/mediaqueries-5/#prefers-color-scheme) ; [CSS Color Adjust §color-scheme](https://www.w3.org/TR/css-color-adjust-1/#color-scheme-prop) |
| 34 | `light-dark()` CSS function + `color-scheme` interaction (auto-switch palette by light/dark) | desktop+mobile+embedded | not-started | >=90% (Baseline 2024) | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [CSS Color 5 §light-dark](https://www.w3.org/TR/css-color-5/#light-dark) |
| 35 | `@media (prefers-reduced-motion: reduce)` and `prefers-reduced-transparency` | desktop+mobile+embedded | not-started | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [Media Queries 5 §prefers-reduced-motion](https://www.w3.org/TR/mediaqueries-5/#prefers-reduced-motion) ; [§prefers-reduced-transparency](https://www.w3.org/TR/mediaqueries-5/#prefers-reduced-transparency) |
| 36 | `Save-Data` request header + `@media (prefers-reduced-data: reduce)` (Save-Data client hint) | desktop+mobile+embedded | not-started (no `net` header mapping) | 50–75% | P4 | S | Chromium: yes (header + client hint + media query) · Firefox: header only · WebKit: header only · Servo: no · Ladybird: no · Flow: no | [RFC 8941 §3](https://datatracker.ietf.org/doc/html/rfc8941) ; [Save-Data](https://wicg.github.io/savedata/) ; [Network Quality Metrics](https://wicg.github.io/netinfo/) |
| 37 | `@media (prefers-contrast: more\|less\|custom)` and `@media (forced-colors: active\|none)` + `forced-color-adjust` CSS property | desktop+mobile+embedded | not-started | >=90% | P4 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [Media Queries 5 §prefers-contrast](https://www.w3.org/TR/mediaqueries-5/#prefers-contrast) ; [§forced-colors](https://www.w3.org/TR/mediaqueries-5/#forced-colors) ; [CSS Color Adjust §forced-color-adjust](https://www.w3.org/TR/css-color-adjust-1/#forced-color-adjust) |
| 38 | `@media (prefers-reduced-motion: reduce)` honoured for `scroll-behavior: smooth` and View Transitions | desktop+mobile+embedded | not-started | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [CSS Overflow 3 §smooth-scrolling](https://www.w3.org/TR/css-overflow-3/#smooth-scrolling) ; [View Transitions 1](https://www.w3.org/TR/css-view-transitions-1/) |

### §1.6 Chrome a11y + mobile a11y

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 39 | Chrome accessibility: discoverable keyboard shortcuts, `?` help, focusable URL bar, accessible tab list, tab-to-search | desktop | not-started (no shortcut registry, no chrome a11y tree) | >=90% | P6 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [WCAG 2.2 §2.1.1](https://www.w3.org/TR/WCAG22/#keyboard) ; [AOM §mapping-chrome](https://www.w3.org/TR/aria-1.2/) |
| 40 | Page zoom (Ctrl+/-), text-only zoom, layout reflow at 200% / 400% (WCAG 1.4.10) | desktop+mobile+embedded | not-started (no zoom factor in `spiral-render`; CSS `px` → physical pixels is hard-coded) | >=90% | P3 | M | Chromium: yes (full-page + text-only) · Firefox: yes (full-page) · WebKit: yes (full-page) · Servo: partial · Ladybird: no · Flow: no | [WCAG 2.2 §1.4.4](https://www.w3.org/TR/WCAG22/#resize-text) ; [§1.4.10](https://www.w3.org/TR/WCAG22/#reflow) |
| 41 | Mobile pinch-zoom (always-on, not disabled by `user-scalable=no`), double-tap-to-zoom, dynamic type (iOS), font scale (Android) | mobile | not-started | >=90% | P6 | M | Chromium: yes (Android) · Firefox: yes · WebKit: yes (iOS/macOS) · Servo: no · Ladybird: no · Flow: no | [WCAG 2.2 §1.4.4](https://www.w3.org/TR/WCAG22/#resize-text) ; [Apple HIG Accessibility](https://developer.apple.com/design/human-interface-guidelines/accessibility) ; [Material Accessibility](https://m3.material.io/foundations/accessible-design/accessibility-basics) |
| 42 | PDF accessibility: tagged PDF, reading order, alt text, structural tree (`<html>` tagged export from `<embed type="application/pdf">`) | desktop+mobile | not-started (no PDF renderer) | >=90% (Chromium PDFium yes; Firefox pdf.js yes; WebKit yes) | P7 | XL | Chromium: yes (PDFium + AX tree) · Firefox: yes (pdf.js) · WebKit: yes (PDFKit) · Servo: no · Ladybird: no · Flow: no | [PDF/UA-1 (ISO 14289-1)](https://www.iso.org/standard/64599.html) ; [PDF 1.7 §14.7 (tagged PDF)](https://opensource.adobe.com/dc-acrobat-sdk-docs/standards/pdfstandards/pdf/PDF32000_2008.pdf) |
| 43 | Voice-control integration (SpeechRecognition feeding element focus) and Switch Control (single / multi-switch device routing) | desktop+mobile | not-started | 75–90% | P6 | L | Chromium: yes (Web Speech + Switch Access on ChromeOS) · Firefox: partial · WebKit: yes (iOS Switch Control) · Servo: no · Ladybird: no · Flow: no | [Web Speech API](https://wicg.github.io/speech-api/) ; [Apple Switch Control](https://support.apple.com/en-au/guide/iphone/iph8a08cc80/ios) ; [ChromeOS Switch Access](https://support.google.com/chromebook/answer/9032659) |
| 44 | `reading-flow` CSS (`book`, `flex`, `grid`, `normal`) — explicit source-order mapping for sequential content | desktop+mobile+embedded | not-started | niche (Chromium 121+, Firefox 125+, WebKit no) | P6 | M | Chromium: yes · Firefox: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [CSS Display 4 §reading-flow](https://www.w3.org/TR/css-display-4/#reading-flow) ; [CSS Reading Flow Items](https://drafts.csswg.org/css-reading-flow-items/) |

---

## Open questions for the user

1. **A11y tree priority.** `specs/GAP_ANALYSIS.md` §4.5 lists screen reader / ARIA as M61–84. Given that ARIA semantics are entangled with the HTML element model (chunk 1 §A.1.5 / §A.1.7 form controls), is it acceptable to defer the full a11y tree to that horizon, or should the *implicit-ARIA-per-element* surface (rows 5, 6, 7) be promoted to the P4/P5 range so a basic screen-reader story exists before M61?

2. **Cross-engine AT bridge choice.** The four platform AT bridges (AT-SPI2, UIA, NSAccessibility, iOS/Android) are independent surfaces. Is the plan a single `spiral-a11y` crate owning all four, or split by platform (`spiral-a11y-linux`, `spiral-a11y-windows`, `spiral-a11y-macos`, `spiral-a11y-mobile`)?

3. **`forced-colors` vs `prefers-contrast`.** Row 37 covers both. Should Spiral's UA stylesheet react to either, or only to `forced-colors` (the WCAG-aligned choice)? Firefox and WebKit honour both, Chromium leans on `forced-colors`.

4. **Caption / AD track on `<video>`.** Rows 28–32 presume `<video>` lands in P4 (per chunk 1 row 35). If the media stack slips to P5, do caption rows slip too, or is a stub `<track>` parser still in scope for P4 so the text format is testable in isolation?

5. **`reading-flow` CSS.** Row 44 is shipping only in Chromium 121+ and Firefox 125+; WebKit has not committed. Given the WebKit gap and the M4 design constraint of cross-engine spec coverage, is `reading-flow` in scope for Spiral or tracked as a future-only row?

6. **PDF accessibility (row 42).** No PDF renderer exists in Spiral. Should tagged-PDF support be a hard requirement of the first PDF renderer, or tracked as a separate ADR (`0000-PDF-renderer.md`)?

7. **Switch / Voice / Eye control.** Rows 32 and 43 depend on chunk 6 (Speech / Web Speech) and chunk 6 (WebXR). Are these scoped into the a11y delivery, or do they ride the Speech / XR deliveries and a11y just exposes the focus hooks?
