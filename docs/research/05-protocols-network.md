# Chunk 2 — Networking & Protocols

> **Companion to the research index in `README.md`.** This file covers
> the network stack — transport, application, and resolution layers —
> and any protocol-level capabilities a modern browser must or should
> support.
>
> **Style of rows:** Protocol support = `S`–`M`; TLS/DNS subsystems =
> `L`; full HTTP/3+QUIC stack = `XL`. Per `00-methodology.md` §6.

---

## Scope

**In:** HTTP/1.1, HTTP/2, HTTP/3, QUIC, WebSocket, WebTransport,
TLS 1.2/1.3 and certificate handling, DNS transport and resolution,
HTTP caching, HTTP authentication, preloading/speculation, Web Push,
CORS/CORP/COEP transport-relevant subset, Service Workers transport
subset, proxy support, content encoding, data/blob/object URLs,
connection management, subresource integrity (transport-relevant).

**Out:** Security policy detail (CSP, SRI, HSTS, Mixed Content) —
chunk 3. Storage & state (cookies, localStorage, IndexedDB) — chunk 4.
Media transport (WebRTC, RTP, DTLS-SRTP) — chunk 5. Platform APIs
(Fetch, Streams, Workers) — chunk 6. Developer tools — chunk 8.

---

## Methodology for this chunk

Rows derived from: IETF RFCs (Tier 1), WHATWG Fetch/HTML specs
(Tier 1), MDN browser-compat-data (Tier 2), Can I Use (Tier 2),
Chrome Platform Status (Tier 3), Firefox release notes (Tier 3),
WebKit blog (Tier 3), Servo GitHub issues/PRs (Tier 3), Ladybird
GitHub issues (Tier 3). Engine notes are one-line per engine;
full engine-detail deferred to chunk 7.

---

## 2.1 HTTP Transport

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 1 | HTTP/1.1 (RFC 9112) — persistent connections, chunked transfer encoding, Host header | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc9112 ; https://developer.mozilla.org/en-US/docs/Web/HTTP |
| 2 | HTTP/2 (RFC 9113) — multiplexing, HPACK header compression, stream prioritisation, server push (deprecated) | all | not-started | ubiquitous | P4 | L | all yes/stable (server push deprecated in Chromium 106+) | https://www.rfc-editor.org/rfc/rfc9113 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP |
| 3 | HTTP/2 cleartext upgrade (h2c) | desktop | not-started | niche | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9113#section-3.4 |
| 4 | HTTP/3 (RFC 9114) — QPACK header compression, stream multiplexing over QUIC | all | not-started | widespread | P4 | XL | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9114 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP |
| 5 | QUIC transport (RFC 9000) — UDP-based, 1-RTT handshake, built-in TLS 1.3 | all | not-started | widespread | P4 | XL | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9000 |
| 6 | HTTP/3 connection migration — seamless network switch using QUIC connection IDs | mobile+embedded | not-started | widespread | P4 | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9000#section-9 |
| 7 | QUIC 0-RTT — early data on resumed connections | all | not-started | widespread | P4 | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9000#section-4.1 ; https://www.rfc-editor.org/rfc/rfc8446#section-4.2.1 |
| 8 | Alt-Svc header (RFC 7838) — advertise alternative service endpoints (HTTP/3 upgrade path) | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc7838 |
| 9 | TCP keep-alive and connection reuse | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc9112#section-9.3 |
| 10 | HTTP/2+ connection coalescing — reuse TLS connection for aliased origins | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9113#section-9.1.1 |
| 11 | ORIGIN frame (RFC 8336) — HTTP/2 origin set declaration | desktop | not-started | niche | P4 | S | Chromium yes/stable; Gecko no; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc8336 |
| 12 | HTTP/2 GOAWAY and graceful shutdown | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc9113#section-6.8 |

---

