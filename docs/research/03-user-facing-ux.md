# Chunk 7 — User-Facing UX (Deep)

> **Chunk 7 of 14.** This is the **deep UX** chunk. Per the user's
> explicit selection, every row carries (a) a **UX expectation** paragraph
> describing the user-visible behaviour, and (b) **full per-engine notes**
> for Chromium / Firefox / WebKit / Servo / Ladybird / Flow. Matrix
> width is wider than other chunks by design — this is the surface the
> human touches, and the breadth is the point.

---

## Scope

**In:** tab and window management; address bar / omnibox; navigation;
bookmarks and saved items; downloads; find-in-page and text tools;
privacy and tracking UI; passwords and autofill; sharing and
integration; reading, viewing, media UI; customisation and ergonomics;
sync and account; settings; crash handling and diagnostics; telemetry
and feedback; help and onboarding.

**Out:** engine internals (chunks 1, 2, 3, 4, 5, 6), DevTools panels
(chunk 8), accessibility primitives (chunk 9), extension manager UI is
in scope but extension APIs are chunk 10, distribution / installer
(chunk 11), AI assistant (per methodology §9), crypto wallet (per
methodology §9).

**Naming:** spec / capability names per methodology §7. No brand
names for product surfaces; engine names (Chromium, Firefox, WebKit,
Servo, Ladybird, Flow) are allowed because they are engine identities,
not product brands.

**Splitting:** the file splits into two parts because the full row set
would exceed the 600-line cap. Sections 1–8 live in
`03-user-facing-ux.md` (this file); sections 9–16 live in
`03-user-facing-ux-extension.md` (companion). An index sits at the
end of this file.

---

## Spiral current state (grounding)

- `spiral-ui` (`crates/spiral-ui/src/lib.rs`) — only a `BrowserUi` struct
  with a `Vec<Tab>` and a `url_bar: String`. No render, no event loop
  binding, no chrome.
- `spiral-theme` (`crates/spiral-theme/src/lib.rs`) — `ThemeMode` enum
  (Light / Dark / System) and `ThemeTokens` struct. Not wired into
  `spiral-ui` or `spiral-browser`.
- `spiral-browser` (`crates/spiral-browser/src/`) — `BrowserShell` with
  `open_tab()` / `activate_tab()`, a `TabRegistry` (single active tab,
  no groups, no chrome), a `BrowserTheme` passthrough, and a software
  renderer that emits a hello-world PNG. No URL bar, no back/forward,
  no find, no downloads UI, no settings, no profiles.
- `specs/GAP_ANALYSIS.md` §3.3 (lines ~220–235) flags every chrome
  surface as `[ ]` (not started) except `spiral-theme` (Phase 4, do
  not touch) and `spiral-ui` winit scaffold (Phase 4, do not touch).
- Architecture bet: `docs/architecture/design/shared-everything.md`. Spiral
  ships a custom UI on a custom renderer; the chromium / firefox
  / webkit UI shells are not in scope as code dependencies, only as
  feature references for this matrix.

**Scoring rule applied throughout this chunk:** every row with a
`not-started` Spiral status is a candidate for the M5+ chrome backlog,
but the chunk **does not** scope them — synthesis does.

---

