# Competitive Matrix — Domain: Developer / Power-User Surface

**File:** `02-competitive-matrix-developer.md`
**Date:** 2026-06-16
**Source:** `08-developer-experience.md`
**Methodology:** `00-methodology.md`

## Column legend

- **Status in Spiral:** `shipped` / `partial` / `designed` / `not-started` / `do-not-touch`
- **Prevalence:** `ubiquitous` (>95%) / `widespread` (70–95%) / `mixed` (two+ engines, at least one no) / `niche` (one engine) / `experimental` (flag-only) / `legacy` (deprecated)
- **Phase:** per `00-methodology.md` §5
- **Complexity:** `S` / `M` / `L` / `XL`
- **Engine columns:** `yes` / `partial` / `no` / `behind-flag`

---

## §1 DevTools panels

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 1 | Element Inspector (DOM tree, attributes, box model overlay) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | partial | behind-flag |
| 2 | Styles / Computed / Cascade panel | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 3 | Layout / Box Model panel | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 4 | Network panel (request list, headers, timing) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 5 | Sources / Debugger panel (breakpoints, call stack, scope, watch) | desktop+mobile+embedded | not-started | ubiquitous | P3 | XL | yes | yes | yes | no | no |
| 6 | Console (REPL, log levels, object previews, filtering) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | no |
| 7 | Memory / Heap profiler (snapshots, allocation timeline) | desktop+mobile+embedded | not-started | ubiquitous | P3 | XL | yes | yes | yes | no | no |
| 8 | Performance panel (flame chart, main-thread, long tasks, layout shifts) | desktop+mobile+embedded | not-started | ubiquitous | P3 | XL | yes | yes | yes | no | no |
| 9 | Application panel (Storage, SW, IndexedDB, OPFS, Cookies) | desktop+mobile+embedded | not-started | ubiquitous | P3 | XL | yes | yes | yes | no | no |
| 10 | Security panel (cert viewer, TLS, mixed content, CSP, COOP/COEP) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 11 | Recorder (record/replay interactions, export traces) | desktop | not-started | niche | P3 | L | yes | no | no | no | no |
| 12 | Animations panel (scrub CSS/Web Animations, slow-down) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 13 | Coverage panel (per-file used-vs-unused CSS/JS bytes) | desktop | not-started | mixed | P3 | M | yes | no | no | no | no |
| 14 | Sensors panel (geolocation, accelerometer, gyroscope override) | desktop | not-started | widespread | P3 | S | yes | yes | yes | no | no |
| 15 | Network conditions (throttle, UA override, content-encoding) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 16 | Rendering panel (force colours, paint flashing, layer borders) | desktop | not-started | ubiquitous | P3 | S | yes | partial | yes | no | no |
| 17 | Audits / Lighthouse (perf/a11y/SEO/PWA, JSON export) | desktop | not-started | niche | P3 | L | yes | no | no | no | no |

## §2 DevTools console features

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 18 | Console REPL (autocomplete, multi-line, history) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | no |
| 19 | Top-level await in REPL | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 20 | Command-line API ($, $$, $0–$4, dir, copy, clear, table) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 21 | Inline object previews (lazy-expandable trees) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 22 | Eager evaluation (live result while typing) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |

## §3 DevTools source features

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 23 | Source maps (v3 + extensions) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 24 | Breakpoint types (line, conditional, logpoint, XHR, DOM mutation, exception) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 25 | Watch expressions, scope inspection, call stack | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 26 | Async stack traces (full chain across await/Promise) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 27 | Blackboxing (skip frames from file/pattern) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 28 | Snippets (save JS, run in any frame) | desktop | not-started | mixed | P3 | S | yes | yes | yes | no | no |
| 29 | File-system access (workspace folder, showDirectoryPicker) | desktop | not-started | mixed | P3 | M | yes | partial | partial | no | no |
| 30 | Local overrides (workspace mapping, save changes to disk) | desktop | not-started | mixed | P3 | L | yes | yes | partial | no | no |
| 31 | Exception breakpoints (pause on caught/uncaught) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 32 | Inline value preview on hover | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 33 | Pretty-print minified sources | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |

