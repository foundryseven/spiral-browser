# Chunk 4 — Storage & State

> **§A (Web Platform APIs & Runtime)** will be added by chunk 6 to this
> file. If the combined file exceeds 600 lines, §B splits into a separate
> `06-storage-state.md` and the index links accordingly.

---

## §B — Storage & State

## Scope

**In:** Cookies, Web Storage, IndexedDB, Origin Private File System
(OPFS), Cache Storage API, storage quota management, AppCache
(deprecated), Clear-Site-Data header, storage partitioning (double-keyed,
first-party isolation, CHIPS, TCP), Storage Bucket API, Background Fetch,
Background Sync / Periodic Background Sync, Broadcast Channel API,
SharedWorker, Lock Manager API, Credential Management API, Navigation /
History API and session history entries.

**Out:** Cookie security policy (SameSite enforcement, `__Secure-` prefix
enforcement) — chunk 3. Service Worker fetch interception / offline mode —
chunk 2 / chunk 6. Media storage (codec cache, DRM key storage) — chunk
5. IndexedDB schema design patterns / developer guidance — chunk 8.
Extension storage (`chrome.storage.local`) — chunk 10. Cookies & privacy
(fingerprinting, tracking) — chunk 3.

---

## Methodology for this chunk

Rows are informed by: WHATWG Storage Living Standard, HTML Living Standard
§Web Storage / §Session history, W3C IndexedDB 2.0 Recommendation, W3C
File API, W3C File System Access / OPFS spec, W3C Service Workers (Cache
Storage), W3C Clear-Site-Data, W3C Storage Bucket API, W3C Background
Fetch / Background Sync / Periodic Background Sync, WICG Storage Access
API, WHATWG Credential Management, WHATWG Navigation API, RFC 6265
(Cookies), IETF CHIPS draft (Cookies Having Independent Partitioned State),
MDN Baseline tables, Can I Use support matrices, Chrome Platform Status,
WebKit blog, Mozilla Hacks. Snapshot date: 2026-06-16.

---

