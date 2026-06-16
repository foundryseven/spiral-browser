# Chunk 12 — Competitive Matrix: Networking & Protocols

**File:** `02-competitive-matrix-network.md`
**Date:** 2026-06-16
**Source:** `05-protocols-network.md`
**Row count:** 94

---

## 2.1 HTTP Transport

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 1 | HTTP/1.1 — persistent connections, chunked transfer encoding, Host header | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 2 | HTTP/2 — multiplexing, HPACK header compression, stream prioritisation, server push (deprecated) | all | not-started | ubiquitous | P4 | L | yes | yes | yes | yes | yes |
| 3 | HTTP/2 cleartext upgrade (h2c) | desktop | not-started | niche | P4 | M | yes | yes | no | no | no |
| 4 | HTTP/3 — QPACK header compression, stream multiplexing over QUIC | all | not-started | widespread | P4 | XL | yes | yes | yes | no | no |
| 5 | QUIC transport — UDP-based, 1-RTT handshake, built-in TLS 1.3 | all | not-started | widespread | P4 | XL | yes | yes | yes | no | no |
| 6 | HTTP/3 connection migration — seamless network switch via QUIC connection IDs | mobile+embedded | not-started | widespread | P4 | L | yes | yes | yes | no | no |
| 7 | QUIC 0-RTT — early data on resumed connections | all | not-started | widespread | P4 | L | yes | yes | yes | no | no |
| 8 | Alt-Svc header — advertise alternative service endpoints (HTTP/3 upgrade path) | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 9 | TCP keep-alive and connection reuse | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 10 | HTTP/2+ connection coalescing — reuse TLS connection for aliased origins | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 11 | ORIGIN frame — HTTP/2 origin set declaration | desktop | not-started | niche | P4 | S | yes | no | no | no | no |
| 12 | HTTP/2 GOAWAY and graceful shutdown | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |

---

