# Chunk 12 — Competitive Matrix: Security & Privacy

**File:** `02-competitive-matrix-security.md`
**Date:** 2026-06-16
**Source:** `04-privacy-security-standards.md`
**Row count:** 128

---

## 3.1 Same-Origin Policy (SOP)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 1 | Same-origin definition — scheme + host + port triple | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 2 | Effective TLD +1 (eTLD+1) — registrable domain computation for same-site determination | all | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 3 | document.domain setter — relax origin to eTLD+1 (deprecated, disableable) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 4 | Sandbox origin — opaque origin for sandboxed iframes without allow-same-origin | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 5 | blob: URL origin — inherited from creating context | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 6 | Origin check in DOM access (cross-origin contentWindow restricted) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |

---

## 3.2 Content Security Policy (CSP)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 7 | CSP Level 1 — default-src, script-src, style-src, img-src, connect-src, font-src, object-src, media-src, frame-src, sandbox, report-uri | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes |
| 8 | CSP Level 2 — frame-ancestors, base-uri, form-action, plugin-types, reflected-xss, referrer, hash source expressions | all | not-started | ubiquitous | P3 | L | yes | yes | yes | yes | yes |
| 9 | CSP Level 3 — strict-dynamic, nonce-based script allowlisting, unsafe-hashes, navigate-to, worker-src, manifest-src | all | not-started | widespread | P3 | L | yes | yes | yes | no | no |
| 10 | CSP require-trusted-types-for 'script' — enforce Trusted Types for DOM XSS sinks | all | not-started | mixed | P3 | L | yes | yes | experimental | no | no |
| 11 | Trusted Types — TrustedHTML, TrustedScript, TrustedScriptURL, TrustedURL type wrappers for DOM sinks | all | not-started | mixed | P3 | L | yes | yes | experimental | no | no |
| 12 | CSP violation reporting — report-uri (deprecated) / report-to + Reporting API | all | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 13 | CSP source expressions: unsafe-inline, unsafe-eval, wasm-unsafe-eval, unsafe-allow-redirects | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 14 | Multiple CSP headers — intersection enforcement (most restrictive wins) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |

---

## 3.3 Cross-Origin Resource Sharing (CORS)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 15 | CORS simple requests — GET/HEAD/POST with safelisted headers and content types | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 16 | CORS preflight — OPTIONS request for non-simple methods/headers | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 17 | CORS credentials mode — Access-Control-Allow-Credentials, include mode, cookie attachment | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 18 | Access-Control-Allow-Origin: * — wildcard (no credentials) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 19 | Access-Control-Expose-Headers — whitelist response headers for cross-origin reads | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 20 | Access-Control-Max-Age — preflight cache duration | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 21 | CORS-preflight cache — per-method, per-header cache with max-age | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |

---

## 3.4 Cross-Origin Resource Policy (CORP)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 22 | CORP same-origin — reject no-cors cross-origin loads | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 23 | CORP same-site — reject no-cors cross-site loads | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 24 | CORP cross-origin — explicitly opt-in to cross-origin no-cors loads | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |

---

## 3.5 Cross-Origin Opener Policy (COOP)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 25 | COOP same-origin — isolate browsing context group; no cross-origin references via window.open | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 26 | COOP same-origin-allow-popups — retain references to same-origin popups only | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 27 | COOP unsafe-none — default; no isolation | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 28 | COOP reporting — violation reports via Reporting API | all | not-started | mixed | P3 | S | yes | yes | partial | no | no |

---

## 3.6 Cross-Origin Embedder Policy (COEP)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 29 | COEP require-corp — require CORP or CORS on all subresources | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 30 | COEP credentialless — send no credentials cross-origin (no CORP needed) | all | not-started | mixed | P3 | M | yes | yes | no | no | no |
| 31 | COEP unsafe-none — default; no restriction | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 32 | COEP + COOP pairing for SharedArrayBuffer / high-resolution timer access | all | not-started | widespread | P3 | L | yes | yes | yes | no | no |

---

## 3.7 HTTP Strict Transport Security (HSTS)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 33 | HSTS header — Strict-Transport-Security: max-age=...; includeSubDomains; preload | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 34 | HSTS preload list — hardcoded HTTPS-only list for major domains | all | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 35 | HSTS priming — preflight HSTS check via /.well-known/http-opportunistic | all | not-started | niche | P3 | S | no | experimental | no | no | no |

---

