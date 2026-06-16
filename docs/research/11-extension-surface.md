# Chunk 10 — Extension and Customisation Surface

> **Chunk 10 of 14.** This file covers the surface by which third-party
> developers extend the browser (extensions, themes, search engines,
> dictionaries) **and** the surface by which users personalise it
> (custom themes, custom CSS, custom JS, custom keyboard shortcuts,
> custom search engines, custom user scripts). The extension API layer
> is the canonical WebExtensions / Manifest V3 surface (MDN "Browser
> Extensions" docs).
>
> **Worktree:** `research/competitive-parity`
> (base: `audit/m4-window`).
> **Methodology contract:** `00-methodology.md`.
> **Source ladder:** `citations/sources.md`.
> **Output contract:** `README.md` §"Per-chunk output contract".

---

## Scope

**In:** Manifest V2 and Manifest V3 (background, content scripts,
action, side panel, omnibox, devtools, options, sandbox, storage,
permissions, host_permissions, declarativeNetRequest, native
messaging, web_accessible_resources, externally_connectable,
content_security_policy, browser_specific_settings, theme,
internationalisation); the WebExtensions core API namespaces
(tabs, windows, storage, cookies, webRequest, declarativeNetRequest,
webNavigation, history, bookmarks, downloads, contextMenus,
notifications, alarms, runtime, extension, management, permissions,
contentScripts, scripting, identity, i18n, omnibox, sidePanel,
sidebarAction, tabGroups, userScripts, scripting, devtools, theme,
tts, pageCapture, printing, enterprise); the user-side customisation
surface (custom CSS injection, custom JS, custom search engines
including OpenSearch, custom keyboard shortcuts, custom themes,
custom new tab, custom homepage, custom toolbar); userscript
ecosystem (Greasemonkey-style `@match`, `@grant`, `@require`,
`@resource`, `GM.*` value bridge, Tampermonkey / Violentmonkey
compat).

**Out:** Web platform APIs (chunk 6). Storage mechanics (chunk 4)
except as they appear inside the extension storage API. Security
policy (chunk 3) except as it appears inside the extension
content security policy. DevTools as a developer tool (chunk 8) —
this chunk only covers the **devtools_page** surface that
extensions register to extend DevTools. User-facing UX (chunk 7)
except where the UX is the extension management surface. OS
distribution and platform sandboxing (chunk 11).

---

## Methodology for this chunk

Rows derived from: MDN "Browser Extensions" reference (Tier 2,
canonical for API surface), `manifest.json` schema documented at
`developer.chrome.com/docs/extensions/reference/manifest` and
`developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json`
(Tier 2), Chrome Extensions documentation at
`developer.chrome.com/docs/extensions` (Tier 3, vendor reference
but treated as primary for Chromium specifics), Chrome MV2
deprecation timeline at
`developer.chrome.com/docs/extensions/develop/migrate/mv2-deprecation-timeline`
(Tier 3, dated facts about MV3 sunset), Apple "Safari Web
Extensions" reference (Tier 3, vendor), the OpenSearch description
document format spec (Tier 1, A9/OpenSearch.org XML schema),
the Greasemonkey 4 / Tampermonkey metadata block convention
(Tier 3, de-facto).

Naming follows `00-methodology.md` §7: use the spec / MDN term,
not the vendor's product name. WebExtensions is a Mozilla term but
it is also the umbrella MDN uses, so it is acceptable. "Chrome
Web Store" and "Mozilla Add-ons" are acceptable as proper nouns
for the store fronts; we refer to them as "the gallery" or "the
store" generically and spell out the proper noun in the row body.

Where a capability has different status across the three families
(Chromium, Firefox, WebKit), the prevalence bucket reflects the
*most prevalent* position (the "Browser prevalence" column is
"would a 2026 user expect this to work" rather than "all three
do it the same way"). Engine notes spell out the divergence.

Engine status legend used below:
- **Chromium** = Google-authored open source engine used by
  Chrome, Edge, Brave, Arc, Vivaldi, Opera and the OEM Android
  System WebView.
- **Firefox** = Mozilla's engine family (Gecko, SpiderMonkey).
- **WebKit** = Apple's engine family used by Safari (macOS, iOS,
  iPadOS) and the WKWebView / SFSafariViewController embedded
  surface. WebExtensions-on-WebKit (Safari 14+) is a real but
  MV2-style API, not MV3.
- **Servo** = Mozilla Research's experimental Rust engine; not a
  shipping product browser, but called out for completeness.
- **Ladybird** = the from-scratch C++ browser being built by the
  SerenityOS contributors; no extension host in 2026.
- **Flow** = the Meta-funded JSC-based mobile shell, used inside
  Facebook / Instagram / WhatsApp in-app browsers.

---

## Section index

| Section | Rows | File |
|---------|------|------|
| 1. Manifest and packaging | 1–7 | this file |
| 2. Content scripts | 8–11 | this file |
| 3. Background contexts | 12–14 | this file |
| 4. Core API namespaces | 15–28 | this file |
| 5. UI surfaces (popup, action, omnibox, side panel, devtools, options, sandbox) | 29–36 | this file |
| 6. Network modification and content filtering | 37–39 | this file |
| 7. Identity, i18n, native messaging, enterprise | 40–44 | this file |
| 8. Discovery, distribution, signing, policies | 45–50 | this file |
| 9. User customisation (custom CSS, custom JS, search engines, keyboard, toolbar, new tab, theme) | 51–65 | this file |
| 10. User-script ecosystem | 66–70 | this file |
| **Total** | **70** | — |

> **File split note.** Per the per-file 600-line cap, this chunk is
> delivered as two files:
>
> - `11-extension-surface.md` (this file) — rows 1–50, the
>   WebExtensions platform surface (developer-facing extension
>   APIs).
> - `11-customisation.md` — rows 51–70, the user-side
>   customisation surface (custom CSS / JS / search / shortcuts /
>   themes / userscripts).
>
> The row numbering is contiguous across both files.

---

## Section 1 — Manifest and packaging

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 1 | Manifest V3 (`manifest_version: 3`, declarative fields, `background.service_worker`, `action`, `host_permissions`, `optional_host_permissions`, `content_scripts`, `web_accessible_resources`, `externally_connectable`, `declarative_net_request`, `permissions`, `optional_permissions`, `content_security_policy`, `browser_specific_settings`) | desktop+mobile+embedded | not-started | widespread (MV3 is the manifest Chromium mandates for new submissions; Firefox supports MV3 as an opt-in alternative to MV2; WebKit/Safari uses a non-versioned manifest, not MV3 per se) | P6 / 1.0-blocker | XL | Chromium: yes/stable (MV3 mandatory for new submissions, MV2 legacy); Firefox: yes/stable for MV2 (default), MV3 opt-in supported, MV2 not deprecated as of 2026; WebKit: partial (uses a manifest model inspired by WebExtensions, "Safari Web Extensions", MV2-style API only, MV3 in development); Servo: no; Ladybird: no; Flow: no. | [Chrome manifest reference](https://developer.chrome.com/docs/extensions/reference/manifest); [MDN manifest_version](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/manifest_version) |
| 2 | Manifest V2 legacy (`background.scripts`, `background.persistent`, `webRequest` blocking, `content_scripts.css`, `browser_action`, `page_action`) | desktop+mobile | not-started | legacy (Chromium deprecated, Firefox still default for many extensions, WebKit retained) | do-not-touch (rely on MV3 instead) | L (we only need to read MV2 manifests for *recognition*, not execute them; matching MV2 is `S`) | Chromium: deprecated, disabled by default in 2025, removed in stages through 2026; Firefox: yes/stable (default manifest model); WebKit: yes/stable (effectively the only model); Servo/Ladybird/Flow: no. | [Chrome MV2 deprecation timeline](https://developer.chrome.com/docs/extensions/develop/migrate/mv2-deprecation-timeline) |
| 3 | Static and dynamic `declarative_net_request` rules (static rule set declared in `rules.json` at install time, dynamic rules added/removed at runtime via `chrome.declarativeNetRequest.updateDynamicRules`, session-scoped rules via `updateSessionRules`, matched-rules query via `getMatchedRules`) | desktop+mobile+embedded | not-started | widespread (MV3's content-filter primitive) | P6 | L (rule engine inside the network stack — see chunk 5's network filter as the Spiral analog) | Chromium: yes/stable (MV3 only); Firefox: partial (MV3 supports it, MV2 path uses `webRequest` blocking instead); WebKit: no; Servo: no; Ladybird: no; Flow: no. | [MDN declarative_net_request](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/declarative_net_request); [Chrome DNR](https://developer.chrome.com/docs/extensions/reference/api/declarativeNetRequest) |
| 4 | Manifest version negotiation and `browser_specific_settings` (gecko id, strict_min_version update_url, edge add-on store id, safari bundle id) | desktop+mobile+embedded | not-started | widespread | P6 | M | Chromium: yes (uses `browser_specific_settings` for cross-publish); Firefox: yes (gecko id is mandatory for AMO); WebKit: yes (bundle id); Servo/Ladybird/Flow: no. | [MDN browser_specific_settings](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/browser_specific_settings) |
| 5 | Web-accessible resources (declarative `web_accessible_resources.matches` with `extension_ids` and `use_dynamic_url`, exposes extension assets to web pages) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | Chromium: yes/stable (MV3 schema is `web_accessible_resources` as an array of objects); Firefox: yes/stable; WebKit: yes/stable (the Safari equivalent is `resources` on the extension target). | [MDN web_accessible_resources](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/web_accessible_resources) |
| 6 | Externally connectable (`externally_connectable.ids`, `externally_connectable.matches`, allows specific web pages to `runtime.connect`/`runtime.sendMessage` to the extension) | desktop+mobile+embedded | not-started | widespread | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable. | [MDN externally_connectable](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/externally_connectable) |
| 7 | Extension content security policy (`content_security_policy.extension_pages` and `content_security_policy.content_scripts`; `script-src 'self'`; `object-src 'self'`; no remote code, no `eval`) | desktop+mobile+embedded | not-started | ubiquitous (MV3 forbids remote code, `eval`, `Function()` in extension pages by default) | P6 | M | Chromium: yes/stable (MV3 default is `script-src 'self'; object-src 'self'`); Firefox: yes/stable (slightly relaxed for content scripts); WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN content_security_policy in extensions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/content_security_policy) |

## Section 2 — Content scripts

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 8 | Static content scripts (`content_scripts[].matches`, `js`, `css`, `run_at`, `all_frames`, `match_about_blank`, `match_origin_as_fallback`, `world`, `css_origin`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | XL (content-script world, isolated world, frame-iteration, content-script CSS injection, page ↔ extension typed message bus — this is a new runtime, a new isolation boundary, and a new permission grant) | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [MDN content_scripts](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/content_scripts); [MDN content_scripts css_origin](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/content_scripts#css_origin) |
| 9 | `run_at` timing (`document_start`, `document_end`, `document_idle`, plus Firefox's `document_start_early` and `document_idle` semantics) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M (timing is implementable on top of the load-event pipeline that the host already has; the cost is hooking the world, not the timing itself) | Chromium: yes/stable (`document_start`, `document_end`, `document_idle`); Firefox: yes/stable (adds `document_start_early` to win against the page); WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN run_at](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/content_scripts#run_at) |
| 10 | Dynamic content scripts (`chrome.scripting.registerContentScripts` / `unregisterContentScripts` / `getRegisteredContentScripts` / `updateContentScripts` in MV3; `chrome.tabs.executeScript`/`insertCSS` legacy path) | desktop+mobile+embedded | not-started | widespread | P6 | L (script registration has to be persistent across page loads, with the right isolation) | Chromium: yes/stable (MV3 `scripting` namespace, replaces `tabs.executeScript`); Firefox: yes/stable (`contentScripts.register` API is a Firefox extension); WebKit: yes/stable (`browser.tabs.executeScript` retained); Servo/Ladybird/Flow: no. | [MDN scripting.registerContentScripts](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/scripting/registerContentScripts); [MDN contentScripts.register](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/contentScripts/register) |
| 11 | Isolated world / main world split (`world: "ISOLATED"` is the default, `world: "MAIN"` exposes a content script to the page's JS context, used for `Page.goBack` shims and banking-trojan-style interceptors — not recommended) | desktop+mobile+embedded | not-started | widespread | P6 | L (Vortex would need a "world" concept per V8 spec; Spiral can use the existing V8 isolated world semantics if Vortex follows the same model) | Chromium: yes/stable (added in MV3); Firefox: yes/stable (added in MV3); WebKit: partial (no public MAIN world access in Web Extensions); Servo/Ladybird/Flow: no. | [Chrome content scripts MAIN world](https://developer.chrome.com/docs/extensions/develop/concepts/content-scripts#host-page-communication) |

## Section 3 — Background contexts

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 12 | MV3 service-worker background (`background.service_worker`, `importScripts`, `chrome.runtime.onInstalled`, `onStartup`, `onMessage`, `alarms`-based keep-alive, idle shutdown after 30s of inactivity, no DOM access) | desktop+mobile+embedded | not-started | widespread | P6 | XL (needs a Service Worker host inside the extension process; Vortex can run as a worker; this is the *first* major integration of a worker context with the extension host) | Chromium: yes/stable; Firefox: yes/stable (MV3 supported as of Firefox 121, April 2024); WebKit: no (Web Extensions on Safari do not have a service worker background — they use a non-persistent background page instead); Servo/Ladybird/Flow: no. | [Chrome extension service workers](https://developer.chrome.com/docs/extensions/develop/concepts/service-workers); [MDN background](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/background) |
| 13 | MV2 background page (`background.scripts`, `background.page`, `background.persistent: true` for always-on, `false` for event page) | desktop+mobile | not-started | legacy (still the dominant model in Firefox and WebKit) | do-not-touch (defer recognition only) | L (we only need to *recognise* MV2 manifests, not run them — recognising is `S`; running is `L`) | Chromium: deprecated; Firefox: yes/stable (default); WebKit: yes/stable (effectively the only model); Servo/Ladybird/Flow: no. | [MDN MV2 background](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/background#manifest_v2_examples) |
| 14 | Offscreen document (`chrome.offscreen.createDocument` / `closeDocument` / `hasDocument`, gives a MV3 service worker a hidden DOM for tasks that need it: media decoding, PDF rendering, Clipboard API, audio playback) | desktop+mobile | not-started | mixed (Chromium only in 2026) | P6 | M (a hidden web view tied to the extension service worker) | Chromium: yes/stable (MV3 only); Firefox: no; WebKit: no; Servo/Ladybird/Flow: no. | [Chrome offscreen API](https://developer.chrome.com/docs/extensions/reference/api/offscreen) |

## Section 4 — Core API namespaces

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 15 | `tabs` API (`query`, `get`, `getCurrent`, `create`, `update`, `remove`, `duplicate`, `reload`, `onUpdated`, `onRemoved`, `onActivated`, `onAttached`, `onDetached`, `onMoved`, `onReplaced`, `onCreated`, `onZoomChange`, `captureVisibleTab`, `connect`, `discard`, `goBack`, `goForward`, `group`, `ungroup`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | XL (this is the canonical "what's a tab" bridge; the `spiral-browser` tab model has to expose every observable event and accept a create / remove / move that matches the WebExtensions model) | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable (Safari has a partial implementation — no `captureVisibleTab` for security, no `group`); Servo/Ladybird/Flow: no. | [MDN tabs API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs) |
| 16 | `tabGroups` API (`query`, `update`, `get`, `onUpdated`, `onMoved`, `onRemoved`, colour, collapsed, title) | desktop | not-started | mixed (Chromium primary, Firefox has the *concept* via `contextualIdentities` / containers but not the API, WebKit no) | P6 | L (the data model is small but the chrome-level tab grouping UI is its own P4 deliverable) | Chromium: yes/stable (added 2024 in MV3, supersedes the older `tabGroups` work); Firefox: no (uses "Containers" as a different model — per-origin identity profiles, not a UI grouping); WebKit: no; Servo/Ladybird/Flow: no. | [Chrome tabGroups](https://developer.chrome.com/docs/extensions/reference/api/tabGroups) |
| 17 | `windows` API (`getCurrent`, `get`, `getAll`, `create`, `update`, `remove`, `onCreated`, `onRemoved`, `onFocusChanged`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN windows API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/windows) |
| 18 | `storage` API (`storage.local`, `storage.sync`, `storage.managed`, `storage.session`, `storage.onChanged`, `bytesInUse`, `setAccessLevel` / `getKeys`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L (storage.session is a `OPFS`-backed store with a key-value API; storage.sync is a remote-replicated store; storage.managed is a read-only policy-injected store) | Chromium: yes/stable (MV3 adds `storage.session`); Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN storage API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/storage) |
| 19 | `cookies` API (`get`, `set`, `getAll`, `remove`, `getAllCookieStores`, `onChanged`, partition key support for MV3) | desktop+mobile | not-started | ubiquitous | P6 | L (cookie jar permissions + cookie store enumeration, partition key support for CHIPS) | Chromium: yes/stable (MV3 cookie partitioning); Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN cookies API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/cookies) |
| 20 | `webRequest` API (MV2 only, MV3 deprecated; `onBeforeRequest`, `onBeforeSendHeaders`, `onSendHeaders`, `onHeadersReceived`, `onAuthRequired`, `onResponseStarted`, `onCompleted`, `onErrorOccurred`, `onBeforeRedirect`, blocking response with `cancel`, `redirect`, `modifyHeaders`) | desktop+mobile+embedded | not-started | legacy (the MV2 "blocking webRequest" model is the thing MV3 replaced with declarativeNetRequest) | do-not-touch (MV2 path only; if shipped, ship MV3 DNR instead) | L | Chromium: deprecated (MV3 path is DNR); Firefox: yes/stable (MV2 still default, blocking retained); WebKit: no; Servo/Ladybird/Flow: no. | [MDN webRequest API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest); [Chrome MV2 deprecation](https://developer.chrome.com/docs/extensions/develop/migrate/mv2-deprecation-timeline) |
| 21 | `declarativeNetRequest` runtime API (already covered for static rules in row 3; this is the *runtime* side: `getDynamicRules`, `getSessionRules`, `updateDynamicRules`, `updateSessionRules`, `getMatchedRules`, rule limit (30,000 static + 30,000 dynamic + 5,000 session), `isUrlConditionCaseSensitive`) | desktop+mobile+embedded | not-started | widespread | P6 | L | Chromium: yes/stable; Firefox: yes/stable (MV3); WebKit: no; Servo/Ladybird/Flow: no. | [MDN declarativeNetRequest](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/declarativeNetRequest) |
| 22 | `webNavigation` API (`onBeforeNavigate`, `onCommitted`, `onDOMContentLoaded`, `onCompleted`, `onErrorOccurred`, `onHistoryStateUpdated`, `onReferenceFragmentUpdated`, `onTabReplaced`, `getAllFrames`, `getFrame`) | desktop+mobile+embedded | not-started | widespread | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: no (Safari exposes a subset via `browser.webNavigation` as a "not in production" state); Servo/Ladybird/Flow: no. | [MDN webNavigation](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webNavigation) |
| 23 | `history` API (`search`, `getVisits`, `addUrl`, `deleteUrl`, `deleteAll`, `deleteRange`, `onVisited`, `onVisitRemoved`) | desktop+mobile | not-started | ubiquitous | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable (no `deleteAll`, requires user gesture for writes); Servo/Ladybird/Flow: no. | [MDN history API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/history) |
| 24 | `bookmarks` API (`getTree`, `get`, `getRecent`, `getChildren`, `getSubTree`, `create`, `move`, `remove`, `update`, `search`, `onCreated`, `onChanged`, `onMoved`, `onRemoved`, `onImportBegan`, `onImportEnded`) | desktop+mobile | not-started | ubiquitous | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN bookmarks API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/bookmarks) |
| 25 | `downloads` API (`search`, `download`, `cancel`, `pause`, `resume`, `removeFile`, `open`, `show`, `showDefaultFolder`, `setShelfEnabled`, `onCreated`, `onChanged`, `onErased`, `onDeterminingFilename`) | desktop+mobile | not-started | ubiquitous | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable (no `open` for security — extensions can prompt, not open); Servo/Ladybird/Flow: no. | [MDN downloads API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/downloads) |
| 26 | `contextMenus` API (`create`, `update`, `remove`, `removeAll`, `onClicked`, contexts: `page`, `selection`, `link`, `image`, `video`, `audio`, `frame`, `editable`, `password`, `all`, `launcher`, `bookmark`, `tab`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN contextMenus API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/menus) |
| 27 | `notifications` API (`create`, `update`, `clear`, `getAll`, `getPermissionLevel`, `onClicked`, `onButtonClicked`, `onClosed`, `onPermissionLevelChanged`) | desktop+mobile+embedded | not-started | widespread | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN notifications API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/notifications) |
| 28 | `runtime` API (`sendMessage`, `sendNativeMessage`, `onMessage`, `onUserScriptMessage`, `connect`, `onConnect`, `onConnectExternal`, `onMessageExternal`, `onInstalled`, `onStartup`, `onSuspend`, `onSuspendCanceled`, `onUpdateAvailable`, `onRestartRequired`, `getURL`, `getManifest`, `getPlatformInfo`, `getBrowserInfo`, `getContexts`, `setUninstallURL`, `reload`, `requestUpdateCheck`, `id`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L (the message bus is half the work — typed messages with origin checking, port lifetimes, native-message hand-off to a separate OS process) | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN runtime API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime) |

## Section 5 — UI surfaces (popup, action, omnibox, side panel, devtools, options, sandbox)

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 29 | `action` API (unified `browser_action` / `page_action` in MV3, `default_popup`, `default_icon`, `default_title`, `onClicked`, `setIcon`, `setPopup`, `setTitle`, `setBadgeText`, `setBadgeBackgroundColor`, `setUserSettings`, `getUserSettings`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | Chromium: yes/stable (MV3); Firefox: yes/stable (MV3 supports `action`, but Firefox Manifest V2 still uses `browser_action` / `page_action`); WebKit: partial (Safari has `browser_action` / `page_action` as the only model); Servo/Ladybird/Flow: no. | [MDN action](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/action) |
| 30 | `sidePanel` API (MV3, `setOptions`, `open`, `setPanelBehavior`, `setIcon`, `getOptions`, `getPanelBehavior`, `onOpened`) and `sidebarAction` (Firefox legacy) | desktop+mobile | not-started | widespread | P6 | L (the side panel is a separate web view per tab group / window, hosting an extension page; this is a new chrome surface, not a tweak) | Chromium: yes/stable (MV3 `sidePanel`); Firefox: yes/stable (`sidebarAction` is the older Firefox-only API, sidePanel is not supported in Firefox); WebKit: no; Servo/Ladybird/Flow: no. | [Chrome sidePanel](https://developer.chrome.com/docs/extensions/reference/api/sidePanel); [MDN sidebarAction](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/sidebarAction) |
| 31 | `omnibox` API (`keyword`, `description`, `defaultSuggestion`, `onInputStarted`, `onInputChanged`, `onInputEntered`, `onDeleteSuggestion`, suggestion styles, `setDefaultSuggestion`) | desktop | not-started | widespread | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable (Safari 16+); Servo/Ladybird/Flow: no. | [MDN omnibox](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/omnibox) |
| 32 | `devtools_page` (a top-level extension HTML page with access to `chrome.devtools.*`, opens inside DevTools as a panel, drawer, or sidebar) | desktop | not-started | mixed (Chromium only at the API level; Firefox exposes `devtools.panels` and `devtools.inspectedWindow` but the `devtools_page` is a Chromium convention) | P6 | M | Chromium: yes/stable (`chrome.devtools.panels.create`, `chrome.devtools.inspectedWindow.eval`, `chrome.devtools.network.onRequestFinished` returning HAR); Firefox: partial (`devtools.panels.create`, `devtools.inspectedWindow.eval`, no HAR); WebKit: no (Safari's Web Inspector has no extension API); Servo/Ladybird/Flow: no. | [Chrome devtools API](https://developer.chrome.com/docs/extensions/reference/api/devtools); [MDN devtools](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/devtools) |
| 33 | `options_ui` (`options_ui.page` or `options_ui.open_in_tab`, the extension's settings page opened from `chrome://extensions`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN options_ui](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/options_ui) |
| 34 | `sandbox` page (`sandbox.pages`, an extension page with a unique opaque origin, CSP `sandbox`, no extension APIs, used for cross-extension trusted UI like a content blocker UI shim) | desktop+mobile | not-started | niche (Chromium-only) | P6 | M | Chromium: yes/stable; Firefox: no (uses iframe sandbox via CSP); WebKit: no; Servo/Ladybird/Flow: no. | [MDN sandbox](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/sandbox) |
| 35 | `chrome_url_overrides` (override the new tab page, bookmarks page, history page, or a custom page with an extension HTML page) | desktop | not-started | mixed (Chromium + Firefox; WebKit has no new tab override) | P6 | M | Chromium: yes/stable (newtab, bookmarks, history); Firefox: yes/stable (newtab only); WebKit: no; Servo/Ladybird/Flow: no. | [MDN chrome_url_overrides](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/chrome_url_overrides) |
| 36 | `commands` API (keyboard shortcut bindings: `_execute_browser_action`, `_execute_page_action`, `_execute_sidebar_action`, custom shortcuts with `description`, `suggested_key`, `global`, `OnKey` event) | desktop | not-started | widespread | P6 | M (shortcut surface is shared with chunk 7's user-side customisation, but here the surface is extension-declared) | Chromium: yes/stable; Firefox: yes/stable (with extra "Ctrl+Alt+R" override semantics); WebKit: partial (Safari supports `commands` but no `global`); Servo/Ladybird/Flow: no. | [MDN commands](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/commands) |

## Section 6 — Network modification and content filtering

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 37 | `proxy` API (`proxy.settings` with `mode` `direct`, `auto_detect`, `pac_script`, `fixed_servers`, `system`, `per_host`; the extension returns a `ProxyConfig` object synchronously from `onRequest`) | desktop+mobile | not-started | mixed (Chromium and Firefox; WebKit has no proxy extension API) | P6 | M (the proxy config is consumed by the network stack — the Spiral `spiral-net` analog already has a config surface) | Chromium: yes/stable; Firefox: yes/stable; WebKit: no; Servo/Ladybird/Flow: no. | [MDN proxy](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/proxy) |
| 38 | `contentScripts` runtime API (Firefox extension of the `content_scripts` manifest key — `register`, `unregister`) | desktop+mobile | not-started | niche (Firefox only) | P6 | M (the Spiral analog is the `scripting.registerContentScripts` Chromium path) | Chromium: no (use `chrome.scripting.registerContentScripts`); Firefox: yes/stable; WebKit: no; Servo/Ladybird/Flow: no. | [MDN contentScripts.register](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/contentScripts/register) |
| 39 | `userScripts` API (`userScripts.register`, `userScripts.configureWorld`, `userScripts.getScripts`, `userScripts.unregister`, runs as MAIN world scripts with a separate "USER_SCRIPT" world, used for Tampermonkey / Violentmonkey) | desktop+mobile | not-started | widespread (Chrome MV3 in 2024+, Firefox since 2021, Safari via Safari Web Extensions userscripts support) | P6 | L (the user-script world is separate from the extension's main world, has its own CSP relaxation, and the registration is per-origin) | Chromium: yes/stable (added in MV3 in Chrome 120+); Firefox: yes/stable (`browser.userScripts`, behind a `userScripts` permission in MV2 and a `user_scripts` API in MV3); WebKit: yes/stable (Safari has userscripts support via Web Extensions on macOS 13+); Servo/Ladybird/Flow: no. | [Chrome userScripts](https://developer.chrome.com/docs/extensions/reference/api/userScripts); [MDN userScripts](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/userScripts) |

## Section 7 — Identity, i18n, native messaging, enterprise

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 40 | `identity` API (`getAuthToken`, `launchWebAuthFlow`, `getRedirectURL`, OAuth2 helpers, profile email, `removeCachedAuthToken`) | desktop+mobile+embedded | not-started | mixed (Chromium primary; Firefox is "in development" and the API is gated behind the `identity` permission; WebKit no) | P6 | M (the WebAuth flow requires a loopback HTTP listener, or a custom-scheme redirect, depending on platform) | Chromium: yes/stable; Firefox: partial (limited implementation); WebKit: no; Servo/Ladybird/Flow: no. | [Chrome identity](https://developer.chrome.com/docs/extensions/reference/api/identity); [MDN identity](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/identity) |
| 41 | `i18n` API (`getMessage`, `getUILanguage`, `getAcceptLanguages`, `getMessage` with substitutions, `default_locale` in manifest, `locales/` directory with `messages.json` per locale) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN i18n](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/i18n) |
| 42 | `nativeMessaging` API (`runtime.sendNativeMessage`, `runtime.connectNative`, allows an extension to talk to a registered native host via stdin/stdout JSON-framed messages; manifest registration on `com.*` / `org.*` etc. for Mac, registry for Windows, `.desktop` files for Linux) | desktop | not-started | mixed (Chromium + Firefox; WebKit has its own SFSafariExtensionHandler for native messaging on macOS) | P6 | L (the native messaging host is an out-of-process bridge; the JSON framing is well-defined; the install path is platform-specific) | Chromium: yes/stable (`runtime.connectNative`); Firefox: yes/stable (`browser.runtime.connectNative`); WebKit: yes/stable (via SFSafariExtensionHandler and `SafariExtensionHandler` on iOS); Servo/Ladybird/Flow: no. | [Chrome native messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging); [MDN nativeMessaging](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/nativeMessaging) |
| 43 | Enterprise / managed extensions (`ExtensionSettings` policy, `ExtensionInstallForcelist`, `ExtensionInstallBlocklist`, `ExtensionInstallAllowlist`, `ExtensionInstallSources`, `ExtensionSettings` schema, `Update` URL override, `runtime.installedType: "admin"`, `storage.managed` for read-only config) | desktop+mobile | not-started | mixed (Chromium is the canonical enterprise surface, driven by `chrome://policy` and the `ExtensionSettings` schema; Firefox has `policies.json` and managed bookmarks; WebKit has a `ManagedConfiguration` for managed app config) | P6 | M | Chromium: yes/stable (the enterprise distribution path is "extension_settings" + "extension_install_forcelist" in the policy blob); Firefox: yes/stable (`policies.json` shipped to a managed location); WebKit: no (Safari has `ManagedConfiguration` for *iOS apps* using WKWebView, not the same model); Servo/Ladybird/Flow: no. | [Chrome enterprise extension policy](https://chromeenterprise.google/policies/?policy=ExtensionSettings) |
| 44 | `management` API (`getAll`, `get`, `getSelf`, `getPermissionWarningsByManifest`, `getSelf` returns the extension's own enabled state, `setEnabled`, `uninstall`, `uninstallSelf`, `onInstalled`, `onUninstalled`, `onEnabled`, `onDisabled`) | desktop+mobile | not-started | widespread | P6 | M (this is a privileged API; the extension needs the `management` permission) | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN management API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/management) |

## Section 8 — Discovery, distribution, signing, policies

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 45 | Extension gallery (developer-facing: signed, vetted, with categories, ratings, reviews, screenshots, languages, version history, and a public "manifest" page showing the declared permissions) | desktop+mobile | not-started | widespread (every shipping browser family has one) | 1.0-blocker | XL (the gallery is a service: sign-in for developers, submission workflow, automatic static + dynamic analysis, reviewer queue, search, ratings, comments, takedown process, regional compliance for EU DSA / Apple tax) | Chromium: yes/stable ("Chrome Web Store" and Microsoft "Edge Add-ons" and Brave's "Brave Add-on Store" all reuse the CWS pipeline); Firefox: yes/stable ("Mozilla Add-ons" / addons.mozilla.org, AMO); WebKit: yes/stable ("App Store" submission for Safari Web Extensions on macOS and iOS); Servo/Ladybird/Flow: no. | [Chrome Web Store](https://developer.chrome.com/docs/webstore) |
| 46 | Extension signing and self-distribution (Chromium: required for CWS, optional for unpacked; Firefox: required for AMO, optional for about:debugging; WebKit: required for App Store, optional for "unsigned" via spctl on macOS) | desktop+mobile | not-started | ubiquitous | 1.0-blocker | M (the signing is just a signature check; the policy behind it is the heavy part) | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [Chrome web store publish](https://developer.chrome.com/docs/webstore/publish) |
| 47 | Extension review pipeline (static analysis: manifest lint, remote-code check, CSP check, declared permission audit; dynamic analysis: sand-boxed run of the extension against benign pages; human review for "privileged" extensions touching privacy / tabs / webRequest / proxy / devtools) | desktop+mobile | not-started | widespread | 1.0-blocker | XL (this is the second-largest "you are a browser" capability in this file — only the manifest parser is larger) | Chromium: yes/stable (a mix of automated + human review, with the "Privacy practices" disclosure enforced for V3); Firefox: yes/stable (a mix of automated + human review, with the "Required permissions" disclosure enforced); WebKit: yes/stable (App Store review, two-week typical cycle); Servo/Ladybird/Flow: no. | [Chrome Web Store program policies](https://developer.chrome.com/docs/webstore/program-policies) |
| 48 | In-product extension management UI (a `chrome://extensions` / `about:addons` / `about:debugging#/runtime/this-firefox` page that lists installed extensions, shows the manifest, the source, the permissions, the store listing link, and a remove / disable / allow-in-incognito toggle) | desktop+mobile | not-started | ubiquitous | P6 | L (the page itself is a web view, but the data source is a privileged "extensions store" and the toggle behaviour is the consumer-side of `storage.managed` and `incognito` / `split` / `spanning`) | Chromium: yes/stable (`chrome://extensions`); Firefox: yes/stable (`about:addons`); WebKit: yes/stable (Safari's "Manage Extensions" sheet); Servo/Ladybird/Flow: no. | [Chrome extensions UI](https://developer.chrome.com/docs/extensions) |
| 49 | Permission model: host_permissions, optional_permissions, runtime permissions, `permissions.request` (with user-gesture gating on MV3), `permissions.remove`, `permissions.contains`, `permissions.getAll` | desktop+mobile | not-started | ubiquitous | P6 | L (this is the second half of the extension host — every API namespace consults the permission store before allowing a call) | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN permissions API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/permissions) |
| 50 | `incognito` and `file://` access (`incognito: "spanning" | "split" | "not_allowed"`, `incognito_context: "incognito_persistent" | "incognito_session_only"`, and the per-extension file-scheme / about:blank access controls) | desktop | not-started | widespread | P6 | M (small data model; the UX is the "allow in private" toggle) | Chromium: yes/stable; Firefox: yes/stable (`incognito: "not_allowed" | "spanning"`); WebKit: yes/stable; Servo/Ladybird/Flow: no. | [MDN incognito](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/incognito) |

---

> **Continue to:** `11-customisation.md` for rows 51–70
> (user-side customisation: custom CSS, custom JS, custom search
> engines, custom keyboard shortcuts, custom themes, custom
> new tab / home page, custom toolbar, userscript ecosystem).
