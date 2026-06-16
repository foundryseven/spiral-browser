# Chunk 1 — Core Web Platform Standards Inventory: CSS

> **Companion to `01-feature-inventory-index.md` (the chunk header) and
> `01-feature-inventory-html-dom-js.md`.** This file covers the
> engine-level CSS surface: every property, selector, function,
> at-rule, value type, and unit in CSS Properties, Selectors, Values,
> Box Model, Backgrounds, Borders, Text, Fonts, Lists, Color,
> Compositing, Filter, Mask, Shapes, Contain, Cascade, and the
> high-priority drafts (View Transitions, Anchor Positioning,
> Scroll-driven Animations, Container Queries, `:has()`,
> `:focus-visible`, subgrid, masonry, `color-mix()`, `oklch()`,
> relative colour, `light-dark()`, `sibling-index()`,
> `nth-child(<of …>)`, `font-variant-emoji`, `text-wrap: balance`/
> `pretty`, `scroll-timeline`).
>
> **Style of rows:** CSS property = `S`, CSS selector engine work =
> `M`, major sub-system (Shadow DOM, Container Queries, View
> Transitions) = `L`, engine-grade (WASM, threads, SIMD) = `XL`.
> Per `00-methodology.md` §6 and `README.md` §"Per-chunk output
> contract".

---

