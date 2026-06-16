# Chunk 3 — Security & Privacy Standards

> **Companion to the research index in `README.md`.** This file covers
> the browser's enforcement of security policy on web content and the
> privacy mechanisms that protect users. Transport layer (TLS, HTTP
> caching) is chunk 2; storage mechanics (cookie jar, localStorage,
> IndexedDB) is chunk 4.
>
> **Dividing line:** this chunk is about *how the browser enforces
> security policy on web content and protects user privacy*, not about
> the transport carrying it or the storage holding it.

---

## Scope

**In:** Content Security Policy (Levels 1–3, Trusted Types), Same-Origin
Policy (origin definition, `document.domain`, sandboxed origins), CORS,
CORP, COOP, COEP, Mixed Content, HSTS, Certificate Transparency, OCSP
stapling/CRLite/CRLSets (policy dimension), Permissions Policy, Storage
Access API, Private State Tokens, FedCM, popup restrictions, sandboxing
(`<iframe sandbox>`, CSP sandbox), Subresource Integrity (security policy
dimension), Referrer Policy, cookie security flags, fingerprinting
resistance, private browsing, content type sniffing, XSS auditor
(deprecated), permission delegation, download restrictions, site
isolation, HTTP/0.9 blocking, Clear-Site-Data, NEL/Reporting API.

**Out:** Network transport (HTTP caching, TLS handshake, DNS) — chunk 2.
Storage mechanics (cookie jar, localStorage, IndexedDB internals) —
chunk 4. Media EME — chunk 5. Platform APIs (WebAuthn, Payment Request)
— chunk 6. Developer tools — chunk 8. Extension permissions — chunk 10.
Distribution & platform sandboxing (seccomp, Seatbelt) — chunk 11.

---

## Methodology for this chunk

Rows derived from: WHATWG Fetch/HTML/Storage specs (Tier 1), W3C CSP3,
CREDENTIAL-MANAGEMENT, Permissions Policy, SRI, Mixed Content
specifications (Tier 1), IETF RFCs for HSTS (6797), CT (6962),
OCSP (6960) (Tier 1), MDN browser-compat-data (Tier 2), Can I Use
(Tier 2), Chrome Platform Status (Tier 3), privacytests.org (Tier 4),
EFF Cover Your Tracks (Tier 4), academic papers on SOP/CORS/SRI (Tier 4).
Engine notes are one-line per engine; full engine-detail deferred to
chunk 7.

---

## 3.1 Same-Origin Policy (SOP)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 1 | Same-origin definition — scheme + host + port triple | all | not-started | ubiquitous | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/browsers.html#origin ; https://developer.mozilla.org/en-US/docs/Web/Security/Same-origin_policy |
| 2 | Effective TLD +1 (eTLD+1) — registrable domain computation for same-site determination | all | not-started | ubiquitous | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://publicsuffix.org/ ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#third-party_cookies |
| 3 | `document.domain` setter — relax origin to eTLD+1 (deprecated, disableable) | all | not-started | ubiquitous | P3 | M | all yes/stable (document.domain setter deprecated; behind flag in some contexts) | https://html.spec.whatwg.org/multipage/origin.html#relaxing-the-same-origin-restriction |
| 4 | Sandbox origin — opaque origin for sandboxed iframes without `allow-same-origin` | all | not-started | ubiquitous | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/browsers.html#sandboxed-origin |
| 5 | `blob:` URL origin — inherited from creating context | all | not-started | ubiquitous | P3 | S | all yes/stable | https://w3c.github.io/FileAPI/#url |
| 6 | Origin check in DOM access (cross-origin `contentWindow` restricted) | all | not-started | ubiquitous | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-access |

---

## 3.2 Content Security Policy (CSP)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 7 | CSP Level 1 — `Content-Security-Policy` header, `default-src`, `script-src`, `style-src`, `img-src`, `connect-src`, `font-src`, `object-src`, `media-src`, `frame-src`, `sandbox`, `report-uri` | all | not-started | ubiquitous | P3 | L | all yes/stable | https://www.w3.org/TR/CSP1/ ; https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP |
| 8 | CSP Level 2 — `frame-ancestors`, `base-uri`, `form-action`, `plugin-types`, `reflected-xss`, `referrer`, `hash` source expressions | all | not-started | ubiquitous | P3 | L | all yes/stable | https://www.w3.org/TR/CSP2/ |
| 9 | CSP Level 3 — `strict-dynamic`, `'strict-dynamic'` source expression, `nonce`-based script allowlisting, `unsafe-hashes`, `'report-sample'`, `navigate-to`, `worker-src`, `manifest-src`, `prefetch-src` (deprecated in favour of `default-src`) | all | not-started | widespread | P3 | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/CSP3/ |
| 10 | CSP `require-trusted-types-for 'script'` — enforce Trusted Types for DOM XSS sinks | all | not-started | mixed | P3 | L | Chromium yes/stable; Gecko yes/stable; WebKit experimental; Servo no; Ladybird no; Flow no | https://w3c.github.io/trusted-types/dist/spec/ ; https://chromestatus.com/feature/5765498209881088 |
| 11 | Trusted Types — `TrustedHTML`, `TrustedScript`, `TrustedScriptURL`, `TrustedURL` type wrappers for DOM sinks | all | not-started | mixed | P3 | L | Chromium yes/stable; Gecko yes/stable; WebKit experimental; Servo no; Ladybird no; Flow no | https://w3c.github.io/trusted-types/dist/spec/ |
| 12 | CSP violation reporting — `report-uri` (deprecated) / `report-to` + Reporting API | all | not-started | ubiquitous | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/CSP3/#violation-reports ; https://www.w3.org/TR/reporting-1/ |
| 13 | CSP `source` expression: `'unsafe-inline'`, `'unsafe-eval'`, `'wasm-unsafe-eval'`, `'unsafe-allow-redirects'` | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/CSP3/#framework-directive-source-list |
| 14 | Multiple CSP headers — intersection enforcement (most restrictive wins) | all | not-started | ubiquitous | P3 | M | all yes/stable | https://www.w3.org/TR/CSP3/#multiple-policies |

