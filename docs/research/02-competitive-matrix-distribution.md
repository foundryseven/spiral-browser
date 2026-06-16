# Competitive Matrix — Domain: Distribution & Platform Integration

**File:** `02-competitive-matrix-distribution.md`
**Date:** 2026-06-16
**Sources:** `10-distribution-platforms.md`, `10-enterprise-policy.md`
**Methodology:** `00-methodology.md`

## Column legend

- **Status in Spiral:** `shipped` / `partial` / `designed` / `not-started` / `do-not-touch`
- **Prevalence:** `ubiquitous` (>95%) / `widespread` (70–95%) / `mixed` (two+ engines, at least one no) / `niche` (one engine) / `experimental` (flag-only) / `legacy` (deprecated)
- **Phase:** per `00-methodology.md` section 5
- **Complexity:** `S` / `M` / `L` / `XL`
- **Engine columns:** `yes` / `partial` / `no` / `behind-flag`

---

## Section 1 — Linux distribution / packaging

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 1 | .deb package (Debian, Ubuntu, derivatives) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 2 | .rpm package (Fedora, RHEL, openSUSE) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 3 | AppImage (portable, no-install) | desktop | not-started | mixed | P6 | M | no | yes | no | no | no | no |
| 4 | Flatpak (Flathub, sandboxed) | desktop+embedded | not-started | mixed | P6 | M | yes | yes | yes | no | no | no |
| 5 | Snap (Ubuntu Store, snapd, confined) | desktop+embedded | not-started | mixed | P6 | M | yes | yes | no | no | no | no |
| 6 | Arch Linux AUR (PKGBUILD) | desktop+embedded | not-started | widespread | P6 | S | yes | yes | no | yes | yes | no |
| 7 | Nixpkgs derivation | desktop+embedded | not-started | niche | P6 | M | yes | yes | yes | yes | yes | no |
| 8 | Gentoo ebuild (source-based, USE flags) | desktop+embedded | not-started | niche | P6 | M | yes | yes | yes | yes | yes | no |
| 9 | Linux distro-key package signing (RPM GPG, Debian signed-by) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 10 | Flatpak portal / sandbox profile (Bubblewrap, xdg-desktop-portal) | desktop+embedded | not-started | mixed | P6 | L | yes | yes | yes | no | no | no |

## Section 2 — macOS distribution / packaging

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 11 | .dmg disk image (drag-to-Applications) | desktop | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 12 | .pkg Installer (pre/post install scripts) | desktop | not-started | widespread | P6 | M | partial | yes | yes | no | no | no |
| 13 | Homebrew Cask | desktop | not-started | widespread | P6 | S | yes | yes | yes | yes | yes | no |
| 14 | Mac App Store distribution (sandboxed) | desktop | not-started | mixed | P6 | L | no | no | yes | no | no | no |
| 15 | Apple notarisation (notary service, notarytool) | desktop | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 16 | Gatekeeper (quarantine, xattr, code-sign verification) | desktop | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 17 | Hardened Runtime entitlements (library-validation, JIT) | desktop | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 18 | macOS sandbox profile (Seatbelt, SBPL) | desktop | designed | ubiquitous | P6 | L | yes | yes | yes | no | no | no |
| 19 | App Sandbox (entitlement-based) | desktop | not-started | mixed | P6 | L | yes | partial | yes | no | no | no |
| 20 | Sparkle framework for auto-update (legacy, BSD) | desktop | not-started | niche | P6 | M | no | no | no | no | no | no |

## Section 3 — Windows distribution / packaging

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 21 | .exe installer (NSIS, Inno Setup, MSI) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 22 | MSIX (modern Windows app packaging) | desktop+embedded | not-started | niche | P6 | M | yes | no | no | no | no | no |
| 23 | Microsoft Store distribution | desktop+embedded | not-started | mixed | P6 | M | yes | no | no | no | no | no |
| 24 | winget package manager | desktop | not-started | mixed | P6 | S | yes | yes | no | yes | no | no |
| 25 | Chocolatey package | desktop | not-started | mixed | P6 | S | yes | yes | no | yes | no | no |
| 26 | Scoop package | desktop | not-started | niche | P6 | S | yes | yes | no | yes | yes | no |
| 27 | Authenticode code signing (EV / OV certificates) | desktop | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 28 | SmartScreen reputation (anti-malware) | desktop | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |

## Section 4 — Mobile distribution / packaging

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 29 | Android APK (sideload) | mobile+embedded | not-started | mixed | P6 | S | yes | yes | no | no | no | no |
| 30 | Google Play distribution | mobile | not-started | ubiquitous | P6 | L | yes | yes | no | no | no | no |
| 31 | F-Droid (open-source, reproducible builds) | mobile | not-started | experimental | P6 | M | no | yes | no | no | no | no |
| 32 | Amazon Appstore (Kindle Fire, Fire TV) | mobile+embedded | not-started | experimental | P6 | S | yes | yes | no | no | no | no |
| 33 | iOS App Store (mandatory; WebKit engine required on iOS) | mobile | not-started | ubiquitous | P6 | XL | no | no | yes | no | no | no |
| 34 | Android App Bundle (AAB) | mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |

## Section 5 — Auto-update channels, signing, rollback

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 35 | Differential (binary-delta) update | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no | no |
| 36 | Background update (applied on next start) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 37 | Staged rollout (percentage-based, metric-driven) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 38 | Release channels (stable, beta, dev, canary / nightly) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | partial | partial | no |
| 39 | Update server (signed manifest endpoint) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | yes | yes | yes | yes | partial | no |
| 40 | Update payload signing (Ed25519 / RSA-2048) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | yes | no | no |
| 41 | Rollback on update failure (restore previous version) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 42 | Hot reload / restartless update (partial binary hot-swap) | desktop | not-started | niche | P6 | XL | no | partial | no | no | no | no |
| 43 | About this browser, Restart to update UX | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 44 | Update notification badge (hamburger / Settings / status bar) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |

## Section 6 — Installation / uninstallation semantics

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 45 | Per-user install (no admin, AppData / ~/.local) | desktop+embedded | not-started | mixed | P6 | S | yes | yes | yes | no | no | no |
| 46 | Per-machine install (system-wide, requires admin) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 47 | MSI transform (enterprise customisation) | desktop | not-started | mixed | P6 | M | partial | yes | no | no | no | no |
| 48 | NSIS custom installer (uninstall keys, MUI) | desktop | not-started | mixed | P6 | M | partial | yes | no | no | no | no |
| 49 | Uninstallation — profile retained by default | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 50 | Uninstallation — explicit remove my data option | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 51 | Uninstallation — registry / daemon cleanup (no orphans) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |

## Section 7 — OS integration: default-handler and registration

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 52 | Default-browser registration (per-user) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 53 | Default-browser registration (per-machine / system-wide) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 54 | Protocol-handler registration (http, https, ftp, file) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 55 | Custom web+ protocol scheme registration | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 56 | Mail protocol handler (mailto:) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 57 | Telephony / SMS / calendar / geo protocol handlers | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 58 | File-extension association (.html, .htm, .svg, .pdf, .mht) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 59 | Native messaging host (extension to OS app, manifest registration) | desktop | not-started | mixed | P6 | L | yes | yes | partial | no | no | no |

## Section 8 — OS integration: shell and system services

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 60 | OS share sheet (macOS, Windows, Android, iOS) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 61 | OS file dialogs (open, save, choose directory) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | partial | partial | no |
| 62 | OS notifications (native API, action buttons, progress, image) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 63 | OS taskbar jump lists (Windows) | desktop | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 64 | OS dock badge counts (macOS NSDockTile) | desktop | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 65 | OS task progress (Windows ITaskbarList3) | desktop | not-started | widespread | P6 | M | yes | yes | no | no | no | no |
| 66 | OS task overlay icon (Windows small overlay) | desktop | not-started | mixed | P6 | M | yes | yes | no | no | no | no |
| 67 | OS app menu (macOS top-bar menubar) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 68 | OS title bar (custom-drawn / native / traffic lights / vibrancy) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 69 | OS system tray (tray icon, tray menu, minimised-to-tray) | desktop | not-started | widespread | P6 | M | yes | yes | no | no | no | no |
| 70 | OS auto-launch on login (login items, scheduled tasks) | desktop+mobile+embedded | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 71 | OS dark mode (follow system, override) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | yes | yes | no |
| 72 | OS accent colour (system accent) | desktop+mobile+embedded | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 73 | OS font (system font, font fallbacks) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | yes | yes | no |
| 74 | OS appearance (vibrancy, transparency, native chrome) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |
| 75 | OS user account (per-user profile, multi-user, guest mode) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no | no |
| 76 | OS screen lock — pause updates, hide notifications | desktop+mobile+embedded | not-started | mixed | P6 | S | yes | yes | yes | no | no | no |
| 77 | OS power (sleep/wake handling, low-power mode) | desktop+mobile+embedded | not-started | mixed | P6 | M | yes | yes | yes | no | no | no |
| 78 | OS accessibility (VoiceOver, NVDA, Narrator, Orca integration) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | yes | yes | yes | no | partial | no |
| 79 | OS input (IME, keyboard layouts, mouse, touch, trackpad) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | yes | partial | no |
| 80 | OS printing (native print dialog, preview, save as PDF) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 81 | OS media keys (play/pause, next/prev, volume) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |
| 82 | OS spellcheck (native spellcheck, custom dictionary) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 83 | OS clipboard (copy/paste/cut, clipboard history) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | partial | partial | no |
| 84 | OS drag-and-drop (native, file, text, URL drag) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | partial | partial | no |
| 85 | OS voice / speech (native speech, dictation) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | no | yes | no | no | no |
| 86 | OS hardware pairing prompt (Bluetooth, USB, serial, NFC) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |

## Section 9 — Sandbox model and process isolation

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 87 | Process isolation — per-site (one process per site) | desktop+mobile+embedded | designed | ubiquitous | P5 | XL | yes | yes | yes | no | no | no |
| 88 | Process isolation — per-frame (cross-origin iframe) | desktop+mobile+embedded | designed | ubiquitous | P5 | XL | yes | yes | yes | no | no | no |
| 89 | Process isolation — per-tab (one process per top-level tab) | desktop+mobile+embedded | designed | ubiquitous | P5 | L | yes | yes | yes | partial | no | no |
| 90 | Process isolation — per-extension | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 91 | OS sandbox — Linux Landlock (kernel 5.13+) | desktop+embedded | not-started | mixed | P6 | L | no | no | no | no | no | no |
| 92 | OS sandbox — Linux seccomp-bpf | desktop+embedded | not-started | ubiquitous | P6 | L | yes | yes | no | no | no | no |
| 93 | OS sandbox — Linux user + mount namespaces | desktop+embedded | not-started | mixed | P6 | M | yes | yes | no | no | no | no |
| 94 | OS sandbox — Linux AppArmor profile | desktop+embedded | not-started | niche | P6 | M | partial | partial | no | no | no | no |
| 95 | OS sandbox — macOS Seatbelt (sandbox profile language) | desktop | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no | no |
| 96 | OS sandbox — macOS App Sandbox (entitlements) | desktop | not-started | mixed | P6 | M | yes | no | yes | no | no | no |
| 97 | OS sandbox — Windows Restricted Token + Job Object + integrity levels | desktop | not-started | ubiquitous | P6 | L | yes | yes | no | no | no | no |
| 98 | OS sandbox — Windows AppContainer (lowbox) | desktop+embedded | not-started | mixed | P6 | M | yes | no | no | no | no | no |
| 99 | Privilege separation — broker / sandboxed processes / IPC | desktop+mobile+embedded | designed | ubiquitous | P5 | XL | yes | yes | yes | no | no | no |
| 100 | GPU process — separate, isolated | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | no | no | no |
| 101 | Network process — separate, isolated | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | no | no | no |
| 102 | Storage process — separate, isolated | desktop+mobile+embedded | not-started | niche | P5 | L | partial | no | no | no | no | no |
| 103 | Utility process — per-feature (audio, network decode, FS) | desktop+mobile+embedded | not-started | mixed | P5 | M | yes | partial | yes | no | no | no |
| 104 | Memory pressure — reduce memory in low-memory conditions | desktop+mobile+embedded | not-started | widespread | P5 | M | yes | yes | yes | no | no | no |
| 105 | Exploit mitigations (CFI, ASLR, CFG, sandbox escape detection) | desktop+mobile+embedded | not-started | ubiquitous | P6 | XL | yes | yes | yes | partial | partial | no |
| 106 | Privilege escalation hardening (no-new-privileges, drop capabilities) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |

## Section 10 — Telemetry and crash reporting

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 107 | Crash reporter (opt-in / opt-out / no telemetry) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 108 | Crash dump — minidump (Breakpad / Crashpad) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 109 | Crash dump — full dump (complete memory, opt-in only) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | no | no | no | no |
| 110 | Telemetry — opt-in (no data without explicit consent) | desktop+mobile+embedded | not-started | mixed | P6 | M | no | no | yes | no | no | no |
| 111 | Telemetry — opt-out (data collected by default, user can disable) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 112 | Telemetry — anonymisation (differential privacy, k-anonymity, RAPPOR) | desktop+mobile+embedded | not-started | widespread | P6 | L | yes | yes | yes | no | no | no |
| 113 | Usage statistics (anonymised, opt-in) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 114 | Help improve surveys / feedback prompts | desktop+mobile+embedded | not-started | mixed | P6 | S | yes | yes | no | no | no | no |
| 115 | Onboarding telemetry opt-in (first-run prompt) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 116 | Privacy-respecting telemetry — noise, randomised (Prio/OHTTP/RAPPOR) | desktop+mobile+embedded | not-started | mixed | P6 | L | yes | yes | yes | no | no | no |

## Section 11 — Default browser and first-run experience

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 117 | Default-browser check (first-run prompt) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 118 | Make default flow (settings page, OS integration dialog) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 119 | First-run experience (onboarding wizard) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 120 | Profile import on first run (from Chromium, Firefox, WebKit, etc.) | desktop | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no | no |
| 121 | Sign-in / sign-up on first run (account sync) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | yes | yes | yes | no | no | no |
| 122 | Theme and dark mode selection on first run | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 123 | Default search engine selection on first run (EU DSA mandate) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 124 | Privacy-respecting defaults (tracking protection ON, telemetry OFF) | desktop+mobile+embedded | not-started | widespread | P6 | M | partial | yes | yes | no | no | no |

## Section 12 — Policy and enterprise deployment

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 125 | Group Policy / plist / JSON policy (Windows GPO, macOS plist, Linux JSON) | desktop+embedded | not-started | ubiquitous | P6 | L | yes | yes | no | no | no | no |
| 126 | Recommended policies (managed config, not enforced) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 127 | Required policies (mandatory, cannot be overridden by user) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 128 | Mandatory extensions (ExtensionInstallForcelist) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 129 | Extension block list (ExtensionInstallBlocklist) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 130 | ExtensionSettings policy (fine-grained extension control) | desktop+embedded | not-started | ubiquitous | P6 | L | yes | yes | no | no | no | no |
| 131 | Auto-updates disabled (enterprise freeze) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 132 | Default browser forced (cannot be changed by user) | desktop+embedded | not-started | widespread | P6 | S | yes | yes | no | no | no | no |
| 133 | Allow / deny URL lists (URLBlocklist, URLAllowlist) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 134 | Proxy forced (ProxyMode, ProxyServer, ProxyPacUrl) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 135 | Search engine forced (DefaultSearchProviderName, URL, keyword) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 136 | Bookmark forced (ManagedBookmarks) | desktop+embedded | not-started | ubiquitous | P6 | M | yes | yes | no | no | no | no |
| 137 | Home page forced (HomepageLocation, HomepageIsNewTabPage) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 138 | SafeBrowsing force / disable (SafeBrowsingEnabled) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 139 | Password manager policy (PasswordManagerEnabled) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 140 | Autofill policy (AutoFillEnabled) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 141 | Tracking protection policy (TrackingProtectionMode) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 142 | Translate policy (TranslateEnabled) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | no | no | no | no | no |
| 143 | Default download directory (DownloadDirectory) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 144 | Print policy (PrintingEnabled, PrintHeaderFooter) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 145 | Developer tools disabled (DeveloperToolsAvailability) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | yes | no | no | no | no |
| 146 | SmartScreen for websites (SmartScreenEnabled) | desktop+embedded | not-started | ubiquitous | P6 | S | yes | no | no | no | no | no |

## Section 13 — Diagnostics surfaces

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 147 | about:support / support report (profile, GPU, extensions, modified prefs) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |
| 148 | about:telemetry / telemetry viewer (see what data is collected) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | no | no | no | no |
| 149 | Crash reporter UI (send / don't send, crash details) | desktop+mobile+embedded | not-started | ubiquitous | P6 | S | yes | yes | yes | no | no | no |
| 150 | Encrypted crash reports (HTTPS upload, encrypted at rest) | desktop+mobile+embedded | not-started | ubiquitous | P6 | M | yes | yes | yes | no | no | no |

---

**Total rows: 150** (106 distribution/platform + 44 enterprise/policy)
