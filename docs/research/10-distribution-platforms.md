# Chunk 11 — Distribution & Platform Integration

> **Chunk 11 of 14.** This chunk is the surface that makes the browser
> ship to users. It covers installers, update channels, OS sandboxing,
> platform integration, default-handler registration, telemetry, crash
> reporting, and policy / enterprise deployment. The web-platform
> surface (chunks 1, 6) and the engine-level security model (chunk 3)
> are deliberately out of scope; this chunk is the *OS-facing* shell
> around the engine.
>
> **Worktree:** `research/competitive-parity` (base: `audit/m4-window`).
> **Methodology contract:** `00-methodology.md`. **Source ladder:**
> `citations/sources.md`. **Output contract:** `README.md` §"Per-chunk
> output contract".
>
> **Naming:** spec / capability names per methodology §7. Engine
> identities (Chromium, Firefox, WebKit, Servo, Ladybird, Flow) are
> allowed because they are engine identities, not product brands.
>
> **Splitting:** the file splits into two parts because the row set
> exceeds the 600-line cap. Sections 1–9 live in
> `10-distribution-platforms.md` (this file); sections 10–13 live in
> `10-enterprise-policy.md` (companion). An index sits at the end of
> this file.

---

## Scope

**In:** distribution / packaging (Linux, macOS, Windows, mobile);
auto-update channels and signing; installation / uninstallation
semantics; OS integration (default handler, protocol / file-type
registration, native messaging, share sheet, file dialogs,
notifications, taskbar / dock, app menu, title bar, system tray,
dark mode, accent colour, fonts, appearance, profiles, screen lock,
power, accessibility, input, printing, media keys, spellcheck,
clipboard, drag-and-drop, speech, hardware pairing); sandbox model
(process model, OS-level sandbox per platform, privilege separation,
GPU / network / storage / utility / renderer process isolation,
memory pressure, exploit mitigations).

**Out:** web platform APIs (chunk 6), WebExtensions API (chunk 10),
security policy engine (chunk 3), storage (chunk 4), media (chunk 5),
UX chrome (chunk 7), DevTools (chunk 8), accessibility primitives
(chunk 9), network transport (chunk 2), telemetry / crash / policy /
enterprise (companion file `10-enterprise-policy.md`).

---

## Methodology for this chunk

Rows derived from: Tier 1 platform documentation (Apple Developer
Documentation, Microsoft Learn, Freedesktop specifications, Linux
man-pages), Tier 2 references (MDN `registerProtocolHandler`, Can I
Use), Tier 3 vendor / engine release notes and architecture blogs
(Chromium sandbox design docs, Mozilla Hacks, WebKit blog, Servo
release notes, Ladybird blog), Tier 4 third-party material (EU CRA
for SBOM, privacytests.org for privacy posture), Tier 5 distro-
package indexes (Debian, Fedora, Arch AUR, Homebrew, Nixpkgs,
Gentoo) for packaging-format coverage. Sandbox references lean on
**Chromium Sandbox design doc**, **Linux man-pages for `seccomp`,
`landlock`, `namespaces`, `prctl`**, **Apple Seatbelt Scheme
language**, **Microsoft AppContainer / Restricted Token** docs.

**Engine notes** are brief (one line per engine) per the contract;
chunk 7 is the engine-coverage deep dive.

**Spiral ground truth (verified 2026-06-16):**

- `spiral-sandbox` exists with a 53-line `lib.rs` exposing
  `SandboxConfig` (restrict_filesystem / network / process) and a
  `Sandbox::new()` constructor. No per-platform backends implemented.
  Marked `⛔ Deferred` per `GAP_ANALYSIS.md:5.4`.
- `spiral-browser` is the binary surface; it has no installer, no
  default-browser registration, no auto-update, no crash reporter,
  no telemetry, no policy surface.
- `spiral-ipc` provides the `IpcTransport` trait + Unix/Windows
  transport impls + bincode framing. The multi-process model needed
  for the sandbox section is *designed* in
  `docs/architecture/design/shared-everything.md` (Bet 1) but the runtime
  is M25+ per `GAP_ANALYSIS.md:1.7`.
- `ROADMAP.md` Phase 6 (M61–84) explicitly lists "cross-platform
  packaging, v0.1.0".
- `GAP_ANALYSIS.md:4.1` flags `spiral-sandbox` as Phase 4 / do not
  touch, re-evaluated under Bet 1.
- `GAP_ANALYSIS.md:4.1` (last row) confirms: "Telemetry / phone-
  home — None — by design."

---