## 2.2 WebSocket & WebTransport

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 13 | WebSocket (RFC 6455) — full-duplex text and binary frames over a single TCP connection | all | not-started | ubiquitous | P4 | L | all yes/stable | https://www.rfc-editor.org/rfc/rfc6455 ; https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API |
| 14 | WebSocket compression (permessage-deflate, RFC 7692) | all | not-started | widespread | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc7692 |
| 15 | WebSocket over TLS (wss:// scheme) | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc6455#section-11.1.2 |
| 16 | WebTransport (W3C) — bidirectional streams and datagrams over HTTP/3 | all | not-started | mixed | P4 | XL | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/webtransport/ ; https://developer.mozilla.org/en-US/docs/Web/API/WebTransport |

---

## 2.3 TLS & Certificate Handling

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 17 | TLS 1.2 (RFC 5246) — legacy support | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc5246 |
| 18 | TLS 1.3 (RFC 8446) — 1-RTT handshake, 0-RTT resumption, AEAD-only ciphersuites | all | not-started | ubiquitous | P4 | L | all yes/stable | https://www.rfc-editor.org/rfc/rfc8446 |
| 19 | X.509 path building and validation (RFC 5280+) | all | not-started | ubiquitous | P4 | L | all yes/stable | https://www.rfc-editor.org/rfc/rfc5280 |
| 20 | Certificate Transparency (CT, RFC 6962) — SCT verification | all | not-started | ubiquitous | P4 | M | all yes/stable (Chromium mandated since 2018) | https://www.rfc-editor.org/rfc/rfc6962 ; https://developer.mozilla.org/en-US/docs/Web/Security/Certificate_Transparency |
| 21 | OCSP stapling (RFC 6066 §8) — server-provided revocation proof | all | not-started | ubiquitous | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc6066#section-8 |
| 22 | OCSP must-staple (TLS feature extension, RFC 7633) | all | not-started | mixed | P4 | S | Chromium no; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc7633 |
| 23 | CRLite — Firefox's compressed CRL set for client-side revocation | desktop | not-started | niche | P4 | M | Gecko yes/stable (Firefox 86+); others no | https://blog.mozilla.org/security/2020/08/25/crlite-compressed-revocation/ |
| 24 | CRLSets — Chromium's server-pushed compressed CRL | desktop | not-started | niche | P4 | M | Chromium yes/stable; others no (different mechanism) | https://chromium.googlesource.com/chromium/src/+/main/net/cert/crl_set.md |
| 25 | AIA (Authority Information Access) fetching — on-demand intermediate cert retrieval | all | not-started | ubiquitous | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc5280#section-4.2.2.1 |
| 26 | TLS certificate compression (RFC 8879) — X.509 compressed with Brotli/Zstandard | all | not-started | widespread | P4 | S | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc8879 |
| 27 | Delegated credentials (RFC 9345) — short-lived TLS credentials | all | not-started | mixed | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9345 |
| 28 | TLS Encrypted ClientHello (ECH, RFC draft) — encrypt SNI in ClientHello | all | not-started | experimental | P4 | L | Chromium behind flag; Gecko behind flag; WebKit no; Servo no; Ladybird no; Flow no | https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni/ ; https://chromestatus.com/feature/5124606246518784 |
| 29 | TLS session resumption (session tickets, RFC 8446 §4.6.1) | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc8446#section-4.6.1 |
| 30 | TLS ALPN (Application-Layer Protocol Negotiation, RFC 7301) — h2/h3 negotiation | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc7301 |

---

## 2.4 DNS Resolution & Transport

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 31 | DNS-over-HTTPS (DoH, RFC 8484) — DNS queries over HTTPS/2 or HTTPS/3 | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc8484 ; https://developer.mozilla.org/en-US/docs/Mozilla/Projects/NSPR/Reference/DNS_over_HTTPS |
| 32 | DNS-over-TLS (DoT, RFC 7858) — DNS queries over TLS on port 853 | all | not-started | niche | P4 | M | Chromium no; Gecko no (system only); WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc7858 |
| 33 | DNS-over-QUIC (DoQ, RFC 9250) — DNS queries over QUIC | all | not-started | experimental | P4 | M | Chromium no; Gecko no; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9250 |
| 34 | DNSSEC validation (RFC 4033–4035) — client-side DNS security | all | not-started | niche | P4 | L | Chromium no; Gecko yes/stable (with DNSSEC-aware resolver); WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc4033 ; https://www.rfc-editor.org/rfc/rfc4035 |
| 35 | SVCB/HTTPS DNS records (RFC 9460/9461) — service binding and HTTPS records for HTTP bootstrapping | all | not-started | mixed | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9460 ; https://www.rfc-editor.org/rfc/rfc9461 |
| 36 | Happy Eyeballs v2 (RFC 8305) — dual-stack IPv4/IPv6 racing with TCP fallback | all | not-started | ubiquitous | P4 | M | all yes/stable (Chromium/Gecko/WebKit all implement the algorithm) | https://www.rfc-editor.org/rfc/rfc8305 |
| 37 | Async DNS resolver — non-blocking name resolution | all | partial (Resolver trait designed, stub only) | ubiquitous | P4 | M | all yes/stable | https://developer.mozilla.org/en-US/docs/Web/API/DNS |
| 38 | DNS caching — client-side TTL-respecting cache | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc1035#section-3.2.1 ; https://developer.mozilla.org/en-US/docs/Web/Performance/Understanding_latency |
| 39 | EDNS Client Subnet (ECS, RFC 7871) — relay client subnet for CDN routing | all | not-started | niche | P4 | S | Chromium no; Gecko no; WebKit no; Servo no; Ladybird no; Flow no (typically OS resolver handles) | https://www.rfc-editor.org/rfc/rfc7871 |
| 40 | DNS prefetch (rel=dns-prefetch) — hint to resolve hostname before navigation | all | not-started | ubiquitous | P4 | S | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel/dns-prefetch |
| 41 | mDNS (RFC 6762) — multicast DNS for .local names | all | not-started | niche | P4 | M | Chromium yes/stable (ChromeOS, Android mDNS); Gecko no; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc6762 |

---

## 2.5 HTTP Caching

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 42 | ETag / If-None-Match — conditional cache validation | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc9110#section-8.8.3 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag |
| 43 | Last-Modified / If-Modified-Since — date-based conditional requests | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc9110#section-8.8.2 |
| 44 | Cache-Control directives (max-age, no-cache, no-store, must-revalidate, public, private, s-maxage, proxy-revalidate) | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc9111 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control |
| 45 | Vary header — content-negotiation-aware cache key | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc9110#section-12.5.3 |
| 46 | stale-while-revalidate (RFC 5861) — serve stale cache while revalidating in background | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc5861 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control |
| 47 | immutable cache directive (RFC 8246) — no revalidation for unversioned resources | all | not-started | widespread | P4 | S | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc8246 |
| 48 | Cache API (programmatic cache via Service Workers) | all | not-started | ubiquitous | P4 | M | all yes/stable | https://developer.mozilla.org/en-US/docs/Web/API/Cache |
| 49 | Double-keyed / partitioned caching — per-origin cache partitioning to prevent cross-site tracking | all | not-started | widespread | P4 | M | Chromium yes/stable (since 2020); Gecko yes/stable (Total Cookie Protection); WebKit yes/stable (ITP); Servo no; Ladybird no; Flow no | https://chromestatus.com/feature/5765498209881088 ; https://developer.mozilla.org/en-US/docs/Web/Privacy/State_Partitioning |
| 50 | Range requests / partial content (RFC 9110 §14) — byte-range serving and If-Range | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc9110#section-14 |

---

## 2.6 HTTP Authentication

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 51 | HTTP Basic authentication (RFC 7617) | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc7617 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication |
| 52 | HTTP Digest authentication (RFC 7616) | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc7616 |
| 53 | Negotiate/SPNEGO (RFC 4559) — Kerberos/NTLM integrated auth | desktop | not-started | niche | P4 | L | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc4559 |
| 54 | NTLM authentication — Windows-native challenge/response | desktop | not-started | niche | P4 | L | Chromium yes/stable; Gecko yes/stable; WebKit partial (macOS only); Servo no; Ladybird no; Flow no | https://learn.microsoft.com/en-us/windows/win32/secauthn/microsoft-ntlm |
| 55 | OAuth 2.0 device flow (RFC 8628) — device authorisation grant | all | not-started | experimental | P4 | M | all no (browser-managed; typically handled by web app, not engine) | https://www.rfc-editor.org/rfc/rfc8628 |
| 56 | WebAuthn integration — credential management for authentication | all | not-started | widespread | P4 | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/webauthn-3/ ; https://developer.mozilla.org/en-US/docs/Web/API/Web_Authentication_API |

---

## 2.7 Preloading, Prefetching & Speculation

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 57 | Preconnect (rel=preconnect) — early TCP+TLS handshake hint | all | not-started | ubiquitous | P4 | S | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel/preconnect |
| 58 | Preload (rel=preload) — fetch resource for current navigation | all | not-started | ubiquitous | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/preload/ ; https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel/preload |
| 59 | Prefetch (rel=prefetch) — fetch resource for likely future navigation | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel/prefetch |
| 60 | Speculation Rules (document rules) — JSON-driven prerender/prefetch declarations | all | not-started | experimental | P4 | L | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://wicg.github.io/nav-speculation/speculation-rules.html ; https://chromestatus.com/feature/5765498209881088 |
| 61 | Prerender (rel=prerender / Speculation Rules) — pre-render full page in hidden context | all | not-started | experimental | P4 | XL | Chromium yes/stable; Gecko no; WebKit no; Servo no; Ladybird no; Flow no | https://wicg.github.io/nav-speculation/ ; https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel/prerender |
| 62 | Priority Hints (fetchpriority attribute) — influence resource fetch priority | all | not-started | widespread | P4 | S | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/priority-hints/ ; https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/fetchpriority |

---

## 2.8 Web Push

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 63 | Push API — subscribe to push messages from a server | all | not-started | widespread | P4 | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable (Safari 16.4+); Servo no; Ladybird no; Flow no | https://www.w3.org/TR/push-api/ ; https://developer.mozilla.org/en-US/docs/Web/API/Push_API |
| 64 | Notification API — display system notifications | all | not-started | ubiquitous | P4 | M | all yes/stable | https://notifications.spec.whatwg.org/ ; https://developer.mozilla.org/en-US/docs/Web/API/Notifications_API |
| 65 | Web Push protocol (RFC 8030) — message delivery to push services | all | not-started | widespread | P4 | L | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc8030 |
| 66 | VAPID (RFC 8292) — Voluntary Application Server Identification for Web Push | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc8292 |
| 67 | Web Push message encryption (RFC 8291) — ECDH + HKDF content encryption | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc8291 |

---

## 2.9 Transport-Relevant Security Policies

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 68 | CORS (Cross-Origin Resource Sharing) — cross-origin fetch with preflight and credentialed requests | all | not-started | ubiquitous | P4 | L | all yes/stable | https://fetch.spec.whatwg.org/#http-cors-protocol ; https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS |
| 69 | CORP (Cross-Origin-Resource-Policy) — restrict cross-origin no-cors loads | all | not-started | ubiquitous | P4 | M | all yes/stable | https://fetch.spec.whatwg.org/#cross-origin-resource-policy-header ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Cross-Origin_Resource_Policy |
| 70 | COEP (Cross-Origin-Embedder-Policy) — require CORP/CORS on subresources | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://html.spec.whatwg.org/multipage/browsers.html#cross-origin-embedder-policy |
| 71 | CSP connect-src — restrict network request targets | all | not-started | ubiquitous | P4 | M | all yes/stable (transport-relevant subset only; full CSP in chunk 3) | https://www.w3.org/TR/CSP3/#directive-connect-src |
| 72 | Mixed content blocking — block HTTP subresources on HTTPS pages | all | not-started | ubiquitous | P4 | M | all yes/stable | https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content ; https://www.w3.org/TR/mixed-content/ |

---

## 2.10 Service Workers (Transport Subset)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 73 | Service Worker fetch interception — intercept and respond to network requests | all | not-started | ubiquitous | P4 | L | all yes/stable | https://www.w3.org/TR/service-workers/#fetch-event ; https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent |
| 74 | Service Worker offline mode — serve cached responses when network unavailable | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.w3.org/TR/service-workers/#cache-objects |
| 75 | Service Worker background sync — deferred network requests when connectivity resumes | all | not-started | mixed | P4 | M | Chromium yes/stable; Gecko experimental; WebKit no; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/background-fetch/ ; https://wicg.github.io/background-sync/spec/ |
| 76 | Service Worker navigation preload — parallel SW activation + network fetch | all | not-started | widespread | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit yes/stable; Servo no; Ladybird no; Flow no | https://www.w3.org/TR/service-workers/#navigation-preload |

---

## 2.11 Proxy Support

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 77 | HTTP CONNECT tunneling — proxy tunnel for HTTPS through HTTP proxy | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc9110#section-9.3.6 |
| 78 | PAC (Proxy Auto-Config) files — JavaScript-based proxy selection | all | not-started | ubiquitous | P4 | M | all yes/stable | https://developer.mozilla.org/en-US/docs/Web/HTTP/Proxy_servers_and_tunneling/Proxy_Auto-Configuration_PAC_file |
| 79 | WPAD (Web Proxy Auto-Discovery, RFC draft-wpad) — automatic PAC file discovery via DNS/DHCP | desktop | not-started | mixed | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit partial; Servo no; Ladybird no; Flow no | https://datatracker.ietf.org/doc/html/draft-wpad/ ; https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2008-R2-and-2008/cc995152(v=technet.10) |
| 80 | SOCKS4 / SOCKS5 proxy (RFC 1928) — TCP/UDP proxy protocol | all | not-started | ubiquitous | P4 | M | all yes/stable | https://www.rfc-editor.org/rfc/rfc1928 |
| 81 | HTTPS proxy (CONNECT over TLS, RFC 9298) — proxy connection wrapped in TLS | all | not-started | mixed | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit no; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9298 |
| 82 | Proxy authentication — Basic, NTLM, Negotiate to proxy server | desktop | not-started | ubiquitous | P4 | M | Chromium yes/stable; Gecko yes/stable; WebKit partial; Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc9110#section-11.7 |

---

## 2.12 Content Encoding (HTTP)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 83 | Brotli (RFC 7932) — compression for static and dynamic content | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc7932 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding |
| 84 | Zstandard (RFC 8878) — next-generation compression for HTTP content | all | not-started | mixed | P4 | S | Chromium yes/stable (Chrome 123+); Gecko yes/stable (Firefox 132+); WebKit yes/stable (Safari 17.4+); Servo no; Ladybird no; Flow no | https://www.rfc-editor.org/rfc/rfc8878 ; https://chromestatus.com/feature/5765498209881088 |
| 85 | gzip (RFC 1952) — legacy deflate compression | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc1952 |
| 86 | deflate (RFC 1951) — raw deflate without gzip wrapper | all | not-started | ubiquitous | P4 | S | all yes/stable (rarely used; gzip preferred) | https://www.rfc-editor.org/rfc/rfc1951 |
| 87 | Content-Encoding negotiation (Accept-Encoding header) — client-driven encoding selection | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc9110#section-12.5.3 |

---

## 2.13 Non-HTTP Resource URLs

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 88 | data: URLs (RFC 2397) — inline base64-encoded resources | all | not-started | ubiquitous | P4 | S | all yes/stable | https://www.rfc-editor.org/rfc/rfc2397 ; https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs |
| 89 | blob: URLs — opaque origin URLs for in-memory Blob objects | all | not-started | ubiquitous | P4 | M | all yes/stable | https://w3c.github.io/FileAPI/#url ; https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL |
| 90 | Object URLs (createObjectURL / revokeObjectURL) — generate blob: URL for File/Blob/MediaSource | all | not-started | ubiquitous | P4 | M | all yes/stable | https://w3c.github.io/FileAPI/#creating-revoking |
| 91 | about: URLs (about:blank, about:srcdoc) — browser-internal resource identifiers | all | not-started | ubiquitous | P4 | S | all yes/stable | https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/create |
| 92 | javascript: URLs — execute JS from address bar / href (legacy) | all | not-started | ubiquitous | P4 | S | all yes/stable (blocked in address bar for security; works in href) | https://developer.mozilla.org/en-US/docs/Web/URL/Schemes#javascript_url |
| 93 | file: URLs — local filesystem access | desktop | not-started | ubiquitous | P4 | M | all yes/stable (restricted; not for web content in modern engines) | https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs |

---

## 2.14 Subresource Integrity (Transport-Relevant Subset)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase | Complexity | Engine notes | Sources |
|---|------------|---------|------------------|--------------------|-------|------------|--------------|---------|
| 94 | Subresource Integrity (SRI) — integrity attribute on `<script>` and `<link>` | all | not-started | ubiquitous | P4 | M | all yes/stable (full SRI security analysis in chunk 3) | https://www.w3.org/TR/SRI/ ; https://developer.mozilla.org/en-US/docs/Web/Security/Subresource_Integrity |

---

## Cross-refs

These rows overlap with existing items in `specs/GAP_ANALYSIS.md`:

| This chunk row | GAP_ANALYSIS reference | Notes |
|----------------|------------------------|-------|
| #1–12 (HTTP transport) | §2.1 `spiral-network` HTTP client via hyper `[~]` | Stub exists at `crates/spiral-network/src/lib.rs:24-74`; returns empty 200 |
| #17–30 (TLS) | §2.1 `spiral-net` TLS via rustls `[ ]` | TlsConfig struct exists at `crates/spiral-net/src/lib.rs:8-17` but unused |
| #31–41 (DNS) | §2.1 `spiral-net` DNS resolver via hickory-dns `[~]` | Stub at `crates/spiral-net/src/lib.rs:25-56`; returns 127.0.0.1 |
| #13–16 (WebSocket/WebTransport) | §2.1 WebSockets `[ ]` | No code |
| #2 (HTTP/2) | §2.1 HTTP/2 `[ ]` | No code |
| #4 (HTTP/3) | §2.1 HTTP/3 `[ ]` | No code |
| #94 (SRI) | §2.2 Subresource Hash integrity `[ ]` | No code |
| #73–76 (Service Workers) | §2.1 Speculative caching `[ ]` | No code |
| #57–62 (Preloading) | §2.1 Speculative caching / preload `[ ]` | No code |

---

## Open questions for the user

- **DoT and DoQ in Spiral:** DNS-over-TLS and DNS-over-QUIC have essentially zero browser-level adoption (browsers prefer DoH). Should Spiral implement DoT/DoQ as a differentiator, or standardise on DoH only and leave DoT/DoQ to system resolvers?
- **CRLite vs CRLSets:** Firefox's CRLite is privacy-superior (no OCSP server contact) but Chromium uses CRLSets. Should Spiral adopt CRLite, CRLSets, or both? A custom revocation approach may be an ADR-worthy decision.
- **ECH (Encrypted ClientHello):** Still experimental behind flags in Chromium/Gecko. Is this a Phase 4 priority or can it wait until Phase 5/6?
- **NTLM / Negotiate:** These are Windows-enterprise-specific. Should Spiral support them on macOS/Linux, or only on Windows builds?
- **OAuth 2.0 device flow:** Currently no browser engine implements this natively (it's an application-layer concern). Should Spiral provide any engine-level support, or leave it to web apps?
- **Speculation Rules:** Chromium-only and still evolving. Should Spiral implement a compatible subset, or wait for a multi-engine consensus to emerge?
- **mDNS:** Chromium supports it on ChromeOS/Android. Is mDNS in scope for Spiral's desktop-first approach?
- **Content encoding:** Zstandard HTTP support shipped in Chromium/Gecko/WebKit in 2024. Should Spiral prioritise Zstandard over Brotli for initial implementation, given its higher compression ratio for dynamic content?
