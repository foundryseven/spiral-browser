# Chunk 6 — Web Platform APIs & Runtime (A) / Storage & State (B)

> §A (Web Platform APIs & Runtime) is written by chunk 6. §B (Storage &
> State) was written by chunk 4. Per-file cap: 600 lines. Both sections
> live in this file because the combined size is well under 600.

---

## §A — Web Platform APIs & Runtime

## Scope

**In:** Fetch, Streams, Workers (Dedicated / Shared / Service), background
task scheduling, WebAssembly, WebGPU, WebGL / WebGL2, WebNN, WebAuthn,
Payment Request, Web Share, Clipboard (async), File System Access, Origin
Private File System sync access, Notifications, Web Push, WebTransport,
WebSocket, Gamepad, Speech (synthesis + recognition), Pointer / Touch /
Mouse / Keyboard event model, Battery, Vibration, Geolocation, Generic
Sensors, Web MIDI, Web Serial, WebUSB, Web Bluetooth, Web NFC, WebCodecs,
WebXR, Compression Streams, Encoding, URL, Blob / File / FileReader /
FileReaderSync, structuredClone, atob / btoa, SubtleCrypto surface,
Permissions API, Prioritised Task Scheduling.

**Out:** HTML elements, DOM, CSS, networking protocols, security policy
(CSP, CORS, SRI), storage (cookies / WebStorage / IndexedDB / OPFS /
CacheStorage), media codecs / EME / MSE / WebRTC media plane, user-facing
UX, DevTools, accessibility, extension APIs, platform distribution.

**Naming:** WHATWG / W3C / IETF spec names. No product names.

**Grounding:** `crates/spiral-vortex/src/` exposes ES builtins
(`Array`, `Math`, `Object`, `Console`) only. No browser API surface
(fetch / Worker / WebAssembly / crypto / URL / Blob / structuredClone /
TextEncoder) is implemented. `crates/spiral-crypto/src/lib.rs` provides
`random_bytes` and `sha256` (CSPRNG + SHA-256) but is not yet wired
into the Vortex global object.

