# Chunk 12 — Competitive Matrix: Storage & State

**File:** `02-competitive-matrix-storage.md`
**Date:** 2026-06-16
**Source:** `06-web-platform-apis.md` (section B — Storage & State)
**Row count:** 89

---

## Cookies

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 1 | Set-Cookie header parsing | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | partial |
| 2 | document.cookie getter | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | partial |
| 3 | document.cookie setter | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | partial |
| 4 | Cookie request header (cookie jar emit) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 5 | Cookie jar (per-process persistent store) | all | not-started | ubiquitous | P4 | L | yes | yes | yes | no | no |
| 6 | Cookie expiry (Max-Age / Expires) | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | partial |
| 7 | Cookie scope (domain / path matching) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | partial |
| 8 | SameSite attribute (Lax / Strict / None) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 9 | Partitioned attribute (CHIPS) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | no | no | no |
| 10 | Secure cookie attribute | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 11 | HttpOnly cookie attribute | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 12 | __Secure- / __Host- name prefixes | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 13 | First-party vs third-party cookie blocking | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 14 | Third-party storage partitioning (TCP) | desktop+mobile | not-started | widespread | P4 | L | yes | yes | no | no | no |
| 15 | Cross-site tracking mitigation policy (7-day expiry cap) | desktop+mobile | not-started | niche | P4 | M | no | no | yes | no | no |
| 16 | Cookie size / count limits per domain | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 17 | Clear-Site-Data: cookies directive | desktop+mobile | not-started | widespread | P4 | S | yes | yes | no | no | no |

---

## Web Storage

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 18 | localStorage (synchronous, persistent, 5–10 MB) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | partial |
| 19 | sessionStorage (per-tab, per-origin, same-navigation-entry) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | partial |
| 20 | Storage event (cross-tab same-origin broadcast) | all | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no |
| 21 | StorageManager.persist() / persisted() | desktop+mobile | not-started | widespread | P4 | S | yes | yes | no | no | no |
| 22 | Clear-Site-Data: storage directive | desktop+mobile | not-started | mixed | P4 | S | yes | yes | no | no | no |
| 23 | sessionStorage per navigation entry (survives reload, not cross-origin) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | partial |

---

## IndexedDB

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 24 | IndexedDB database opening / versioning (open / onupgradeneeded) | all | not-started | ubiquitous | P4/P5 | L | yes | yes | yes | no | no |
| 25 | IndexedDB object stores (key path, autoIncrement) | all | not-started | ubiquitous | P4/P5 | M | yes | yes | yes | no | no |
| 26 | IndexedDB transactions (readonly / readwrite / versionchange) | all | not-started | ubiquitous | P4/P5 | L | yes | yes | yes | no | no |
| 27 | IndexedDB indexes (unique / multiEntry) | all | not-started | ubiquitous | P4/P5 | M | yes | yes | yes | no | no |
| 28 | IndexedDB cursors (key range, direction, iteration) | all | not-started | ubiquitous | P4/P5 | M | yes | yes | yes | no | no |
| 29 | IndexedDB binary keys (ArrayBuffer / TypedArray) | all | not-started | ubiquitous | P4/P5 | S | yes | yes | yes | no | no |
| 30 | IndexedDB large value storage (structured clone) | all | not-started | ubiquitous | P4/P5 | M | yes | yes | yes | no | no |
| 31 | IDBFactory.databases() enumeration | all | not-started | widespread | P4/P5 | S | yes | yes | no | no | no |
| 32 | IDBFactory.deleteDatabase() | all | not-started | ubiquitous | P4/P5 | S | yes | yes | yes | no | no |
| 33 | IndexedDB per-origin quota / storage limits | all | not-started | ubiquitous | P4/P5 | M | yes | yes | yes | no | no |
| 34 | IndexedDB key generator algorithm | all | not-started | ubiquitous | P4/P5 | S | yes | yes | yes | no | no |
| 35 | IndexedDB compound indexes | all | not-started | ubiquitous | P4/P5 | S | yes | yes | yes | no | no |
| 36 | IndexedDB onversionchange / schema upgrade notification | all | not-started | ubiquitous | P4/P5 | S | yes | yes | yes | no | no |

---

