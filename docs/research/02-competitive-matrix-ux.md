# Chunk 12 — Competitive Matrix: User-Facing UX

**Generated:** 2026-06-16
**Sources:** `03-user-facing-ux.md` (sections 1–8, rows 1–66), `03-user-facing-ux-extension.md` (sections 9–16, rows 67–126)
**Methodology:** `00-methodology.md`
**Total rows:** 126

---

## Engine column codes

| Code | Meaning |
|------|
| `yes` | Feature shipped and stable (or near-stable) |
| `partial` | Partial implementation, behind extension, or limited surface |
| `no` | Not shipped |
| `behind-flag` | Available behind a flag or origin trial only |

---

## Section 1 — Tab and window management

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | Tab open, close, duplicate, reopen-last-closed | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | partial |
| 2 | Tab move (reorder, move to new window, tear-off) | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | no |
| 3 | Tab pin, mute, freeze (process) | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | no |
| 4 | Tab groups (named, coloured, collapsible, persist) | desktop | not-started | widespread | P5 | L | yes | partial | yes | no | no |
| 5 | Tab search (search by title/URL across open tabs) | desktop+mobile | not-started | widespread | P4 | M | yes | no | yes | no | no |
| 6 | Named tab set / workspace (switchable, persists) | desktop+mobile | not-started | niche | P6 | L | partial | partial | no | no | no |
| 7 | Profile-level tab separation (work vs personal, multiple sessions, cookie isolation) | desktop+mobile | not-started | widespread | P6 | XL | yes | yes | no | no | no |
| 8 | Tab strip layouts — horizontal, vertical, tiled | desktop+mobile | not-started | widespread | P4 | M | partial | yes | partial | no | no |
| 9 | Tab preview on hover (image or text-only) | desktop | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 10 | On-startup options: continue where you left off / new tab page / restore previous session | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | partial |

---

## Section 2 — Address bar (omnibox) and search

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 11 | URL typing, autocomplete, smart keyword detection (URL vs search) | desktop+mobile | not-started | ubiquitous | P3 | L | yes | yes | yes | partial | no |
| 12 | Suggestion sources — history, bookmarks, open tabs, top sites, search | desktop+mobile | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 13 | Suggestion ranking, debouncing, type-ahead | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 14 | Custom search engines and keyword shortcuts (tab-to-search) | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 15 | Search engine switching (default, per-region, per-search) | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 16 | Search leakage mitigations (preconnect-on-focus, no DNS resolution on keystroke) | desktop+mobile | not-started | mixed | P5 | M | yes | yes | yes | no | no |
| 17 | Quick-find bar (slash-key shortcut to find-in-page) | desktop | not-started | niche | P6 | S | partial | partial | partial | no | no |

---

## Section 3 — Navigation

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 18 | Back, forward, reload, stop, home buttons | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | partial |
| 19 | Mouse back/forward buttons (button 3 / 4 / 5 / 6) | desktop | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 20 | Pull-to-refresh (touch/embedded) | mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 21 | History (full view, search, by date, by site) | desktop+mobile | not-started | ubiquitous | P4 | L | yes | yes | yes | no | no |
| 22 | Reopen closed tab (single + n-deep) | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 23 | Recently closed tabs and windows (in history UI) | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 24 | URL bar hides/shows on scroll (mobile) | mobile+embedded | not-started | widespread | P5 | S | yes | yes | yes | no | no |
| 25 | Swipe navigation (back/forward edge swipe) | mobile | not-started | widespread | P5 | S | yes | yes | yes | no | no |
| 26 | Home button (configurable, multi-home, URL list) | desktop+mobile | not-started | widespread | P3 | S | yes | yes | yes | no | no |

---

