# Chunk 2a — Competitive Matrix: HTML Elements, Attributes, Parser

**File:** `02-competitive-matrix-html.md`
**Date:** 2026-06-16
**Sources:** `01-feature-inventory-html-dom-js.md` (§A.1)
**Methodology:** `00-methodology.md`

Engine column values: `yes` = stable support, `partial` = partial/experimental,
`no` = not shipped, `behind-flag` = behind feature flag, `—` = not applicable.

Prevalence buckets: `ubiquitous` (≥90%), `widespread` (75–90%), `mixed` (50–75%),
`niche` (25–50%), `experimental` (<25% / flag-only), `legacy` (deprecated).

**Rows in this file:** 83
**Continued in:** `02-competitive-matrix-dom-css.md` (rows 84–459)
**Total across both files:** 459

---

### A.1.1 Document metadata root

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 1 | `<!DOCTYPE html>` parsing & mode (quirks / limited-quirks / no-quirks) | all | partial (DOCTYPE token handled; limited-quirks classifier not yet; GAP 1.1 `[~]`) | ubiquitous | P2 | M | yes | yes | yes | partial | yes | partial |
| 2 | `<html>` root element (implicit and explicit) | all | shipped (via `parse_html`; GAP 1.1) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 3 | `<head>` element and its required children | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 4 | `<title>` element (with Rawtext content model) | all | shipped (Rawtext mode in tokeniser; GAP Delta 2) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 5 | `<base>` element (`href`, `target`) | all | not-started (parser accepts; no DOM attach) | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 6 | `<link>` element (`rel`, `href`, `type`, `media`, `sizes`, `as`, `crossorigin`, `integrity`, `imagesrcset`, `imagesizes`, `blocking`, `fetchpriority`, `disabled`, `color`) | all | not-started (parser accepts; rel-handler not built) | ubiquitous | P4 | L | yes | yes | yes | yes | yes | yes |
| 7 | `<meta>` element (`charset`, `name`, `http-equiv`, `content`, `media`, `property`, `itemprop`) | all | not-started (parser accepts; `http-equiv` is M5+) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 8 | `<style>` element (with Rawtext content model) | all | partial (Rawtext tokeniser yes; cascade/integration with `spiral-fmt` is the shim path — see ADR 0001) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 9 | `<script>` element (with ScriptData content model; `type`, `src`, `async`, `defer`, `module`, `integrity`, `crossorigin`, `referrerpolicy`, `blocking`, `fetchpriority`, `nomodule`, `attributionsrc`, `noModule`) | all | partial (ScriptData yes; no execute path; GAP 1.6) | ubiquitous | P2 / P3 | L | yes | yes | yes | yes | yes | yes |
| 10 | `<noscript>` element | all | not-started | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.1.2 Sectioning root and body

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 11 | `<body>` element | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 12 | `<article>`, `<section>`, `<nav>`, `<aside>`, `<header>`, `<footer>`, `<main>` (sectioning content + landmarks) | all | shipped (parsed; no a11y/landmark tree yet — chunk 8) | ubiquitous | P2 / P3 | M | yes | yes | yes | yes | yes | yes |
| 13 | `<address>` | all | not-started | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 14 | `<h1>` … `<h6>` heading elements (with outline algorithm) | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.1.3 Grouping content

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 15 | `<p>` | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 16 | `<hr>` | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 17 | `<pre>` | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 18 | `<blockquote>` (`cite`) | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 19 | `<ol>`, `<ul>`, `<li>` (incl. `start`, `reversed`, `type` on `<ol>`; `value` on `<li>`) | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 20 | `<dl>`, `<dt>`, `<dd>`, `<dfn>` (description list) | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 21 | `<figure>`, `<figcaption>` | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 22 | `<div>` (generic flow container) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.1.4 Text-level semantics

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 23 | `<a>` (`href`, `target`, `rel`, `download`, `ping`, `type`, `referrerpolicy`, `hreflang`) | all | not-started (parsed) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 24 | `<em>`, `<strong>`, `<small>`, `<s>`, `<cite>`, `<q>` (`cite` on `<q>`) | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 25 | `<dfn>`, `<abbr>`, `<ruby>`, `<rt>`, `<rp>`, `<rb>`, `<rtc>`, `<data>` (`value`), `<time>` (`datetime`) | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 26 | `<code>`, `<var>`, `<samp>`, `<kbd>`, `<sub>`, `<sup>`, `<i>`, `<b>`, `<u>`, `<mark>`, `<bdi>`, `<bdo>` (`dir`) | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 27 | `<span>` (generic phrasing container) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 28 | `<br>`, `<wbr>` | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 29 | `<ins>`, `<del>` (`cite`, `datetime`) | all | shipped (parsed) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 30 | Text content (character data, entity references — `&amp;` `&lt;` `&gt;` `&quot;` `&apos;` and the named set) | all | shipped (Delta 3) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.1.5 Embedded content

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 31 | `<img>` (`src`, `alt`, `width`, `height`, `srcset`, `sizes`, `loading`, `fetchpriority`, `decoding`, `crossorigin`, `referrerpolicy`, `ismap`, `usemap`) | all | not-started (parsed) | ubiquitous | P4 | L | yes | yes | yes | yes | yes | yes |
| 32 | `<picture>`, `<source>` (`srcset`, `sizes`, `type`, `media`, `src`, `width`, `height`) | all | not-started | widespread | P4 | M | yes | yes | yes | yes | yes | yes |
| 33 | `<iframe>` (`src`, `srcdoc`, `name`, `sandbox`, `allow`, `allowfullscreen`, `allowpaymentrequest`, `credentialless`, `loading`, `referrerpolicy`, `csp`) | all | not-started (parsed) | ubiquitous | P4 | XL | yes | yes | yes | yes | yes | yes |
| 34 | `<embed>`, `<object>` (`data`, `type`, `name`, `width`, `height`, `typemustmatch`), `<param>` | all | not-started | mixed | P4 | M | yes | yes | yes | yes | yes | yes |
| 35 | `<video>`, `<audio>` (chunk 5 surface — listed here for parser coverage) | all | not-started (parser accepts) | ubiquitous | P4 / P5 | XL | yes | yes | yes | yes | yes | yes |
| 36 | `<track>` (`kind`, `src`, `srclang`, `label`, `default`) | all | not-started | mixed | P4 | M | yes | yes | yes | yes | yes | yes |
| 37 | `<map>`, `<area>` (`shape`, `coords`, `href`, `alt`, `download`, `ping`, `rel`, `target`) | all | not-started | mixed | P3 | M | yes | yes | yes | yes | yes | yes |
| 38 | `<svg>` and SVG namespace (foreign content parser mode) | all | not-started (per tokeniser docs, foreign content deferred to M5+) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 39 | `<math>` and MathML namespace (foreign content parser mode) | all | not-started | mixed | P3 | L | yes | yes | yes | yes | yes | yes |