## B.1 Selectors

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 1 | Type selector (`div`), universal (`*`) | all | partial (`spiral-fmt::css::selector::TypeSelector` exists; matcher not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#type-selectors ; https://developer.mozilla.org/en-US/docs/Web/CSS/Type_selectors |
| 2 | ID selector (`#foo`) | all | partial | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#id-selectors ; https://developer.mozilla.org/en-US/docs/Web/CSS/ID_selectors |
| 3 | Class selector (`.foo`) | all | partial | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#class-html |
| 4 | Attribute selector (`[attr]`, `[attr=value]`, `[attr~=value]`, `[attr|=value]`, `[attr^=value]`, `[attr$=value]`, `[attr*=value]`) with case sensitivity | all | partial (all six matchers in `spiral-fmt::css::selector::AttributeMatcher`; matcher not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#attribute-selectors ; https://developer.mozilla.org/en-US/docs/Web/CSS/Attribute_selectors |
| 5 | Combinators (descendant, child, next-sibling, subsequent-sibling) | all | partial (`Combinator` enum) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#combinators ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_selectors/Selectors_and_combinators |
| 6 | Selector list (`,` grouping) | all | partial (`SelectorList` struct) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#selector-list |
| 7 | `:hover`, `:focus`, `:active`, `:visited`, `:link`, `:checked`, `:disabled`, `:enabled`, `:default`, `:indeterminate`, `:valid`, `:invalid`, `:required`, `:optional`, `:in-range`, `:out-of-range`, `:read-only`, `:read-write`, `:placeholder-shown`, `:defined`, `:target`, `:current`, `:past`, `:future`, `:user-invalid`, `:user-valid` (UI state pseudo-classes) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#useraction-pseudos ; https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes |
| 8 | `:focus-visible`, `:focus-within` | all | not-started | >=90% (Baseline 2022) | P3 | S | all yes/stable | https://drafts.csswg.org/selectors-4/#the-focus-visible-pseudo ; https://developer.mozilla.org/en-US/docs/Web/CSS/:focus-visible |
| 9 | `:has(<relative-selector-list>)` (the relational pseudo-class) | all | not-started | 75–90% (Baseline 2023) | P3 | L | all yes/stable | https://drafts.csswg.org/selectors-4/#relational ; https://developer.mozilla.org/en-US/docs/Web/CSS/:has |
| 10 | `:is()`, `:where()`, `:not()` (the functional negation / matching / specificity-relaxing pseudo-classes) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#is-pseudo ; https://drafts.csswg.org/selectors-4/#where-pseudo ; https://developer.mozilla.org/en-US/docs/Web/CSS/:is |
| 11 | `:nth-child(an+b [of S]?)`, `:nth-last-child(an+b [of S]?)`, `:nth-of-type(an+b)`, `:nth-last-of-type(an+b)` (the `of S` quantifier on `nth-child` is the new 2023 form) | all | not-started | >=90% (basic); 75–90% (the `of S` form, Baseline 2024) | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#nth-child-pseudo ; https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-child |
| 12 | `:only-child`, `:only-of-type`, `:first-child`, `:last-child`, `:first-of-type`, `:last-of-type`, `:empty` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/selectors-4/#structural-pseudos ; https://developer.mozilla.org/en-US/docs/Web/CSS/:first-child |
| 13 | `:root`, `:scope` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/selectors-4/#root-pseudo ; https://drafts.csswg.org/selectors-4/#scope-pseudo |
| 14 | `:lang(<ident-or-string>)` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/selectors-4/#lang-pseudo ; https://developer.mozilla.org/en-US/docs/Web/CSS/:lang |
| 15 | `:dir(ltr | rtl)`, `:any-link` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/selectors-4/#dir-pseudo ; https://drafts.csswg.org/selectors-4/#any-link-pseudo |
| 16 | `:autofill`, `:blank`, `:user-invalid` (input states) | all | not-started | 75–90% (Baseline 2023) | P3 | S | all yes/stable | https://drafts.csswg.org/selectors-4/#user-pseudos ; https://developer.mozilla.org/en-US/docs/Web/CSS/:autofill |
| 17 | `::before`, `::after`, `::marker`, `::placeholder`, `::selection`, `::file-selector-button` (the standard pseudo-elements) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-pseudo-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/::before |
| 18 | `::backdrop` (for `<dialog>`) | all | not-started | >=90% | P3 | S | all yes/stable | https://fullscreen.spec.whatwg.org/#::backdrop-pseudo ; https://developer.mozilla.org/en-US/docs/Web/CSS/::backdrop |
| 19 | `::part(<ident-list>)` and `::slotted(<compound-selector>)` (the shadow-DOM pseudo-elements) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-shadow-parts-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/::part |
| 20 | Specificity calculation (A, B, C) and `!important` ordering | all | partial (`Specificity` struct in `spiral-fmt::css::specificity`; cascade not yet) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/selectors-4/#specificity ; https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity |
| 21 | `*` specificity override and `:where()` zeroing | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/selectors-4/#zero-matches |

## B.2 At-rules

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 22 | `@charset` (the document charset) | all | not-started (parser tokenises; meta-eval not built) | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-syntax-3/#charset-rule ; https://developer.mozilla.org/en-US/docs/Web/CSS/@charset |
| 23 | `@import` (`url(...)`, `<string>`, `layer`, `supports(...)`, `media(...)` qualifiers) | all | partial (parser yes; fetch + cycle detection not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-cascade-5/#at-import ; https://developer.mozilla.org/en-US/docs/Web/CSS/@import |
| 24 | `@namespace`, `@namespace url(...) <ident>`, default namespace | all | partial (parser yes) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-namespaces-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@namespace |
| 25 | `@media` (block form with full media query list) | all | partial (parser yes; match-eval not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/mediaqueries-5/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@media |
| 26 | `@supports` (`supports(decl-list)`, `supports(selector)`, `not`, `and`, `or`) | all | partial (parser yes; eval not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-conditional-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@supports |
| 27 | `@container` (block form with name, type / size / inline-size / block-size / aspect-ratio / style / scroll-state container queries) | all | not-started | 75–90% (Baseline 2023) | P3 | L | all yes/stable | https://drafts.csswg.org/css-conditional-5/ ; https://drafts.csswg.org/css-contain-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@container |
| 28 | `@layer` (block and statement forms) | all | partial (parser yes; cascade not) | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-cascade-5/#layering ; https://developer.mozilla.org/en-US/docs/Web/CSS/@layer |
| 29 | `@font-face` (with the full `src` `format()` list, `unicode-range`, `font-display`, `font-variation-settings`, `size-adjust`, `ascent-override`, `descent-override`, `line-gap-override`) | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-fonts-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face |
| 30 | `@font-feature-values` (`@stylistic`, `@historical-forms`, `@styleset`, `@character-variant`, `@swash`, `@ornaments`, `@annotation`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-fonts-4/#font-feature-values-prop ; https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values |
| 31 | `@keyframes`, `@-webkit-keyframes` (Vendor-prefix compatibility) | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-animations-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@keyframes |
| 32 | `@property` (`syntax`, `inherits`, `initial-value`) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-properties-values-api-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@property |
| 33 | `@counter-style` (`system`, `symbols`, `additive-symbols`, `negative`, `prefix`, `suffix`, `pad`, `range`, `speak-as`, `fallback`) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-counter-styles-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@counter-style |
| 34 | `@page` (page area, page margin boxes, `:first`, `:left`, `:right`, named pages, `size`) | all | not-started | 75–90% (paged media) | P3 | M | all yes/stable (used for print) | https://drafts.csswg.org/css-page-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@page |
| 35 | `@view-transition` (the document-level opt-in for view transitions) | all | not-started | 75–90% (Baseline 2024) | P3 | L | all yes/stable | https://drafts.csswg.org/css-view-transitions-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@view-transition |
| 36 | `@scroll-timeline` (the explicit-name scroll-timeline at-rule) | all | not-started | 50–75% (Chromium 115+; WebKit 17.4+; Gecko 132+) | P3 | L | mixed: Chromium/WebKit yes/stable; Gecko yes/stable (recent); Servo no | https://drafts.csswg.org/scroll-animations-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@scroll-timeline |
| 37 | `@position-try` (the anchor-positioning fallback-policy at-rule) | all | not-started | 25–50% (Chromium 125+) | P3 | L | behind flag in most engines; spec recently stabilised | https://drafts.csswg.org/css-anchor-position-1/ |

## B.3 Value types and units

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 38 | `<length>` (absolute: `px`, `in`, `cm`, `mm`, `pt`, `pc`, `Q`; relative: `em`, `ex`, `ch`, `rem`, `cap`, `ic`, `lh`, `rlh`, `vw`, `vh`, `vmin`, `vmax`, `vi`, `vb`, `svw`, `svh`, `lvw`, `lvh`, `dvw`, `dvh`, `cqw`, `cqh`, `cqi`, `cqb`, `cqmin`, `cqmax`) | all | partial (parser tokenises; resolution not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-values-4/#lengths ; https://developer.mozilla.org/en-US/docs/Web/CSS/length |
| 39 | `<angle>` (`deg`, `rad`, `grad`, `turn`) | all | partial | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-values-4/#angles ; https://developer.mozilla.org/en-US/docs/Web/CSS/angle |
| 40 | `<time>` (`s`, `ms`) | all | partial | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-values-4/#time ; https://developer.mozilla.org/en-US/docs/Web/CSS/time |
| 41 | `<frequency>` (`Hz`, `kHz`) | all | partial | >=90% | P3 | S | all yes/stable (used in `voice-pitch`) | https://drafts.csswg.org/css-values-4/#frequency ; https://developer.mozilla.org/en-US/docs/Web/CSS/frequency |
| 42 | `<resolution>` (`dpi`, `dpcm`, `dppx`, `x`) | all | partial | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-values-4/#resolution ; https://developer.mozilla.org/en-US/docs/Web/CSS/resolution |
| 43 | `<percentage>` (`%`) | all | partial | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-values-4/#percentages ; https://developer.mozilla.org/en-US/docs/Web/CSS/percentage |
| 44 | `<flex>` (the flex factor — a number) | all | partial | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#flex-common ; https://developer.mozilla.org/en-US/docs/Web/CSS/flex |
| 45 | `<integer>`, `<number>`, `<calc>` (the arithmetic on `<length>`/`<percentage>`/`<angle>`/`<time>`/`<number>`) | all | partial (`Value` enum has the basic forms; calc not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-values-4/#calc-type ; https://developer.mozilla.org/en-US/docs/Web/CSS/calc |
| 46 | `min()`, `max()`, `clamp()`, `round()`, `mod()`, `rem()`, `abs()`, `sign()`, `sin()`, `cos()`, `tan()`, `asin()`, `acos()`, `atan()`, `atan2()`, `pow()`, `sqrt()`, `hypot()`, `log()`, `exp()` (math functions) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-values-4/#math ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Functions |
| 47 | `color()` (with `srgb`, `srgb-linear`, `display-p3`, `a98-rgb`, `prophoto-rgb`, `rec2020`, `xyz`, `xyz-d50`, `xyz-d65` colour spaces) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-color-4/#color-function ; https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/color |
| 48 | `oklch()`, `oklab()`, `lch()`, `lab()` (the perceptually uniform colour functions) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-color-4/#the-oklch-notation ; https://developer.mozilla.org/en-US/docs/Web/CSS/oklch |
| 49 | `color-mix(in <color-space>, <color>, <color> [<percentage>])` (colour mixing) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-color-5/#color-mix ; https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/color-mix |
| 50 | Relative colour syntax (`oklch(from <color> l c h / a)`) | all | not-started | 50–75% (Baseline 2024) | P3 | M | all yes/stable | https://drafts.csswg.org/css-color-5/#relative-color ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_colors/Relative_colors |
| 51 | `light-dark(<light>, <dark>)` (the preferred-color-scheme colour pairing) | all | not-started | 75–90% (Baseline 2024) | P3 | M | all yes/stable | https://drafts.csswg.org/css-color-5/#light-dark ; https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/light-dark |
| 52 | `attr(<ident> <type-or-unit> [, <fallback>])` (the typed attribute value) | all | not-started | 50–75% (Baseline 2023, advanced types still in progress) | P3 | M | all yes/stable (limited types) | https://drafts.csswg.org/css-values-5/#attr-notation ; https://developer.mozilla.org/en-US/docs/Web/CSS/attr |
| 53 | `url()`, `src()` (the unified src function for `@font-face`), `image()` (`<image>`, `<color>`, `image-set()`, `cross-fade()`) | all | partial (parser tokenises `url()`; rest not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-values-4/#url-value ; https://drafts.csswg.org/css-images-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/url |
| 54 | `var()`, `env()` (the custom-property substitution and the UA-environment lookup) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-variables-1/ ; https://drafts.csswg.org/css-env-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/var |
| 55 | `counter()`, `counters()`, `content()` (the generated-content counters) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-lists-3/#counter-functions ; https://developer.mozilla.org/en-US/docs/Web/CSS/counter |
| 56 | `random()`, `random-item()` (the procedural-content functions) | all | not-started | <25% (Chromium 139+ behind flag) | P5 | M | behind flag; very recent | https://drafts.csswg.org/css-content-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/random |
| 57 | `toggle()`, `lang()`, `first-valid()` (the legacy value functions) | all | not-started | <25% (mostly historical / Annex B) | P3 | M | Gecko yes/stable; others no | https://drafts.csswg.org/css-values-4/#value-functions |
| 58 | `sibling-index()`, `sibling-count()` (the structural-selector-driven value functions) | all | not-started | 25–50% (Chromium 137+) | P3 | M | behind flag; very recent | https://drafts.csswg.org/css-values-5/#sibling-index ; https://developer.mozilla.org/en-US/docs/Web/CSS/sibling-index |

## B.4 Box model and layout — display, sizing, position

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 59 | `display` (the multi-keyword form: `block flow`, `inline flow`, `inline-block flow`, `flex flow`, `grid flow`, `inline-flex flow`, `inline-grid flow`, `flow-root flow`, `table`, `table-row`, `table-cell`, `table-caption`, `list-item`, `none`, `contents`, `inline flow-root`, etc.) | all | not-started (Gyre partial) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-display-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/display |
| 60 | `display: contents` (the disappearing-parent box) | all | not-started | 75–90% (Baseline 2023, after a11y re-fix) | P3 | M | all yes/stable | https://drafts.csswg.org/css-display-3/#box-generation ; https://developer.mozilla.org/en-US/docs/Web/CSS/display |
| 61 | `display: list-item` (`list-style-type`, `list-style-position`, `list-style-image`, `::marker`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-lists-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/display |
| 62 | `position` (`static`, `relative`, `absolute`, `fixed`, `sticky`) | all | not-started (Gyre partial) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-position-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/position |
| 63 | `top`, `right`, `bottom`, `left` (offsetting for `position: non-static`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-position-3/#position-properties ; https://developer.mozilla.org/en-US/docs/Web/CSS/top |
| 64 | `z-index` (stacking) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-position-3/#z-index ; https://developer.mozilla.org/en-US/docs/Web/CSS/z-index |
| 65 | `width`, `height`, `min-width`, `min-height`, `max-width`, `max-height` (intrinsic and extrinsic sizing) | all | not-started (Gyre partial) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-sizing-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/width |
| 66 | `aspect-ratio` (preferred aspect ratio) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-sizing-4/#aspect-ratio ; https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio |
| 67 | `box-sizing` (`content-box`, `border-box`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-sizing-3/#box-sizing ; https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing |
| 68 | `margin`, `margin-*`, `margin-block`, `margin-inline`, `margin-trim` (margin collapse, margin trim) | all | not-started (Gyre partial) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-box-4/#margins ; https://drafts.csswg.org/css-box-4/#margin-trim ; https://developer.mozilla.org/en-US/docs/Web/CSS/margin |
| 69 | `padding`, `padding-*`, `padding-block`, `padding-inline` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-box-4/#paddings ; https://developer.mozilla.org/en-US/docs/Web/CSS/padding |
| 70 | `border`, `border-*`, `border-block`, `border-inline` (incl. `border-block-start`, `border-inline-end`, etc.) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-borders-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/border |
| 71 | `border-radius` (8 corner radii: `border-{top,right,bottom,left}-{start,end}-radius`, with elliptical form) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-borders-4/#border-radius ; https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius |
| 72 | `border-image` (`source`, `slice`, `width`, `outset`, `repeat`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-borders-4/#border-image ; https://developer.mozilla.org/en-US/docs/Web/CSS/border-image |
| 73 | `box-shadow`, `text-shadow` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-backgrounds-3/#box-shadow ; https://drafts.csswg.org/css-text-decor-4/#text-shadow ; https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow |
| 74 | `outline`, `outline-*`, `outline-offset`, `outline-color`, `outline-style`, `outline-width` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-ui-4/#outline ; https://developer.mozilla.org/en-US/docs/Web/CSS/outline |
| 75 | `overflow`, `overflow-x`, `overflow-y` (`visible`, `hidden`, `clip`, `scroll`, `auto`), `overflow-clip-margin` | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-overflow-3/ ; https://drafts.csswg.org/css-overflow-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/overflow |
| 76 | `overflow: clip` and `overflow-clip-margin` (the non-scroll-clipping variant) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-overflow-4/#overflow-properties ; https://developer.mozilla.org/en-US/docs/Web/CSS/overflow |
| 77 | `visibility` (`visible`, `hidden`, `collapse`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-display-3/#visibility ; https://developer.mozilla.org/en-US/docs/Web/CSS/visibility |
| 78 | `float` and `clear` | all | not-started (Gyre partial) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css2/#floats.html ; https://developer.mozilla.org/en-US/docs/Web/CSS/float |
| 79 | `object-fit` (`fill`, `contain`, `cover`, `none`, `scale-down`) and `object-position` | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-images-3/#sizing ; https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit |
| 80 | `contain` (`none`, `strict`, `content`, `size`, `layout`, `style`, `paint`, `inline-size`, `block-size`) | all | not-started | >=90% (subset Baseline 2022) | P3 | M | all yes/stable | https://drafts.csswg.org/css-contain-2/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/contain |
| 81 | `content-visibility` (`visible`, `auto`, `hidden`) | all | not-started | 75–90% (Baseline 2024) | P3 | M | all yes/stable | https://drafts.csswg.org/css-contain-2/#content-visibility ; https://developer.mozilla.org/en-US/docs/Web/CSS/content-visibility |

## B.5 Backgrounds, images, filters, masks, compositing

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 82 | `background`, `background-color`, `background-image`, `background-position`, `background-size`, `background-repeat`, `background-origin`, `background-clip`, `background-attachment` | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-backgrounds-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/background |
| 83 | `background-image` with `image-set(...)` and `image(...)` | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-images-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/image/image-set |
| 84 | `filter` (`blur`, `brightness`, `contrast`, `drop-shadow`, `grayscale`, `hue-rotate`, `invert`, `opacity`, `saturate`, `sepia`, `url(#filter)`, `none`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-filter-effects-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/filter |
| 85 | `backdrop-filter` (the same set as filter, but on the back of the element) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-filter-effects-2/#BackdropFilter ; https://developer.mozilla.org/en-US/docs/Web/CSS/backdrop-filter |
| 86 | `mask`, `mask-image`, `mask-mode`, `mask-repeat`, `mask-position`, `mask-clip`, `mask-origin`, `mask-size`, `mask-composite`, `mask-type` | all | not-started | 75–90% (Baseline 2023) | P3 | L | all yes/stable | https://drafts.csswg.org/css-masking-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/mask |
| 87 | `mix-blend-mode` and `isolation` (the CSS Compositing and Blending Level 1 properties) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/compositing-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/mix-blend-mode |
| 88 | `opacity` (0–1; applies a stack-group effect) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-color-4/#transparency ; https://developer.mozilla.org/en-US/docs/Web/CSS/opacity |
| 89 | `will-change` (`auto`, `<ident>`, `transform`, `opacity`, `scroll-position`, `contents`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-will-change-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/will-change |

## B.6 Text, fonts, writing modes, list styles

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 90 | `font` shorthand and longhands (`font-family`, `font-size`, `font-style`, `font-weight`, `font-stretch`, `font-variant-*`, `font-feature-settings`, `font-variation-settings`, `line-height`) | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-fonts-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/font |
| 91 | System and web fonts (the platform-font stack: `system-ui`, `ui-serif`, `ui-sans-serif`, `ui-monospace`, `ui-rounded`, `-apple-system`, `BlinkMacSystemFont`, `Roboto`, `Segoe UI`, `Helvetica`, `Arial`, `sans-serif`, `serif`, `monospace`, `cursive`, `fantasy`, `emoji`, `math`, `fangsong`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-fonts-4/#generic-font-families ; https://developer.mozilla.org/en-US/docs/Web/CSS/font-family |
| 92 | Variable fonts (`font-variation-settings`, `font-optical-sizing`, `font-weight: 1..1000`, `font-stretch: 50%..200%`) | all | not-started | 75–90% (Baseline 2023) | P3 | L | all yes/stable | https://drafts.csswg.org/css-fonts-4/#font-variation-settings ; https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings |
| 93 | `color` (named colours, `currentColor`, `transparent`, `hex`, `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hwb()`, `lab()`, `lch()`, `oklab()`, `oklch()`, `color()`) | all | not-started (parser tokenises the basic forms) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-color-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/color |
| 94 | `line-height` (`normal`, `<number>`, `<length>`, `<percentage>`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css2/#line-height ; https://drafts.csswg.org/css-inline-3/#line-height ; https://developer.mozilla.org/en-US/docs/Web/CSS/line-height |
| 95 | `text-align` (`start`, `end`, `left`, `right`, `center`, `justify`, `justify-all`, `match-parent`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-text-3/#text-align-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/text-align |
| 96 | `text-align: start` / `end` and `:dir(ltr/rtl)` interaction | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-text-3/#text-align-property |
| 97 | `text-decoration` (`text-decoration-line`, `text-decoration-style`, `text-decoration-color`, `text-decoration-thickness`, `text-underline-offset`, `text-underline-position`, `text-decoration-skip-ink`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-text-decor-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration |
| 98 | `text-transform` (`none`, `capitalize`, `uppercase`, `lowercase`, `full-width`, `full-size-kana`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-text-3/#text-transform-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform |
| 99 | `white-space` (`normal`, `nowrap`, `pre`, `pre-wrap`, `pre-line`, `break-spaces`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-text-4/#white-space-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/white-space |
| 100 | `word-break` (`normal`, `break-all`, `keep-all`, `auto-phrase`), `word-wrap` / `overflow-wrap` (`normal`, `break-word`, `anywhere`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-text-4/#word-break-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/word-break |
| 101 | `text-wrap` (`wrap`, `nowrap`, `balance`, `pretty`, `stable`) | all | not-started | 50–75% (Baseline 2024) | P3 | M | all yes/stable (modern) | https://drafts.csswg.org/css-text-4/#text-wrap-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/text-wrap |
| 102 | `text-wrap: balance` / `pretty` (the modern line-balancing options) | all | not-started | 50–75% (Baseline 2024) | P3 | M | all yes/stable | https://developer.mozilla.org/en-US/docs/Web/CSS/text-wrap |
| 103 | `line-break` (`auto`, `loose`, `normal`, `strict`, `anywhere`), `word-spacing`, `letter-spacing`, `tab-size` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-text-4/#line-break-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing |
| 104 | `direction` (`ltr`, `rtl`) and `unicode-bidi` (`normal`, `embed`, `isolate`, `bidi-override`, `isolate-override`, `plaintext`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-writing-modes-3/#direction ; https://drafts.csswg.org/css-writing-modes-3/#unicode-bidi |
| 105 | `writing-mode` (`horizontal-tb`, `vertical-rl`, `vertical-lr`, `sideways-rl`, `sideways-lr`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-writing-modes-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/writing-mode |
| 106 | `font-variant` shorthand and the full `font-variant-*` longhand set (incl. `font-variant-caps`, `font-variant-numeric`, `font-variant-ligatures`, `font-variant-alternates`, `font-variant-east-asian`, `font-variant-position`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-fonts-4/#font-variant-prop ; https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant |
| 107 | `font-variant-emoji` (`normal`, `text`, `emoji`, `unicode`) | all | not-started | 50–75% (Baseline 2023) | P3 | S | all yes/stable | https://drafts.csswg.org/css-fonts-4/#font-variant-emoji-prop ; https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant-emoji |
| 108 | `font-synthesis` (`weight`, `style`, `small-caps`, `position`, `none`, `auto`) and `font-synthesis-position`, `font-synthesis-small-caps` | all | not-started | >=90% (Baseline 2022) | P3 | S | all yes/stable | https://drafts.csswg.org/css-fonts-4/#font-synthesis-prop |
| 109 | `font-kerning` (`auto`, `normal`, `none`), `font-optical-sizing` (`auto`, `none`), `font-language-override` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-fonts-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/font-kerning |
| 110 | `font-feature-settings` (the OpenType low-level feature toggles) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-fonts-4/#font-feature-settings-prop ; https://developer.mozilla.org/en-US/docs/Web/CSS/font-feature-settings |
| 111 | `font-size-adjust` (`none`, `ex-height`, `cap-height`, `ch-width`, `ic-width`, `from-font`) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-fonts-4/#font-size-adjust-descdef |
| 112 | `letter-spacing` (tracking), `word-spacing` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-text-4/#letter-spacing-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing |
| 113 | `text-indent`, `text-justify`, `text-align-last` | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-text-3/#text-indent-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/text-indent |
| 114 | `list-style` (shorthand), `list-style-type` (incl. `disc`, `circle`, `square`, `decimal`, `decimal-leading-zero`, `lower-roman`, `upper-roman`, `lower-greek`, `lower-latin`, `upper-latin`, `armenian`, `georgian`, `lower-alpha`, `upper-alpha`, `none`, `disclosure-open`, `disclosure-closed`, `<counter-style>`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-lists-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type |
| 115 | `list-style-position` (`inside`, `outside`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-lists-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-position |
| 116 | `list-style-image` (with `url()` and image fallbacks) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-lists-3/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-image |

## B.7 Flexbox (CSS Flexbox 1)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 117 | `display: flex` / `inline-flex` (the flex container) | all | not-started (Gyre partial) | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#flex-containers ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_flexible_box_layout |
| 118 | `flex-direction` (`row`, `row-reverse`, `column`, `column-reverse`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#flex-direction-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction |
| 119 | `flex-wrap` (`nowrap`, `wrap`, `wrap-reverse`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#flex-wrap-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap |
| 120 | `flex-flow` (shorthand) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#flex-flow-property |
| 121 | `justify-content` (`flex-start`, `flex-end`, `center`, `space-between`, `space-around`, `space-evenly`, `normal`, `stretch`, `start`, `end`, `left`, `right`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#justify-content-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content |
| 122 | `align-items` and `align-self` (`flex-start`, `flex-end`, `center`, `baseline`, `stretch`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#align-items-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/align-items |
| 123 | `align-content` (the multi-line cross-axis) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#align-content-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/align-content |
| 124 | `order` (the visual order, no DOM change) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#order-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/order |
| 125 | `flex-grow`, `flex-shrink`, `flex-basis` (the flex factor) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#flex-common ; https://developer.mozilla.org/en-US/docs/Web/CSS/flex |
| 126 | `flex` shorthand | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-flexbox-1/#flex-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/flex |
| 127 | `gap`, `row-gap`, `column-gap` (the shared multi-layout gap property) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-align-3/#gaps ; https://developer.mozilla.org/en-US/docs/Web/CSS/gap |

## B.8 Grid (CSS Grid Layout 1 / 2)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 128 | `display: grid` / `inline-grid` | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-grid-1/#grid-containers ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_grid_layout |
| 129 | `grid-template-columns`, `grid-template-rows` (with `repeat()`, `minmax()`, `auto-fit`, `auto-fill`, named lines, named areas) | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-grid-1/#track-sizing ; https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns |
| 130 | `grid-template-areas`, `grid-area`, named areas | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-grid-1/#grid-template-areas-property |
| 131 | `grid` shorthand, `grid-template` shorthand | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-grid-1/#grid-shorthand |
| 132 | `grid-column`, `grid-row`, `grid-row-start`/`end`, `grid-column-start`/`end` (with `<ident>`, `<integer>`, `span`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-grid-1/#placement ; https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column |
| 133 | `grid-auto-rows`, `grid-auto-columns`, `grid-auto-flow` (`row`, `column`, `dense`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-grid-1/#auto-tracks ; https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow |
| 134 | `justify-self`, `align-self`, `justify-items`, `align-items` (the grid alignment set) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-align-3/#positional-values ; https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self |
| 135 | `place-content`, `place-items`, `place-self` (the grid-alignment shorthand trio) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-align-3/#place-content ; https://developer.mozilla.org/en-US/docs/Web/CSS/place-content |
| 136 | Subgrid (`grid-template-columns: subgrid`, `grid-template-rows: subgrid`) | all | not-started | 75–90% (Baseline 2023) | P3 | L | Chromium yes/stable; Firefox yes/stable; Safari yes/stable (in 17.0+); Servo no | https://drafts.csswg.org/css-grid-2/#subgrid ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_grid_layout/Subgrid |
| 137 | Masonry layout (`grid-template-rows: masonry`, `masonry-auto-flow`) | all | not-started | <25% (still draft; Firefox has the implementation) | P3 / P5 | L | Firefox behind flag (yes/partial); Chromium / WebKit no; spec at W3C WD | https://drafts.csswg.org/css-grid-3/#masonry-layout |
| 138 | `masonry-template-rows`, `masonry-template-columns`, `masonry-slack` | all | not-started | <25% (spec WD) | P5 | L | Firefox yes/partial; spec WD | https://drafts.csswg.org/css-grid-3/ |

## B.9 Transforms, animations, transitions, motion

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 139 | `transform` (with `translate`, `translateX`, `translateY`, `translateZ`, `translate3d`, `scale`, `scaleX`, `scaleY`, `scaleZ`, `rotate`, `rotateX`/`Y`/`Z`, `rotate3d`, `skew`, `skewX`, `skewY`, `matrix`, `matrix3d`, `perspective`) | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-transforms-1/ ; https://drafts.csswg.org/css-transforms-2/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/transform |
| 140 | `transform-origin`, `transform-style` (`flat`, `preserve-3d`), `perspective`, `perspective-origin`, `backface-visibility` | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-transforms-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/transform-origin |
| 141 | Individual `translate` / `scale` / `rotate` properties (the 2022 longhand form) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-transforms-2/#individual-transforms ; https://developer.mozilla.org/en-US/docs/Web/CSS/translate |
| 142 | `transition` shorthand and longhands (`transition-property`, `transition-duration`, `transition-timing-function`, `transition-delay`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-transitions-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/transition |
| 143 | `transition-behavior: allow-discrete` (the discrete-property transition allow-flag) | all | not-started | 75–90% (Baseline 2024) | P3 | M | all yes/stable | https://drafts.csswg.org/css-transitions-2/#transition-behavior-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/transition-behavior |
| 144 | `animation` shorthand and longhands (`animation-name`, `animation-duration`, `animation-timing-function`, `animation-delay`, `animation-iteration-count`, `animation-direction`, `animation-fill-mode`, `animation-play-state`, `animation-composition`) | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-animations-1/ ; https://drafts.csswg.org/css-animations-2/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/animation |
| 145 | `@keyframes` rule (with `from`/`to`, `<percentage>`) | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-animations-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@keyframes |
| 146 | `animation-composition` (`replace`, `add`, `accumulate`) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-animations-2/#animation-composition |
| 147 | CSS `cubic-bezier()`, `steps()`, `linear`, `ease`, `ease-in`, `ease-out`, `ease-in-out` (the timing-function values) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-easing-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function |
| 148 | `linear()` easing function (the per-keyframe linear-progress easing, CSS Easing 2) | all | not-started | 50–75% (Baseline 2023) | P3 | S | all yes/stable | https://drafts.csswg.org/css-easing-2/#the-linear-easing-function ; https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function/linear |
| 149 | Scroll-driven animations (animation-timeline: `scroll()`, `view()`; animation-range: `cover`, `contain`, `entry`, `exit`, `entry-crossing`, `exit-crossing`, `normal`) | all | not-started | 25–50% (Chromium 115+; WebKit 17.4+; Gecko 132+) | P3 | L | mixed: Chromium/WebKit yes/stable; Gecko yes/stable (recent); Servo no | https://drafts.csswg.org/scroll-animations-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/animation-timeline |
| 150 | `scroll-timeline`, `scroll-timeline-name`, `scroll-timeline-axis`, `view-timeline`, `view-timeline-name`, `view-timeline-axis` | all | not-started | 25–50% (same as #149) | P3 | L | mixed: same as #149 | https://drafts.csswg.org/scroll-animations-1/ |
| 151 | `offset-path` (with `path()`, `ray()`, `<basic-shape>`, `none`) and `offset-distance`, `offset-rotate`, `offset-anchor`, `offset-position` | all | not-started | 75–90% (Baseline 2023) | P3 | L | all yes/stable | https://drafts.csswg.org/motion-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/offset-path |
| 152 | View Transitions (the cross-document and same-document `document.startViewTransition()`, `::view-transition`, `::view-transition-group()`, `::view-transition-image-pair()`, `::view-transition-old()`, `::view-transition-new()`, `@view-transition`) | all | not-started | 75–90% (Baseline 2024) | P3 | XL | all yes/stable | https://drafts.csswg.org/css-view-transitions-1/ ; https://developer.mozilla.org/en-US/docs/Web/API/View_Transitions_API |

## B.10 Container queries and high-priority draft integration

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 153 | `container` (the shorthand for `container-name` / `container-type`), `container-name` (a list of `<ident>`), `container-type` (`normal`, `size`, `inline-size`, `block-size`) | all | not-started | 75–90% (Baseline 2023) | P3 | L | all yes/stable | https://drafts.csswg.org/css-conditional-5/#container-queries ; https://drafts.csswg.org/css-contain-3/#container-type ; https://developer.mozilla.org/en-US/docs/Web/CSS/container |
| 154 | `container-query` (the `@container` block at-rule, with `name`, `type`, `inline-size`, `block-size`, `aspect-ratio`, `style`, `scroll-state` queries) | all | not-started | 75–90% (size subset Baseline 2023; style + scroll-state in 2024) | P3 | L | all yes/stable | https://drafts.csswg.org/css-conditional-5/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@container |
| 155 | `@container style()`, `@container scroll-state()` (the newer query categories) | all | not-started | 50–75% (Baseline 2024) | P3 | M | all yes/stable | https://drafts.csswg.org/css-conditional-5/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@container |
| 156 | `contain-intrinsic-size` (the reserved intrinsic size for `content-visibility: auto`) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-size ; https://developer.mozilla.org/en-US/docs/Web/CSS/contain-intrinsic-size |
| 157 | Anchor positioning (the `anchor-name` / `position-anchor` / `position-area` / `position-try` / `position-try-order` / `position-try-fallbacks` / `position-visibility` set) | all | not-started | 25–50% (Chromium 125+ behind flag; spec stabilised 2024) | P3 | L | Chromium yes/partial (behind flag); spec recently stabilised | https://drafts.csswg.org/css-anchor-position-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_anchor_positioning |
| 158 | `field-sizing` (`fixed`, `content`) (the new form-input-sizing property) | all | not-started | 50–75% (Baseline 2024) | P3 | S | all yes/stable | https://drafts.csswg.org/css-sizing-4/#field-sizing ; https://developer.mozilla.org/en-US/docs/Web/CSS/field-sizing |
| 159 | `interactivity` (`auto`, `none`) (the inert-content-via-CSS feature) | all | not-started | 25–50% (Chromium 137+) | P3 | M | Chromium yes/partial; spec in development | https://drafts.csswg.org/css-ui-4/#interactivity |

## B.11 MathML, SVG (CSS side)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 160 | CSS in MathML (top-level defaults for `math` element) | all | not-started (depends on #39 foreign content) | 50–75% (MathML removed from Chromium 2024) | P3 | M | mixed: Gecko/WebKit yes/stable; Chromium no | https://mathml-refresh.github.io/mathml-core/ ; https://developer.mozilla.org/en-US/docs/Web/MathML |
| 161 | SVG-in-CSS (the `fill`, `stroke`, `stroke-width`, `stroke-dasharray`, `stroke-linecap`, `stroke-linejoin`, `vector-effect`, `paint-order` properties in CSS) | all | not-started | >=90% (in CSSOM; paint side is chunk 5) | P3 | M | all yes/stable | https://svgwg.org/svg2-draft/styling.html ; https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill |
| 162 | `clip-path` (`none`, `<basic-shape>`, `url(#clip)`, `<geometry-box>`, `margin-box`, `border-box`, `padding-box`, `content-box`, `fill-box`, `stroke-box`, `view-box`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-masking-1/#the-clip-path ; https://developer.mozilla.org/en-US/docs/Web/CSS/clip-path |
| 163 | `path()` (the CSS shape source) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-shapes-2/#funcdef-basic-shape-path ; https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape/path |
| 164 | `<basic-shape>` (the `circle()`, `ellipse()`, `inset()`, `polygon()`, `xywh()`, `rect()` set) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-shapes-1/#basic-shape-functions ; https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape |
| 165 | `shape-outside` (the text-wrap-around-a-shape property) | all | not-started | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-shapes-1/#shape-outside-property ; https://developer.mozilla.org/en-US/docs/Web/CSS/shape-outside |
| 166 | `shape-image-threshold`, `shape-margin` | all | not-started | 75–90% (Baseline 2023) | P3 | S | all yes/stable | https://drafts.csswg.org/css-shapes-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/shape-margin |
| 167 | CSS exclusions (the older `wrap-flow` / `wrap-through` set, not implemented in modern engines) | not in any current browser | n/a | <25% (IE/Edge legacy only) | n/a | n/a | not in scope; no row in modern engines | https://drafts.csswg.org/css-exclusions-1/ |
| 168 | CSS regions (`flow-from` / `flow-into`; deprecated) | not in any current browser | n/a | <25% (legacy) | n/a | n/a | not in scope; not in any current engine | https://drafts.csswg.org/css-regions-1/ |

## B.12 Cascade and origin (CSS Cascading and Inheritance Level 5)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 169 | Cascade origins (user-agent, user, author, animation, transition) | all | not-started (parser yes; cascade not) | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/css-cascade-5/#cascade-origin ; https://developer.mozilla.org/en-US/docs/Web/CSS/@layer |
| 170 | Cascade priority (normal vs `!important`, with normal < important at same origin) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-cascade-5/#cascade-priority |
| 171 | Cascade layers (`@layer`, `layer()`) | all | not-started (parser yes) | 75–90% (Baseline 2023) | P3 | M | all yes/stable | https://drafts.csswg.org/css-cascade-5/#layering ; https://developer.mozilla.org/en-US/docs/Web/CSS/@layer |
| 172 | Cascade sorting per origin (specificity + order) | all | not-started (specificity struct exists) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-cascade-5/#cascade-sort ; https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity |
| 173 | `all` shorthand (reset or inherit of all properties) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-cascade-5/#all-shorthand ; https://developer.mozilla.org/en-US/docs/Web/CSS/all |
| 174 | `inherit`, `initial`, `unset`, `revert`, `revert-layer` (the cascade-keyword set) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-cascade-5/#defaulting ; https://developer.mozilla.org/en-US/docs/Web/CSS/revert |
| 175 | Custom properties (`--foo: bar;`) with `var(--foo)` consumption and `@property` registration | all | not-started (parser tokenises; resolution not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-variables-1/ ; https://drafts.csswg.org/css-properties-values-api-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/--* |
| 176 | `env()` and `env(viewport-segment-width, …)` (UA-defined environment variables) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/css-env-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/env |

## B.13 Cascade integration (View Transitions, Container, Anchor) and media queries

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 177 | Media query level 5 (`prefers-color-scheme`, `prefers-reduced-motion`, `prefers-reduced-transparency`, `prefers-contrast`, `prefers-reduced-data`, `prefers-color-scheme: dark`, `prefers-color-scheme: light`, `forced-colors`, `hover`, `pointer`, `any-pointer`, `any-hover`, `orientation`, `aspect-ratio`, `resolution`) | all | not-started (parser tokenises; match-eval not) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/mediaqueries-5/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_media_queries/Using_media_queries |
| 178 | Media query level 4 (`scripting`, `update`, `overflow-block`, `overflow-inline`, `grid`, `color-gamut`, `dynamic-range`, `video-dynamic-range`, `device-aspect-ratio`, `monochrome`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/mediaqueries-4/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/@media |
| 179 | User-preference queries (`@media (prefers-color-scheme: dark)`, etc.) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/mediaqueries-5/#user-preferences |

---

## B.14 Notes on scoring methodology specific to this file

- **"shipped" is very rare.** Per `00-methodology.md` §3 and the wiring
  rule in `AGENTS.md`, a row is `shipped` only when the feature is
  reachable from a real surface AND an integration test exercises it.
  M4.4.1 Item 4 (`spiral-fmt::css` parser) is the only CSS sub-system
  with code that has integration tests. The cascade, selector
  matching, and computed-value resolution are all `not-started`.
- **Parser coverage is the floor.** Where the parser tokenises a
  property but no cascade / computed-value code exists, the row is
  `partial`. This is the modal status in this file.
- **The "Engine notes" column is shallow on purpose.** Per the
  contract, chunk 7 is the engine-coverage deep dive. Here we use
  "yes/stable" / "yes/partial" / "behind flag" / "no" only.
- **Numerical scores (rows 121–123, 134–135, 144, 146, 152) reference
  Baseline dates from `webstatus.dev` and MDN.** Dates are best-effort
  as of 2026-06-16; chunk 12 (the matrix) should re-verify.

---

## B.15 Cross-references to `specs/GAP_ANALYSIS.md`

| Row in this file | GAP_ANALYSIS row | Comment |
|------------------|------------------|---------|
| 1–20, 22–58, 168–179 | §1.2 (CSS parser & cascade) | rows 1–20 selectors, 22–58 at-rules / values, 168–176 cascade, 177–179 media |
| 59–81, 117–138, 139–152 | §1.4 (Layout — Gyre) | rows 59–81 box model, 117–127 flex, 128–138 grid, 139–152 transforms / motion |
| 134–142, 145, 152, 162, 165–166 | §1.5 (Render) | rows that produce output for the paint pipeline |
| 84–88, 86, 152 | §1.4 / §1.5 boundary | filters and masks bridge layout → render |

## B.16 Open questions specific to this file

1. **Masonry is in scope?** Per the contract it is, but the spec is
   still W3C WD with limited engine coverage (Firefox only). Should
   it be de-listed to "next-gen" and tracked in chunk 8? Currently
   kept here per the contract.
2. **CSS Houdini (`@property`, custom layout / paint / animation
   worklets).** Per the contract, custom paint / layout / animation
   worklets belong in chunk 8 (APIs). But the `@property` rule and
   typed custom properties are engine-level CSS. Currently `@property`
   is row 32, and the worklets are punted to chunk 8.
3. **`@scope` (the rule-based scoping at-rule, not the `:scope`
   pseudo-class).** `@scope (.light) to (.shadow)` is a recent
   Chromium-only feature and would normally be a chunk 8 entry.
   Currently not in this file — needs an explicit decision.
4. **Scrolling container pseudo-elements (`::scroll-marker`,
   `::scroll-button`, `::scroll-marker-group`).** Recent
   Chromium 137+; tied to scroll-driven animations work. Listed in
   chunk 8 territory. Currently not in this file.
5. **Color font / COLRv1 / variable-color font support.** CSS-side
   surface is `font-palette` and `font-variant-emoji` (#107). The
   paint side is chunk 5. Currently only `font-variant-emoji` is
   a row. Should `font-palette` get its own row? Yes, but it
   needs cross-chunk coordination.
6. **The CSS `attr()` typed-form (row #52).** Currently scoped to
   "advanced types in progress". The conservative answer is to
   split into rows for `attr() string`, `attr() length`,
   `attr() color`, etc., but the per-engine coverage is identical
   so one row is fine.