## Section 4 — Bookmarks and saved items

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 27 | Bookmark, bookmark folder, bookmark manager UI | desktop+mobile | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 28 | Bookmark bar (always-visible strip) | desktop | not-started | widespread | P3 | S | yes | yes | yes | no | no |
| 29 | Bookmark import / export (HTML format, JSON) | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 30 | Synced bookmarks (cross-device, cloud-mediated) | desktop+mobile | not-started | widespread | P6 | XL | yes | yes | yes | no | no |
| 31 | Reading list / saved-for-later (article-specific, offline cache) | desktop+mobile | not-started | widespread | P5 | L | partial | yes | yes | no | no |
| 32 | Highlights and annotations (article-level) | desktop+mobile | not-started | niche | P6 | L | no | partial | yes | no | no |

---

## Section 5 — Downloads

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 33 | Download manager UI (progress, pause, resume, cancel) | desktop+mobile | not-started | ubiquitous | P4 | L | yes | yes | yes | no | no |
| 34 | Download history, clear on quit | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 35 | Download location (default + per-format) | desktop+mobile | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 36 | "Ask where to save" toggle | desktop | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 37 | Safe Browsing check on download, dangerous file type warning | desktop+mobile | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 38 | Resumable downloads (HTTP Range) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 39 | "Always open this file type" association | desktop | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 40 | Download notifications (system-level) | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |

---

## Section 6 — Find-in-page and text tools

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 41 | Find-in-page (Ctrl/Cmd+F), case-sensitive, whole-word, regex | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | partial |
| 42 | Text zoom / page zoom (per-site, default, full-page vs text-only) | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | no |
| 43 | Force enable zoom (mobile) | mobile | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 44 | Reader mode (auto-detect, manual toggle, themable, save offline) | desktop+mobile | not-started | widespread | P5 | L | partial | yes | yes | no | no |
| 45 | Page translation (auto-detect, engine choice) | desktop+mobile | not-started | widespread | P6 | XL | yes | yes | yes | no | no |
| 46 | Dictionary lookup (selection → definition) | desktop+mobile | not-started | niche | P6 | M | partial | yes | yes | no | no |
| 47 | Link preview / hover preview (on hover, in a popover) | desktop | not-started | niche | P6 | M | partial | partial | no | no | no |

---

## Section 7 — Privacy and tracking controls (UI surface)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 48 | URL-pattern permission scopes (per-site: camera, mic, location, notifications, etc.) | desktop+mobile | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 49 | Per-site content settings (JS, images, cookies, popups, redirects, ads, fingerprinting, autoplay) | desktop+mobile | not-started | ubiquitous | P3 | XL | yes | yes | yes | no | no |
| 50 | Per-site content filter (in-bar toggle for tracker/ads blocking) | desktop+mobile | not-started | widespread | P4 | L | partial | yes | yes | no | no |
| 51 | Global Privacy Control (GPC header) | desktop+mobile | not-started | widespread | P3 | S | partial | yes | yes | no | no |
| 52 | Do Not Track (DNT) — legacy / deprecated | desktop | not-started | mixed | P6 | S | no | no | no | no | no |
| 53 | Cookie banner blocking / auto-dismiss | desktop+mobile | not-started | widespread | P4 | L | partial | no | partial | no | no |
| 54 | HTTPS-only mode (block HTTP, warn on mixed content) | desktop+mobile | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 55 | Connection secure/insecure indicator in the address bar | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 56 | Certificate viewer, TLS details | desktop+mobile | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 57 | Phishing / malware warning page | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 58 | Deceptive site warning (lookalike URL) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 59 | Private browsing mode (entry, exit, what is cleared, what is not) | desktop+mobile | not-started | ubiquitous | P4 | L | yes | yes | yes | no | no |

---

## Section 8 — Passwords and autofill

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 60 | Built-in password manager (save, autofill, generate, audit) | desktop+mobile | not-started | ubiquitous | P4 | XL | yes | yes | yes | no | no |
| 61 | Cross-device password sync (E2EE) | desktop+mobile | not-started | widespread | P6 | XL | yes | yes | yes | no | no |
| 62 | Password breach detection (Have I Been Pwned integration) | desktop+mobile | not-started | widespread | P5 | M | yes | yes | yes | no | no |
| 63 | Two-factor code autofill (SMS / TOTP) | desktop+mobile | not-started | widespread | P5 | L | yes | partial | yes | no | no |
| 64 | Address, payment, identity autofill | desktop+mobile | not-started | widespread | P5 | XL | yes | yes | yes | no | no |
| 65 | Passkeys (WebAuthn, discoverable credentials) | desktop+mobile | not-started | widespread | P5 | L | yes | yes | yes | no | no |
| 66 | Federated sign-in (sign-in with identity provider) | desktop+mobile | not-started | widespread | P5 | L | yes | yes | yes | no | no |