---

## 3.3 Cross-Origin Resource Sharing (CORS)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 15 | CORS simple requests — GET/HEAD/POST with safelisted headers and content types | all | not-started | ubiquitous | P3 | M | all yes/stable | https://fetch.spec.whatwg.org/#cors-request |
| 16 | CORS preflight — OPTIONS request for non-simple methods/headers | all | not-started | ubiquitous | P3 | M | all yes/stable | https://fetch.spec.whatwg.org/#cors-preflight-request |
| 17 | CORS credentials mode — `Access-Control-Allow-Credentials`, `include` mode, cookie attachment | all | not-started | ubiquitous | P3 | M | all yes/stable | https://fetch.spec.whatwg.org/#cors-protocol |
| 18 | `Access-Control-Allow-Origin: *` — wildcard (no credentials) | all | not-started | ubiquitous | P3 | S | all yes/stable | https://fetch.spec.whatwg.org/#http-access-control-allow-origin |
| 19 | `Access-Control-Expose-Headers` — whitelist response headers for cross-origin reads | all | not-started | ubiquitous | P3 | S | all yes/stable | https://fetch.spec.whatwg.org/#http-access-control-expose-headers |
| 20 | `Access-Control-Max-Age` — preflight cache duration | all | not-started | ubiquitous | P3 | S | all yes/stable (capped per engine: Chromium 2h, Gecko 24h, WebKit 10min) | https://fetch.spec.whatwg.org/#http-access-control-max-age |
| 21 | CORS-preflight cache — per-method, per-header cache with max-age | all | not-started | ubiquitous | P3 | M | all yes/stable | https://fetch.spec.whatwg.org/#cors-preflight-cache |

---

## 3.4 Cross-Origin Resource Policy (CORP)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 22 | CORP `same-origin` — reject no-cors cross-origin loads | all | not-started | ubiquitous | P3 | M | all yes/stable | https://fetch.spec.whatwg.org/#cross-origin-resource-policy-header |
| 23 | CORP `same-site` — reject no-cors cross-site loads | all | not-started | ubiquitous | P3 | S | all yes/stable | https://fetch.spec.whatwg.org/#cross-origin-resource-policy-header |
| 24 | CORP `cross-origin` — explicitly opt-in to cross-origin no-cors loads | all | not-started | ubiquitous | P3 | S | all yes/stable | https://fetch.spec.whatwg.org/#cross-origin-resource-policy-header |

---

## 3.5 Cross-Origin Opener Policy (COOP)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 25 | COOP `same-origin` — isolate browsing context group; no cross-origin references via `window.open` | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-opener-policies |
| 26 | COOP `same-origin-allow-popups` — retain references to same-origin popups only | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-opener-policies |
| 27 | COOP `unsafe-none` — default; no isolation | all | not-started | ubiquitous | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-opener-policies |
| 28 | COOP reporting — violation reports via Reporting API | all | not-started | mixed | P3 | S | Chromium yes/stable; Gecko yes/stable; WebKit partial; Servo no; Ladybird no; Flow no | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-opener-policies |

---

## 3.6 Cross-Origin Embedder Policy (COEP)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 29 | COEP `require-corp` — require CORP or CORS on all subresources | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-embedder-policy |
| 30 | COEP `credentialless` — send no credentials cross-origin (no CORP needed) | all | not-started | mixed | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-embedder-policy |
| 31 | COEP `unsafe-none` — default; no restriction | all | not-started | ubiquitous | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-embedder-policy |
| 32 | COEP + COOP pairing for `SharedArrayBuffer` / high-resolution timer access | all | not-started | widespread | P3 | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://web.dev/articles/why-coop-coep ; https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer |

---

