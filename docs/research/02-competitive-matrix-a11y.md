# Competitive Matrix — Domain: Accessibility & Internationalisation

**File:** `02-competitive-matrix-a11y.md`
**Date:** 2026-06-16
**Sources:** `09-accessibility-i18n.md`, `09-i18n-engine.md`
**Methodology:** `00-methodology.md`

## Column legend

- **Status in Spiral:** `shipped` / `partial` / `designed` / `not-started` / `do-not-touch`
- **Prevalence:** `ubiquitous` (>95%) / `widespread` (70–95%) / `mixed` (two+ engines, at least one no) / `niche` (one engine) / `experimental` (flag-only) / `legacy` (deprecated)
- **Phase:** per `00-methodology.md` §5
- **Complexity:** `S` / `M` / `L` / `XL`
- **Engine columns:** `yes` / `partial` / `no` / `behind-flag`

---

## §1 ARIA reflection on the DOM

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 1 | `role` attribute and ARIAMixin / ElementInternals reflection | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 2 | `aria-*` IDL properties (full ARIAMixin set) | desktop+mobile+embedded | not-started | ubiquitous | P5 | XL | yes | yes | yes | partial | no | no |
| 3 | Accessible-name computation (aria-label, aria-labelledby, aria-describedby) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 4 | `aria-live` polite/assertive + atomic + relevant | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 5 | Implicit ARIA semantics per HTML element | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | partial | no | no |
| 6 | Landmark roles (banner, main, navigation, contentinfo, etc.) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 7 | `role=alert` / `role=status` / `role=log` / `role=timer` (implicit aria-live) | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | yes | yes | yes | partial | no | no |
| 8 | `aria-invalid` / `aria-errormessage` / `aria-required` form-state plumbing | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 9 | `aria-grabbed` / `aria-dropeffect` (legacy) + HTML5 DnD a11y | desktop+mobile+embedded | not-started | mixed | P6 | M | yes | partial | partial | no | no | no |
| 10 | `aria-controls` / `aria-flowto` / `aria-details` element references | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |

## §2 Accessibility tree + platform bridges

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 11 | Accessibility tree construction (parallel to render tree) | desktop+mobile+embedded | not-started | ubiquitous | P5 | XL | yes | yes | yes | partial | no | no |
| 12 | AT-SPI2 bridge (Linux, D-Bus) | desktop+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | no | no | no |
| 13 | UIA / MSAA bridge (Windows) | desktop | not-started | ubiquitous | P5 | L | yes | yes | yes | no | no | no |
| 14 | NSAccessibility bridge (macOS) | desktop | not-started | ubiquitous | P5 | M | yes | yes | yes | no | no | no |
| 15 | AX iOS / iPadOS bridge + Android AccessibilityNodeProvider | mobile | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no | no |
| 16 | ChromeVox-style in-process screen reader | desktop | not-started | niche | P6 | XL | yes | no | no | no | no | no |
| 17 | Accessibility event mutation queue + coalesced notifications | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |

## §3 Keyboard, focus, and chrome a11y

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 18 | Sequential focus navigation (Tab/Shift+Tab, tabindex) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | stub | no |
| 19 | `:focus-visible` CSS pseudo-class | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | no | no |
| 20 | `:focus-within` CSS pseudo-class | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | no | no |
| 21 | Focus trap inside modal `<dialog>` + inert subtree | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | no | no |
| 22 | Focus restoration (back/forward, dialog dismiss, route change) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | no | no |
| 23 | Skip-to-content links + browser keyboard shortcuts | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | no | no |
| 24 | Focus rings (UA default + :focus-visible outline) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | no | no |
| 25 | `inert` attribute + IDL property | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no | no |
| 26 | Spacing / hit-target enlargement for small controls | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no | no |
| 27 | Pointer-cancellation semantics (pointercancel, touch-action) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | no | no |

