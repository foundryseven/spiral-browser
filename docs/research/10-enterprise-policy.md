# Chunk 11 (companion) — Telemetry, Crash Reporting, Default Browser, First Run, Policy & Enterprise

> **Companion to `10-distribution-platforms.md`.** Sections 1–9 (packaging,
> OS integration, sandbox) live in the main file. This file covers
> telemetry / crash reporting, default browser / first-run experience,
> policy / enterprise deployment, and diagnostics surfaces.
>
> **Methodology contract:** `00-methodology.md`. **Source ladder:**
> `citations/sources.md`. **Output contract:** `README.md` §"Per-chunk
> output contract".

---

## Section 10 — Telemetry and crash reporting

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 107 | Crash reporter (opt-in / opt-out / no telemetry by default) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (opt-out by default); Firefox: yes/stable (opt-out by default); WebKit: yes/stable (opt-in on macOS, opt-out on iOS); Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/components/crash/ ; https://wiki.mozilla.org/Breakpad |
| 108 | Crash dump — minidump (Breakpad / Crashpad) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (Crashpad); Firefox: yes/stable (Breakpad); WebKit: yes/stable (CrashReporter on macOS); Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/breakpad/breakpad/ ; https://chromium.googlesource.com/crashpad/crashpad/ |
| 109 | Crash dump — full dump (complete memory, opt-in only) | desktop+mobile+embedded | not-started | 75-90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: no; Servo: no; Ladybird: no; Flow: no. | https://wiki.mozilla.org/Performance:Full_Memory_Dumps |
| 110 | Telemetry — opt-in (no data collected without explicit consent) | desktop+mobile+embedded | not-started (by design — "None — by design" per GAP) | 50-75% | P6 | M | Chromium: no (opt-out); Firefox: no (opt-out); WebKit: yes/stable (opt-in on macOS); Servo: no; Ladybird: no; Flow: no. | https://www.apple.com/legal/privacy/en-ww/ ; https://wiki.mozilla.org/Telemetry |
| 111 | Telemetry — opt-out (data collected by default, user can disable) | desktop+mobile+embedded | not-started (Spiral's design: no telemetry) | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: no (opt-in); Servo: no; Ladybird: no; Flow: no. | https://policies.google.com/technologies/telemetry ; https://wiki.mozilla.org/Telemetry |
| 112 | Telemetry — anonymisation (differential privacy, k-anonymity, RAPPOR) | desktop+mobile+embedded | not-started | 75-90% | P6 | L | Chromium: yes/stable (RAPPOR, k-anonymity for Safe Browsing); Firefox: yes/stable (OHTTP, Prio); WebKit: yes/stable (differential privacy for Safari suggestions); Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/components/rappor/ ; https://wiki.mozilla.org/Prio ; https://www.apple.com/privacy/docs/Differential_Privacy_Overview.pdf |
| 113 | Usage statistics (anonymised, opt-in) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (UMA — User Metrics Analysis); Firefox: yes/stable; WebKit: yes/stable (opt-in); Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/tools/metrics/histograms/ |
| 114 | "Help improve" surveys / feedback prompts | desktop+mobile+embedded | not-started | 50-75% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: no; Servo: no; Ladybird: no; Flow: no. | https://support.mozilla.org/en-US/kb/firefox-health-report |
| 115 | Onboarding telemetry opt-in (first-run prompt) | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable (opt-in during setup); Servo: no; Ladybird: no; Flow: no. | https://support.mozilla.org/en-US/kb/usage-collection-and-opt-out |
| 116 | Privacy-respecting telemetry — noise, randomised reporting (Prio / OHTTP / RAPPOR) | desktop+mobile+embedded | not-started | 50-75% | P6 | L | Chromium: yes/stable (RAPPOR); Firefox: yes/stable (Prio + OHTTP); WebKit: yes/stable (differential privacy); Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/components/rappor/ ; https://wiki.mozilla.org/Prio ; https://www.ietf.org/archive/id/draft-thomson-ppm-dap-02.html |

---

## Section 11 — Default browser and first-run experience

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 117 | Default-browser check (first-run prompt) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable (macOS); Servo: no; Ladybird: no; Flow: no. | https://support.google.com/chrome/answer/95417 ; https://support.mozilla.org/en-US/kb/change-default-browser |
| 118 | "Make default" flow (settings page, OS integration dialog) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://support.google.com/chrome/answer/95417 |
| 119 | First-run experience (onboarding wizard) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://support.mozilla.org/en-US/kb/first-run-pages-and-personal-settings |
| 120 | Profile import on first run (from Chromium, Firefox, WebKit, Servo, etc.) | desktop | not-started | >=90% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://support.google.com/chrome/answer/146258 ; https://support.mozilla.org/en-US/kb/switching-chrome-firefox |
| 121 | Sign-in / sign-up on first run (account sync) | desktop+mobile+embedded | not-started | >=90% | P6 | L | Chromium: yes/stable (Google Account); Firefox: yes/stable (Firefox Account); WebKit: yes/stable (Apple ID / iCloud); Servo: no; Ladybird: no; Flow: no. | https://support.google.com/accounts/answer/112802 ; https://support.mozilla.org/en-US/kb/how-do-i-set-up-firefox-sync |
| 122 | Theme and dark mode selection on first run | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://support.mozilla.org/en-US/kb/themes |
| 123 | Default search engine selection on first run | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (EU DSA mandate); Firefox: yes/stable (EU DSA mandate); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://blog.google/around-the-globe/google-europe/eu-choice-screen/ ; https://blog.mozilla.org/en/mozilla-search-update/ |
| 124 | Privacy-respecting defaults (tracking protection ON, telemetry OFF) | desktop+mobile+embedded | not-started | 75-90% | P6 | M | Chromium: partial (Safe Browsing ON, telemetry opt-out); Firefox: yes/stable (ETP ON by default); WebKit: yes/stable (ITP ON by default); Servo: no; Ladybird: no; Flow: no. | https://wiki.mozilla.org/Tracking_Protection ; https://webkit.org/tracking-prevention-policy/ |

---

## Section 12 — Policy and enterprise deployment

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 125 | Group Policy / plist / JSON policy (Windows GPO, macOS plist, Linux JSON, `.reg`) | desktop+embedded | not-started | >=90% | P6 | L | Chromium: yes/stable (Chrome Policy Templates); Firefox: yes/stable (policies.json); WebKit: n/a (Safari uses macOS plist); Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ ; https://mozilla.github.io/policy-templates/ |
| 126 | Recommended policies (managed config, not enforced) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ ; https://mozilla.github.io/policy-templates/ |
| 127 | Required policies (mandatory, cannot be overridden by user) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 128 | Mandatory extensions (ExtensionInstallForcelist) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ ; https://mozilla.github.io/policy-templates/ |
| 129 | Extension block list (ExtensionInstallBlocklist) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 130 | ExtensionSettings policy (fine-grained extension control) | desktop+embedded | not-started | >=90% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 131 | Auto-updates disabled (enterprise freeze) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable (UpdateDefault); Firefox: yes/stable (AppAutoUpdate); WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 132 | Default browser forced (cannot be changed by user) | desktop+embedded | not-started | 75-90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 133 | Allow / deny URL lists (URLBlocklist, URLAllowlist) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ ; https://mozilla.github.io/policy-templates/ |
| 134 | Proxy forced (ProxyMode, ProxyServer, ProxyPacUrl) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable (network.proxy.* prefs); WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ ; https://mozilla.github.io/policy-templates/ |
| 135 | Search engine forced (DefaultSearchProviderName, URL, keyword) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 136 | Bookmark forced (ManagedBookmarks) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 137 | Home page forced (HomepageLocation, HomepageIsNewTabPage) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 138 | SafeBrowsing force / disable (SafeBrowsingEnabled) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 139 | Password manager policy (PasswordManagerEnabled) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 140 | Autofill policy (AutoFillEnabled) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 141 | Tracking protection policy (TrackingProtectionMode) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ ; https://mozilla.github.io/policy-templates/ |
| 142 | Translate policy (TranslateEnabled) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: n/a; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 143 | Default download directory (DownloadDirectory) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 144 | Print policy (PrintingEnabled, PrintHeaderFooter) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 145 | Developer tools disabled (DeveloperToolsAvailability) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromeenterprise.google/policy/ |
| 146 | SmartScreen for websites (SmartScreenEnabled) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable (via Edge policy); Firefox: no; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/deployedge/microsoft-edge-security-smartscreen |

---

## Section 13 — Diagnostics surfaces

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 147 | `about:support` / support report (profile, GPU, extensions, modified prefs) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (`chrome://gpu`, `chrome://version`); Firefox: yes/stable (`about:support`); WebKit: yes/stable (`about://diagnostics`); Servo: no; Ladybird: no; Flow: no. | https://support.mozilla.org/en-US/kb/troubleshooting-information-page |
| 148 | `about:telemetry` / telemetry viewer (see what data is collected) | desktop+mobile+embedded | not-started | 75-90% | P6 | M | Chromium: yes/stable (`chrome://histograms`); Firefox: yes/stable (`about:telemetry`); WebKit: no; Servo: no; Ladybird: no; Flow: no. | https://firefox-source-docs.mozilla.org/toolkit/components/telemetry/ |
| 149 | Crash reporter UI (send / don't send, crash details) | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://support.mozilla.org/en-US/kb/mozilla-crash-reporter |
| 150 | Encrypted crash reports (HTTPS upload, encrypted at rest) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (HTTPS + signed); Firefox: yes/stable (HTTPS + signed); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://wiki.mozilla.org/Breakpad |

---

## Cross-refs to `specs/GAP_ANALYSIS.md` (companion file)

| GAP row | Title | Status in GAP | What it covers in this file |
|---------|-------|----------------|----------------------------|
| 4.1 (last row) | Telemetry / phone-home | `[x]` — "None — by design" | §10 rows 110–111, §12 telemetry-forced |
| 3.13 | Native platform integration | `⛔/❌` | §11 rows 117–124 |
| 4.1 | Policy / enterprise | not in GAP | §12 (all rows) — new surface for chunk 13 synthesis |

No rows in `specs/GAP_ANALYSIS.md` currently cover telemetry, crash
reporting, default-browser / first-run, policy, or diagnostics. Chunk
13 (synthesis) will append Deltas for the ones Spiral intends to
address.

---

## Open questions for the user (companion)

1. **Telemetry by design.** `GAP_ANALYSIS.md:4.1` says "None — by
   design." Rows 110–116 document what the industry ships, but
   Spiral's posture is "no telemetry at all" for v0.1.0. Is this
   the permanent posture, or should a crash reporter (opt-in) be
   added post-v0.1.0? Currently the rows are descriptive only and
   do not imply a Spiral implementation choice.

2. **Enterprise policy scope.** Section 12 rows 125–146 cover the
   full Chromium policy surface (~100+ policies). For v0.1.0, is a
   *single* policy file (`spiral.json` / `spiral.plist` / GPO)
   with 10–20 core policies sufficient, or must Spiral match the
   full Chromium surface? Currently scored as "Chromium: yes/stable"
   without a Spiral target.

3. **Profile import on first run.** Row 120 says >=90% of browsers
   import profiles. The import sources are: Chromium (100M+ data
   points), Firefox, WebKit/Safari. For v0.1.0, is import from
   Chromium the minimum, or must Firefox and WebKit also be
   supported? Currently all three scored as >=90% (industry-wide).

4. **Default search engine EU DSA mandate.** Row 123 references the
   EU Digital Services Act choice screen mandate. Spiral is not
   subject to DSA (not a gatekeeper), but the capability is still
   relevant for user choice. Should Spiral ship a search engine
   choice screen on first run, or default to a privacy-respecting
   engine (e.g. DuckDuckGo) without prompting?

5. **Diagnostics surfaces.** Rows 147–150 describe `about:support`
   and `about:telemetry` equivalents. For v0.1.0, is a single
   `spiral://support` page with build info, GPU info, and extension
   list sufficient? Or should it be a full `about:support` clone?

---

## Sources (companion)

| Tier | Source | URL | Used for |
|------|--------|-----|----------|
| 1 | Chromium Policy Templates | https://chromeenterprise.google/policy/ | Policy §12 (all rows) |
| 1 | Mozilla Policy Templates | https://mozilla.github.io/policy-templates/ | Policy §12 (all rows) |
| 1 | Chromium Telemetry (UMA) | https://chromium.googlesource.com/chromium/src/+/main/tools/metrics/histograms/ | Telemetry (row 113) |
| 1 | Mozilla Telemetry | https://wiki.mozilla.org/Telemetry | Telemetry (rows 110–116) |
| 1 | Chromium RAPPOR | https://chromium.googlesource.com/chromium/src/+/main/components/rappor/ | Privacy telemetry (rows 112, 116) |
| 1 | Mozilla Prio | https://wiki.mozilla.org/Prio | Privacy telemetry (rows 112, 116) |
| 1 | Apple Differential Privacy | https://www.apple.com/legal/privacy/en-ww/ | Privacy telemetry (row 112) |
| 1 | W3C Web Share API | https://w3c.github.io/web-share/ | Cross-ref from §10 |
| 1 | EU DSA | https://digital-strategy.ec.europa.eu/en/policies/dsa | Search engine choice (row 123) |
| 1 | Microsoft SmartScreen | https://learn.microsoft.com/en-us/deployedge/microsoft-edge-security-smartscreen | SmartScreen policy (row 146) |
| 2 | MDN — Crash Reporter | https://wiki.mozilla.org/Breakpad | Crash dump (row 108) |
| 2 | Chromium Crashpad | https://chromium.googlesource.com/crashpad/crashpad/ | Crash dump (row 108) |
| 2 | Mozilla Support — about:support | https://support.mozilla.org/en-US/kb/troubleshooting-information-page | Diagnostics (row 147) |
| 2 | Firefox Source Docs — Telemetry | https://firefox-source-docs.mozilla.org/toolkit/components/telemetry/ | Telemetry viewer (row 148) |
| 2 | Mozilla Support — Crash Reporter | https://support.mozilla.org/en-US/kb/mozilla-crash-reporter | Crash UI (row 149) |
| 2 | Mozilla Support — Default browser | https://support.mozilla.org/en-US/kb/change-default-browser | Default browser (row 117) |
| 2 | Mozilla Support — First run | https://support.mozilla.org/en-US/kb/first-run-pages-and-personal-settings | First run (row 119) |
| 2 | Mozilla Support — Firefox Sync | https://support.mozilla.org/en-US/kb/how-do-i-set-up-firefox-sync | Sign-in (row 121) |
| 2 | Mozilla Support — Tracking Protection | https://wiki.mozilla.org/Tracking_Protection | Privacy defaults (row 124) |
| 2 | WebKit Tracking Prevention | https://webkit.org/tracking-prevention-policy/ | Privacy defaults (row 124) |
| 2 | Google Chrome — Import bookmarks | https://support.google.com/chrome/answer/146258 | Profile import (row 120) |
| 2 | Mozilla Support — Switch from Chrome | https://support.mozilla.org/en-US/kb/switching-chrome-firefox | Profile import (row 120) |
| 3 | Google Blog — EU Choice Screen | https://blog.google/around-the-globe/google-europe/eu-choice-screen/ | Search engine choice (row 123) |
| 3 | Mozilla Blog — Search Update | https://blog.mozilla.org/en/mozilla-search-update/ | Search engine choice (row 123) |
| 3 | Chromium Health Report | https://support.mozilla.org/en-US/kb/firefox-health-report | Feedback (row 114) |
| 3 | Chromium GPU diagnostics | https://chromium.googlesource.com/chromium/src/+/main/gpu/ | Diagnostics (row 147) |
| 3 | Mozilla Crash Reporter | https://support.mozilla.org/en-US/kb/mozilla-crash-reporter | Crash UI (row 149) |

Per-row URLs are inlined in the table cells; this table is the master index for the companion file.