## 3.7 HTTP Strict Transport Security (HSTS)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 33 | HSTS header — `Strict-Transport-Security: max-age=...; includeSubDomains; preload` | all | not-started | ubiquitous | P3 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc6797 |
| 34 | HSTS preload list — hardcoded HTTPS-only list for major domains | all | not-started | ubiquitous | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://hstspreload.org/ ; https://source.chromium.org/chromium/chromium/src/+/main:net/http/transport_security_state_static.json |
| 35 | HSTS priming — preflight HSTS check via `/.well-known/http-opportunistic` | all | not-started | niche | P3 | S | Gecko experimental; others no | https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-hsts-prime/ |

---

## 3.8 Certificate Transparency (CT) & Revocation

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 36 | CT SCT validation — verify Signed Certificate Timestamps from CT logs | all | not-started | ubiquitous | P3 | M | all yes/stable (Chromium mandated since 2018; Gecko/WebKit validate SCTs) | https://www.rfc-editor.org/rfc/rfc6962 ; https://developer.mozilla.org/en-US/docs/Web/Security/Certificate_Transparency |
| 37 | CT policy for CAs — minimum number of SCTs, diversity of logs | all | not-started | ubiquitous | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://googlechrome.github.io/CertificateTransparency/ct_policy.html |
| 38 | OCSP stapling — server-provided revocation proof (policy enforcement) | all | not-started | ubiquitous | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc6066#section-8 |
| 39 | OCSP must-staple — TLS feature extension requiring staple | all | not-started | mixed | P3 | S | Chromium no; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc7633 |
| 40 | CRLite — compressed CRL set for client-side revocation (Firefox) | desktop | not-started | niche | P3 | M | Gecko yes/stable (Firefox 86+); others no | https://blog.mozilla.org/security/2020/08/25/crlite-compressed-revocation/ |
| 41 | CRLSets — server-pushed compressed CRL (Chromium) | desktop | not-started | niche | P3 | M | Chromium yes/stable; others no | https://chromium.googlesource.com/chromium/src/+/main/net/cert/crl_set.md |

---

## 3.9 Permissions Policy & Feature Policy

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 42 | Permissions Policy header — `Permissions-Policy: camera=(), microphone=(), geolocation=(), ...` | all | not-started | widespread | P3 | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/permissions-policy-1/ ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Permissions-Policy |
| 43 | Permissions Policy in `<iframe allow>` attribute — delegate feature access to embedded content | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/permissions-policy-1/#iframe-allow-attribute |
| 44 | Feature Policy (deprecated predecessor) — `Feature-Policy: vibrate 'self'; geolocation 'none'` | all | not-started | widespread | P3 | M | Chromium yes/stable (deprecated in favour of Permissions-Policy); Gecko no; WebKit partial; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Feature-Policy |
| 45 | Default allowlists — `'self'`, `'src'` (for iframes), `*` per feature | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/permissions-policy-1/#default-allowlists |

---

## 3.10 Sandboxing

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 46 | `<iframe sandbox>` — restrict iframe capabilities (scripts, forms, popups, same-origin, etc.) | all | not-started | ubiquitous | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/iframe-embed-object.html#attr-iframe-sandbox |
| 47 | CSP `sandbox` directive — apply sandbox flags to entire page via header | all | not-started | ubiquitous | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/CSP3/#directive-sandbox |
| 48 | Sandbox flags: `allow-same-origin`, `allow-scripts`, `allow-forms`, `allow-popups`, `allow-modals`, `allow-popups-to-escape-sandbox`, `allow-top-navigation`, `allow-top-navigation-by-user-activation`, `allow-downloads`, `allow-presentation`, `allow-orientation-lock`, `allow-pointer-lock` | all | not-started | ubiquitous | P3 | M | all yes/stable (flag set varies by engine) | https://html.spec.whatwg.org/multipage/iframe-embed-object.html#attr-iframe-sandbox |
| 49 | Sandboxed origin unique generation — each sandboxed iframe gets a unique opaque origin | all | not-started | ubiquitous | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/browsers.html#sandboxed-origin |

---

## 3.11 Mixed Content

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 50 | Block active mixed content — block HTTP scripts, stylesheets, iframes on HTTPS pages | all | not-started | ubiquitous | P3 | M | all yes/stable | https://www.w3.org/TR/mixed-content/ ; https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content |
| 51 | Upgrade passive mixed content — auto-upgrade images, audio, video from HTTP to HTTPS | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/upgrade-insecure-requests/ ; https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content |
| 52 | `block-all-mixed-content` (deprecated CSP directive) — block all mixed HTTP subresources | all | not-started | deprecated | P3 | S | Chromium deprecated; Gecko deprecated; WebKit deprecated | https://www.w3.org/TR/CSP3/#directive-block-all-mixed-content |
| 53 | `Upgrade-Insecure-Requests: 1` header — auto-upgrade HTTP navigations and subresources | all | not-started | ubiquitous | P3 | S | all yes/stable | https://www.w3.org/TR/upgrade-insecure-requests/ |

---