## §4 Media, captions, and audio description

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 28 | `<track kind="subtitles\|captions\|descriptions\|chapters">` + TextTrack | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | yes | yes | yes | partial | no | no |
| 29 | WebVTT rendering (cue settings, CSS extensions) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | partial | no | no |
| 30 | Native caption settings UI (position, colour, font, size) | desktop+mobile+embedded | not-started | widespread | P5 | M | yes | yes | yes | no | no | no |
| 31 | Audio-description track selection (AD button on video) | desktop+mobile+embedded | not-started | widespread | P5 | M | yes | yes | yes | no | no | no |
| 32 | Sign-language overlay via PiP of sign-language stream | desktop+mobile | not-started | mixed | P6 | L | yes | yes | yes | no | no | no |

## §5 User-preference media queries

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 33 | `prefers-color-scheme` + `color-scheme` CSS property | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | no | no |
| 34 | `light-dark()` CSS function + color-scheme interaction | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no | no |
| 35 | `prefers-reduced-motion` + `prefers-reduced-transparency` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | no | no |
| 36 | `Save-Data` header + `prefers-reduced-data` media query | desktop+mobile+embedded | not-started | mixed | P4 | S | yes | partial | partial | no | no | no |
| 37 | `prefers-contrast` + `forced-colors` + `forced-color-adjust` | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no | no |
| 38 | `prefers-reduced-motion` honoured for smooth scroll + View Transitions | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no | no |

## §6 Chrome a11y + mobile a11y

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 39 | Chrome a11y (keyboard shortcuts, ? help, focusable URL bar, tab-to-search) | desktop | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 40 | Page zoom (Ctrl+/-), text-only zoom, layout reflow at 200%/400% | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | no | no |
| 41 | Mobile pinch-zoom, double-tap-to-zoom, dynamic type (iOS), font scale (Android) | mobile | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 42 | PDF accessibility (tagged PDF, reading order, alt text, structural tree) | desktop+mobile | not-started | ubiquitous | P7 | XL | yes | yes | yes | no | no | no |
| 43 | Voice-control + Switch Control integration | desktop+mobile | not-started | widespread | P6 | L | yes | partial | yes | no | no | no |
| 44 | `reading-flow` CSS (book, flex, grid, normal) | desktop+mobile+embedded | not-started | niche | P6 | M | yes | yes | no | no | no | no |

## §7 Locale detection and negotiation

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 45 | `navigator.language` / `navigator.languages` (BCP 47) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | stub | no |
| 46 | Accept-Language header construction (RFC 9110 quality values) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | stub | no |
| 47 | `<html lang>` / `xml:lang` attribute on HTMLElement.lang | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | no | no |
| 48 | `Intl.Locale` constructor (tag parsing, getters) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |
| 49 | Per-resource locale negotiation (Sec-CH-Lang, Content-Language) | desktop+mobile | not-started | mixed | P6 | M | yes | partial | yes | no | no | no |

## §8 Intl.* API surface

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 50 | `Intl.NumberFormat` (decimal/currency/percent/unit, compact, signDisplay) | desktop+mobile+embedded | not-started | ubiquitous | P5 | XL | yes | yes | yes | yes | no | no |
| 51 | `Intl.DateTimeFormat` (locale, calendar, dateStyle/timeStyle, timeZone) | desktop+mobile+embedded | not-started | ubiquitous | P5 | XL | yes | yes | yes | yes | no | no |
| 52 | `Intl.RelativeTimeFormat` (long/short/narrow, always/auto) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |
| 53 | `Intl.ListFormat` (conjunction/disjunction/unit, long/short/narrow) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |
| 54 | `Intl.Segmenter` (grapheme/word/sentence iteration) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | no | no | no |
| 55 | `Intl.DisplayNames` (language/region/script/currency/calendar) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |
| 56 | `Intl.PluralRules` (cardinal/ordinal, select) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |
| 57 | `Intl.Collator` (sort/search, sensitivity, caseFirst, numeric) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |
| 58 | `Intl.supportedValuesOf(key)` | desktop+mobile+embedded | not-started | widespread | P5 | S | yes | yes | yes | no | no | no |
| 59 | `Intl.getCanonicalLocales` + default locale fallback | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | yes | yes | yes | yes | no | no |