## 3.8 Certificate Transparency (CT) & Revocation

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 36 | CT SCT validation — verify Signed Certificate Timestamps from CT logs | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 37 | CT policy for CAs — minimum number of SCTs, diversity of logs | all | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 38 | OCSP stapling — server-provided revocation proof (policy enforcement) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 39 | OCSP must-staple — TLS feature extension requiring staple | all | not-started | mixed | P3 | S | no | yes | no | no | no |
| 40 | CRLite — compressed CRL set for client-side revocation (Firefox) | desktop | not-started | niche | P3 | M | no | yes | no | no | no |
| 41 | CRLSets — server-pushed compressed CRL (Chromium) | desktop | not-started | niche | P3 | M | yes | no | no | no | no |

---

## 3.9 Permissions Policy & Feature Policy

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 42 | Permissions Policy header | all | not-started | widespread | P3 | L | yes | yes | yes | no | no |
| 43 | Permissions Policy in iframe allow attribute — delegate feature access to embedded content | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 44 | Feature Policy (deprecated predecessor) | all | not-started | widespread | P3 | M | yes | no | partial | no | no |
| 45 | Default allowlists — self, src (for iframes), * per feature | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |

---

## 3.10 Sandboxing

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 46 | iframe sandbox — restrict iframe capabilities (scripts, forms, popups, same-origin, etc.) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 47 | CSP sandbox directive — apply sandbox flags to entire page via header | all | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 48 | Sandbox flags: allow-same-origin, allow-scripts, allow-forms, allow-popups, allow-modals, allow-popups-to-escape-sandbox, allow-top-navigation, allow-top-navigation-by-user-activation, allow-downloads, allow-presentation, allow-orientation-lock, allow-pointer-lock | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 49 | Sandboxed origin unique generation — each sandboxed iframe gets a unique opaque origin | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |

---

## 3.11 Mixed Content

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 50 | Block active mixed content — block HTTP scripts, stylesheets, iframes on HTTPS pages | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 51 | Upgrade passive mixed content — auto-upgrade images, audio, video from HTTP to HTTPS | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 52 | block-all-mixed-content (deprecated CSP directive) — block all mixed HTTP subresources | all | not-started | deprecated | P3 | S | no | no | no | no | no |
| 53 | Upgrade-Insecure-Requests: 1 header — auto-upgrade HTTP navigations and subresources | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |

---

## 3.12 Subresource Integrity (SRI) — Policy Dimension

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 54 | SRI on script — verify hash of fetched script matches integrity attribute | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 55 | SRI on link rel=stylesheet — verify hash of fetched CSS | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 56 | SRI on link rel=preload / rel=modulepreload — integrity on preloaded resources | all | not-started | mixed | P3 | S | yes | yes | partial | no | no |
| 57 | SRI hash algorithms — SHA-256, SHA-384, SHA-512 | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 58 | SRI fallback — allow multiple hashes, algorithm agility | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |

---

## 3.13 Referrer Policy

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 59 | Referrer-Policy header — control referrer information sent with requests | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 60 | Referrer Policy in meta name=referrer | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 61 | Referrer Policy in referrerpolicy attribute on a, area, img, script, link, iframe | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 62 | strict-origin-when-cross-origin — default in modern browsers; send origin only cross-origin, full URL same-origin | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 63 | no-referrer — send no referrer header at all | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 64 | same-origin — send referrer only for same-origin requests | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |

---

## 3.14 Cookie Security (Policy Dimension)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 65 | Secure flag — cookie only sent over HTTPS | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 66 | HttpOnly flag — cookie inaccessible to JavaScript (document.cookie) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 67 | SameSite=Lax / SameSite=Strict / SameSite=None — cross-site sending policy | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 68 | __Secure- prefix — enforce Secure flag on prefixed cookies | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 69 | __Host- prefix — enforce Secure + no Domain + Path=/ | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 70 | CHIPS — Partitioned attribute for cookie partitioning (top-site-keyed) | all | not-started | mixed | P18+ | M | yes | yes | experimental | no | no |
| 71 | Third-party storage partitioning — per-top-site cache, storage, cookie jar partitioning | all | not-started | widespread | P18+ | L | yes | yes | yes | no | no |

---

## 3.15 Storage Access API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 72 | document.requestStorageAccess() — request unpartitioned cookie/storage access in third-party context | all | not-started | widespread | P18+ | L | yes | yes | yes | no | no |
| 73 | document.hasStorageAccess() — check if storage access is already granted | all | not-started | widespread | P18+ | M | yes | yes | yes | no | no |
| 74 | document.getStorageAccessHandle() — Storage Access API Level 2 (extended partitioned state access) | all | not-started | experimental | P18+ | L | experimental | experimental | no | no | no |
| 75 | Storage-Access-Activate header — HTTP-based storage access activation | all | not-started | experimental | P18+ | M | experimental | no | no | no | no |
| 76 | Sec-Fetch-Storage-Access header — indicate storage access mode in fetch metadata | all | not-started | experimental | P18+ | S | experimental | experimental | no | no | no |

