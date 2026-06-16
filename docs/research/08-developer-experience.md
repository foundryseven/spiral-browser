# Chunk 8 — Developer & Power-User Surface

> **Chunk 8 of 14.** This is the **developer and power-user** chunk.
> The surface developers, testers, automation harnesses, and CI
> engineers actually touch. It is **distinct from chunk 7** (deep
> end-user UX): chunk 7 covers what the human user clicks; this chunk
> covers what the developer types, the automation driver drives, and
> the test rig runs.
>
> **Worktree:** `research/competitive-parity` (base: `audit/m4-window`).
> **Methodology contract:** `00-methodology.md`. **Source ladder:**
> `citations/sources.md`. **Output contract:** `README.md` §"Per-chunk
> output contract".
>
> **Naming:** per `00-methodology.md` §7 — spec or capability names,
> not product brand names. The inspector shipped by Chromium is
> "DevTools"; the one shipped by WebKit is "Web Inspector". Both
> become rows; the *engine*, not the vendor, decides the row's
> coverage status.

---

## Scope

**In:** DevTools panels (Inspector, Styles, Box Model, Network,
Sources, Console, Memory, Performance, Application, Security,
Recorder, Animations, Coverage, Sensors, Network conditions,
Rendering, Audits/Lighthouse); DevTools console features (REPL,
top-level await, command-line API, inline previews, eager
evaluation); DevTools source features (source maps, breakpoint
types, watch, scope, call stack, async stack, blackboxing,
snippets, FS access, local overrides, exception breakpoints);
DevTools network features (throttling, HAR export, request
blocking, initiator chain, WebSocket/SSE inspector, headers,
cookies, cache, mixed content, CORS preflight detail); DevTools
performance features (flame chart, main-thread breakdown, long
task, layout shift, performance insights, frame viewer, memory
timeline, allocation profiling); DevTools application features
(storage inspector, DB tools, manifest viewer); DevTools security
features (cert viewer, mixed content, CSP violation list, COOP/
COEP/CORP, secure context, HTTPS-only indicator); DevTools
accessibility features (a11y tree, contrast, ARIA inspector);
remote-debugging protocols (CDP, WebKit Inspector Protocol,
Firefox RDP, USB + network targets); view source; reader mode
developer surface; error pages; offline / no-internet pages;
internal `about:` / `chrome://` / `edge://` / `about:config`
surface; browser flags; test pages (WPT, WebDriver / WebDriver
BiDi, Playwright, Puppeteer); headless mode (`--headless`,
"headless: new", headless surface area); automation protocols
(WebDriver, WebDriver BiDi, CDP, Marionette, Playwright,
Puppeteer, Selenium); built-in PDF viewer; PDF generation
(page-to-PDF, screen-to-PDF, print-to-PDF, custom size/margin);
RSS / Atom feed discovery (`<link rel="alternate">`, live
bookmark, WebSub); user scripts (per-site JS/CSS); internal
protocol handlers (`view-source:`, `devtools:`, `about:`,
`chrome:`, `edge:`, `browser:`, `javascript:`); inline data /
blob URLs; onboarding (telemetry opt-in, default-browser check,
profile creation); performance and memory (task manager, energy
impact, network usage); diagnostic dumps (net-export, tracing,
heap dumps, profile dumps, GPU dumps); compatibility view
(enterprise mode, IE emulation); network condition simulation
(CPU / network throttle, sensor override); external application
handler (`web+myapp`, custom file extensions, mailto); local
file access (`file://`, directory listing, mixed content,
local→remote fetch).

**Out:** engine internals (chunk 1), security policy (chunk 3),
storage mechanics (chunk 4), media playback APIs (chunk 5),
web-platform APIs (chunk 6), user-facing UX (chunk 7),
accessibility (chunk 9), extension API surface (chunk 10),
distribution / installer (chunk 11), AI assistant integration
(methodology §9), crypto wallet (methodology §9).

---

## Spiral ground truth (verified 2026-06-16)

| Sub-system | Crate | State | What "shipped" means here |
|------------|-------|-------|---------------------------|
| HTML tokeniser + tree builder | `spiral-fmt::html` | real code, 8 insertion modes | `spiral_fmt::parse_html` |
| CSS parser (tokeniser / parser / selectors / specificity / values / at-rules / declarations / attribute matchers) | `spiral-fmt::css` | real code, 8 modules | `spiral_fmt::parse_css` |
| DOM | `spiral-dom` | `Node`, `Element`, `Text`, `Comment`, `Document`, `Dom` + arena | returned by `parse_html` |
| Layout (Gyre) | `spiral-gyre` | box-model types + one trivial pass | stylesheet currently unused |
| JS engine (Vortex) | `spiral-vortex` | tree-walking interp, microtask + macrotask loop | in-process only |
| DOM bindings | `spiral-vortex::dom_bindings` | stub (returns empty `JsObject`) | none |
| Network (Track E wrapper) | `spiral-network` | `Client` trait + hyper-backed impl | partial, M4.5+ |
| Crypto | `spiral-crypto` | real sha2 + getrandom, 18 tests | `spiral_crypto::*` |
| Image decoder | `spiral-imagedecoder` | format sniffing works; decode returns 1×1 | `spiral_imagedecoder::Decoder` is a stub |
| Browser binary | `spiral-browser` | winit window, event loop, tabs, theme, shell | M4.5 partial; no chrome pages yet |
| Internal pages | `spiral-browser` | `about:blank` only (`spiral-core/src/lib.rs:103`) | none |
| DevTools | none | not started | methodology §5: `v0.1-blocker` or `1.0-blocker` |
| Headless mode | none | not started | tracked in `specs/GAP_ANALYSIS.md` |
| PDF | none | not on roadmap as a dedicated crate | `spiral-imagedecoder` is PDF-adjacent, not PDF |
| RSS / Atom reader | none | not started | not in any current sprint |
| WebDriver / automation | none | not started | not in any current sprint |
| WPT fixtures | none | `tests/wpt/` exists, empty | GAP §5.2 |
| CDP / WebKit Inspector / Firefox RDP | none | not started | not in any current sprint |
| Error pages | none | no chrome pages yet | not in any current sprint |
| Browser flags (`chrome://flags`-equivalent) | none | not started | not in any current sprint |
| Task manager / energy impact | none | not started | not in any current sprint |
| Net-export / tracing / heap dumps | none | not started | not in any current sprint |
| User scripts | none | not started | extension-adjacent, v1.0 |
| `file://` access | none | not started | not in any current sprint |

These are the bars for the `Status in Spiral` column on every row.

---

## Cross-references to `specs/GAP_ANALYSIS.md`

These rows in the GAP file are direct ground truth for the "Status in
Spiral" column and are referenced from the row tables below.