## Inventory

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|------------------|--------------------|--------------|------------|--------------|---------|
| 1 | `Set-Cookie` header parsing | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: partial · Flow: unknown | [RFC 6265 §4.1](https://httpwg.org/specs/rfc6265.html#sane-set-cookie) |
| 2 | `document.cookie` getter | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: partial · Flow: unknown | [HTML §dom-document-cookie](https://html.spec.whatwg.org/multipage/dom.html#dom-document-cookie) |
| 3 | `document.cookie` setter | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: partial · Flow: unknown | [HTML §dom-document-cookie](https://html.spec.whatwg.org/multipage/dom.html#dom-document-cookie) |
| 4 | `Cookie` request header (cookie jar emit) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [RFC 6265 §4.2](https://httpwg.org/specs/rfc6265.html#sane-cookie) |
| 5 | Cookie jar (per-process persistent store) | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [RFC 6265](https://httpwg.org/specs/rfc6265.html), [Chrome Platform Status: CHIPS](https://chromestatus.com/feature/5179189105782784) |
| 6 | Cookie expiry (`Max-Age` / `Expires`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: partial · Flow: unknown | [RFC 6265 §4.1.2](https://httpwg.org/specs/rfc6265.html#sane-expires) |
| 7 | Cookie scope (domain / path matching) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: partial · Flow: unknown | [RFC 6265 §5.1–§5.4](https://httpwg.org/specs/rfc6265.html#sane-domain) |
| 8 | `SameSite` attribute (Lax / Strict / None) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [RFC 6265bis §5.3.7](https://httpwg.org/http-extensions/draft-ietf-httpbis-rfc6265bis.html#samesite) |
| 9 | `Partitioned` attribute (CHIPS) | desktop+mobile | not-started | widespread | P4 | M | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [IETF draft-ietf-httpbis-CHIPS](https://datatracker.ietf.org/doc/draft-ietf-httpbis-chips/) |
| 10 | `Secure` cookie attribute | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [RFC 6265 §4.1.2.5](https://httpwg.org/specs/rfc6265.html#sane-secure) |
| 11 | `HttpOnly` cookie attribute | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [RFC 6265 §4.1.2.6](https://httpwg.org/specs/rfc6265.html#sane-httponly) |
| 12 | `__Secure-` / `__Host-` name prefixes | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [RFC 6265bis §5.4](https://httpwg.org/http-extensions/draft-ietf-httpbis-rfc6265bis.html#name-prefixes) |
| 13 | First-party vs third-party cookie blocking | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [Chrome Platform Status](https://chromestatus.com/feature/5765269566431232), [WebKit Tracking Prevention](https://webkit.org/tracking-prevention/) |
| 14 | Third-party storage partitioning (TCP) | desktop+mobile | not-started | widespread | P4 | L | Chromium: yes · Gecko: yes (dynamic partitioning) · WebKit: no (uses expiry cap instead) · Servo: no · Ladybird: no · Flow: unknown | [Mozilla Hacks: TCP](https://hacks.mozilla.org/total-cookie-protection-in-firefox-86/), [Chrome CHIPS](https://developer.chrome.com/blog/cookie-chips) |
| 15 | Cross-site tracking mitigation policy (7-day expiry cap) | desktop+mobile | not-started | niche | P4 | M | Chromium: no · Gecko: no · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [WebKit ITP](https://webkit.org/tracking-prevention-policy/) |
| 16 | Cookie size / count limits per domain | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: 4096 B / 180 per domain · Gecko: 4097 B / 1000 · WebKit: 4096 B / 180 · Servo: no · Ladybird: no · Flow: unknown | [MDN: HTTP Cookies](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies) |
| 17 | Clear-Site-Data: `cookies` directive | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C Clear-Site-Data §3.1](https://w3c.github.io/webappsec-clear-site-data/#cookies) |
| 18 | `localStorage` (synchronous, persistent, 5–10 MB) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: partial · Flow: unknown | [HTML §webstorage](https://html.spec.whatwg.org/multipage/webstorage.html) |
| 19 | `sessionStorage` (per-tab, per-origin, same-navigation-entry) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: partial · Flow: unknown | [HTML §webstorage](https://html.spec.whatwg.org/multipage/webstorage.html) |
| 20 | `Storage` event (cross-tab same-origin broadcast) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [HTML §webstorage](https://html.spec.whatwg.org/multipage/webstorage.html#the-storageevent-interface) |
| 21 | `StorageManager.persist()` / `persisted()` | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §persistence](https://storage.spec.whatwg.org/#persistence) |
| 22 | Clear-Site-Data: `storage` directive | desktop+mobile | not-started | mixed | P4 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C Clear-Site-Data §3.3](https://w3c.github.io/webappsec-clear-site-data/#storage) |
| 23 | IndexedDB database opening / versioning (`open` / `onupgradeneeded`) | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §4](https://www.w3.org/TR/IndexedDB2/#database-connection) |
| 24 | IndexedDB object stores (key path, autoIncrement) | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §6](https://www.w3.org/TR/IndexedDB2/#object-store-construct) |
| 25 | IndexedDB transactions (readonly / readwrite / versionchange) | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §7](https://www.w3.org/TR/IndexedDB2/#transaction) |
| 26 | IndexedDB indexes (unique / multiEntry) | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §8](https://www.w3.org/TR/IndexedDB2/#index-construct) |
| 27 | IndexedDB cursors (key range, direction, iteration) | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §8.4](https://www.w3.org/TR/IndexedDB2/#cursor) |
| 28 | IndexedDB binary keys (ArrayBuffer / TypedArray) | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §3.1.1](https://www.w3.org/TR/IndexedDB2/#key-type) |
| 29 | IndexedDB large value storage (structured clone) | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [HTML §safe-passing-of-structured-data](https://html.spec.whatwg.org/multipage/structured-data.html) |
| 30 | `IDBFactory.databases()` enumeration | desktop+mobile+embedded | not-started | widespread | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §4.1](https://www.w3.org/TR/IndexedDB2/#dom-idbfactory-databases) |
| 31 | `IDBFactory.deleteDatabase()` | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §4.4](https://www.w3.org/TR/IndexedDB2/#dom-idbfactory-deletedatabase) |
| 32 | IndexedDB per-origin quota / storage limits | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | M | Chromium: ~60% disk · Gecko: 50% disk · WebKit: ~1 GB initially · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §usage](https://storage.spec.whatwg.org/#usage-and-quota), [MDN: Storage quotas](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas) |
| 33 | IndexedDB key generator algorithm | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §3.4](https://www.w3.org/TR/IndexedDB2/#key-generator-construct) |
| 34 | IndexedDB compound indexes | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §3.1.2](https://www.w3.org/TR/IndexedDB2/#compound-construct) |
| 35 | IndexedDB `onversionchange` / schema upgrade notification | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C IndexedDB 2.0 §4.5](https://www.w3.org/TR/IndexedDB2/#dom-idbdatabase-onversionchange) |
| 36 | `navigator.storage.getDirectory()` (OPFS root) | desktop+mobile | not-started | widespread | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [WHATWG File System §opfs](https://fs.spec.whatwg.org/#sandboxed-filesystem) |
| 37 | OPFS synchronous file access (`createSyncAccessHandle` in Workers) | desktop+mobile | not-started | widespread | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [WHATWG File System §sync-access-handle](https://fs.spec.whatwg.org/#api-filesystemsyncaccesshandle) |
| 38 | OPFS async file access (`getFile` / `createWritable`) | desktop+mobile | not-started | widespread | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [WHATWG File System §file-handle](https://fs.spec.whatwg.org/#api-filesystemfilehandle) |
| 39 | OPFS directory operations (`entries` / `keys` / `values` / `resolve`) | desktop+mobile | not-started | widespread | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [WHATWG File System §directory-handle](https://fs.spec.whatwg.org/#api-filesystemdirectoryhandle) |
| 40 | OPFS per-origin quota management | desktop+mobile | not-started | widespread | P4 / P5 | M | Chromium: origin quota · Gecko: origin quota · WebKit: origin quota · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §usage](https://storage.spec.whatwg.org/#usage-and-quota) |
| 41 | OPFS naming constraints (reserved chars, length limits) | desktop+mobile | not-started | widespread | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [WHATWG File System §naming](https://fs.spec.whatwg.org/#sandboxed-filesystem) |
| 42 | Cache Storage API: `CacheStorage.open()` | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Service Workers §cache-storage](https://w3c.github.io/ServiceWorker/#cache-storage) |
| 43 | Cache Storage API: `Cache.match()` / `matchAll()` | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Service Workers §cache-match](https://w3c.github.io/ServiceWorker/#cache-match) |
| 44 | Cache Storage API: `Cache.put()` / `add()` / `addAll()` | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Service Workers §cache-put](https://w3c.github.io/ServiceWorker/#cache-put) |
| 45 | Cache Storage API: `Cache.delete()` | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Service Workers §cache-delete](https://w3c.github.io/ServiceWorker/#cache-delete) |
| 46 | Cache Storage API: `CacheStorage.keys()` | desktop+mobile+embedded | not-started | ubiquitous | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Service Workers §cache-storage-keys](https://w3c.github.io/ServiceWorker/#cache-storage-keys) |
| 47 | Cache Storage API: `CacheStorage.has()` | desktop+mobile+embedded | not-started | widespread | P4 / P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Service Workers §cache-storage-has](https://w3c.github.io/ServiceWorker/#cache-storage-has) |
| 48 | `navigator.storage.estimate()` (usage / quota) | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §estimate](https://storage.spec.whatwg.org/#dom-storagemanager-estimate) |
| 49 | `navigator.storage.persist()` (request persistent storage) | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §persist](https://storage.spec.whatwg.org/#dom-storagemanager-persist) |
| 50 | `navigator.storage.persisted()` (query persistent state) | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §persisted](https://storage.spec.whatwg.org/#dom-storagemanager-persisted) |
| 51 | Eviction policies (LRU-based origin eviction) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes (LRU) · Gecko: yes (LRU) · WebKit: yes (LRU + disk-pressure) · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §eviction](https://storage.spec.whatwg.org/#eviction) |
| 52 | Per-origin quota limits (storage pressure heuristics) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: ~60% disk · Gecko: 50% disk · WebKit: ~1 GB then prompts · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §usage](https://storage.spec.whatwg.org/#usage-and-quota) |
| 53 | Quota pressure / eviction notifications | desktop+mobile | not-started | mixed | P4 | S | Chromium: yes (`StoragePressureEvent`) · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [Chrome Platform Status](https://chromestatus.com/feature/5765526349897728) |
| 54 | `StorageManager` interface (unified estimate/persist/persisted) | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: partial (estimate only) · Servo: no · Ladybird: no · Flow: unknown | [WHATWG Storage §storagemanager](https://storage.spec.whatwg.org/#storagemanager) |
| 55 | Application Cache (manifest attribute) | desktop+mobile+embedded | not-started | legacy | P4 | M | Chromium: removed · Gecko: removed · WebKit: removed · Servo: no · Ladybird: no · Flow: unknown | [HTML §appcache](https://html.spec.whatwg.org/multipage/offline.html) (removed from spec) |
| 56 | AppCache deprecation / removal status | desktop+mobile+embedded | not-started | legacy | — | — | Chromium: removed 2024 · Gecko: removed 2023 · WebKit: removed 2024 · Servo: never implemented · Ladybird: no · Flow: unknown | [Chrome Platform Status](https://chromestatus.com/feature/5700519340679168) |
| 57 | Clear-Site-Data: `cache` directive | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C Clear-Site-Data §3.2](https://w3c.github.io/webappsec-clear-site-data/#cache) |
| 58 | Clear-Site-Data: `executionContexts` directive | desktop+mobile | not-started | mixed | P4 | S | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C Clear-Site-Data §3.4](https://w3c.github.io/webappsec-clear-site-data/#executionContexts) |
| 59 | Clear-Site-Data: wildcard `*` (all directives) | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C Clear-Site-Data §3.5](https://w3c.github.io/webappsec-clear-site-data/#grammardef-) |
| 60 | Double-keyed cache partitioning (top-level site + frame site) | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | Chromium: yes · Gecko: yes (TCP) · WebKit: yes (ITP) · Servo: no · Ladybird: no · Flow: unknown | [Chromium partitioning](https://developers.google.com/privacy-sandbox/3pcd/partitioned-state), [Mozilla Hacks: TCP](https://hacks.mozilla.org/total-cookie-protection-in-firefox-86/) |
| 61 | DNS cache partitioning (network-state keyed by top-level + frame origin) | desktop+mobile | not-started | widespread | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [Chromium partitioning](https://developers.google.com/privacy-sandbox/3pcd/partitioned-state) |
| 62 | HSTS cache partitioning | desktop+mobile | not-started | widespread | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [Chromium partitioning](https://developers.google.com/privacy-sandbox/3pcd/partitioned-state) |
| 63 | TLS session cache partitioning | desktop+mobile | not-started | widespread | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [Chromium partitioning](https://developers.google.com/privacy-sandbox/3pcd/partitioned-state) |
| 64 | First-party isolation mode (all state keyed by first-party site) | desktop+mobile | not-started | niche | P4 | L | Chromium: no (uses CHIPS instead) · Gecko: yes (`privacy.firstparty.isolate`) · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [Mozilla: FPI](https://wiki.mozilla.org/Privacy/First_Party_Isolation) |
| 65 | Per-frame storage partitioning (frame origin isolation) | desktop+mobile | not-started | widespread | P4 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [Chromium partitioning](https://developers.google.com/privacy-sandbox/3pcd/partitioned-state) |
| 66 | Storage Access API (`requestStorageAccess` / `hasStorageAccess`) | desktop+mobile | not-started | widespread | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Storage Access API](https://privacycg.github.io/storage-access/) |
| 67 | `navigator.storageBuckets.open()` (Storage Bucket API) | desktop+mobile | not-started | experimental | P4 | M | Chromium: yes (origin trial → stable) · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WICG Storage Buckets](https://wicg.github.io/storage-buckets/) |
| 68 | Named storage buckets (per-bucket persistence / eviction) | desktop+mobile | not-started | experimental | P4 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WICG Storage Buckets](https://wicg.github.io/storage-buckets/) |
| 69 | Storage bucket expiration / durability hints | desktop+mobile | not-started | experimental | P4 | S | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WICG Storage Buckets](https://wicg.github.io/storage-buckets/) |
| 70 | Storage bucket quota allocation | desktop+mobile | not-started | experimental | P4 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WICG Storage Buckets](https://wicg.github.io/storage-buckets/) |
| 71 | Background Fetch API (persistent downloads surviving tab close) | desktop+mobile | not-started | mixed | P5 | L | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C Background Fetch](https://wicg.github.io/background-fetch/) |
| 72 | Background Sync API (`SyncManager.register()`) | desktop+mobile | not-started | mixed | P5 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C Background Sync](https://wicg.github.io/background-sync/spec/) |
| 73 | Periodic Background Sync API (`PeriodicSyncManager.register()`) | desktop+mobile | not-started | niche | P5 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [W3C Periodic Background Sync](https://wicg.github.io/periodic-background-sync/) |
| 74 | Broadcast Channel API (cross-tab same-origin messaging) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [HTML §broadcastchannel](https://html.spec.whatwg.org/multipage/web-messaging.html#broadcasting-to-other-browsing-contexts) |
| 75 | SharedWorker (shared state across tabs, same-origin) | desktop+mobile+embedded | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [HTML §shared-workers](https://html.spec.whatwg.org/multipage/workers.html#shared-workers) |
| 76 | Lock Manager API (`navigator.locks.request()` / `query()`) | desktop+mobile | not-started | widespread | P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Web Locks](https://w3c.github.io/web-locks/) |
| 77 | Credential Management API: `navigator.credentials.get()` | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Credential Management](https://w3c.github.io/webappsec-credential-management/) |
| 78 | Credential Management API: `navigator.credentials.create()` | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Credential Management](https://w3c.github.io/webappsec-credential-management/) |
| 79 | Credential Management API: `navigator.credentials.preventSilentAccess()` | desktop+mobile | not-started | widespread | P5 | S | Chromium: yes · Gecko: yes · WebKit: partial · Servo: no · Ladybird: no · Flow: unknown | [W3C Credential Management §preventsilentaccess](https://w3c.github.io/webappsec-credential-management/#dom-credentialscontainer-preventsilentaccess) |
| 80 | `PasswordCredential` (password storage / retrieval) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Credential Management](https://w3c.github.io/webappsec-credential-management/#passwordcredential) |
| 81 | `FederatedCredential` (federated identity / IdP credentials) | desktop+mobile+embedded | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: partial · Servo: no · Ladybird: no · Flow: unknown | [W3C Credential Management](https://w3c.github.io/webappsec-credential-management/#federatedcredential) |
| 82 | History API: `history.pushState()` / `replaceState()` / `state` | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [HTML §history](https://html.spec.whatwg.org/multipage/history.html) |
| 83 | `popstate` / `hashchange` events | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [HTML §event-popstate](https://html.spec.whatwg.org/multipage/browsing-the-web.html#event-popstate) |
| 84 | Session history entries (document state per navigation) | desktop+mobile+embedded | not-started | ubiquitous | P3 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [HTML §session-history](https://html.spec.whatwg.org/multipage/history.html#session-history) |
| 85 | `sessionStorage` per navigation entry (survives reload, not cross-origin) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: partial · Flow: unknown | [HTML §webstorage](https://html.spec.whatwg.org/multipage/webstorage.html) |
| 86 | Navigation API (`window.navigation` / `navigate` event) | desktop+mobile | not-started | widespread | P6 | L | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WICG Navigation API](https://wicg.github.io/navigation-api/) |
| 87 | Navigation API: `intercept()` / `scroll()` / `transition` | desktop+mobile | not-started | niche | P6 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: unknown | [WICG Navigation API](https://wicg.github.io/navigation-api/) |
| 88 | Service Worker `Clients.matchAll()` (cross-tab state coordination) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Service Workers §clients-matchall](https://w3c.github.io/ServiceWorker/#clients-matchall) |
| 89 | Service Worker `Clients.get()` (single client lookup) | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: unknown | [W3C Service Workers §clients-get](https://w3c.github.io/ServiceWorker/#clients-get) |

---

## Cross-refs to `specs/GAP_ANALYSIS.md`

| GAP_ANALYSIS row | Section | Line |
|---|---|---|
| Cookie jar | §2.3 Storage | 189 |
| LocalStorage | §2.3 Storage | 190 |
| SessionStorage | §2.3 Storage | 191 |
| IndexedDB | §2.3 Storage | 192 |
| OPFS (Origin Private File System) | §2.3 Storage | 193 |
| CacheStorage (Service Workers) | §2.3 Storage | 194 |
| Quota management / eviction | §2.3 Storage | 195 |
| Storage partitioning (Bet 1) | §2.3 Storage | 196 |
| `spiral-storage` crate | §2.3 Storage | 198 |
| Cookie partitioning (CHIPS / Storage Access API) | §4.1 Security/Privacy | 249 |
| Secure cookie flags (`Secure`, `HttpOnly`, `SameSite`) | §4.1 Security/Privacy | 248 |
| Settings panel; cookie jar; form submission | ROADMAP Month 28-30 | — |
| IndexedDB | ROADMAP Month 49-54 | — |

---

## Open questions for the user

1. **Cookie jar priority:** The ROADMAP places the cookie jar in Phase 4
   (Month 28-30). Cookies are prerequisite for any real HTTP interaction.
   Should they be pulled forward into Phase 3 (Months 21-22, when
   networking starts) as a skeleton with just `SameSite=Lax` +
   `HttpOnly` + `Secure` support, or is the Phase 4 placement intentional?

2. **IndexedDB vs localStorage sequencing:** The ROADMAP places IndexedDB
   in Phase 5 (Month 49-54) but does not mention `localStorage` /
   `sessionStorage` at all. Most web apps depend on one of these. Should
   Web Storage (`localStorage` / `sessionStorage`) be considered a Phase
   4 deliverable alongside the cookie jar, with IndexedDB deferred to
   Phase 5?

3. **Storage partitioning scope:** The GAP_ANALYSIS notes storage
   partitioning as "M30+" under Bet 1. Should this include double-keyed
   network state (DNS cache, HSTS cache, TLS session cache) from day one,
   or is cookie+storage partitioning the minimum viable implementation?

4. **Storage Bucket API:** This is Chromium-only and experimental. Should
   it be tracked as a future capability (accepting the Chromium lock-in)
   or deprioritised until a second engine ships it?

5. **Navigation API:** Chromium-only (`window.navigation`). It is the
   intended replacement for the History API but has no Gecko/WebKit
   commitment. Is Spiral interested in early adoption or should it wait
   for wider implementation?

6. **Background Fetch / Sync:** Chromium-only (Background Sync) or
   mixed (Background Fetch). These require Service Worker infrastructure.
   Since Service Workers are not scheduled until Phase 5 (Month 49-54),
   should these APIs be deferred to Phase 6 or bundled with Service
   Workers in Phase 5?

7. **`spiral-storage` crate:** The GAP_ANALYSIS flags that `spiral-storage`
   does not exist yet and "should exist." Is the intention a unified crate
   covering cookies, Web Storage, IndexedDB, OPFS, and quota management,
   or separate per-mechanism crates?