## 3.12 Subresource Integrity (SRI) — Policy Dimension

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 54 | SRI on `<script>` — verify hash of fetched script matches `integrity` attribute | all | not-started | ubiquitous | P3 | M | all yes/stable | https://www.w3.org/TR/SRI/ ; https://developer.mozilla.org/en-US/docs/Web/Security/Subresource_Integrity |
| 55 | SRI on `<link rel="stylesheet">` — verify hash of fetched CSS | all | not-started | ubiquitous | P3 | M | all yes/stable | https://www.w3.org/TR/SRI/ |
| 56 | SRI on `<link rel="preload">`, `<link rel="modulepreload">` — integrity on preloaded resources | all | not-started | mixed | P3 | S | Chromium yes/stable; Gecko yes/stable; WebKit partial; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/SRI/ |
| 57 | SRI hash algorithms — SHA-256, SHA-384, SHA-512 (sha256, sha384, sha512 in spec) | all | not-started | ubiquitous | P3 | S | all yes/stable | https://www.w3.org/TR/SRI/#cryptographic-hash-functions |
| 58 | SRI fallback — allow multiple hashes, algorithm agility | all | not-started | ubiquitous | P3 | S | all yes/stable | https://www.w3.org/TR/SRI/#agility |

---

## 3.13 Referrer Policy

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 59 | `Referrer-Policy` header — control referrer information sent with requests | all | not-started | ubiquitous | P3 | M | all yes/stable | https://w3c.github.io/webappsec-referrer-policy/ ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy |
| 60 | Referrer Policy in `<meta name="referrer">` | all | not-started | ubiquitous | P3 | S | all yes/stable | https://w3c.github.io/webappsec-referrer-policy/#referrer-policy-delivery-meta |
| 61 | Referrer Policy in `referrerpolicy` attribute on `<a>`, `<area>`, `<img>`, `<script>`, `<link>`, `<iframe>` | all | not-started | ubiquitous | P3 | S | all yes/stable | https://w3c.github.io/webappsec-referrer-policy/#referrer-policy-delivery-element |
| 62 | `strict-origin-when-cross-origin` — default in modern browsers; send origin only cross-origin, full URL same-origin | all | not-started | ubiquitous | P3 | S | all yes/stable (default since Chromium 85, Gecko 87, WebKit 16.4) | https://w3c.github.io/webappsec-referrer-policy/#referrer-policy-strict-origin-when-cross-origin |
| 63 | `no-referrer` — send no referrer header at all | all | not-started | ubiquitous | P3 | S | all yes/stable | https://w3c.github.io/webappsec-referrer-policy/#referrer-policy-no-referrer |
| 64 | `same-origin` — send referrer only for same-origin requests | all | not-started | ubiquitous | P3 | S | all yes/stable | https://w3c.github.io/webappsec-referrer-policy/#referrer-policy-same-origin |

---

## 3.14 Cookie Security (Policy Dimension)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 65 | `Secure` flag — cookie only sent over HTTPS | all | not-started | ubiquitous | P3 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc6265#section-4.1.2.5 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#secure |
| 66 | `HttpOnly` flag — cookie inaccessible to JavaScript (`document.cookie`) | all | not-started | ubiquitous | P3 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc6265#section-4.1.2.6 |
| 67 | `SameSite=Lax` / `SameSite=Strict` / `SameSite=None` — cross-site sending policy | all | not-started | ubiquitous | P3 | M | all yes/stable (default=Lax in Chromium 80+, Gecko 69+, WebKit 13+) | https://www.rfc-editor.org/rfc/rfc6265bis#section-5.3.7 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#samesite |
| 68 | `__Secure-` prefix — enforce `Secure` flag on prefixed cookies | all | not-started | ubiquitous | P3 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc6265bis#section-5.2.4 |
| 69 | `__Host-` prefix — enforce `Secure` + no `Domain` + `Path=/` | all | not-started | ubiquitous | P3 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc6265bis#section-5.2.5 |
| 70 | CHIPS — `Partitioned` attribute for cookie partitioning (top-site-keyed) | all | not-started | mixed | P18+ | M | Chromium yes/stable; Gecko yes/stable; WebKit experimental; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/Privacy/Privacy_sandbox/Partitioned_cookies ; https://www.ietf.org/archive/id/draft-cutler-httpbis-partitioned-cookies/ |
| 71 | Third-party storage partitioning — per-top-site cache, storage, cookie jar partitioning | all | not-started | widespread | P18+ | L | Chromium yes/stable; Gecko yes/stable (Total Cookie Protection); WebKit yes/stable (cross-site tracking mitigation policy); Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/Privacy/State_Partitioning ; https://webkit.org/tracking-prevention/ |

---

