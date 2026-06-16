# Chunk 1 — Core Web Platform Standards Inventory: HTML, DOM, JavaScript

> **Companion to `01-feature-inventory-index.md` (the chunk header).**
> This file covers §A.1 (WHATWG HTML), §A.2 (WHATWG DOM + CSSOM
> document-side), §A.3 (ECMAScript / ECMA-402). CSS lives in
> `01-feature-inventory-css.md`.
>
> **Status in Spiral** is scored against the current code on
> `research/competitive-parity` (M4.4 complete, M4.5 in flight as of
> 2026-06-16). See the index file for the per-crate ground truth.

---

## A.1 WHATWG HTML — elements, attributes, parser behaviours

### A.1.1 Document metadata root

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 1 | `<!DOCTYPE html>` parsing & mode (quirks / limited-quirks / no-quirks) | all | partial (DOCTYPE token handled; limited-quirks classifier not yet; GAP 1.1 `[~]`) | >=90% | P2 | M | all yes/stable; Servo partial; Flow partial | https://html.spec.whatwg.org/multipage/parsing.html#the-doctype ; https://developer.mozilla.org/en-US/docs/Web/HTML/Quirks_Mode_and_Standards_Mode |
| 2 | `<html>` root element (implicit and explicit) | all | shipped (via `parse_html`; GAP 1.1) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/semantics.html#the-html-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html |
| 3 | `<head>` element and its required children | all | shipped | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/semantics.html#the-head-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/head |
| 4 | `<title>` element (with Rawtext content model) | all | shipped (Rawtext mode in tokeniser; GAP Delta 2) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/semantics.html#the-title-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title |
| 5 | `<base>` element (`href`, `target`) | all | not-started (parser accepts; no DOM attach) | >=90% | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/semantics.html#the-base-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base |
| 6 | `<link>` element (`rel`, `href`, `type`, `media`, `sizes`, `as`, `crossorigin`, `integrity`, `imagesrcset`, `imagesizes`, `blocking`, `fetchpriority`, `disabled`, `color`) | all | not-started (parser accepts; rel-handler not built) | >=90% | P4 | L | all yes/stable (subset of `rel` values is Baseline) | https://html.spec.whatwg.org/multipage/links.html#the-link-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link |
| 7 | `<meta>` element (`charset`, `name`, `http-equiv`, `content`, `media`, `property`, `itemprop`) | all | not-started (parser accepts; `http-equiv` is M5+) | >=90% | P3 | M | all yes/stable; `<meta http-equiv="refresh">` is behind flag in some privacy-focused forks | https://html.spec.whatwg.org/multipage/semantics.html#the-meta-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta |
| 8 | `<style>` element (with Rawtext content model) | all | partial (Rawtext tokeniser yes; cascade/integration with `spiral-fmt` is the shim path — see ADR 0001) | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/semantics.html#the-style-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style |
| 9 | `<script>` element (with ScriptData content model; `type`, `src`, `async`, `defer`, `module`, `integrity`, `crossorigin`, `referrerpolicy`, `blocking`, `fetchpriority`, `nomodule`, `attributionsrc`, `noModule`) | all | partial (ScriptData yes; no execute path; GAP 1.6) | >=90% | P2 / P3 | L | all yes/stable | https://html.spec.whatwg.org/multipage/scripting.html#the-script-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script |
| 10 | `<noscript>` element | all | not-started | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/scripting.html#the-noscript-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noscript |

### A.1.2 Sectioning root and body

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 11 | `<body>` element | all | shipped | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/sections.html#the-body-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body |
| 12 | `<article>`, `<section>`, `<nav>`, `<aside>`, `<header>`, `<footer>`, `<main>` (sectioning content + landmarks) | all | shipped (parsed; no a11y/landmark tree yet — chunk 8) | >=90% | P2 / P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/sections.html#sections ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article |
| 13 | `<address>` | all | not-started | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/sections.html#the-address-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/address |
| 14 | `<h1>` … `<h6>` heading elements (with outline algorithm) | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements |

### A.1.3 Grouping content

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 15 | `<p>` | all | shipped | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/grouping-content.html#the-p-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p |
| 16 | `<hr>` | all | shipped | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/grouping-content.html#the-hr-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr |
| 17 | `<pre>` | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/grouping-content.html#the-pre-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre |
| 18 | `<blockquote>` (`cite`) | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/grouping-content.html#the-blockquote-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote |
| 19 | `<ol>`, `<ul>`, `<li>` (incl. `start`, `reversed`, `type` on `<ol>`; `value` on `<li>`) | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/grouping-content.html#the-ol-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol |
| 20 | `<dl>`, `<dt>`, `<dd>`, `<dfn>` (description list) | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/grouping-content.html#the-dl-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl |
| 21 | `<figure>`, `<figcaption>` | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/grouping-content.html#the-figure-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figure |
| 22 | `<div>` (generic flow container) | all | shipped | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/grouping-content.html#the-div-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div |

### A.1.4 Text-level semantics

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 23 | `<a>` (`href`, `target`, `rel`, `download`, `ping`, `type`, `referrerpolicy`, `hreflang`) | all | not-started (parsed) | >=90% | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a |
| 24 | `<em>`, `<strong>`, `<small>`, `<s>`, `<cite>`, `<q>` (`cite` on `<q>`) | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-em-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em |
| 25 | `<dfn>`, `<abbr>`, `<ruby>`, `<rt>`, `<rp>`, `<rb>`, `<rtc>`, `<data>` (`value`), `<time>` (`datetime`) | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-dfn-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dfn |
| 26 | `<code>`, `<var>`, `<samp>`, `<kbd>`, `<sub>`, `<sup>`, `<i>`, `<b>`, `<u>`, `<mark>`, `<bdi>`, `<bdo>` (`dir`) | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-code-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code |
| 27 | `<span>` (generic phrasing container) | all | shipped | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-span-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span |
| 28 | `<br>`, `<wbr>` | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-br-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br |
| 29 | `<ins>`, `<del>` (`cite`, `datetime`) | all | shipped (parsed) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/edits.html#the-ins-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins |
| 30 | Text content (character data, entity references — `&amp;` `&lt;` `&gt;` `&quot;` `&apos;` and the named set) | all | shipped (Delta 3) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references ; https://developer.mozilla.org/en-US/docs/Glossary/Entity |

### A.1.5 Embedded content (chunk 5 covers video/audio, listed here for parser coverage)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 31 | `<img>` (`src`, `alt`, `width`, `height`, `srcset`, `sizes`, `loading`, `fetchpriority`, `decoding`, `crossorigin`, `referrerpolicy`, `ismap`, `usemap`) | all | not-started (parsed) | >=90% | P4 | L | all yes/stable; `loading="lazy"` Baseline; `decoding` Baseline | https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img |
| 32 | `<picture>`, `<source>` (`srcset`, `sizes`, `type`, `media`, `src`, `width`, `height`) | all | not-started | 75–90% (responsive art direction is widely used) | P4 | M | all yes/stable | https://html.spec.whatwg.org/multipage/embedded-content.html#the-picture-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture |
| 33 | `<iframe>` (`src`, `srcdoc`, `name`, `sandbox`, `allow`, `allowfullscreen`, `allowpaymentrequest`, `credentialless`, `loading`, `referrerpolicy`, `csp`) | all | not-started (parsed) | >=90% | P4 | XL | all yes/stable; `credentialless` Chromium-only currently | https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-iframe-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe |
| 34 | `<embed>`, `<object>` (`data`, `type`, `name`, `width`, `height`, `typemustmatch`), `<param>` | all | not-started | 50–75% (plugins legacy) | P4 | M | all yes/stable (mostly inert — NPAPI gone) | https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-embed-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed |
| 35 | `<video>`, `<audio>` (chunk 5 surface — listed here for parser coverage) | all | not-started (parser accepts) | >=90% | P4 / P5 | XL | all yes/stable; see chunk 5 | https://html.spec.whatwg.org/multipage/media.html#the-video-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video |
| 36 | `<track>` (`kind`, `src`, `srclang`, `label`, `default`) | all | not-started | 50–75% (captioned media) | P4 | M | all yes/stable | https://html.spec.whatwg.org/multipage/media.html#the-track-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track |
| 37 | `<map>`, `<area>` (`shape`, `coords`, `href`, `alt`, `download`, `ping`, `rel`, `target`) | all | not-started | 50–75% (image maps legacy) | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/image-maps.html#the-map-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map |
| 38 | `<svg>` and SVG namespace (foreign content parser mode) | all | not-started (per tokeniser docs, foreign content deferred to M5+) | >=90% | P3 | L | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inforeign ; https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg |
| 39 | `<math>` and MathML namespace (foreign content parser mode) | all | not-started | 50–75% (MathML removed from Chromium 2024; Gecko/WebKit yes) | P3 | L | mixed: Chromium removed MathML renderer 2024, Gecko + WebKit yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inforeign ; https://developer.mozilla.org/en-US/docs/Web/MathML/Element/math |

