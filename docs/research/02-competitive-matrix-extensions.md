# Competitive Matrix — Domain: Extensions & Customisation

**File:** `02-competitive-matrix-extensions.md`
**Date:** 2026-06-16
**Sources:** `11-extension-surface.md`, `11-customisation.md`
**Methodology:** `00-methodology.md`

## Column legend

- **Status in Spiral:** `shipped` / `partial` / `designed` / `not-started` / `do-not-touch`
- **Prevalence:** `ubiquitous` (>95%) / `widespread` (70–95%) / `mixed` (two+ engines, at least one no) / `niche` (one engine) / `experimental` (flag-only) / `legacy` (deprecated)
- **Phase:** per `00-methodology.md` §5
- **Complexity:** `S` / `M` / `L` / `XL`
- **Engine columns:** `yes` / `partial` / `no` / `behind-flag`

---

## §1 Manifest and packaging

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 1 | Manifest V3 (service_worker, action, host_permissions, declarative_net_request, web_accessible_resources) | desktop+mobile+embedded | not-started | widespread | P6 / 1.0-blocker | XL | yes | yes | partial | no | no |
| 2 | Manifest V2 legacy (background.scripts, webRequest blocking, browser_action) | desktop+mobile | not-started | legacy | do-not-touch | L | yes | yes | yes | no | no |
| 3 | Static and dynamic declarativeNetRequest rules (rules.json, updateDynamicRules, session rules) | desktop+mobile+embedded | not-started | widespread | P6 | L | yes | partial | no | no | no |
| 4 | Manifest version negotiation + browser_specific_settings (gecko id, edge id, safari bundle id) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | no | no |
| 5 | Web-accessible resources (matches + extension_ids + use_dynamic_url) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 6 | Externally connectable (ids + matches, runtime.connect / sendMessage) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | no | no |
| 7 | Extension content security policy (script-src 'self', no remote code) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |

## §2 Content scripts

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 8 | Static content_scripts (matches, js, css, run_at, all_frames, world) | desktop+mobile+embedded | not-started | ubiquitous | P6 | XL | yes | yes | yes | no | no |
| 9 | run_at timing (document_start, document_end, document_idle) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 10 | Dynamic content scripts (scripting.registerContentScripts, tabs.executeScript) | desktop+mobile+embedded | not-started | widespread | P6 | L | yes | yes | yes | no | no |
| 11 | Isolated world / main world split (world: "ISOLATED" / "MAIN") | desktop+mobile+embedded | not-started | widespread | P6 | L | yes | yes | partial | no | no |

## §3 Background contexts

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 12 | MV3 service-worker background (onInstalled, onMessage, alarms keep-alive) | desktop+mobile+embedded | not-started | widespread | P6 | XL | yes | yes | no | no | no |
| 13 | MV2 background page (persistent: true/false event page) | desktop+mobile | not-started | legacy | do-not-touch | L | yes | yes | yes | no | no |
| 14 | Offscreen document (offscreen.createDocument for media, clipboard, audio) | desktop+mobile | not-started | mixed | P6 | M | yes | no | no | no | no |

## §4 Core API namespaces

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 15 | tabs API (query, create, update, remove, onUpdated, captureVisibleTab, group) | desktop+mobile+embedded | not-started | ubiquitous | P6 | XL | yes | yes | partial | no | no |
| 16 | tabGroups API (query, update, colour, collapsed, title) | desktop | not-started | mixed | P6 | L | yes | no | no | no | no |
| 17 | windows API (getCurrent, create, update, remove, onFocusChanged) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 18 | storage API (local, sync, managed, session, onChanged) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no |
| 19 | cookies API (get, set, getAll, remove, getAllCookieStores, partition key) | desktop+mobile | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no |
| 20 | webRequest API (MV2 only — blocking, onBeforeRequest, cancel/redirect/modify) | desktop+mobile+embedded | not-started | legacy | do-not-touch | L | yes | yes | no | no | no |
| 21 | declarativeNetRequest runtime API (getDynamicRules, getMatchedRules, limits) | desktop+mobile+embedded | not-started | widespread | P6 | L | yes | yes | no | no | no |
| 22 | webNavigation API (onBeforeNavigate, onCommitted, onCompleted, getAllFrames) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | no | no | no |
| 23 | history API (search, getVisits, addUrl, deleteUrl, onVisited) | desktop+mobile | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 24 | bookmarks API (getTree, create, move, remove, search) | desktop+mobile | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 25 | downloads API (search, download, cancel, pause, resume, onDeterminingFilename) | desktop+mobile | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 26 | contextMenus API (create, update, remove, onClicked, contexts) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 27 | notifications API (create, update, clear, onClicked) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | no | no |
| 28 | runtime API (sendMessage, connect, onMessage, onInstalled, getURL, getManifest) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no |

## §5 UI surfaces (popup, action, omnibox, side panel, devtools, options, sandbox)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 29 | action API (unified browser_action / page_action, default_popup, setBadge, setTitle) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | partial | no | no |
| 30 | sidePanel API (setOptions, open, setPanelBehavior) + sidebarAction (Firefox) | desktop+mobile | not-started | widespread | P6 | L | yes | yes | no | no | no |
| 31 | omnibox API (keyword, description, onInputChanged, onInputEntered) | desktop | not-started | widespread | P6 | M | yes | yes | yes | no | no |
| 32 | devtools_page (extension panels, devtools.panels.create, inspectedWindow.eval) | desktop | not-started | mixed | P6 | M | yes | partial | no | no | no |
| 33 | options_ui (options_ui.page, open_in_tab) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no |
| 34 | sandbox page (sandbox.pages, opaque origin, CSP sandbox) | desktop+mobile | not-started | niche | P6 | M | yes | no | no | no | no |
| 35 | chrome_url_overrides (new tab, bookmarks, history override) | desktop | not-started | mixed | P6 | M | yes | yes | no | no | no |
| 36 | commands API (keyboard shortcut bindings, _execute_browser_action, custom shortcuts) | desktop | not-started | widespread | P6 | M | yes | yes | partial | no | no |