## Section 1 — Tab and window management

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 1 | Tab open, close, duplicate, reopen-last-closed | desktop+mobile | not-started | ubiquitous | P3 | S | User can open a new tab (Ctrl+T), close a tab (Ctrl+W), duplicate a tab (Ctrl+Shift+D), and reopen the most recently closed tab (Ctrl+Shift+T). Reopen-last restores tab in place and re-focuses it. | Chromium: yes/stable (native, 10-deep undo stack); Firefox: yes/stable (native, per-window stack); WebKit: yes/stable; Servo: partial (open/close, no duplicate, no undo stack); Ladybird: partial (open/close only); Flow: no. | [MDN Tabbed browsing](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs) ; [Chromium tab model](https://chromium.googlesource.com/chromium/src/+/main/docs/tabbed_browser.md) |
| 2 | Tab move (reorder, move to new window, tear-off) | desktop+mobile | not-started | ubiquitous | P3 | M | User can drag a tab to a new position in the strip, drag it out of the window to tear into a new window, and merge windows by dropping a tab into another window's strip. Mobile uses long-press + drag or drag-between-windows gesture. | Chromium: yes/stable (native + extensions API); Firefox: yes/stable; WebKit: yes/stable; Servo: partial (reorder only, no tear-off); Ladybird: no; Flow: no. | [WebExtensions `tabs.move`](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/move) |
| 3 | Tab pin, mute, freeze (process) | desktop+mobile | not-started | ubiquitous | P3 | M | Pinned tabs lock to the left of the strip at reduced width, survive close-all and reload, and are skipped during session restore of unpinned tabs. Mute silences tab audio without affecting other tabs. Freeze unloads the renderer process for a background tab; restore on focus. | Chromium: yes/stable (pin + mute + discard/freeze); Firefox: yes/stable (pin + mute, no freeze in stable, sleep-tabs is partial); WebKit: yes/stable (pin + mute); Servo: partial (pin only); Ladybird: no; Flow: no. | [Chromium tab discard](https://chromium.googlesource.com/chromium/src/+/main/docs/tab_discarding.md) |
| 4 | Tab groups (named, coloured, collapsible, persist) | desktop | not-started | widespread | P5 | L | User can select a contiguous range of tabs, "Group tabs", name the group, pick a colour, and collapse the group to a single header chip. Groups survive restart, can be closed as a unit, and persist ordering. | Chromium: yes/stable (`tabGroups` permission); Firefox: partial (named groups via containers + open tabs view, not collapsible strip); WebKit: yes/stable (Safari 15+); Servo: no; Ladybird: no; Flow: no. | [Chromium Tab Groups](https://chromium.googlesource.com/chromium/src/+/main/docs/tab_groups.md) ; [WebExtensions `tabGroups`](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabGroups) |
| 5 | Tab search (search by title/URL across open tabs) | desktop+mobile | not-started | widespread | P4 | M | Typing into the tab-search overlay filters visible tabs by title and URL with live highlight, supports fuzzy match, and Enter switches to the selected tab. Escape closes the overlay. | Chromium: yes/stable (native, fuzzy); Firefox: no (search via add-on or via `about:pages`); WebKit: yes/stable (Safari 16+); Servo: no; Ladybird: no; Flow: no. | [WebExtensions `tabs.query`](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/query) |
| 6 | Named tab set / workspace (switchable, persists) | desktop+mobile | not-started | niche | P6 | L | User can create a workspace, name it, switch via sidebar or shortcut, and have it remember its tabs. Workspaces are independent of profiles — they share login state. (Affects session restore; cookie jar is per-profile, not per-workspace.) | Chromium: partial (workspaces via third-party, "profile" is the in-engine equivalent); Firefox: partial (Multi-Account Containers as a proxy); WebKit: no (uses profiles instead); Servo: no; Ladybird: no; Flow: no. | [ARC Workspaces discussion](https://en.wikipedia.org/wiki/Arc_(browser)) ; see also [Firefox Containers](https://support.mozilla.org/en-US/kb/containers) |
| 7 | Profile-level tab separation (work vs personal, multiple sessions, cookie isolation) | desktop+mobile | not-started | widespread | P6 | XL | Browser supports ≥2 named profiles per installation. Each profile has its own cookie jar, history, bookmarks, and login sessions. Opening a link from one profile in another asks the user. Profiles share the binary, the download folder, the certificate store, and the system keychain. | Chromium: yes/stable (native multi-profile + "guest"); Firefox: yes/stable (Multi-Account Containers + profile manager); WebKit: no (single profile, private mode as escape hatch); Servo: no; Ladybird: no; Flow: no. | [Chromium multi-profile](https://chromium.googlesource.com/chromium/src/+/main/docs/linux_profiles.md) ; [Firefox profile manager](https://support.mozilla.org/en-US/kb/profile-manager-create-remove-switch-profiles) |
| 8 | Tab strip layouts — horizontal, vertical, tiled | desktop+mobile | not-started | widespread | P4 | M | User can switch the tab strip between horizontal (default) and vertical (sidebar) layouts. Vertical tabs show full title; horizontal shows favicon + truncated title. Tiled layout arranges a group of tabs in a 2-D grid. | Chromium: partial (extensions: vertical tabs via Tree Style Tab or native sidebar; tiled tabs in some forks); Firefox: yes/stable (native vertical tabs, no tiling); WebKit: partial (sidebar in Safari 17+); Servo: no; Ladybird: no; Flow: no. | [Firefox sidebar tabs](https://support.mozilla.org/en-US/kb/use-sidebar-tabs-organize-tabs) ; [WebExtensions `sidebarAction`](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/sidebarAction) |
| 9 | Tab preview on hover (image or text-only) | desktop | not-started | widespread | P4 | M | Hovering over a tab for 600ms shows a tooltip with the page title and (optionally) a thumbnail of the last-rendered frame. Click dismisses; Esc dismisses; thumbnail updates when the tab re-renders. | Chromium: yes/stable (thumbnail, configurable on/off); Firefox: yes/stable (text tooltip, no thumbnail in default); WebKit: yes/stable (text + thumbnail in Safari 17+); Servo: no; Ladybird: no; Flow: no. | [Chromium tab-tooltip-thumbnail](https://chromium-review.googlesource.com/c/chromium/src/+/3000000) |
| 10 | On-startup options: continue where you left off / new tab page / restore previous session | desktop+mobile | not-started | ubiquitous | P3 | M | User picks from three on-startup modes. "Continue" restores the exact tabs and scroll position from last quit. "New tab" always opens a blank tab. "Restore previous session" reopens the last browser session (single undo level). | Chromium: yes/stable (three modes, plus per-launch flag); Firefox: yes/stable (configurable, plus "Restore previous session" via history); WebKit: yes/stable; Servo: partial (no "previous session" without explicit save); Ladybird: partial; Flow: no. | [Chromium prefs::kRestoreOnStartup](https://source.chromium.org/chromium/chromium/src/+/main:components/prefs/pref_registry.cc) |

---

## Section 2 — Address bar (omnibox) and search

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 11 | URL typing, autocomplete, smart keyword detection (URL vs search) | desktop+mobile | not-started | ubiquitous | P3 | L | Typing into the address bar shows the typed string as a URL until the input is space-separated, then re-routes to the default search engine. Autocomplete expands "googl" → "google.com" for known hosts. Smart routing respects TLDs, schemes, and host-only tokens. | Chromium: yes/stable (omnibox); Firefox: yes/stable (urlbar classifier); WebKit: yes/stable; Servo: partial (basic classify); Ladybird: no; Flow: no. | [Chromium omnibox](https://chromium.googlesource.com/chromium/src/+/main/components/omnibox/) ; [Firefox urlbar classifier](https://searchfox.org/mozilla-central/source/browser/components/urlbar/UrlbarClassifier.jsm) |
| 12 | Suggestion sources — history, bookmarks, open tabs, top sites, search | desktop+mobile | not-started | ubiquitous | P3 | L | Address bar shows a multi-row suggestion list mixing history, bookmarks, open tabs, top sites, and search suggestions. Each row has favicon, title, URL, and a typed-highlighted match. Rows dedupe identical URLs from different sources. | Chromium: yes/stable (multi-provider); Firefox: yes/stable (multi-provider, adaptive history); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chromium omnibox providers](https://chromium.googlesource.com/chromium/src/+/main/components/omnibox/browser/autocomplete_provider.h) |
| 13 | Suggestion ranking, debouncing, type-ahead | desktop+mobile | not-started | ubiquitous | P3 | M | The first suggestion is the "default" — Enter activates it. Default ranking scores: visit count (recency × frequency), bookmark boost, search-suggestion boost, and exact-prefix match. Debounce is 30–50ms after the last keystroke. | Chromium: yes/stable (AC classifier, zero-prefix); Firefox: yes/stable (autocomplete tokens); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chromium ACMatches](https://chromium.googlesource.com/chromium/src/+/main/components/omnibox/browser/ac_provider.cc) |
| 14 | Custom search engines and keyword shortcuts (tab-to-search) | desktop+mobile | not-started | ubiquitous | P3 | M | User can register a custom search engine with a keyword (e.g. `w` for Wikipedia). Typing `w spiral browser` searches Wikipedia and skips the default search route. Tab-to-search: user types a site name, then Tab, and the bar switches to "Search <site>:" mode. | Chromium: yes/stable (custom search engines in `chrome://settings/searchEngines`); Firefox: yes/stable (keyword bookmarks + Tab-to-Search); WebKit: yes/stable (Smart Search field); Servo: no; Ladybird: no; Flow: no. | [Chrome search engines](https://developer.chrome.com/docs/extensions/reference/api/search) ; [Firefox keyword bookmarks](https://support.mozilla.org/en-US/kb/how-search-from-address-bar) |
| 15 | Search engine switching (default, per-region, per-search) | desktop+mobile | not-started | ubiquitous | P3 | S | User can change the default search engine from a list, and per-region locale can offer different defaults on first run. Some browsers expose a "this search" submenu in the bar to switch engines for a single query. | Chromium: yes/stable (default + per-region + per-extension override); Firefox: yes/stable (default + per-region; per-search engine switcher in bar); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chrome default search](https://www.chromium.org/developers/design-documents/default-search-engine) |
| 16 | Search leakage mitigations (preconnect-on-focus, no DNS resolution on keystroke) | desktop+mobile | not-started | mixed | P5 | M | Browser does not perform DNS resolution or preconnect for keystrokes that will be re-routed to a search engine. The preconnect happens only when the user picks a URL completion. Some engines (DuckDuckGo et al.) call this out as a privacy feature. | Chromium: yes/stable (no preconnect on text classified as a search); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chrome preconnect-on-focus](https://chromium.googlesource.com/chromium/src/+/main/docs/security/search-leakage.md) |
| 17 | Quick-find bar (slash-key shortcut to find-in-page) | desktop | not-started | niche | P6 | S | Pressing `/` focuses a quick-find bar at the top of the viewport, with matches highlighted live, `n` for next, `N` for previous, Esc to dismiss. Vimium-style link hints (`f` to follow link by label) is an extension, not a core. | Chromium: partial (extensions: Vimium, ddgo.html-vim); Firefox: partial (extensions); WebKit: partial (Safari Quick Find on `/`); Servo: no; Ladybird: no; Flow: no. | [Vimium source](https://github.com/philc/vimium) ; not a W3C spec |

---

## Section 3 — Navigation

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 18 | Back, forward, reload, stop, home buttons | desktop+mobile | not-started | ubiquitous | P3 | S | Toolbar exposes Back, Forward, Reload/Stop (toggles on load), and Home. Back is disabled at the start of history; Forward disabled at the end. Home navigates to the configured home URL (about:blank by default). | Chromium: yes/stable (native); Firefox: yes/stable; WebKit: yes/stable; Servo: partial (back/forward only); Ladybird: partial (back/forward only); Flow: no. | [MDN History API](https://developer.mozilla.org/en-US/docs/Web/API/History_API) |
| 19 | Mouse back/forward buttons (button 3 / 4 / 5 / 6) | desktop | not-started | ubiquitous | P3 | S | Pressing mouse button 4 (or X1) navigates Back, button 5 (X2) navigates Forward. Button 3 is a generic context menu, not a navigation. Buttons are configurable to next-tab / previous-tab in some engines. | Chromium: yes/stable (native, configurable per OS); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [MouseEvent.button](https://w3c.github.io/uievents/#dom-mouseevent-button) |
| 20 | Pull-to-refresh (touch/embedded) | mobile+embedded | not-started | widespread | P4 | S | Pulling down from the top of the page reloads, with a visual indicator that releases to refresh. Disabled when at scroll position 0 in some engines to avoid clobbering scroll. | Chromium: yes/stable (mobile only); Firefox: yes/stable (mobile); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Pointer Events spec](https://www.w3.org/TR/pointerevents3/) |
| 21 | History (full view, search, by date, by site) | desktop+mobile | not-started | ubiquitous | P4 | L | History view shows every visited URL with timestamp. User can search (full-text), filter by date range, filter by host, sort by visit count, and delete individual entries or the whole list. Private-browsing entries are excluded. | Chromium: yes/stable (chrome://history); Firefox: yes/stable (Library → History); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [MDN history deletion](https://developer.mozilla.org/en-US/docs/Web/API/History) |
| 22 | Reopen closed tab (single + n-deep) | desktop+mobile | not-started | ubiquitous | P3 | S | Ctrl+Shift+T reopens the most recently closed tab; pressing it 10 times reopens the last 10. On mobile, the equivalent is in the tab-switcher "Recently closed" tray. | Chromium: yes/stable (10-deep); Firefox: yes/stable; WebKit: yes/stable (Safari "Reopen Last Closed Tab"); Servo: no; Ladybird: no; Flow: no. | [Chrome SessionRestore](https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/sessions/) |
| 23 | Recently closed tabs and windows (in history UI) | desktop+mobile | not-started | ubiquitous | P4 | S | History view has a "Recently closed" section listing closed tabs (with their original URL) and closed windows (with their full tab list). Click reopens. | Chromium: yes/stable (chrome://history → Recently closed); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Sessions API](https://developer.chrome.com/docs/extensions/reference/api/sessions) |
| 24 | URL bar hides/shows on scroll (mobile) | mobile+embedded | not-started | widespread | P5 | S | Scrolling down hides the URL bar; scrolling up reveals it. Hides are animated, not instant, and respect user accessibility preferences (prefers-reduced-motion). | Chromium: yes/stable (Chrome Android, WebView); Firefox: yes/stable; WebKit: yes/stable (iOS 16+); Servo: no; Ladybird: no; Flow: no. | [Viewport segments](https://www.w3.org/TR/viewport-segments/) |
| 25 | Swipe navigation (back/forward edge swipe) | mobile | not-started | widespread | P5 | S | Swiping in from the left edge of the viewport navigates Back; right edge navigates Forward. Swipe must be at least 30% of viewport width to trigger, with a live preview. | Chromium: yes/stable (Chrome Android); Firefox: yes/stable; WebKit: yes/stable (iOS Safari); Servo: no; Ladybird: no; Flow: no. | [Edge-swipe gestures](https://developer.apple.com/design/human-interface-guidelines/swipe-actions) |
| 26 | Home button (configurable, multi-home, URL list) | desktop+mobile | not-started | widespread | P3 | S | The Home button navigates to a configured URL (or a list of URLs opened as separate tabs). Configuration is in `chrome://settings/onStartup` for Chromium, `about:preferences#home` for Firefox. | Chromium: yes/stable (single URL, also controls on-startup); Firefox: yes/stable (multi-URL home); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chromium prefs::kHomePage](https://source.chromium.org/chromium/chromium/src/+/main:components/prefs/pref_registry.cc) |

---

## Section 4 — Bookmarks and saved items

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 27 | Bookmark, bookmark folder, bookmark manager UI | desktop+mobile | not-started | ubiquitous | P3 | L | User can bookmark the current page (Ctrl+D), pick a folder, and manage all bookmarks in a tree UI (create/rename/delete folders, drag to reorder, search, filter). | Chromium: yes/stable (chrome://bookmarks); Firefox: yes/stable (Library); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [WebExtensions `bookmarks`](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/bookmarks) |
| 28 | Bookmark bar (always-visible strip) | desktop | not-started | widespread | P3 | S | An optional always-visible strip below the URL bar shows the top-level bookmark folder contents as favicon-only buttons. Toggleable. On mobile, the bookmark bar moves into a sheet that opens from the toolbar. | Chromium: yes/stable (Bookmarks Bar); Firefox: yes/stable (Bookmarks Toolbar); WebKit: yes/stable (Favorites Bar); Servo: no; Ladybird: no; Flow: no. | [Chrome bookmarks bar](https://support.google.com/chrome/answer/95725) |
| 29 | Bookmark import / export (HTML format, JSON) | desktop+mobile | not-started | ubiquitous | P3 | S | User can export the entire bookmark tree to a Netscape-format HTML file, and import from a Netscape-format HTML file. Round-trip preserves folder structure. | Chromium: yes/stable (HTML only, no JSON); Firefox: yes/stable (HTML + JSON via backup); WebKit: yes/stable (HTML); Servo: no; Ladybird: no; Flow: no. | [Netscape bookmark format](https://learn.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa753582(v=vs.85)) |
| 30 | Synced bookmarks (cross-device, cloud-mediated) | desktop+mobile | not-started | widespread | P6 | XL | Bookmarks sync across all devices on the same account. Sync is end-to-end encrypted (key derived from user passphrase or device-bound). On-device encryption is optional, default for the credential subset. | Chromium: yes/stable (Sync, E2EE optional); Firefox: yes/stable (Sync, E2EE on by default for credentials); WebKit: yes/stable (iCloud Tabs + Safari Bookmarks); Servo: no; Ladybird: no; Flow: no. | [Firefox Sync cryptography](https://hacks.mozilla.org/2018/11/firefox-sync-privacy/) |
| 31 | Reading list / saved-for-later (article-specific, offline cache) | desktop+mobile | not-started | widespread | P5 | L | User can save an article to a reading list from the share menu or a keyboard shortcut. Saved items are cached for offline reading, retain their layout (with reader mode applied), and show read/unread state. | Chromium: partial (Reading List in iOS Safari build, Chrome on macOS in 2024+); Firefox: yes/stable (Pocket integration; native reading list on iOS); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Reading List API proposal](https://github.com/WICG/reading-list) |
| 32 | Highlights and annotations (article-level) | desktop+mobile | not-started | niche | P6 | L | User can highlight a text range in a saved article, add a note, and have the highlight + note sync across devices. Highlights are not a W3C spec — this is product-specific. | Chromium: no (third-party extensions only); Firefox: partial (Pocket annotations, deprecated 2025); WebKit: yes/stable (Safari Highlights, iOS 17+); Servo: no; Ladybird: no; Flow: no. | [Web Annotation spec](https://www.w3.org/TR/annotation-model/) (spec exists; browser coverage is product-specific) |

---

## Section 5 — Downloads

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 33 | Download manager UI (progress, pause, resume, cancel) | desktop+mobile | not-started | ubiquitous | P4 | L | A persistent UI shows in-progress downloads with progress, speed, ETA, and per-row Pause/Resume/Cancel. Open file, Open folder, and Remove from list are also per-row. | Chromium: yes/stable (native + downloads UI); Firefox: yes/stable (Library → Downloads); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [WebExtensions `downloads`](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/downloads) |
| 34 | Download history, clear on quit | desktop+mobile | not-started | ubiquitous | P4 | S | The download manager shows a history of completed and failed downloads. "Clear on quit" wipes the history (and optionally the file) at browser exit. | Chromium: yes/stable (configurable); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #33 |
| 35 | Download location (default + per-format) | desktop+mobile | not-started | widespread | P4 | S | User configures a default download directory. Optional per-extension (e.g. `*.pdf` → `~/Documents`); on Windows, per-association. Mobile: scoped to the app sandbox by default. | Chromium: yes/stable (default + per-extension rule); Firefox: yes/stable (default, per-MIME via `browser.downloads`); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Firefox downloads prefs](https://support.mozilla.org/en-US/kb/change-where-downloads-are-saved) |
| 36 | "Ask where to save" toggle | desktop | not-started | widespread | P4 | S | When on, every download prompts for a destination. When off, downloads go to the default location without prompting. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #33 |
| 37 | Safe Browsing check on download, dangerous file type warning | desktop+mobile | not-started | ubiquitous | P4 | M | Browser sends a hash of the downloaded file (and the URL) to a Safe-Browsing-style service; on hit, blocks with a full-page warning. Local heuristic check flags `.exe` and other high-risk extensions. | Chromium: yes/stable (Safe Browsing, Enhanced Protection); Firefox: yes/stable (Google Safe Browsing); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Google Safe Browsing API](https://developers.google.com/safe-browsing/v4) |
| 38 | Resumable downloads (HTTP `Range`) | desktop+mobile | not-started | widespread | P4 | M | If a download is interrupted (network drop, browser crash), the next attempt resumes from the byte position the server advertises via `Content-Range` / `Accept-Ranges: bytes`. UI shows "Resume" affordance. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [RFC 9110 §14.1.2 Range](https://www.rfc-editor.org/rfc/rfc9110#name-range) |
| 39 | "Always open this file type" association | desktop | not-started | widespread | P4 | S | After completing a download of a given MIME type, the manager offers a "Always open files of this type" checkbox. Choice is persisted per-OS-user. | Chromium: yes/stable; Firefox: yes/stable (configurable, plus per-MIME `network.protocol-handler`); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Firefox applications](https://support.mozilla.org/en-US/kb/change-firefox-behavior-when-open-file) |
| 40 | Download notifications (system-level) | desktop+mobile | not-started | ubiquitous | P4 | S | On completion, a system notification appears with Open and Reveal-in-folder actions. Clicking the notification opens the file. On mobile, the notification is a foreground service. | Chromium: yes/stable (native); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Notifications spec](https://notifications.spec.whatwg.org/) |

---

## Section 6 — Find-in-page and text tools

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 41 | Find-in-page (Ctrl/Cmd+F), case-sensitive, whole-word, regex | desktop+mobile | not-started | ubiquitous | P3 | M | Ctrl+F opens a find bar; matches highlight in real time as the user types; status shows `n of N` matches; Enter / F3 advances, Shift+Enter / Shift+F3 retreats; case-sensitive, whole-word, and regex toggles exist; Escape closes the bar and clears the highlight. | Chromium: yes/stable (Ctrl+F, F3, native); Firefox: yes/stable (Ctrl+F, F3, native, with quick-find `/`); WebKit: yes/stable; Servo: partial (basic Ctrl+F, no regex); Ladybird: partial (basic Ctrl+F, no regex, no quick-find); Flow: no. | [Firefox find bar](https://searchfox.org/mozilla-central/source/toolkit/components/typeaheadfind/) |
| 42 | Text zoom / page zoom (per-site, default, full-page vs text-only) | desktop+mobile | not-started | ubiquitous | P3 | M | Ctrl+/Ctrl- scales the page (default 100%, range 25–500%); Ctrl+0 resets. Per-site zoom is remembered. Text-only zoom is exposed as a separate option in accessibility settings. | Chromium: yes/stable (full-page zoom); Firefox: yes/stable (full-page + text-only); WebKit: yes/stable; Servo: partial (full-page only); Ladybird: no; Flow: no. | [WCAG 1.4.4 Resize Text](https://www.w3.org/TR/WCAG22/#resize-text) |
| 43 | Force enable zoom (mobile) | mobile | not-started | widespread | P4 | S | On mobile, sites can disable pinch-zoom via the viewport meta tag. Browser-level setting can override the tag and re-enable zoom, including on sites that opted out. | Chromium: yes/stable (Android only); Firefox: yes/stable; WebKit: yes/stable (Safari 14+); Servo: no; Ladybird: no; Flow: no. | [Visual Viewport API](https://www.w3.org/TR/visual-viewport/) |
| 44 | Reader mode (auto-detect, manual toggle, themable, save offline) | desktop+mobile | not-started | widespread | P5 | L | A button in the URL bar toggles reader mode, which strips chrome, ads, and sidebars, leaving a single-column text view with adjustable font, size, line-height, and theme. Some engines auto-suggest reader mode on article-shaped pages. | Chromium: partial (DOM Distiller in Android, reader mode in Chrome desktop behind flag); Firefox: yes/stable (Reader View, themable); WebKit: yes/stable (Safari Reader); Servo: no; Ladybird: no; Flow: no. | [DOM Distiller](https://chromium.googlesource.com/chromium/src/+/main/components/dom_distiller/) |
| 45 | Page translation (auto-detect, engine choice) | desktop+mobile | not-started | widespread | P6 | XL | User can translate the current page to a target language, with engine choice. The page's content is replaced in-place (no sidebar). Input-field translation is a separate, opt-in setting. | Chromium: yes/stable (Google Translate, in-place); Firefox: yes/stable (translation, via partner engines); WebKit: yes/stable (Safari Translation, iOS 14+); Servo: no; Ladybird: no; Flow: no. | [Translator API proposal](https://github.com/WICG/translator-api) (W3C draft 2024–2025, not yet in shipping engines) |
| 46 | Dictionary lookup (selection → definition) | desktop+mobile | not-started | niche | P6 | M | Selecting a single word and pressing a shortcut (or selecting from the text toolbar) shows a small popover with a definition from a built-in or partner dictionary. | Chromium: partial (macOS only, system dictionary); Firefox: yes/stable (dictionary on selection); WebKit: yes/stable (Look Up, system integration); Servo: no; Ladybird: no; Flow: no. | [Wiktionary API](https://en.wiktionary.org/w/api.php) (third-party) |
| 47 | Link preview / hover preview (on hover, in a popover) | desktop | not-started | niche | P6 | M | Hovering a link (300ms+) opens a small popover with the destination's preview — title, favicon, first paragraph, and a small thumbnail. Clicking the popover navigates in the background tab. | Chromium: partial (extensions); Firefox: partial (extensions); WebKit: no; Servo: no; Ladybird: no; Flow: no. | no W3C spec |

---

## Section 7 — Privacy and tracking controls (UI surface)

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 48 | URL-pattern permission scopes (per-site: camera, mic, location, notifications, etc.) | desktop+mobile | not-started | ubiquitous | P3 | L | A central site-permissions UI shows every granted permission, grouped by site, with revoke / always-allow / always-deny per origin. Permissions are inherited from the URL-pattern root, not the registrable domain alone. | Chromium: yes/stable (chrome://settings/content); Firefox: yes/stable (about:preferences#privacy); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Permissions spec](https://www.w3.org/TR/permissions/) ; [MDN Permissions API](https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API) |
| 49 | Per-site content settings (JS, images, cookies, popups, redirects, ads, fingerprinting, autoplay) | desktop+mobile | not-started | ubiquitous | P3 | XL | A site-settings UI lets the user block JavaScript, images, cookies, popups, redirects, autoplay, fingerprinting, and tracking ads — globally and per-site. Per-site overrides are visible from the address bar (lock icon → site settings). | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chromium content settings](https://chromium.googlesource.com/chromium/src/+/main/components/content_settings/core/browser/content_settings_registry.h) |
| 50 | Per-site content filter (in-bar toggle for tracker/ads blocking) | desktop+mobile | not-started | widespread | P4 | L | A toolbar icon shows the count of blocked trackers/ads on the current page. Clicking the icon opens a panel with toggle, breakdown by category, and a per-site exception. Toggling reloads the tab. | Chromium: partial (extensions only, e.g. uBlock Origin); Firefox: yes/stable (Enhanced Tracking Protection, icon + counter); WebKit: yes/stable (Privacy Report); Servo: no; Ladybird: no; Flow: no. | [Firefox ETP](https://support.mozilla.org/en-US/kb/enhanced-tracking-protection-firefox-desktop) ; [privacytests.org](https://privacytests.org) |
| 51 | Global Privacy Control (GPC header) | desktop+mobile | not-started | widespread | P3 | S | Browser sends `Sec-GPC: 1` on all requests when GPC is enabled. The header is signalled to JS via `navigator.globalPrivacyControl`. Default on in some engines. | Chromium: partial (GPC respected but not signalled by default until 2024+); Firefox: yes/stable (default off, opt-in); WebKit: yes/stable (default on); Servo: no; Ladybird: no; Flow: no. | [W3C GPC spec](https://www.w3.org/TR/global-privacy-control/) |
| 52 | Do Not Track (DNT) — legacy / deprecated | desktop | not-started | mixed | P6 | S | Browser sends `DNT: 1` when enabled. Spec is deprecated; engines now treat it as a hint only. (Row kept for parity; should not be a new build target.) | Chromium: no (removed 2019, intent-to-deprecate 2020); Firefox: no (removed 2024); WebKit: no (never sent); Servo: no; Ladybird: no; Flow: no. | [EFF on DNT](https://www.eff.org/issues/do-not-track) ; [Mozilla DNT removal](https://blog.mozilla.org/security/2024/06/dnt/) |
| 53 | Cookie banner blocking / auto-dismiss | desktop+mobile | not-started | widespread | P4 | L | Browser detects cookie-consent banners (via heuristics or a built-in list) and dismisses them automatically. The user sees the option to re-show and to opt out per-site. | Chromium: partial (Chrome 120+ in some regions); Firefox: no; WebKit: partial (Safari "Hide Distracting Items" + third-party extensions); Servo: no; Ladybird: no; Flow: no. | [DuckDuckGo Cookie Pop-ups](https://duckduckgo.com/cookie-consent-blocking) ; [privacytests.org](https://privacytests.org) |
| 54 | HTTPS-only mode (block HTTP, warn on mixed content) | desktop+mobile | not-started | widespread | P3 | M | When on, every HTTP request is upgraded to HTTPS; if the site has no HTTPS, the user sees a full-page block. Mixed content is blocked by default at the page level. | Chromium: yes/stable (HTTPS-First Mode); Firefox: yes/stable (HTTPS-Only Mode); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [MDN upgrade-insecure-requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Upgrade-Insecure-Requests) ; [Mixed Content spec](https://www.w3.org/TR/mixed-content/) |
| 55 | Connection secure/insecure indicator in the address bar | desktop+mobile | not-started | ubiquitous | P3 | S | The address bar shows a lock icon for HTTPS, a "Not secure" label for HTTP, and a red strikethrough for invalid certificate. Clicking the lock opens the certificate viewer. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chromium EV UI deprecation](https://security.googleblog.com/2019/05/no-more-mixed-messages-about-https.html) |
| 56 | Certificate viewer, TLS details | desktop+mobile | not-started | ubiquitous | P4 | M | Clicking the lock icon opens a panel with the certificate chain, subject, issuer, validity dates, key algorithm, and TLS version + cipher. User can export the certificate. | Chromium: yes/stable (`chrome://settings/certificates`); Firefox: yes/stable (`about:certificate`); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [RFC 8446 TLS 1.3](https://www.rfc-editor.org/rfc/rfc8446) |
| 57 | Phishing / malware warning page | desktop+mobile | not-started | ubiquitous | P3 | M | When the user navigates to a known phishing or malware URL, the browser shows a full-page red interstitial with a "Back to safety" button and a "Proceed anyway" link (after typing the phrase). | Chromium: yes/stable (Safe Browsing); Firefox: yes/stable (Google Safe Browsing); WebKit: yes/stable (Fraudulent Website Warning); Servo: no; Ladybird: no; Flow: no. | [Google Safe Browsing API](https://developers.google.com/safe-browsing/v4) |
| 58 | Deceptive site warning (lookalike URL) | desktop+mobile | not-started | widespread | P4 | M | Browser flags domains that look like a popular brand (IDN homograph attacks, typosquats) and shows a full-page warning. | Chromium: yes/stable (Chrome 75+); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chromium lookalike URLs](https://chromium.googlesource.com/chromium/src/+/main/components/lookalikes/) |
| 59 | Private browsing mode (entry, exit, what is cleared, what is not) | desktop+mobile | not-started | ubiquitous | P4 | L | Opening a private window shows a "you are in private mode" splash; on exit, all cookies, history, cache, form data, and local storage for that window are cleared. Bookmarks, saved passwords (if user opts in), and OS-level download files persist. | Chromium: yes/stable (Incognito); Firefox: yes/stable (Private Window, includes tracking protection); WebKit: yes/stable (Private Browsing, with per-window ephemeral storage); Servo: no; Ladybird: no; Flow: no. | [MDN Storage Access API](https://developer.mozilla.org/en-US/docs/Web/API/Storage_Access_API) (cross-ref for non-private ephemerals) |

---

## Section 8 — Passwords and autofill

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 60 | Built-in password manager (save, autofill, generate, audit) | desktop+mobile | not-started | ubiquitous | P4 | XL | Browser detects a sign-up / sign-in form, offers to save the password, and on next visit, autofills. Built-in password generator creates 16+ char passwords with mixed classes. Password audit flags reused, weak, and breached passwords. | Chromium: yes/stable (Password Manager + Google Password Manager backend); Firefox: yes/stable (Lockwise + Sync); WebKit: yes/stable (iCloud Keychain, password monitoring); Servo: no; Ladybird: no; Flow: no. | [MDN Credential Management](https://developer.mozilla.org/en-US/docs/Web/API/Credential_Management_API) |
| 61 | Cross-device password sync (E2EE) | desktop+mobile | not-started | widespread | P6 | XL | Saved passwords sync across all devices on the same account. The credential subset is end-to-end encrypted with a key derived from a user passphrase or device-bound recovery key. | Chromium: yes/stable (E2EE optional); Firefox: yes/stable (E2EE on by default); WebKit: yes/stable (iCloud Keychain E2EE); Servo: no; Ladybird: no; Flow: no. | [Firefox Sync E2EE](https://hacks.mozilla.org/2018/11/firefox-sync-privacy/) |
| 62 | Password breach detection (Have I Been Pwned integration) | desktop+mobile | not-started | widespread | P5 | M | On every sign-in, the browser hashes the password (k-anonymity, prefix-only) and checks the prefix against a known-breach corpus. On hit, shows a warning. Bulk audit on the passwords page flags all breached entries. | Chromium: yes/stable (Google Password Manager); Firefox: yes/stable (Firefox Monitor, integrated into the password manager); WebKit: yes/stable (iCloud Keychain compromised-password detection); Servo: no; Ladybird: no; Flow: no. | [HIBP Pwned Passwords API](https://haveibeenpwned.com/API/v3#PwnedPasswords) |
| 63 | Two-factor code autofill (SMS / TOTP) | desktop+mobile | not-started | widespread | P5 | L | On a sign-in form with a 2FA code field, the browser offers the most-recently-copied code from SMS or a TOTP app. Autofill is one-tap; codes auto-clear after 60 seconds. | Chromium: yes/stable (Android only, SMS code detection); Firefox: partial (extensions); WebKit: yes/stable (iOS 15+ SMS code autofill); Servo: no; Ladybird: no; Flow: no. | [Web OTP API](https://wicg.github.io/web-otp/) |
| 64 | Address, payment, identity autofill | desktop+mobile | not-started | widespread | P5 | XL | A single user profile (name, address, email, phone, organisation) autofills all matching fields on checkout / sign-up forms. Credit card autofill shows the last 4 digits and requires CVV. | Chromium: yes/stable (Autofill / Google Pay); Firefox: yes/stable (`formautofill` pref); WebKit: yes/stable (AutoFill); Servo: no; Ladybird: no; Flow: no. | [WHATWG Autofill §credit-card](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#autofill-detail) ; [Payment Request spec](https://www.w3.org/TR/payment-request/) |
| 65 | Passkeys (WebAuthn, discoverable credentials) | desktop+mobile | not-started | widespread | P5 | L | Browser offers to create a passkey on sign-up and to use an existing passkey on sign-in. The passkey is synced via the OS keychain (iCloud, Google Password Manager, or third-party password manager) and is bound to the origin. | Chromium: yes/stable; Firefox: yes/stable (sync via Firefox Relay + Password Manager); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [WebAuthn spec](https://www.w3.org/TR/webauthn-2/) |
| 66 | Federated sign-in (sign-in with identity provider) | desktop+mobile | not-started | widespread | P5 | L | Browser shows a chooser for "Sign in with…" identity providers on sites that opt in. The chooser integrates with the OS keychain so the user does not leave the page. | Chromium: yes/stable (FedCM, Identity Credentials API); Firefox: yes/stable (FedCM, partial); WebKit: yes/stable (FedCM); Servo: no; Ladybird: no; Flow: no. | [FedCM spec](https://fedidcg.github.io/FedCM/) ; [WebAuthn §identity](https://www.w3.org/TR/webauthn-2/#sctn-credential-privacy) |

---

## Cross-refs to `specs/GAP_ANALYSIS.md` (sections 1–8)

| GAP_ANALYSIS row | Section | Line |
|---|---|---|
| `spiral-theme` design tokens (Zen-style) | §3.3 Presentation Layer | 222 |
| `spiral-ui` winit window / event loop | §3.3 Presentation Layer | 223 |
| Sidebar tabs (create, switch, close, drag) | §3.3 Presentation Layer | 224 |
| Floating URL bar / Omnibox | §3.3 Presentation Layer | 225 |
| Navigation buttons (back/forward/reload/home) | §3.3 Presentation Layer | 226 |
| Tab context menu | §3.3 Presentation Layer | 227 |
| Find-in-page | §3.3 Presentation Layer | 230 |
| Downloads UI / manager | §3.3 Presentation Layer | 231 |
| Settings panel | §3.3 Presentation Layer | 232 |
| WebExtensions API (manifest v3) | §3.3 Presentation Layer | 233 |
| Content script injection (sandboxed) | §3.3 Presentation Layer | 234 |
| Extensions ↔ page typed message bus | §3.3 Presentation Layer | 235 |

---

## Open questions for the user (sections 1–8)

1. **Tab groups row (#4) Phase assignment.** GAP_ANALYSIS lists tab
   groups as out-of-scope, but the user-task ("tab groups, named, coloured,
   collapsible") appears in section 1 with P5/L. Is the P5 placement
   correct, or should tab groups ship as part of the M5 chrome pass
   (i.e. P3)?

2. **Password manager Phase 4 vs Phase 6.** P4 (current) for
   single-device save/autofill, P6 for sync. The M4.4 IPC work is
   prerequisite to any secure storage. Should the single-device manager
   be M5-scoped, or deferred to align with the password sync roadmap?

3. **Reader mode (#44) is a P5/L cost.** Reader mode requires
   article-extraction heuristics (Chromium's DOM Distiller or
   Mozilla's Readability). This is non-trivial. Is the user OK with
   a third-party Readability port, or do we need a Spiral-native
   extractor?

4. **Private browsing (#59) and the shared-everything bet.** Spiral's
   architecture bet is shared-everything multi-process. Private mode
   needs ephemeral storage that does not share a cache key with the
   default profile. Does the bet already support this, or is private
   mode blocked on architecture?

5. **GPC (#51) default-on vs default-off.** W3C says engines "MAY send";
   Safari ships default-on, Firefox default-off, Chromium default-off.
   Which side does Spiral take?

6. **Cookie banner blocking (#53).** Chromium is rolling this in
   regionally in 2024–2026. This requires a per-region ruleset
   (`duckduckgo/cookie-consent-blocking`-style). Is Spiral willing to
   ship a partner-rendered list, or is this a Phase 6+ item?

---

## Index — companion file

Sections 9–16 live in `03-user-facing-ux-extension.md`:

- 9. Sharing and integration
- 10. Reading, viewing, media UI
- 11. Customisation and ergonomics
- 12. Sync and account
- 13. Settings and configuration
- 14. Crash handling and diagnostics
- 15. Telemetry and feedback
- 16. Help and onboarding