### A.1.6 Tabular data

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 40 | `<table>`, `<caption>`, `<colgroup>`, `<col>` (`span`), `<thead>`, `<tbody>`, `<tfoot>`, `<tr>`, `<th>`, `<td>` (`colspan`, `rowspan`, `headers`, `scope`, `abbr`) | all | partial (parsed; tree builder has no `InTable` / `InCell` modes yet — see `html/tree.rs` \"full insertion-mode machine (tables, select, template, foreign content) lands in M5+\") | >=90% | P2 / P3 | L | all yes/stable | https://html.spec.whatwg.org/multipage/tables.html#the-table-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table |

### A.1.7 Forms (large sub-system — chunk 4 covers full split)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 41 | `<form>` (`action`, `method`, `enctype`, `target`, `autocomplete`, `name`, `novalidate`, `rel`, `accept-charset`) | all | not-started (parsed) | >=90% | P4 | L | all yes/stable | https://html.spec.whatwg.org/multipage/forms.html#the-form-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form |
| 42 | `<label>` (`for`) | all | not-started | >=90% | P4 | M | all yes/stable | https://html.spec.whatwg.org/multipage/forms.html#the-label-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label |
| 43 | `<input>` element — type matrix (`text`, `password`, `checkbox`, `radio`, `submit`, `reset`, `button`, `file`, `hidden`, `image`, `email`, `url`, `tel`, `search`, `number`, `range`, `color`, `date`, `time`, `datetime-local`, `month`, `week`) | all | not-started (parsed) | >=90% | P4 | XL | all yes/stable; full UA shadow root tree per input type | https://html.spec.whatwg.org/multipage/input-element.html#the-input-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input |
| 44 | `<button>` (`type`, `form`, `formaction`, `formmethod`, `formnovalidate`, `formtarget`, `formenctype`, `popovertarget`, `popovertargetaction`) | all | not-started | >=90% | P4 | M | all yes/stable | https://html.spec.whatwg.org/multipage/form-elements.html#the-button-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button |
| 45 | `<select>`, `<datalist>`, `<optgroup>`, `<option>` (`selected`, `value`, `label`, `disabled`) | all | not-started | >=90% | P4 | L | all yes/stable | https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select |
| 46 | `<textarea>` (`rows`, `cols`, `wrap`, `placeholder`, `readonly`, `disabled`, `required`, `maxlength`, `minlength`, `autocomplete`, `dirname`, `form`, `inputmode`, `enterkeyhint`, `autocapitalize`, `spellcheck`) | all | not-started | >=90% | P4 | M | all yes/stable | https://html.spec.whatwg.org/multipage/form-elements.html#the-textarea-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea |
| 47 | `<output>` (`for`, `form`, `name`) | all | not-started | 75–90% | P4 | S | all yes/stable | https://html.spec.whatwg.org/multipage/form-elements.html#the-output-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output |
| 48 | `<progress>` (`value`, `max`) | all | not-started | 75–90% | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/form-elements.html#the-progress-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress |
| 49 | `<meter>` (`value`, `min`, `max`, `low`, `high`, `optimum`) | all | not-started | 75–90% | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/form-elements.html#the-meter-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter |
| 50 | `<fieldset>`, `<legend>` (`disabled`, `form`, `name`) | all | not-started | 75–90% | P4 | M | all yes/stable | https://html.spec.whatwg.org/multipage/form-elements.html#the-fieldset-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset |

### A.1.8 Interactive elements

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 51 | `details` / `summary` (with `open` attribute) | all | not-started | >=90% | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/interactive-elements.html#the-details-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details |
| 52 | `<dialog>` (`open`, `showModal()` algorithm, top-layer, `::backdrop`) | all | not-started | 75–90% (Baseline 2022) | P3 | L | all yes/stable; Gecko landed in 2022; Servo in progress | https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog |
| 53 | `<menu>`, `<menuitem>` (context menu) | all | not-started | 25–50% (largely unimplemented) | P3 | L | Gecko yes/stable; others no (HTML5.3 removed; Gecko retains) | https://html.spec.whatwg.org/multipage/interactive-elements.html#the-menu-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menu |

### A.1.9 Web Components — Custom Elements and Shadow DOM

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 54 | `customElements` registry (`define`, `get`, `upgrade`, `whenDefined`) | all | not-started | >=90% | P3 | L | all yes/stable; Servo yes/partial; Ladybird no | https://html.spec.whatwg.org/multipage/custom-elements.html#custom-element-conformance ; https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements |
| 55 | Autonomous custom elements (lifecycle: `connectedCallback`, `disconnectedCallback`, `attributeChangedCallback`, `adoptedCallback`) | all | not-started | >=90% | P3 | L | all yes/stable | https://html.spec.whatwg.org/multipage/custom-elements.html#custom-element-life-cycle ; https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements |
| 56 | Customised built-in elements (`is="…"`) | all | not-started | 50–75% (Chromium/WebKit yes; Gecko removed in 2024, planning to re-add) | P3 | M | mixed: Chromium/WebKit yes/stable; Gecko no (removed 2024) | https://html.spec.whatwg.org/multipage/custom-elements.html#customized-built-in-element ; https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements#customized_built-in_elements |
| 57 | Shadow DOM (`attachShadow({mode: 'open' / 'closed'})`, `shadowRoot`) | all | not-started | >=90% | P3 | XL | all yes/stable; Servo yes/partial; Ladybird no | https://dom.spec.whatwg.org/#interface-shadowroot ; https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow |
| 58 | Slot / `::slotted()` / `<slot name="…">` (light-DOM projection) | all | not-started | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#slotables ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot |
| 59 | Declarative Shadow DOM (`<template shadowrootmode="open / closed">`, `shadowrootclonable`, `shadowrootserializable`) | all | not-started | 50–75% (Chromium 90+, WebKit 16.4+, Gecko 123+) | P3 | L | Chromium yes/stable; WebKit yes/stable; Gecko yes/stable; Servo/Ladybird no | https://html.spec.whatwg.org/multipage/scripting.html#declarative-shadow-dom ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template#declarative_shadow_dom |
| 60 | Event retargeting (composedPath, `composed` flag, scoped retargeting through shadow roots) | all | not-started | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#events-composed-path ; https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath |
| 61 | `:host`, `:host()`, `:host-context()` CSS pseudo-classes | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/css-scoping/#host-element ; https://developer.mozilla.org/en-US/docs/Web/CSS/:host |
| 62 | Constructable Stylesheets (`new CSSStyleSheet()`, `adoptedStyleSheets`, `replaceSync`, `replace`) | all | not-started | 75–90% (Chromium 73+, WebKit 16.4+, Gecko 101+) | P3 | M | all yes/stable; Ladybird no | https://wicg.github.io/construct-stylesheets/ ; https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptedStyleSheets |

### A.1.10 Template, slot, and HTML parser features

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 63 | `<template>` element and its content document fragment | all | partial (tokenised; GAP 1.1 — "Returns the element itself (no document fragment). Lossy.") | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/scripting.html#the-template-element ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template |
| 64 | Adoption agency algorithm (for misnested formatting) | all | not-started (per GAP 1.1) | >=90% | P2 | L | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#adoption-agency-algorithm |
| 65 | Foster parenting (out-of-table / in-table element placement) | all | not-started | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#foster-parenting |
| 66 | Encoding sniffing (BOM, `<meta charset>`, transport-layer charset, content sniff) | all | not-started (per GAP 1.1 — "Assumes UTF-8 only") | >=90% | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#encoding-sniffing-algorithm ; https://encoding.spec.whatwg.org/ |
| 67 | Numeric character references (`&#NN;` and `&#xHH;` with the spec-mandated Windows-1252 fixup range 0x80..=0x9F) | all | shipped (Delta 3) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#numeric-character-reference-end-state ; https://html.spec.whatwg.org/multipage/parsing.html#table-charref |
| 68 | Format-extracted character references (the named set: `&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`) | all | shipped (Delta 3) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#named-character-reference-state |
| 69 | Rawtext / ScriptData / ScriptDataEscaped / RCData modes (per-element content models for `<script>`, `<style>`, `<title>`, `<textarea>`, `<noscript>`, `<noembed>`, `<noframes>`, `<xmp>`, `<iframe>`, etc.) | all | shipped (Delta 2) | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#rawtext-state ; https://html.spec.whatwg.org/multipage/parsing.html#script-data-state |
| 70 | PLAINTEXT state (everything after `<plaintext>` is text) | all | shipped (tokeniser) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#plaintext-state |

### A.1.11 Obsolete / parser-compatible elements (collapsed per the open question)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 71 | HTML5 obsolete-but-parsed elements (`<acronym>`, `<applet>`, `<basefont>`, `<bgsound>`, `<big>`, `<blink>`, `<center>`, `<command>`, `<content>`, `<dir>`, `<element>`, `<font>`, `<frame>`, `<frameset>`, `<hgroup>`, `<image>`, `<keygen>`, `<listing>`, `<marquee>`, `<menuitem>`, `<multicol>`, `<nextid>`, `<nobr>`, `<noembed>`, `<noframes>`, `<plaintext>`, `<shadow>`, `<spacer>`, `<strike>`, `<tt>`, `<xmp>`) | all | not-started (parser tokenises some via Rawtext paths; no DOM attach; `<plaintext>` yes per #70) | <25% (parity for parser compat) | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/obsolete.html ; https://developer.mozilla.org/en-US/docs/Web/HTML/Element#obsolete_and_deprecated_elements |

### A.1.12 HTML attributes — global and element-specific

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 72 | Global attributes (`id`, `class`, `style`, `title`, `lang`, `dir`, `hidden`, `tabindex`, `accesskey`, `draggable`, `spellcheck`, `autocapitalize`, `autocorrect`, `autofocus`, `contenteditable`, `enterkeyhint`, `inputmode`, `is`, `itemid`, `itemprop`, `itemref`, `itemscope`, `itemtype`, `nonce`, `part`, `role`, `slot`, `translate`, `inert`, `popover`, `writing-suggestions`, `virtualkeyboardpolicy`, `exportparts`, `anchor`) | all | not-started (parsed as attributes; behaviour unmodelled; `slot` / `is` / `part` tied to Web Components work) | >=90% | P2 / P3 | L | all yes/stable; `inert` Baseline 2022; `popover` Baseline 2024; `anchor` behind flag in all but Chromium-stable | https://html.spec.whatwg.org/multipage/dom.html#global-attributes ; https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes |
| 73 | Microdata (`itemscope`, `itemtype`, `itemprop`, `itemref`, `itemid`) | all | not-started | 50–75% (microdata is implemented but JSON-LD dominates in 2026) | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/microdata.html ; https://developer.mozilla.org/en-US/docs/Web/HTML/Microdata |
| 74 | ARIA reflection (the `role` and `aria-*` attributes) | all | not-started (parser passes them through; a11y tree is chunk 8) | >=90% | P3 | L | all yes/stable | https://www.w3.org/TR/wai-aria-1.2/ ; https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA |
| 75 | `data-*` custom data attributes (with `dataset` IDL reflection) | all | not-started (parsed; `dataset` getter is M5+) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/dom.html#embedding-custom-non-visible-data-with-the-data-attributes ; https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dataset |
| 76 | Popover (`popover` attribute, `popovertarget`, top-layer, light dismiss) | all | not-started | 75–90% (Baseline 2024) | P3 | L | all yes/stable | https://html.spec.whatwg.org/multipage/popover.html ; https://developer.mozilla.org/en-US/docs/Web/API/Popover_API |
| 77 | Anchor positioning (`anchor` attribute, `position-anchor` CSS, `position-try` / `position-try-options`, `position-visibility`) | all | not-started | 25–50% (Chromium 125+; WebKit/Gecko in progress) | P3 | L | behind flag in some engines; spec recently stabilised | https://drafts.csswg.org/css-anchor-position-1/ ; https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_anchor_positioning |

### A.1.13 HTML parser internals

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 78 | Insertion-mode machine (full 33-mode state machine incl. tables, select, in-cell, template, foreign content) | all | partial (8 modes; GAP 1.1 — "the full insertion-mode machine (tables, select, template, foreign content) lands in M5+") | >=90% | P2 / P3 | XL | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#the-insertion-mode |
| 79 | Active formatting elements (the `afe` list used by the adoption agency) | all | not-started (dependent on #64) | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#active-formatting-elements |
| 80 | Stack of open elements + common ancestor / appropriate end-tag lookups | all | shipped (per `spiral-fmt/src/html/tree.rs` InsertionMode) | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#stack-of-open-elements |
| 81 | Quirk mode classifier (per `Content-Type` sniffing + doctype + comments) | all | not-started (GAP 1.1) | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#how-to-determine-the-quirks-mode |
| 82 | `<template>` content document-fragment construction | all | not-started (GAP 1.3) | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#template-contents-owner |
| 83 | Fragment parsing algorithm (`DOMParser.parseFragment`, `document.implementation.createHTMLDocument`) | all | not-started | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/parsing.html#html-fragment-parsing-algorithm |

---

## A.2 WHATWG DOM — interfaces and behaviour

### A.2.1 Core tree interfaces

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 84 | `Node` (12 `nodeType` constants: `ELEMENT_NODE`, `ATTRIBUTE_NODE`, `TEXT_NODE`, `CDATA_SECTION_NODE`, `COMMENT_NODE`, `DOCUMENT_FRAGMENT_NODE`, `PROCESSING_INSTRUCTION_NODE`, `DOCUMENT_TYPE_NODE`, `DOCUMENT_NODE`, etc.) | all | partial (`Node` enum has 4 variants: Element/Text/Comment/Document; GAP 1.3 — "DOCTYPE node variant [ ]", "Document fragment [ ]") | >=90% | P2 | M | all yes/stable; Spiral lacks `DocumentType`, `CDATA`, `ProcessingInstruction`, `DocumentFragment` | https://dom.spec.whatwg.org/#interface-node ; https://developer.mozilla.org/en-US/docs/Web/API/Node |
| 85 | `Element` (id, className, tagName, attributes NamedNodeMap, namespace, prefix, localName, classList) | all | partial (`Element` struct exists with attrs `Vec<(String, String)>`; no `classList`/`getAttribute`/`setAttribute` IDL surface; GAP 1.3) | >=90% | P2 | M | all yes/stable | https://dom.spec.whatwg.org/#interface-element ; https://developer.mozilla.org/en-US/docs/Web/API/Element |
| 86 | `Document` (URL, documentURI, compatMode, characterSet, contentType, doctype, documentElement, body, head, title, forms, images, links, scripts, anchors, readyState, currentScript, implementation, location, defaultView, activeElement, hasFocus) | all | not-started (DOM has `Document` variant; IDL surface in HTML document section not yet built; GAP 1.3) | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#interface-document ; https://developer.mozilla.org/en-US/docs/Web/API/Document |
| 87 | `DocumentType` (`name`, `publicId`, `systemId`) | all | not-started (GAP 1.3) | >=90% | P2 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-documenttype ; https://developer.mozilla.org/en-US/docs/Web/API/DocumentType |
| 88 | `DocumentFragment` (light-DOM container; `children`, `querySelector`, `append`, `prepend`, `replaceChildren`) | all | not-started (GAP 1.3) | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#interface-documentfragment ; https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment |
| 89 | `Text` (`data`, `length`, `wholeText`, `replaceData`, `appendData`, `splitText`) | all | partial (`Text` struct exists; no methods) | >=90% | P2 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-text ; https://developer.mozilla.org/en-US/docs/Web/API/Text |
| 90 | `Comment` (`data`, `length`) | all | partial (`Comment` struct exists) | >=90% | P2 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-comment ; https://developer.mozilla.org/en-US/docs/Web/API/Comment |
| 91 | `CDATASection` (text-only XML escape) | all | not-started | >=90% | P2 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-cdatasection ; https://developer.mozilla.org/en-US/docs/Web/API/CDATASection |
| 92 | `ProcessingInstruction` (`target`, `data`) | all | not-started | 50–75% (used in SVG) | P2 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-processinginstruction ; https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction |
| 93 | `Attr` (`name`, `value`, `namespaceURI`, `prefix`, `localName`, `ownerElement`) | all | not-started (DOM stores `Vec<(String, String)>` pairs; no `Attr` interface) | >=90% | P2 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-attr ; https://developer.mozilla.org/en-US/docs/Web/API/Attr |
| 94 | `DOMTokenList` (`classList`, `relList`, `sandbox`, `linkSizes`, etc.; `add`/`remove`/`toggle`/`contains`/`replace`/`supports`/`value`/`length`/`entries`) | all | not-started | >=90% | P2 | M | all yes/stable | https://dom.spec.whatwg.org/#interface-domtokenlist ; https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList |
| 95 | `NamedNodeMap` (attribute collection; `length`, `item`, `getNamedItem`, `setNamedItem`, `removeNamedItem`) | all | not-started | >=90% | P2 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-namednodemap ; https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap |
| 96 | `HTMLCollection` (live ordered collection; `length`, `item`, `namedItem`) | all | not-started | >=90% | P2 | M | all yes/stable | https://dom.spec.whatwg.org/#interface-htmlcollection ; https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection |
| 97 | `NodeList` (static or live; `length`, `item`, `entries`/`keys`/`values`/`forEach`) | all | not-started | >=90% | P2 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-nodelist ; https://developer.mozilla.org/en-US/docs/Web/API/NodeList |

### A.2.2 Tree mutation operations

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 98 | `appendChild`, `removeChild`, `replaceChild`, `insertBefore` | all | not-started (GAP 1.3 — "open-codes insert-before via remove+re-append. DOM API itself lacks it") | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#dom-node-appendchild ; https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild |
| 99 | `Node.append`, `Node.prepend`, `Node.replaceChildren`, `Node.remove` (the `ParentNode` mixin) | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#dom-parentnode-append ; https://developer.mozilla.org/en-US/docs/Web/API/ParentNode |
| 100 | `Element.before`, `Element.after`, `Element.replaceWith`, `Element.insertAdjacentElement` | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#dom-childnode-before ; https://developer.mozilla.org/en-US/docs/Web/API/Element/before |
| 101 | `Element.innerHTML`, `Element.outerHTML`, `Element.insertAdjacentHTML` | all | not-started | >=90% | P3 | L | all yes/stable (full HTML parser invocation) | https://w3c.github.io/DOM-Parsing/ ; https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML |
| 102 | `Document.parseHTMLUnsafe`, `Document.parseHTML` (proposed; safe variant stripping XSS vectors) | all | not-started | 25–50% (Chromium 137+) | P3 | M | Chromium yes; others in progress | https://github.com/WICG/declarative-partial-shadow-dom ; https://developer.mozilla.org/en-US/docs/Web/API/Document/parseHTMLUnsafe |
| 103 | `Element.cloneNode`, `Document.importNode`, `Node.cloneNode` | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#dom-node-clonenode ; https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode |
| 104 | `Document.adoptNode`, `Node.isConnected`, `Node.getRootNode` | all | not-started | >=90% | P3 | S | all yes/stable | https://dom.spec.whatwg.org/#dom-document-adoptnode ; https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptNode |
| 105 | `TreeWalker`, `NodeIterator` (depth-first / breadth-first traversals with `whatToShow`/`NodeFilter`) | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#interface-treewalker ; https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker |

### A.2.3 Selectors / element lookup

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 106 | `Element.getElementById`, `Document.getElementById` | all | not-started (GAP 1.3) | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#dom-nonelementparentnode-getelementbyid ; https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById |
| 107 | `Element.getElementsByClassName`, `Element.getElementsByTagName`, `Element.getElementsByTagNameNS` | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#dom-element-getelementsbyclassname ; https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByClassName |
| 108 | `Element.querySelector`, `Element.querySelectorAll` (with selectors parsed by `spiral-fmt::css::selector`) | all | not-started (selector parser exists; matcher not) | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#dom-parentnode-queryselector ; https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector |
| 109 | `Document.querySelector`, `Document.querySelectorAll` | all | not-started | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#dom-parentnode-queryselector ; https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector |
| 110 | `:scope` (explicit scope reference inside `:scope :is()` / `Element.query(':scope > div')`) | all | not-started | >=90% | P3 | S | all yes/stable | https://dom.spec.whatwg.org/#dom-parentnode-queryselectorall ; https://developer.mozilla.org/en-US/docs/Web/CSS/:scope |
| 111 | `Element.closest` (selector match against self and ancestors) | all | not-started | >=90% | P3 | S | all yes/stable | https://dom.spec.whatwg.org/#dom-element-closest ; https://developer.mozilla.org/en-US/docs/Web/API/Element/closest |
| 112 | `Element.matches` (does this element match the selector) | all | not-started | >=90% | P3 | S | all yes/stable | https://dom.spec.whatwg.org/#dom-element-matches ; https://developer.mozilla.org/en-US/docs/Web/API/Element/matches |
| 113 | `Document.createElement`, `Document.createElementNS`, `Document.createTextNode`, `Document.createComment`, `Document.createDocumentFragment` | all | not-started (constructors for the new `Dom` exist; `Document` IDL not built) | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#dom-document-createelement ; https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement |
| 114 | `Node.contains`, `Node.compareDocumentPosition`, `Node.isEqualNode`, `Node.isSameNode` | all | not-started | >=90% | P3 | S | all yes/stable | https://dom.spec.whatwg.org/#dom-node-contains ; https://developer.mozilla.org/en-US/docs/Web/API/Node/contains |

### A.2.4 Events (DOM event model)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 115 | `EventTarget` (`addEventListener`, `removeEventListener`, `dispatchEvent`) | all | not-started (GAP 1.6 — DOM bindings stub) | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#interface-eventtarget ; https://developer.mozilla.org/en-US/docs/Web/API/EventTarget |
| 116 | `Event` (`type`, `target`, `currentTarget`, `eventPhase`, `bubbles`, `cancelable`, `composed`, `defaultPrevented`, `preventDefault`, `stopPropagation`, `stopImmediatePropagation`, `timeStamp`) | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#interface-event ; https://developer.mozilla.org/en-US/docs/Web/API/Event |
| 117 | `CustomEvent` (`detail`, `init`) | all | not-started | >=90% | P3 | S | all yes/stable | https://dom.spec.whatwg.org/#interface-customevent ; https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent |
| 118 | `Event` constructors (`new MouseEvent(...)`, `new KeyboardEvent(...)`, etc.) | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#events ; https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent |
| 119 | `UIEvent`, `MouseEvent`, `KeyboardEvent`, `FocusEvent`, `InputEvent`, `PointerEvent`, `WheelEvent`, `CompositionEvent`, `BeforeUnloadEvent` (UI sub-set) | all | not-started | >=90% | P3 | L | all yes/stable; `PointerEvent` Baseline | https://w3c.github.io/uievents/ ; https://developer.mozilla.org/en-US/docs/Web/API/UIEvent |
| 120 | Event capturing / bubbling model + `stopPropagation` ordering | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#concept-event-dispatch |
| 121 | `Event.composed` + retargeting across shadow boundaries (see A.1.9 #60) | all | not-started (covered as part of #60) | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#events-composed-path |
| 122 | `AbortController` / `AbortSignal` (used to cancel fetch, timers, etc.) | all | not-started | >=90% | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#interface-abortcontroller ; https://developer.mozilla.org/en-US/docs/Web/API/AbortController |
| 123 | `addEventListener` options (`capture`, `once`, `passive`, `signal`) | all | not-started | >=90% | P3 | S | all yes/stable | https://dom.spec.whatwg.org/#dom-eventtarget-addeventlistener ; https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener |

### A.2.5 Mutation observers and ranges

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 124 | `MutationObserver` (`observe` with `attributes`, `childList`, `subtree`, `characterData`, `attributeOldValue`, `characterDataOldValue`, `attributeFilter`), `MutationRecord` | all | not-started (GAP 1.3) | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#interface-mutationobserver ; https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver |
| 125 | `Range` (`setStart`, `setEnd`, `setStartBefore`, `setEndAfter`, `selectNode`, `selectNodeContents`, `collapse`, `cloneContents`, `extractContents`, `insertNode`, `deleteContents`, `surroundContents`, `comparePoint`, `intersectsNode`, `isPointInRange`, `getBoundingClientRect`, `getClientRects`) | all | not-started | >=90% | P3 | L | all yes/stable | https://dom.spec.whatwg.org/#interface-range ; https://developer.mozilla.org/en-US/docs/Web/API/Range |
| 126 | `Selection` (`getSelection`, `rangeCount`, `anchorNode`, `focusNode`, `addRange`, `removeRange`, `removeAllRanges`, `collapse`, `extend`, `setBaseAndExtent`, `selectAllChildren`, `deleteFromDocument`, `containsNode`, `modify`) | all | not-started | >=90% | P3 | L | all yes/stable | https://w3c.github.io/selection-api/ ; https://developer.mozilla.org/en-US/docs/Web/API/Selection |
| 127 | Static `Range` methods (`Range.getBoundingClientRect`, `Range.getClientRects`, `Range.intersectsNode`, `Range.pointFromNode`) | all | not-started | 75–90% (Baseline 2024) | P3 | M | all yes/stable | https://dom.spec.whatwg.org/#interface-range ; https://developer.mozilla.org/en-US/docs/Web/API/Range/getClientRects |

### A.2.6 Geometric / layout-reading interfaces

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 128 | `Element.getBoundingClientRect` (`DOMRect` with `x`, `y`, `width`, `height`, `top`, `right`, `bottom`, `left`) | all | not-started (depends on Gyre layout) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/cssom-view/#dom-element-getboundingclientrect ; https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect |
| 129 | `Element.getClientRects` (text line boxes) | all | not-started (depends on inline / text layout) | >=90% | P3 / P5 | L | all yes/stable | https://drafts.csswg.org/cssom-view/#dom-element-getclientrects ; https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects |
| 130 | `Element.scrollIntoView`, `Element.scrollTo`, `Element.scroll` (and `Element.scrollTop`/`Left`/`Width`/`Height`) | all | not-started (depends on layout + paint) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/cssom-view/#dom-element-scrollintoview ; https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView |
| 131 | `IntersectionObserver` (`root`, `rootMargin`, `thresholds`, `observe`, `unobserve`, `disconnect`, entries) | all | not-started | >=90% | P3 | L | all yes/stable | https://w3c.github.io/IntersectionObserver/ ; https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver |
| 132 | `ResizeObserver` (`observe`, `unobserve`, `disconnect`, `contentBoxSize`, `borderBoxSize`, `devicePixelContentBoxSize`) | all | not-started | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/resize-observer/ ; https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver |
| 133 | `DOMRect`, `DOMRectReadOnly`, `DOMPoint`, `DOMMatrix` (geometry types) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.fxtf.org/geometry-1/ ; https://developer.mozilla.org/en-US/docs/Web/API/DOMRect |

### A.2.7 CSSOM — StyleSheet interfaces (document-side; selector/value/box model in `01-feature-inventory-css.md`)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 134 | `StyleSheet` abstract (inherited by `CSSStyleSheet`, `CSSGroupingRule`, `CSSConditionRule`) | all | not-started (parser produces `Stylesheet`; no `CSSStyleSheet` IDL wrapper; GAP 1.2) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/cssom-1/#stylesheet |
| 135 | `CSSStyleSheet` (`insertRule`, `deleteRule`, `cssRules`, `ownerNode`, `title`, `href`, `type`, `media`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/cssom-1/#cssstylesheet ; https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet |
| 136 | `CSSRule`, `CSSStyleRule`, `CSSImportRule`, `CSSMediaRule`, `CSSKeyframesRule`, `CSSKeyframeRule`, `CSSSupportsRule`, `CSSContainerRule`, `CSSLayerBlockRule`, `CSSLayerStatementRule`, `CSSNamespaceRule`, `CSSFontFaceRule`, `CSSCounterStyleRule` (full hierarchy) | all | not-started (parser produces `Rule` enum; no `CSSRule` IDL wrapper) | >=90% | P3 | L | all yes/stable | https://drafts.csswg.org/cssom-1/#cssrule ; https://developer.mozilla.org/en-US/docs/Web/API/CSSRule |
| 137 | `CSSStyleDeclaration` (`cssText`, `length`, `item`, `getPropertyValue`, `setProperty`, `removeProperty`) | all | not-started | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/cssom-1/#cssstyledeclaration ; https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration |
| 138 | `Element.style` (inline `CSSStyleDeclaration`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/cssom-1/#dom-elementcssinlinestyle-style ; https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/style |
| 139 | `getComputedStyle` (`window.getComputedStyle(element, pseudoElt?)` → live `CSSStyleDeclaration`) | all | not-started (depends on cascade + computed-value resolution) | >=90% | P3 / P4 | L | all yes/stable | https://drafts.csswg.org/cssom-1/#dom-window-getcomputedstyle ; https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle |
| 140 | `StyleMedia` (`window.styleMedia`, `matchMedium(mediaQuery)`) | all | not-started | >=90% | P3 | S | all yes/stable | https://drafts.csswg.org/cssom-view/#the-stylemedia-interface ; https://developer.mozilla.org/en-US/docs/Web/API/StyleMedia |
| 141 | `MediaQueryList` (`window.matchMedia`, `mql.matches`, `mql.media`, `addEventListener('change', …)`, `addListener`, `removeListener`) | all | not-started (parser tokenises `@media`; match-eval not built) | >=90% | P3 | M | all yes/stable | https://drafts.csswg.org/cssom-view/#the-mediaquerylist-interface ; https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList |
| 142 | Constructable Stylesheets (`new CSSStyleSheet()`, `adoptedStyleSheets`) (see A.1.9 #62) | all | not-started | 75–90% | P3 | M | all yes/stable | https://wicg.github.io/construct-stylesheets/ |

### A.2.8 ElementInternals and other DOM sub-systems

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 143 | `ElementInternals` (form-associated custom elements, `form`, `validity`, `validationMessage`, `setFormValue`, `setValidity`, `labels`, `willValidate`, `checkValidity`, `reportValidity`) | all | not-started | 75–90% (Baseline 2024) | P4 | L | all yes/stable | https://html.spec.whatwg.org/multipage/custom-elements.html#element-internals ; https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals |
| 144 | `HTMLFormElement.elements`, `HTMLFormElement.submit`, `HTMLFormElement.reset`, `HTMLFormElement.checkValidity`, `HTMLFormElement.action`, `HTMLFormElement.method` | all | not-started (chunk 4 territory) | >=90% | P4 | L | all yes/stable | https://html.spec.whatwg.org/multipage/forms.html#the-form-element |
| 145 | `ValidityState` (`valueMissing`, `typeMismatch`, `patternMismatch`, `tooLong`, `tooShort`, `rangeUnderflow`, `rangeOverflow`, `stepMismatch`, `badInput`, `customError`, `valid`) | all | not-started | >=90% | P4 | M | all yes/stable | https://html.spec.whatwg.org/multipage/forms.html#the-validitystate-interface ; https://developer.mozilla.org/en-US/docs/Web/API/ValidityState |

---

## A.3 ECMAScript (ECMA-262) — engine-level

### A.3.1 Lexical grammar

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 146 | Source text (UTF-16, with optional BOM, line terminators) | all | shipped (Vortex lexer `Cursor`) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-source-text |
| 147 | Input element division: tokens (identifier, keyword, punctuator, numeric, string, template, regular-expression) | all | shipped (Vortex `lexer` module per GAP 1.6) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-ecmascript-language-lexical-grammar |
| 148 | Identifier resolution including Unicode escape sequences (`\u00E9`), `ZWNJ`/`ZWJ`, and the full `ID_Start` / `ID_Continue` Unicode property | all | partial (lex tokenises; full Unicode tables not exhaustive) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-names-and-keywords ; https://tc39.es/ecma262/#prod-IdentifierName |
| 149 | Reserved words (current and contextual: `await`, `yield`, `let`, `static`, `from`, `of`, `as`, `async`, etc.) | all | shipped (Vortex parser) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-keywords-and-reserved-words |
| 150 | Numeric literals (decimal, binary `0b`, octal `0o`, hex `0x`, BigInt `0n`; underscore separators) | all | shipped (per Vortex lexer AST coverage) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-numeric-literals |
| 151 | String literals (single/double quoted, with all escape sequences incl. line continuation; line terminator restrictions) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-string-literals |
| 152 | Template literals (no-substitution template, `${expr}` substitution, tag functions, raw form via `String.raw`) | all | partial (parser yes; tag functions yes) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-template-literals |
| 153 | Regular expression literals (`/pattern/flags`, with Unicode `u`, `v`, dotAll `s`, sticky `y`, named groups, lookbehind, Unicode property escapes) | all | partial (lex tokenises; `RegExp` built-in coverage partial) | >=90% | P2 | L | all yes/stable | https://tc39.es/ecma262/#sec-regular-expression-literals ; https://tc39.es/ecma262/#prod-RegularExpressionLiteral |
| 154 | Automatic Semicolon Insertion (ASI) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-automatic-semicolon-insertion |
| 155 | Hashbang grammar (`#!/usr/bin/env node` at the top of a script) | all | not-started (recent ES2024 addition) | >=90% (ES2024) | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-hashbang |

### A.3.2 Expressions

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 156 | Primary expressions (`this`, `null`, `true`/`false`, `undefined`, literals, `[…]`, `{…}`, `(expr)`, function expression, class expression, generator, async function) | all | shipped (Vortex AST) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-primary-expression |
| 157 | Property access (member and computed) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-property-accessors |
| 158 | `new` operator with argument list | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-new-operator |
| 159 | Function calls (direct, method, `eval`, indirect `eval`, optional chaining `?.`, tail calls) | all | partial (basic yes; tail calls partial; proper tail calls in spec semantics only ES2017) | >=90% | P2 | M | all yes/stable (proper tail call semantics: yes) | https://tc39.es/ecma262/#sec-function-calls ; https://tc39.es/ecma262/#sec-optional-chains ; https://tc39.es/ecma262/#sec-tail-position-calls |
| 160 | Left-hand-side expressions incl. `super`, `import.meta`, `import()` (dynamic import), `new.target`, `new.super` | all | partial (Vortex AST has ES2015+ types) | >=90% | P2 / P3 | M | all yes/stable | https://tc39.es/ecma262/#sec-left-hand-side-expressions |
| 161 | Update expressions (`++`, `--`, prefix and postfix) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-update-expressions |
| 162 | Unary operators (`!`, `~`, `+`, `-`, `typeof`, `void`, `delete`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-unary-operators |
| 163 | Binary operators (arithmetic, bitwise, comparison, equality, `in`, `instanceof`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-multiplicative-operators ; https://tc39.es/ecma262/#sec-additive-operators ; https://tc39.es/ecma262/#sec-bitwise-shift-operators ; https://tc39.es/ecma262/#sec-relational-operators ; https://tc39.es/ecma262/#sec-equality-operators |
| 164 | Logical operators (`&&`, `||`, `??`, `&&=`, `||=`, `??=`) | all | shipped (per ES2021 logical assignment) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-binary-logical-operators ; https://tc39.es/ecma262/#sec-assignment-operators |
| 165 | Conditional expression (`a ? b : c`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-conditional-operator |
| 166 | Assignment expressions (simple, destructuring) | all | shipped (AST yes) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-assignment-operators |
| 167 | Sequence / comma operator | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-comma-operator |
| 168 | Spread element in call and array literal, rest in parameter list and destructuring | all | shipped (ES2015+) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-spread-syntax ; https://tc39.es/ecma262/#sec-destructuring-assignment |
| 169 | Arrow functions (`x => x * 2`, including expression body and block body) | all | shipped (per Vortex partial) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-arrow-function-definitions |
| 170 | Async/await expressions (including `await` at module top level) | all | partial (Vortex interpreter; needs bytecode VM for real perf) | >=90% | P2 | L | all yes/stable | https://tc39.es/ecma262/#sec-async-function-definitions ; https://tc39.es/ecma262/#sec-top-level-await |
| 171 | Destructuring patterns (array, object, nested, with defaults, with rest, with rename) | all | shipped (AST yes) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-destructuring-assignment ; https://tc39.es/ecma262/#sec-destructuring-binding-patterns |
| 172 | Class expressions (with `extends`, `static`, computed method names, `#privateName`, `static #privateName`, `static {}` blocks) | all | partial (Vortex partial) | >=90% | P2 | L | all yes/stable | https://tc39.es/ecma262/#sec-class-definitions ; https://tc39.es/ecma262/#sec-class-static-initialization-blocks |
| 173 | Generator function expressions (`function*`, `yield`, `yield*`) | all | partial (Vortex partial) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-generator-function-definitions |
| 174 | Async generator function expressions (`async function*`, `for await…of`) | all | partial (Vortex partial) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-async-generator-function-definitions |
| 175 | BigInt literals and operations (`123n`, `BigInt('456')`, `BigInt.asIntN`, `BigInt.asUintN`) | all | partial (Vortex has `BigInt` type per `builtins/mod.rs`) | >=90% | P2 / P5 | M | all yes/stable | https://tc39.es/ecma262/#sec-ecmascript-language-types-bigint-type ; https://tc39.es/ecma262/#sec-bigint-objects |

### A.3.3 Statements

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 176 | Expression / declaration statements (incl. `let`, `const`, `var`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-expression-statement ; https://tc39.es/ecma262/#sec-variable-statement ; https://tc39.es/ecma262/#sec-let-and-const-declarations |
| 177 | Block statement (`{ … }`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-block |
| 178 | `if` / `else` | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-if-statement |
| 179 | `while` / `do-while` | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-while-statement ; https://tc39.es/ecma262/#sec-do-while-statement |
| 180 | `for` (three-clause C-style) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-for-statement |
| 181 | `for-in` (enumerable own and inherited string-keyed properties) | all | partial | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-for-in-and-for-of-statements |
| 182 | `for-of` (iterator protocol; works with any object exposing `@@iterator`) | all | partial | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-for-in-and-for-of-statements |
| 183 | `switch` | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-switch-statement |
| 184 | `return`, `break`, `continue`, `throw`, `try`/`catch`/`finally` | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-return-statement ; https://tc39.es/ecma262/#sec-break-statement ; https://tc39.es/ecma262/#sec-continue-statement ; https://tc39.es/ecma262/#sec-throw-statement ; https://tc39.es/ecma262/#sec-try-statement |
| 185 | `with` statement (Annex B) | all | not-started (deprecated in strict mode) | >=90% (Annex B) | P2 | S | all yes/stable (Annex B); disallowed in strict | https://tc39.es/ecma262/#sec-with-statement |
| 186 | Labelled statement, `break label`, `continue label` | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-labelled-statements |
| 187 | Empty statement (`;`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-empty-statement |
| 188 | `debugger` statement (no-op) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-debugger-statement |

### A.3.4 Functions, classes, modules

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 189 | Function declarations and expressions (incl. default parameters, rest, destructuring) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-function-definitions |
| 190 | Lexical environment / scope chain + closures | all | partial (Vortex interpreter has closures) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-lexical-environments |
| 191 | Strict mode (the "use strict" directive and module mode) | all | partial (Vortex partial) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-strict-mode-code |
| 192 | Class declarations (with constructor, methods, getters/setters, `static`, computed, `#privateBrand`, `static {}`) | all | partial (Vortex partial) | >=90% | P2 | L | all yes/stable | https://tc39.es/ecma262/#sec-class-definitions ; https://tc39.es/proposal-class-fields ; https://tc39.es/proposal-private-methods ; https://tc39.es/proposal-static-class-features |
| 193 | Class fields (public and `#private`) | all | partial | >=90% | P2 | M | all yes/stable | https://tc39.es/proposal-class-fields |
| 194 | Private methods and accessors (`#privateMethod`, `#privateGetter`/`#privateSetter`) | all | partial | >=90% | P2 | M | all yes/stable | https://tc39.es/proposal-private-methods |
| 195 | Module syntax (`import` / `export`, `import.meta.url`, `import('...')` dynamic, top-level `await`) | all | partial (Vortex partial) | >=90% | P3 | L | all yes/stable | https://tc39.es/ecma262/#sec-modules ; https://tc39.es/ecma262/#sec-imports ; https://tc39.es/ecma262/#sec-exports ; https://tc39.es/proposal-top-level-await |
| 196 | `Symbol` (incl. `Symbol.iterator`, `Symbol.asyncIterator`, `Symbol.hasInstance`, `Symbol.toPrimitive`, `Symbol.toStringTag`, `Symbol.species`, `Symbol.for`, `Symbol.keyFor`) | all | partial (Vortex has `Symbol`) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-symbol-value ; https://tc39.es/ecma262/#sec-well-known-symbols |
| 197 | `Proxy` (handler traps: `get`, `set`, `has`, `deleteProperty`, `apply`, `construct`, `ownKeys`, `getOwnPropertyDescriptor`, `defineProperty`, `getPrototypeOf`, `setPrototypeOf`, `preventExtensions`, `isExtensible`, `revocable`) | all | not-started | >=90% | P2 / P5 | XL | all yes/stable | https://tc39.es/ecma262/#sec-proxy-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy |
| 198 | `Reflect` (the static reflection namespace: `Reflect.get`, `set`, `has`, `deleteProperty`, `apply`, `construct`, `ownKeys`, `getOwnPropertyDescriptor`, `defineProperty`, `getPrototypeOf`, `setPrototypeOf`, `preventExtensions`, `isExtensible`) | all | not-started | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-reflect-object ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect |
| 199 | Iterator protocol (`@@iterator`, `next()` returning `{ value, done }`) and the `function*` async variant | all | partial (Vortex partial) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-iteration |
| 200 | `await using` / explicit resource management (`Symbol.dispose`, `Symbol.asyncDispose`) | all | not-started (ES2025) | 25–50% (new) | P3 | M | Chromium 134+; others in progress | https://github.com/tc39/proposal-explicit-resource-management |

### A.3.5 Built-in objects — value / utility

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 201 | `Object` (assign, create, defineProperties, defineProperty, entries, freeze, fromEntries, getOwnPropertyDescriptor, getOwnPropertyDescriptors, getOwnPropertyNames, getOwnPropertySymbols, getPrototypeOf, is, isExtensible, isFrozen, isSealed, keys, preventExtensions, seal, setPrototypeOf, values) | all | partial (Vortex has Object) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-objects-and-symbols ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object |
| 202 | `Array` (from, of, isArray, concat, copyWithin, entries, every, fill, filter, find, findIndex, findLast, findLastIndex, flat, flatMap, forEach, includes, indexOf, join, keys, lastIndexOf, map, pop, push, reduce, reduceRight, reverse, shift, slice, some, sort, splice, toLocaleString, toReversed, toSorted, toSpliced, unshift, values, with) | all | partial (Vortex has Array; copes, flatMap, sort, slice, splice, concat, etc.) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-array-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array |
| 203 | `ArrayBuffer` and `SharedArrayBuffer` (the latter requires cross-origin isolation headers) | all | partial (Vortex heap is arena, not yet exposed as `ArrayBuffer`) | >=90% | P3 | L | all yes/stable; `SharedArrayBuffer` requires COOP/COEP | https://tc39.es/ecma262/#sec-arraybuffer-objects ; https://tc39.es/ecma262/#sec-sharedarraybuffer-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer |
| 204 | `DataView` (typed array view with explicit endian and byte offset) | all | not-started | >=90% | P3 | M | all yes/stable | https://tc39.es/ecma262/#sec-dataview-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView |
| 205 | Typed arrays (Int8Array, Uint8Array, Uint8ClampedArray, Int16Array, Uint16Array, Int32Array, Uint32Array, Float32Array, Float64Array, BigInt64Array, BigUint64Array) | all | not-started | >=90% | P3 | L | all yes/stable | https://tc39.es/ecma262/#sec-typedarray-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray |
| 206 | `Boolean`, `Number` (incl. `EPSILON`, `MAX_SAFE_INTEGER`, `MIN_SAFE_INTEGER`, `parseInt`, `parseFloat`, `isFinite`, `isInteger`, `isNaN`, `isSafeInteger`) | all | shipped (per Vortex builtins list) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-boolean-objects ; https://tc39.es/ecma262/#sec-number-objects |
| 207 | `String` (fromCharCode, fromCodePoint, raw, includes, startsWith, endsWith, repeat, padStart, padEnd, trim, trimStart, trimEnd, slice, substring, substr, indexOf, lastIndexOf, toUpperCase, toLowerCase, localeCompare, normalize, match, matchAll, replace, replaceAll, search, split, at, isWellFormed, toWellFormed) | all | partial (Vortex has String per `builtins/mod.rs`) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-string-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String |
| 208 | `RegExp` (named groups, lookbehind, unicode property escapes, `flags`, `sticky`, `unicodeSets`, `hasIndices`) | all | partial | >=90% | P2 | L | all yes/stable | https://tc39.es/ecma262/#sec-regexp-regular-expression-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp |
| 209 | `Date` (`Date.parse`, `Date.UTC`, `Date.now`, `getTime`, `toISOString`, `toJSON`, `toLocaleString`, etc.) | all | partial | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-date-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date |
| 210 | `Temporal` (the modern replacement for `Date`; `Temporal.PlainDate`, `PlainTime`, `PlainDateTime`, `ZonedDateTime`, `Instant`, `Duration`, `Calendar`, `TimeZone`) | all | not-started | 25–50% (Baseline 2025) | P5 | XL | Chromium 144+ (2026), WebKit yes/stable, Gecko in progress | https://tc39.es/proposal-temporal ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal |
| 211 | `Math` (constants and 40+ methods) | all | shipped (Vortex builtins `math.rs`) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-math-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math |
| 212 | `JSON` (parse, stringify, with replacer / reviver / `toJSON`) | all | shipped (Vortex builtins per `builtins/mod.rs`) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-json-object ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON |
| 213 | `Map` and `Set` (with insertion-order iteration) | all | partial (Vortex has Map/Set per `builtins/mod.rs`) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-keyed-collection |
| 214 | `WeakMap` and `WeakSet` (entries weakly held; only object keys) | all | partial (Vortex has WeakMap/WeakSet per `builtins/mod.rs`) | >=90% | P2 / P5 | M | all yes/stable | https://tc39.es/ecma262/#sec-weakmap-objects ; https://tc39.es/ecma262/#sec-weakset-objects |
| 215 | `WeakRef` and `FinalizationRegistry` | all | not-started (Vortex `gc` is the engine; not yet exposed to JS) | >=90% | P2 / P5 | L | all yes/stable | https://tc39.es/proposal-weakrefs ; https://tc39.es/ecma262/#sec-weakref-objects ; https://tc39.es/ecma262/#sec-finalization-registry-objects |
| 216 | `Error` (and the sub-classes: `EvalError`, `RangeError`, `ReferenceError`, `SyntaxError`, `TypeError`, `URIError`, `AggregateError`) | all | partial (Vortex has `Error`, `TypeError`) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-error-objects ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error |
| 217 | `Promise` (with `all`, `allSettled`, `any`, `race`, `resolve`, `reject`, `withResolvers`, `try`) | all | partial (Vortex EventLoop; per `builtins/mod.rs` Promise is in scope) | >=90% | P2 | L | all yes/stable | https://tc39.es/ecma262/#sec-promise-objects ; https://tc39.es/proposal-promise-try ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise |
| 218 | `Atomics` and `SharedArrayBuffer` shared-memory atomic operations (`wait`, `notify`, `load`, `store`, `add`, `sub`, `and`, `or`, `xor`, `exchange`, `compareExchange`, `isLockFree`) | all | not-started | >=90% (where COOP/COEP is set) | P3 / P5 | XL | all yes/stable; requires isolation headers | https://tc39.es/ecma262/#sec-atomics-object ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics |
| 219 | `structuredClone` (deep-clone of any cloneable value) | all | not-started | >=90% | P2 | M | all yes/stable | https://html.spec.whatwg.org/multipage/structured-data.html#dom-structuredclone ; https://developer.mozilla.org/en-US/docs/Web/API/structuredClone |
| 220 | `globalThis` (universal reference to the global) | all | not-started | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-globalthis ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/globalThis |
| 221 | `queueMicrotask` (schedule a microtask) | all | shipped (Vortex EventLoop has `queue_microtask`) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#dom-queuemicrotask ; https://developer.mozilla.org/en-US/docs/Web/API/queueMicrotask |
| 222 | `eval` (direct and indirect) | all | partial (Vortex supports direct eval) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-eval-x ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval |
| 223 | `setTimeout` / `setInterval` / `clearTimeout` / `clearInterval` (host-defined timers) | all | shipped (Vortex EventLoop has both) | >=90% | P2 | S | all yes/stable | https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#dom-settimeout ; https://developer.mozilla.org/en-US/docs/Web/API/setTimeout |

### A.3.6 Control flow, iteration, annexes

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 224 | Iteration protocols: `for-of` (sync) + `for-await-of` (async) + `@@iterator` + `@@asyncIterator` | all | partial (Vortex partial) | >=90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-iteration |
| 225 | `Array.from` (with mapFn and `thisArg`), `Array.of` | all | shipped (Vortex builtins `array.rs`) | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-array.from ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/from |
| 226 | `Array.prototype.flat`, `Array.prototype.flatMap` | all | shipped (Vortex partial) | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-flatMap ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat |
| 227 | `Object.values`, `Object.entries`, `Object.fromEntries` | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-object.values ; https://tc39.es/ecma262/#sec-object.entries ; https://tc39.es/ecma262/#sec-object.fromentries |
| 228 | `String.prototype.replaceAll` | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-string.prototype.replaceall |
| 229 | Logical assignment (`&&=`, `||=`, `??=`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-logical-assignment |
| 230 | Numeric separators (`1_000_000`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-numeric-separator |
| 231 | Optional catch binding (`try { … } catch { … }`) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-optional-catch-binding |
| 232 | `Array.prototype.at`, `String.prototype.at`, `TypedArray.prototype.at` | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-relative-indexing-method |
| 233 | `Object.hasOwn` (preferred over `Object.prototype.hasOwnProperty.call`) | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-accessible-object-hasownproperty |
| 234 | `Error.cause` (the `new Error(msg, { cause })` chain) | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-error-cause |
| 235 | `Array.prototype.findLast` / `findLastIndex` | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-array-find-from-last |
| 236 | `Array.prototype.toReversed`, `toSorted`, `toSpliced`, `with` (non-mutating) | all | partial | 75–90% (Baseline 2024) | P2 | S | all yes/stable | https://tc39.es/proposal-change-array-by-copy |
| 237 | `Array.prototype.group`, `Array.prototype.groupToMap` | all | not-started | 75–90% (Baseline 2024) | P2 | M | all yes/stable | https://tc39.es/proposal-array-grouping |
| 238 | `Promise.allSettled`, `Promise.any` | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-promise-allSettled ; https://tc39.es/proposal-promise-any |
| 239 | `Promise.withResolvers` | all | partial | 75–90% (Baseline 2024) | P2 | S | all yes/stable | https://tc39.es/proposal-promise-with-resolvers |
| 240 | `Symbol.description` | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-symbol-description |
| 241 | `String.prototype.matchAll` | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-string-matchall |
| 242 | `String.prototype.replaceAll` | all | partial | >=90% | P2 | S | all yes/stable | https://tc39.es/proposal-string.prototype.replaceall |
| 243 | `String.prototype.trimStart` / `trimEnd` (alias for `trimLeft`/`trimRight` Annex B) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-string.prototype.trimstart ; https://tc39.es/ecma262/#sec-string.prototype.trimend |
| 244 | `String.prototype.replace` (with replacer function) | all | shipped | >=90% | P2 | S | all yes/stable | https://tc39.es/ecma262/#sec-string.prototype.replace |
| 245 | Annex B (additional legacy features: `Date.prototype.getYear`, `escape`, `unescape`, `String.prototype.substr`, `RegExp` left-context matching, `Object.prototype.__proto__`) | all | not-started (Annex B is opt-in per spec) | 75–90% (still required for parity; strict disallows) | P2 | M | all yes/stable (Annex B); strict disallows | https://tc39.es/ecma262/#sec-additional-ecmascript-features-for-web-browsers |
| 246 | Annex C (Web browsers: `eval` semantics, `Object.prototype.__defineGetter__`, `__defineSetter__`, `__lookupGetter__`, `__lookupSetter__`, `Error.prototype.stack`, `toString` enumeration) | all | partial (basic eval yes; rest no) | 75–90% | P2 | M | all yes/stable | https://tc39.es/ecma262/#sec-additional-ecmascript-features-for-web-browsers-annex-c |

### A.3.7 ECMAScript for XML (E4X) and other historical annexes

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 247 | E4X (Annex E, `new XML(...)` syntax) | not in any current browser (SpiderMonkey removed in FF 21) | n/a | <25% (only historical) | n/a | n/a | SpiderMonkey historical only; not in scope | https://web.archive.org/web/2015*/https://developer.mozilla.org/en-US/docs/Archive/Web/E4X |

### A.3.8 Internationalisation (ECMA-402 / `Intl`)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 248 | `Intl.Collator` (`compare`, `resolvedOptions`) | all | not-started | >=90% | P5 | L | all yes/stable | https://tc39.es/ecma402/#sec-intl-collator ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator |
| 249 | `Intl.DateTimeFormat` (with all `formatMatcher`, `dateStyle`, `timeStyle`, `calendar`, `numberingSystem`, `hour12`/`hourCycle`, `timeZone` options) | all | not-started | >=90% | P5 | L | all yes/stable | https://tc39.es/ecma402/#sec-intl-datetimeformat ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat |
| 250 | `Intl.ListFormat` (`conjuction`, `disjunction`, `unit` styles) | all | not-started | >=90% | P5 | M | all yes/stable | https://tc39.es/proposal-intl-list-format ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat |
| 251 | `Intl.NumberFormat` (with `notation`, `compactDisplay`, `useGrouping`, `currencyDisplay`, `currencySign`, `unit`, `unitDisplay`, `signDisplay`, `minimumIntegerDigits`, etc.) | all | not-started | >=90% | P5 | L | all yes/stable | https://tc39.es/ecma402/#sec-intl-numberformat ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat |
| 252 | `Intl.PluralRules` (`select`, `selectRange`) | all | not-started | >=90% | P5 | M | all yes/stable | https://tc39.es/ecma402/#sec-intl-pluralrules ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules |
| 253 | `Intl.RelativeTimeFormat` (`format`, `formatToParts`, `resolvedOptions`, `auto`, `always`, `numeric` styles) | all | not-started | >=90% | P5 | M | all yes/stable | https://tc39.es/proposal-intl-relative-time ; https://tc39.es/proposal-intl-relative-time-format ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat |
| 254 | `Intl.Segmenter` (locale-sensitive grapheme / word / sentence segmentation) | all | not-started | 75–90% (Baseline 2024) | P5 | M | all yes/stable | https://tc39.es/proposal-intl-segmenter ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter |
| 255 | `Intl.DurationFormat` (locale-sensitive duration formatting) | all | not-started | 75–90% (Baseline 2024) | P5 | M | all yes/stable | https://tc39.es/proposal-intl-duration-format ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat |
| 256 | `Intl.DisplayNames` (translated language, region, script, currency names) | all | not-started | 75–90% (Baseline 2024) | P5 | M | all yes/stable | https://tc39.es/proposal-intl-displaynames ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames |
| 257 | `Intl.getCanonicalLocales`, `Intl.supportedValuesOf` (`unit`, `currency`, `calendar`, `collation`, `hourCycle`, `numberingSystem`, `timeZone`) | all | not-started | 75–90% (Baseline 2024) | P5 | M | all yes/stable | https://tc39.es/ecma402/#sec-intl.getcanonicallocales ; https://tc39.es/proposal-intl-enumeration ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/supportedValuesOf |
| 258 | `Intl.Locale` (`minimize`, `maximize`, `baseName`, `language`, `script`, `region`, `variants`, `extensions`) | all | not-started | >=90% | P5 | M | all yes/stable | https://tc39.es/ecma402/#sec-intl-locale ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale |
| 259 | `Intl.MessageFormat` (Stage 3 in 2025; locale message template) | all | not-started | 25–50% (very recent) | P5 | L | spec just stabilised; engine coverage in flux | https://github.com/tc39/proposal-intl-messageformat |
| 260 | Locale data: BCP 47 language tags, CLDR ICU data, language fallback, ICU integration | all | not-started | >=90% | P5 | XL | all yes/stable (CLDR is the canonical data set) | https://cldr.unicode.org/ ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl |

### A.3.9 Host-defined objects the engine must provide

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 261 | `window` / `self` / `frames` / `top` / `parent` / `opener` / `closed` (window-browsing-context) | all | not-started (Vortex DOM bindings stub returns empty `Window` per `dom_bindings/mod.rs`) | >=90% | P3 | L | all yes/stable | https://html.spec.whatwg.org/multipage/nav-history-apis.html#the-window-object ; https://developer.mozilla.org/en-US/docs/Web/API/Window |
| 262 | `document` (initial object handed to scripts; references the DOM `Document`) | all | not-started (Vortex stub) | >=90% | P3 | L | all yes/stable | https://html.spec.whatwg.org/multipage/dom.html#the-document-object ; https://developer.mozilla.org/en-US/docs/Web/API/Document |
| 263 | `console` (`log`, `info`, `warn`, `error`, `debug`, `trace`, `assert`, `dir`, `dirxml`, `table`, `group`, `groupEnd`, `time`, `timeEnd`, `timeLog`, `count`, `clear`, `profile`, `profileEnd`) | all | partial (Vortex has `console` per `builtins/console.rs`; GAP 1.6 — "Not yet wired to `RendererToBrowser::ConsoleMessage` IPC") | >=90% | P2 | S | all yes/stable | https://console.spec.whatwg.org/ ; https://developer.mozilla.org/en-US/docs/Web/API/console |
| 264 | `URL` and `URLSearchParams` (the WHATWG URL parser) | all | not-started (chunk 2) | >=90% | P2 | L | all yes/stable | https://url.spec.whatwg.org/ ; https://developer.mozilla.org/en-US/docs/Web/API/URL |
| 265 | `URLPattern` (pathname / query / hash patterns) | all | not-started | 75–90% (Baseline 2024) | P3 | M | all yes/stable | https://urlpattern.spec.whatwg.org/ ; https://developer.mozilla.org/en-US/docs/Web/API/URLPattern |
| 266 | `TextEncoder` / `TextDecoder` (WHATWG Encoding) | all | not-started (chunk 2) | >=90% | P3 | M | all yes/stable | https://encoding.spec.whatwg.org/ ; https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder |
| 267 | `requestIdleCallback` / `requestAnimationFrame` / `cancelAnimationFrame` | all | partial (Vortex EventLoop has `requestAnimationFrame` per its doc) | >=90% | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/imagebitmap-and-animations.html ; https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame |
| 268 | `scheduler` (the Scheduler API: `scheduler.yield`, `scheduler.postTask`) | all | not-started | 25–50% (Chromium 129+) | P3 | M | Chromium only (yes/partial); others in progress | https://github.com/WICG/scheduling-apis ; https://developer.mozilla.org/en-US/docs/Web/API/Scheduler |
| 269 | `EventCounts` (the `performance.eventCounts` count of dispatched events) | all | not-started | 50–75% (Baseline 2024) | P3 | S | all yes/stable | https://w3c.github.io/event-timing/ ; https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEventTiming |

### A.3.10 Storage interface and engine-level quotas (chunk 4 covers full split; listed here for engine-level surface)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 270 | `localStorage` / `sessionStorage` (the Web Storage synchronous key-value store) | all | not-started (chunk 4) | >=90% | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/webstorage.html ; https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage |
| 271 | `IndexedDB` (the asynchronous object-store database) | all | not-started (chunk 4) | >=90% | P4 | XL | all yes/stable | https://w3c.github.io/IndexedDB/ ; https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API |
| 272 | `Cache` (the request / response cache used by service workers) | all | not-started (chunk 4) | >=90% | P4 | L | all yes/stable | https://w3c.github.io/ServiceWorker/ ; https://developer.mozilla.org/en-US/docs/Web/API/Cache |
| 273 | `navigator.storage.estimate` / `persist` (storage quota API) | all | not-started (chunk 4) | >=90% | P4 | M | all yes/stable | https://storage.spec.whatwg.org/ ; https://developer.mozilla.org/en-US/docs/Web/API/StorageManager |

### A.3.11 Engine-level sub-systems (Vortex / SpiderMonkey / JSC / V8 — language-engine concerns, not DOM)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 274 | Bytecode VM (register or stack-based, with type-specialised opcodes) | all | not-started (GAP 1.6 — "M10–24") | >=90% (every engine) | P3 | XL | all yes/stable | https://v8.dev/blog/v8-release-7.0 ; https://spidermonkey.dev/ ; https://developer.apple.com/documentation/javascriptcore |
| 275 | Baseline JIT compiler (Cranelift, Baseline) | all | not-started (Bet 2 — Vortex is JIT-Optional) | >=90% | P5 | XL | all yes/stable | https://cftb.ca/zelda/ ; https://github.com/bytecodealliance/wasmtime |
| 276 | Mark-sweep / generational / incremental GC (roots from real VM stack + globals) | all | partial (Vortex has 84 GC tests; per-origin arenas; VortexHeap ↔ Runtime glue; GAP 1.6 — "Roots from environment chain; not yet from VM stack") | >=90% | P2 | L | all yes/stable | https://en.wikipedia.org/wiki/Garbage_collection_(computer_science) ; https://wingolog.org/archives/2022/11/18/a-quick-look-at-spider-monkey-internals |
| 277 | Inline caches (polymorphic / megamorphic dispatch) | all | not-started | >=90% | P3 | L | all yes/stable | https://en.wikipedia.org/wiki/Inline_caching |
| 279 | WebAssembly (WASM) core spec + JS interop (`WebAssembly.Module`, `WebAssembly.Instance`, `WebAssembly.Memory`, `WebAssembly.Table`, `WebAssembly.Global`, `WebAssembly.Function`, `WebAssembly.CompileError`, `WebAssembly.LinkError`, `WebAssembly.RuntimeError`, streaming `compileStreaming`, `instantiateStreaming`, `Tag` and `Exception` for EH) | all | not-started | 75–90% (Baseline 2022) | P5 | XL | all yes/stable; Vortex will defer to Cranelift+WASMtime for production WASM runtime per Bet 2 | https://webassembly.github.io/spec/core/ ; https://webassembly.github.io/exception-handling/ ; https://developer.mozilla.org/en-US/docs/WebAssembly |
| 280 | SIMD (WebAssembly 128-bit packed types + the proposal-stage JS SIMD) | all | not-started | 50–75% (WASM SIMD Baseline) | P5 | XL | WASM yes/stable; JS SIMD no (proposal folded) | https://github.com/WebAssembly/spec/blob/main/proposals/simd/SIMD.md ; https://github.com/tc39/proposal-javascript-simd-data |
| 281 | Threads (`Atomics.wait`/`notify`, `SharedArrayBuffer`, `Worker`) | all | partial (Vortex event loop; no `Worker` yet; no `Atomics` yet) | >=90% (where COOP/COEP is set) | P3 / P5 | XL | all yes/stable | https://html.spec.whatwg.org/multipage/workers.html ; https://tc39.es/ecma262/#sec-atomics-object |

---

## A.4 Notes on scoring methodology specific to this file

- **"shipped" is rare on purpose.** A row is `shipped` only when the
  capability is reachable from a real surface (a `pub` symbol on
  `spiral-dom`, `spiral-vortex`, or `spiral-fmt`) AND an integration
  test exercises it. Per `AGENTS.md` (the wiring rule, adopted from
  `docs/decisions/0006-cross-cutting-features.md` in the upstream
  Zeus repo). The M4.4 SSOT restructure has been audited against
  this — 12 of 48 prior orphans wired via integration tests in M4.4.
- **"partial" is the modal status.** M4.4 was a vendored-parsers
  sprint. Lexer, parser, and tree-builder are in; downstream
  integration (Gyre cascade, Vortex DOM bindings, CSS value
  resolution) is the next year's work.
- **"designed" never appears in this file.** Where there is a
  design doc or ADR for a row, it is in the relevant area (`docs/architecture/`
  for Gyre, `docs/design-vortex-heap.md` for Vortex's heap), and
  the row says "not-started" because no code exists yet. This is
  deliberate — `designed` is reserved for code-in-flight cases
  (e.g. ongoing branch work) per the M4.4 review.
- **The "Phase" column maps Spiral-internal phases (P2 core engine
  → P3 app shell → P4 platform → P5 advanced) to the external
  standards body, not the other way around.** A `>=90%` prevalence
  feature still hits P5 if its engine work is JIT-grade.

---

## A.5 Cross-references to `specs/GAP_ANALYSIS.md`

| Row in this file | GAP_ANALYSIS row | Comment |
|------------------|------------------|---------|
| 1, 66, 78–83 | §1.1 (HTML parser) | row 1 is `<!DOCTYPE>` quirks, row 66 is encoding sniff, row 78 is full insertion-mode machine |
| 8, 134–142 | §1.2 (CSS parser & cascade) | row 8 is `<style>` integration; 134–142 are CSSOM `StyleSheet` API |
| 63, 82, 84–114, 124, 126, 144, 145 | §1.3 (DOM) | DOM tree yes (rows 84–90), IDL surface no (rows 86, 88, 91–114, 124, 126, 144, 145) |
| 128–132, 139 | §1.4 (Layout — Gyre) | row 128 is `getBoundingClientRect`, row 139 is `getComputedStyle` — both block on layout completion |
| 146–223, 230–260 | §1.6 (Vortex) | rows 146–159 lexer/parser, 160–195 statements/functions, 200–223 built-ins, 248–260 Intl |
| 115–123, 261–263 | §1.6 (Vortex DOM bindings stub) | rows 115–123 are events, 261–263 are `window` / `document` / `console` |

## A.6 Open questions specific to this file

1. **`<plaintext>` (#70).** The tokeniser state is shipped, but the
   tree builder needs to drop straight into a `Text` node and never
   exit. This is a one-line behaviour but it lives in the tree
   builder, not the tokeniser, and is hard to test in isolation.
   Should it move to a separate PR?
2. **Page-level vs host-defined.** Some rows (e.g. `URL` #264) live
   in the WHATWG URL spec but are surfaced as JS built-ins. The
   "Engine-level sub-systems" section groups them, but a more
   rigorous answer would split "language surface" (ECMA-262) from
   "host surface" (everything else). Currently collapsed.
3. **Annex B and Annex C (#245, #246).** The two are different
   and Annex C is the more visible one. Should they get separate
   rows? Currently grouped.
4. **Custom Elements v1 (autonomous) vs v0.** The v0 form
   (`document.registerElement` returning a `document.register`
   wrapper) is gone from all engines since 2018; no row needed.
   But "v0 of custom elements" was a real Baseline subset, so worth
   flagging for synthesis.
5. **HTML parsing of unknown custom element tag names.** The
   parser's behaviour for `<my-component>` is to integrate it
   into the `HTMLElement` superclass with no special casing. Spiral
   needs to do the same — i.e. any unknown tag becomes an element
   with `localName = "my-component"`. Already shipped (DOM
   `Element` struct doesn't reject tags), so no row needed.