## Section 1 — Linux distribution / packaging

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 1 | `.deb` package (Debian, Ubuntu, derivatives) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a on Linux; Servo: no; Ladybird: no; Flow: no. | https://www.debian.org/doc/debian-policy/ |
| 2 | `.rpm` package (Fedora, RHEL, openSUSE) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://rpm.org/ ; https://docs.fedoraproject.org/en-US/packaging-guidelines/ |
| 3 | AppImage (portable, no-install) | desktop | not-started | 50-75% | P6 | M | Chromium: no; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://appimage.org/ |
| 4 | Flatpak (Flathub, sandboxed) | desktop+embedded | not-started | 50-75% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable (WebKitGTK); Servo: no; Ladybird: no; Flow: no. | https://docs.flatpak.org/ |
| 5 | Snap (Ubuntu Store, snapd, confined) | desktop+embedded | not-started | 50-75% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://snapcraft.io/docs |
| 6 | Arch Linux AUR (PKGBUILD) | desktop+embedded | not-started | 75-90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: yes/stable; Ladybird: yes/stable; Flow: no. | https://wiki.archlinux.org/title/AUR |
| 7 | Nixpkgs derivation | desktop+embedded | not-started | 25-50% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: yes/stable; Ladybird: yes/stable; Flow: no. | https://nixos.org/manual/nixpkgs/stable/ |
| 8 | Gentoo ebuild (source-based, USE flags) | desktop+embedded | not-started | 25-50% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: yes/stable; Ladybird: yes/stable; Flow: no. | https://devmanual.gentoo.org/ |
| 9 | Linux distro-key package signing (RPM GPG, Debian `signed-by`) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://wiki.debian.org/SecureApt |
| 10 | Flatpak portal / sandbox profile (Bubblewrap, xdg-desktop-portal) | desktop+embedded | not-started | 50-75% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://docs.flatpak.org/en/latest/portal-api.html |

---

## Section 2 — macOS distribution / packaging

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 11 | `.dmg` disk image (drag-to-Applications) | desktop | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/bundleresources/disk_image_files |
| 12 | `.pkg` Installer (pre/post install scripts) | desktop | not-started | 75-90% | P6 | M | Chromium: partial (enterprise only); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/installer |
| 13 | Homebrew Cask | desktop | not-started | 75-90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: yes/stable; Ladybird: yes/stable; Flow: no. | https://github.com/Homebrew/homebrew-cask |
| 14 | Mac App Store distribution (sandboxed) | desktop | not-started | 50-75% | P6 | L | Chromium: no (engine-swap rules); Firefox: no; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/app-store/review/guidelines/ |
| 15 | Apple notarisation (notary service, `notarytool`) | desktop | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution |
| 16 | Gatekeeper (quarantine, xattr, code-sign verification) | desktop | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/security/hardened_runtime |
| 17 | Hardened Runtime entitlements (library-validation, JIT) | desktop | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/security/hardened_runtime_entitlements |
| 18 | macOS sandbox profile (Seatbelt, SBPL) | desktop | designed (no impl) | >=90% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/security/sandbox |
| 19 | App Sandbox (entitlement-based) | desktop | not-started | 50-75% | P6 | L | Chromium: yes/stable; Firefox: partial; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/security/app_sandbox |
| 20 | Sparkle framework for auto-update (legacy, BSD) | desktop | not-started | niche | P6 | M | Chromium: no (Keystone); Firefox: no (Balrog); WebKit: no (Software Update); Servo: no; Ladybird: no; Flow: no. | https://sparkle-project.org/documentation/ |

---

## Section 3 — Windows distribution / packaging

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 21 | `.exe` installer (NSIS, Inno Setup, MSI) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable (NSIS + MSI); WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://nsis.sourceforge.io/ |
| 22 | MSIX (modern Windows app packaging) | desktop+embedded | not-started | 25-50% | P6 | M | Chromium: yes/stable (Edge); Firefox: no; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/msix/ |
| 23 | Microsoft Store distribution | desktop+embedded | not-started | 50-75% | P6 | M | Chromium: yes/stable (Edge); Firefox: no; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/apps/publish/ |
| 24 | `winget` package manager | desktop | not-started | 50-75% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: yes/stable; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/package-manager/ |
| 25 | Chocolatey package | desktop | not-started | 50-75% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: yes/stable; Ladybird: no; Flow: no. | https://docs.chocolatey.org/en-us/ |
| 26 | Scoop package | desktop | not-started | 25-50% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: yes/stable; Ladybird: yes/stable; Flow: no. | https://scoop.sh/ |
| 27 | Authenticode code signing (EV / OV certificates) | desktop | not-started | >=90% | P6 | M | Chromium: yes/stable (EV); Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows-hardware/drivers/install/authenticode |
| 28 | SmartScreen reputation (anti-malware) | desktop | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/deployedge/microsoft-edge-security-smartscreen |

---

## Section 4 — Mobile distribution / packaging

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 29 | Android APK (sideload) | mobile+embedded | not-started | 50-75% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://developer.android.com/guide/topics/app-package |
| 30 | Google Play distribution | mobile | not-started | >=90% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://support.google.com/googleplay/android-developer/ |
| 31 | F-Droid (open-source, reproducible builds) | mobile | not-started | <25% | P6 | M | Chromium: no; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://f-droid.org/docs/ |
| 32 | Amazon Appstore (Kindle Fire, Fire TV) | mobile+embedded | not-started | <25% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://developer.amazon.com/docs/app-submission/why-amazon-appstore.html |
| 33 | iOS App Store (mandatory; WebKit engine required on iOS) | mobile | not-started | >=90% | P6 | XL | Chromium: no (WebKit mandate); Firefox: no (WebKit-backed build); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/app-store/review/guidelines/ |
| 34 | Android App Bundle (AAB) | mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://developer.android.com/guide/app-bundle |