---

## Section 9 — Sharing and integration

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 67 | Native share (OS share sheet, mobile) | mobile+embedded+desktop | not-started | ubiquitous | P4 | S | yes | partial | yes | no | no |
| 68 | Send-tab-to-self (cross-device) | desktop+mobile | not-started | widespread | P6 | L | yes | yes | yes | no | no |
| 69 | QR code generation for URL | desktop+mobile | not-started | widespread | P4 | S | yes | no | yes | no | no |
| 70 | QR code scanner (camera) | mobile+embedded | not-started | widespread | P5 | M | yes | no | yes | no | no |
| 71 | Cast / AirPlay (presentation of page to TV) | desktop+mobile | not-started | widespread | P5 | L | yes | no | yes | no | no |
| 72 | Tab sharing with another user (live link) | desktop+mobile | not-started | niche | P6 | L | partial | partial | no | no | no |
| 73 | Email-link, message-link, copy-link | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |

---

## Section 10 — Reading, viewing, media UI

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 74 | PDF viewer (built-in vs plugin) | desktop+mobile | not-started | widespread | P5 | L | yes | yes | yes | no | no |
| 75 | Image viewer (full-size, zoom, pan, rotate, EXIF strip, save) | desktop+mobile | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 76 | Video player UI (chapters, subtitles, fullscreen, PiP, cast) | desktop+mobile | not-started | ubiquitous | P4 | L | yes | yes | yes | no | no |
| 77 | In-tab media controls (audio playback UI on tab strip) | desktop+mobile | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 78 | Global media controls (lock-screen, OS-level, browser toolbar) | desktop+mobile | not-started | widespread | P4 | L | yes | yes | yes | no | no |
| 79 | Subtitle / caption styling, language picker | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 80 | Dark mode / forced colours (auto, system, on, off; per-site override) | desktop+mobile | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 81 | Site theme colour (browser chrome reflects meta name="theme-color") | mobile+embedded+desktop | not-started | widespread | P4 | S | yes | yes | yes | no | no |

---

## Section 11 — Customisation and ergonomics

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 82 | Home page, new tab page (background, layout, widgets, search box placement) | desktop+mobile | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 83 | Custom themes (background image, dark/light, accent colour, font) | desktop+mobile | not-started | widespread | P4 | L | partial | yes | no | no | no |
| 84 | Toolbar customisation (drag-and-drop buttons) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | partial | no | no |
| 85 | Keyboard shortcut customisation | desktop+mobile | not-started | widespread | P4 | M | yes | yes | no | no | no |
| 86 | Mouse gesture customisation | desktop | not-started | niche | P6 | M | partial | partial | no | no | no |
| 87 | Mouse-wheel-scroll tab switching | desktop | not-started | niche | P6 | S | partial | partial | no | no | no |
| 88 | Sidebar (bookmarks, history, downloads, reading list, tabs list, notes) | desktop+mobile | not-started | widespread | P4 | L | yes | yes | yes | no | no |
| 89 | Vertical tabs | desktop | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 90 | Tab hover preview (image preview on mouse hover) | desktop | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 91 | Session manager (save all open tabs as a named session, restore later) | desktop+mobile | not-started | widespread | P5 | M | partial | partial | no | no | no |
| 92 | Named tab set / workspace (capability-agnostic) | desktop+mobile | not-started | niche | P6 | L | partial | partial | no | no | no |
| 93 | Tidy tabs / auto-group similar tabs / suspend background tabs | desktop | not-started | niche | P6 | L | yes | partial | yes | no | no |
| 94 | Command palette (Ctrl/Cmd+Shift+P, type-to-search UI elements) | desktop | not-started | niche | P5 | M | yes | no | no | no | no |