## 2.2 WebSocket & WebTransport

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 13 | WebSocket — full-duplex text and binary frames over a single TCP connection | all | not-started | ubiquitous | P4 | L | yes | yes | yes | yes | yes |
| 14 | WebSocket compression (permessage-deflate) | all | not-started | widespread | P4 | M | yes | yes | yes | yes | yes |
| 15 | WebSocket over TLS (wss:// scheme) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 16 | WebTransport — bidirectional streams and datagrams over HTTP/3 | all | not-started | mixed | P4 | XL | yes | yes | yes | no | no |

---

## 2.3 TLS & Certificate Handling

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 17 | TLS 1.2 — legacy support | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 18 | TLS 1.3 — 1-RTT handshake, 0-RTT resumption, AEAD-only ciphersuites | all | not-started | ubiquitous | P4 | L | yes | yes | yes | yes | yes |
| 19 | X.509 path building and validation | all | not-started | ubiquitous | P4 | L | yes | yes | yes | yes | yes |
| 20 | Certificate Transparency — SCT verification | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 21 | OCSP stapling — server-provided revocation proof | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 22 | OCSP must-staple (TLS feature extension) | all | not-started | mixed | P4 | S | no | yes | no | no | no |
| 23 | CRLite — compressed CRL set for client-side revocation | desktop | not-started | niche | P4 | M | no | yes | no | no | no |
| 24 | CRLSets — server-pushed compressed CRL | desktop | not-started | niche | P4 | M | yes | no | no | no | no |
| 25 | AIA (Authority Information Access) fetching — on-demand intermediate cert retrieval | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 26 | TLS certificate compression (Brotli/Zstandard) | all | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 27 | Delegated credentials — short-lived TLS credentials | all | not-started | mixed | P4 | M | yes | yes | no | no | no |
| 28 | TLS Encrypted ClientHello (ECH) — encrypt SNI in ClientHello | all | not-started | experimental | P4 | L | behind-flag | behind-flag | no | no | no |
| 29 | TLS session resumption (session tickets) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 30 | TLS ALPN — h2/h3 negotiation | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |

---

## 2.4 DNS Resolution & Transport

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 31 | DNS-over-HTTPS (DoH) — DNS queries over HTTPS/2 or HTTPS/3 | all | not-started | widespread | P4 | M | yes | yes | no | no | no |
| 32 | DNS-over-TLS (DoT) — DNS queries over TLS on port 853 | all | not-started | niche | P4 | M | no | no | no | no | no |
| 33 | DNS-over-QUIC (DoQ) — DNS queries over QUIC | all | not-started | experimental | P4 | M | no | no | no | no | no |
| 34 | DNSSEC validation — client-side DNS security | all | not-started | niche | P4 | L | no | yes | no | no | no |
| 35 | SVCB/HTTPS DNS records — service binding and HTTPS records for HTTP bootstrapping | all | not-started | mixed | P4 | M | yes | yes | no | no | no |
| 36 | Happy Eyeballs v2 — dual-stack IPv4/IPv6 racing with TCP fallback | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 37 | Async DNS resolver — non-blocking name resolution | all | partial (Resolver trait designed, stub only) | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 38 | DNS caching — client-side TTL-respecting cache | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 39 | EDNS Client Subnet (ECS) — relay client subnet for CDN routing | all | not-started | niche | P4 | S | no | no | no | no | no |
| 40 | DNS prefetch (rel=dns-prefetch) — hint to resolve hostname before navigation | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 41 | mDNS — multicast DNS for .local names | all | not-started | niche | P4 | M | yes | no | no | no | no |

---

## 2.5 HTTP Caching

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 42 | ETag / If-None-Match — conditional cache validation | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 43 | Last-Modified / If-Modified-Since — date-based conditional requests | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 44 | Cache-Control directives (max-age, no-cache, no-store, must-revalidate, public, private, s-maxage, proxy-revalidate) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 45 | Vary header — content-negotiation-aware cache key | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 46 | stale-while-revalidate — serve stale cache while revalidating in background | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 47 | immutable cache directive — no revalidation for unversioned resources | all | not-started | widespread | P4 | S | yes | yes | no | no | no |
| 48 | Cache API (programmatic cache via Service Workers) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 49 | Double-keyed / partitioned caching — per-origin cache partitioning to prevent cross-site tracking | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 50 | Range requests / partial content — byte-range serving and If-Range | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |

---

## 2.6 HTTP Authentication

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 51 | HTTP Basic authentication | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 52 | HTTP Digest authentication | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 53 | Negotiate/SPNEGO — Kerberos/NTLM integrated auth | desktop | not-started | niche | P4 | L | yes | yes | no | no | no |
| 54 | NTLM authentication — Windows-native challenge/response | desktop | not-started | niche | P4 | L | yes | yes | partial | no | no |
| 55 | OAuth 2.0 device flow — device authorisation grant | all | not-started | experimental | P4 | M | no | no | no | no | no |
| 56 | WebAuthn integration — credential management for authentication | all | not-started | widespread | P4 | L | yes | yes | yes | no | no |

---

## 2.7 Preloading, Prefetching & Speculation

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 57 | Preconnect (rel=preconnect) — early TCP+TLS handshake hint | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 58 | Preload (rel=preload) — fetch resource for current navigation | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 59 | Prefetch (rel=prefetch) — fetch resource for likely future navigation | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 60 | Speculation Rules (document rules) — JSON-driven prerender/prefetch declarations | all | not-started | experimental | P4 | L | yes | experimental | no | no | no |
| 61 | Prerender (rel=prerender / Speculation Rules) — pre-render full page in hidden context | all | not-started | experimental | P4 | XL | yes | no | no | no | no |
| 62 | Priority Hints (fetchpriority attribute) — influence resource fetch priority | all | not-started | widespread | P4 | S | yes | yes | yes | no | no |

---

## 2.8 Web Push

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 63 | Push API — subscribe to push messages from a server | all | not-started | widespread | P4 | L | yes | yes | yes | no | no |
| 64 | Notification API — display system notifications | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 65 | Web Push protocol — message delivery to push services | all | not-started | widespread | P4 | L | yes | yes | yes | no | no |
| 66 | VAPID — Voluntary Application Server Identification for Web Push | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 67 | Web Push message encryption — ECDH + HKDF content encryption | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |

---

## 2.9 Transport-Relevant Security Policies

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 68 | CORS — cross-origin fetch with preflight and credentialed requests | all | not-started | ubiquitous | P4 | L | yes | yes | yes | yes | yes |
| 69 | CORP — restrict cross-origin no-cors loads | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 70 | COEP — require CORP/CORS on subresources | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 71 | CSP connect-src — restrict network request targets | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 72 | Mixed content blocking — block HTTP subresources on HTTPS pages | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |

---

## 2.10 Service Workers (Transport Subset)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 73 | Service Worker fetch interception — intercept and respond to network requests | all | not-started | ubiquitous | P4 | L | yes | yes | yes | yes | yes |
| 74 | Service Worker offline mode — serve cached responses when network unavailable | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 75 | Service Worker background sync — deferred network requests when connectivity resumes | all | not-started | mixed | P4 | M | yes | experimental | no | no | no |
| 76 | Service Worker navigation preload — parallel SW activation + network fetch | all | not-started | widespread | P4 | M | yes | yes | yes | no | no |

---

## 2.11 Proxy Support

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 77 | HTTP CONNECT tunneling — proxy tunnel for HTTPS through HTTP proxy | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 78 | PAC (Proxy Auto-Config) files — JavaScript-based proxy selection | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 79 | WPAD (Web Proxy Auto-Discovery) — automatic PAC file discovery via DNS/DHCP | desktop | not-started | mixed | P4 | M | yes | yes | partial | no | no |
| 80 | SOCKS4 / SOCKS5 proxy — TCP/UDP proxy protocol | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 81 | HTTPS proxy (CONNECT over TLS) — proxy connection wrapped in TLS | all | not-started | mixed | P4 | M | yes | yes | no | no | no |
| 82 | Proxy authentication — Basic, NTLM, Negotiate to proxy server | desktop | not-started | ubiquitous | P4 | M | yes | yes | partial | no | no |

---

## 2.12 Content Encoding (HTTP)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 83 | Brotli — compression for static and dynamic content | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 84 | Zstandard — next-generation compression for HTTP content | all | not-started | mixed | P4 | S | yes | yes | yes | no | no |
| 85 | gzip — legacy deflate compression | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 86 | deflate — raw deflate without gzip wrapper | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 87 | Content-Encoding negotiation (Accept-Encoding header) — client-driven encoding selection | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |

---

## 2.13 Non-HTTP Resource URLs

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 88 | data: URLs — inline base64-encoded resources | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 89 | blob: URLs — opaque origin URLs for in-memory Blob objects | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 90 | Object URLs (createObjectURL / revokeObjectURL) — generate blob: URL for File/Blob/MediaSource | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |
| 91 | about: URLs (about:blank, about:srcdoc) — browser-internal resource identifiers | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 92 | javascript: URLs — execute JS from address bar / href (legacy) | all | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | yes |
| 93 | file: URLs — local filesystem access | desktop | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |

---

## 2.14 Subresource Integrity (Transport-Relevant Subset)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 94 | Subresource Integrity (SRI) — integrity attribute on `<script>` and `<link>` | all | not-started | ubiquitous | P4 | M | yes | yes | yes | yes | yes |

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