## §6 Network modification and content filtering

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 37 | proxy API (proxy.settings, mode, PAC script, per_host) | desktop+mobile | not-started | mixed | P6 | M | yes | yes | no | no | no |
| 38 | contentScripts runtime API (register, unregister — Firefox extension) | desktop+mobile | not-started | niche | P6 | M | no | yes | no | no | no |
| 39 | userScripts API (register, configureWorld, USER_SCRIPT world) | desktop+mobile | not-started | widespread | P6 | L | yes | yes | yes | no | no |

## §7 Identity, i18n, native messaging, enterprise

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 40 | identity API (getAuthToken, launchWebAuthFlow, getRedirectURL) | desktop+mobile+embedded | not-started | mixed | P6 | M | yes | partial | no | no | no |
| 41 | i18n API (getMessage, getUILanguage, getAcceptLanguages, locales/ directory) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 42 | nativeMessaging API (sendNativeMessage, connectNative, stdin/stdout JSON) | desktop | not-started | mixed | P6 | L | yes | yes | yes | no | no |
| 43 | Enterprise / managed extensions (ExtensionSettings, forcelist, blocklist, managed storage) | desktop+mobile | not-started | mixed | P6 | M | yes | yes | no | no | no |
| 44 | management API (getAll, getSelf, setEnabled, uninstall, onInstalled) | desktop+mobile | not-started | widespread | P6 | M | yes | yes | yes | no | no |

## §8 Discovery, distribution, signing, policies

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 45 | Extension gallery (signed, vetted, categories, reviews, submission workflow) | desktop+mobile | not-started | widespread | 1.0-blocker | XL | yes | yes | yes | no | no |
| 46 | Extension signing and self-distribution | desktop+mobile | not-started | ubiquitous | 1.0-blocker | M | yes | yes | yes | no | no |
| 47 | Extension review pipeline (static analysis, dynamic analysis, human review) | desktop+mobile | not-started | widespread | 1.0-blocker | XL | yes | yes | yes | no | no |
| 48 | In-product extension management UI (chrome://extensions, about:addons) | desktop+mobile | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no |
| 49 | Permission model (host_permissions, optional_permissions, permissions.request) | desktop+mobile | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no |
| 50 | incognito and file:// access (spanning / split / not_allowed) | desktop | not-started | widespread | P6 | M | yes | yes | yes | no | no |

## §9 User customisation

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 51 | User stylesheet injection (userContent.css, per-site custom CSS) | desktop | not-started | widespread | P6 | M | no | yes | partial | no | no |
| 52 | userChrome.css (CSS override of browser chrome UI) | desktop | not-started | niche | P6 | M | no | yes | no | no | no |
| 53 | Custom search engine definition (OpenSearch description format) | desktop+mobile | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no |
| 54 | Custom keyword-triggered search (keyword field: `g hello`) | desktop+mobile | not-started | widespread | P6 | S | yes | yes | no | no | no |
| 55 | Custom keyboard shortcut mapping for browser UI actions | desktop | not-started | mixed | P6 | M | yes | no | no | no | no |
| 56 | Custom mouse gesture binding | desktop | not-started | niche | P4 | M | no | no | no | no | no |
| 57 | Custom new tab page background image | desktop+mobile | not-started | mixed | P4 | S | yes | no | no | no | no |
| 58 | Custom new tab page layout (toggle sections, widget placement) | desktop+mobile | not-started | mixed | P4 | M | yes | no | no | no | no |
| 59 | Custom homepage and startup page | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 60 | Custom toolbar layout (drag-and-drop buttons, toggle visibility) | desktop | not-started | mixed | P4 | M | yes | yes | yes | no | no |
| 61 | Custom theme (data-only manifest: colours, images, properties) | desktop | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 62 | Dark mode toggle (system / always / never, per-site override) | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 63 | Forced colours / high-contrast mode toggle | desktop | not-started | mixed | P4 | S | yes | yes | no | no | no |
| 64 | Accent colour selection (toolbar accent colour) | desktop | not-started | mixed | P4 | S | yes | no | no | no | no |
| 65 | Custom font selection (sans-serif, serif, monospace, min font size) | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |

## §10 User-script ecosystem

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|---|
| 66 | Userscript metadata block (==UserScript== header with @match, @grant, @require, @resource) | desktop+mobile | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 67 | @match and @include URL pattern matching (glob-style) | desktop+mobile | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no |
| 68 | @grant and GM.* / GM_* value bridge (GM_setValue, GM_xmlhttpRequest, etc.) | desktop+mobile | not-started | widespread | P6 | L | yes | yes | yes | no | no |
| 69 | @require and @resource (external scripts/assets fetched at install, injected at run) | desktop+mobile | not-started | widespread | P6 | M | yes | yes | yes | no | no |
| 70 | Userscript manager integration page (dashboard, search, install, edit, enable/disable) | desktop+mobile | not-started | widespread | P4 | L | yes | yes | yes | no | no |

---

**Total rows: 70** (50 extension surface + 20 customisation)