## §4 DevTools network features

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 34 | Network throttling (latency, bandwidth, offline presets) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 35 | HAR export (request, response, timings, content) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 36 | Request blocking (pattern-based, persisted) | desktop | not-started | mixed | P3 | S | yes | yes | yes | no | no |
| 37 | Request initiator chain (Parser, Script, Other, stack trace) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 38 | WebSocket inspector (frames, payload, per-direction) | desktop | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 39 | Server-Sent Events inspector | desktop | not-started | mixed | P3 | M | yes | partial | partial | no | no |
| 40 | Request/response headers + body view | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 41 | Cookie inspection (name, domain, sameSite, partitionKey) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 42 | Cache inspection (CacheStorage, browser cache per origin) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 43 | Mixed-content indicator + CORS preflight detail | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |

## §5 DevTools performance features

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 44 | Performance recording (start/stop, full main thread, screenshots) | desktop+mobile+embedded | not-started | ubiquitous | P3 | XL | yes | yes | yes | no | no |
| 45 | Flame chart (CPU stack trace per frame, zoom/pan) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 46 | Main-thread breakdown (scripting/rendering/painting/idle) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 47 | Long task markers (≥50ms), layout shift regions (CLS) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 48 | Performance Insights (LCP/CLS/INP/TTFB callouts) | desktop | not-started | mixed | P3 | M | yes | no | no | no | no |
| 49 | Frame viewer (RAIL-aligned, drop frame heatmap, FPS meter) | desktop | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 50 | Memory timeline (JS heap, documents, nodes, GPU) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 51 | Allocation profiling (sampling, heap allocation timeline) | desktop | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |

## §6 DevTools application features

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 52 | Storage inspector (LocalStorage, IndexedDB, OPFS, SW, Cookies) | desktop+mobile+embedded | not-started | ubiquitous | P3 | XL | yes | yes | yes | no | no |
| 53 | Database tools (IndexedDB schema, OPFS file browser) | desktop | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 54 | Manifest viewer (Web App Manifest, installability errors) | desktop+mobile | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 55 | Background Services panel (Background Fetch/Sync, Push) | desktop+mobile | not-started | widespread | P3 | M | yes | partial | partial | no | no |
| 56 | Frame tree (top-level, subframes, OOPIF boundary) | desktop | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 57 | "Inspect" picker tool (click to highlight node) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 58 | Element screenshot + full-page screenshot | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |

## §7 DevTools security features

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 59 | Certificate viewer (CN, SAN, issuer, validity, key usage) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 60 | Mixed-content listing (insecure requests under HTTPS) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 61 | CSP violation list (blocked URIs, report-only) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 62 | COOP / COEP / CORP status (per-page isolation) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 63 | Secure-context check indicator | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 64 | HTTPS-only enforcement indicator | desktop+mobile+embedded | not-started | widespread | P3 | S | yes | yes | yes | no | no |

## §8 DevTools accessibility features

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 65 | Accessibility tree (role, name, properties, parent/child) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 66 | Contrast checker (text contrast ratio, fix suggestion) | desktop | not-started | mixed | P3 | S | yes | no | no | no | no |
| 67 | ARIA inspector (ARIA attributes, warnings for invalid) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |

## §9 Remote debugging protocols

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 68 | Chrome DevTools Protocol (CDP) — bi-directional JSON-RPC | desktop+mobile+embedded | not-started | niche | P3 | XL | yes | no | no | no | behind-flag |
| 69 | WebKit Inspector Protocol — bi-directional JSON-RPC, USB | desktop+mobile+embedded | not-started | niche | P3 | XL | no | no | yes | no | no |
| 70 | Firefox Remote Debugging Protocol (RDP, actor model) | desktop+mobile+embedded | not-started | niche | P3 | XL | no | yes | no | no | no |
| 71 | USB + network target enumeration, adb integration | desktop | not-started | widespread | P3 | M | yes | partial | yes | no | no |

## §10 View source + reader mode developer surface

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 72 | View source (syntax highlighting, prettify, line numbers) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 73 | Reader mode developer surface (detection hooks for extensions) | desktop+mobile+embedded | not-started | widespread | P3 | M | behind-flag | yes | yes | no | no |
| 74 | Save page as (HTML, complete, MHTML, PDF, plain text) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |

## §11 Error pages

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 75 | HTTP error pages (404, 403, 500, 502, 503, network error) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | partial |
| 76 | Connection/security error pages (CERT_INVALID, SSL, TIMEOUT, etc.) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | partial |
| 77 | Safe-browsing / threat pages (malware, phishing interstitial) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 78 | Captive portal detection + interstitial | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 79 | No-Internet / offline page | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | partial |