## §9 Calendar and time zone

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 60 | Calendar systems (Gregorian, Buddhist, Chinese, Hebrew, Islamic, Japanese, Persian, ROC, etc.) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | yes | no | no |
| 61 | IANA time zone database (DST, historical transitions, leap seconds) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | yes | no | no |
| 62 | `<input type="date\|time\|datetime-local\|month\|week">` locale-aware picker | desktop+mobile | not-started | ubiquitous | P4 | XL | yes | yes | yes | no | no | no |
| 63 | `Intl.DateTimeFormat` era display (Reiwa, Heisei, BCE/CE, AH/BH, BE) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |

## §10 Text shaping, bidi, vertical text

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 64 | Bidi algorithm (UAX #9) — paragraph/inline resolution, isolates, mirror | desktop+mobile+embedded | not-started | ubiquitous | P4 | XL | yes | yes | yes | yes | no | no |
| 65 | CSS `direction: rtl\|ltr`, `unicode-bidi`, `<bdi>` / `<bdo>` | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | no | no |
| 66 | `dir="auto"` heuristic on HTML and per-element | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | no | no |
| 67 | Vertical typesetting (writing-mode, text-orientation) | desktop+mobile+embedded | not-started | ubiquitous | P6 | XL | yes | yes | yes | partial | no | no |
| 68 | Line breaking (UAX #14) + CSS line-break, word-break, hyphens, text-wrap | desktop+mobile+embedded | not-started | ubiquitous | P4 | XL | yes | yes | yes | yes | no | no |
| 69 | East Asian Width (UAX #11) — fullwidth/halfwidth glyph metrics | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |
| 70 | Emoji sequences (UTS #51) — ZWJ, regional indicators, Fitzpatrick, variation selectors | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |
| 71 | Complex text layout (Indic/Arabic/Hebrew/Thai/Khmer shaping, HarfBuzz-equivalent) | desktop+mobile+embedded | not-started | ubiquitous | P5 | XL | yes | yes | yes | yes | no | no |

## §11 Normalisation, encoding, IDNA

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 72 | Unicode normalisation (NFC/NFD/NFKC/NFKD, String.prototype.normalize) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | no | no |
| 73 | IDNA 2008 + UTS #46 for hostnames in URL | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | no | no |
| 74 | Character encoding detection (BOM, `<meta charset>`, Content-Type, prescan) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | no | no |
| 75 | Default output encoding UTF-8 (TextEncoder/TextDecoder) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | no | no |

## §12 Page translation and locale-aware input

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 76 | Built-in page translation (Translator/LanguageDetector API, W3C) | desktop+mobile | not-started | mixed | P6 | XL | behind-flag | no | no | no | no | no |
| 77 | Locale-aware `<input type="number">` (decimal separator per locale) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no | no |
| 78 | Locale-aware currency display (Intl.NumberFormat with currency style) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |

## §13 Spellcheck, IME, keyboard

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 79 | Browser spellcheck (multi-language dictionary, custom dict sync) | desktop+mobile | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no | no |
| 80 | IME integration (CJK, composition events, dead keys, candidate window) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | partial | no | no |
| 81 | Locale-aware keyboard layout switch (per-element via lang) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 82 | Compose-key / dead-key sequence resolution | desktop+mobile | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |

## §14 Locales, fonts, and culture

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 83 | Locale-specific quotation marks (via lang + quotes CSS) | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | yes | yes | yes | yes | no | no |
| 84 | Locale-specific indentation (text-indent, ideographic space) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | no | no |
| 85 | Locale-aware case mapping (Turkish dotless i, German ß, Greek final sigma) | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | yes | yes | yes | yes | no | no |
| 86 | Locale-aware sort/search (Intl.Collator, localeCompare) | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | yes | yes | yes | yes | no | no |
| 87 | Locale-specific font fallback (system-ui, Noto Sans CJK, PingFang, etc.) | desktop+mobile | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | no | no |
| 88 | OpenType features (ligatures, kerning, variable fonts, font-variant-*) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | yes | no | no |
| 89 | Inter-ideograph/inter-word/inter-cluster justification (text-justify) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | no | no | no |
| 90 | Locale fallback chain (system → requested → base, CLDR substitution) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | no | no |

---

**Total rows: 90** (44 a11y + 46 i18n)