---

## 3.16 Private State Tokens (formerly Trust Tokens)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 77 | Private State Token issuance — origin-bound cryptographic token for anti-fraud attestation | all | not-started | experimental | P22+ | L | yes | experimental | no | no | no |
| 78 | Private State Token redemption — present token to different origin for verification | all | not-started | experimental | P22+ | L | yes | experimental | no | no | no |
| 79 | Sec-Private-State-Token-Crypto-Version header — negotiate crypto version | all | not-started | experimental | P22+ | S | yes | no | no | no | no |

---

## 3.17 Federated Credential Management (FedCM)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 80 | FedCM navigator.credentials.get({identity: ...}) — mediated identity provider sign-in | all | not-started | mixed | P22+ | L | yes | experimental | no | no | no |
| 81 | FedCM IdP manifest (/.well-known/web-identity) — identity provider configuration endpoint | all | not-started | mixed | P22+ | M | yes | experimental | no | no | no |
| 82 | FedCM account list endpoint — display user's accounts for selection | all | not-started | mixed | P22+ | M | yes | experimental | no | no | no |

---

## 3.18 Popups & Window Restrictions

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 83 | rel=noopener — prevent opener reference on a target=_blank | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 84 | rel=noreferrer — suppress referrer and opener reference | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 85 | Noopener-by-default for target=_blank — implicit noopener without explicit rel | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 86 | Popup blocker — block window.open() not triggered by user gesture | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 87 | Pop-under prevention — block windows opened behind current window | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 88 | window.focus() / window.blur() restrictions — limit programmatic focus changes | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 89 | Cross-origin window property restrictions — window.length, window.closed, window.location getter restrictions | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |

---

## 3.19 Content Type Sniffing

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 90 | X-Content-Type-Options: nosniff — prevent MIME type sniffing for script and stylesheet | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 91 | MIME type sniffing algorithm — WHATWG MIME Sniff spec (safelisted, sniffable, explicitly nosniff) | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 92 | nosniff enforcement on script and link rel=stylesheet — block if type doesn't match | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |

---

## 3.20 XSS Protection & Trusted Types

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 93 | X-XSS-Protection: 0 header — disable legacy XSS auditor | all | not-started | deprecated | P3 | S | no | no | no | no | no |
| 94 | Trusted Types enforcement — block DOM XSS sinks without typed values | all | not-started | mixed | P3 | L | yes | yes | experimental | no | no |
| 95 | Content-Type-Options: nosniff + CSP script-src — combined hardening against MIME confusion attacks | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |

---

## 3.21 Permission Delegation & Device Access

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 96 | Permissions API — navigator.permissions.query() for standardised permission status | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |
| 97 | iframe allow permission delegation — camera, microphone, geolocation delegated to iframe | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 98 | Permissions Policy delegation inheritance — child frame inherits parent's policy by default | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |

---

## 3.22 Download Restrictions

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 99 | Content-Disposition: attachment — force download instead of inline display | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 100 | a download attribute — trigger download with suggested filename | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 101 | Cross-origin download restrictions — block a download on cross-origin URLs | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 102 | Mixed-content download blocking — warn/block on HTTP downloads from HTTPS pages | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 103 | Dangerous file type warnings — warn on .exe, .dmg, .apk downloads from untrusted sources | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 104 | Content-Disposition: inline + sandbox without allow-downloads — block downloads from sandboxed context | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |

---

## 3.23 Site Isolation / Process Isolation

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 105 | Out-of-process iframes — cross-origin iframes in separate renderer processes | all | not-started | widespread | P3 | XL | yes | yes | yes | no | no |
| 106 | Process-per-site / process-per-origin — group same-site tabs into one process | all | not-started | widespread | P3 | XL | yes | yes | yes | no | no |
| 107 | Site Isolation for Spectre — isolate cross-origin data in separate address spaces | all | not-started | widespread | P3 | XL | yes | yes | yes | no | no |

---

## 3.24 HTTP/0.9 & Downgrade Blocking

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 108 | HTTP/0.9 blocking — block HTTP/0.9 responses on non-default ports | all | not-started | widespread | P3 | S | yes | yes | yes | no | no |
| 109 | HTTP downgrade prevention — block insecure redirects from HTTPS to HTTP | all | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | yes |

---

## 3.25 Clear-Site-Data

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 110 | Clear-Site-Data: "cache", "cookies", "storage", "*" — clear browsing data via HTTP header | all | not-started | mixed | P3 | M | yes | yes | no | no | no |