## Origin Private File System (OPFS)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 37 | navigator.storage.getDirectory() (OPFS root) | desktop+mobile | not-started | widespread | P4/P5 | M | yes | yes | yes | no | no |
| 38 | OPFS synchronous file access (createSyncAccessHandle in Workers) | desktop+mobile | not-started | widespread | P4/P5 | M | yes | yes | yes | no | no |
| 39 | OPFS async file access (getFile / createWritable) | desktop+mobile | not-started | widespread | P4/P5 | M | yes | yes | yes | no | no |
| 40 | OPFS directory operations (entries / keys / values / resolve) | desktop+mobile | not-started | widespread | P4/P5 | S | yes | yes | yes | no | no |
| 41 | OPFS per-origin quota management | desktop+mobile | not-started | widespread | P4/P5 | M | yes | yes | yes | no | no |
| 42 | OPFS naming constraints (reserved chars, length limits) | desktop+mobile | not-started | widespread | P4/P5 | S | yes | yes | yes | no | no |

---

## Cache Storage API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 43 | CacheStorage.open() | all | not-started | ubiquitous | P4/P5 | M | yes | yes | yes | no | no |
| 44 | Cache.match() / matchAll() | all | not-started | ubiquitous | P4/P5 | M | yes | yes | yes | no | no |
| 45 | Cache.put() / add() / addAll() | all | not-started | ubiquitous | P4/P5 | M | yes | yes | yes | no | no |
| 46 | Cache.delete() | all | not-started | ubiquitous | P4/P5 | S | yes | yes | yes | no | no |
| 47 | CacheStorage.keys() | all | not-started | ubiquitous | P4/P5 | S | yes | yes | yes | no | no |
| 48 | CacheStorage.has() | all | not-started | widespread | P4/P5 | S | yes | yes | yes | no | no |

---

## Storage Quota & Eviction

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 49 | navigator.storage.estimate() (usage / quota) | desktop+mobile | not-started | widespread | P4 | S | yes | yes | yes | no | no |
| 50 | navigator.storage.persist() (request persistent storage) | desktop+mobile | not-started | widespread | P4 | S | yes | yes | no | no | no |
| 51 | navigator.storage.persisted() (query persistent state) | desktop+mobile | not-started | widespread | P4 | S | yes | yes | no | no | no |
| 52 | Eviction policies (LRU-based origin eviction) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 53 | Per-origin quota limits (storage pressure heuristics) | all | not-started | ubiquitous | P4 | M | yes | yes | yes | no | no |
| 54 | Quota pressure / eviction notifications | desktop+mobile | not-started | mixed | P4 | S | yes | no | no | no | no |
| 55 | StorageManager interface (unified estimate/persist/persisted) | desktop+mobile | not-started | widespread | P4 | S | yes | yes | partial | no | no |

---

## Application Cache (Legacy)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 56 | Application Cache (manifest attribute) | all | not-started | legacy | P4 | M | no | no | no | no | no |
| 57 | AppCache deprecation / removal status | all | not-started | legacy | — | — | no | no | no | no | no |

---

## Clear-Site-Data (Storage Dimension)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 58 | Clear-Site-Data: cache directive | desktop+mobile | not-started | widespread | P4 | S | yes | yes | no | no | no |
| 59 | Clear-Site-Data: executionContexts directive | desktop+mobile | not-started | mixed | P4 | S | yes | no | no | no | no |
| 60 | Clear-Site-Data: wildcard * (all directives) | desktop+mobile | not-started | widespread | P4 | S | yes | yes | no | no | no |

---

## Storage Partitioning

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 61 | Double-keyed cache partitioning (top-level site + frame site) | all | not-started | ubiquitous | P4 | L | yes | yes | yes | no | no |
| 62 | DNS cache partitioning (network-state keyed by top-level + frame origin) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 63 | HSTS cache partitioning | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 64 | TLS session cache partitioning | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |
| 65 | First-party isolation mode (all state keyed by first-party site) | desktop+mobile | not-started | niche | P4 | L | no | yes | no | no | no |
| 66 | Per-frame storage partitioning (frame origin isolation) | desktop+mobile | not-started | widespread | P4 | L | yes | yes | yes | no | no |
| 67 | Storage Access API (requestStorageAccess / hasStorageAccess) | desktop+mobile | not-started | widespread | P4 | M | yes | yes | yes | no | no |

---