---

## Section 5 — Auto-update channels, signing, rollback

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 35 | Differential (binary-delta) update | desktop+mobile+embedded | not-started | >=90% | P6 | L | Chromium: yes/stable (Courgette + bsdiff); Firefox: yes/stable (partial MAR); WebKit: yes/stable (SUDelta); Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/components/courgette/ ; https://wiki.mozilla.org/Software_Update:MAR |
| 36 | Background update (applied on next start) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/update/ |
| 37 | Staged rollout (percentage-based, metric-driven) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (Omaha cohort); Firefox: yes/stable (Balrog); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/docs/updater/protocol.html |
| 38 | Release channels: stable, beta, dev, canary / nightly | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (4 channels); Firefox: yes/stable (stable, beta, nightly, ESR); WebKit: yes/stable (Safari Technology Preview); Servo: partial (nightly only); Ladybird: no (nightly only); Flow: no. | https://www.chromium.org/developers/calendar ; https://wiki.mozilla.org/Release_Management/Calendar |
| 39 | Update server (signed manifest endpoint) | desktop+mobile+embedded | not-started | >=90% | P6 | L | Chromium: yes/stable (Omaha); Firefox: yes/stable (Balrog / aus5.mozilla.org); WebKit: yes/stable (gdmf.apple.com); Servo: yes/stable (GitHub Releases); Ladybird: partial; Flow: no. | https://wiki.mozilla.org/Software_Update |
| 40 | Update payload signing (Ed25519 / RSA-2048) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (RSA-2048); Firefox: yes/stable (RSA-2048); WebKit: yes/stable (Code Signing Blobs); Servo: yes/stable (Ed25519); Ladybird: no; Flow: no. | https://wiki.mozilla.org/MAR |
| 41 | Rollback on update failure (restore previous version) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://wiki.mozilla.org/Software_Update:Recovery |
| 42 | Hot reload / restartless update (partial binary hot-swap) | desktop | not-started | niche | P6 | XL | Chromium: no; Firefox: partial (script swap only); WebKit: no; Servo: no; Ladybird: no; Flow: no. | https://wiki.mozilla.org/Software_Update:Hot_Swap |
| 43 | "About this browser" → "Restart to update" UX | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://support.mozilla.org/en-US/kb/update-firefox-latest-version |
| 44 | Update notification badge (hamburger / Settings / status bar) | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/ui/ |

---

## Section 6 — Installation / uninstallation semantics

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 45 | Per-user install (no admin, AppData / `~/.local`) | desktop+embedded | not-started | 50-75% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/chrome/installer/ |
| 46 | Per-machine install (system-wide, requires admin) | desktop+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/msi/installation-context |
| 47 | MSI transform (enterprise customisation) | desktop | not-started | 50-75% | P6 | M | Chromium: partial; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/msi/transforms |
| 48 | NSIS custom installer (uninstall keys, MUI) | desktop | not-started | 50-75% | P6 | M | Chromium: partial (legacy); Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://nsis.sourceforge.io/Docs/Chapter4.html |
| 49 | Uninstallation — profile retained by default | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://support.google.com/chrome/answer/95319 |
| 50 | Uninstallation — explicit "remove my data" option | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://support.mozilla.org/en-US/kb/refresh-firefox-reset-add-ons-and-settings |
| 51 | Uninstallation — registry / daemon cleanup (no orphans) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/msi/uninstall-register |

---

## Section 7 — OS integration: default-handler and registration

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 52 | Default-browser registration (per-user: registry / `defaults write` / `xdg-mime`) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://html.spec.whatwg.org/multipage/system-state.html#custom-handlers |
| 53 | Default-browser registration (per-machine / system-wide) | desktop+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/ |
| 54 | Protocol-handler registration (`http`, `https`, `ftp`, `file`) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://html.spec.whatwg.org/multipage/system-state.html#custom-handlers ; https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler |
| 55 | Custom `web+` protocol scheme registration | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://html.spec.whatwg.org/multipage/system-state.html#custom-handlers |
| 56 | Mail protocol handler (`mailto:`) | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://html.spec.whatwg.org/multipage/system-state.html#custom-handlers |
| 57 | Telephony / SMS / calendar / geo protocol handlers (`tel:`, `sms:`, `webcal:`, `geo:`) | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://html.spec.whatwg.org/multipage/system-state.html#custom-handlers |
| 58 | File-extension association (`.html`, `.htm`, `.xhtml`, `.svg`, `.pdf`, `.mht`, `.mhtml`) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/shell/fa-file-types ; https://developer.apple.com/documentation/coreservices/launch_services |
| 59 | Native messaging host (extension ↔ OS app, manifest registration) | desktop | not-started | 50-75% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: partial; Servo: no; Ladybird: no; Flow: no. | https://developer.chrome.com/docs/apps/nativeMessaging ; https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests |

---

## Section 8 — OS integration: shell and system services

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 60 | OS share sheet (macOS share, Windows share, Android share, iOS share) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://w3c.github.io/web-share/ |
| 61 | OS file dialogs (open file, save file, choose directory) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: partial; Ladybird: partial; Flow: no. | https://html.spec.whatwg.org/multipage/input.html#file-upload-state-(type=file) |
| 62 | OS notifications (native notification API, action buttons, progress, image) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (Rich Notifications); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://notifications.spec.whatwg.org/ |
| 63 | OS taskbar jump lists (Windows) | desktop | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/shell/taskbar-extensions |
| 64 | OS dock badge counts (macOS — NSDockTile) | desktop | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/appkit/nsdocktile |
| 65 | OS task progress (Windows — ITaskbarList3) | desktop | not-started | 75-90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3 |
| 66 | OS task overlay icon (Windows — small icon overlay) | desktop | not-started | 50-75% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3 |
| 67 | OS app menu (macOS — top-bar application menu, menubar) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/design/human-interface-guidelines/the-menu-bar |
| 68 | OS title bar (custom-drawn / native / hidden; traffic lights; vibrancy) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (custom-drawn on macOS); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/appkit/nswindow ; https://learn.microsoft.com/en-us/windows/apps/design/style/title-bar |
| 69 | OS system tray (tray icon, tray menu, minimised-to-tray) | desktop | not-started | 75-90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-notifyicondataa ; https://developer.apple.com/documentation/appkit/nsstatusitem |
| 70 | OS auto-launch on login (login items, scheduled tasks) | desktop+mobile+embedded | not-started | 75-90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/servicemanagement ; https://learn.microsoft.com/en-us/windows/win32/taskschd/task-scheduler-objects |
| 71 | OS dark mode (follow system, override) | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: yes/stable; Ladybird: yes/stable; Flow: no. | https://developer.apple.com/documentation/appkit/supporting_dark_mode_in_your_interface |
| 72 | OS accent colour (system accent) | desktop+mobile+embedded | not-started | 75-90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/apps/design/style/color |
| 73 | OS font (system font, font fallbacks) | desktop+mobile+embedded | not-started | >=90% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: yes/stable; Ladybird: yes/stable; Flow: no. | https://developer.mozilla.org/en-US/docs/Web/CSS/font-family |
| 74 | OS appearance (vibrancy, transparency, native chrome) | desktop+mobile+embedded | not-started | 75-90% | P6 | M | Chromium: yes/stable (NSVisualEffectView / Acrylic); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/appkit/nsvisualeffectview ; https://learn.microsoft.com/en-us/windows/apps/design/style/acrylic |
| 75 | OS user account (per-user profile, multi-user, guest mode) | desktop+mobile+embedded | not-started | >=90% | P6 | L | Chromium: yes/stable (multi-profile + guest); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/profiles/ |
| 76 | OS screen lock — pause updates, hide notifications | desktop+mobile+embedded | not-started | 50-75% | P6 | S | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/notifications/ |
| 77 | OS power (sleep / wake handling, low-power mode) | desktop+mobile+embedded | not-started | 50-75% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/win32/power/system-power-status |
| 78 | OS accessibility (VoiceOver, NVDA, Narrator, Orca integration) | desktop+mobile+embedded | not-started | >=90% | P6 | L | Chromium: yes/stable (AT-SPI, UIA, AX); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: partial; Flow: no. | https://www.w3.org/TR/wai-aria-1.2/ |
| 79 | OS input (IME, keyboard layouts, mouse, touch, trackpad) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: yes/stable; Ladybird: partial; Flow: no. | https://w3c.github.io/uievents/ |
| 80 | OS printing (native print dialog, preview, save as PDF) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#printing |
| 81 | OS media keys (play / pause, next / prev, volume) | desktop+mobile+embedded | not-started | 75-90% | P6 | M | Chromium: yes/stable (Media Session API); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://w3c.github.io/mediasession/ |
| 82 | OS spellcheck (native spellcheck, custom dictionary) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable (Hunspell); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.mozilla.org/en-US/docs/Web/API/Element/spellcheck |
| 83 | OS clipboard (copy / paste / cut, clipboard history) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: partial; Ladybird: partial; Flow: no. | https://w3c.github.io/clipboard-apis/ |
| 84 | OS drag-and-drop (native drag, file drag, text drag, URL drag) | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: partial; Ladybird: partial; Flow: no. | https://html.spec.whatwg.org/multipage/dnd.html |
| 85 | OS voice / speech (native speech, dictation) | desktop+mobile+embedded | not-started | 75-90% | P6 | M | Chromium: yes/stable (Web Speech API); Firefox: no; WebKit: yes/stable (Siri dictation on iOS); Servo: no; Ladybird: no; Flow: no. | https://wicg.github.io/speech-api/ |
| 86 | OS hardware pairing prompt (Bluetooth, USB, serial, NFC — chrome prompt gates the deep API) | desktop+mobile+embedded | not-started | 75-90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://webbluetoothcg.github.io/web-bluetooth/ ; https://wicg.github.io/webusb/ |

