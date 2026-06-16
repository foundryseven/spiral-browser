# Chunk 7 — User-Facing UX (Deep) — Extension (sections 9–16)

> **Companion to `03-user-facing-ux.md`.** Sections 9–16 of the chunk
> live in this file. Index and cross-references are in the main file.

---

## Section 9 — Sharing and integration

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 67 | Native share (OS share sheet, mobile) | mobile+embedded+desktop | not-started | ubiquitous | P4 | S | Tapping Share on a page opens the OS share sheet with the page URL, title, and selected text. On desktop, the equivalent is a submenu of installed share targets. | Chromium: yes/stable (Web Share API, mobile + desktop in 2024+); Firefox: partial (mobile only); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Web Share API](https://w3c.github.io/web-share/) |
| 68 | Send-tab-to-self (cross-device) | desktop+mobile | not-started | widespread | P6 | L | User clicks "Send to device" in the toolbar menu, picks a signed-in device, and the tab is pushed to that device's sync queue. On the target device, a notification offers to open it. | Chromium: yes/stable (native, requires Chrome Sync); Firefox: yes/stable (Send Tab); WebKit: yes/stable (iCloud Handoff); Servo: no; Ladybird: no; Flow: no. | [Chrome sync send-tab](https://support.google.com/chrome/answer/165139) |
| 69 | QR code generation for URL | desktop+mobile | not-started | widespread | P4 | S | The toolbar share menu has a "Create QR code" entry. The QR encodes the current page URL (or selected text) and can be saved as PNG. | Chromium: yes/stable (desktop, mobile); Firefox: no (third-party); WebKit: yes/stable (iOS share sheet); Servo: no; Ladybird: no; Flow: no. | no W3C spec — feature is product-specific |
| 70 | QR code scanner (camera) | mobile+embedded | not-started | widespread | P5 | M | The URL bar (or a share affordance) can launch the camera to scan a QR code. On a hit, the user is asked whether to open the URL. | Chromium: yes/stable (Android); Firefox: no; WebKit: yes/stable (iOS); Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 71 | Cast / AirPlay (presentation of page to TV) | desktop+mobile | not-started | widespread | P5 | L | User can cast the current tab to a Cast-capable or AirPlay-capable display. The cast surface mirrors the rendered tab, not the URL. | Chromium: yes/stable (Cast); Firefox: no (third-party); WebKit: yes/stable (AirPlay); Servo: no; Ladybird: no; Flow: no. | [Remote Playback API](https://www.w3.org/TR/remote-playback/) |
| 72 | Tab sharing with another user (live link) | desktop+mobile | not-started | niche | P6 | L | User can generate a shareable link to a specific open tab; the link, when opened, navigates to the same URL with a hint that the originator is on the page. Two-way collaboration (cursors, presence) is product-specific and not a W3C spec. | Chromium: partial (Google Docs-style share tab in Workspace; native browser-level sharing in some forks); Firefox: partial (Firefox Notes / Screenshots, deprecated for tabs); WebKit: no; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 73 | Email-link, message-link, copy-link | desktop+mobile | not-started | ubiquitous | P4 | S | Standard share-menu entries: "Email link" (opens default mail client with the URL in the body), "Message link" (opens default messenger with the URL), "Copy link" (puts URL on the clipboard). | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #67 |

---

## Section 10 — Reading, viewing, media UI

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 74 | PDF viewer (built-in vs plugin) | desktop+mobile | not-started | widespread | P5 | L | The browser opens a PDF inline (in a tab), with page navigation, zoom, search, print, and save. No plugin is downloaded. PDF.js is the standard open-source viewer; Chromium ships a native one. | Chromium: yes/stable (native PDFium-based viewer); Firefox: yes/stable (PDF.js); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [PDF.js](https://mozilla.github.io/pdf.js/) ; [Chromium PDFium](https://pdfium.googlesource.com/pdfium/) |
| 75 | Image viewer (full-size, zoom, pan, rotate, EXIF strip, save) | desktop+mobile | not-started | ubiquitous | P4 | M | Clicking an image on a page (or opening an image URL directly) shows the image full-size with mouse-wheel zoom, click-and-drag pan, rotation (90° steps), and an "open externally" / "save as" option. EXIF data is shown but not by default. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 76 | Video player UI (chapters, subtitles, fullscreen, PiP, cast) | desktop+mobile | not-started | ubiquitous | P4 | L | The HTMLMediaElement surface gives the site a default chrome: play/pause, scrub, volume, fullscreen, PiP, captions toggle, chapters (if present in WebVTT), and cast. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [HTMLMediaElement spec](https://html.spec.whatwg.org/multipage/media.html#htmlmediaelement) ; [Picture-in-Picture spec](https://wicg.github.io/picture-in-picture/) ; [WebVTT spec](https://www.w3.org/TR/webvtt1/) |
| 77 | In-tab media controls (audio playback UI on tab strip) | desktop+mobile | not-started | widespread | P4 | S | An audio icon appears on tabs with playing audio, with a click-to-mute. Some engines (Safari, Firefox) show a small overlay with play/pause/scrub on hover. | Chromium: yes/stable (tab audio icon); Firefox: yes/stable (tab overlay); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [MediaSession spec](https://www.w3.org/TR/mediasession/) |
| 78 | Global media controls (lock-screen, OS-level, browser toolbar) | desktop+mobile | not-started | widespread | P4 | L | When a tab is in the background, media playback is still controllable from a global key (play/pause/next/prev) and from the OS media widget (lock-screen on mobile, system tray on desktop). The web page registers intent via `MediaSession`. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [MediaSession spec](https://www.w3.org/TR/mediasession/) |
| 79 | Subtitle / caption styling, language picker | desktop+mobile | not-started | widespread | P4 | M | When a `<track>` element is present, the player UI exposes a CC button and a language picker. Caption styling (font, size, colour, background) is controllable from the page via `::cue` CSS or from the browser's accessibility settings. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [WebVTT spec](https://www.w3.org/TR/webvtt1/) ; [CSS `::cue`](https://www.w3.org/TR/css-pseudo-4/#cue-pseudo) |
| 80 | Dark mode / forced colours (auto, system, on, off; per-site override) | desktop+mobile | not-started | widespread | P3 | M | Browser chrome can follow the OS theme (System mode), be forced Light, or forced Dark. A per-site override lets the user lock a specific site to Light or Dark regardless of theme. | Chromium: yes/stable (auto, light, dark, per-site via DevTools); Firefox: yes/stable (auto, light, dark, no per-site); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [prefers-color-scheme](https://www.w3.org/TR/mediaqueries-5/#prefers-color-scheme) ; [forced-colors](https://www.w3.org/TR/css-color-adjust-1/#forced-colors) |
| 81 | Site theme colour (browser chrome reflects `<meta name="theme-color">`) | mobile+embedded+desktop | not-started | widespread | P4 | S | When a page declares `<meta name="theme-color">` (with `media` query support), the browser chrome (URL bar, status bar on mobile) takes that colour. | Chromium: yes/stable; Firefox: yes/stable (Android, Theme API); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [HTML §meta-theme-color](https://html.spec.whatwg.org/multipage/semantics.html#meta-theme-color) |

---

## Section 11 — Customisation and ergonomics

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 82 | Home page, new tab page (background, layout, widgets, search box placement) | desktop+mobile | not-started | ubiquitous | P3 | L | The new-tab page is configurable: background image, layout (grid of top sites, list of recently closed, news feed if shipped), search box placement (centre, top), and optional widgets. The home button navigates to a configurable URL. | Chromium: yes/stable (custom NTP via extensions, NTP Customize); Firefox: yes/stable (`about:newtab` configurable); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chrome NTP docs](https://developer.chrome.com/docs/extensions/reference/api/override) |
| 83 | Custom themes (background image, dark/light, accent colour, font) | desktop+mobile | not-started | widespread | P4 | L | User can pick a built-in theme or load a custom CSS. Spiral's `spiral-theme` is the design-token surface for this. Themes are switchable at runtime and persist per profile. | Chromium: partial (Chrome themes, limited to colours + background); Firefox: yes/stable (full themes + colour rules); WebKit: no; Servo: no; Ladybird: no; Flow: no. | [Chrome themes format](https://developer.chrome.com/docs/extensions/reference/api/theme) ; [MDN theme API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/theme) |
| 84 | Toolbar customisation (drag-and-drop buttons) | desktop+mobile | not-started | widespread | P4 | M | User can drag toolbar buttons between visible, overflow, and hidden sets. Some engines support pinning buttons to the toolbar permanently. | Chromium: yes/stable (custom toolbar, drag-drop); Firefox: yes/stable (customise toolbar); WebKit: partial (Safari 17+); Servo: no; Ladybird: no; Flow: no. | [Chrome custom toolbar](https://support.google.com/chrome/answer/95454) |
| 85 | Keyboard shortcut customisation | desktop+mobile | not-started | widespread | P4 | M | User can rebind any keyboard shortcut in `chrome://extensions/shortcuts` (Chromium) or `about:preferences#general` → Keyboard Shortcuts (Firefox). Conflicts are detected and warned. | Chromium: yes/stable (limited, no override of all built-ins); Firefox: yes/stable (extension shortcuts; some built-in rebindable); WebKit: no; Servo: no; Ladybird: no; Flow: no. | [WebExtensions commands](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/commands) |
| 86 | Mouse gesture customisation | desktop | not-started | niche | P6 | M | User can record a mouse gesture (right-button drag in a pattern) and bind it to an action (Back, Forward, Close tab, New tab, etc.). Common in browsers with power-user roots. | Chromium: partial (extensions: Gesturefy, smoothGestures); Firefox: partial (extensions); WebKit: no; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 87 | Mouse-wheel-scroll tab switching | desktop | not-started | niche | P6 | S | Scrolling the mouse wheel over the tab strip cycles through tabs. (Mute / no-op is the alternative.) | Chromium: partial (extensions only); Firefox: partial (configurable via `about:config`); WebKit: no; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 88 | Sidebar (bookmarks, history, downloads, reading list, tabs list, notes) | desktop+mobile | not-started | widespread | P4 | L | A collapsible sidebar with tabs for each major data surface. On mobile, a swipe-in panel. | Chromium: yes/stable (sidebar in 2024+); Firefox: yes/stable (sidebar); WebKit: yes/stable (sidebar in Safari 17+); Servo: no; Ladybird: no; Flow: no. | [Chromium sidebar](https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/ui/views/side_panel/) |
| 89 | Vertical tabs | desktop | not-started | widespread | P4 | M | Tabs render in a vertical sidebar instead of the horizontal strip. Title is shown next to the favicon. | Chromium: yes/stable (native vertical tabs in 2024+); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Firefox sidebar tabs](https://support.mozilla.org/en-US/kb/use-sidebar-tabs-organize-tabs) |
| 90 | Tab hover preview (image preview on mouse hover) | desktop | not-started | widespread | P4 | M | Hovering a tab for 600ms shows a tooltip with title and (optionally) a thumbnail of the last-rendered frame. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | cross-ref with #9 |
| 91 | Session manager (save all open tabs as a named session, restore later) | desktop+mobile | not-started | widespread | P5 | M | User can "Save current tabs as session", name it, and restore the named session later. Sessions persist across restart. Multiple sessions can exist. | Chromium: partial (Session Buddy extension, native only for "continue where you left off"); Firefox: partial (Tab Session Manager extension, native partial); WebKit: no; Servo: no; Ladybird: no; Flow: no. | [WebExtensions sessions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/sessions) |
| 92 | Named tab set / workspace (capability-agnostic) | desktop+mobile | not-started | niche | P6 | L | User can create a named set of tabs (a "workspace"), switch between workspaces, and have each workspace remember its tabs. Workspaces share the cookie jar. | Chromium: partial (Workspaces via third-party, Profiles as in-engine equivalent); Firefox: partial (Multi-Account Containers as a proxy); WebKit: no; Servo: no; Ladybird: no; Flow: no. | cross-ref with #6 |
| 93 | Tidy tabs / auto-group similar tabs / suspend background tabs | desktop | not-started | niche | P6 | L | Browser detects tabs from the same origin and offers to group them. Background tabs that have not been focused for N minutes are unloaded from memory. | Chromium: yes/stable (Tab Groups auto-group in 2024+; tab discard); Firefox: partial (Tab Unloader extension); WebKit: yes/stable (Safari 17+ auto-group); Servo: no; Ladybird: no; Flow: no. | cross-ref with #3 (freeze) and #4 (groups) |
| 94 | Command palette (Ctrl/Cmd+Shift+P, type-to-search UI elements) | desktop | not-started | niche | P5 | M | Pressing Ctrl+Shift+P opens a command palette that lists every browser action, settings entry, and tab; user types to filter, Enter to invoke. | Chromium: yes/stable (Chrome command palette in 2024+); Firefox: no; WebKit: no; Servo: no; Ladybird: no; Flow: no. | no W3C spec |

---

## Section 12 — Sync and account

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 95 | Sign-in (browser-level account) | desktop+mobile | not-started | ubiquitous | P6 | L | Browser offers to sign in with a partner IdP. Sign-in enables cross-device sync. Sign-in is optional and does not block browsing. | Chromium: yes/stable; Firefox: yes/stable (Firefox Account); WebKit: yes/stable (Apple ID); Servo: no; Ladybird: no; Flow: no. | [WebExtensions identity](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/identity) |
| 96 | Sync (bookmarks, history, tabs, passwords, extensions, settings, open tabs, reading list) | desktop+mobile | not-started | widespread | P6 | XL | When signed in, the user can opt into sync of each data type. The sync server stores an opaque blob; the client decides which fields to include. | Chromium: yes/stable (granular per-data-type); Firefox: yes/stable; WebKit: yes/stable (granular per-data-type); Servo: no; Ladybird: no; Flow: no. | [Firefox Sync design](https://mozilla-services.readthedocs.io/en/latest/sync/) |
| 97 | End-to-end encryption for synced data | desktop+mobile | not-started | widespread | P6 | L | The credential subset (passwords, payment methods) is E2EE with a key derived from the user's passphrase (or a recovery key). Non-credential subsets are server-side encrypted. | Chromium: yes/stable (opt-in, passphrase-derived key); Firefox: yes/stable (E2EE on by default for credentials); WebKit: yes/stable (E2EE for iCloud Keychain); Servo: no; Ladybird: no; Flow: no. | cross-ref with #30, #61 |
| 98 | Send-tab-to-self | desktop+mobile | not-started | widespread | P6 | M | User can send a tab to another device on the same account. The target device shows a notification. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | cross-ref with #68 |
| 99 | Cross-device tab pickup (open this tab on phone) | desktop+mobile | not-started | widespread | P6 | M | On the desktop, the user can push the current tab to the signed-in phone. The phone's browser opens the URL. Reverse direction is also supported on mobile. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #68 |
| 100 | "Account" UI in settings | desktop+mobile | not-started | widespread | P6 | S | Settings has an "Account" or "You and Google" / "Firefox Account" / "Apple ID" section showing the signed-in identity, sync status, and sign-out. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #95 |

---

## Section 13 — Settings and configuration

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 101 | Settings panel (chrome / appearance, privacy, search, extensions, accessibility, advanced, system) | desktop+mobile | not-started | ubiquitous | P3 | XL | A single settings page (`chrome://settings` / `about:preferences`) is organised by section: Appearance, Privacy, Search, Extensions, Accessibility, Advanced, System. Search-in-settings is supported. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chrome settings docs](https://developer.chrome.com/docs/extensions/reference/api/settings) |
| 102 | Site settings (per-site permissions, content) | desktop+mobile | not-started | ubiquitous | P3 | L | Settings has a "Site settings" sub-section with the per-origin grants and content settings. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Permissions spec](https://www.w3.org/TR/permissions/) |
| 103 | Search engine management (default, custom, regional) | desktop+mobile | not-started | ubiquitous | P3 | S | Settings → Search has a list of installed search engines (with regional defaults) and an "Add search engine" form. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #14, #15 |
| 104 | Default browser handler (registration on each OS) | desktop+mobile | not-started | ubiquitous | P3 | S | Browser registers itself as the default for `http`, `https`, `ftp`, and HTML. On first run, the user is asked. Settings → Default Browser lets the user re-assert. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [W3C default-browser proposal](https://github.com/w3c/browsing-the-web/blob/main/payments-and-default-browser.md) |
| 105 | Reset settings, reset profile | desktop+mobile | not-started | widespread | P3 | S | Settings → Reset has two options: "Reset settings" (restores defaults but keeps data) and "Reset profile" (wipes everything). | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #101 |
| 106 | Import data from other browsers (HTML bookmarks, passwords CSV, history, cookies) | desktop+mobile | not-started | widespread | P4 | L | On first run (or on demand from settings), the user can pick another browser and import bookmarks (HTML), passwords (CSV), history (JSON/HTML), and cookies. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Chrome import data](https://support.google.com/chrome/answer/9439747) |
| 107 | Export data (bookmarks, passwords, history) | desktop+mobile | not-started | widespread | P4 | S | Settings has an "Export" or "Backup" option. Bookmarks → HTML; passwords → CSV (encrypted with a passphrase); history → JSON. | Chromium: partial (no password export by default; extensions); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #29 |
| 108 | Policy / enterprise management (managed config, group policy) | desktop+mobile | not-started | widespread | P6 | XL | An enterprise can push a configuration via OS-level mechanisms (Windows Registry, macOS plist, Linux dconf, JSON file). The browser reads at start and refuses to start on policy violation. | Chromium: yes/stable; Firefox: yes/stable (policies.json); WebKit: yes/stable (managed configuration profile); Servo: no; Ladybird: no; Flow: no. | [Chrome enterprise policies](https://chromeenterprise.google/policies/) ; [Firefox policies](https://mozilla.github.io/policy-templates/) |

---

## Section 14 — Crash handling and diagnostics

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 109 | Crash reporter UI (opt-in) | desktop+mobile | not-started | widespread | P4 | M | On crash, the next launch shows a "Send crash report?" dialog with a preview of the data. The user can opt out per-session and globally. | Chromium: yes/stable (Crashpad + opt-in); Firefox: yes/stable (Crash Reporter); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | [Crashpad](https://chromium.googlesource.com/crashpad/crashpad/) |
| 110 | "Page is not responding" / unresponsive script dialog | desktop+mobile | not-started | ubiquitous | P3 | S | A long-running script triggers a "Page is unresponsive" dialog with "Wait" and "Exit page" options. The dialog is non-modal and dismissible with Esc. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 111 | Memory saver / energy saver (background tab freezing) | desktop+mobile | not-started | widespread | P4 | M | A toolbar icon (or settings toggle) "frees memory" by unloading background tabs that have not been focused for N minutes. Tabs restore on focus. | Chromium: yes/stable (Memory Saver); Firefox: yes/stable (Discard inactive tabs); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | cross-ref with #3 (freeze) |
| 112 | Send feedback (form, screenshot, system info) | desktop+mobile | not-started | ubiquitous | P4 | M | Settings → Help → Send Feedback opens a form with optional screenshot, browser version, OS, and a free-text description. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 113 | "About this browser" / version info | desktop+mobile | not-started | ubiquitous | P3 | S | Settings → About shows the browser version, the engine version, the build channel, and a check-for-updates button. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 114 | Update notifications, in-place upgrade | desktop+mobile | not-started | widespread | P4 | M | When an update is available, a non-blocking notification appears. In-place upgrade runs at next restart. Some engines run the update silently in the background. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable (App Store / Software Update); Servo: no; Ladybird: no; Flow: no. | [Omaha / Chrome updater](https://chromium.googlesource.com/chromium/src/+/main/components/update_client/) |
| 115 | Beta / dev / canary channels | desktop+mobile | not-started | widespread | P4 | S | The browser ships in Stable, Beta, Dev (Nightly), and Canary. Each channel is a separate install on macOS / Linux; separate profile on Windows. | Chromium: yes/stable (4 channels); Firefox: yes/stable (Stable, Beta, Nightly, ESR); WebKit: yes/stable (Safari Technology Preview); Servo: no; Ladybird: no; Flow: no. | no W3C spec |

---

## Section 15 — Telemetry and feedback

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 116 | Anonymous usage stats (opt-in) | desktop+mobile | not-started | widespread | P4 | M | On first run, the user is asked whether to send anonymous usage stats. The setting is reversible. The collected data is enumerated in a privacy policy. | Chromium: yes/stable (default off, opt-in); Firefox: yes/stable (default off, opt-in); WebKit: yes/stable (default off); Servo: no; Ladybird: no; Flow: no. | [Chrome privacy whitepaper](https://www.google.com/chrome/privacy/whitepaper.html) |
| 117 | Crash report (opt-in) | desktop+mobile | not-started | widespread | P4 | M | See #109. Treated as a separate opt-in from usage stats. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #109 |
| 118 | "Help improve" surveys (NPS, in-product) | desktop+mobile | not-started | mixed | P6 | S | The browser may show a one-question NPS-style survey after a major update or a feature use. Surveys are opt-in. | Chromium: partial (in-product surveys via UMA); Firefox: partial; WebKit: no; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 119 | Screenshot for bug reports | desktop+mobile | not-started | widespread | P4 | S | The feedback form attaches a screenshot of the current viewport (or a selection) to the report. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #112 |
| 120 | "What's new" page on update | desktop+mobile | not-started | widespread | P4 | S | After an update, the new-tab page shows a one-page summary of the release notes. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | no W3C spec |

---

## Section 16 — Help and onboarding

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | UX expectation | Engine notes (Chromium / Firefox / WebKit / Servo / Ladybird / Flow) | Sources |
|---|---|---|---|---|---|---|---|---|---|
| 121 | First-run experience (welcome page, theme picker, default search) | desktop+mobile | not-started | ubiquitous | P3 | M | On first launch, the user sees a welcome page that lets them pick a theme, a default search engine, and an import-from-other-browser option. The page is skippable. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 122 | Sign-in prompt (deferred until a sync feature is invoked) | desktop+mobile | not-started | widespread | P6 | S | The user is not prompted to sign in on first run. Sign-in is requested the first time a sync feature is used (e.g. bookmarks sync, send-tab). | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #95 |
| 123 | Tour (interactive product tour) | desktop+mobile | not-started | niche | P6 | L | A short, skippable overlay tour highlights the toolbar, the URL bar, the tab strip, and the settings entry. Some engines add a "highlight on hover" pattern. | Chromium: partial (in-product help cards); Firefox: no; WebKit: yes/stable (Welcome tour in iOS); Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 124 | Help pages (URL-bar queries like `browser://help`) | desktop+mobile | not-started | widespread | P3 | S | A `help:` (Firefox) or `chrome://help` (Chromium) page lists the most common questions and links to the online help. | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | no W3C spec |
| 125 | Search keyboard shortcuts (in-app help) | desktop+mobile | not-started | widespread | P3 | S | A "?" overlay (or a settings sub-page) lists every keyboard shortcut, searchable. | Chromium: yes/stable (chrome://settings/appearance); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | same as #85 |
| 126 | Tutorial for new users (web-based, deep) | desktop+mobile | not-started | niche | P6 | M | An optional, deep tutorial covers 5–10 core workflows. Web-based so it can be updated without a release. | Chromium: yes/stable (web-based); Firefox: yes/stable (SUMO); WebKit: yes/stable (support.apple.com); Servo: no; Ladybird: no; Flow: no. | no W3C spec |

---

## Cross-refs to `specs/GAP_ANALYSIS.md` (sections 9–16)

| GAP_ANALYSIS row | Section | Line |
|---|---|---|
| Settings panel | §3.3 Presentation Layer | 232 |
| WebExtensions API (manifest v3) | §3.3 Presentation Layer | 233 |
| Sidebar tabs (create, switch, close, drag) | §3.3 Presentation Layer | 224 |
| Floating URL bar / Omnibox | §3.3 Presentation Layer | 225 |
| Find-in-page | §3.3 Presentation Layer | 230 |
| Downloads UI / manager | §3.3 Presentation Layer | 231 |
| Content script injection (sandboxed) | §3.3 Presentation Layer | 234 |
| Extensions ↔ page typed message bus | §3.3 Presentation Layer | 235 |

**Note:** Every row in sections 9–16 maps to a `[ ]` (not started) row in
GAP_ANALYSIS §3.3 (lines 220–235) and to the Phase 4–6 deliverable set in
`ROADMAP.md`. No new GAP_ANALYSIS rows are introduced; this chunk is a
**parity** view, not a new gap-claim.

---

## Open questions for the user (sections 9–16)

1. **Command palette row (#94).** Chromium shipped a command palette
   in 2024. Is the user interested in this as a Spiral chrome
   feature, or is it a "nice to have" to defer past Phase 6?

2. **Mouse gesture customisation (#86) and mouse-wheel scroll tab
   switching (#87).** These are power-user features common in
   chromium-fork markets. Are they a P6 stretch, or in-scope for
   Phase 4 customisation?

3. **Session manager (#91) and named workspaces (#92).** These are
   overlapping. Should they be one feature with two modes (single-
   session = sessions, multi-session = workspaces) or two separate
   features?

4. **Passkeys (#65) Phase 5.** WebAuthn is web-platform, not chrome.
   Spiral's `spiral-context` may need to back passkey storage. Is
   the passkey work in scope for chunk 7 (chrome) or chunk 4
   (web platform)?

5. **Tab sharing / live link (#72).** This is product-specific, not
   a W3C spec, and the closest thing is Web Share API. Spiral has no
   obvious backend. Is this a P6 we are intentionally de-scoping?

6. **Cast / AirPlay (#71).** This depends on `Remote Playback API`
   (chunk 6, partial coverage). The chrome surface (Cast icon in
   the toolbar) is small; the protocol stack is large. Is the
   chrome surface in scope for Phase 5 with a stub, or is this a
   chunk 5 cross-ref that defers the row?

7. **Send-tab-to-self (#68, #98).** This is a sync-server product
   feature; the "Account" row (#100) is the storage. Spiral's
   sync server is not in scope per the ROADMAP until late Phase 5.
   Should we track these rows as "blocked on sync" or pull sync
   forward?

---

## Sources (sections 1–8 + 9–16, combined)

### Tier 1 — Standards

- W3C Permissions: <https://www.w3.org/TR/permissions/>
- W3C Credential Management: <https://w3c.github.io/webappsec-credential-management/>
- W3C WebAuthn: <https://w3c.github.io/webauthn/>
- W3C FedCM: <https://fedidcg.github.io/FedCM/>
- W3C Global Privacy Control: <https://www.w3.org/TR/global-privacy-control/>
- W3C Payment Request: <https://www.w3.org/TR/payment-request/>
- W3C Web OTP: <https://wicg.github.io/web-otp/>
- W3C Web Share: <https://w3c.github.io/web-share/>
- W3C Web Locks: <https://w3c.github.io/web-locks/>
- W3C WebVTT: <https://www.w3.org/TR/webvtt1/>
- W3C MediaSession: <https://www.w3.org/TR/mediasession/>
- W3C Visual Viewport: <https://www.w3.org/TR/visual-viewport/>
- W3C Picture-in-Picture: <https://wicg.github.io/picture-in-picture/>
- W3C Remote Playback: <https://www.w3.org/TR/remote-playback/>
- W3C CSS Pseudo-4 (`::cue`): <https://www.w3.org/TR/css-pseudo-4/>
- W3C CSS Color Adjust 1 (`forced-colors`): <https://www.w3.org/TR/css-color-adjust-1/>
- WHATWG Notifications: <https://notifications.spec.whatwg.org/>
- WHATWG HTML §history: <https://html.spec.whatwg.org/multipage/history.html>
- WHATWG HTML §session-history: <https://html.spec.whatwg.org/multipage/history.html#session-history>
- WHATWG HTML §webstorage: <https://html.spec.whatwg.org/multipage/webstorage.html>
- WHATWG HTML §meta-theme-color: <https://html.spec.whatwg.org/multipage/semantics.html#meta-theme-color>
- WHATWG HTML §shared-workers: <https://html.spec.whatwg.org/multipage/workers.html#shared-workers>
- WHATWG HTMLMediaElement: <https://html.spec.whatwg.org/multipage/media.html#htmlmediaelement>
- IETF RFC 9110 §14.1.2 (Range): <https://www.rfc-editor.org/rfc/rfc9110#name-range>
- IETF RFC 8446 (TLS 1.3): <https://www.rfc-editor.org/rfc/rfc8446>

### Tier 2 — MDN / Can I Use

- MDN Permissions API: <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
- MDN Credential Management: <https://developer.mozilla.org/en-US/docs/Web/API/Credential_Management_API>
- MDN History API: <https://developer.mozilla.org/en-US/docs/Web/API/History_API>
- MDN Storage Access API: <https://developer.mozilla.org/en-US/docs/Web/API/Storage_Access_API>
- MDN mixed-content: <https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content>
- MDN Web Share API: <https://developer.mozilla.org/en-US/docs/Web/API/Web_Share_API>
- MDN mouse events: <https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent>

### Tier 3 — Engine / vendor

- Chromium tabs: <https://chromium.googlesource.com/chromium/src/+/main/docs/tabbed_browser.md>
- Chromium tab discard: <https://chromium.googlesource.com/chromium/src/+/main/docs/tab_discarding.md>
- Chromium tab groups: <https://chromium.googlesource.com/chromium/src/+/main/docs/tab_groups.md>
- Chromium lookalikes: <https://chromium.googlesource.com/chromium/src/+/main/components/lookalikes/>
- Chromium content settings: <https://chromium.googlesource.com/chromium/src/+/main/components/content_settings/core/browser/content_settings_registry.h>
- Chromium DOM Distiller: <https://chromium.googlesource.com/chromium/src/+/main/components/dom_distiller/>
- Chromium PDFium: <https://pdfium.googlesource.com/pdfium/>
- Chromium session restore: <https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/sessions/>
- Chromium sidebar: <https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/ui/views/side_panel/>
- Chromium update client: <https://chromium.googlesource.com/chromium/src/+/main/components/update_client/>
- Chromium multi-profile: <https://chromium.googlesource.com/chromium/src/+/main/docs/linux_profiles.md>
- Chromium search-leakage: <https://chromium.googlesource.com/chromium/src/+/main/docs/security/search-leakage.md>
- Chromium omnibox: <https://chromium.googlesource.com/chromium/src/+/main/components/omnibox/>
- Chromium Crashpad: <https://chromium.googlesource.com/crashpad/crashpad/>
- Chromium search engine design: <https://www.chromium.org/developers/design-documents/default-search-engine>
- Firefox urlbar classifier: <https://searchfox.org/mozilla-central/source/browser/components/urlbar/UrlbarClassifier.jsm>
- Firefox typeahead find: <https://searchfox.org/mozilla-central/source/toolkit/components/typeaheadfind/>
- Firefox Sync privacy: <https://hacks.mozilla.org/2018/11/firefox-sync-privacy/>
- Firefox ETP: <https://support.mozilla.org/en-US/kb/enhanced-tracking-protection-firefox-desktop>
- Firefox Containers: <https://support.mozilla.org/en-US/kb/containers>
- Firefox profile manager: <https://support.mozilla.org/en-US/kb/profile-manager-create-remove-switch-profiles>
- Firefox sidebar tabs: <https://support.mozilla.org/en-US/kb/use-sidebar-tabs-organize-tabs>
- Firefox keyword bookmarks: <https://support.mozilla.org/en-US/kb/how-search-from-address-bar>
- Firefox downloads prefs: <https://support.mozilla.org/en-US/kb/change-where-downloads-are-saved>
- Firefox applications: <https://support.mozilla.org/en-US/kb/change-firefox-behavior-when-open-file>
- Mozilla DNT removal: <https://blog.mozilla.org/security/2024/06/dnt/>
- Chrome send-tab: <https://support.google.com/chrome/answer/165139>
- Chrome Custom toolbar: <https://support.google.com/chrome/answer/95454>
- Chrome import data: <https://support.google.com/chrome/answer/9439747>
- Chrome NTP docs: <https://developer.chrome.com/docs/extensions/reference/api/override>
- Chrome themes format: <https://developer.chrome.com/docs/extensions/reference/api/theme>
- Chrome enterprise policies: <https://chromeenterprise.google/policies/>
- Chrome privacy whitepaper: <https://www.google.com/chrome/privacy/whitepaper.html>
- WebExtensions `bookmarks`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/bookmarks>
- WebExtensions `tabs.move`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/move>
- WebExtensions `tabs.query`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/query>
- WebExtensions `tabGroups`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabGroups>
- WebExtensions `downloads`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/downloads>
- WebExtensions `sessions`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/sessions>
- WebExtensions `commands`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/commands>
- WebExtensions `theme`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/theme>
- WebExtensions `identity`: <https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/identity>
- WebExtensions `search`: <https://developer.chrome.com/docs/extensions/reference/api/search>
- PDF.js: <https://mozilla.github.io/pdf.js/>
- Mozilla policy templates: <https://mozilla.github.io/policy-templates/>

### Tier 4 — Third-party

- privacytests.org: <https://privacytests.org>
- EFF Cover Your Tracks: <https://coveryourtracks.eff.org>
- EFF on DNT: <https://www.eff.org/issues/do-not-track>
- HIBP Pwned Passwords API: <https://haveibeenpwned.com/API/v3#PwnedPasswords>

### Tier 5 — Supporting

- DuckDuckGo cookie pop-up blocking: <https://duckduckgo.com/cookie-consent-blocking>
- ARC Workspaces overview: <https://en.wikipedia.org/wiki/Arc_(browser)>
- Apple HIG swipe actions: <https://developer.apple.com/design/human-interface-guidelines/swipe-actions>
- Vimium: <https://github.com/philc/vimium>
- Netscape bookmark format: <https://learn.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa753582(v=vs.85)>

---

## Row count summary

| Section | Rows | Range |
|---------|------|-------|
| 1. Tabs and windows | 10 | 1–10 |
| 2. Address bar | 7 | 11–17 |
| 3. Navigation | 9 | 18–26 |
| 4. Bookmarks | 6 | 27–32 |
| 5. Downloads | 8 | 33–40 |
| 6. Find and text tools | 7 | 41–47 |
| 7. Privacy UI | 12 | 48–59 |
| 8. Passwords and autofill | 7 | 60–66 |
| 9. Sharing and integration | 7 | 67–73 |
| 10. Reading, viewing, media UI | 8 | 74–81 |
| 11. Customisation and ergonomics | 13 | 82–94 |
| 12. Sync and account | 6 | 95–100 |
| 13. Settings and configuration | 8 | 101–108 |
| 14. Crash handling and diagnostics | 7 | 109–115 |
| 15. Telemetry and feedback | 5 | 116–120 |
| 16. Help and onboarding | 6 | 121–126 |
| **Total** | **126** | **1–126** |
