# Chunk 12 — Competitive Matrix: Web Platform APIs & Runtime

**Domain file:** `02-competitive-matrix-apis.md`
**Date:** 2026-06-16
**Source:** `06-web-platform-apis.md` §A (Web Platform APIs & Runtime)
**Methodology:** `00-methodology.md`
**Row count:** 162

**Engine column codes:**

| Code | Meaning |
|------|---------|
| `yes` | Ships in stable (or equivalent release channel for Servo/Ladybird/Flow) |
| `partial` | Incomplete implementation, prefixed, or limited subset only |
| `no` | Not implemented or removed |
| `behind-flag` | Available behind compile/runtime flag only |

**Notes:**
- `stub` in the source is mapped to `partial` (interface exists, behaviour incomplete).
- `yes (OT)` / `yes (Nightly)` / `yes (Safari TP)` in the source is mapped to `behind-flag` unless the entry explicitly states stable shipping.
- Section B (Storage & State) is in a separate file.
- Scoring per methodology §2 (six engines): Chromium (Blink), Firefox (Gecko), WebKit, Servo, Ladybird, Flow.

---

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird | Flow |
|---|-----------|---------|------------------|------------|-------|------------|----------|---------|--------|-------|----------|------|
| 1 | `fetch(input, init?)` global | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | yes | yes | yes | partial | partial | no |
| 2 | `Request` constructor | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | partial | no |
| 3 | `Response` constructor | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | partial | no |
| 4 | `Headers` constructor | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | partial | no |
| 5 | `Body` mixin (`arrayBuffer` / `blob` / `formData` / `json` / `text`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | partial | no |
| 6 | `AbortController` / `AbortSignal` | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | no | no |
| 7 | `AbortSignal.timeout(ms)` | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 8 | `AbortSignal.any(signals)` | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 9 | Fetch priority hints (`fetchPriority` / `priority` header) | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 10 | Fetch keepalive | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 11 | Fetch redirect modes (`follow` / `error` / `manual`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | partial | no |
| 12 | Fetch credentials modes (`omit` / `same-origin` / `include`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | partial | no |
| 13 | Fetch `mode` (`cors` / `no-cors` / `same-origin` / `navigate`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | partial | no |
| 14 | Fetch `referrerPolicy` | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | partial | no |
| 15 | Duplex streaming (`request.duplex: "half"`) | desktop+mobile+embedded | not-started | niche | P4 | M | yes | partial | no | no | no | no |
| 16 | `ReadableStream` constructor / default reader | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | yes | yes | yes | partial | partial | no |
| 17 | `WritableStream` constructor / default writer | desktop+mobile+embedded | not-started | ubiquitous | P4 | L | yes | yes | yes | partial | partial | no |
| 18 | `TransformStream` | desktop+mobile+embedded | not-started | widespread | P4 | M | yes | yes | yes | partial | no | no |
| 19 | `ReadableStreamBYOBReader` (bring-your-own-buffer) | desktop+mobile+embedded | not-started | widespread | P4 | M | yes | yes | yes | partial | no | no |
| 20 | `ReadableStream.pipeTo` / `pipeThrough` | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | no | no |
| 21 | `ReadableStream.tee()` | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 22 | Stream backpressure (high-water mark, pull, write queuing) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | no | no |
| 23 | `ByteLengthQueuingStrategy` / `CountQueuingStrategy` | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 24 | `Worker` constructor (dedicated worker) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | partial | no | no |
| 25 | `SharedWorker` constructor | desktop+mobile+embedded | not-started | widespread | P6 | L | yes | yes | yes | no | no | no |
| 26 | Service Worker registration (`navigator.serviceWorker.register`) | desktop+mobile+embedded | not-started | ubiquitous | P5 | XL | yes | yes | yes | no | no | no |
| 27 | `ServiceWorkerContainer` / `ServiceWorkerRegistration` | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | no | no | no |
| 28 | `importScripts(url, ...)` in workers | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | yes | yes | yes | no | no | no |
| 29 | `postMessage` / `onmessage` (worker messaging) | desktop+mobile+embedded | not-started | ubiquitous | P5 | S | yes | yes | yes | partial | no | no |
| 30 | `structuredClone` algorithm (worker message body) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 31 | `WorkerGlobalScope` (self, location, navigator, caches) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 32 | `CacheStorage` (`caches.open`) inside service worker | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | no | no | no |
| 33 | OPFS sync access handle inside worker (`createSyncAccessHandle`) | desktop+mobile+embedded | not-started | niche | P5 | M | yes | yes | no | no | no | no |
| 34 | Worker module scripts (`new Worker(url, {type:"module"})`) | desktop+mobile+embedded | not-started | widespread | P5 | M | yes | yes | yes | no | no | no |
| 35 | Service Worker update flow (`update`, `skipWaiting`, `clients.claim`) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | no | no | no |
| 36 | `requestIdleCallback` / `cancelIdleCallback` | desktop+mobile | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 37 | `requestAnimationFrame` / `cancelAnimationFrame` (timing only) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | partial | no |
| 38 | Prioritised Task Scheduling API (`scheduler.postTask`, `scheduler.yield`) | desktop+mobile+embedded | not-started | niche | P4 | M | yes | no | no | no | no | no |
| 39 | Task priorities (`user-blocking` / `user-visible` / `background`) | desktop+mobile+embedded | not-started | niche | P4 | S | yes | no | no | no | no | no |
| 40 | `isInputPending()` (input-priority scheduling) | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | no | no | no | no | no |
| 41 | WebAssembly `WebAssembly.Module` | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | partial | yes |
| 42 | `WebAssembly.Instance` | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | partial | yes |
| 43 | `WebAssembly.Memory` (shared / non-shared) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | yes | partial | yes |
| 44 | `WebAssembly.Table` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 45 | `WebAssembly.Global` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 46 | `WebAssembly.compile` / `compileStreaming` (off-thread) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 47 | `WebAssembly.instantiate` / `instantiateStreaming` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 48 | Wasm reference types (externref, funcref) | desktop+mobile+embedded | not-started | widespread | P3 | M | yes | yes | yes | yes | partial | yes |
| 49 | Wasm SIMD (`v128` ops) | desktop+mobile+embedded | not-started | widespread | P3 | M | yes | yes | yes | partial | partial | partial |
| 50 | Wasm threads (atomic memory ops, `memory.shared`) | desktop+mobile+embedded | not-started | widespread | P3 | M | yes | yes | yes | partial | partial | yes |
| 51 | Wasm exception handling (`try` / `catch` / `throw` / `rethrow`) | desktop+mobile+embedded | not-started | widespread | P3 | M | yes | yes | yes | partial | partial | yes |
| 52 | Wasm tail calls (`return_call` / `return_call_indirect`) | desktop+mobile+embedded | not-started | niche | P3 | S | yes | yes | yes | no | no | partial |
| 53 | Wasm multi-value returns | desktop+mobile+embedded | not-started | widespread | P3 | S | yes | yes | yes | yes | partial | yes |
| 54 | Wasm bulk memory ops (`memory.copy` / `memory.fill`) | desktop+mobile+embedded | not-started | widespread | P3 | S | yes | yes | yes | yes | partial | yes |
| 55 | Wasm relaxed SIMD | desktop+mobile+embedded | not-started | niche | P3 | S | yes | yes | yes | no | no | partial |
| 56 | Wasm memory64 (>4 GiB address space) | desktop+mobile+embedded | not-started | niche | P3 | M | yes | yes | yes | no | no | partial |
| 57 | Wasm GC (struct, array, ref types, gc-managed objects) | desktop+mobile+embedded | not-started | niche | P4 | XL | yes | yes | no | no | no | no |
| 58 | Wasm component model + WASI preview | desktop+mobile+embedded | not-started | niche | P5 | XL | partial | no | no | no | no | no |
| 59 | WebGPU `GPUAdapter` / `GPUDevice` | desktop+mobile | not-started | widespread | P6 | XL | yes | behind-flag | behind-flag | no | no | no |
| 60 | `GPUBuffer` / `GPUTexture` / `GPUTextureView` | desktop+mobile | not-started | widespread | P6 | L | yes | behind-flag | behind-flag | no | no | no |
| 61 | `GPURenderPipeline` / `GPUComputePipeline` | desktop+mobile | not-started | widespread | P6 | L | yes | behind-flag | behind-flag | no | no | no |
| 62 | `GPUBindGroup` / `GPUBindGroupLayout` | desktop+mobile | not-started | widespread | P6 | M | yes | behind-flag | behind-flag | no | no | no |
| 63 | WGSL shader language | desktop+mobile | not-started | widespread | P6 | L | yes | behind-flag | behind-flag | no | no | no |
| 64 | `GPUCanvasContext` (compositor hook from `<canvas>`) | desktop+mobile | not-started | widespread | P6 | M | yes | behind-flag | behind-flag | no | no | no |
| 65 | `GPUCommandEncoder` / pass encoders | desktop+mobile | not-started | widespread | P6 | L | yes | behind-flag | behind-flag | no | no | no |
| 66 | WebGL 1.0 (`WebGLRenderingContext`) | desktop+mobile+embedded | not-started | ubiquitous | P6 | L | yes | yes | yes | yes | partial | yes |
| 67 | WebGL 2.0 (`WebGL2RenderingContext`) | desktop+mobile+embedded | not-started | widespread | P6 | L | yes | yes | yes | partial | partial | yes |
| 68 | WebGL extensions (compressed textures, instancing, transform feedback) | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | partial | partial | yes |
| 69 | ANGLE as default WebGL backend | desktop+mobile+embedded | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |
| 70 | WebNN `MLContext` / `MLGraphBuilder` | desktop+mobile | not-started | niche | P6 | XL | behind-flag | no | no | no | no | no |
| 71 | WebNN operations (`conv2d`, `matmul`, `softmax`, `relu`, `pooling`) | desktop+mobile | not-started | niche | P6 | L | behind-flag | no | no | no | no | no |
| 72 | WebNN operand types (`float32` / `int32` / `int8` / `uint4`) | desktop+mobile | not-started | niche | P6 | M | behind-flag | no | no | no | no | no |
| 73 | WebNN MLGraph compilation + dispatch | desktop+mobile | not-started | niche | P6 | L | behind-flag | no | no | no | no | no |
| 74 | WebAuthn `navigator.credentials.create({publicKey})` | desktop+mobile | not-started | widespread | P5 | L | yes | yes | yes | no | no | no |
| 75 | WebAuthn `navigator.credentials.get({publicKey})` | desktop+mobile | not-started | widespread | P5 | L | yes | yes | yes | no | no | no |
| 76 | WebAuthn resident keys (`requireResidentKey`, `residentKey`) | desktop+mobile | not-started | widespread | P5 | S | yes | yes | yes | no | no | no |
| 77 | WebAuthn user verification (`userVerification: "required"`) | desktop+mobile | not-started | widespread | P5 | S | yes | yes | yes | no | no | no |
| 78 | WebAuthn attestation (direct / indirect / none) | desktop+mobile | not-started | widespread | P5 | M | yes | yes | yes | no | no | no |
| 79 | Payment Request `PaymentRequest` constructor | desktop+mobile | not-started | widespread | P6 | L | yes | yes | yes | no | no | no |
| 80 | `PaymentResponse` / `PaymentMethodData` | desktop+mobile | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |
| 81 | Payment Request shipping address + contact info events | desktop+mobile | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |
| 82 | `canMakePayment()` | desktop+mobile | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 83 | `navigator.share()` (Web Share API) | desktop+mobile | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 84 | `navigator.canShare()` | desktop+mobile | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 85 | Share target (handler registration in web app manifest) | mobile | not-started | niche | P6 | M | yes | no | no | no | no | no |
| 86 | Async Clipboard (`navigator.clipboard.readText` / `writeText`) | desktop+mobile+embedded | not-started | widespread | P4 | M | yes | yes | yes | no | no | no |
| 87 | Clipboard `read()` / `write()` (arbitrary MIME, images) | desktop+mobile | not-started | niche | P4 | M | yes | partial | yes | no | no | no |
| 88 | Clipboard permission model (transient activation gating) | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 89 | File System Access `showOpenFilePicker` | desktop | not-started | niche | P6 | L | yes | no | no | no | no | no |
| 90 | File System Access `showSaveFilePicker` | desktop | not-started | niche | P6 | L | yes | no | no | no | no | no |
| 91 | File System Access `showDirectoryPicker` | desktop | not-started | niche | P6 | L | yes | no | no | no | no | no |
| 92 | `FileSystemFileHandle` / `FileSystemDirectoryHandle` | desktop | not-started | niche | P6 | M | yes | no | no | no | no | no |
| 93 | `createWritable()` (file handle → writable stream) | desktop | not-started | niche | P6 | S | yes | no | no | no | no | no |
| 94 | Notifications `Notification` constructor | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no | no |
| 95 | Notification permission model (`granted` / `denied` / `default`) | desktop+mobile | not-started | ubiquitous | P4 | S | yes | yes | yes | no | no | no |
| 96 | Notification `actions` (button array) | desktop+mobile | not-started | widespread | P4 | S | yes | yes | no | no | no | no |
| 97 | Notification `badge` / `image` / `vibrate` | desktop+mobile | not-started | widespread | P4 | S | yes | yes | partial | no | no | no |
| 98 | Web Push (`PushManager.subscribe`, `PushSubscription`) | desktop+mobile+embedded | not-started | widespread | P5 | L | yes | yes | yes | no | no | no |
| 99 | Push event + VAPID keys | desktop+mobile+embedded | not-started | widespread | P5 | M | yes | yes | yes | no | no | no |
| 100 | WebTransport (`new WebTransport(url)`) | desktop+mobile | not-started | niche | P6 | L | yes | yes | behind-flag | no | no | no |
| 101 | WebTransport datagrams (unreliable, low-latency) | desktop+mobile | not-started | niche | P6 | M | yes | yes | behind-flag | no | no | no |
| 102 | WebTransport bidirectional + unidirectional streams | desktop+mobile | not-started | niche | P6 | M | yes | yes | behind-flag | no | no | no |
| 103 | WebSocket (`new WebSocket(url, protocols)`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | partial | yes |
| 104 | WebSocket binary types (`blob` / `arraybuffer`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | partial | yes |
| 105 | WebSocket close codes (1000-4999) | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | yes | partial | yes |
| 106 | WebSocket subprotocol negotiation | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | yes | partial | yes |
| 107 | Gamepad (`navigator.getGamepads()`, `GamepadEvent`) | desktop+mobile | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |
| 108 | Gamepad haptic feedback (vibration actuator) | desktop | not-started | niche | P6 | S | yes | yes | no | no | no | no |
| 109 | SpeechSynthesis (`speechSynthesis.speak`) | desktop+mobile | not-started | widespread | P6 | M | yes | yes | yes | no | no | no |
| 110 | SpeechSynthesis voices + onvoiceschanged | desktop+mobile | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 111 | SpeechRecognition (`new SpeechRecognition()`) | desktop+mobile | not-started | niche | P6 | L | partial | no | yes | no | no | no |
| 112 | Pointer Events (unified mouse / touch / pen) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | partial | yes |
| 113 | Touch Events (`touchstart` / `touchmove` / `touchend`) | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | partial | yes |
| 114 | Mouse Events (click / mousedown / mousemove / wheel) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | partial | yes |
| 115 | Keyboard Events (keydown / keyup + `code` / `key` / `keyCode`) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | partial | partial | yes |
| 116 | Battery Status (`navigator.getBattery()`) | desktop+mobile | not-started | niche | P6 | S | partial | yes | no | no | no | no |
| 117 | Vibration (`navigator.vibrate(pattern)`) | desktop+mobile | not-started | niche | P6 | S | yes | yes | no | no | no | no |
| 118 | Geolocation (`navigator.geolocation.getCurrentPosition`) | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 119 | Geolocation `watchPosition` / `clearWatch` | desktop+mobile+embedded | not-started | ubiquitous | P5 | M | yes | yes | yes | partial | no | no |
| 120 | Generic Sensor: Accelerometer | mobile+embedded | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 121 | Generic Sensor: Gyroscope | mobile+embedded | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 122 | Generic Sensor: Magnetometer | mobile+embedded | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 123 | Generic Sensor: AmbientLightSensor | mobile+embedded | not-started | widespread | P6 | S | yes | yes | yes | no | no | no |
| 124 | Generic Sensor: Barometer (`PressureSensor`) | mobile+embedded | not-started | niche | P6 | S | yes | yes | no | no | no | no |
| 125 | Web MIDI (`navigator.requestMIDIAccess()`) | desktop | not-started | niche | P6 | M | yes | yes | no | no | no | no |
| 126 | Web Serial (`navigator.serial.requestPort()`) | desktop | not-started | niche | P6 | M | yes | no | no | no | no | no |
| 127 | WebUSB (`navigator.usb.requestDevice()`) | desktop | not-started | niche | P6 | M | yes | no | no | no | no | no |
| 128 | Web Bluetooth (`navigator.bluetooth.requestDevice()`) | desktop+mobile+embedded | not-started | niche | P6 | M | yes | no | no | no | no | no |
| 129 | Web NFC (`new NDEFReader()`) | mobile | not-started | niche | P6 | M | yes | no | no | no | no | no |
| 130 | WebCodecs `VideoEncoder` / `VideoDecoder` | desktop+mobile | not-started | widespread | P5 | L | yes | yes | yes | no | no | no |
| 131 | WebCodecs `AudioEncoder` / `AudioDecoder` | desktop+mobile | not-started | widespread | P5 | L | yes | yes | yes | no | no | no |
| 132 | WebCodecs `VideoFrame` / `AudioData` | desktop+mobile | not-started | widespread | P5 | L | yes | yes | yes | no | no | no |
| 133 | WebCodecs `ImageDecoder` (animated images) | desktop+mobile | not-started | widespread | P5 | M | yes | yes | yes | no | no | no |
| 134 | WebXR `XRSystem` / `XRSession` | desktop+mobile | not-started | niche | P6 | XL | yes | yes | yes | no | no | no |
| 135 | WebXR `immersive-vr` mode | desktop+mobile | not-started | niche | P6 | L | yes | yes | no | no | no | no |
| 136 | WebXR `immersive-ar` mode (camera passthrough) | mobile | not-started | niche | P6 | L | yes | no | yes | no | no | no |
| 137 | WebXR `inline` mode (2D viewport) | desktop+mobile | not-started | niche | P6 | M | yes | yes | yes | no | no | no |
| 138 | WebXR hand input + hit test + anchors | mobile | not-started | niche | P6 | L | yes | no | yes | no | no | no |
| 139 | WebXR plane + mesh detection | mobile | not-started | niche | P6 | L | yes | no | yes | no | no | no |
| 140 | WebXR depth sensing + layers + DOM overlay | mobile | not-started | niche | P6 | M | yes | no | yes | no | no | no |
| 141 | Compression Streams (`new CompressionStream("gzip")`) | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 142 | Decompression Streams (`new DecompressionStream("gzip")`) | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 143 | Compression formats (`gzip` / `deflate` / `deflate-raw` / `br` / `zstd`) | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 144 | `TextEncoder` / `TextDecoder` (UTF-8 default) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 145 | `TextEncoderStream` / `TextDecoderStream` | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 146 | `URL` constructor + `URL.parse` / `URL.canParse` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 147 | `URLSearchParams` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 148 | `Blob` constructor + `slice` | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | partial | yes |
| 149 | `File` constructor + name, type, lastModified | desktop+mobile+embedded | not-started | ubiquitous | P4 | S | yes | yes | yes | partial | partial | yes |
| 150 | `FileReader` / `FileList` / blob URL | desktop+mobile+embedded | not-started | ubiquitous | P4 | M | yes | yes | yes | partial | partial | yes |
| 151 | `FileReaderSync` (worker-only) | desktop+mobile+embedded | not-started | widespread | P5 | S | yes | yes | yes | no | no | no |
| 152 | `structuredClone(value)` (global) | desktop+mobile+embedded | not-started | ubiquitous | P3 | M | yes | yes | yes | partial | no | yes |
| 153 | `atob` / `btoa` (base64 helpers) | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 154 | `crypto.getRandomValues(buffer)` | desktop+mobile+embedded | not-started | ubiquitous | P3 | S | yes | yes | yes | yes | partial | yes |
| 155 | `crypto.randomUUID()` | desktop+mobile+embedded | not-started | widespread | P3 | S | yes | yes | yes | no | no | yes |
| 156 | `crypto.subtle` (SubtleCrypto: digest, encrypt, sign, deriveKey) | desktop+mobile+embedded | not-started | ubiquitous | P5 | L | yes | yes | yes | partial | no | yes |
| 157 | SubtleCrypto ECDSA + Ed25519 + RSA-PSS | desktop+mobile+embedded | not-started | widespread | P5 | M | yes | yes | yes | partial | no | yes |
| 158 | SubtleCrypto AES-GCM + AES-CTR + ChaCha20-Poly1305 | desktop+mobile+embedded | not-started | widespread | P5 | M | yes | yes | yes | partial | no | yes |
| 159 | SubtleCrypto ECDH + HKDF + PBKDF2 (key derivation) | desktop+mobile+embedded | not-started | widespread | P5 | M | yes | yes | yes | partial | no | yes |
| 160 | `navigator.permissions.query({name})` | desktop+mobile+embedded | not-started | widespread | P4 | S | yes | yes | yes | no | no | no |
| 161 | `navigator.permissions.revoke` | desktop+mobile+embedded | not-started | niche | P4 | S | yes | no | no | no | no | no |
| 162 | Push permission (`name: "push"`, with userVisibleOnly) | desktop+mobile+embedded | not-started | widespread | P5 | S | yes | yes | yes | no | no | no |