---

## Section 9 — Sandbox model and process isolation

| # | Capability | Surface (desktop/mobile/embedded) | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|------------|----------------------------------|------------------|--------------------|---------------|------------|--------------|---------|
| 87 | Process isolation — per-site (one process per site) | desktop+mobile+embedded | designed (M25+) | >=90% | P5 | XL | Chromium: yes/stable (Site Isolation); Firefox: yes/stable (Fission); WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/docs/security/site-isolation.md ; https://wiki.mozilla.org/Project_Fission |
| 88 | Process isolation — per-frame (cross-origin iframe) | desktop+mobile+embedded | designed (M25+) | >=90% | P5 | XL | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/docs/security/site-isolation.md |
| 89 | Process isolation — per-tab (one process per top-level tab) | desktop+mobile+embedded | designed (M25+) | >=90% | P5 | L | Chromium: yes/stable; Firefox: yes/stable (e10s); WebKit: yes/stable (WebKit2); Servo: partial; Ladybird: no; Flow: no. | https://wiki.mozilla.org/Electrolysis |
| 90 | Process isolation — per-extension | desktop+mobile+embedded | not-started | >=90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.chrome.com/docs/extensions/mv3/architecture |
| 91 | OS sandbox — Linux Landlock (kernel 5.13+) | desktop+embedded | not-started | 50-75% | P6 | L | Chromium: no (uses seccomp + namespaces); Firefox: no; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://docs.kernel.org/userspace-api/landlock.html |
| 92 | OS sandbox — Linux seccomp-bpf | desktop+embedded | not-started | >=90% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://www.kernel.org/doc/Documentation/prctl/seccomp_filter.txt ; https://chromium.googlesource.com/chromium/src/+/main/sandbox/linux/ |
| 93 | OS sandbox — Linux user + mount namespaces | desktop+embedded | not-started | 50-75% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://man7.org/linux/man-pages/man7/namespaces.7.html |
| 94 | OS sandbox — Linux AppArmor profile | desktop+embedded | not-started | 25-50% | P6 | M | Chromium: partial (distro-shipped); Firefox: partial; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://gitlab.com/apparmor/apparmor/-/wikis/Documentation |
| 95 | OS sandbox — macOS Seatbelt (sandbox profile language) | desktop | not-started | >=90% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/security/sandbox ; https://chromium.googlesource.com/chromium/src/+/main/sandbox/mac/ |
| 96 | OS sandbox — macOS App Sandbox (entitlements) | desktop | not-started | 50-75% | P6 | M | Chromium: yes/stable (App Store); Firefox: no; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://developer.apple.com/documentation/security/app_sandbox |
| 97 | OS sandbox — Windows Restricted Token + Job Object + integrity levels | desktop | not-started | >=90% | P6 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/sandbox/win/ ; https://learn.microsoft.com/en-us/windows/win32/secauthz/restricted-tokens |
| 98 | OS sandbox — Windows AppContainer (lowbox) | desktop+embedded | not-started | 50-75% | P6 | M | Chromium: yes/stable (renderer since 2021); Firefox: no; WebKit: n/a; Servo: no; Ladybird: no; Flow: no. | https://learn.microsoft.com/en-us/windows/security/identity-protection/access-control/access-tokens |
| 99 | Privilege separation — broker / sandboxed processes / IPC | desktop+mobile+embedded | designed (M25+) | >=90% | P5 | XL | Chromium: yes/stable (Broker / Renderer); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/docs/security/sandbox.md ; https://wiki.mozilla.org/Security/Sandbox |
| 100 | GPU process — separate, isolated | desktop+mobile+embedded | not-started | >=90% | P5 | L | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/gpu/ |
| 101 | Network process — separate, isolated | desktop+mobile+embedded | not-started | >=90% | P5 | L | Chromium: yes/stable (Network Service); Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/services/network/ |
| 102 | Storage process — separate, isolated | desktop+mobile+embedded | not-started | niche (Chromium developing) | P5 | L | Chromium: partial; Firefox: no; WebKit: no; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/services/storage/ |
| 103 | Utility process — per-feature (audio, network decode, FS access) | desktop+mobile+embedded | not-started | 50-75% | P5 | M | Chromium: yes/stable; Firefox: partial; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/services/ |
| 104 | Memory pressure — reduce memory in low-memory conditions | desktop+mobile+embedded | not-started | 75-90% | P5 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/docs/memory/ |
| 105 | Exploit mitigations — CFI, ASLR, CFG, sandbox escape detection | desktop+mobile+embedded | not-started | >=90% | P6 | XL | Chromium: yes/stable (CFGuard, CFG, CFI, sandbox escape detector); Firefox: yes/stable; WebKit: yes/stable; Servo: partial; Ladybird: partial; Flow: no. | https://chromium.googlesource.com/chromium/src/+/main/docs/security/sandbox.md |
| 106 | Privilege escalation hardening — no-new-privileges, drop capabilities | desktop+mobile+embedded | not-started | 75-90% | P6 | M | Chromium: yes/stable; Firefox: yes/stable; WebKit: yes/stable; Servo: no; Ladybird: no; Flow: no. | https://www.kernel.org/doc/Documentation/prctl/no_new_privs.txt |