## Storage Bucket API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 68 | navigator.storageBuckets.open() | desktop+mobile | not-started | experimental | P4 | M | yes | no | no | no | no |
| 69 | Named storage buckets (per-bucket persistence / eviction) | desktop+mobile | not-started | experimental | P4 | M | yes | no | no | no | no |
| 70 | Storage bucket expiration / durability hints | desktop+mobile | not-started | experimental | P4 | S | yes | no | no | no | no |
| 71 | Storage bucket quota allocation | desktop+mobile | not-started | experimental | P4 | M | yes | no | no | no | no |

---

## Background Sync & Fetch

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 72 | Background Fetch API (persistent downloads surviving tab close) | desktop+mobile | not-started | mixed | P5 | L | yes | no | no | no | no |
| 73 | Background Sync API (SyncManager.register()) | desktop+mobile | not-started | mixed | P5 | M | yes | no | no | no | no |
| 74 | Periodic Background Sync API (PeriodicSyncManager.register()) | desktop+mobile | not-started | niche | P5 | M | yes | no | no | no | no |

---

## Cross-Tab Communication & Coordination

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 75 | Broadcast Channel API (cross-tab same-origin messaging) | all | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 76 | SharedWorker (shared state across tabs, same-origin) | all | not-started | widespread | P5 | M | yes | yes | yes | no | no |
| 77 | Lock Manager API (navigator.locks.request() / query()) | desktop+mobile | not-started | widespread | P5 | S | yes | yes | yes | no | no |

---

## Credential Management API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 78 | navigator.credentials.get() | all | not-started | ubiquitous | P5 | L | yes | yes | yes | no | no |
| 79 | navigator.credentials.create() | all | not-started | ubiquitous | P5 | L | yes | yes | yes | no | no |
| 80 | navigator.credentials.preventSilentAccess() | desktop+mobile | not-started | widespread | P5 | S | yes | yes | partial | no | no |
| 81 | PasswordCredential (password storage / retrieval) | all | not-started | ubiquitous | P5 | M | yes | yes | yes | no | no |
| 82 | FederatedCredential (federated identity / IdP credentials) | all | not-started | widespread | P5 | M | yes | yes | partial | no | no |

---

## History & Navigation API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 83 | history.pushState() / replaceState() / state | all | not-started | ubiquitous | P3 | M | yes | yes | yes | no | no |
| 84 | popstate / hashchange events | all | not-started | ubiquitous | P3 | S | yes | yes | yes | no | no |
| 85 | Session history entries (document state per navigation) | all | not-started | ubiquitous | P3 | L | yes | yes | yes | no | no |
| 86 | Navigation API (window.navigation / navigate event) | desktop+mobile | not-started | widespread | P6 | L | yes | no | no | no | no |
| 87 | Navigation API: intercept() / scroll() / transition | desktop+mobile | not-started | niche | P6 | M | yes | no | no | no | no |

---

## Service Worker Client APIs (State Coordination)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|------------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|
| 88 | Service Worker Clients.matchAll() (cross-tab state coordination) | all | not-started | ubiquitous | P5 | M | yes | yes | yes | no | no |
| 89 | Service Worker Clients.get() (single client lookup) | all | not-started | ubiquitous | P5 | S | yes | yes | yes | no | no |

---

## Open questions for the user

- **Cookie jar priority:** The ROADMAP places the cookie jar in Phase 4 (Month 28-30). Cookies are prerequisite for any real HTTP interaction. Should they be pulled forward into Phase 3 as a skeleton with just SameSite=Lax + HttpOnly + Secure support?
- **IndexedDB vs localStorage sequencing:** The ROADMAP places IndexedDB in Phase 5 but does not mention localStorage / sessionStorage. Should Web Storage be a Phase 4 deliverable alongside the cookie jar?
- **Storage partitioning scope:** Should this include double-keyed network state (DNS cache, HSTS cache, TLS session cache) from day one, or is cookie+storage partitioning the minimum viable implementation?
- **Storage Bucket API:** Chromium-only and experimental. Should it be tracked as a future capability or deprioritised until a second engine ships it?
- **Navigation API:** Chromium-only (window.navigation). Is Spiral interested in early adoption or should it wait for wider implementation?
- **Background Fetch / Sync:** These require Service Worker infrastructure. Should they be deferred to Phase 6 or bundled with Service Workers in Phase 5?
- **spiral-storage crate:** The GAP_ANALYSIS flags that spiral-storage does not exist yet. Is the intention a unified crate covering cookies, Web Storage, IndexedDB, OPFS, and quota management, or separate per-mechanism crates?