| GAP section | Title | Status in GAP | What it covers in this chunk |
|-------------|-------|---------------|------------------------------|
| §1.1 | HTML parser (`spiral-fmt`) | partial (8 insertion modes, 13 e2e tests) | DevTools DOM-tree inspection rows; view source |
| §1.2 | CSS parser & cascade | partial (parser done; cascade deferred) | DevTools Styles / Computed / Cascade panels |
| §1.3 | DOM | partial (tree yes, API surface no) | DevTools Inspector; accessibility tree |
| §1.4 | Layout — Gyre | partial (box model yes; rest no) | DevTools Box Model + Layout panels |
| §1.5 | Render | partial (software renderer) | DevTools Rendering; Performance / paint |
| §1.6 | Vortex | partial (lex/parse/interp yes; VM/built-ins/closures partial; DOM bindings stub) | DevTools Console; Sources; Debugger |
| §1.7 | Shared-Everything Multi-Process (Bet 1) | partial (types only) | Per-process task manager; per-tab memory |
| §3 (chrome UI) | DevTools element inspector, console, network | `[ ]` | Rows §1–§3 below |
| §4.1 | SOP / CSP / mixed content / HSTS | `[ ]` | DevTools Security panel rows |
| §4.2 | Filter / ad-blocking | partial (policy engine only) | DevTools Network request-blocking row |
| §4.4 | `spiral-media` (MSE/EME/codecs) | not started | DevTools Media panel (out of scope here, chunk 5) |
| §4.5 | i18n | not started | Console-locale features (deferred) |
| §5.1 | IPC + per-process routing | partial (transport done; per-process routing keys deferred) | Remote debugging; USB/network targets |
| §5.2 | WPT / test infrastructure | `[ ]` (`tests/wpt/` empty) | WPT runner row; harness row |
| §5.3 | Build / CI / docs | mixed | `cargo bench`, `cargo-llvm-cov` rows |
| §7 (Boats) #1 | `spiral-fmt` from-spec | shipped M4.4.1 | view source; dev tools DOM inspection |
| §7 (Boats) #2 | `spiral-crypto` security bug | shipped M4.4 | DevTools Security: cert viewer (adjacent) |
| §7 (Boats) #6 | Vortex ↔ browser console pipe | not started | DevTools Console REPL row |

GAP_ANALYSIS rows that fall **outside** chunk 8's scope:

- §1.6 (Vortex VM / built-ins) — chunk 1 (engine-internal)
- §2.1–2.4 (HTTP / TLS / DNS, Storage, Image decoder) — chunks 2/4/6
- §3 (chrome UI surface) — chunk 7 (deep UX)
- §4.1 (security policy) — chunk 3
- §4.3 (GPU / compositing) — chunk 9
- §4.4 (media) — chunk 5

---

## Tables

Status in Spiral: per `00-methodology.md` §3 — `shipped` (audited wired),
`partial` (real code, partial coverage), `designed` (documented intent,
no code), `not-started`. Phase impact: per `00-methodology.md` §5 —
P1 = P0-internal, P2 = core engine, P3 = application shell,
P4 = platform, P5 = advanced. Complexity: `XS` / `S` / `M` / `L` / `XL`
per `00-methodology.md` §6. Surface: `desktop` / `mobile` / `embedded`
(mobile + embedded = WKWebView, Android System Web View, WebView2, Servo
WebView; see methodology §2). Browser prevalence: methodology §4 —
`>=90%` / `75-90%` / `50-75%` / `25-50%` / `<25%` / `behind flag` /
`not in any shipped browser`.

