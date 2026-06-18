# Chunk 9 (ii) — Internationalisation Engine

> **Companion to `09-accessibility-i18n.md`.** This file is the
> internationalisation half of chunk 9. The accessibility half is
> `09-accessibility-i18n.md` (rows 1–44). This file covers rows 45–90.
>
> **Scope:** the engine + chrome surface that lets users in non-English
> locales, with non-Latin scripts, with non-Gregorian calendars, with
> non-Western number / currency conventions, and with non-LTR
> direction actually use the web. Backing is ICU + CLDR via the `icu`
> Rust crate. Per the methodology, this is the compliance surface for
> EU Accessibility Act + EAA, JIS X 8341, Section 508 (i18n clauses),
> and Unicode conformance (UAX #9, #11, #14, #29, #39, #46; UTS #46;
> UTS #51).

---

## §2 Internationalisation

### §2.1 Locale detection and negotiation

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 45 | `navigator.language` / `navigator.languages` (BCP 47 priority list) | desktop+mobile+embedded | not-started (no `navigator` in `spiral-vortex`) | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: stub · Flow: no | [HTML §dom-navigator-language](https://html.spec.whatwg.org/multipage/system-state.html#dom-navigator-language) ; [RFC 9110 §8.5 (Accept-Language)](https://www.rfc-editor.org/rfc/rfc9110#field.accept-language) |
| 46 | Accept-Language header construction from `navigator.languages` (RFC 9110 §8.5 quality values) | desktop+mobile+embedded | not-started | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: stub · Flow: no | [RFC 9110 §8.5](https://www.rfc-editor.org/rfc/rfc9110#field.accept-language) ; [Fetch §accept-language](https://fetch.spec.whatwg.org/#http-network-fetch) |
| 47 | `<html lang>` / `xml:lang` attribute on `HTMLElement.lang` / `HTMLElement.translate` | desktop+mobile+embedded | not-started (no `<html>` element type yet; chunk 1 row 1 partial) | >=90% | P3 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [HTML §htmleelement](https://html.spec.whatwg.org/multipage/dom.html#htmlelement) ; [BCP 47](https://www.rfc-editor.org/info/bcp47) |
| 48 | `Intl.Locale` constructor (tag parsing, base-name, script, region, variant, extension, language / script / region getters) | desktop+mobile+embedded | not-started (no `Intl` in Vortex builtins) | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §12 Locale](https://tc39.es/ecma402/#locale-objects) ; [BCP 47](https://www.rfc-editor.org/info/bcp47) |
| 49 | Per-resource locale negotiation via `Sec-CH-Lang` / `Content-Language` round-trip and per-subdomain / per-path locale variants | desktop+mobile | not-started | 50–75% (Accept-Language only) | P6 | M | Chromium: yes (client hints) · Firefox: partial · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [RFC 8707 (Accept-Language negotiation)](https://www.rfc-editor.org/rfc/rfc8707) ; [Client Hints §lang](https://wicg.github.io/client-hints-infrastructure/) |

### §2.2 `Intl.*` API surface

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 50 | `Intl.NumberFormat` (locale, numbering system, `style`: decimal/currency/percent/unit, `currency`, `currencyDisplay`, `currencySign`, `minimumFractionDigits`, `maximumFractionDigits`, `minimumSignificantDigits`, `maximumSignificantDigits`, `useGrouping`, `notation`: standard/compact/scientific/engineering, `compactDisplay`, `signDisplay`: auto/always/never/exceptZero, `localeMatcher`, `roundingMode`, `roundingPriority`, `trailingZeroDisplay`) | desktop+mobile+embedded | not-started | >=90% | P5 | XL | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (rust-icu) · Ladybird: no · Flow: no | [ECMA-402 §15 NumberFormat](https://tc39.es/ecma402/#numberformat-objects) ; [UTS #35 §6 Number Elements](https://unicode.org/reports/tr35/tr35-numbers.html) |
| 51 | `Intl.DateTimeFormat` (locale, calendar, numbering system, `dateStyle` / `timeStyle`, `localeMatcher`, `timeZone`, `hour12`, `hourCycle`, `formatMatcher`, `weekday`, `era`, `year`, `month`, `day`, `dayPeriod`, `hour`, `minute`, `second`, `fractionalSecondDigits`, `timeZoneName`, `format`) | desktop+mobile+embedded | not-started | >=90% | P5 | XL | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §12 DateTimeFormat](https://tc39.es/ecma402/#datetimeformat-objects) ; [UTS #35 §4 Dates](https://unicode.org/reports/tr35/tr35-dates.html) |
| 52 | `Intl.RelativeTimeFormat` (locale, `style`: long/short/narrow, `numeric`: always/auto) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §16 RelativeTimeFormat](https://tc39.es/ecma402/#relativetimeformat-objects) |
| 53 | `Intl.ListFormat` (locale, `style`: long/short/narrow, `type`: conjunction/disjunction/unit) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §18 ListFormat](https://tc39.es/ecma402/#listformat-objects) |
| 54 | `Intl.Segmenter` (locale, `granularity`: grapheme / word / sentence) for text iteration | desktop+mobile+embedded | not-started | >=90% | P5 | L | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [ECMA-402 §19 Segmenter](https://tc39.es/ecma402/#segmenter-objects) ; [UAX #29 Text Segmentation](https://unicode.org/reports/tr29/) |
| 55 | `Intl.DisplayNames` (locale, `style`: long/short/narrow, `type`: language / region / script / currency / calendar / dateTimeField) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §13 DisplayNames](https://tc39.es/ecma402/#displaynames-objects) |
| 56 | `Intl.PluralRules` (locale, `type`: cardinal/ordinal, `minimumIntegerDigits`, `fractionDigits`, `significantDigits`, `roundingMode`, select) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §17 PluralRules](https://tc39.es/ecma402/#pluralrules-objects) ; [UTS #35 §6 Plural Rules](https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules) |
| 57 | `Intl.Collator` (locale, `usage`, `sensitivity`: base / accent / case / variant, `caseFirst`, `numeric`, `collation`, `ignorePunctuation`, `localeMatcher`) for sort and search | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §14 Collator](https://tc39.es/ecma402/#collator-objects) ; [UTS #10 Collation](https://unicode.org/reports/tr10/) |
| 58 | `Intl.supportedValuesOf(key)` (currency, calendar, collation, hourCycle, numberingSystem, timeZone, unit) | desktop+mobile+embedded | not-started | 75–90% | P5 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [ECMA-402 §20 SupportedValuesOf](https://tc39.es/ecma402/#sec-intl.supportedvaluesof) |
| 59 | `Intl.getCanonicalLocales(locales)` + default locale fallback (system locale → base locale) | desktop+mobile+embedded | not-started | >=90% | P5 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §8.2.2 CanonicalizeLocaleList](https://tc39.es/ecma402/#sec-canonicalizelocalelist) |

### §2.3 Calendar and time zone

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 60 | Calendar systems: Gregorian, Buddhist, Chinese, Coptic, Dangi, Ethiopic, Ethiopic-Amete-Alem, Hebrew, Indian, Islamic (civil, tabular, Umm-al-Qura variants), Japanese, Persian, ROC | desktop+mobile+embedded | not-started | >=90% | P5 | L | Chromium: yes (CLDR) · Firefox: yes (CLDR) · WebKit: yes (CLDR) · Servo: yes (ICU) · Ladybird: no · Flow: no | [UTS #35 §4.4 Calendars](https://unicode.org/reports/tr35/tr35-dates.html#Calendars) ; [CLDR calendars.xml](https://github.com/unicode-org/cldr/blob/main/common/bcp47/calendar.xml) |
| 61 | IANA time zone database (`tzdata` / `zoneinfo`) via ICU `TimeZone` (DST, historical transitions, leap seconds) | desktop+mobile+embedded | not-started | >=90% | P5 | L | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (chrono-tz) · Ladybird: no · Flow: no | [IANA tz database](https://www.iana.org/time-zones) ; [UTS #35 §4.5 Time Zones](https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology) |
| 62 | `<input type="date|time|datetime-local|month|week">` locale-aware picker (alternate calendar, era, time zone where the spec allows) | desktop+mobile | not-started (chunk 1 row 43 partial) | >=90% | P4 | XL | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [HTML §date-state](https://html.spec.whatwg.org/multipage/input.html#date-state-(type=date)) |
| 63 | `Intl.DateTimeFormat` era display (Reiwa, Heisei, Showa, ROC, BCE/CE, AH/BH, BE) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [UTS #35 §4.4.1 Eras](https://unicode.org/reports/tr35/tr35-dates.html#dfst-era) ; [CLDR era names](https://www.unicode.org/cldr/charts/latest/supplemental/calendarData.html) |

### §2.4 Text shaping, bidi, vertical text

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 64 | Bidi algorithm (UAX #9) — paragraph + inline resolution, isolates, embeddings, overrides, mirror glyphs | desktop+mobile+embedded | not-started (Gyre has no bidi pass) | >=90% | P4 | XL | Chromium: yes (ICU BiDi) · Firefox: yes (ICU BiDi) · WebKit: yes (custom + ICU) · Servo: yes (unicode-bidi crate) · Ladybird: no · Flow: no | [UAX #9 Unicode Bidirectional Algorithm](https://unicode.org/reports/tr9/) ; [CSS Writing Modes 4 §bidi](https://www.w3.org/TR/css-writing-modes-4/#bidi) |
| 65 | CSS `direction: rtl\|ltr`, `unicode-bidi` (normal / embed / isolate / bidi-override / plaintext), `<bdi>` / `<bdo>` elements | desktop+mobile+embedded | not-started | >=90% | P4 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [CSS Writing Modes 4 §direction](https://www.w3.org/TR/css-writing-modes-4/#direction) ; [HTML §bdi](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-bdi-element) |
| 66 | `dir="auto"` heuristic on `<html>` and per-element | desktop+mobile+embedded | not-started | >=90% | P4 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [HTML §the-dir-attribute](https://html.spec.whatwg.org/multipage/dom.html#the-dir-attribute) |
| 67 | Vertical typesetting (`writing-mode: vertical-rl\|vertical-lr\|horizontal-tb\|sideways-rl\|sideways-lr`, `text-orientation: mixed\|upright\|sideways`) | desktop+mobile+embedded | not-started (Gyre layout is single-axis) | >=90% | P6 | XL | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [CSS Writing Modes 4](https://www.w3.org/TR/css-writing-modes-4/) |
| 68 | Line breaking (UAX #14) + CSS `line-break: auto\|loose\|normal\|strict\|anywhere`, `word-break: normal\|break-all\|keep-all`, `overflow-wrap: normal\|break-word\|anywhere`, `hyphens: none\|manual\|auto`, `white-space-collapse`, `text-wrap: balance\|pretty\|stable` | desktop+mobile+embedded | not-started | >=90% | P4 | XL | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (line-break + hyphenation) · Ladybird: no · Flow: no | [UAX #14 Line Breaking](https://unicode.org/reports/tr14/) ; [CSS Text 4](https://www.w3.org/TR/css-text-4/) |
| 69 | East Asian Width (UAX #11) — fullwidth / halfwidth / wide / narrow / halfwidth-kana glyph metrics | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (unicode-width crate) · Ladybird: no · Flow: no | [UAX #11 East Asian Width](https://unicode.org/reports/tr11/) |
| 70 | Emoji sequences (UTS #51) — ZWJ, regional indicators, Fitzpatrick modifiers, keycap, tag sequences, variation selectors | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes (emoji-data + variation selectors) · Firefox: yes · WebKit: yes · Servo: yes (unicode-emoji crate) · Ladybird: no · Flow: no | [UTS #51 Emoji](https://unicode.org/reports/tr51/) ; [Emoji Variation Sequences](https://www.unicode.org/Public/UCD/latest/ucd/emoji/emoji-variation-sequences.txt) |
| 71 | Complex text layout: Indic / Arabic / Hebrew / Thai / Khmer shaping (HarfBuzz-equivalent — base + mark + reordering) | desktop+mobile+embedded | not-started (Render uses `font.rs` ASCII-only path) | >=90% | P5 | XL | Chromium: yes (HarfBuzz) · Firefox: yes (HarfBuzz) · WebKit: yes (custom + HarfBuzz) · Servo: yes (rustybuzz) · Ladybird: no · Flow: no | [OpenType Specification](https://learn.microsoft.com/en-us/typography/opentype/spec/) ; [HarfBuzz](https://harfbuzz.github.io/) |

### §2.5 Normalisation, encoding, IDNA

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 72 | Unicode normalisation forms: NFC, NFD, NFKC, NFKD (`String.prototype.normalize`, URL pathname, IDL) | desktop+mobile+embedded | not-started (no `String.prototype.normalize`) | >=90% | P4 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (unicode-normalization crate) · Ladybird: no · Flow: no | [UAX #15 Unicode Normalization Forms](https://unicode.org/reports/tr15/) ; [ECMA-262 §21.1.3.13](https://tc39.es/ecma262/#sec-string.prototype.normalize) |
| 73 | IDNA 2008 + UTS #46 processing for hostnames in `URL` (mapping, normalisation, Bidi rules, contextual rules, length limits) | desktop+mobile+embedded | not-started | >=90% | P4 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (idna crate) · Ladybird: no · Flow: no | [UTS #46 Unicode IDNA Compatibility Processing](https://unicode.org/reports/tr46/) ; [RFC 5890–5894 (IDNA 2008)](https://www.rfc-editor.org/rfc/rfc5890) ; [URL §host-parsing](https://url.spec.whatwg.org/#host-parsing) |
| 74 | Character encoding detection (BOM, `<meta charset>`, `Content-Type: charset=`, prescan) for HTML, CSS, JS, XML | desktop+mobile+embedded | not-started (chunk 1 documents this as deferred; see `01-feature-inventory-html-dom-js.md`) | >=90% | P3 | L | Chromium: yes (ICU + heuristic) · Firefox: yes (chardetng) · WebKit: yes (custom) · Servo: yes (encoding_rs) · Ladybird: no · Flow: no | [Encoding §decode](https://encoding.spec.whatwg.org/#decode) ; [HTML §encoding](https://html.spec.whatwg.org/multipage/parsing.html#character-encodings) |
| 75 | Default output encoding UTF-8 (always; `TextDecoder` UTF-8 / UTF-16 / single-byte fallbacks) | desktop+mobile+embedded | not-started (no `TextEncoder` / `TextDecoder`) | >=90% | P4 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (encoding_rs) · Ladybird: no · Flow: no | [Encoding §interface](https://encoding.spec.whatwg.org/#interface-textdecoder) ; [Encoding §textdecoder](https://encoding.spec.whatwg.org/#textdecoder) |

### §2.6 Page translation and locale-aware input

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 76 | Built-in page translation (cloud-backed; UI in chunk 7). Here: the `Translator` / `LanguageDetector` API surface (W3C) | desktop+mobile | not-started (chunk 7 owns the UI; no `Translator` API) | 75–90% (UI is everywhere; JS API experimental) | P6 | XL | Chromium: yes (UI + `Translator` API behind flag) · Firefox: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Translator and Language Detector API](https://github.com/webmachinelearning/translation-api) ; [W3C WebNN-ML](https://www.w3.org/TR/webnn/) |
| 77 | Locale-aware `<input type="number">` (decimal separator per locale, step validation) | desktop+mobile+embedded | not-started | >=90% | P4 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [HTML §number-state](https://html.spec.whatwg.org/multipage/input.html#number-state-(type=number)) ; [BCP 47 regional subtags](https://www.rfc-editor.org/info/bcp47) |
| 78 | Locale-aware currency display (`Intl.NumberFormat` with `style: "currency"`, `currencyDisplay: "code"\|"symbol"\|"narrowSymbol"\|"name"`, `currencySign: "standard"\|"accounting"`) | desktop+mobile+embedded | not-started (row 50 not-started) | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-402 §15.5 Currency](https://tc39.es/ecma402/#sec-intl-numberformat-constructor) ; [UTS #35 §6.3 Currency Symbols](https://unicode.org/reports/tr35/tr35-numbers.html#Currencies) |

### §2.7 Spellcheck, IME, keyboard

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 79 | Browser spellcheck (`<textarea spellcheck>`, `HTMLElement.spellcheck` IDL, contextual squiggle) with multi-language dictionary + custom dictionary sync | desktop+mobile | not-started | >=90% | P6 | L | Chromium: yes (Hunspell + OS dictionary) · Firefox: yes (Hunspell) · WebKit: yes (NSSpellChecker / autocorrect) · Servo: no · Ladybird: no · Flow: no | [HTML §spelling-and-grammar-checking](https://html.spec.whatwg.org/multipage/interaction.html#spelling-and-grammar-checking) ; [Hunspell](https://hunspell.github.io/) |
| 80 | IME (Input Method Editor) integration for CJK, Thai, Khmer, Indic, Arabic — composition events (`compositionstart`, `compositionupdate`, `compositionend`), dead keys, candidate window | desktop+mobile+embedded | not-started (Vortex has no event-loop CompositionEvent path yet) | >=90% | P5 | L | Chromium: yes · Firefox: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [UI Events §event-type-compositionstart](https://www.w3.org/TR/uievents/#event-type-compositionstart) ; [Input Method Editor API (W3C i18n WG)](https://www.w3.org/International/wiki/IME) |
| 81 | Locale-aware keyboard layout switch (per-element via `lang`, system-level layout picker) | desktop+mobile+embedded | not-started (OS-owned; browser exposes hooks only) | >=90% | P6 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C i18n: Keyboard layouts](https://www.w3.org/International/articles/inline-bidi-markup/) ; [UI Events §keys](https://www.w3.org/TR/uievents/#keys) |
| 82 | Compose-key / dead-key sequence resolution (`XCompose`, macOS Option-key, Windows AltGr) | desktop+mobile | not-started (OS-owned) | >=90% | P6 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [XCompose (X11)](https://www.x.org/releases/X11R7.7/doc/libX11/compose/en_US.UTF-8/Compose.gz) ; [Apple Compose](https://developer.apple.com/library/archive/technotes/tn2050/) |

### §2.8 Locales, fonts, and culture

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 83 | Locale-specific quotation marks (`"…"`, `'…'`, `«…»`, `「…」`, `『…』`, `「…」`, `「…」`, `⹁…`) via `lang` and `quotes` CSS (`content: open-quote`) | desktop+mobile+embedded | not-started | >=90% | P5 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [CSS Generated Content 3 §quotes](https://www.w3.org/TR/css-content-3/#quotes) ; [UTS #35 §5 Punctuation](https://unicode.org/reports/tr35/tr35-general.html#Punctuation) |
| 84 | Locale-specific indentation (`text-indent`, ideographic space, em-quad, em-space, figure space, hair space) | desktop+mobile+embedded | not-started | >=90% | P4 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [CSS Text 3 §text-indent-property](https://www.w3.org/TR/css-text-3/#text-indent-property) ; [UAX #11 §Intercharacter Space](https://unicode.org/reports/tr11/) |
| 85 | Locale-aware case mapping (Turkish dotless `ı`/`İ`, German `ß`/`SS`, Greek final sigma) | desktop+mobile+embedded | not-started (`String.prototype.toLocaleLowerCase` / `toLocaleUpperCase`) | >=90% | P5 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-262 §22.1.3.26](https://tc39.es/ecma262/#sec-string.prototype.tolocaleuppercase) ; [UTS #53 Segment Casings](https://unicode.org/reports/tr53/) ; [Unicode Case Folding](https://www.unicode.org/Public/UCD/latest/ucd/CaseFolding.txt) |
| 86 | Locale-aware sort and search (`Intl.Collator.compare` + `String.prototype.localeCompare`, with `caseFirst`, `numeric`, `ignorePunctuation`) | desktop+mobile+embedded | not-started | >=90% | P5 | S | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [ECMA-262 §22.1.3.28](https://tc39.es/ecma262/#sec-string.prototype.localecompare) ; [UTS #10 Collation](https://unicode.org/reports/tr10/) |
| 87 | Locale-specific font fallback (system font stack: `system-ui`, `-apple-system`, `BlinkMacSystemFont`, `Segoe UI`, `Roboto`, `Hiragino Sans`, `Noto Sans CJK`, `Microsoft YaHei`, `PingFang SC`) | desktop+mobile | not-started (Render uses one font) | >=90% | P4 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes · Ladybird: no · Flow: no | [CSS Fonts 4 §system-fonts](https://www.w3.org/TR/css-fonts-4/#system-fonts) ; [Noto CJK](https://github.com/notofonts/noto-cjk) |
| 88 | OpenType features (ligatures, kerning, `font-variant-emoji`, `font-variant-east-asian`, `font-variant-numeric`, `font-optical-sizing`, variable-font `font-variation-settings`) | desktop+mobile+embedded | not-started | >=90% | P5 | L | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (rustybuzz) · Ladybird: no · Flow: no | [CSS Fonts 4](https://www.w3.org/TR/css-fonts-4/) ; [OpenType feature tags](https://learn.microsoft.com/en-us/typography/opentype/spec/featurelist) ; [OpenType Variations](https://learn.microsoft.com/en-us/typography/opentype/spec/otvar/) |
| 89 | Inter-ideograph, inter-word, inter-cluster justification (`text-justify: auto\|inter-word\|inter-character\|inter-ideograph\|distribute`) | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [CSS Text 3 §text-justify-property](https://www.w3.org/TR/css-text-3/#text-justify-property) |
| 90 | Locale fallback chain (system locale → requested locale → base locale `en` / `und`) with graceful CLDR-region / CLDR-script substitution | desktop+mobile+embedded | not-started | >=90% | P5 | M | Chromium: yes · Firefox: yes · WebKit: yes · Servo: yes (icu_locid) · Ladybird: no · Flow: no | [UTS #35 §3 Matching](https://unicode.org/reports/tr35/tr35.html#Locale_Matching) ; [CLDR locale resolution](https://www.unicode.org/cldr/charts/latest/supplemental/locale_aliases.html) |

---

## Cross-cutting: ICU + CLDR backing

All 46 i18n rows above assume the engine can resolve a locale to the
appropriate Unicode data. The `icu` crate (Rust) provides this via
`icu_locid`, `icu_collator`, `icu_datetime`, `icu_decimal`,
`icu_list`, `icu_locale`, `icu_normalizer`, `icu_plurals`,
`icu_segmenter`, `icu_transliterator`, `icu_experimental` for newer
modules. CLDR data is shipped as the bundled `icu_cldr` data blob.
`specs/GAP_ANALYSIS.md` §4.5 marks ICU integration as M61–84; chunk 1
documents the absence of `Intl` as a P3–P5 gap.

For a Phase 2 / M9 base, the minimum viable i18n delivery is rows
45–47, 50, 51, 64–66, 72–74. Everything else slots in via P5+.

## Packet-to-row mapping (2026-06-18, ADR-0007)

The Phase 2 Steps 2.9–2.12 close the cheap and medium i18n wins per
[`docs/decisions/0007-i18n-table-stakes-bet.md`](../decisions/0007-i18n-table-stakes-bet.md).
Rows that are still "not-started" after Steps 2.9–2.12 land are
flagged below as `deferred-v0.2+`.

| Row | Capability | Packet | Crate | Dep |
|-----|------------|--------|-------|-----|
| 45 | `navigator.language` | 2.9.1 | `spiral-vortex` | (none) |
| 46 | `navigator.languages` (Accept-Language) | 2.9.1 | `spiral-vortex` | (none) |
| 47 | `<html lang>` / `xml:lang` | 2.9.2 | `spiral-dom` + `spiral-fmt` | (none) |
| 50 | `Intl.NumberFormat` | `deferred-v0.2+` (Phase 5) | — | `icu` crate (M61+) |
| 51 | `Intl.DateTimeFormat` | `deferred-v0.2+` (Phase 5) | — | `icu` crate |
| 64 | Bidi (UAX #9) | 2.10.1 | `spiral-gyre` | `unicode-bidi` |
| 65 | CSS `direction`, `unicode-bidi`, `<bdi>`, `<bdo>` | 2.10.1 | `spiral-gyre` | `unicode-bidi` |
| 66 | `dir="auto"` heuristic | 2.10.1 | `spiral-gyre` | `unicode-bidi` + `unicode-script` (via 2.11.2) |
| 68 | Line breaking (UAX #14), `line-break`, `word-break`, `overflow-wrap`, `hyphens` | 2.10.2 | `spiral-gyre` | `linebreak` |
| 69 | East Asian Width (UAX #11) | 2.10.4 | `spiral-gyre` | `unicode-width` |
| 71 | Complex text layout (Indic / Arabic / Hebrew / Thai / Khmer shaping) | 2.11.1 + 2.11.2 + 2.11.3 | `spiral-gyre` | `rustybuzz` (behind `harfbuzz` feature flag) + `unicode-script` |
| 72 | Unicode normalisation (NFC, NFD, NFKC, NFKD) | 2.10.3 | `spiral-vortex` | `unicode-normalization` |
| 73 | IDNA 2008 + UTS #46 for URL hostnames | 2.12.1 | `spiral-fmt` or `spiral-net` (TBD by 2.7.1) | `idna` |
| 74 | Character encoding detection (BOM, `<meta>`, `Content-Type`) | 2.9.3 | `spiral-fmt` | `encoding_rs` |
| 75 | Default output UTF-8 / `TextEncoder` / `TextDecoder` | 2.9.4 | `spiral-vortex` | (none — uses Vortex `String` UTF-8) |
| 48, 49, 52–63, 76–90 | All other i18n rows | `deferred-v0.2+` (Phase 5+) | — | `icu` crate family |

After Steps 2.9–2.12 ship, the matrix goes from "46 of 46 not-started"
to "12 not-started, all Phase 5+ work behind ICU integration." The
"deferred-v0.2+" rows are documented here for traceability; their
packets will be added to a future Phase 5 sub-Phase when the architectural
bet is proven.
45–47, 50, 51, 64–66, 72–74. Everything else slots in via P5+.

## Open questions for the user

1. **Intl namespace shape.** Is the plan a single `Intl` global object
   backed by `icu` crate, or a per-object import
   (`@spiral/intl/number-format`)? Single global matches ECMA-402 and
   all four engines; per-object is friendlier for tree-shaking but
   non-conformant.

2. **CLDR data size.** Bundling full CLDR (100+ MB unpacked) or the
   trimmed `icu_cldr_small` subset (1–2 MB)? Chromium ships full CLDR
   behind a build flag; Firefox trims to runtime-needed locales.
   Spiral's bundle-size position will set the trim profile.

3. **Emoji strategy.** Row 70 requires either an emoji-font dependency
   (Noto Color Emoji, 9 MB) or shipping Apple/Google/Microsoft
   glyphs. Chromium uses a multi-font fallback chain; Firefox uses
   Twemoji / system. What is Spiral's policy?

4. **HarfBuzz / rustbuzz / custom?** Row 71's CTL pass is a multi-month
   effort. Options: (a) FFI to system HarfBuzz; (b) pure-Rust
   `rustybuzz`; (c) custom MiniRust shaper. (a) is fastest path,
   (b) is portable, (c) honours the "no Taffy" precedent of Gyre.

5. **`<input type="date">` calendar choice (row 62).** Some users
   expect the native OS picker (locale-aware via OS) vs an in-browser
   picker (locale-aware via `Intl`). Firefox ships OS-picker-where-
   available, Chromium ships in-browser. What is Spiral's choice?

6. **Translation API (row 76).** The W3C Translator / Language
   Detector API is Chromium-only behind a flag. If Spiral ships page
   translation, do we expose a JS API surface, or keep translation as
   pure chrome UI (chunk 7) with no `Translator` global?

7. **Locale fallback priority (row 90).** Spiral's base locale is
   currently undefined. Should it be `en-US` (US English), `en-001`
   (World English), or `und` (undetermined → defer to OS)? This
   affects every locale negotiation downstream.
