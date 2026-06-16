# Chunk 2b — Competitive Matrix: DOM, JavaScript, CSS

**File:** `02-competitive-matrix-dom-css.md`
**Date:** 2026-06-16
**Sources:** `01-feature-inventory-html-dom-js.md` (§A.2–A.3), `01-feature-inventory-css.md` (§B)
**Methodology:** `00-methodology.md`

Engine column values: `yes` = stable support, `partial` = partial/experimental,
`no` = not shipped, `behind-flag` = behind feature flag, `—` = not applicable.

Prevalence buckets: `ubiquitous` (≥90%), `widespread` (75–90%), `mixed` (50–75%),
`niche` (25–50%), `experimental` (<25% / flag-only), `legacy` (deprecated).

**Rows in this file:** 376
**Preceded by:** `02-competitive-matrix-html.md` (rows 1–83)
**Total across both files:** 459

---

## Part 1: DOM and JavaScript (§A.2–A.3)


### A.2.1 Core tree interfaces

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 84 | `Node` (12 `nodeType` constants: `ELEMENT_NODE`, `ATTRIBUTE_NODE`, `TEXT_NODE`, `CDATA_SECTION_NODE`, `COMMENT_NODE`, `DOCUMENT_FRAGMENT_NODE`, `PROCESSING_INSTRUCTION_NODE`, `DOCUMENT_TYPE_NODE`, `DOCUMENT_NODE`, etc.) | all | partial (`Node` enum has 4 variants: Element/Text/Comment/Document; GAP 1.3 — "DOCTYPE node variant [ ]", "Document fragment [ ]") | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 85 | `Element` (id, className, tagName, attributes NamedNodeMap, namespace, prefix, localName, classList) | all | partial (`Element` struct exists with attrs `Vec<(String, String)>`; no `classList`/`getAttribute`/`setAttribute` IDL surface; GAP 1.3) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 86 | `Document` (URL, documentURI, compatMode, characterSet, contentType, doctype, documentElement, body, head, title, forms, images, links, scripts, anchors, readyState, currentScript, implementation, location, defaultView, activeElement, hasFocus) | all | not-started (DOM has `Document` variant; IDL surface in HTML document section not yet built; GAP 1.3) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 87 | `DocumentType` (`name`, `publicId`, `systemId`) | all | not-started (GAP 1.3) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 88 | `DocumentFragment` (light-DOM container; `children`, `querySelector`, `append`, `prepend`, `replaceChildren`) | all | not-started (GAP 1.3) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 89 | `Text` (`data`, `length`, `wholeText`, `replaceData`, `appendData`, `splitText`) | all | partial (`Text` struct exists; no methods) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 90 | `Comment` (`data`, `length`) | all | partial (`Comment` struct exists) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 91 | `CDATASection` (text-only XML escape) | all | not-started | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 92 | `ProcessingInstruction` (`target`, `data`) | all | not-started | mixed | P2 | S | yes | yes | yes | yes | yes | yes |
| 93 | `Attr` (`name`, `value`, `namespaceURI`, `prefix`, `localName`, `ownerElement`) | all | not-started (DOM stores `Vec<(String, String)>` pairs; no `Attr` interface) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 94 | `DOMTokenList` (`classList`, `relList`, `sandbox`, `linkSizes`, etc.; `add`/`remove`/`toggle`/`contains`/`replace`/`supports`/`value`/`length`/`entries`) | all | not-started | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 95 | `NamedNodeMap` (attribute collection; `length`, `item`, `getNamedItem`, `setNamedItem`, `removeNamedItem`) | all | not-started | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 96 | `HTMLCollection` (live ordered collection; `length`, `item`, `namedItem`) | all | not-started | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 97 | `NodeList` (static or live; `length`, `item`, `entries`/`keys`/`values`/`forEach`) | all | not-started | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.2.2 Tree mutation operations

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 98 | `appendChild`, `removeChild`, `replaceChild`, `insertBefore` | all | not-started (GAP 1.3 — "open-codes insert-before via remove+re-append. DOM API itself lacks it") | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 99 | `Node.append`, `Node.prepend`, `Node.replaceChildren`, `Node.remove` (the `ParentNode` mixin) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 100 | `Element.before`, `Element.after`, `Element.replaceWith`, `Element.insertAdjacentElement` | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 101 | `Element.innerHTML`, `Element.outerHTML`, `Element.insertAdjacentHTML` | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 102 | `Document.parseHTMLUnsafe`, `Document.parseHTML` (proposed; safe variant stripping XSS vectors) | all | not-started | niche | P3 | M | yes | partial | partial | partial | partial | partial |
| 103 | `Element.cloneNode`, `Document.importNode`, `Node.cloneNode` | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 104 | `Document.adoptNode`, `Node.isConnected`, `Node.getRootNode` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 105 | `TreeWalker`, `NodeIterator` (depth-first / breadth-first traversals with `whatToShow`/`NodeFilter`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |

### A.2.3 Selectors / element lookup

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 106 | `Element.getElementById`, `Document.getElementById` | all | not-started (GAP 1.3) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 107 | `Element.getElementsByClassName`, `Element.getElementsByTagName`, `Element.getElementsByTagNameNS` | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 108 | `Element.querySelector`, `Element.querySelectorAll` (with selectors parsed by `spiral-fmt::css::selector`) | all | not-started (selector parser exists; matcher not) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 109 | `Document.querySelector`, `Document.querySelectorAll` | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 110 | `:scope` (explicit scope reference inside `:scope :is()` / `Element.query(':scope > div')`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 111 | `Element.closest` (selector match against self and ancestors) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 112 | `Element.matches` (does this element match the selector) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 113 | `Document.createElement`, `Document.createElementNS`, `Document.createTextNode`, `Document.createComment`, `Document.createDocumentFragment` | all | not-started (constructors for the new `Dom` exist; `Document` IDL not built) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 114 | `Node.contains`, `Node.compareDocumentPosition`, `Node.isEqualNode`, `Node.isSameNode` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |

### A.2.4 Events (DOM event model)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 115 | `EventTarget` (`addEventListener`, `removeEventListener`, `dispatchEvent`) | all | not-started (GAP 1.6 — DOM bindings stub) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 116 | `Event` (`type`, `target`, `currentTarget`, `eventPhase`, `bubbles`, `cancelable`, `composed`, `defaultPrevented`, `preventDefault`, `stopPropagation`, `stopImmediatePropagation`, `timeStamp`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 117 | `CustomEvent` (`detail`, `init`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 118 | `Event` constructors (`new MouseEvent(...)`, `new KeyboardEvent(...)`, etc.) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 119 | `UIEvent`, `MouseEvent`, `KeyboardEvent`, `FocusEvent`, `InputEvent`, `PointerEvent`, `WheelEvent`, `CompositionEvent`, `BeforeUnloadEvent` (UI sub-set) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 120 | Event capturing / bubbling model + `stopPropagation` ordering | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 121 | `Event.composed` + retargeting across shadow boundaries (see A.1.9 #60) | all | not-started (covered as part of #60) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 122 | `AbortController` / `AbortSignal` (used to cancel fetch, timers, etc.) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 123 | `addEventListener` options (`capture`, `once`, `passive`, `signal`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |

### A.2.5 Mutation observers and ranges

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 124 | `MutationObserver` (`observe` with `attributes`, `childList`, `subtree`, `characterData`, `attributeOldValue`, `characterDataOldValue`, `attributeFilter`), `MutationRecord` | all | not-started (GAP 1.3) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 125 | `Range` (`setStart`, `setEnd`, `setStartBefore`, `setEndAfter`, `selectNode`, `selectNodeContents`, `collapse`, `cloneContents`, `extractContents`, `insertNode`, `deleteContents`, `surroundContents`, `comparePoint`, `intersectsNode`, `isPointInRange`, `getBoundingClientRect`, `getClientRects`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 126 | `Selection` (`getSelection`, `rangeCount`, `anchorNode`, `focusNode`, `addRange`, `removeRange`, `removeAllRanges`, `collapse`, `extend`, `setBaseAndExtent`, `selectAllChildren`, `deleteFromDocument`, `containsNode`, `modify`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 127 | Static `Range` methods (`Range.getBoundingClientRect`, `Range.getClientRects`, `Range.intersectsNode`, `Range.pointFromNode`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |

### A.2.6 Geometric / layout-reading interfaces

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 128 | `Element.getBoundingClientRect` (`DOMRect` with `x`, `y`, `width`, `height`, `top`, `right`, `bottom`, `left`) | all | not-started (depends on Gyre layout) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 129 | `Element.getClientRects` (text line boxes) | all | not-started (depends on inline / text layout) | ubiquitous | P3 / P5 | L | yes | yes | yes | yes | yes | yes |
| 130 | `Element.scrollIntoView`, `Element.scrollTo`, `Element.scroll` (and `Element.scrollTop`/`Left`/`Width`/`Height`) | all | not-started (depends on layout + paint) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 131 | `IntersectionObserver` (`root`, `rootMargin`, `thresholds`, `observe`, `unobserve`, `disconnect`, entries) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 132 | `ResizeObserver` (`observe`, `unobserve`, `disconnect`, `contentBoxSize`, `borderBoxSize`, `devicePixelContentBoxSize`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 133 | `DOMRect`, `DOMRectReadOnly`, `DOMPoint`, `DOMMatrix` (geometry types) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |

### A.2.7 CSSOM — StyleSheet interfaces

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 134 | `StyleSheet` abstract (inherited by `CSSStyleSheet`, `CSSGroupingRule`, `CSSConditionRule`) | all | not-started (parser produces `Stylesheet`; no `CSSStyleSheet` IDL wrapper; GAP 1.2) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 135 | `CSSStyleSheet` (`insertRule`, `deleteRule`, `cssRules`, `ownerNode`, `title`, `href`, `type`, `media`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 136 | `CSSRule`, `CSSStyleRule`, `CSSImportRule`, `CSSMediaRule`, `CSSKeyframesRule`, `CSSKeyframeRule`, `CSSSupportsRule`, `CSSContainerRule`, `CSSLayerBlockRule`, `CSSLayerStatementRule`, `CSSNamespaceRule`, `CSSFontFaceRule`, `CSSCounterStyleRule` (full hierarchy) | all | not-started (parser produces `Rule` enum; no `CSSRule` IDL wrapper) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 137 | `CSSStyleDeclaration` (`cssText`, `length`, `item`, `getPropertyValue`, `setProperty`, `removeProperty`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 138 | `Element.style` (inline `CSSStyleDeclaration`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 139 | `getComputedStyle` (`window.getComputedStyle(element, pseudoElt?)` → live `CSSStyleDeclaration`) | all | not-started (depends on cascade + computed-value resolution) | ubiquitous | P3 / P4 | L | yes | yes | yes | yes | yes | yes |
| 140 | `StyleMedia` (`window.styleMedia`, `matchMedium(mediaQuery)`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 141 | `MediaQueryList` (`window.matchMedia`, `mql.matches`, `mql.media`, `addEventListener('change', …)`, `addListener`, `removeListener`) | all | not-started (parser tokenises `@media`; match-eval not built) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 142 | Constructable Stylesheets (`new CSSStyleSheet()`, `adoptedStyleSheets`) (see A.1.9 #62) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |

### A.2.8 ElementInternals and DOM sub-systems

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 143 | `ElementInternals` (form-associated custom elements, `form`, `validity`, `validationMessage`, `setFormValue`, `setValidity`, `labels`, `willValidate`, `checkValidity`, `reportValidity`) | all | not-started | widespread | P4 | L | yes | yes | yes | yes | yes | yes |
| 144 | `HTMLFormElement.elements`, `HTMLFormElement.submit`, `HTMLFormElement.reset`, `HTMLFormElement.checkValidity`, `HTMLFormElement.action`, `HTMLFormElement.method` | all | not-started (chunk 4 territory) | ubiquitous | P4 | L | yes | yes | yes | yes | yes | yes |
| 145 | `ValidityState` (`valueMissing`, `typeMismatch`, `patternMismatch`, `tooLong`, `tooShort`, `rangeUnderflow`, `rangeOverflow`, `stepMismatch`, `badInput`, `customError`, `valid`) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes | yes |

### A.3.1 Lexical grammar

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 146 | Source text (UTF-16, with optional BOM, line terminators) | all | shipped (Vortex lexer `Cursor`) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 147 | Input element division: tokens (identifier, keyword, punctuator, numeric, string, template, regular-expression) | all | shipped (Vortex `lexer` module per GAP 1.6) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 148 | Identifier resolution including Unicode escape sequences (`\u00E9`), `ZWNJ`/`ZWJ`, and the full `ID_Start` / `ID_Continue` Unicode property | all | partial (lex tokenises; full Unicode tables not exhaustive) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 149 | Reserved words (current and contextual: `await`, `yield`, `let`, `static`, `from`, `of`, `as`, `async`, etc.) | all | shipped (Vortex parser) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 150 | Numeric literals (decimal, binary `0b`, octal `0o`, hex `0x`, BigInt `0n`; underscore separators) | all | shipped (per Vortex lexer AST coverage) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 151 | String literals (single/double quoted, with all escape sequences incl. line continuation; line terminator restrictions) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 152 | Template literals (no-substitution template, `${expr}` substitution, tag functions, raw form via `String.raw`) | all | partial (parser yes; tag functions yes) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 153 | Regular expression literals (`/pattern/flags`, with Unicode `u`, `v`, dotAll `s`, sticky `y`, named groups, lookbehind, Unicode property escapes) | all | partial (lex tokenises; `RegExp` built-in coverage partial) | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 154 | Automatic Semicolon Insertion (ASI) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 155 | Hashbang grammar (`#!/usr/bin/env node` at the top of a script) | all | not-started (recent ES2024 addition) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.3.2 Expressions

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 156 | Primary expressions (`this`, `null`, `true`/`false`, `undefined`, literals, `[…]`, `{…}`, `(expr)`, function expression, class expression, generator, async function) | all | shipped (Vortex AST) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 157 | Property access (member and computed) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 158 | `new` operator with argument list | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 159 | Function calls (direct, method, `eval`, indirect `eval`, optional chaining `?.`, tail calls) | all | partial (basic yes; tail calls partial; proper tail calls in spec semantics only ES2017) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 160 | Left-hand-side expressions incl. `super`, `import.meta`, `import()` (dynamic import), `new.target`, `new.super` | all | partial (Vortex AST has ES2015+ types) | ubiquitous | P2 / P3 | M | yes | yes | yes | yes | yes | yes |
| 161 | Update expressions (`++`, `--`, prefix and postfix) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 162 | Unary operators (`!`, `~`, `+`, `-`, `typeof`, `void`, `delete`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 163 | Binary operators (arithmetic, bitwise, comparison, equality, `in`, `instanceof`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 164 | Logical operators (`&&`, `||`, `??`, `&&=`, `||=`, `??=`) | all | shipped (per ES2021 logical assignment) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 165 | Conditional expression (`a ? b : c`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 166 | Assignment expressions (simple, destructuring) | all | shipped (AST yes) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 167 | Sequence / comma operator | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 168 | Spread element in call and array literal, rest in parameter list and destructuring | all | shipped (ES2015+) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 169 | Arrow functions (`x => x * 2`, including expression body and block body) | all | shipped (per Vortex partial) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 170 | Async/await expressions (including `await` at module top level) | all | partial (Vortex interpreter; needs bytecode VM for real perf) | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 171 | Destructuring patterns (array, object, nested, with defaults, with rest, with rename) | all | shipped (AST yes) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 172 | Class expressions (with `extends`, `static`, computed method names, `#privateName`, `static #privateName`, `static {}` blocks) | all | partial (Vortex partial) | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 173 | Generator function expressions (`function*`, `yield`, `yield*`) | all | partial (Vortex partial) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 174 | Async generator function expressions (`async function*`, `for await…of`) | all | partial (Vortex partial) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 175 | BigInt literals and operations (`123n`, `BigInt('456')`, `BigInt.asIntN`, `BigInt.asUintN`) | all | partial (Vortex has `BigInt` type per `builtins/mod.rs`) | ubiquitous | P2 / P5 | M | yes | yes | yes | yes | yes | yes |

### A.3.3 Statements

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 176 | Expression / declaration statements (incl. `let`, `const`, `var`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 177 | Block statement (`{ … }`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 178 | `if` / `else` | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 179 | `while` / `do-while` | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 180 | `for` (three-clause C-style) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 181 | `for-in` (enumerable own and inherited string-keyed properties) | all | partial | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 182 | `for-of` (iterator protocol; works with any object exposing `@@iterator`) | all | partial | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 183 | `switch` | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 184 | `return`, `break`, `continue`, `throw`, `try`/`catch`/`finally` | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 185 | `with` statement (Annex B) | all | not-started (deprecated in strict mode) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 186 | Labelled statement, `break label`, `continue label` | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 187 | Empty statement (`;`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 188 | `debugger` statement (no-op) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.3.4 Functions, classes, modules

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 189 | Function declarations and expressions (incl. default parameters, rest, destructuring) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 190 | Lexical environment / scope chain + closures | all | partial (Vortex interpreter has closures) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 191 | Strict mode (the "use strict" directive and module mode) | all | partial (Vortex partial) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 192 | Class declarations (with constructor, methods, getters/setters, `static`, computed, `#privateBrand`, `static {}`) | all | partial (Vortex partial) | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 193 | Class fields (public and `#private`) | all | partial | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 194 | Private methods and accessors (`#privateMethod`, `#privateGetter`/`#privateSetter`) | all | partial | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 195 | Module syntax (`import` / `export`, `import.meta.url`, `import('...')` dynamic, top-level `await`) | all | partial (Vortex partial) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 196 | `Symbol` (incl. `Symbol.iterator`, `Symbol.asyncIterator`, `Symbol.hasInstance`, `Symbol.toPrimitive`, `Symbol.toStringTag`, `Symbol.species`, `Symbol.for`, `Symbol.keyFor`) | all | partial (Vortex has `Symbol`) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 197 | `Proxy` (handler traps: `get`, `set`, `has`, `deleteProperty`, `apply`, `construct`, `ownKeys`, `getOwnPropertyDescriptor`, `defineProperty`, `getPrototypeOf`, `setPrototypeOf`, `preventExtensions`, `isExtensible`, `revocable`) | all | not-started | ubiquitous | P2 / P5 | XL | yes | yes | yes | yes | yes | yes |
| 198 | `Reflect` (the static reflection namespace: `Reflect.get`, `set`, `has`, `deleteProperty`, `apply`, `construct`, `ownKeys`, `getOwnPropertyDescriptor`, `defineProperty`, `getPrototypeOf`, `setPrototypeOf`, `preventExtensions`, `isExtensible`) | all | not-started | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 199 | Iterator protocol (`@@iterator`, `next()` returning `{ value, done }`) and the `function*` async variant | all | partial (Vortex partial) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 200 | `await using` / explicit resource management (`Symbol.dispose`, `Symbol.asyncDispose`) | all | not-started (ES2025) | niche | P3 | M | yes | partial | partial | partial | partial | partial |

### A.3.5 Built-in objects — value / utility

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 201 | `Object` (assign, create, defineProperties, defineProperty, entries, freeze, fromEntries, getOwnPropertyDescriptor, getOwnPropertyDescriptors, getOwnPropertyNames, getOwnPropertySymbols, getPrototypeOf, is, isExtensible, isFrozen, isSealed, keys, preventExtensions, seal, setPrototypeOf, values) | all | partial (Vortex has Object) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 202 | `Array` (from, of, isArray, concat, copyWithin, entries, every, fill, filter, find, findIndex, findLast, findLastIndex, flat, flatMap, forEach, includes, indexOf, join, keys, lastIndexOf, map, pop, push, reduce, reduceRight, reverse, shift, slice, some, sort, splice, toLocaleString, toReversed, toSorted, toSpliced, unshift, values, with) | all | partial (Vortex has Array; copes, flatMap, sort, slice, splice, concat, etc.) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 203 | `ArrayBuffer` and `SharedArrayBuffer` (the latter requires cross-origin isolation headers) | all | partial (Vortex heap is arena, not yet exposed as `ArrayBuffer`) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 204 | `DataView` (typed array view with explicit endian and byte offset) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 205 | Typed arrays (Int8Array, Uint8Array, Uint8ClampedArray, Int16Array, Uint16Array, Int32Array, Uint32Array, Float32Array, Float64Array, BigInt64Array, BigUint64Array) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 206 | `Boolean`, `Number` (incl. `EPSILON`, `MAX_SAFE_INTEGER`, `MIN_SAFE_INTEGER`, `parseInt`, `parseFloat`, `isFinite`, `isInteger`, `isNaN`, `isSafeInteger`) | all | shipped (per Vortex builtins list) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 207 | `String` (fromCharCode, fromCodePoint, raw, includes, startsWith, endsWith, repeat, padStart, padEnd, trim, trimStart, trimEnd, slice, substring, substr, indexOf, lastIndexOf, toUpperCase, toLowerCase, localeCompare, normalize, match, matchAll, replace, replaceAll, search, split, at, isWellFormed, toWellFormed) | all | partial (Vortex has String per `builtins/mod.rs`) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 208 | `RegExp` (named groups, lookbehind, unicode property escapes, `flags`, `sticky`, `unicodeSets`, `hasIndices`) | all | partial | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 209 | `Date` (`Date.parse`, `Date.UTC`, `Date.now`, `getTime`, `toISOString`, `toJSON`, `toLocaleString`, etc.) | all | partial | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 210 | `Temporal` (the modern replacement for `Date`; `Temporal.PlainDate`, `PlainTime`, `PlainDateTime`, `ZonedDateTime`, `Instant`, `Duration`, `Calendar`, `TimeZone`) | all | not-started | niche | P5 | XL | yes | yes | yes | yes | yes | yes |
| 211 | `Math` (constants and 40+ methods) | all | shipped (Vortex builtins `math.rs`) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 212 | `JSON` (parse, stringify, with replacer / reviver / `toJSON`) | all | shipped (Vortex builtins per `builtins/mod.rs`) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 213 | `Map` and `Set` (with insertion-order iteration) | all | partial (Vortex has Map/Set per `builtins/mod.rs`) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 214 | `WeakMap` and `WeakSet` (entries weakly held; only object keys) | all | partial (Vortex has WeakMap/WeakSet per `builtins/mod.rs`) | ubiquitous | P2 / P5 | M | yes | yes | yes | yes | yes | yes |
| 215 | `WeakRef` and `FinalizationRegistry` | all | not-started (Vortex `gc` is the engine; not yet exposed to JS) | ubiquitous | P2 / P5 | L | yes | yes | yes | yes | yes | yes |
| 216 | `Error` (and the sub-classes: `EvalError`, `RangeError`, `ReferenceError`, `SyntaxError`, `TypeError`, `URIError`, `AggregateError`) | all | partial (Vortex has `Error`, `TypeError`) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 217 | `Promise` (with `all`, `allSettled`, `any`, `race`, `resolve`, `reject`, `withResolvers`, `try`) | all | partial (Vortex EventLoop; per `builtins/mod.rs` Promise is in scope) | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 218 | `Atomics` and `SharedArrayBuffer` shared-memory atomic operations (`wait`, `notify`, `load`, `store`, `add`, `sub`, `and`, `or`, `xor`, `exchange`, `compareExchange`, `isLockFree`) | all | not-started | ubiquitous | P3 / P5 | XL | yes | yes | yes | yes | yes | yes |
| 219 | `structuredClone` (deep-clone of any cloneable value) | all | not-started | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 220 | `globalThis` (universal reference to the global) | all | not-started | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 221 | `queueMicrotask` (schedule a microtask) | all | shipped (Vortex EventLoop has `queue_microtask`) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 222 | `eval` (direct and indirect) | all | partial (Vortex supports direct eval) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 223 | `setTimeout` / `setInterval` / `clearTimeout` / `clearInterval` (host-defined timers) | all | shipped (Vortex EventLoop has both) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |

### A.3.6 Control flow, iteration, annexes

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 224 | Iteration protocols: `for-of` (sync) + `for-await-of` (async) + `@@iterator` + `@@asyncIterator` | all | partial (Vortex partial) | ubiquitous | P2 | M | yes | yes | yes | yes | yes | yes |
| 225 | `Array.from` (with mapFn and `thisArg`), `Array.of` | all | shipped (Vortex builtins `array.rs`) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 226 | `Array.prototype.flat`, `Array.prototype.flatMap` | all | shipped (Vortex partial) | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 227 | `Object.values`, `Object.entries`, `Object.fromEntries` | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 228 | `String.prototype.replaceAll` | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 229 | Logical assignment (`&&=`, `||=`, `??=`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 230 | Numeric separators (`1_000_000`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 231 | Optional catch binding (`try { … } catch { … }`) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 232 | `Array.prototype.at`, `String.prototype.at`, `TypedArray.prototype.at` | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 233 | `Object.hasOwn` (preferred over `Object.prototype.hasOwnProperty.call`) | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 234 | `Error.cause` (the `new Error(msg, { cause })` chain) | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 235 | `Array.prototype.findLast` / `findLastIndex` | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 236 | `Array.prototype.toReversed`, `toSorted`, `toSpliced`, `with` (non-mutating) | all | partial | widespread | P2 | S | yes | yes | yes | yes | yes | yes |
| 237 | `Array.prototype.group`, `Array.prototype.groupToMap` | all | not-started | widespread | P2 | M | yes | yes | yes | yes | yes | yes |
| 238 | `Promise.allSettled`, `Promise.any` | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 239 | `Promise.withResolvers` | all | partial | widespread | P2 | S | yes | yes | yes | yes | yes | yes |
| 240 | `Symbol.description` | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 241 | `String.prototype.matchAll` | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 242 | `String.prototype.replaceAll` | all | partial | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 243 | `String.prototype.trimStart` / `trimEnd` (alias for `trimLeft`/`trimRight` Annex B) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 244 | `String.prototype.replace` (with replacer function) | all | shipped | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 245 | Annex B (additional legacy features: `Date.prototype.getYear`, `escape`, `unescape`, `String.prototype.substr`, `RegExp` left-context matching, `Object.prototype.__proto__`) | all | not-started (Annex B is opt-in per spec) | widespread | P2 | M | yes | yes | yes | yes | yes | yes |
| 246 | Annex C (Web browsers: `eval` semantics, `Object.prototype.__defineGetter__`, `__defineSetter__`, `__lookupGetter__`, `__lookupSetter__`, `Error.prototype.stack`, `toString` enumeration) | all | partial (basic eval yes; rest no) | widespread | P2 | M | yes | yes | yes | yes | yes | yes |

### A.3.7 E4X and historical annexes

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 247 | E4X (Annex E, `new XML(...)` syntax) | not in any current browser (SpiderMonkey removed in FF 21) | n/a | legacy | n/a | n/a | yes | yes | yes | yes | yes | yes |

### A.3.8 Internationalisation (Intl)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 248 | `Intl.Collator` (`compare`, `resolvedOptions`) | all | not-started | ubiquitous | P5 | L | yes | yes | yes | yes | yes | yes |
| 249 | `Intl.DateTimeFormat` (with all `formatMatcher`, `dateStyle`, `timeStyle`, `calendar`, `numberingSystem`, `hour12`/`hourCycle`, `timeZone` options) | all | not-started | ubiquitous | P5 | L | yes | yes | yes | yes | yes | yes |
| 250 | `Intl.ListFormat` (`conjuction`, `disjunction`, `unit` styles) | all | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | yes | yes |
| 251 | `Intl.NumberFormat` (with `notation`, `compactDisplay`, `useGrouping`, `currencyDisplay`, `currencySign`, `unit`, `unitDisplay`, `signDisplay`, `minimumIntegerDigits`, etc.) | all | not-started | ubiquitous | P5 | L | yes | yes | yes | yes | yes | yes |
| 252 | `Intl.PluralRules` (`select`, `selectRange`) | all | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | yes | yes |
| 253 | `Intl.RelativeTimeFormat` (`format`, `formatToParts`, `resolvedOptions`, `auto`, `always`, `numeric` styles) | all | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | yes | yes |
| 254 | `Intl.Segmenter` (locale-sensitive grapheme / word / sentence segmentation) | all | not-started | widespread | P5 | M | yes | yes | yes | yes | yes | yes |
| 255 | `Intl.DurationFormat` (locale-sensitive duration formatting) | all | not-started | widespread | P5 | M | yes | yes | yes | yes | yes | yes |
| 256 | `Intl.DisplayNames` (translated language, region, script, currency names) | all | not-started | widespread | P5 | M | yes | yes | yes | yes | yes | yes |
| 257 | `Intl.getCanonicalLocales`, `Intl.supportedValuesOf` (`unit`, `currency`, `calendar`, `collation`, `hourCycle`, `numberingSystem`, `timeZone`) | all | not-started | widespread | P5 | M | yes | yes | yes | yes | yes | yes |
| 258 | `Intl.Locale` (`minimize`, `maximize`, `baseName`, `language`, `script`, `region`, `variants`, `extensions`) | all | not-started | ubiquitous | P5 | M | yes | yes | yes | yes | yes | yes |
| 259 | `Intl.MessageFormat` (Stage 3 in 2025; locale message template) | all | not-started | niche | P5 | L | yes | yes | yes | yes | yes | yes |
| 260 | Locale data: BCP 47 language tags, CLDR ICU data, language fallback, ICU integration | all | not-started | ubiquitous | P5 | XL | yes | yes | yes | yes | yes | yes |

### A.3.9 Host-defined objects

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 261 | `window` / `self` / `frames` / `top` / `parent` / `opener` / `closed` (window-browsing-context) | all | not-started (Vortex DOM bindings stub returns empty `Window` per `dom_bindings/mod.rs`) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 262 | `document` (initial object handed to scripts; references the DOM `Document`) | all | not-started (Vortex stub) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 263 | `console` (`log`, `info`, `warn`, `error`, `debug`, `trace`, `assert`, `dir`, `dirxml`, `table`, `group`, `groupEnd`, `time`, `timeEnd`, `timeLog`, `count`, `clear`, `profile`, `profileEnd`) | all | partial (Vortex has `console` per `builtins/console.rs`; GAP 1.6 — "Not yet wired to `RendererToBrowser::ConsoleMessage` IPC") | ubiquitous | P2 | S | yes | yes | yes | yes | yes | yes |
| 264 | `URL` and `URLSearchParams` (the WHATWG URL parser) | all | not-started (chunk 2) | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 265 | `URLPattern` (pathname / query / hash patterns) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 266 | `TextEncoder` / `TextDecoder` (WHATWG Encoding) | all | not-started (chunk 2) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 267 | `requestIdleCallback` / `requestAnimationFrame` / `cancelAnimationFrame` | all | partial (Vortex EventLoop has `requestAnimationFrame` per its doc) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 268 | `scheduler` (the Scheduler API: `scheduler.yield`, `scheduler.postTask`) | all | not-started | niche | P3 | M | yes | partial | partial | partial | partial | partial |
| 269 | `EventCounts` (the `performance.eventCounts` count of dispatched events) | all | not-started | mixed | P3 | S | yes | yes | yes | yes | yes | yes |

### A.3.10 Storage interfaces

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 270 | `localStorage` / `sessionStorage` (the Web Storage synchronous key-value store) | all | not-started (chunk 4) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 271 | `IndexedDB` (the asynchronous object-store database) | all | not-started (chunk 4) | ubiquitous | P4 | XL | yes | yes | yes | yes | yes | yes |
| 272 | `Cache` (the request / response cache used by service workers) | all | not-started (chunk 4) | ubiquitous | P4 | L | yes | yes | yes | yes | yes | yes |
| 273 | `navigator.storage.estimate` / `persist` (storage quota API) | all | not-started (chunk 4) | ubiquitous | P4 | M | yes | yes | yes | yes | yes | yes |

### A.3.11 Engine-level sub-systems

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 274 | Bytecode VM (register or stack-based, with type-specialised opcodes) | all | not-started (GAP 1.6 — "M10–24") | ubiquitous | P3 | XL | yes | yes | yes | yes | yes | yes |
| 275 | Baseline JIT compiler (Cranelift, Baseline) | all | not-started (Bet 2 — Vortex is JIT-Optional) | ubiquitous | P5 | XL | yes | yes | yes | yes | yes | yes |
| 276 | Mark-sweep / generational / incremental GC (roots from real VM stack + globals) | all | partial (Vortex has 84 GC tests; per-origin arenas; VortexHeap ↔ Runtime glue; GAP 1.6 — "Roots from environment chain; not yet from VM stack") | ubiquitous | P2 | L | yes | yes | yes | yes | yes | yes |
| 277 | Inline caches (polymorphic / megamorphic dispatch) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 278 | WebAssembly (WASM) core spec + JS interop (`WebAssembly.Module`, `WebAssembly.Instance`, `WebAssembly.Memory`, `WebAssembly.Table`, `WebAssembly.Global`, `WebAssembly.Function`, `WebAssembly.CompileError`, `WebAssembly.LinkError`, `WebAssembly.RuntimeError`, streaming `compileStreaming`, `instantiateStreaming`, `Tag` and `Exception` for EH) | all | not-started | widespread | P5 | XL | yes | yes | yes | yes | yes | yes |
| 279 | SIMD (WebAssembly 128-bit packed types + the proposal-stage JS SIMD) | all | not-started | mixed | P5 | XL | yes | yes | yes | yes | yes | yes |
| 280 | Threads (`Atomics.wait`/`notify`, `SharedArrayBuffer`, `Worker`) | all | partial (Vortex event loop; no `Worker` yet; no `Atomics` yet) | ubiquitous | P3 / P5 | XL | yes | yes | yes | yes | yes | yes |

## Part 2: CSS (§B)


### B.1 Selectors

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 281 | Type selector (`div`), universal (`*`) | all | partial (`spiral-fmt::css::selector::TypeSelector` exists; matcher not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 282 | ID selector (`#foo`) | all | partial | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 283 | Class selector (`.foo`) | all | partial | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 284 | Attribute selector (`[attr]`, `[attr=value]`, `[attr~=value]`, `[attr|=value]`, `[attr^=value]`, `[attr$=value]`, `[attr*=value]`) with case sensitivity | all | partial (all six matchers in `spiral-fmt::css::selector::AttributeMatcher`; matcher not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 285 | Combinators (descendant, child, next-sibling, subsequent-sibling) | all | partial (`Combinator` enum) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 286 | Selector list (`,` grouping) | all | partial (`SelectorList` struct) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 287 | `:hover`, `:focus`, `:active`, `:visited`, `:link`, `:checked`, `:disabled`, `:enabled`, `:default`, `:indeterminate`, `:valid`, `:invalid`, `:required`, `:optional`, `:in-range`, `:out-of-range`, `:read-only`, `:read-write`, `:placeholder-shown`, `:defined`, `:target`, `:current`, `:past`, `:future`, `:user-invalid`, `:user-valid` (UI state pseudo-classes) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 288 | `:focus-visible`, `:focus-within` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 289 | `:has(<relative-selector-list>)` (the relational pseudo-class) | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 290 | `:is()`, `:where()`, `:not()` (the functional negation / matching / specificity-relaxing pseudo-classes) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 291 | `:nth-child(an+b [of S]?)`, `:nth-last-child(an+b [of S]?)`, `:nth-of-type(an+b)`, `:nth-last-of-type(an+b)` (the `of S` quantifier on `nth-child` is the new 2023 form) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 292 | `:only-child`, `:only-of-type`, `:first-child`, `:last-child`, `:first-of-type`, `:last-of-type`, `:empty` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 293 | `:root`, `:scope` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 294 | `:lang(<ident-or-string>)` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 295 | `:dir(ltr | rtl)`, `:any-link` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 296 | `:autofill`, `:blank`, `:user-invalid` (input states) | all | not-started | widespread | P3 | S | yes | yes | yes | yes | yes | yes |
| 297 | `::before`, `::after`, `::marker`, `::placeholder`, `::selection`, `::file-selector-button` (the standard pseudo-elements) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 298 | `::backdrop` (for `<dialog>`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 299 | `::part(<ident-list>)` and `::slotted(<compound-selector>)` (the shadow-DOM pseudo-elements) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 300 | Specificity calculation (A, B, C) and `!important` ordering | all | partial (`Specificity` struct in `spiral-fmt::css::specificity`; cascade not yet) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 301 | `*` specificity override and `:where()` zeroing | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |

### B.2 At-rules

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 302 | `@charset` (the document charset) | all | not-started (parser tokenises; meta-eval not built) | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 303 | `@import` (`url(...)`, `<string>`, `layer`, `supports(...)`, `media(...)` qualifiers) | all | partial (parser yes; fetch + cycle detection not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 304 | `@namespace`, `@namespace url(...) <ident>`, default namespace | all | partial (parser yes) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 305 | `@media` (block form with full media query list) | all | partial (parser yes; match-eval not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 306 | `@supports` (`supports(decl-list)`, `supports(selector)`, `not`, `and`, `or`) | all | partial (parser yes; eval not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 307 | `@container` (block form with name, type / size / inline-size / block-size / aspect-ratio / style / scroll-state container queries) | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 308 | `@layer` (block and statement forms) | all | partial (parser yes; cascade not) | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 309 | `@font-face` (with the full `src` `format()` list, `unicode-range`, `font-display`, `font-variation-settings`, `size-adjust`, `ascent-override`, `descent-override`, `line-gap-override`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 310 | `@font-feature-values` (`@stylistic`, `@historical-forms`, `@styleset`, `@character-variant`, `@swash`, `@ornaments`, `@annotation`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 311 | `@keyframes`, `@-webkit-keyframes` (Vendor-prefix compatibility) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 312 | `@property` (`syntax`, `inherits`, `initial-value`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 313 | `@counter-style` (`system`, `symbols`, `additive-symbols`, `negative`, `prefix`, `suffix`, `pad`, `range`, `speak-as`, `fallback`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 314 | `@page` (page area, page margin boxes, `:first`, `:left`, `:right`, named pages, `size`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 315 | `@view-transition` (the document-level opt-in for view transitions) | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 316 | `@scroll-timeline` (the explicit-name scroll-timeline at-rule) | all | not-started | mixed | P3 | L | yes | yes | yes | no | yes | yes |
| 317 | `@position-try` (the anchor-positioning fallback-policy at-rule) | all | not-started | niche | P3 | L | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag |

### B.3 Value types and units

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 318 | `<length>` (absolute: `px`, `in`, `cm`, `mm`, `pt`, `pc`, `Q`; relative: `em`, `ex`, `ch`, `rem`, `cap`, `ic`, `lh`, `rlh`, `vw`, `vh`, `vmin`, `vmax`, `vi`, `vb`, `svw`, `svh`, `lvw`, `lvh`, `dvw`, `dvh`, `cqw`, `cqh`, `cqi`, `cqb`, `cqmin`, `cqmax`) | all | partial (parser tokenises; resolution not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 319 | `<angle>` (`deg`, `rad`, `grad`, `turn`) | all | partial | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 320 | `<time>` (`s`, `ms`) | all | partial | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 321 | `<frequency>` (`Hz`, `kHz`) | all | partial | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 322 | `<resolution>` (`dpi`, `dpcm`, `dppx`, `x`) | all | partial | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 323 | `<percentage>` (`%`) | all | partial | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 324 | `<flex>` (the flex factor — a number) | all | partial | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 325 | `<integer>`, `<number>`, `<calc>` (the arithmetic on `<length>`/`<percentage>`/`<angle>`/`<time>`/`<number>`) | all | partial (`Value` enum has the basic forms; calc not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 326 | `min()`, `max()`, `clamp()`, `round()`, `mod()`, `rem()`, `abs()`, `sign()`, `sin()`, `cos()`, `tan()`, `asin()`, `acos()`, `atan()`, `atan2()`, `pow()`, `sqrt()`, `hypot()`, `log()`, `exp()` (math functions) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 327 | `color()` (with `srgb`, `srgb-linear`, `display-p3`, `a98-rgb`, `prophoto-rgb`, `rec2020`, `xyz`, `xyz-d50`, `xyz-d65` colour spaces) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 328 | `oklch()`, `oklab()`, `lch()`, `lab()` (the perceptually uniform colour functions) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 329 | `color-mix(in <color-space>, <color>, <color> [<percentage>])` (colour mixing) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 330 | Relative colour syntax (`oklch(from <color> l c h / a)`) | all | not-started | mixed | P3 | M | yes | yes | yes | yes | yes | yes |
| 331 | `light-dark(<light>, <dark>)` (the preferred-color-scheme colour pairing) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 332 | `attr(<ident> <type-or-unit> [, <fallback>])` (the typed attribute value) | all | not-started | mixed | P3 | M | yes | yes | yes | yes | yes | yes |
| 333 | `url()`, `src()` (the unified src function for `@font-face`), `image()` (`<image>`, `<color>`, `image-set()`, `cross-fade()`) | all | partial (parser tokenises `url()`; rest not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 334 | `var()`, `env()` (the custom-property substitution and the UA-environment lookup) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 335 | `counter()`, `counters()`, `content()` (the generated-content counters) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 336 | `random()`, `random-item()` (the procedural-content functions) | all | not-started | experimental | P5 | M | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag |
| 337 | `toggle()`, `lang()`, `first-valid()` (the legacy value functions) | all | not-started | legacy | P3 | M | no | yes | no | no | no | no |
| 338 | `sibling-index()`, `sibling-count()` (the structural-selector-driven value functions) | all | not-started | niche | P3 | M | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag |

### B.4 Box model and layout

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 339 | `display` (the multi-keyword form: `block flow`, `inline flow`, `inline-block flow`, `flex flow`, `grid flow`, `inline-flex flow`, `inline-grid flow`, `flow-root flow`, `table`, `table-row`, `table-cell`, `table-caption`, `list-item`, `none`, `contents`, `inline flow-root`, etc.) | all | not-started (Gyre partial) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 340 | `display: contents` (the disappearing-parent box) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 341 | `display: list-item` (`list-style-type`, `list-style-position`, `list-style-image`, `::marker`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 342 | `position` (`static`, `relative`, `absolute`, `fixed`, `sticky`) | all | not-started (Gyre partial) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 343 | `top`, `right`, `bottom`, `left` (offsetting for `position: non-static`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 344 | `z-index` (stacking) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 345 | `width`, `height`, `min-width`, `min-height`, `max-width`, `max-height` (intrinsic and extrinsic sizing) | all | not-started (Gyre partial) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 346 | `aspect-ratio` (preferred aspect ratio) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 347 | `box-sizing` (`content-box`, `border-box`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 348 | `margin`, `margin-*`, `margin-block`, `margin-inline`, `margin-trim` (margin collapse, margin trim) | all | not-started (Gyre partial) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 349 | `padding`, `padding-*`, `padding-block`, `padding-inline` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 350 | `border`, `border-*`, `border-block`, `border-inline` (incl. `border-block-start`, `border-inline-end`, etc.) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 351 | `border-radius` (8 corner radii: `border-{top,right,bottom,left}-{start,end}-radius`, with elliptical form) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 352 | `border-image` (`source`, `slice`, `width`, `outset`, `repeat`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 353 | `box-shadow`, `text-shadow` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 354 | `outline`, `outline-*`, `outline-offset`, `outline-color`, `outline-style`, `outline-width` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 355 | `overflow`, `overflow-x`, `overflow-y` (`visible`, `hidden`, `clip`, `scroll`, `auto`), `overflow-clip-margin` | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 356 | `overflow: clip` and `overflow-clip-margin` (the non-scroll-clipping variant) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 357 | `visibility` (`visible`, `hidden`, `collapse`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 358 | `float` and `clear` | all | not-started (Gyre partial) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 359 | `object-fit` (`fill`, `contain`, `cover`, `none`, `scale-down`) and `object-position` | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 360 | `contain` (`none`, `strict`, `content`, `size`, `layout`, `style`, `paint`, `inline-size`, `block-size`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 361 | `content-visibility` (`visible`, `auto`, `hidden`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |

### B.5 Backgrounds, images, filters, masks, compositing

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 362 | `background`, `background-color`, `background-image`, `background-position`, `background-size`, `background-repeat`, `background-origin`, `background-clip`, `background-attachment` | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 363 | `background-image` with `image-set(...)` and `image(...)` | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 364 | `filter` (`blur`, `brightness`, `contrast`, `drop-shadow`, `grayscale`, `hue-rotate`, `invert`, `opacity`, `saturate`, `sepia`, `url(#filter)`, `none`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 365 | `backdrop-filter` (the same set as filter, but on the back of the element) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 366 | `mask`, `mask-image`, `mask-mode`, `mask-repeat`, `mask-position`, `mask-clip`, `mask-origin`, `mask-size`, `mask-composite`, `mask-type` | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 367 | `mix-blend-mode` and `isolation` (the CSS Compositing and Blending Level 1 properties) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 368 | `opacity` (0–1; applies a stack-group effect) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 369 | `will-change` (`auto`, `<ident>`, `transform`, `opacity`, `scroll-position`, `contents`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |

### B.6 Text, fonts, writing modes, list styles

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 370 | `font` shorthand and longhands (`font-family`, `font-size`, `font-style`, `font-weight`, `font-stretch`, `font-variant-*`, `font-feature-settings`, `font-variation-settings`, `line-height`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 371 | System and web fonts (the platform-font stack: `system-ui`, `ui-serif`, `ui-sans-serif`, `ui-monospace`, `ui-rounded`, `-apple-system`, `BlinkMacSystemFont`, `Roboto`, `Segoe UI`, `Helvetica`, `Arial`, `sans-serif`, `serif`, `monospace`, `cursive`, `fantasy`, `emoji`, `math`, `fangsong`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 372 | Variable fonts (`font-variation-settings`, `font-optical-sizing`, `font-weight: 1..1000`, `font-stretch: 50%..200%`) | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 373 | `color` (named colours, `currentColor`, `transparent`, `hex`, `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hwb()`, `lab()`, `lch()`, `oklab()`, `oklch()`, `color()`) | all | not-started (parser tokenises the basic forms) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 374 | `line-height` (`normal`, `<number>`, `<length>`, `<percentage>`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 375 | `text-align` (`start`, `end`, `left`, `right`, `center`, `justify`, `justify-all`, `match-parent`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 376 | `text-align: start` / `end` and `:dir(ltr/rtl)` interaction | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 377 | `text-decoration` (`text-decoration-line`, `text-decoration-style`, `text-decoration-color`, `text-decoration-thickness`, `text-underline-offset`, `text-underline-position`, `text-decoration-skip-ink`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 378 | `text-transform` (`none`, `capitalize`, `uppercase`, `lowercase`, `full-width`, `full-size-kana`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 379 | `white-space` (`normal`, `nowrap`, `pre`, `pre-wrap`, `pre-line`, `break-spaces`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 380 | `word-break` (`normal`, `break-all`, `keep-all`, `auto-phrase`), `word-wrap` / `overflow-wrap` (`normal`, `break-word`, `anywhere`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 381 | `text-wrap` (`wrap`, `nowrap`, `balance`, `pretty`, `stable`) | all | not-started | mixed | P3 | M | yes | yes | yes | yes | yes | yes |
| 382 | `text-wrap: balance` / `pretty` (the modern line-balancing options) | all | not-started | mixed | P3 | M | yes | yes | yes | yes | yes | yes |
| 383 | `line-break` (`auto`, `loose`, `normal`, `strict`, `anywhere`), `word-spacing`, `letter-spacing`, `tab-size` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 384 | `direction` (`ltr`, `rtl`) and `unicode-bidi` (`normal`, `embed`, `isolate`, `bidi-override`, `isolate-override`, `plaintext`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 385 | `writing-mode` (`horizontal-tb`, `vertical-rl`, `vertical-lr`, `sideways-rl`, `sideways-lr`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 386 | `font-variant` shorthand and the full `font-variant-*` longhand set (incl. `font-variant-caps`, `font-variant-numeric`, `font-variant-ligatures`, `font-variant-alternates`, `font-variant-east-asian`, `font-variant-position`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 387 | `font-variant-emoji` (`normal`, `text`, `emoji`, `unicode`) | all | not-started | mixed | P3 | S | yes | yes | yes | yes | yes | yes |
| 388 | `font-synthesis` (`weight`, `style`, `small-caps`, `position`, `none`, `auto`) and `font-synthesis-position`, `font-synthesis-small-caps` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 389 | `font-kerning` (`auto`, `normal`, `none`), `font-optical-sizing` (`auto`, `none`), `font-language-override` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 390 | `font-feature-settings` (the OpenType low-level feature toggles) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 391 | `font-size-adjust` (`none`, `ex-height`, `cap-height`, `ch-width`, `ic-width`, `from-font`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 392 | `letter-spacing` (tracking), `word-spacing` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 393 | `text-indent`, `text-justify`, `text-align-last` | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 394 | `list-style` (shorthand), `list-style-type` (incl. `disc`, `circle`, `square`, `decimal`, `decimal-leading-zero`, `lower-roman`, `upper-roman`, `lower-greek`, `lower-latin`, `upper-latin`, `armenian`, `georgian`, `lower-alpha`, `upper-alpha`, `none`, `disclosure-open`, `disclosure-closed`, `<counter-style>`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 395 | `list-style-position` (`inside`, `outside`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 396 | `list-style-image` (with `url()` and image fallbacks) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |

### B.7 Flexbox

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 397 | `display: flex` / `inline-flex` (the flex container) | all | not-started (Gyre partial) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 398 | `flex-direction` (`row`, `row-reverse`, `column`, `column-reverse`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 399 | `flex-wrap` (`nowrap`, `wrap`, `wrap-reverse`) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 400 | `flex-flow` (shorthand) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 401 | `justify-content` (`flex-start`, `flex-end`, `center`, `space-between`, `space-around`, `space-evenly`, `normal`, `stretch`, `start`, `end`, `left`, `right`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 402 | `align-items` and `align-self` (`flex-start`, `flex-end`, `center`, `baseline`, `stretch`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 403 | `align-content` (the multi-line cross-axis) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 404 | `order` (the visual order, no DOM change) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 405 | `flex-grow`, `flex-shrink`, `flex-basis` (the flex factor) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 406 | `flex` shorthand | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 407 | `gap`, `row-gap`, `column-gap` (the shared multi-layout gap property) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |

### B.8 Grid

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 408 | `display: grid` / `inline-grid` | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 409 | `grid-template-columns`, `grid-template-rows` (with `repeat()`, `minmax()`, `auto-fit`, `auto-fill`, named lines, named areas) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 410 | `grid-template-areas`, `grid-area`, named areas | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 411 | `grid` shorthand, `grid-template` shorthand | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 412 | `grid-column`, `grid-row`, `grid-row-start`/`end`, `grid-column-start`/`end` (with `<ident>`, `<integer>`, `span`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 413 | `grid-auto-rows`, `grid-auto-columns`, `grid-auto-flow` (`row`, `column`, `dense`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 414 | `justify-self`, `align-self`, `justify-items`, `align-items` (the grid alignment set) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 415 | `place-content`, `place-items`, `place-self` (the grid-alignment shorthand trio) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 416 | Subgrid (`grid-template-columns: subgrid`, `grid-template-rows: subgrid`) | all | not-started | widespread | P3 | L | yes | yes | yes | no | yes | yes |
| 417 | Masonry layout (`grid-template-rows: masonry`, `masonry-auto-flow`) | all | not-started | experimental | P3 / P5 | L | no | behind-flag | no | behind-flag | behind-flag | behind-flag |
| 418 | `masonry-template-rows`, `masonry-template-columns`, `masonry-slack` | all | not-started | experimental | P5 | L | yes | partial | yes | yes | yes | yes |

### B.9 Transforms, animations, transitions, motion

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 419 | `transform` (with `translate`, `translateX`, `translateY`, `translateZ`, `translate3d`, `scale`, `scaleX`, `scaleY`, `scaleZ`, `rotate`, `rotateX`/`Y`/`Z`, `rotate3d`, `skew`, `skewX`, `skewY`, `matrix`, `matrix3d`, `perspective`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 420 | `transform-origin`, `transform-style` (`flat`, `preserve-3d`), `perspective`, `perspective-origin`, `backface-visibility` | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 421 | Individual `translate` / `scale` / `rotate` properties (the 2022 longhand form) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 422 | `transition` shorthand and longhands (`transition-property`, `transition-duration`, `transition-timing-function`, `transition-delay`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 423 | `transition-behavior: allow-discrete` (the discrete-property transition allow-flag) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 424 | `animation` shorthand and longhands (`animation-name`, `animation-duration`, `animation-timing-function`, `animation-delay`, `animation-iteration-count`, `animation-direction`, `animation-fill-mode`, `animation-play-state`, `animation-composition`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 425 | `@keyframes` rule (with `from`/`to`, `<percentage>`) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 426 | `animation-composition` (`replace`, `add`, `accumulate`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 427 | CSS `cubic-bezier()`, `steps()`, `linear`, `ease`, `ease-in`, `ease-out`, `ease-in-out` (the timing-function values) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 428 | `linear()` easing function (the per-keyframe linear-progress easing, CSS Easing 2) | all | not-started | mixed | P3 | S | yes | yes | yes | yes | yes | yes |
| 429 | Scroll-driven animations (animation-timeline: `scroll()`, `view()`; animation-range: `cover`, `contain`, `entry`, `exit`, `entry-crossing`, `exit-crossing`, `normal`) | all | not-started | niche | P3 | L | yes | yes | yes | no | yes | yes |
| 430 | `scroll-timeline`, `scroll-timeline-name`, `scroll-timeline-axis`, `view-timeline`, `view-timeline-name`, `view-timeline-axis` | all | not-started | niche | P3 | L | yes | yes | yes | yes | yes | yes |
| 431 | `offset-path` (with `path()`, `ray()`, `<basic-shape>`, `none`) and `offset-distance`, `offset-rotate`, `offset-anchor`, `offset-position` | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 432 | View Transitions (the cross-document and same-document `document.startViewTransition()`, `::view-transition`, `::view-transition-group()`, `::view-transition-image-pair()`, `::view-transition-old()`, `::view-transition-new()`, `@view-transition`) | all | not-started | widespread | P3 | XL | yes | yes | yes | yes | yes | yes |

### B.10 Container queries and draft integration

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 433 | `container` (the shorthand for `container-name` / `container-type`), `container-name` (a list of `<ident>`), `container-type` (`normal`, `size`, `inline-size`, `block-size`) | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 434 | `container-query` (the `@container` block at-rule, with `name`, `type`, `inline-size`, `block-size`, `aspect-ratio`, `style`, `scroll-state` queries) | all | not-started | widespread | P3 | L | yes | yes | yes | yes | yes | yes |
| 435 | `@container style()`, `@container scroll-state()` (the newer query categories) | all | not-started | mixed | P3 | M | yes | yes | yes | yes | yes | yes |
| 436 | `contain-intrinsic-size` (the reserved intrinsic size for `content-visibility: auto`) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 437 | Anchor positioning (the `anchor-name` / `position-anchor` / `position-area` / `position-try` / `position-try-order` / `position-try-fallbacks` / `position-visibility` set) | all | not-started | niche | P3 | L | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag | behind-flag |
| 438 | `field-sizing` (`fixed`, `content`) (the new form-input-sizing property) | all | not-started | mixed | P3 | S | yes | yes | yes | yes | yes | yes |
| 439 | `interactivity` (`auto`, `none`) (the inert-content-via-CSS feature) | all | not-started | niche | P3 | M | partial | yes | yes | yes | yes | yes |

### B.11 MathML, SVG (CSS side)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 440 | CSS in MathML (top-level defaults for `math` element) | all | not-started (depends on #39 foreign content) | mixed | P3 | M | no | yes | yes | yes | yes | yes |
| 441 | SVG-in-CSS (the `fill`, `stroke`, `stroke-width`, `stroke-dasharray`, `stroke-linecap`, `stroke-linejoin`, `vector-effect`, `paint-order` properties in CSS) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 442 | `clip-path` (`none`, `<basic-shape>`, `url(#clip)`, `<geometry-box>`, `margin-box`, `border-box`, `padding-box`, `content-box`, `fill-box`, `stroke-box`, `view-box`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 443 | `path()` (the CSS shape source) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 444 | `<basic-shape>` (the `circle()`, `ellipse()`, `inset()`, `polygon()`, `xywh()`, `rect()` set) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 445 | `shape-outside` (the text-wrap-around-a-shape property) | all | not-started | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 446 | `shape-image-threshold`, `shape-margin` | all | not-started | widespread | P3 | S | yes | yes | yes | yes | yes | yes |
| 447 | CSS exclusions (the older `wrap-flow` / `wrap-through` set, not implemented in modern engines) | not in any current browser | n/a | legacy | n/a | n/a | — | — | — | — | — | — |
| 448 | CSS regions (`flow-from` / `flow-into`; deprecated) | not in any current browser | n/a | legacy | n/a | n/a | — | — | — | — | — | — |

### B.12 Cascade and origin

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 449 | Cascade origins (user-agent, user, author, animation, transition) | all | not-started (parser yes; cascade not) | ubiquitous | P3 | L | yes | yes | yes | yes | yes | yes |
| 450 | Cascade priority (normal vs `!important`, with normal < important at same origin) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 451 | Cascade layers (`@layer`, `layer()`) | all | not-started (parser yes) | widespread | P3 | M | yes | yes | yes | yes | yes | yes |
| 452 | Cascade sorting per origin (specificity + order) | all | not-started (specificity struct exists) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 453 | `all` shorthand (reset or inherit of all properties) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 454 | `inherit`, `initial`, `unset`, `revert`, `revert-layer` (the cascade-keyword set) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
| 455 | Custom properties (`--foo: bar;`) with `var(--foo)` consumption and `@property` registration | all | not-started (parser tokenises; resolution not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 456 | `env()` and `env(viewport-segment-width, …)` (UA-defined environment variables) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |

### B.13 Cascade integration and media queries

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 457 | Media query level 5 (`prefers-color-scheme`, `prefers-reduced-motion`, `prefers-reduced-transparency`, `prefers-contrast`, `prefers-reduced-data`, `prefers-color-scheme: dark`, `prefers-color-scheme: light`, `forced-colors`, `hover`, `pointer`, `any-pointer`, `any-hover`, `orientation`, `aspect-ratio`, `resolution`) | all | not-started (parser tokenises; match-eval not) | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 458 | Media query level 4 (`scripting`, `update`, `overflow-block`, `overflow-inline`, `grid`, `color-gamut`, `dynamic-range`, `video-dynamic-range`, `device-aspect-ratio`, `monochrome`) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes | yes |
| 459 | User-preference queries (`@media (prefers-color-scheme: dark)`, etc.) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes | yes |