### 1. DevTools panels

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 1 | **Element Inspector (DOM tree, attributes, box model overlay, "Inspect" tool)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable (Web Inspector). Servo: partial (no overlay). Ladybird: behind flag (`ladybird --devtools`). Flow: unknown (re-verify in chunk 12). | [MDN](https://developer.mozilla.org/en-US/docs/Learn/Common_questions/Tools_and_setup/What_are_browser_developer_tools) · [Chrome DevTools Docs](https://developer.chrome.com/docs/devtools/) |
| 2 | **Styles / Computed / Cascade panel (matched rules, cascade layers, `!important`, custom properties, source link)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable (`styles`, `computed`, `cascade` panels). Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN CSS pane](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Cascade_layers) |
| 3 | **Layout / Box Model panel (padding/border/margin, position offsets, getComputedStyle live readback)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN getComputedStyle](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle) |
| 4 | **Network panel (request list, headers, body, initiator chain, timing breakdown, resource type filter)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable (full HAR-equivalent). Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Network reference](https://developer.chrome.com/docs/devtools/network/reference) |
| 5 | **Sources / Debugger panel (file tree, editor with syntax highlight, breakpoints pane, call stack, scope, watch)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | XL | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Sources](https://developer.chrome.com/docs/devtools/javascript/reference) |
| 6 | **Console (REPL, log levels, object previews, log grouping, filtering, preserve log)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: partial. Ladybird: no. Flow: unknown. | [Console API spec](https://console.spec.whatwg.org/) · [MDN console](https://developer.mozilla.org/en-US/docs/Web/API/console) |
| 7 | **Memory / Heap profiler (heap snapshots, allocation timeline, retaining paths, sampling profiler)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | XL | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Memory](https://developer.chrome.com/docs/devtools/memory-problems/) |
| 8 | **Performance panel (recording, flame chart, main-thread breakdown, scripting/rendering/painting, frame timeline, long tasks, layout shifts, performance insights)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | XL | Chromium: yes/stable (rebranded as "Performance Insights" 2024). Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Performance Insights](https://developer.chrome.com/docs/devtools/performance-insights/) |
| 9 | **Application panel (Storage, Service Workers, IndexedDB, CacheStorage, OPFS, Background Fetch, Background Sync, Interest Groups, Shared Storage, Cookies, Manifest)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | XL | Chromium: yes/stable. Gecko: yes/stable (subset; OPFS yes, Shared Storage no). WebKit: yes/stable (subset). Servo: no. Ladybird: no. Flow: unknown. | [MDN Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API) · [Chrome DevTools Application](https://developer.chrome.com/docs/devtools/storage/application) |
| 10 | **Security panel (certificate viewer, TLS details, mixed-content listing, CSP violation list, COOP/COEP/CORP, secure-context check, HTTPS-only indicator)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Security](https://developer.chrome.com/docs/devtools/security/) |
| 11 | **Recorder (record user interactions, replay, export as Puppeteer / Chrome extension / JSON trace)** | desktop | not-started | `>=90%` (Chromium-only, but ubiquitous in Chromium family) | P3 | L | Chromium: yes/stable. Gecko: no. WebKit: no. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Recorder](https://developer.chrome.com/docs/devtools/recorder/) |
| 12 | **Animations panel (scrub through CSS / Web Animations, slow-down, replay)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Animations](https://developer.chrome.com/docs/devtools/css/animations/) |
| 13 | **Coverage panel (per-file used-vs-unused CSS / JS bytes, export as Istanbul / raw)** | desktop | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | M | Chromium: yes/stable. Gecko: no. WebKit: no. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Coverage](https://developer.chrome.com/docs/devtools/coverage/) |
| 14 | **Sensors panel (override geolocation, accelerometer, gyroscope, device orientation, ambient light, force touch)** | desktop | not-started | `>=90%` (Chromium) / `75-90%` overall | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Sensors](https://developer.chrome.com/docs/devtools/sensors/) |
| 15 | **Network conditions (latency / bandwidth / offline throttle, user-agent override, custom client hints, content-encoding override)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Throttling](https://developer.chrome.com/docs/devtools/network/reference/#throttling) |
| 16 | **Rendering panel (force colours, contrast, prefers-reduced-motion, paint flashing, layer borders, scroll performance, font fallback, emulate CSS media)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: partial. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Rendering](https://developer.chrome.com/docs/devtools/rendering/) |
| 17 | **Audits / Lighthouse (Lighthouse run from DevTools, performance / a11y / SEO / PWA categories, JSON export)** | desktop | not-started | `>=90%` (Chromium family) | P3 | L | Chromium: yes/stable. Gecko: no (Lighthouse runs externally). WebKit: no. Servo: no. Ladybird: no. Flow: unknown. | [Lighthouse](https://developer.chrome.com/docs/lighthouse/overview/) |

### 2. DevTools console features

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 18 | **Console REPL with autocomplete, multi-line edit, history (up-arrow)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: partial. Ladybird: no. Flow: unknown. | [Console API spec](https://console.spec.whatwg.org/) |
| 19 | **Top-level await in REPL (no need to wrap in IIFE for `await`)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Top-level await TC39 proposal](https://github.com/tc39/proposal-top-level-await) |
| 20 | **Command-line API (`$`, `$$`, `$0`–`$4`, `dir`, `keys`, `values`, `queryObjects`, `monitor`, `monitorEvents`, `unmonitor`, `copy`, `clear`, `table`, `profile`, `profileEnd`, `inspect`)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN Command line API](https://developer.mozilla.org/en-US/docs/Web/API/Console_API) |
| 21 | **Inline object previews (lazy-expandable trees, getters shown on hover, "copy property path")** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Objects](https://developer.chrome.com/blog/devtools-tips-1/) |
| 22 | **Eager evaluation (live result while typing, with debounced execution to avoid side effects)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Console features](https://developer.chrome.com/docs/devtools/console/) |

### 3. DevTools source features

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 23 | **Source maps (v3 + v3.1+ extensions, `sourcesContent`, `x_google_ignoreList`, source map debugging-in-page)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Source map v3 spec](https://sourcemaps.info/spec.html) · [MDN](https://developer.mozilla.org/en-US/docs/Tools/Debugger/How_to/Use_a_source_map) |
| 24 | **Breakpoint types: line, conditional, logpoint ("tracepoint"), XHR/fetch URL, DOM mutation, event listener, exception (caught / uncaught / none)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Breakpoints](https://developer.chrome.com/docs/devtools/javascript/breakpoints/) |
| 25 | **Watch expressions, scope inspection (local, closure, global, block, module), call stack with click-to-jump** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Watch](https://developer.chrome.com/docs/devtools/javascript/reference/#watch) |
| 26 | **Async stack traces (full chain across `await` / `Promise.then` / microtasks)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable (`asyncStackTagDepthLimit`, `asyncCallStackDepth`). Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [V8 async stack](https://v8.dev/blog/v8-release-58) |
| 27 | **Blackboxing (skip frames from a file/pattern when stepping, show only call-site on stack)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN Blackbox](https://developer.mozilla.org/en-US/docs/Tools/Debugger/How_to/Blackbox_a_source) |
| 28 | **Snippets (save JS, run in any frame's context, multi-snippet pane)** | desktop | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | S | Chromium: yes/stable. Gecko: yes/stable ("Scratchpad" historical, removed in 113; replaced by "Run" snippet in newer). WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Snippets](https://developer.chrome.com/docs/devtools/javascript/snippets/) |
| 29 | **File-system access (`filesystem:`, File System Access API, `showDirectoryPicker` from DevTools, "save as" workspace folder)** | desktop | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | M | Chromium: yes/stable. Gecko: partial (no DevTools FS pane). WebKit: partial. Servo: no. Ladybird: no. Flow: unknown. | [MDN File System Access](https://developer.mozilla.org/en-US/docs/Web/API/File_System_Access_API) |
| 30 | **Local overrides (workspace folder mapping, "save changes locally" to disk, "enable local overrides" toggle)** | desktop | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | L | Chromium: yes/stable. Gecko: yes/stable ("Local overrides" 2024). WebKit: partial. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Overrides](https://developer.chrome.com/docs/devtools/overrides/) |
| 31 | **Exception breakpoints ("pause on caught", "pause on uncaught", filter by source)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Pause on exceptions](https://developer.chrome.com/docs/devtools/javascript/breakpoints/#exceptions) |
| 32 | **Inline value preview on hover (variable under caret shown in tooltip)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Hover preview](https://developer.chrome.com/docs/devtools/javascript/reference/) |
| 33 | **Pretty-print minified sources (`{}` button on Sources panel)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Pretty print](https://developer.chrome.com/docs/devtools/javascript/reference/#pretty-print) |

### 4. DevTools network features

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 34 | **Network throttling (latency ms, download kbps, upload kbps, offline preset, custom profile, slow 3G / fast 3G presets)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Throttling](https://developer.chrome.com/docs/devtools/network/reference/#throttling) |
| 35 | **HAR export (request, response, timings, content, cookies, queryString, postData)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable (HAR 1.2 spec). Gecko: yes/stable (extension / addon historically). WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [HAR 1.2 spec](http://www.softwareishard.com/blog/har-12-spec/) |
| 36 | **Request blocking (`*.png`, `*google-analytics*` patterns, persisted across reloads)** | desktop | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Request blocking](https://developer.chrome.com/docs/devtools/network/reference/#block) |
| 37 | **Request initiator chain (Parser, Script, Other, "Initiator" column, "Stack trace" expansion)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Initiator](https://developer.chrome.com/docs/devtools/network/reference/#initiator-and-dependency) |
| 38 | **WebSocket inspector (frames list, payload preview, per-direction filter, "Reconnect" button)** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools WebSocket](https://developer.chrome.com/docs/devtools/network/reference/#websocket) |
| 39 | **Server-Sent Events inspector (EventStream view, event parsing, retry, last-event-id)** | desktop | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | M | Chromium: yes/stable. Gecko: partial. WebKit: partial. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools SSE](https://developer.chrome.com/docs/devtools/network/reference/#eventstream) |
| 40 | **Request/response headers + body view (form-urlencoded / multipart / JSON pretty / hex / raw)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Headers](https://developer.chrome.com/docs/devtools/network/reference/#headers) |
| 41 | **Cookie inspection (name, value, domain, path, expires, sameSite, partitionKey, priority, size)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [RFC 6265](https://datatracker.ietf.org/doc/html/rfc6265) |
| 42 | **Cache inspection (Application → Cache Storage, browser cache list per origin, "Delete cache")** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN CacheStorage](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage) |
| 43 | **Mixed-content indicator + CORS preflight detail (request type `preflight`, `OPTIONS`, Access-Control-* response headers)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Fetch CORS protocol](https://fetch.spec.whatwg.org/#cors-protocol) |

### 5. DevTools performance features

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 44 | **Performance recording (start / stop, capture full main thread, screenshots, "Save profile" → JSON)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | XL | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Performance](https://developer.chrome.com/docs/devtools/performance/) |
| 45 | **Flame chart (CPU stack trace per frame, colour-coded by domain, zoom + pan, search)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Flame chart](https://developer.chrome.com/docs/devtools/performance/reference/#flame-chart) |
| 46 | **Main-thread breakdown (scripting / rendering / painting / system / idle time bars)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Summary](https://developer.chrome.com/docs/devtools/performance/reference/#summary) |
| 47 | **Long task markers (≥50ms tasks, "attribution" → culprit script), layout shift regions (CLS visualisation)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Long Tasks API](https://w3c.github.io/longtasks/) · [Layout Instability API](https://wicg.github.io/layout-instability/) |
| 48 | **Performance Insights (LCP / CLS / INP / TTFB callouts, suggestions, "Save insight")** | desktop | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | M | Chromium: yes/stable (2024+). Gecko: no. WebKit: no. Servo: no. Ladybird: no. Flow: unknown. | [Performance Insights](https://developer.chrome.com/docs/devtools/performance-insights/) |
| 49 | **Frame viewer (RAIL-aligned, drop frame heatmap, "Show frames per second")** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools FPS meter](https://developer.chrome.com/docs/devtools/performance/reference/#fps-meter) |
| 50 | **Memory timeline (JS heap, documents, nodes, listeners, GPU memory)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Memory](https://developer.chrome.com/docs/devtools/memory-problems/) |
| 51 | **Allocation profiling (sampling vs counting, "Heap allocation timeline", recording allocations on timeline)** | desktop | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [V8 allocation profiler](https://v8.dev/blog/v8-release-68) |

### 6. DevTools application features

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 52 | **Storage inspector (LocalStorage, SessionStorage, IndexedDB, CacheStorage, OPFS, Service Worker registrations, Interest Groups, Shared Storage, Cookies)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | XL | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN Storage](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API) · [Origin Private File System](https://fs.spec.whatwg.org/#origin-private-file-system) |
| 53 | **Database tools (deprecated WebSQL view, IndexedDB schema browser, OPFS file browser)** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [WebSQL spec (deprecated)](https://www.w3.org/TR/webdatabase/) |
| 54 | **Manifest viewer (Web App Manifest, "Installability" errors, "Add to home screen" actions, maskable icons)** | desktop + mobile | not-started | `>=90%` (Chromium family) | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [W3C Web App Manifest](https://www.w3.org/TR/appmanifest/) |
| 55 | **Background Services panel (Background Fetch, Background Sync, Notifications, Push Messaging, Payment Handler)** | desktop + mobile | not-started | `>=90%` (Chromium) / `75-90%` overall | P3 | M | Chromium: yes/stable. Gecko: partial (Background Sync yes, Background Fetch partial). WebKit: partial. Servo: no. Ladybird: no. Flow: unknown. | [Background Fetch](https://wicg.github.io/background-fetch/) · [MDN Background Sync](https://developer.mozilla.org/en-US/docs/Web/API/Background_Synchronization_API) |
| 56 | **Frame tree (top-level document, subframes, `srcdoc`, OOPIF boundary, opener)** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Frames](https://developer.chrome.com/docs/devtools/javascript/breakpoints/#frame) |
| 57 | **"Inspect" picker tool (click page to highlight node, jump to Elements panel)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Inspect](https://developer.chrome.com/docs/devtools/open/) |
| 58 | **Element screenshot + full-page screenshot + region screenshot** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable ("Screenshot" command + `Cmd+Shift+S`). WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome DevTools Screenshot](https://developer.chrome.com/docs/devtools/tips/#capture-screenshots) |

### 7. DevTools security features

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 59 | **Certificate viewer (CN, SAN, issuer, validity, key usage, signature algorithm, SCT for EV)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [RFC 5280](https://datatracker.ietf.org/doc/html/rfc5280) |
| 60 | **Mixed-content listing (insecure requests under secure context, block status, source URL)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN Mixed content](https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content) |
| 61 | **CSP violation list (Security panel → "Content Security Policy" section, blocked URIs, blocked inline, report-only)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [CSP Level 3](https://www.w3.org/TR/CSP3/) |
| 62 | **COOP / COEP / CORP / COA status (per-page isolation, cross-origin opener / embedder / resource policy)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [COOP](https://html.spec.whatwg.org/multipage/origin.html#cross-origin-opener-policies) · [COEP](https://html.spec.whatwg.org/multipage/origin.html#embedder-policy) |
| 63 | **Secure-context check ("This page is a secure context" indicator, restricted features list)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN Secure context](https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts) |
| 64 | **HTTPS-only enforcement indicator ("Upgrade requests to HTTPS" toggle, downgrade counter, broken-lock icon)** | desktop + mobile + embedded | not-started | `>=90%` (Chromium) / `75-90%` overall | P3 | S | Chromium: yes/stable (`chrome://settings/security` → "Always use secure connections"). Gecko: yes/stable (HTTPS-Only Mode). WebKit: yes/stable ("HTTPS Upgrade" via "Use HTTPS when possible"). Servo: no. Ladybird: no. Flow: unknown. | [Chrome security settings](https://support.google.com/chrome/answer/10468685) · [MDN Mixed content](https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content) |
### 8. DevTools accessibility features

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 65 | **Accessibility tree (full a11y node list, role, name, properties, parent/child)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [WAI-ARIA 1.2](https://www.w3.org/TR/wai-aria-1.2/) |
| 66 | **Contrast checker (text contrast ratio, "fix contrast" suggestion, "Lighthouse" report cross-link)** | desktop | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | S | Chromium: yes/stable. Gecko: no. WebKit: no. Servo: no. Ladybird: no. Flow: unknown. | [WCAG 2.2](https://www.w3.org/TR/WCAG22/) |
| 67 | **ARIA inspector (every element, ARIA attributes computed, warnings for invalid / conflicting ARIA)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [WAI-ARIA 1.2](https://www.w3.org/TR/wai-aria-1.2/) |

### 9. Remote debugging protocols

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 68 | **Chrome DevTools Protocol (CDP) — bi-directional JSON-RPC over WebSocket, HTTP discovery at `/.well-known/appspecific/com.chrome.devtools.json` on port, target discovery, multi-target** | desktop + mobile + embedded | not-started | `>=90%` (Chromium family: Chrome, Edge, Brave, Opera, Vivaldi, Arc, etc.) | P3 | XL | Chromium: yes/stable (CDP spec on `chromedevtools.github.io/devtools-protocol/`). Gecko: no (uses Firefox RDP). WebKit: no (uses WebKit Inspector Protocol). Servo: no. Ladybird: behind flag (own "Ladybird Inspector Protocol" draft 2025). Flow: unknown. | [CDP spec](https://chromedevtools.github.io/devtools-protocol/) |
| 69 | **WebKit Inspector Protocol — bi-directional JSON-RPC, `iOS Web Inspector` over USB, `Safari → Develop → Simulator` over network, `Web Inspector Service` daemon on macOS** | desktop + mobile + embedded | not-started | `>=90%` (WebKit family: Safari, all iOS browsers use WebKit) | P3 | XL | WebKit: yes/stable. Chromium: no. Gecko: no. Servo: no. Ladybird: no. Flow: unknown. | [WebKit Inspector Guide](https://webkit.org/web-inspector/) |
| 70 | **Firefox Remote Debugging Protocol (RDP / `devtools-remote` actor model, port 6000 default, `adb forward` for Android)** | desktop + mobile + embedded | not-started | `>=90%` (Gecko family: Firefox, LibreWolf, Tor Browser, Waterfox, Zen, etc.) | P3 | XL | Gecko: yes/stable. Chromium: no. WebKit: no. Servo: no. Ladybird: no. Flow: unknown. | [Firefox Remote Debugging](https://firefox-source-docs.mozilla.org/devtools-user/remote_debugging/index.html) |
| 71 | **USB + network target enumeration (`chrome://inspect/#devices`, "Port forwarding", "Discover network targets" toggle, `adb` integration)** | desktop | not-started | `>=90%` (Chromium family) | P3 | M | Chromium: yes/stable. Gecko: partial. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome USB debugging](https://developer.chrome.com/docs/devtools/remote-debugging/) |

### 10. View source + reader mode developer surface

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 72 | **View source (`view-source:https://...` URL, syntax highlighting, prettify, save as, print, line numbers, search)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable (no save-as; "View source" only). WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [WHATWG URL scheme `view-source:`](https://url.spec.whatwg.org/#view-source) |
| 73 | **Reader mode developer surface (`document.featurePolicy.allowsFeature('document-reader')`, `Readerable` SaveData heuristic, content scripts testing reader detection)** | desktop + mobile + embedded | not-started | `>=90%` (UI is chunk 7; this row is the developer hook) | P3 | M | Chromium: yes/stable (behind "Reader Mode triggering" flag in `chrome://flags`, exposed to extensions). Gecko: yes/stable (`reader.parse-on-load.enabled`, `about:reader` actor). WebKit: yes/stable (Reader mode API for content blockers). Servo: no. Ladybird: no. Flow: unknown. | [Reader Mode in Firefox](https://wiki.mozilla.org/Reader_View) |
| 74 | **Save page as (HTML only, HTML complete, single file MHTML, PDF, plain text) — developer surface is the "Save as" command handler** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [WHATWG MHTML](https://tools.ietf.org/html/draft-ietf-mhtml-mime-regs) |

### 11. Error pages

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 75 | **HTTP error pages (404, 403, 500, 502, 503, 504, network error) — renderable, themeable, localisable, with "Try again" / "Search suggestions" / "Cached copy"** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable (`net_error.cc`). Gecko: yes/stable (`browser.xhtml`, `about:neterror`). WebKit: yes/stable. Servo: partial. Ladybird: partial. Flow: unknown. | [Chromium net error](https://chromium.googlesource.com/chromium/src/+/main/components/net_error/) |
| 76 | **Connection / security error pages (`ERR_CERT_AUTHORITY_INVALID`, `ERR_CERT_DATE_INVALID`, `ERR_SSL_PROTOCOL_ERROR`, `ERR_CONNECTION_REFUSED`, `ERR_CONNECTION_TIMED_OUT`, `ERR_NAME_NOT_RESOLVED`, `ERR_INTERNET_DISCONNECTED`, `ERR_TOO_MANY_REDIRECTS`, `ERR_BLOCKED_BY_CLIENT`, `ERR_BLOCKED_BY_RESPONSE`, `ERR_NETWORK_CHANGED`, `SSL handshake failed`)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable (60+ error codes in `net_error_list.h`). Gecko: yes/stable. WebKit: yes/stable. Servo: partial. Ladybird: partial. Flow: unknown. | [Chromium net error list](https://chromium.googlesource.com/chromium/src/+/main/net/base/net_error_list.h) |
| 77 | **Safe-browsing / threat pages (malware, phishing, deceptive site / unwanted software, "you are about to login to a deceptive site" interstitial)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | L | Chromium: yes/stable (Google Safe Browsing). Gecko: yes/stable (Google Safe Browsing). WebKit: yes/stable (Google Safe Browsing). Servo: no (no integrated safe-browsing). Ladybird: no. Flow: unknown. | [Google Safe Browsing](https://safebrowsing.google.com/) |
| 78 | **Captive portal detection + interstitial (test against known good URL on portal detection, "Sign in" button)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Captive Portal detection](https://wicg.github.io/captive-portal/) |
| 79 | **No-Internet / offline page (with offline fallback to dino game / fox / no game)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | S | Chromium: yes/stable (dino game). Gecko: yes/stable (fox game). WebKit: yes/stable (no game, plain text). Servo: partial. Ladybird: partial. Flow: unknown. | [Chromium dino game](https://chromium.googlesource.com/chromium/src/+/main/components/net_error/resources/offline.html) |

### 12. Internal pages (`about:` / `chrome://` / `edge://` / `about:config`)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 80 | **Internal pages: `about:blank`, `about:newtab`, `about:settings`, `about:version`, `about:flags`, `about:labs`, `about:support`, `about:memory`, `about:gpu`, `about:tracing`, `about:net-export`, `about:processes`, `about:sandbox`, `about:performance`, `about:serviceworker`, `about:preferences#*`, `about:addons`, `about:debugging`, `chrome://`, `edge://`, `browser://` (per-engine) — the full internal settings surface, ~100+ distinct pages per engine** | desktop + mobile + embedded | partial (only `about:blank` wired; see `spiral-core/src/lib.rs:103`) | `>=90%` | P3 | XL | Chromium: yes/stable (`chrome://` namespace, 100+ pages). Gecko: yes/stable (`about:` namespace + `about:config` for prefs). WebKit: yes/stable (smaller `about:` set). Servo: partial. Ladybird: partial. Flow: unknown. | [Chromium chrome:// URLs](https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/ui/webui/chrome_web_ui_controller_factory.cc) |
| 81 | **Browser flags (`chrome://flags` / `about:config` / hidden preferences): experimental toggles behind an explicit "Enable" button, restart-required badge, per-platform availability, experimental-features disclaimer** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable (`chrome://flags`, `chrome://features`). Gecko: yes/stable (`about:config` and `about:preferences#general`). WebKit: yes/stable (Developer → Experimental Features). Servo: no. Ladybird: no. Flow: unknown. | [Chrome flags](https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/about_flags.cc) |
| 82 | **Web Platform Tests runner (WPT, hosted at `web-platform-tests.org`, runs from local binary, `--webdriver` / `--testdriver` mode, harness generates result files conforming to the WPT format)** | desktop + mobile + embedded | not-started (GAP §5.2: `tests/wpt/` exists, empty) | `>=90%` (every shipping engine uses WPT) | P3 | L | Chromium: yes/stable (in-tree at `//third_party/blink/web_tests/` + WPT). Gecko: yes/stable. WebKit: yes/stable. Servo: yes/stable. Ladybird: yes/stable. Flow: unknown. | [WPT](https://web-platform-tests.org/) · [GAP §5.2](/Users/james/spiral-research/specs/GAP_ANALYSIS.md) |

### 13. Headless mode

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 83 | **Headless mode: `--headless` flag, headless: new mode (Chromium's "new headless" since 2022, real Chrome binary minus UI), `HEADLESS=1` env, surface area equivalent to headed except for windowing, GPU, and rendering paths** | desktop + server (CI) | not-started | `>=90%` | P3 | M | Chromium: yes/stable (headless: new is the default since 128). Gecko: yes/stable (`-headless`). WebKit: yes/stable (`-headless` via WebDriver only). Servo: yes/stable. Ladybird: partial. Flow: unknown. | [Headless Chromium](https://developer.chrome.com/blog/headless-chromium/) · [Firefox headless](https://wiki.mozilla.org/Firefox/Headless_mode) |
| 84 | **Headless surface area: full DevTools, full WebDriver, full networking, no real GPU (SwiftShader fallback), no window, screenshot via DevTools/CDP, PDF generation via DevTools/CDP** | desktop + server (CI) | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: yes/stable. Ladybird: partial. Flow: unknown. | [Chrome headless flags](https://developer.chrome.com/docs/headless) |

### 14. Automation protocols

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 85 | **Automation protocols (umbrella): WebDriver (W3C REC), WebDriver BiDi (W3C draft, bi-directional JSON-RPC for storage / network / fetch interception), Chrome DevTools Protocol (Chromium-specific), Marionette (Gecko-specific), Playwright (cross-engine, built on top of WebDriver BiDi + CDP), Puppeteer (Chromium-first, now BiDi-capable), Selenium WebDriver (older) — one or more per engine** | desktop + server (CI) | not-started | `>=90%` | P3 | XL | Chromium: yes/stable (CDP + WebDriver + WebDriver BiDi). Gecko: yes/stable (Marionette + WebDriver + WebDriver BiDi). WebKit: yes/stable (WebDriver + WebDriver BiDi). Servo: yes/stable (WebDriver partial). Ladybird: partial. Flow: unknown. | [WebDriver REC](https://www.w3.org/TR/webdriver/) · [WebDriver BiDi](https://w3c.github.io/webdriver-bidi/) · [CDP](https://chromedevtools.github.io/devtools-protocol/) · [Marionette](https://firefox-source-docs.mozilla.org/testing/marionette/Intro.html) |

### 15. Built-in PDF

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 86 | **Built-in PDF viewer (navigation, zoom, rotate, print, save, find in PDF, text selection, fill form, sign — Chromium; annotate — Safari; thumbnails panel, two-page view, dark mode)** | desktop + mobile + embedded | not-started | `>=90%` (Chromium) / `75-90%` overall; `spiral-imagedecoder` is PDF-adjacent but not PDF | P3 | XL | Chromium: yes/stable (`chrome://pdf/`, PDFium engine, v23+). Gecko: yes/stable (`pdf.js` shipped in-tree since 2013). WebKit: yes/stable (`PDFKit`, annotate). Servo: no. Ladybird: no. Flow: unknown. | [PDFium](https://pdfium.googlesource.com/pdfium/) · [PDF.js](https://mozilla.github.io/pdf.js/) |
| 87 | **PDF generation (`Page.printToPDF` CDP, `window.print()` → "Save as PDF", custom size, custom margin, header/footer, page ranges)** | desktop + server (CI) | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [CDP Page domain](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF) |
### 16. RSS / Atom feed discovery

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 88 | **Feed auto-discovery (`<link rel="alternate" type="application/rss+xml">`, `<link rel="alternate" type="application/atom+xml">`, `<link rel="alternate" type="application/json" title="JSON Feed">`) — the browser surfaces a follow / subscribe affordance when detected** | desktop + mobile + embedded | not-started | `>=90%` (auto-discovery); `50-75%` (built-in reader UI) | P3 | M | Chromium: yes/stable (UI, no built-in reader since 2021 deprecation of Google Reader-like UI; feeds are surfaced to extensions). Gecko: yes/stable (Live Bookmarks removed in 2018; current state: feed icon → open feed in tab). WebKit: yes/stable (RSS icon in URL bar → open in tab). Servo: no. Ladybird: no. Flow: unknown. | [WHATWG HTML link type `alternate`](https://html.spec.whatwg.org/multipage/links.html#link-type-alternate) · [RSS 2.0](https://www.rssboard.org/rss-specification) · [Atom 1.0 RFC 4287](https://datatracker.ietf.org/doc/html/rfc4287) · [JSON Feed 1.1](https://jsonfeed.org/version/1.1) |
| 89 | **Feed reader UI: read articles inline, "Mark as read", "Save to read later", "Subscribe" hook to external service, OPML import/export, WebSub hub support** | desktop + mobile | not-started | `<25%` (built-in; many engines rely on extensions) | P3 | L | Chromium: no (UI removed 2021; relies on extension). Gecko: no (Live Bookmarks removed 2018; relies on extension). WebKit: no (no built-in; relies on extension). Servo: no. Ladybird: no. Flow: unknown. | [WebSub W3C REC](https://www.w3.org/TR/websub/) |

### 17. User scripts

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 90 | **User scripts: per-site JS / CSS small customisations loaded by URL pattern, distinct from full extensions, persistent across sessions, sync-able, sandboxed** | desktop | not-started | `75-90%` | P3 | M | Chromium: yes/stable (User Scripts API for Manifest V3 extensions). Gecko: yes/stable ("userChrome.css" + "userContent.css" + UserScript `@grant`-style via extensions). WebKit: yes/stable (Content Blocker / Content Script for app extensions). Servo: no. Ladybird: no. Flow: unknown. | [Chrome User Scripts API](https://developer.chrome.com/docs/extensions/reference/api/userScripts) |

### 18. Internal protocol handlers

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 91 | **Internal protocol handlers: `view-source:`, `devtools:`, `chrome-devtools:`, `about:`, `chrome:`, `edge:`, `browser:`, `javascript:` — registerable, navigable, security-checked (e.g. `javascript:` blocked from cross-origin click)** | desktop + mobile + embedded | partial (`about:blank` only) | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: partial. Ladybird: partial. Flow: unknown. | [WHATWG URL](https://url.spec.whatwg.org/) · [MDN view-source:](https://developer.mozilla.org/en-US/docs/Web/API/Window/open#javascript_urls) |
| 92 | **Inline data URLs (`data:text/html;base64,...`, `data:image/png;base64,...`, `data:application/javascript;base64,...`) and `blob:` URLs — the developer surface is the URL parser + media-type sniffing + base64 decoding** | desktop + mobile + embedded | partial (URL parser does these per `spiral-network`; `blob:` not yet) | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: yes/stable. Ladybird: yes/stable. Flow: unknown. | [RFC 2397 data URLs](https://datatracker.ietf.org/doc/html/rfc2397) · [WHATWG URL blob:](https://url.spec.whatwg.org/#concept-url-blob) |

### 19. Onboarding

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 93 | **Onboarding: telemetry opt-in dialog (first-run), default-browser check ("Set as default" suggestion), profile creation, sync sign-in, theme pick, "Import from another browser" flow** | desktop + mobile | not-started | `>=90%` | P3 | L | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. Spiral "telemetry: none" posture is by design (GAP §4.1). | [GAP §4.1](/Users/james/spiral-research/specs/GAP_ANALYSIS.md) |

### 20. Performance and memory (browser-level)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 94 | **Built-in task manager (browser-level process table: PID, frame, memory RSS, CPU%, network I/O, GPU memory, sandbox status, kill-process action)** | desktop + mobile | not-started | `>=90%` | P3 | M | Chromium: yes/stable (`chrome://discards`, `chrome://process-internals`, `Shift+Esc` legacy). Gecko: yes/stable (`about:processes` + `about:performance`). WebKit: yes/stable (Web Inspector → Timelines → Memory). Servo: no. Ladybird: no. Flow: unknown. | [Chrome task manager](https://support.google.com/chrome/answer/95664) |
| 95 | **Memory profiler (per-frame, per-origin breakdown, "memory saver" mode, "tab freezing" status)** | desktop | not-started | `>=90%` | P3 | L | Chromium: yes/stable (Memory Saver mode, tab throttling). Gecko: yes/stable (`about:memory` + `memory.free.resourcePolicy`). WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN Memory](https://developer.mozilla.org/en-US/docs/Web/API/Performance_Memory) |
| 96 | **Energy impact (battery / CPU efficiency per tab, "high-impact" badge, "energy saver" mode)** | desktop + mobile | not-started | `>=90%` (Chromium) / `50-75%` overall | P3 | M | Chromium: yes/stable (Task Manager → "Energy impact"). Gecko: yes/stable (about:performance, "Energy" column). WebKit: yes/stable (Web Inspector → Timelines). Servo: no. Ladybird: no. Flow: unknown. | [Chrome energy saver](https://support.google.com/chrome/answer/12978370) |
| 97 | **Network usage (per-frame bytes sent / received, "data saver" mode, "preload pages" toggle)** | desktop + mobile | not-started | `>=90%` | P3 | S | Chromium: yes/stable (Task Manager → "Network"). Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome data saver](https://support.google.com/chrome/answer/2392284) |

### 21. Diagnostic dumps

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 98 | **net-export (full network log dump: URL, headers, body, timings, cookies, sockets, TLS handshake bytes, in JSON or HAR 1.2)** | desktop + mobile | not-started | `>=90%` | P3 | M | Chromium: yes/stable (`chrome://net-export/`). Gecko: yes/stable (`about:networking` → save log). WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chromium net-export](https://chromium.googlesource.com/chromium/src/+/main/components/captive_portal/) |
| 99 | **Tracing (`chrome://tracing` / `about:tracing`, JSON output compatible with Perfetto / catapult, includes all processes, GPU command buffer, IPC, compositor, main thread, V8 trace events)** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: partial (`about:telemetry`, `Gecko Profiler`). WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chromium tracing](https://chromium.googlesource.com/catapult/) |
| 100 | **Heap dumps (`chrome://heapdump`, save a snapshot of the renderer heap to disk, inspectable with DevTools "Load heap snapshot" + `heapsnapshot` format)** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [V8 heap snapshot format](https://v8.dev/blog/heap-snapshots) |
| 101 | **Profile dumps (Gecko Profiler / V8 prof / Speedscope JSON / Linux perf data export, sampled-call-stack per process)** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable (V8 prof + Speedscope export). Gecko: yes/stable (Gecko Profiler). WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Gecko Profiler](https://profiler.firefox.com/) |
| 102 | **GPU dumps (Vulkan / Metal / DX12 capture via RenderDoc / ANGLE / About GPU, render-pass / command-buffer traces for debugging)** | desktop | not-started | `>=90%` | P3 | L | Chromium: yes/stable (RenderDoc integration, `chrome://gpu`). Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [RenderDoc](https://renderdoc.org/) |

### 22. Compatibility view

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 103 | **Compatibility view (enterprise mode): IE emulation (Trident document mode), `X-UA-Compatible` meta / header, per-site compatibility list, group policy override** | desktop | not-started | `25-50%` (Edge-only; Trident emulation is EOL June 2022 in IE 11) | P3 | L | Chromium (Edge): yes/stable (IE Mode in Edge for Business, EOL announced 2029). Gecko: no. WebKit: no. Servo: no. Ladybird: no. Flow: unknown. | [Edge IE Mode](https://learn.microsoft.com/en-us/deployedge/edge-ie-mode) |

### 23. Network condition simulation

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 104 | **CPU throttle (`chrome://flags` → "CPU throttling multiplier", or DevTools Performance → "4× slowdown", measured against the test host)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome CPU throttle](https://developer.chrome.com/docs/devtools/performance/reference/#cpu-throttling) |
| 105 | **Network throttle (latency / bandwidth / offline / custom profile, saveable throttle preset, per-tab throttle override, Slow 3G / Fast 3G / Offline presets)** | desktop + mobile | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [Chrome throttling](https://developer.chrome.com/docs/devtools/network/reference/#throttling) |
| 106 | **Sensor override (geolocation lat/lon, accelerometer x/y/z, gyroscope alpha/beta/gamma, device orientation, ambient light level, force touch pressure)** | desktop | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN Sensor APIs](https://developer.mozilla.org/en-US/docs/Web/API/Sensor_APIs) |

### 24. External application handler

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 107 | **Custom protocol / scheme handlers (`web+myapp:` namespace, `navigator.registerProtocolHandler`, OS-level handoff for `mailto:`, `webcal:`, `web+myapp:`, `intent://` on Android)** | desktop + mobile + embedded | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN registerProtocolHandler](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler) · [Web Intents](https://www.w3.org/TR/web-intents/) (historical) |
| 108 | **Custom file extensions (`navigator.registerProtocolHandler('web+myapp', 'https://app.example/?url=%s', 'My App')` for `web+` schemes, and the Manifest `file_handlers` member for PWA installable handlers)** | desktop | not-started | `75-90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: partial. Servo: no. Ladybird: no. Flow: unknown. | [W3C Web App Manifest file_handlers](https://www.w3.org/TR/appmanifest/#file_handlers-member) |
| 109 | **`mailto:` / `tel:` / `sms:` handler (open default OS mail client / dialer / messenger, customise from settings)** | desktop + mobile | not-started | `>=90%` | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [RFC 6068 mailto](https://datatracker.ietf.org/doc/html/rfc6068) · [RFC 3966 tel](https://datatracker.ietf.org/doc/html/rfc3966) |

### 25. Local file access

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 110 | **`file://` protocol (navigate to local HTML, render local file tree, open `.html`/`.htm` from the OS, drag-drop into browser)** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: partial. Ladybird: yes/stable. Flow: unknown. Spiral `spiral-fmt` covers parsing, but no file:// scheme handler yet. | [WHATWG File URL scheme](https://url.spec.whatwg.org/#file-urls) |
| 111 | **`file://` directory listing (render folder as directory index, allow navigation into sub-folders)** | desktop | not-started | `>=90%` (with `--allow-file-access-from-files` flag) | P3 | S | Chromium: yes/stable (toggle). Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: yes/stable. Flow: unknown. | [Chromium file access flag](https://chromium.googlesource.com/chromium/src/+/main/docs/user_data_dir.md) |
| 112 | **Mixed content: can `file://` fetch `https://`? (Default: no; requires `--allow-file-access-from-files` and `--disable-web-security`; affects `fetch`/XHR only, not `<img>`/`<script>` with crossorigin attribute)** | desktop | not-started | `>=90%` (with flag) | P3 | S | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN CORS errors](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS/Errors) |
| 113 | **`file://` security restrictions (no `localStorage` / `IndexedDB` for `file://` origins in most engines, treated as opaque origin)** | desktop | not-started | `>=90%` | P3 | M | Chromium: yes/stable. Gecko: yes/stable. WebKit: yes/stable. Servo: no. Ladybird: no. Flow: unknown. | [MDN Same-origin policy](https://developer.mozilla.org/en-US/docs/Web/Security/Same-origin_policy) |

---

## Open questions for the user

These surfaced while writing this chunk and need a decision before
chunk 12 (the competitive matrix) is built. They are captured here per
the contract; the inventory above is not blocked on them.

1. **DevTools scope for Spiral.** The methodology marks DevTools as
   `v0.1-blocker` or `1.0-blocker`, but doesn't pick. The lowest
   surface that any self-respecting browser ships in 2026 is (a) an
   element inspector, (b) a console, (c) a network panel. Everything
   else (heap profiler, performance insights, recorder, etc.) is
   incremental. For Spiral, do we commit to (a)+(b)+(c) as a v0.1
   minimum, or is it a v1.0 concern (in which case rows 1–67 are
   deferred in their entirety)?

2. **Headless mode ownership.** `--headless` is what every CI
   harness expects. Spiral's current `spiral-render/src/software.rs:8`
   says "neither of which exist in headless tests" — so headless is
   referenced, not implemented. Is headless mode a Phase 2 deliverable
   for `spiral-browser` (rows 83-84, 85) or Phase 4?

3. **WPT scope.** WPT is a 70k-test corpus; the GAP file (§5.2) has
   `tests/wpt/` as an empty directory. Are we targeting WPT for
   `spiral-fmt` only (parser conformance, M4.4 deliverable), or for
   `spiral-gyre`, `spiral-network`, `spiral-vortex` as well? The
   decision changes the Phase impact on row 82 from M5+ to M18+.

4. **PDF: real or punt?** `spiral-imagedecoder` exists but is for
   images, not PDF. Two distinct paths: (a) embed PDFium or pdf.js as
   a viewer (row 86, complexity XL), or (b) punt to "open with the
   system PDF reader" via a `web+pdf` external app handler (row 107,
   complexity M). The latter is dramatically cheaper but is a
   competitive deficit against every shipping engine that ships a
   built-in viewer.

5. **Reader mode.** Reader mode UI is in chunk 7. Row 73 is the
   *developer surface* (the hook for extensions / content scripts to
   test reader detection). If chunk 7 ships reader mode as a
   chunk-7-row, do we still need a separate row 73 here, or merge?

6. **Flow engine.** Methodology §1 lists Flow as a sixth engine to
   score. Flow is in alpha (announced 2024, Chromium-based but
   AI-rewritten). The "Engine notes" column says "unknown" on
   almost every row. Do we re-verify Flow in chunk 12 (the matrix
   chunk) once it ships a stable release, or score it as `n/a` in
   this chunk?

7. **Ladybird "behind flag" treatment.** Ladybird's `--devtools` flag
   is real (in the Ladybird source, 2024) but not in a stable
   release yet. Methodology §4 says "behind flag" is a valid bucket —
   that is what row 1 uses. Confirm the bucketing is right, or should
   "in development, not shipped" be a separate bucket?

8. **Telemetry in onboarding (row 93).** Spiral's posture
   (GAP §4.1) is "telemetry: none by default". The row currently
   scores prevalence `>=90%` and engine notes flag the discrepancy.
   Should the row be split into "telemetry opt-in dialog" (prevalence
   `>=90%`, not-started in Spiral) and "telemetry: none by default"
   (prevalence `<25%`, designed in Spiral, links to ADR 0001)?

---

## Sources

Per `00-methodology.md` §1 every row above is grounded in at least
one Tier-1 to Tier-3 source. Master list in
`/Users/james/spiral-research/docs/research/citations/sources.md`.

| Tier | Source | Used for |
|------|--------|----------|
| 1 | WHATWG URL spec (`view-source:`, `file:`, `data:`, `blob:`) | rows 72, 91, 92, 110, 111 |
| 1 | WHATWG HTML spec (`<link rel="alternate">`) | row 88 |
| 1 | WHATWG Console API spec | rows 6, 18, 20 |
| 1 | WHATWG Fetch spec (CORS preflight) | row 43 |
| 1 | WHATWG File System spec (OPFS) | row 52 |
| 1 | W3C WebDriver REC | row 85 |
| 1 | W3C WebDriver BiDi | row 85 |
| 1 | W3C Web App Manifest | rows 54, 108 |
| 1 | W3C CSP Level 3 | row 61 |
| 1 | W3C COOP / COEP (HTML spec) | row 62 |
| 1 | W3C Long Tasks API | row 47 |
| 1 | W3C WAI-ARIA 1.2 | rows 65, 67 |
| 1 | W3C WCAG 2.2 | row 66 |
| 1 | W3C WebSub REC | row 89 |
| 1 | W3C Background Fetch / Sync | row 55 |
| 1 | IETF RFC 5280 (certificates) | row 59 |
| 1 | IETF RFC 6265 (cookies) | row 41 |
| 1 | IETF RFC 2397 (data URLs) | row 92 |
| 1 | IETF RFC 4287 (Atom) | row 88 |
| 1 | IETF RFC 6068 (mailto) | row 109 |
| 1 | IETF RFC 3966 (tel) | row 109 |
| 1 | IETF draft MHTML | row 74 |
| 1 | IETF HAR 1.2 spec | rows 35, 98 |
| 1 | TC39 Top-level await | row 19 |
| 1 | WICG Captive Portal | row 78 |
| 1 | WICG Layout Instability | row 47 |
| 1 | WICG Web Intents (historical) | row 107 |
| 1 | Source map v3 spec | row 23 |
| 2 | MDN — DevTools docs | rows 1-67, 71 |
| 2 | MDN — Web API docs | rows 6, 20, 41, 42, 52, 60, 62, 64, 91, 95, 107, 112, 113 |
| 2 | MDN — WebDriver / automation | row 85 |
| 2 | MDN — `registerProtocolHandler` | row 107 |
| 2 | Chrome for Developers (`developer.chrome.com`) | rows 1, 5, 10, 11, 12, 13, 14, 15, 16, 17, 22, 24, 25, 31, 32, 33, 34, 36, 37, 38, 39, 40, 44, 45, 46, 48, 49, 50, 57, 58, 64, 71, 83, 87, 90, 104, 105 |
| 2 | Chromium source (`chromium.googlesource.com`) | rows 75, 76, 79, 80, 81, 98, 99 |
| 2 | Chrome support pages (`support.google.com`) | rows 64, 94, 96, 97 |
| 2 | Microsoft Edge IE Mode docs | row 103 |
| 2 | Mozilla Developer docs (MDN, Firefox Source Docs) | rows 27, 28, 70, 85, 86 |
| 2 | Mozilla Wiki (Firefox Headless, Reader Mode) | rows 73, 83 |
| 2 | WebKit project page (Web Inspector) | row 69 |
| 2 | Google Safe Browsing | row 77 |
| 2 | Lighthouse docs | row 17 |
| 2 | pdfium / pdf.js project pages | row 86 |
| 2 | web-platform-tests.org | row 82 |
| 2 | v8.dev blog (heap snapshots, async stack, allocation profiler) | rows 26, 50, 51, 100 |
| 2 | Catapult (Chromium tracing) | row 99 |
| 2 | profiler.firefox.com | row 101 |
| 2 | renderdoc.org | row 102 |
| 2 | JSON Feed 1.1 spec | row 88 |
| 2 | RSS 2.0 spec | row 88 |
| 3 | Spiral `specs/GAP_ANALYSIS.md` (status of `tests/wpt/`, DevTools row, telemetry posture) | rows 82, 93, plus "Status in Spiral" baseline for every row |
| 3 | Spiral `crates/spiral-core/src/lib.rs:103` (only `about:blank` wired) | row 80, 91 |
| 3 | Spiral `crates/spiral-render/src/software.rs:8` (headless surface referenced but absent) | rows 83, 84 |