## 3.15 Storage Access API

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 72 | `document.requestStorageAccess()` — request unpartitioned cookie/storage access in third-party context | all | not-started | widespread | P18+ | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://privacycg.github.io/storage-access/ ; https://developer.mozilla.org/en-US/docs/Web/API/Document/requestStorageAccess |
| 73 | `document.hasStorageAccess()` — check if storage access is already granted | all | not-started | widespread | P18+ | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://privacycg.github.io/storage-access/ |
| 74 | `document.getStorageAccessHandle()` — Storage Access API Level 2 (extended partitioned state access) | all | not-started | experimental | P18+ | L | Chromium experimental; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://privacycg.github.io/storage-access/ |
| 75 | `Storage-Access-Activate` header — HTTP-based storage access activation | all | not-started | experimental | P18+ | M | Chromium experimental; others no | https://privacycg.github.io/storage-access/ |
| 76 | `Sec-Fetch-Storage-Access` header — indicate storage access mode in fetch metadata | all | not-started | experimental | P18+ | S | Chromium experimental; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://privacycg.github.io/storage-access/ |

---

## 3.16 Private State Tokens (formerly Trust Tokens)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 77 | Private State Token issuance — origin-bound cryptographic token for anti-fraud attestation | all | not-started | experimental | P22+ | L | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://developer.chrome.com/docs/privacy-sandbox/private-state-tokens/ ; https://wicg.github.io/private-state-token-api/ |
| 78 | Private State Token redemption — present token to different origin for verification | all | not-started | experimental | P22+ | L | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://wicg.github.io/private-state-token-api/ |
| 79 | `Sec-Private-State-Token-Crypto-Version` header — negotiate crypto version | all | not-started | experimental | P22+ | S | Chromium yes/stable; others no | https://wicg.github.io/private-state-token-api/ |

---

## 3.17 Federated Credential Management (FedCM)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 80 | FedCM `navigator.credentials.get({identity: ...})` — mediated identity provider sign-in | all | not-started | mixed | P22+ | L | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://fedidcg.github.io/FedCM/ ; https://developer.mozilla.org/en-US/docs/Web/API/FedCM_API |
| 81 | FedCM IdP manifest (`/.well-known/web-identity`) — identity provider configuration endpoint | all | not-started | mixed | P22+ | M | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://fedidcg.github.io/FedCM/ |
| 82 | FedCM account list endpoint — display user's accounts for selection | all | not-started | mixed | P22+ | M | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://fedidcg.github.io/FedCM/ |

---

## 3.18 Popups & Window Restrictions

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 83 | `rel="noopener"` — prevent opener reference on `<a target="_blank">` | all | not-started | ubiquitous | P3 | S | all yes/stable (Chromium/Gecko/WebKit default noopener since ~2021) | https://html.spec.whatwg.org/multipage/links.html#link-type-noopener |
| 84 | `rel="noreferrer"` — suppress referrer and opener reference | all | not-started | ubiquitous | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/links.html#link-type-noreferrer |
| 85 | Noopener-by-default for `target="_blank"` — implicit noopener without explicit `rel` | all | not-started | ubiquitous | P3 | S | all yes/stable (Chromium 88+, Gecko 79+, WebKit 15+) | https://html.spec.whatwg.org/multipage/links.html#following-hyperlinks |
| 86 | Popup blocker — block `window.open()` not triggered by user gesture | all | not-started | ubiquitous | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/browsers.html#the-rules-for-choosing-a-browsing-context-given-a-browsing-context-name |
| 87 | Pop-under prevention — block windows opened behind current window | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/API/Window/open |
| 88 | `window.focus()` / `window.blur()` restrictions — limit programmatic focus changes | all | not-started | ubiquitous | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/interaction.html#dom-window-focus |
| 89 | Cross-origin window property restrictions — `window.length`, `window.closed`, `window.location` getter restrictions | all | not-started | ubiquitous | P3 | M | all yes/stable | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-properties |

---

## 3.19 Content Type Sniffing

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 90 | `X-Content-Type-Options: nosniff` — prevent MIME type sniffing for script and stylesheet | all | not-started | ubiquitous | P3 | S | all yes/stable | https://mimesniff.spec.whatwg.org/#x-content-type-options-header ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options |
| 91 | MIME type sniffing algorithm — WHATWG MIME Sniff spec (safelisted, sniffable, explicitly nosniff) | all | not-started | ubiquitous | P3 | M | all yes/stable | https://mimesniff.spec.whatwg.org/ |
| 92 | `nosniff` enforcement on `<script>` and `<link rel="stylesheet">` — block if type doesn't match | all | not-started | ubiquitous | P3 | S | all yes/stable | https://mimesniff.spec.whatwg.org/#does-a-resource-parsing-algorithm-abort-on-a-nosniff |

---