---

## Cross-refs to `specs/GAP_ANALYSIS.md`

| GAP row | Title | Status in GAP | What it covers in this chunk |
|---------|-------|----------------|-----------------------------|
| 3.13 | Native platform integration | `⛔/❌` | Sections 7, 8 (rows 52–86) |
| 4.1 | `spiral-sandbox` per-platform profiles | `[~]` — Phase 4, do not touch | Section 9 (rows 87–106) |
| 4.1 | Telemetry / phone-home | `[x]` — "None — by design" | Companion §10 |
| 5.1 | Per-tab renderer process spawn | `[ ]` | Section 9 rows 87–89 |
| 5.4 | `spiral-sandbox` is a stub | `⛔ Deferred` | Section 9 |

No rows in `specs/GAP_ANALYSIS.md` currently cover the packaging rows
(Sections 1–4), the auto-update rows (Section 5), or the OS integration
rows (Sections 6–8). Chunk 13 (synthesis) will append Deltas.

---

## Index — sections in this file vs. companion

| File | Sections | Rows |
|------|----------|------|
| `10-distribution-platforms.md` (this file) | §1 Linux · §2 macOS · §3 Windows · §4 Mobile · §5 Auto-update · §6 Install · §7 Default-handler · §8 OS shell · §9 Sandbox | 1–106 |
| `10-enterprise-policy.md` (companion) | §10 Telemetry / crash · §11 Default browser / first run · §12 Policy / enterprise · §13 Diagnostics | 107–126 |

---

## Open questions for the user

1. **iOS engine swap.** Since 2020 the App Store review guidelines
   mandate WebKit on iOS. A v0.1 iOS build of Spiral *must* use
   WebKit (or skip iOS). Should row 33 be split into "iOS browser
   app (WebKit-backed)" vs. "iOS WebView host (WebKit only)"?
   Currently collapsed.

2. **Distro-target scope.** Section 1 lists 10 Linux distribution
   surfaces. For Phase 6 v0.1.0 the realistic minimum is `.deb`,
   `.rpm`, AppImage, AUR, Flatpak, and Snap (6 surfaces). Nix and
   Gentoo ebuild are nice-to-have. Which is the must-ship set?

3. **Update server hosting.** Sections 5 implies a Spiral update
   server. The cost is non-trivial: signing infrastructure, cohort
   logic, telemetry-free staging. Community mirror model (anyone
   can mirror signed payloads) or centralised Mozilla-style model?
   Currently undecided.

4. **Sandbox vs. capability-typed design.** `docs/architecture-
   shared-everything.md` Bet 1 is "capability-typed, not OS-level".
   Section 9 rows 91–98 are about OS-level sandbox (Landlock,
   Seatbelt, AppContainer) which is the M25+ escalated mode. Is
   v0.1.0 default *capability-only* (no OS sandbox), or is *OS
   sandbox* the default? Currently "OS sandbox optional, capability
   types default" per the design doc.

5. **Telemetry posture in v0.1.0.** `GAP_ANALYSIS.md:4.1` says
   "Telemetry / phone-home — None — by design." But the *crash
   reporter* half is opt-in and not in the GAP. Is v0.1.0 posture
   "no telemetry at all", "opt-in crash reporting only", or
   "opt-out crash reporting"?

6. **Mac App Store.** Row 14 requires App Sandbox (row 19) plus
   dropping the `com.apple.security.cs.allow-jit` entitlement on
   JS engines that use W^X. Should v0.1.0 target Mac App Store,
   or skip it?

7. **System tray (row 69).** macOS has a menu-bar item
   (NSStatusItem), not a system tray the way Windows does. Should
   the row split into "Windows tray icon" and "macOS menu-bar
   item"? Currently conflated.

8. **Differential update in v0.1.0.** Row 35 scores as >=90%
   because "industry ships differential". For a small Spiral
   binary in v0.1.0, full-payload update may be acceptable until
   the binary grows. Is differential update *necessary* in v0.1.0?

---

## Sources