## §12 Internal pages (about: / chrome://)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 80 | Internal pages (~100+ pages: settings, version, flags, support, memory, GPU, etc.) | desktop+mobile+embedded | partial | ubiquitous | P3 | XL | yes | yes | yes | partial | partial |
| 81 | Browser flags (experimental toggles, restart-required, per-platform) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 82 | Web Platform Tests runner (WPT, local binary, result format) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes |

## §13 Headless mode

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 83 | Headless mode (--headless, HEADLESS=1, headless: new) | desktop+server | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | partial |
| 84 | Headless surface area (full DevTools, WebDriver, networking, no GPU) | desktop+server | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | partial |

## §14 Automation protocols

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 85 | Automation protocols umbrella (WebDriver, WebDriver BiDi, CDP, Marionette, Playwright, Puppeteer) | desktop+server | not-started | ubiquitous | P3 | XL | yes | yes | yes | yes | partial |

## §15 Built-in PDF

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 86 | Built-in PDF viewer (navigation, zoom, print, form fill, sign) | desktop+mobile+embedded | not-started | widespread | P3 | XL | yes | yes | yes | no | no |
| 87 | PDF generation (printToPDF, window.print, custom size/margin) | desktop+server | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |

## §16 RSS / Atom feed discovery

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 88 | Feed auto-discovery (`<link rel="alternate">` RSS/Atom/JSON) | desktop+mobile+embedded | not-started | mixed | P3 | M | yes | yes | yes | no | no |
| 89 | Feed reader UI (inline read, mark read, OPML, WebSub) | desktop+mobile | not-started | experimental | P3 | L | no | no | no | no | no |

## §17 User scripts

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 90 | User scripts (per-site JS/CSS, URL pattern, sync, sandboxed) | desktop | not-started | widespread | P3 | M | yes | yes | yes | no | no |

## §18 Internal protocol handlers

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 91 | Internal protocol handlers (view-source:, devtools:, about:, javascript:) | desktop+mobile+embedded | partial | ubiquitous | P3 | S | yes | yes | yes | partial | partial |
| 92 | Inline data URLs (data:, blob:) — URL parser + media-type sniffing | desktop+mobile+embedded | partial | ubiquitous | P3 | S | yes | yes | yes | yes | yes |

## §19 Onboarding

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 93 | Onboarding (telemetry opt-in, default-browser, profile import, theme) | desktop+mobile | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |

## §20 Performance and memory (browser-level)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 94 | Built-in task manager (PID, memory, CPU%, network, GPU) | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 95 | Memory profiler (per-frame, per-origin, memory saver mode) | desktop | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 96 | Energy impact (battery/CPU per tab, high-impact badge, energy saver) | desktop+mobile | not-started | mixed | P3 | M | yes | yes | yes | no | no |
| 97 | Network usage (per-frame bytes, data saver, preload toggle) | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |

## §21 Diagnostic dumps

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 98 | net-export (full network log, HAR 1.2, JSON) | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 99 | tracing (Perfetto/catapult JSON, all processes, GPU, IPC) | desktop | not-started | ubiquitous | P3 | M | yes | partial | yes | no | no |
| 100 | Heap dumps (renderer heap snapshot, inspectable) | desktop | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 101 | Profile dumps (V8 prof / Speedscope / Gecko Profiler export) | desktop | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 102 | GPU dumps (Vulkan/Metal/DX12 capture, render-pass traces) | desktop | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |

## §22 Compatibility view

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 103 | Compatibility view / IE emulation (enterprise mode, X-UA-Compatible) | desktop | not-started | niche | P3 | L | yes | no | no | no | no |

## §23 Network condition simulation

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 104 | CPU throttle (multiplier, measured against host) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 105 | Network throttle (latency/bandwidth/offline, custom, presets) | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 106 | Sensor override (geolocation, accelerometer, gyroscope, ambient light) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |

## §24 External application handler

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 107 | Custom protocol/scheme handlers (web+myapp:, registerProtocolHandler) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 108 | Custom file extensions (file_handlers manifest member) | desktop | not-started | widespread | P3 | M | yes | yes | partial | no | no |
| 109 | mailto: / tel: / sms: handler (default OS client) | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |

## §25 Local file access

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 110 | file:// protocol (navigate local HTML, open from OS, drag-drop) | desktop | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | yes |
| 111 | file:// directory listing (render folder as index) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | yes |
| 112 | Mixed content: can file:// fetch https://? (flag-gated) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 113 | file:// security restrictions (no localStorage/IndexedDB) | desktop | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |

---

**Total rows: 113**