---

## 3.26 Reporting API & Network Error Logging (NEL)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 111 | Reporting API — Report-To / Reporting-Endpoints header, violation report delivery | all | not-started | widespread | P3 | M | yes | yes | yes | no | no |
| 112 | NEL (Network Error Logging) — NEL header, collect network failure telemetry from client | all | not-started | mixed | P3 | M | yes | experimental | no | no | no |
| 113 | Deprecation reports — Report type deprecation for API deprecation warnings | all | not-started | mixed | P3 | S | yes | yes | yes | no | no |
| 114 | Intervention reports — Report type intervention for browser intervention notifications | all | not-started | mixed | P3 | S | yes | no | no | no | no |

---

## 3.27 Fingerprinting Resistance

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 115 | Canvas fingerprinting noise — inject random noise into canvas.toDataURL() and toBlob() | desktop+mobile | not-started | mixed | P22+ | M | no | yes | partial | no | no |
| 116 | WebGL parameter hash rounding — reduce precision of WebGL vendor/renderer strings | desktop+mobile | not-started | mixed | P22+ | M | no | yes | partial | no | no |
| 117 | AudioContext noise — add noise to AudioContext output for fingerprinting resistance | desktop+mobile | not-started | mixed | P22+ | M | no | yes | no | no | no |
| 118 | navigator.connection throttling — reduce precision of Network Information API | desktop+mobile | not-started | mixed | P22+ | S | no | yes | no | no | no |
| 119 | navigator.hardwareConcurrency clamping — limit reported CPU core count | desktop+mobile | not-started | mixed | P22+ | S | no | yes | no | no | no |
| 120 | navigator.plugins / navigator.mimeTypes — empty or spoofed plugin list | all | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | yes |
| 121 | User-Agent Client Hints (Sec-CH-UA*) — reduce UA string entropy | all | not-started | widespread | P3 | M | yes | experimental | no | no | no |
| 122 | navigator.globalPrivacyControl (GPC) — signal user's opt-out of cross-site tracking | all | not-started | mixed | P22+ | S | no | yes | no | no | no |

---

## 3.28 Private Browsing / Incognito Mode

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 123 | Private browsing — ephemeral session that does not persist cookies, storage, history, cache | all | not-started | ubiquitous | P12+ | L | yes | yes | yes | yes | yes |
| 124 | Incognito/private mode storage detection — navigator.storage.estimate() returns special values | all | not-started | widespread | P12+ | M | yes | yes | yes | no | no |
| 125 | Extension access restrictions in private browsing — extensions disabled by default in private mode | all | not-started | ubiquitous | P12+ | M | yes | yes | yes | yes | yes |
| 126 | IndexedDB/private mode ephemeral behaviour — in-memory only, wiped on session end | all | not-started | ubiquitous | P12+ | M | yes | yes | yes | yes | yes |

---

## 3.29 Viewport & Accessibility Security

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 127 | meta viewport user-scalable=no — prevent user zoom (accessibility concern, browsers may override) | mobile | not-started | widespread | P3 | S | yes | yes | yes | no | no |
| 128 | meta viewport maximum-scale — limit zoom scale (accessibility concern) | mobile | not-started | widespread | P3 | S | yes | yes | yes | no | no |

---

## Open questions for the user

- **CSP enforcement priority:** CSP is large (Levels 1–3 + Trusted Types). Should Spiral aim for CSP Level 2 compliance first (ubiquitous, simpler) and defer strict-dynamic / Trusted Types to later phases?
- **Site isolation model:** Bet 1 (SEM) designs capability-typed security as the default, with OS-level process isolation as escalation. Should Spiral aim for process-per-origin from day one, or trust the type system for default security?
- **Fingerprinting resistance depth:** Firefox RFP is comprehensive but breaks many sites. Brave's farble approach is more compatible. Should Spiral adopt a permissive or strict fingerprinting resistance posture?
- **FedCM / Private State Tokens:** These are Chromium-led proposals with limited multi-engine adoption. Should Spiral implement them, wait for multi-engine consensus, or skip entirely?
- **Storage Access API vs CHIPS:** Both address third-party storage partitioning. Should Spiral implement both, or prioritise CHIPS?
- **Reporting API scope:** Should Spiral implement the full Reporting API in Phase 3, or defer NEL and intervention reports to Phase 4+?
- **Clear-Site-Data support:** Only Chromium and Gecko implement this. Is it a Phase 3 priority, or can it wait?
- **Private browsing scope:** Should private browsing be a Phase 3 feature (early, high user impact) or Phase 12+ (when storage infrastructure is mature)?