## 3.20 XSS Protection & Trusted Types

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 93 | `X-XSS-Protection: 0` header — disable legacy XSS auditor (Chrome/Edge removed auditor in 2019) | all | not-started | deprecated | P3 | S | Chromium removed (Chrome 78); Gecko never implemented; WebKit removed; others no | https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-XSS-Protection |
| 94 | Trusted Types enforcement — block DOM XSS sinks without typed values (see §3.2 #10–11) | all | not-started | mixed | P3 | L | see §3.2 #10–11 | https://w3c.github.io/trusted-types/dist/spec/ |
| 95 | `Content-Type-Options: nosniff` + CSP `script-src` — combined hardening against MIME confusion attacks | all | not-started | ubiquitous | P3 | S | all yes/stable | https://w3c.github.io/webappsec-csp/ ; https://mimesniff.spec.whatwg.org/ |

---

## 3.21 Permission Delegation & Device Access

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 96 | Permissions API — `navigator.permissions.query()` for standardised permission status | all | not-started | ubiquitous | P3 | M | all yes/stable | https://w3c.github.io/permissions/ ; https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API |
| 97 | `<iframe allow>` permission delegation — camera, microphone, geolocation delegated to iframe | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/permissions-policy-1/#iframe-allow-attribute |
| 98 | Permissions Policy delegation inheritance — child frame inherits parent's policy by default | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/permissions-policy-1/#algo-define |

---

## 3.22 Download Restrictions

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 99 | `Content-Disposition: attachment` — force download instead of inline display | all | not-started | ubiquitous | P3 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc6266 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition |
| 100 | `<a download>` attribute — trigger download with suggested filename | all | not-started | ubiquitous | P3 | S | all yes/stable | https://html.spec.whatwg.org/multipage/links.html#downloading-resources |
| 101 | Cross-origin download restrictions — block `<a download>` on cross-origin URLs (same-origin filename allowed) | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://html.spec.whatwg.org/multipage/links.html#downloading-resources |
| 102 | Mixed-content download blocking — warn/block on HTTP downloads from HTTPS pages | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content |
| 103 | Dangerous file type warnings — warn on `.exe`, `.dmg`, `.apk` downloads from untrusted sources | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | (vendor-specific heuristic, no spec) |
| 104 | `Content-Disposition: inline` + `sandbox` without `allow-downloads` — block downloads from sandboxed context | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://html.spec.whatwg.org/multipage/iframe-embed-object.html#attr-iframe-sandbox |

---

## 3.23 Site Isolation / Process Isolation

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 105 | Out-of-process iframes — cross-origin iframes in separate renderer processes | all | not-started | widespread | P3 | XL | Chromium yes/stable; Gecko yes/stable (Fission); WebKit yes/stable (per-origin Web Content); Servo no; Ladybird no; Flow no | https://www.chromium.org/developers/design-documents/site-isolation/ ; https://wiki.mozilla.org/Project_Fission |
| 106 | Process-per-site / process-per-origin — group same-site tabs into one process | all | not-started | widespread | P3 | XL | Chromium yes/stable; Gecko yes/stable (Fission); WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.chromium.org/developers/design-documents/process-models/ |
| 107 | Site Isolation for Spectre — isolate cross-origin data in separate address spaces | all | not-started | widespread | P3 | XL | Chromium yes/stable; Gecko yes/stable (Fission); WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.chromium.org/Home/chromium-security/site-isolation/ |

---

## 3.24 HTTP/0.9 & Downgrade Blocking

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 108 | HTTP/0.9 blocking — block HTTP/0.9 responses on non-default ports | all | not-started | widespread | P3 | S | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP |
| 109 | HTTP downgrade prevention — block insecure redirects from HTTPS to HTTP | all | not-started | ubiquitous | P3 | M | all yes/stable | https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content |

---

## 3.25 Clear-Site-Data

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 110 | `Clear-Site-Data: "cache"`, `"cookies"`, `"storage"`, `"*"` — clear browsing data via HTTP header | all | not-started | mixed | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/clear-site-data/ ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Clear-Site-Data |

---

## 3.26 Reporting API & Network Error Logging (NEL)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 111 | Reporting API — `Report-To` / `Reporting-Endpoints` header, violation report delivery | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/reporting-1/ ; https://developer.mozilla.org/en-US/docs/Web/API/Reporting_API |
| 112 | NEL (Network Error Logging) — `NEL` header, collect network failure telemetry from client | all | not-started | mixed | P3 | M | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/network-error-logging-1/ ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Network_Error_Logging |
| 113 | Deprecation reports — `Report` type `deprecation` for API deprecation warnings | all | not-started | mixed | P3 | S | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/reporting-1/#deprecation |
| 114 | Intervention reports — `Report` type `intervention` for browser intervention notifications | all | not-started | mixed | P3 | S | Chromium yes/stable; Gecko no; WebKit no; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/reporting-1/#intervention |

---

## 3.27 Fingerprinting Resistance

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 115 | Canvas fingerprinting noise — inject random noise into `canvas.toDataURL()` and `toBlob()` | desktop+mobile | not-started | mixed | P22+ | M | Gecko yes/stable (RFP: `privacy.resistFingerprinting`); Chromium no (Brave farbles); WebKit partial; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/Privacy/Fingerprint_protection_on_firefox |
| 116 | WebGL parameter hash rounding — reduce precision of WebGL vendor/renderer strings | desktop+mobile | not-started | mixed | P22+ | M | Gecko yes/stable (RFP); Chromium no (Brave farbles); WebKit partial; Servo no; Ladybird no; Flow no | https://brave.com/privacy-updates/4-fingerprinting-defenses-2.0/ |
| 117 | AudioContext noise — add noise to AudioContext output for fingerprinting resistance | desktop+mobile | not-started | mixed | P22+ | M | Gecko yes/stable (RFP); Chromium no (Brave farbles); WebKit no; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/Privacy/Fingerprint_protection_on_firefox |
| 118 | `navigator.connection` throttling — reduce precision of Network Information API | desktop+mobile | not-started | mixed | P22+ | S | Gecko yes/stable (RFP: returns "unknown"); Chromium no; WebKit no; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation |
| 119 | `navigator.hardwareConcurrency` clamping — limit reported CPU core count | desktop+mobile | not-started | mixed | P22+ | S | Gecko yes/stable (RFP: clamps to 2); Chromium no; WebKit no; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency |
| 120 | `navigator.plugins` / `navigator.mimeTypes` — empty or spoofed plugin list | all | not-started | ubiquitous | P3 | S | all yes/stable (deprecated APIs return empty lists since Chromium 90, Gecko 90, WebKit 15) | https://developer.mozilla.org/en-US/docs/Web/API/Navigator/plugins |
| 121 | User-Agent Client Hints (`Sec-CH-UA*`) — reduce UA string entropy | all | not-started | widespread | P3 | M | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://wicg.github.io/ua-client-hints/ ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers#client_hints |
| 122 | `navigator.globalPrivacyControl` (GPC) — signal user's opt-out of cross-site tracking | all | not-started | mixed | P22+ | S | Chromium no (removed); Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://globalprivacycontrol.github.io/gpc-spec/ |

---

## 3.28 Private Browsing / Incognito Mode

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 123 | Private browsing — ephemeral session that does not persist cookies, storage, history, cache | all | not-started | ubiquitous | P12+ | L | all yes/stable | https://developer.mozilla.org/en-US/docs/Web/Privacy/Private_Browsing |
| 124 | Incognito/private mode storage detection — `navigator.storage.estimate()` returns special values | all | not-started | widespread | P12+ | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate |
| 125 | Extension access restrictions in private browsing — extensions disabled by default in private mode | all | not-started | ubiquitous | P12+ | M | all yes/stable | (vendor-specific) |
| 126 | IndexedDB/private mode ephemeral behaviour — in-memory only, wiped on session end | all | not-started | ubiquitous | P12+ | M | all yes/stable | (vendor-specific) |

---

## 3.29 Viewport & Accessibility Security

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 127 | `<meta viewport>` `user-scalable=no` — prevent user zoom (accessibility concern, browsers may override) | mobile | not-started | widespread | P3 | S | Chromium yes/stable (with override for a11y); Gecko yes/stable (ignores `user-scalable=no`); WebKit yes/stable (ignores in iOS Safari 10+) | https://developer.mozilla.org/en-US/docs/Web/HTML/Viewport_meta_tag |
| 128 | `<meta viewport>` `maximum-scale` — limit zoom scale (accessibility concern) | mobile | not-started | widespread | P3 | S | Chromium yes/stable (with override); Gecko yes/stable (ignores); WebKit yes/stable (ignores in iOS Safari 10+) | https://developer.mozilla.org/en-US/docs/Web/HTML/Viewport_meta_tag |

---

## Cross-refs

These rows map to existing items in `specs/GAP_ANALYSIS.md`:

| This chunk row | GAP_ANALYSIS reference | Notes |
|----------------|------------------------|-------|
| #1–6 (SOP) | §4.1 Same-Origin Policy enforcement `[ ]` | Not started. |
| #7–14 (CSP) | §4.1 CSP parser & enforcement `[ ]` | Not started. |
| #33–35 (HSTS) | §4.1 HSTS / HSTS preload `[ ]` | Not started. |
| #36–41 (CT/revocation) | §4.1 Certificate transparency `[ ]`, §4.1 OCSP `[ ]` | Not started. |
| #42–45 (Permissions Policy) | §4.1 Permissions Policy `[ ]` | Not started. |
| #46–49 (Sandboxing) | §4.1 `spiral-sandbox` `[~]` | Stub exists at `crates/spiral-sandbox/src/lib.rs`. |
| #50–53 (Mixed Content) | §4.1 Mixed-content blocking `[ ]` | Not started. |
| #54–58 (SRI) | §4.1 SRI `[ ]`, §2.2 SRI `[ ]` | Not started. `spiral-crypto::sha256` now functional (Delta G0.1). |
| #59–64 (Referrer Policy) | §4.1 Referrer-Policy `[ ]` | Not started. |
| #65–71 (Cookie security) | §4.1 Secure cookie flags `[ ]`, §4.1 Cookie partitioning `[ ]` | Not started. |
| #72–76 (Storage Access) | §4.1 Cookie partitioning (CHIPS/Storage Access API) `[ ]` | M18+ per roadmap. |
| #77–79 (Private State Tokens) | No existing reference. | New row for §4.1. |
| #80–82 (FedCM) | No existing reference. | New row for §4.1. |
| #83–89 (Popups) | No existing reference. | New row for §4.1. |
| #90–92 (Content type sniffing) | No existing reference. | New row for §4.1. |
| #93–95 (XSS/Trusted Types) | No existing reference. | New row for §4.1. |
| #96–98 (Permission delegation) | §4.1 Permissions Policy `[ ]` | Subset of #42–45. |
| #99–104 (Downloads) | No existing reference. | New row for §4.1. |
| #105–107 (Site Isolation) | No existing reference. | New row for §4.1. Bet 1 (SEM) in `docs/architecture/design/shared-everything.md` designs a capability-type alternative. |
| #110 (Clear-Site-Data) | No existing reference. | New row for §4.1. |
| #111–114 (Reporting API/NEL) | No existing reference. | New row for §4.1. |
| #115–122 (Fingerprinting) | §4.1 Anti-fingerprinting posture `[ ]` | Not started. |
| #123–126 (Private browsing) | No existing reference. | New row for §4.1. |
| #127–128 (Viewport security) | No existing reference. | New row for §4.1. |

---

## Spiral-side grounding

**What exists:**

- `spiral-sandbox` — stub at `crates/spiral-sandbox/src/lib.rs`. `Sandbox::new()` and `Sandbox::init()` exist; platform-specific branches log intent (Landlock+seccomp-bpf, Seatbelt, Restricted Token) but do not invoke OS APIs. `is_active()` returns true after `init()`.
- `spiral-filter` — full ad-filter policy engine at `crates/spiral-filter/`. Rule AST (`rule.rs`) includes `Action::Csp { policy }` for injecting CSP headers. Policy levels: Off → WorstOffenders → CommonAnnoyances → PrivacyFocused → Strict → Maximum. Not wired into the browser pipeline.
- `spiral-crypto` — CSPRNG via `getrandom`, SHA-256 via `sha2` crate. Functional after Delta G0.1 fix. `generate_hex_token()` produces random hex for SRI hashes. No SRI validation logic exists.
- `spiral-context` — capability-type skeleton (21 tests). `Origin`, `CapabilitySet`, `Context`, `ContextOps` defined. No runtime; types only.
- Bet 1 (SEM) in `docs/architecture/design/shared-everything.md` — capability-typed security model with per-origin contexts in a single renderer process. OS-level sandbox as escalation for sensitive origins. No implementation beyond type skeleton.
- Bet 3 (filter) in `crates/spiral-filter/` — content filter / privacy model with CBA-derived policy rules. CSP injection in rule actions exists structurally but is not executed.

**What does not exist:**

- No SOP enforcement, no CORS header parsing, no CORP/COOP/COEP enforcement.
- No CSP parser or enforcement engine.
- No HSTS preload list, no CT validation, no OCSP handling.
- No Permissions Policy enforcement.
- No cookie security flags (no cookie jar at all).
- No referrer policy enforcement.
- No SRI validation.
- No fingerprinting resistance measures.
- No private browsing mode.
- No site isolation / out-of-process iframes (SEM designs this but no runtime).

---

## Open questions for the user

- **CSP enforcement priority:** CSP is large (Levels 1–3 + Trusted Types). Should Spiral aim for CSP Level 2 compliance first (ubiquitous, simpler) and defer `strict-dynamic` / Trusted Types to later phases? Trusted Types adoption is mixed (Chromium+Gecko yes, WebKit experimental).
- **Site isolation model:** Bet 1 (SEM) designs capability-typed security as the default, with OS-level process isolation as escalation. Should Spiral aim for process-per-origin from day one (Chromium-class), or trust the type system for default security and only escalate on user request?
- **Fingerprinting resistance depth:** Firefox RFP is comprehensive but breaks many sites. Brave's farble approach is more compatible. Should Spiral adopt a permissive (Brave-like) or strict (Firefox-like) fingerprinting resistance posture?
- **FedCM / Private State Tokens:** These are Chromium-led proposals with limited multi-engine adoption. Should Spiral implement them, wait for multi-engine consensus, or skip entirely?
- **Storage Access API vs CHIPS:** Both address third-party storage partitioning. Should Spiral implement both, or prioritise CHIPS (the simpler mechanism)?
- **Reporting API scope:** The Reporting API delivers CSP violation reports, COOP/COEP reports, deprecation reports, NEL reports, and intervention reports. Should Spiral implement the full Reporting API in Phase 3, or defer NEL and intervention reports to Phase 4+?
- **Clear-Site-Data support:** Only Chromium and Gecko implement this. Is it a Phase 3 priority, or can it wait?
- **Private browsing scope:** Should private browsing be a Phase 3 feature (early, high user impact) or Phase 12+ (when storage infrastructure is mature)?