## Rows

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|---|---|---|---|---|---|---|---|
| 1 | `fetch(input, init?)` global | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §fetch](https://fetch.spec.whatwg.org/#fetch-method) |
| 2 | `Request` constructor | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §request-class](https://fetch.spec.whatwg.org/#request-class) |
| 3 | `Response` constructor | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §response-class](https://fetch.spec.whatwg.org/#response-class) |
| 4 | `Headers` constructor | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §headers-class](https://fetch.spec.whatwg.org/#headers-class) |
| 5 | `Body` mixin (`arrayBuffer` / `blob` / `formData` / `json` / `text`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §body-mixin](https://fetch.spec.whatwg.org/#body-mixin) |
| 6 | `AbortController` / `AbortSignal` | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [DOM §aborting-ongoing-activities](https://dom.spec.whatwg.org/#aborting-ongoing-activities) |
| 7 | `AbortSignal.timeout(ms)` | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [DOM §dom-abortsignal-timeout](https://dom.spec.whatwg.org/#dom-abortsignal-timeout) |
| 8 | `AbortSignal.any(signals)` | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [DOM §dom-abortsignal-any](https://dom.spec.whatwg.org/#dom-abortsignal-any) |
| 9 | Fetch priority hints (`fetchPriority` / `priority` header) | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Fetch Priority](https://w3c.github.io/fetch-priority/) |
| 10 | Fetch keepalive | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [WHATWG Fetch §keepalive-flag](https://fetch.spec.whatwg.org/#keepalive-flag) |
| 11 | Fetch redirect modes (`follow` / `error` / `manual`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §request-redirect-mode](https://fetch.spec.whatwg.org/#request-redirect-mode) |
| 12 | Fetch credentials modes (`omit` / `same-origin` / `include`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §request-credentials-mode](https://fetch.spec.whatwg.org/#request-credentials-mode) |
| 13 | Fetch `mode` (`cors` / `no-cors` / `same-origin` / `navigate`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §request-mode](https://fetch.spec.whatwg.org/#request-mode) |
| 14 | Fetch `referrerPolicy` | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Fetch §request-referrer-policy](https://fetch.spec.whatwg.org/#request-referrer-policy) |
| 15 | Duplex streaming (`request.duplex: "half"`) | desktop+mobile+embedded | not-started | niche | P4 | M | Chromium: yes · Gecko: partial · WebKit: no · Servo: no · Ladybird: no · Flow: no | [Fetch PR #1457](https://github.com/whatwg/fetch/pull/1457) |
| 16 | `ReadableStream` constructor / default reader | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Streams §readable-streams](https://streams.spec.whatwg.org/#readable-streams) |
| 17 | `WritableStream` constructor / default writer | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [WHATWG Streams §writable-streams](https://streams.spec.whatwg.org/#writable-streams) |
| 18 | `TransformStream` (built-in `ReadableStream` ↔ `WritableStream` pair with transform) | desktop+mobile+embedded | not-started | widespread | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WHATWG Streams §transform-streams](https://streams.spec.whatwg.org/#transform-streams) |
| 19 | `ReadableStreamBYOBReader` (bring-your-own-buffer) | desktop+mobile+embedded | not-started | widespread | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WHATWG Streams §byob-reader](https://streams.spec.whatwg.org/#byob-reader) |
| 20 | `ReadableStream.pipeTo(writable)` / `pipeThrough(transform)` | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WHATWG Streams §readable-stream-pipe](https://streams.spec.whatwg.org/#readable-stream-pipe) |
| 21 | `ReadableStream.tee()` (branch to two readers) | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [WHATWG Streams §readable-stream-tee](https://streams.spec.whatwg.org/#readable-stream-tee) |
| 22 | Stream backpressure (high-water mark, pull, write queuing) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [WHATWG Streams §backpressure](https://streams.spec.whatwg.org/#backpressure) |
| 23 | `ByteLengthQueuingStrategy` / `CountQueuingStrategy` | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [WHATWG Streams §queuing-strategies](https://streams.spec.whatwg.org/#queuing-strategies) |
| 24 | `Worker` constructor (dedicated worker) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [HTML §workers](https://html.spec.whatwg.org/multipage/workers.html) |
| 25 | `SharedWorker` constructor | desktop+mobile+embedded | not-started | widespread | P6 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [HTML §sharedworkerglobalscope](https://html.spec.whatwg.org/multipage/workers.html#sharedworkerglobalscope) |
| 26 | Service Worker registration (`navigator.serviceWorker.register`) | desktop+mobile+embedded | not-started | ubiquitous | P5 | XL | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Service Workers §register](https://w3c.github.io/ServiceWorker/#navigator-service-worker-register) |
| 27 | `ServiceWorkerContainer` / `ServiceWorkerRegistration` | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Service Workers §container](https://w3c.github.io/ServiceWorker/#serviceworkercontainer-interface) |
| 28 | `importScripts(url, ...)` in workers | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [HTML §importscripts](https://html.spec.whatwg.org/multipage/workers.html#importscripts) |
| 29 | `postMessage` / `onmessage` (worker messaging) | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [HTML §dom-worker-postmessage](https://html.spec.whatwg.org/multipage/workers.html#dom-worker-postmessage) |
| 30 | `structuredClone` algorithm (worker message body) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [HTML §structured-clone](https://html.spec.whatwg.org/multipage/structured-data.html#structured-clone) |
| 31 | `WorkerGlobalScope` (self, location, navigator, caches) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [HTML §workerglobalscope](https://html.spec.whatwg.org/multipage/workers.html#workerglobalscope) |
| 32 | `CacheStorage` (`caches.open`) inside service worker | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Service Workers §cache](https://w3c.github.io/ServiceWorker/#cache-storage) |
| 33 | OPFS sync access handle inside worker (`createSyncAccessHandle`) | desktop+mobile+embedded | not-started | niche | P5 | M | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG File System Access §syncaccesshandle](https://wicg.github.io/file-system-access/#syncaccesshandle) |
| 34 | Worker module scripts (`new Worker(url, {type:"module"})`) | desktop+mobile+embedded | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [HTML §worker-options](https://html.spec.whatwg.org/multipage/workers.html#worker-options) |
| 35 | Service Worker update flow (`update`, `skipWaiting`, `clients.claim`) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Service Workers §lifetime](https://w3c.github.io/ServiceWorker/#service-worker-lifetime) |
| 36 | `requestIdleCallback` / `cancelIdleCallback` | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [WICG requestIdleCallback](https://w3c.github.io/requestidlecallback/) |
| 37 | `requestAnimationFrame` / `cancelAnimationFrame` (timing only) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: stub · Flow: no | [HTML §requestanimationframe](https://html.spec.whatwg.org/multipage/imagebitmap-and-animations.html#dom-animationframeprovider-requestanimationframe) |
| 38 | Prioritised Task Scheduling API (`scheduler.postTask`, `scheduler.yield`) | desktop+mobile+embedded | not-started | niche | P4 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG Prioritised Task Scheduling](https://wicg.github.io/scheduling-apis/) |
| 39 | Task priorities (`user-blocking` / `user-visible` / `background`) | desktop+mobile+embedded | not-started | niche | P4 | S | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG Scheduling APIs §task-priority](https://wicg.github.io/scheduling-apis/#task-priority) |
| 40 | `isInputPending()` (input-priority scheduling) | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG Scheduling APIs §input](https://wicg.github.io/scheduling-apis/#input-priority) |
| 41 | WebAssembly `WebAssembly.Module` | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [W3C Wasm Embedding §module](https://webassembly.github.io/spec/web-api/js-api.html#module-constructor) |
| 42 | `WebAssembly.Instance` | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [W3C Wasm Embedding §instance](https://webassembly.github.io/spec/web-api/js-api.html#instance-constructor) |
| 43 | `WebAssembly.Memory` (shared / non-shared) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [W3C Wasm Embedding §memory](https://webassembly.github.io/spec/web-api/js-api.html#memory-constructor) |
| 44 | `WebAssembly.Table` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [W3C Wasm Embedding §table](https://webassembly.github.io/spec/web-api/js-api.html#table-constructor) |
| 45 | `WebAssembly.Global` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [W3C Wasm Embedding §global](https://webassembly.github.io/spec/web-api/js-api.html#global-constructor) |
| 46 | `WebAssembly.compile` / `compileStreaming` (off-thread) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [W3C Wasm Embedding §compile](https://webassembly.github.io/spec/web-api/js-api.html#dom-webassembly-compile) |
| 47 | `WebAssembly.instantiate` / `instantiateStreaming` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [W3C Wasm Embedding §instantiate](https://webassembly.github.io/spec/web-api/js-api.html#dom-webassembly-instantiate) |
| 48 | Wasm reference types (externref, funcref) | desktop+mobile+embedded | not-started | widespread | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [Wasm Reference Types](https://webassembly.github.io/reference-types/core/) |
| 49 | Wasm SIMD (`v128` ops) | desktop+mobile+embedded | not-started | widespread | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: partial | [Wasm SIMD](https://webassembly.github.io/simd/core/) |
| 50 | Wasm threads (atomic memory ops, `memory.shared`) | desktop+mobile+embedded | not-started | widespread | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [Wasm Threads](https://webassembly.github.io/threads/core/) |
| 51 | Wasm exception handling (`try` / `catch` / `throw` / `rethrow`) | desktop+mobile+embedded | not-started | widespread | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [Wasm Exception Handling](https://webassembly.github.io/exception-handling/core/) |
| 52 | Wasm tail calls (`return_call` / `return_call_indirect`) | desktop+mobile+embedded | not-started | niche | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: partial | [Wasm Tail Call](https://webassembly.github.io/tail-call/core/) |
| 53 | Wasm multi-value returns | desktop+mobile+embedded | not-started | widespread | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [Wasm Multi-Value](https://webassembly.github.io/multi-value/core/) |
| 54 | Wasm bulk memory ops (`memory.copy` / `memory.fill`) | desktop+mobile+embedded | not-started | widespread | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [Wasm Bulk Memory](https://webassembly.github.io/bulk-memory-operations/core/) |
| 55 | Wasm relaxed SIMD | desktop+mobile+embedded | not-started | niche | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: partial | [Wasm Relaxed SIMD](https://webassembly.github.io/relaxed-simd/core/) |
| 56 | Wasm memory64 (>4 GiB address space) | desktop+mobile+embedded | not-started | niche | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: partial | [Wasm Memory64](https://webassembly.github.io/memory64/core/) |
| 57 | Wasm GC (struct, array, ref types, gc-managed objects) | desktop+mobile+embedded | not-started | niche | P4 | XL | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [Wasm GC proposal](https://github.com/WebAssembly/gc/blob/main/proposals/gc/Overview.md) |
| 58 | Wasm component model + WASI preview | desktop+mobile+embedded | not-started | niche | P5 | XL | Chromium: yes (WASI preview1) · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WASI Proposals](https://github.com/WebAssembly/WASI/blob/main/Proposals.md) |
| 59 | WebGPU `GPUAdapter` / `GPUDevice` | desktop+mobile | not-started | widespread | P6 | XL | Chromium: yes · Gecko: yes (Nightly) · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebGPU §adapter](https://www.w3.org/TR/webgpu/#adapter-interface) |
| 60 | `GPUBuffer` / `GPUTexture` / `GPUTextureView` | desktop+mobile | not-started | widespread | P6 | L | Chromium: yes · Gecko: yes (Nightly) · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebGPU §buffer](https://www.w3.org/TR/webgpu/#buffer-interface) |
| 61 | `GPURenderPipeline` / `GPUComputePipeline` | desktop+mobile | not-started | widespread | P6 | L | Chromium: yes · Gecko: yes (Nightly) · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebGPU §pipeline](https://www.w3.org/TR/webgpu/#pipeline-interface) |
| 62 | `GPUBindGroup` / `GPUBindGroupLayout` | desktop+mobile | not-started | widespread | P6 | M | Chromium: yes · Gecko: yes (Nightly) · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebGPU §bindgroup](https://www.w3.org/TR/webgpu/#bindgroup-interface) |
| 63 | WGSL shader language | desktop+mobile | not-started | widespread | P6 | L | Chromium: yes · Gecko: yes (Nightly) · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WGSL](https://www.w3.org/TR/WGSL/) |
| 64 | `GPUCanvasContext` (compositor hook from `<canvas>`) | desktop+mobile | not-started | widespread | P6 | M | Chromium: yes · Gecko: yes (Nightly) · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebGPU §canvas](https://www.w3.org/TR/webgpu/#canvas-context) |
| 65 | `GPUCommandEncoder` / pass encoders | desktop+mobile | not-started | widespread | P6 | L | Chromium: yes · Gecko: yes (Nightly) · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebGPU §encoder](https://www.w3.org/TR/webgpu/#command-encoder-interface) |
| 66 | WebGL 1.0 (`WebGLRenderingContext`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [Khronos WebGL 1.0](https://registry.khronos.org/webgl/specs/latest/1.0/) |
| 67 | WebGL 2.0 (`WebGL2RenderingContext`) | desktop+mobile+embedded | not-started | widespread | P6 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [Khronos WebGL 2.0](https://registry.khronos.org/webgl/specs/latest/2.0/) |
| 68 | WebGL extensions (compressed textures, instancing, transform feedback) | desktop+mobile+embedded | not-started | widespread | P6 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [WebGL Extension Registry](https://registry.khronos.org/webgl/extensions/) |
| 69 | ANGLE as default WebGL backend | desktop+mobile+embedded | not-started | widespread | P6 | M | Chromium: yes (ANGLE) · Gecko: yes (Windows ANGLE) · WebKit: yes (macOS ANGLE) · Servo: no · Ladybird: no · Flow: no | [ANGLE](https://github.com/google/angle) |
| 70 | WebNN `MLContext` / `MLGraphBuilder` | desktop+mobile | not-started | niche | P6 | XL | Chromium: yes (Origin Trial) · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C WebNN](https://www.w3.org/TR/webnn/) |
| 71 | WebNN operations (`conv2d`, `matmul`, `softmax`, `relu`, `pooling`) | desktop+mobile | not-started | niche | P6 | L | Chromium: yes (OT) · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C WebNN §ops](https://www.w3.org/TR/webnn/#api) |
| 72 | WebNN operand types (`float32` / `int32` / `int8` / `uint4`) | desktop+mobile | not-started | niche | P6 | M | Chromium: yes (OT) · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C WebNN §operands](https://www.w3.org/TR/webnn/#operand-types) |
| 73 | WebNN MLGraph compilation + dispatch | desktop+mobile | not-started | niche | P6 | L | Chromium: yes (OT) · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C WebNN §graph](https://www.w3.org/TR/webnn/#mlgraph) |
| 74 | WebAuthn `navigator.credentials.create({publicKey})` | desktop+mobile | not-started | widespread | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebAuthn §create](https://w3c.github.io/webauthn/#sctn-createCredential) |
| 75 | WebAuthn `navigator.credentials.get({publicKey})` | desktop+mobile | not-started | widespread | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebAuthn §get](https://w3c.github.io/webauthn/#sctn-getAssertion) |
| 76 | WebAuthn resident keys (`requireResidentKey`, `residentKey`) | desktop+mobile | not-started | widespread | P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebAuthn §resident](https://w3c.github.io/webauthn/#sctn-resident-credential) |
| 77 | WebAuthn user verification (`userVerification: "required"`) | desktop+mobile | not-started | widespread | P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebAuthn §user-verification](https://w3c.github.io/webauthn/#user-verification) |
| 78 | WebAuthn attestation (direct / indirect / none) | desktop+mobile | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebAuthn §attestation](https://w3c.github.io/webauthn/#sctn-attestation) |
| 79 | Payment Request `PaymentRequest` constructor | desktop+mobile | not-started | widespread | P6 | L | Chromium: yes · Gecko: yes · WebKit: yes (Apple Pay) · Servo: no · Ladybird: no · Flow: no | [W3C Payment Request §constructor](https://w3c.github.io/payment-request/#paymentrequest-interface) |
| 80 | `PaymentResponse` / `PaymentMethodData` | desktop+mobile | not-started | widespread | P6 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Payment Request §response](https://w3c.github.io/payment-request/#paymentresponse-interface) |
| 81 | Payment Request shipping address + contact info events | desktop+mobile | not-started | widespread | P6 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Payment Request §shipping](https://w3c.github.io/payment-request/#shipping-address-changed-algorithm) |
| 82 | `canMakePayment()` | desktop+mobile | not-started | widespread | P6 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Payment Request §canmakepayment](https://w3c.github.io/payment-request/#canmakepayment-method) |
| 83 | `navigator.share()` (Web Share API) | mobile+desktop (Chromium) | not-started | widespread | P6 | S | Chromium: yes · Gecko: yes (mobile) · WebKit: yes (iOS 12.2+) · Servo: no · Ladybird: no · Flow: no | [W3C Web Share API](https://w3c.github.io/web-share/) |
| 84 | `navigator.canShare()` | mobile+desktop (Chromium) | not-started | widespread | P6 | S | Chromium: yes · Gecko: yes (mobile) · WebKit: yes (iOS 12.2+) · Servo: no · Ladybird: no · Flow: no | [W3C Web Share API §canShare](https://w3c.github.io/web-share/#canshare-method) |
| 85 | Share target (handler registration in web app manifest) | mobile | not-started | niche | P6 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Manifest §share_target](https://w3c.github.io/manifest/#share_target-member) |
| 86 | Async Clipboard (`navigator.clipboard.readText` / `writeText`) | desktop+mobile+embedded | not-started | widespread | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Clipboard API](https://www.w3.org/TR/clipboard-apis/) |
| 87 | Clipboard `read()` / `write()` (arbitrary MIME, images) | desktop+mobile | not-started | niche | P4 | M | Chromium: yes · Gecko: yes (text only) · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Clipboard API §read](https://www.w3.org/TR/clipboard-apis/#dom-clipboard-read) |
| 88 | Clipboard permission model (transient activation gating) | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Clipboard API §permission](https://www.w3.org/TR/clipboard-apis/#permission) |
| 89 | File System Access `showOpenFilePicker` | desktop | not-started | niche | P6 | L | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG File System Access §showopen](https://wicg.github.io/file-system-access/#dom-window-showopenfilepicker) |
| 90 | File System Access `showSaveFilePicker` | desktop | not-started | niche | P6 | L | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG File System Access §showsave](https://wicg.github.io/file-system-access/#dom-window-showsavefilepicker) |
| 91 | File System Access `showDirectoryPicker` | desktop | not-started | niche | P6 | L | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG File System Access §showdir](https://wicg.github.io/file-system-access/#dom-window-showdirectorypicker) |
| 92 | `FileSystemFileHandle` / `FileSystemDirectoryHandle` | desktop | not-started | niche | P6 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG File System Access §handles](https://wicg.github.io/file-system-access/#handles) |
| 93 | `createWritable()` (file handle → writable stream) | desktop | not-started | niche | P6 | S | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG File System Access §createwritable](https://wicg.github.io/file-system-access/#dom-filesystemfilehandle-createwritable) |
| 94 | Notifications `Notification` constructor | desktop+mobile | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes (via polyfill / macOS 16+) · Servo: no · Ladybird: no · Flow: no | [WHATWG Notifications](https://notifications.spec.whatwg.org/) |
| 95 | Notification permission model (`granted` / `denied` / `default`) | desktop+mobile | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [WHATWG Notifications §permission](https://notifications.spec.whatwg.org/#permission-model) |
| 96 | Notification `actions` (button array) | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WHATWG Notifications §actions](https://notifications.spec.whatwg.org/#actions) |
| 97 | Notification `badge` / `image` / `vibrate` | desktop+mobile | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: partial · Servo: no · Ladybird: no · Flow: no | [WHATWG Notifications §resources](https://notifications.spec.whatwg.org/#resources) |
| 98 | Web Push (`PushManager.subscribe`, `PushSubscription`) | desktop+mobile+embedded | not-started | widespread | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes (iOS 16.4+) · Servo: no · Ladybird: no · Flow: no | [W3C Push API](https://www.w3.org/TR/push-api/) |
| 99 | Push event + VAPID keys | desktop+mobile+embedded | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [RFC 8030 §web-push](https://datatracker.ietf.org/doc/html/rfc8030) |
| 100 | WebTransport (`new WebTransport(url)`) | desktop+mobile | not-started | niche | P6 | L | Chromium: yes · Gecko: yes · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebTransport](https://www.w3.org/TR/webtransport/) |
| 101 | WebTransport datagrams (unreliable, low-latency) | desktop+mobile | not-started | niche | P6 | M | Chromium: yes · Gecko: yes · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebTransport §datagrams](https://www.w3.org/TR/webtransport/#datagrams) |
| 102 | WebTransport bidirectional + unidirectional streams | desktop+mobile | not-started | niche | P6 | M | Chromium: yes · Gecko: yes · WebKit: yes (Safari TP) · Servo: no · Ladybird: no · Flow: no | [W3C WebTransport §streams](https://www.w3.org/TR/webtransport/#streams) |
| 103 | WebSocket (`new WebSocket(url, protocols)`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [RFC 6455 §api](https://datatracker.ietf.org/doc/html/rfc6455#section-4.1) |
| 104 | WebSocket binary types (`blob` / `arraybuffer`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [RFC 6455 §binary-type](https://datatracker.ietf.org/doc/html/rfc6455#section-4.1) |
| 105 | WebSocket close codes (1000-4999) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [RFC 6455 §close-codes](https://datatracker.ietf.org/doc/html/rfc6455#section-7.4) |
| 106 | WebSocket subprotocol negotiation | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [RFC 6455 §subprotocol](https://datatracker.ietf.org/doc/html/rfc6455#section-1.9) |
| 107 | Gamepad (`navigator.getGamepads()`, `GamepadEvent`) | desktop+mobile | not-started | widespread | P6 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Gamepad](https://w3c.github.io/gamepad/) |
| 108 | Gamepad haptic feedback (vibration actuator) | desktop | not-started | niche | P6 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Gamepad Extensions](https://w3c.github.io/gamepad/extensions.html) |
| 109 | SpeechSynthesis (`speechSynthesis.speak(new SpeechSynthesisUtterance)`) | desktop+mobile | not-started | widespread | P6 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Speech Synthesis](https://wicg.github.io/speech-api/#speechsynthesis) |
| 110 | SpeechSynthesis voices + onvoiceschanged | desktop+mobile | not-started | widespread | P6 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Speech Synthesis §voices](https://wicg.github.io/speech-api/#speechsynthesisvoice) |
| 111 | SpeechRecognition (`new SpeechRecognition()`) | desktop+mobile | not-started | niche | P6 | L | Chromium: yes (via webkit prefix) · Gecko: no · WebKit: yes (private) · Servo: no · Ladybird: no · Flow: no | [W3C Speech API §recognition](https://wicg.github.io/speech-api/#speechrecognition) |
| 112 | Pointer Events (unified mouse / touch / pen) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [W3C Pointer Events 3](https://www.w3.org/TR/pointerevents3/) |
| 113 | Touch Events (`touchstart` / `touchmove` / `touchend`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes (compat) · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [W3C Touch Events](https://www.w3.org/TR/touch-events/) |
| 114 | Mouse Events (click / mousedown / mousemove / wheel) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [UI Events §mouseevents](https://www.w3.org/TR/uievents/#events-mouse-types) |
| 115 | Keyboard Events (keydown / keyup + `code` / `key` / `keyCode`) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [UI Events §keyboardevents](https://www.w3.org/TR/uievents/#events-keyboard-types) |
| 116 | Battery Status (`navigator.getBattery()`) | desktop+mobile | not-started | niche | P6 | S | Chromium: yes (limited) · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Battery Status](https://www.w3.org/TR/battery-status/) |
| 117 | Vibration (`navigator.vibrate(pattern)`) | mobile+desktop (Chromium) | not-started | niche | P6 | S | Chromium: yes · Gecko: yes (mobile) · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Vibration](https://www.w3.org/TR/vibration/) |
| 118 | Geolocation (`navigator.geolocation.getCurrentPosition`) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [W3C Geolocation](https://www.w3.org/TR/geolocation-API/) |
| 119 | Geolocation `watchPosition` / `clearWatch` | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: no | [W3C Geolocation §watchposition](https://www.w3.org/TR/geolocation-API/#watchposition-method) |
| 120 | Generic Sensor: Accelerometer | mobile+embedded | not-started | widespread | P6 | S | Chromium: yes · Gecko: yes · WebKit: yes (iOS 14.5+) · Servo: no · Ladybird: no · Flow: no | [W3C Accelerometer](https://www.w3.org/TR/accelerometer/) |
| 121 | Generic Sensor: Gyroscope | mobile+embedded | not-started | widespread | P6 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Gyroscope](https://www.w3.org/TR/gyroscope/) |
| 122 | Generic Sensor: Magnetometer | mobile+embedded | not-started | widespread | P6 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Magnetometer](https://www.w3.org/TR/magnetometer/) |
| 123 | Generic Sensor: AmbientLightSensor | mobile+embedded | not-started | widespread | P6 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Ambient Light](https://www.w3.org/TR/ambient-light/) |
| 124 | Generic Sensor: Barometer (`PressureSensor`) | mobile+embedded | not-started | niche | P6 | S | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Proximity](https://www.w3.org/TR/proximity/) |
| 125 | Web MIDI (`navigator.requestMIDIAccess()`) | desktop | not-started | niche | P6 | M | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Web MIDI](https://www.w3.org/TR/webmidi/) |
| 126 | Web Serial (`navigator.serial.requestPort()`) | desktop | not-started | niche | P6 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG Web Serial](https://wicg.github.io/serial/) |
| 127 | WebUSB (`navigator.usb.requestDevice()`) | desktop | not-started | niche | P6 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C WebUSB](https://wicg.github.io/webusb/) |
| 128 | Web Bluetooth (`navigator.bluetooth.requestDevice()`) | desktop+mobile+embedded | not-started | niche | P6 | M | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [WICG Web Bluetooth](https://webbluetoothcg.github.io/web-bluetooth/) |
| 129 | Web NFC (`new NDEFReader()`) | mobile | not-started | niche | P6 | M | Chromium: yes (Android) · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Web NFC](https://w3c.github.io/web-nfc/) |
| 130 | WebCodecs `VideoEncoder` / `VideoDecoder` | desktop+mobile | not-started | widespread | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes (Safari 16.4+) · Servo: no · Ladybird: no · Flow: no | [W3C WebCodecs §video-encoder](https://www.w3.org/TR/webcodecs/#videoencoder-interface) |
| 131 | WebCodecs `AudioEncoder` / `AudioDecoder` | desktop+mobile | not-started | widespread | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes (Safari 16.4+) · Servo: no · Ladybird: no · Flow: no | [W3C WebCodecs §audio-encoder](https://www.w3.org/TR/webcodecs/#audioencoder-interface) |
| 132 | WebCodecs `VideoFrame` / `AudioData` (chunks 5 cross-ref) | desktop+mobile | not-started | widespread | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebCodecs §videoframe](https://www.w3.org/TR/webcodecs/#videoframe-interface) |
| 133 | WebCodecs `ImageDecoder` (animated images) | desktop+mobile | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebCodecs §image-decoder](https://www.w3.org/TR/webcodecs/#imagedecoder-interface) |
| 134 | WebXR `XRSystem` / `XRSession` | desktop+mobile | not-started | niche | P6 | XL | Chromium: yes · Gecko: yes · WebKit: yes (visionOS, iOS 17.4+) · Servo: no · Ladybird: no · Flow: no | [W3C WebXR §session](https://www.w3.org/TR/webxr/#xrsession-interface) |
| 135 | WebXR `immersive-vr` mode | desktop+mobile | not-started | niche | P6 | L | Chromium: yes · Gecko: yes · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C WebXR §immersive-vr](https://www.w3.org/TR/webxr/#immersive-vr-mode) |
| 136 | WebXR `immersive-ar` mode (camera passthrough) | mobile | not-started | niche | P6 | L | Chromium: yes · Gecko: no · WebKit: yes (visionOS, iOS 17.4+) · Servo: no · Ladybird: no · Flow: no | [W3C WebXR AR Module](https://www.w3.org/TR/webxr-ar-module/) |
| 137 | WebXR `inline` mode (2D viewport) | desktop+mobile | not-started | niche | P6 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebXR §inline](https://www.w3.org/TR/webxr/#inline-xr-session-mode) |
| 138 | WebXR hand input + hit test + anchors | mobile | not-started | niche | P6 | L | Chromium: yes · Gecko: no · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebXR Hand Input](https://www.w3.org/TR/webxr-hand-input/) |
| 139 | WebXR plane + mesh detection | mobile | not-started | niche | P6 | L | Chromium: yes · Gecko: no · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebXR Plane Detection](https://www.w3.org/TR/webxr-plane-detection/) |
| 140 | WebXR depth sensing + layers + DOM overlay | mobile | not-started | niche | P6 | M | Chromium: yes · Gecko: no · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C WebXR Depth](https://www.w3.org/TR/webxr-depth-sensing/) |
| 141 | Compression Streams (`new CompressionStream("gzip")`) | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes (Safari 16.4+) · Servo: no · Ladybird: no · Flow: no | [W3C Compression Streams](https://www.w3.org/TR/compression/) |
| 142 | Decompression Streams (`new DecompressionStream("gzip")`) | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Compression Streams §decompression](https://www.w3.org/TR/compression/#decompression-stream) |
| 143 | Compression formats (`gzip` / `deflate` / `deflate-raw` / `br` / `zstd`) | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes (gzip+deflate+deflate-raw) · Gecko: yes (gzip+deflate+br+zstd) · WebKit: yes (gzip+deflate+deflate-raw) · Servo: no · Ladybird: no · Flow: no | [W3C Compression Streams §formats](https://www.w3.org/TR/compression/#supported-formats) |
| 144 | `TextEncoder` / `TextDecoder` (UTF-8 default) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [WHATWG Encoding §interface](https://encoding.spec.whatwg.org/#interface-textencoder) |
| 145 | `TextEncoderStream` / `TextDecoderStream` | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [WHATWG Encoding §stream](https://encoding.spec.whatwg.org/#interface-textencoderstream) |
| 146 | `URL` constructor + `URL.parse` / `URL.canParse` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [WHATWG URL §url-class](https://url.spec.whatwg.org/#url-class) |
| 147 | `URLSearchParams` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [WHATWG URL §searchparams](https://url.spec.whatwg.org/#interface-urlsearchparams) |
| 148 | `Blob` constructor + `slice` | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [W3C File API §blob](https://www.w3.org/TR/FileAPI/#blob-section) |
| 149 | `File` constructor + name, type, lastModified | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [W3C File API §file](https://www.w3.org/TR/FileAPI/#file-section) |
| 150 | `FileReader` / `FileList` / blob URL | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: partial · Flow: yes | [W3C File API §filereader](https://www.w3.org/TR/FileAPI/#filereader-interface) |
| 151 | `FileReaderSync` (worker-only) | desktop+mobile+embedded | not-started | widespread | P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C File API §filereadersync](https://www.w3.org/TR/FileAPI/#FileReaderSync) |
| 152 | `structuredClone(value)` (global) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: yes | [HTML §dom-structuredclone](https://html.spec.whatwg.org/multipage/structured-data.html#dom-structuredclone) |
| 153 | `atob` / `btoa` (base64 helpers) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [HTML §dom-btoa](https://html.spec.whatwg.org/multipage/webappapis.html#dom-btoa) |
| 154 | `crypto.getRandomValues(buffer)` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: yes · Ladybird: partial · Flow: yes | [W3C Web Crypto §getrandomvalues](https://www.w3.org/TR/WebCrypto2/#Crypto-method-getRandomValues) |
| 155 | `crypto.randomUUID()` | desktop+mobile+embedded | not-started | widespread | P3 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: yes | [W3C Web Crypto §randomuuid](https://www.w3.org/TR/WebCrypto2/#Crypto-method-randomUUID) |
| 156 | `crypto.subtle` (SubtleCrypto: digest, encrypt, sign, deriveKey) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: yes | [W3C Web Crypto §subtlecrypto](https://www.w3.org/TR/WebCrypto2/#subtlecrypto-interface) |
| 157 | SubtleCrypto ECDSA + Ed25519 + RSA-PSS | desktop+mobile+embedded | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: yes | [W3C Web Crypto §ecdsa](https://www.w3.org/TR/WebCrypto2/#ecdsa-operations) |
| 158 | SubtleCrypto AES-GCM + AES-CTR + ChaCha20-Poly1305 | desktop+mobile+embedded | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: yes | [W3C Web Crypto §aes](https://www.w3.org/TR/WebCrypto2/#aes-crypto-operations) |
| 159 | SubtleCrypto ECDH + HKDF + PBKDF2 (key derivation) | desktop+mobile+embedded | not-started | widespread | P5 | M | Chromium: yes · Gecko: yes · WebKit: yes · Servo: partial · Ladybird: no · Flow: yes | [W3C Web Crypto §dh](https://www.w3.org/TR/WebCrypto2/#diffie-hellman-operations) |
| 160 | `navigator.permissions.query({name})` | desktop+mobile+embedded | not-started | widespread | P4 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Permissions §query](https://www.w3.org/TR/permissions/#dom-permissions-query) |
| 161 | `navigator.permissions.revoke` | desktop+mobile+embedded | not-started | niche | P4 | S | Chromium: yes · Gecko: no · WebKit: no · Servo: no · Ladybird: no · Flow: no | [W3C Permissions §revoke](https://www.w3.org/TR/permissions/#dom-permissions-revoke) |
| 162 | Push permission (`name: "push"`, with userVisibleOnly) | desktop+mobile+embedded | not-started | widespread | P5 | S | Chromium: yes · Gecko: yes · WebKit: yes · Servo: no · Ladybird: no · Flow: no | [W3C Push API §permissions](https://www.w3.org/TR/push-api/#permissions) |

---

## Cross-refs to `specs/GAP_ANALYSIS.md`

| GAP_ANALYSIS row | Section | Line |
|---|---|---|
| WebSockets (placeholder) | §2.2 Network | 167 |
| WebGL binding (wgpu backend) | §5.3 Plumbing | 293 |
| WebGPU binding | §5.3 Plumbing | 294 |
| 17. WebGL / WebGPU (P3) | §5.3 Plumbing | 392 |
| Same-Origin File System (cross-ref §B OPFS) | §2.3 Storage | 193 |

---

## Open questions for the user

1. **Worker threading model.** Spiral's `AGENTS.md` flags Bet 1 as
   "Shared-Everything Multi-Process" but the worker API contract (one
   thread per DedicatedWorker, one OS thread for ServiceWorker per
   origin) is separate from the process model. Should `spiral-vortex`
   workers (Phase 5) run on `tokio::runtime` blocking tasks, a dedicated
   `wasmtime`-style isolate pool, or piggyback on Bet 1's shared
   heap? This affects memory caps and `Atomics.wait` semantics.

2. **Wasm engine choice.** The `spiral-vortex` README forbids V8, QuickJS,
   and Boa. It does not forbid `wasmtime` or `wasmer`. Should WASM
   execution be embedded in Vortex via `wasmtime` (which is Rust, audited,
   and already a transitive dep of `rustls` for some platforms), or built
   from scratch in Vortex like the JS engine? Building from scratch is
   12-24 months of work minimum.

3. **WebGPU timing.** WebGPU landed in Chromium 113 (2023) and is
   ubiquitous by 2026. Spiral's `spiral-render` uses Vello, which is a
   2D renderer over wgpu. WebGPU and Vello are not the same surface
   (WebGPU is 3D-first, with explicit pipelines). Should Spiral expose
   WebGPU by routing it through wgpu directly, or wait until Vello
   supports compute / 3D?

4. **WebGL priority vs WebGPU.** WebGL 1/2 still ship in every browser
   in 2026 and a meaningful slice of web content still uses it (WebGL2
   is preferred over WebGL1 for numerical code). WebGL on top of wgpu
   requires either `wgpu` exposing a GL compat profile (no) or routing
   through ANGLE (large, Chromium-only). Should Spiral target WebGL2
   via ANGLE, or accept a "no WebGL" stance and require WebGPU from
   web content?

5. **WebXR scope.** WebXR is a Chromium / visionOS / Firefox Reality
   Hub surface. On desktop Linux / Windows / macOS it is rare outside
   Valve Index and Meta Quest Link. Is WebXR a Phase 6 (post-MP) goal
   or a "not in plan" capability?

6. **Hardware APIs (Serial / USB / Bluetooth / NFC / MIDI).** These are
   niche, mostly Chromium-only, and each needs a native device-binding
   layer (IOKit on macOS, libusb / dbus on Linux, WinUSB on Windows).
   Should they be tracked as one "hardware API" cluster with a shared
   `spiral-device` crate, or as separate deliverables, or deferred
   indefinitely until the user signals demand?

7. **Push service.** Web Push requires a push service endpoint
   (FCM, Mozilla autopush, or self-hosted). VAPID, RFC 8030, and the
   subscription handshake are non-trivial. Is there a target push
   service provider, or should Web Push be deferred until the network
   stack is stable enough to talk to a test endpoint?

8. **WebNN.** WebNN is in Origin Trial in Chromium and has no Gecko or
   WebKit commitment. The M4 novelty-claim rule says we should verify
   before claiming "first" or "novel". WebNN is not novel (it is a
   spec), but its priority is low. Confirm: track as a Phase 6
   "evaluate later" item?

9. **File System Access scope.** The Chromium-only API has been
   [WICG-stabilised since 2024](https://wicg.github.io/file-system-access/)
   but is still single-vendor. OPFS (cross-vendor, scoped to a single
   origin) is the safer starting point. Should Spiral ship OPFS first
   (likely in chunk 4 §B already tracks) and defer the picker-based
   File System Access APIs to Phase 6, or build the picker-based APIs
   at the same time?

10. **Speech recognition.** W3C `SpeechRecognition` is not in any
    shipping browser except Chrome (where it is `webkitSpeechRecognition`
    and behind a flag). The vendor-prefixed shape is a real interop
    hazard. Is the speech recognition row above the target (spec
    conformance) or the realistic (Chromium-prefix) baseline?

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