---

## Section 12 — Sync and account

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 95 | Sign-in (browser-level account) | desktop+mobile | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no |
| 96 | Sync (bookmarks, history, tabs, passwords, extensions, settings, open tabs, reading list) | desktop+mobile | not-started | widespread | P6 | XL | yes | yes | yes | no | no |
| 97 | End-to-end encryption for synced data | desktop+mobile | not-started | widespread | P6 | L | yes | yes | yes | no | no |
| 98 | Send-tab-to-self | desktop+mobile | not-started | widespread | P6 | M | yes | yes | yes | no | no |
| 99 | Cross-device tab pickup (open this tab on phone) | desktop+mobile | not-started | widespread | P6 | M | yes | yes | yes | no | no |
| 100 | "Account" UI in settings | desktop+mobile | not-started | widespread | P6 | S | yes | yes | yes | no | no |

---

## Section 13 — Settings and configuration

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 101 | Settings panel (appearance, privacy, search, extensions, accessibility, advanced, system) | desktop+mobile | not-started | ubiquitous | P3 | XL | yes | yes | yes | no | no |
| 102 | Site settings (per-site permissions, content) | desktop+mobile | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 103 | Search engine management (default, custom, regional) | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 104 | Default browser handler (registration on each OS) | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 105 | Reset settings, reset profile | desktop+mobile | not-started | widespread | P3 | S | yes | yes | yes | no | no |
| 106 | Import data from other browsers (HTML bookmarks, passwords CSV, history, cookies) | desktop+mobile | not-started | widespread | P4 | L | yes | yes | yes | no | no |
| 107 | Export data (bookmarks, passwords, history) | desktop+mobile | not-started | widespread | P4 | S | partial | yes | yes | no | no |
| 108 | Policy / enterprise management (managed config, group policy) | desktop+mobile | not-started | widespread | P6 | XL | yes | yes | yes | no | no |

---

## Section 14 — Crash handling and diagnostics

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 109 | Crash reporter UI (opt-in) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 110 | "Page is not responding" / unresponsive script dialog | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 111 | Memory saver / energy saver (background tab freezing) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 112 | Send feedback (form, screenshot, system info) | desktop+mobile | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 113 | "About this browser" / version info | desktop+mobile | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 114 | Update notifications, in-place upgrade | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 115 | Beta / dev / canary channels | desktop+mobile | not-started | widespread | P4 | S | yes | yes | yes | no | no |

---

## Section 15 — Telemetry and feedback

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 116 | Anonymous usage stats (opt-in) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 117 | Crash report (opt-in) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 118 | "Help improve" surveys (NPS, in-product) | desktop+mobile | not-started | mixed | P6 | S | partial | partial | no | no | no |
| 119 | Screenshot for bug reports | desktop+mobile | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 120 | "What's new" page on update | desktop+mobile | not-started | widespread | P4 | S | yes | yes | yes | no | no |

---

## Section 16 — Help and onboarding

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|---|---|---|---|---|---|---|---|---|---|---|
| 121 | First-run experience (welcome page, theme picker, default search) | desktop+mobile | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 122 | Sign-in prompt (deferred until a sync feature is invoked) | desktop+mobile | not-started | widespread | P6 | S | yes | yes | yes | no | no |
| 123 | Tour (interactive product tour) | desktop+mobile | not-started | niche | P6 | L | partial | no | yes | no | no |
| 124 | Help pages (URL-bar queries like browser://help) | desktop+mobile | not-started | widespread | P3 | S | yes | yes | yes | no | no |
| 125 | Search keyboard shortcuts (in-app help) | desktop+mobile | not-started | widespread | P3 | S | yes | yes | yes | no | no |
| 126 | Tutorial for new users (web-based, deep) | desktop+mobile | not-started | niche | P6 | M | yes | yes | yes | no | no |

---

## Summary

| Section | Rows | Range |
|---------|------|
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