| Tier | Source | URL | Used for |
|------|--------|-----|----------|
| 1 | WHATWG HTML — custom handlers | https://html.spec.whatwg.org/multipage/system-state.html#custom-handlers | Protocol handler registration (rows 54–57) |
| 1 | WHATWG Notifications API | https://notifications.spec.whatwg.org/ | Notifications (row 62) |
| 1 | W3C Web Share API | https://w3c.github.io/web-share/ | OS share sheet (row 60) |
| 1 | W3C Clipboard API | https://w3c.github.io/clipboard-apis/ | Clipboard (row 83) |
| 1 | W3C Media Session API | https://w3c.github.io/mediasession/ | Media keys (row 81) |
| 1 | W3C UI Events | https://w3c.github.io/uievents/ | Input (row 79) |
| 1 | W3C ARIA 1.2 | https://www.w3.org/TR/wai-aria-1.2/ | Accessibility (row 78) |
| 1 | WICG Web Speech API | https://wicg.github.io/speech-api/ | Voice (row 85) |
| 1 | Linux man-pages — Landlock | https://docs.kernel.org/userspace-api/landlock.html | Sandbox (row 91) |
| 1 | Linux man-pages — seccomp | https://www.kernel.org/doc/Documentation/prctl/seccomp_filter.txt | Sandbox (row 92) |
| 1 | Linux man-pages — namespaces | https://man7.org/linux/man-pages/man7/namespaces.7.html | Sandbox (row 93) |
| 1 | Linux man-pages — no-new-privileges | https://www.kernel.org/doc/Documentation/prctl/no_new_privs.txt | Hardening (row 106) |
| 1 | Apple — App Sandbox | https://developer.apple.com/documentation/security/app_sandbox | App Sandbox (rows 19, 96) |
| 1 | Apple — Hardened Runtime | https://developer.apple.com/documentation/security/hardened_runtime | Entitlements (row 17) |
| 1 | Apple — Notarisation | https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution | Notarisation (row 15) |
| 1 | Apple — Gatekeeper | https://developer.apple.com/documentation/security/hardened_runtime | Gatekeeper (row 16) |
| 1 | Apple — Seatbelt sandbox | https://developer.apple.com/documentation/security/sandbox | macOS sandbox (row 18, 95) |
| 1 | Apple — NSDockTile | https://developer.apple.com/documentation/appkit/nsdocktile | Dock badge (row 64) |
| 1 | Apple — NSVisualEffectView | https://developer.apple.com/documentation/appkit/nsvisualeffectview | Appearance (row 74) |
| 1 | Apple — App Store Review | https://developer.apple.com/app-store/review/guidelines/ | iOS engine swap (row 33) |
| 1 | Apple — Launch Services | https://developer.apple.com/documentation/coreservices/launch_services | File association (row 58) |
| 1 | Apple — Service Management | https://developer.apple.com/documentation/servicemanagement | Auto-launch (row 70) |
| 1 | Microsoft — MSIX | https://learn.microsoft.com/en-us/windows/msix/ | MSIX (row 22) |
| 1 | Microsoft — Windows Installer | https://learn.microsoft.com/en-us/windows/win32/msi/windows-installer-portal | MSI (row 21) |
| 1 | Microsoft — Authenticode | https://learn.microsoft.com/en-us/windows-hardware/drivers/install/authenticode | Code signing (row 27) |
| 1 | Microsoft — SmartScreen | https://learn.microsoft.com/en-us/deployedge/microsoft-edge-security-smartscreen | SmartScreen (row 28) |
| 1 | Microsoft — ITaskbarList3 | https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3 | Taskbar progress (row 65) |
| 1 | Microsoft — Restricted Tokens | https://learn.microsoft.com/en-us/windows/win32/secauthz/restricted-tokens | Sandbox (row 97) |
| 1 | Microsoft — winget | https://learn.microsoft.com/en-us/windows/package-manager/ | winget (row 24) |
| 1 | Microsoft — Acrylic | https://learn.microsoft.com/en-us/windows/apps/design/style/acrylic | Appearance (row 74) |
| 1 | Microsoft — AppContainer | https://learn.microsoft.com/en-us/windows/security/identity-protection/access-control/access-tokens | AppContainer (row 98) |
| 1 | Freedesktop — Flatpak | https://docs.flatpak.org/ | Flatpak (rows 4, 10) |
| 1 | Freedesktop — Flatpak portals | https://docs.flatpak.org/en/latest/portal-api.html | Portals (row 10) |
| 1 | Debian Policy | https://www.debian.org/doc/debian-policy/ | .deb (row 1) |
| 1 | RPM | https://rpm.org/ | .rpm (row 2) |
| 1 | Arch Wiki — AUR | https://wiki.archlinux.org/title/AUR | AUR (row 6) |
| 1 | Gentoo Devmanual | https://devmanual.gentoo.org/ | Ebuild (row 8) |
| 1 | Nixpkgs Manual | https://nixos.org/manual/nixpkgs/stable/ | Nix (row 7) |
| 1 | Snap Documentation | https://snapcraft.io/docs | Snap (row 5) |
| 1 | AppImage | https://appimage.org/ | AppImage (row 3) |
| 1 | Android — App Bundle | https://developer.android.com/guide/app-bundle | AAB (row 34) |
| 1 | Android — APK | https://developer.android.com/guide/topics/app-package | APK (row 29) |
| 1 | F-Droid Docs | https://f-droid.org/docs/ | F-Droid (row 31) |
| 1 | Amazon Appstore | https://developer.amazon.com/docs/app-submission/why-amazon-appstore.html | Amazon (row 32) |
| 1 | NSIS | https://nsis.sourceforge.io/ | Installer (row 21) |
| 1 | Chromium Sandbox design doc | https://chromium.googlesource.com/chromium/src/+/main/docs/security/sandbox.md | Sandbox (rows 87–106) |
| 1 | Chromium Site Isolation | https://chromium.googlesource.com/chromium/src/+/main/docs/security/site-isolation.md | Process isolation (rows 87–88) |
| 1 | Mozilla Fission | https://wiki.mozilla.org/Project_Fission | Process isolation (row 87) |
| 1 | Mozilla Electrolysis | https://wiki.mozilla.org/Electrolysis | Per-tab (row 89) |
| 1 | Mozilla MAR format | https://wiki.mozilla.org/MAR | Update signing (row 40) |
| 1 | Chromium Courgette | https://chromium.googlesource.com/chromium/src/+/main/components/courgette/ | Differential update (row 35) |
| 1 | Chromium Omaha protocol | https://chromium.googlesource.com/chromium/src/+/main/docs/updater/protocol.html | Staged rollout (row 37) |
| 2 | MDN — registerProtocolHandler | https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler | Protocol handlers (rows 54–57) |
| 2 | MDN — Web Share | https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share | Share (row 60) |
| 2 | MDN — Notifications API | https://developer.mozilla.org/en-US/docs/Web/API/Notifications_API | Notifications (row 62) |
| 2 | MDN — Clipboard API | https://developer.mozilla.org/en-US/docs/Web/API/Clipboard | Clipboard (row 83) |
| 2 | MDN — Media Session | https://developer.mozilla.org/en-US/docs/Web/API/MediaSession | Media keys (row 81) |
| 2 | MDN — Native Messaging | https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests | Native messaging (row 59) |
| 2 | MDN — Speech Recognition | https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition | Voice (row 85) |
| 2 | Homebrew Cask | https://github.com/Homebrew/homebrew-cask | Homebrew (row 13) |
| 2 | Sparkle project | https://sparkle-project.org/documentation/ | Sparkle (row 20) |
| 2 | Chocolatey | https://docs.chocolatey.org/en-us/ | Chocolatey (row 25) |
| 2 | Scoop | https://scoop.sh/ | Scoop (row 26) |
| 2 | AppArmor wiki | https://gitlab.com/apparmor/apparmor/-/wikis/Documentation | AppArmor (row 94) |
| 3 | Chromium updater docs | https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/update/ | Update (row 36) |
| 3 | Mozilla Software Update | https://wiki.mozilla.org/Software_Update | Update (rows 35–42) |
| 3 | Mozilla Release Management | https://wiki.mozilla.org/Release_Management/Calendar | Channels (row 38) |
| 3 | Chromium calendar | https://www.chromium.org/developers/calendar | Channels (row 38) |
| 3 | Chromium profiles | https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/profiles/ | Profiles (row 75) |
| 3 | Chromium GPU | https://chromium.googlesource.com/chromium/src/+/main/gpu/ | GPU process (row 100) |
| 3 | Chromium Network Service | https://chromium.googlesource.com/chromium/src/+/main/services/network/ | Network process (row 101) |
| 3 | Chromium Storage Service | https://chromium.googlesource.com/chromium/src/+/main/services/storage/ | Storage process (row 102) |
| 3 | Chromium memory docs | https://chromium.googlesource.com/chromium/src/+/main/docs/memory/ | Memory pressure (row 104) |
| 3 | WebKit2 blog post | https://webkit.org/blog/6161/little-bug-freddie-lets-talk-about-webkit2/ | Process model (row 89) |
| 3 | Chromium mac sandbox | https://chromium.googlesource.com/chromium/src/+/main/sandbox/mac/ | Seatbelt (row 95) |
| 3 | Chromium win sandbox | https://chromium.googlesource.com/chromium/src/+/main/sandbox/win/ | Windows sandbox (row 97) |
| 3 | Chromium linux sandbox | https://chromium.googlesource.com/chromium/src/+/main/sandbox/linux/ | seccomp (row 92) |
| 3 | Mozilla Security Sandbox | https://wiki.mozilla.org/Security/Sandbox | Sandbox (row 99) |
| 3 | Apple BHDC Seatbelt paper | https://reverse.put.as/wp-content/uploads/2011/09/Apple-Sandbox-BHDC2011-Paper.pdf | Seatbelt (row 95) |
| 3 | Servo releases | https://github.com/servo/servo/blob/main/RELEASES.md | Servo coverage |
| 3 | Ladybird | https://ladybird.org/ | Ladybird coverage |

Per-row URLs are inlined in the table cells; this table is the master index.