### A.1.6 Tabular data

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 40 | `<table>`, `<caption>`, `<colgroup>`, `<col>` (`span`), `<thead>`, `<tbody>`, `<tfoot>`, `<tr>`, `<th>`, `<td>` (`colspan`, `rowspan`, `headers`, `scope`, `abbr`) | all | partial (parsed; tree builder has no `InTable` / `InCell` modes yet — see `html/tree.rs` \"full insertion-mode machine (tables, select, template, foreign content) lands in M5+\") | ubiquitous | P2 / P3 | L | yes | yes | yes | yes | yes | yes |

### A.1.7 Forms

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 41 | `<form>` (`action`, `method`, `enctype`, `target`, `autocomplete`, `name`, `novalidate`, `rel`, `accept-charset`) | all | not-started (parsed) | ubiquitous | P4 | L | yes | yes | yes | yes | yes | yes |
| 42 | `<label>` (`for`) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes | yes |
| 43 | `<input>` element — type matrix (`text`, `password`, `checkbox`, `radio`, `submit`, `reset`, `button`, `file`, `hidden`, `image`, `email`, `url`, `tel`, `search`, `number`, `range`, `color`, `date`, `time`, `datetime-local`, `month`, `week`) | all | not-started (parsed) | ubiquitous | P4 | XL | yes | yes | yes | yes | yes | yes |
| 44 | `<button>` (`type`, `form`, `formaction`, `formmethod`, `formnovalidate`, `formtarget`, `formenctype`, `popovertarget`, `popovertargetaction`) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes | yes |
| 45 | `<select>`, `<datalist>`, `<optgroup>`, `<option>` (`selected`, `value`, `label`, `disabled`) | all | not-started | ubiquitous | P4 | L | yes | yes | yes | yes | yes | yes |
| 46 | `<textarea>` (`rows`, `cols`, `wrap`, `placeholder`, `readonly`, `disabled`, `required`, `maxlength`, `minlength`, `autocomplete`, `dirname`, `form`, `inputmode`, `enterkeyhint`, `autocapitalize`, `spellcheck`) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes | yes |
| 47 | `<output>` (`for`, `form`, `name`) | all | not-started | widespread | P4 | S | yes | yes | yes | yes | yes | yes |
| 48 | `<progress>` (`value`, `max`) | all | not-started | widespread | P3 | S | yes | yes | yes | yes | yes | yes |
| 49 | `<meter>` (`value`, `min`, `max`, `low`, `high`, `optimum`) | all | not-started | widespread | P3 | S | yes | yes | yes | yes | yes | yes |
| 50 | `<fieldset>`, `<legend>` (`disabled`, `form`, `name`) | all | not-started | widespread | P4 | M | yes | yes | yes | yes | yes | yes |

### A.1.8 Interactive elements

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 51 | `details` / `summary` (with `open` attribute) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 52 | `<dialog>` (`open`, `showModal()` algorithm, top-layer, `::backdrop`) | all | not-started | widespread | P3 | L | yes | yes | yes | partial | yes | yes |
| 53 | `<menu>`, `<menuitem>` (context menu) | all | not-started | niche | P3 | L | no | yes | no | no | no | no |

### A.1.9 Web Components

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 54 | `customElements` registry (`define`, `get`, `upgrade`, `whenDefined`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | partial | no | yes |
| 55 | Autonomous custom elements (lifecycle: `connectedCallback`, `disconnectedCallback`, `attributeChangedCallback`, `adoptedCallback`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 56 | Customised built-in elements (`is="…"`) | all | not-started | mixed | P3 | M | yes | no | yes | yes | yes | yes |
| 57 | Shadow DOM (`attachShadow({mode: 'open' / 'closed'})`, `shadowRoot`) | all | not-started | ubiquitous | P3 | XL | yes | yes | yes | partial | no | yes |
| 58 | Slot / `::slotted()` / `<slot name="…">` (light-DOM projection) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 59 | Declarative Shadow DOM (`<template shadowrootmode="open / closed">`, `shadowrootclonable`, `shadowrootserializable`) | all | not-started | mixed | P3 | L | yes | yes | yes | no | no | yes |
| 60 | Event retargeting (composedPath, `composed` flag, scoped retargeting through shadow roots) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 61 | `:host`, `:host()`, `:host-context()` CSS pseudo-classes | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 62 | Constructable Stylesheets (`new CSSStyleSheet()`, `adoptedStyleSheets`, `replaceSync`, `replace`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | no | yes |

### A.1.10 Template, slot, parser features

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 63 | `<template>` element and its content document fragment | all | partial (tokenised; GAP 1.1 — "Returns the element itself (no document fragment). Lossy.") | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 64 | Adoption agency algorithm (for misnested formatting) | all | not-started (per GAP 1.1) | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 65 | Foster parenting (out-of-table / in-table element placement) | all | not-started | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 66 | Encoding sniffing (BOM, `<meta charset>`, transport-layer charset, content sniff) | all | not-started (per GAP 1.1 — "Assumes UTF-8 only") | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 67 | Numeric character references (`&#NN;` and `&#xHH;` with the spec-mandated Windows-1252 fixup range 0x80..=0x9F) | all | shipped (Delta 3) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 68 | Format-extracted character references (the named set: `&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`) | all | shipped (Delta 3) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 69 | Rawtext / ScriptData / ScriptDataEscaped / RCData modes (per-element content models for `<script>`, `<style>`, `<title>`, `<textarea>`, `<noscript>`, `<noembed>`, `<noframes>`, `<xmp>`, `<iframe>`, etc.) | all | shipped (Delta 2) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 70 | PLAINTEXT state (everything after `<plaintext>` is text) | all | shipped (tokeniser) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.1.11 Obsolete / parser-compatible elements

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 71 | HTML5 obsolete-but-parsed elements (`<acronym>`, `<applet>`, `<basefont>`, `<bgsound>`, `<big>`, `<blink>`, `<center>`, `<command>`, `<content>`, `<dir>`, `<element>`, `<font>`, `<frame>`, `<frameset>`, `<hgroup>`, `<image>`, `<keygen>`, `<listing>`, `<marquee>`, `<menuitem>`, `<multicol>`, `<nextid>`, `<nobr>`, `<noembed>`, `<noframes>`, `<plaintext>`, `<shadow>`, `<spacer>`, `<strike>`, `<tt>`, `<xmp>`) | all | not-started (parser tokenises some via Rawtext paths; no DOM attach; `<plaintext>` yes per #70) | experimental | P2 | S | yes | yes | yes | yes | yes | yes |

### A.1.12 HTML attributes

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 72 | Global attributes (`id`, `class`, `style`, `title`, `lang`, `dir`, `hidden`, `tabindex`, `accesskey`, `draggable`, `spellcheck`, `autocapitalize`, `autocorrect`, `autofocus`, `contenteditable`, `enterkeyhint`, `inputmode`, `is`, `itemid`, `itemprop`, `itemref`, `itemscope`, `itemtype`, `nonce`, `part`, `role`, `slot`, `translate`, `inert`, `popover`, `writing-suggestions`, `virtualkeyboardpolicy`, `exportparts`, `anchor`) | all | not-started (parsed as attributes; behaviour unmodelled; `slot` / `is` / `part` tied to Web Components work) | ubiquitous | P2 / P3 | L | behind-flag | yes | yes | yes | yes | yes |
| 73 | Microdata (`itemscope`, `itemtype`, `itemprop`, `itemref`, `itemid`) | all | not-started | mixed | P3 | M | yes | yes | yes | yes | yes | yes |
| 74 | ARIA reflection (the `role` and `aria-*` attributes) | all | not-started (parser passes them through; a11y tree is chunk 8) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 75 | `data-*` custom data attributes (with `dataset` IDL reflection) | all | not-started (parsed; `dataset` getter is M5+) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 76 | Popover (`popover` attribute, `popovertarget`, top-layer, light dismiss) | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 77 | Anchor positioning (`anchor` attribute, `position-anchor` CSS, `position-try` / `position-try-options`, `position-visibility`) | all | not-started | niche | P3 | L | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag |

### A.1.13 HTML parser internals

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 78 | Insertion-mode machine (full 33-mode state machine incl. tables, select, in-cell, template, foreign content) | all | partial (8 modes; GAP 1.1 — "the full insertion-mode machine (tables, select, template, foreign content) lands in M5+") | ubiquitous | P2 / P3 | XL | yes | yes | yes | yes | yes | yes |
| 79 | Active formatting elements (the `afe` list used by the adoption agency) | all | not-started (dependent on #64) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 80 | Stack of open elements + common ancestor / appropriate end-tag lookups | all | shipped (per `spiral-fmt/src/html/tree.rs` InsertionMode) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 81 | Quirk mode classifier (per `Content-Type` sniffing + doctype + comments) | all | not-started (GAP 1.1) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 82 | `<template>` content document-fragment construction | all | not-started (GAP 1.3) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 83 | Fragment parsing algorithm (`DOMParser.parseFragment`, `document.implementation.createHTMLDocument`) | all | not-started | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
