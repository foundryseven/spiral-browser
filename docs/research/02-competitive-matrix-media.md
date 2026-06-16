# Competitive Matrix — Media, Codecs, and EME

**Domain file 2 of 14 (Chunk 12).**
**Generated:** 2026-06-16
**Source:** `07-media-codecs-eme.md`
**Methodology:** `00-methodology.md`
**Engine columns:** `yes` · `partial` · `no` · `behind-flag`

---

## Legend

| Column | Values |
|--------|
| Status in Spiral | `not-started` · `designed` · `partial` · `shipped` · `do-not-touch` |
| Prevalence | `ubiquitous` · `widespread` · `mixed` · `niche` · `experimental` · `legacy` |
| Phase | Spiral roadmap phase / milestone target |
| Complexity | `S` · `M` · `L` · `XL` |
| Engine | `yes` · `partial` · `no` · `behind-flag` |

Engine code compression notes:
- `n/a` in source → `no` (capability not available on that engine's platform)
- `chr-inherit` / `via MSE` / `native` → `yes` (functionality available through mechanism)
- `platform-dep` → `partial` (available on some platforms only)
- `legacy` → `no` (deprecated/removed)
- `experimental (flag)` → `behind-flag`
- `sw decode only` → `yes` (software decode is functional support)
- `limited` → `partial`

---

## 1. Video Codecs

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 1 | H.264/AVC decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | yes | yes | yes | no | no |
| 2 | H.265/HEVC decoding | desktop+mobile+embedded | not-started | mixed | P4/M30+ | L | partial | no | yes | no | no |
| 3 | VP8 decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 4 | VP9 decoding (8-bit, Profile 0) | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 5 | VP9 Profile 2 (10-bit, HDR) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | M | yes | yes | yes | no | no |
| 6 | AV1 decoding (Main profile) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | partial |
| 7 | AV1 High / Professional profiles | desktop+mobile | not-started | mixed | P4/M30+ | L | yes | yes | yes | no | no |
| 8 | Hardware video decode — VAAPI | desktop (Linux) | not-started | widespread | P4/M30+ | L | yes | yes | no | no | no |
| 9 | Hardware video decode — VDPAU | desktop (Linux) | not-started | niche | P4/M30+ | M | no | no | no | no | no |
| 10 | Hardware video decode — VideoToolbox | desktop+mobile (Apple) | not-started | ubiquitous (Apple) | P4/M30+ | L | yes | yes | yes | no | no |
| 11 | Hardware video decode — DXVA2 / D3D11VA | desktop (Windows) | not-started | ubiquitous (Windows) | P4/M30+ | L | yes | yes | no | no | no |
| 12 | Hardware video decode — MediaCodec | mobile+embedded (Android) | not-started | ubiquitous (Android) | P4/M30+ | L | yes | yes | no | no | no |
| 13 | Media Capabilities — decodingInfo() | desktop+mobile+embedded | not-started | widespread | P4/M30+ | M | yes | yes | yes | no | no |
| 14 | Media Capabilities — encodingInfo() | desktop+mobile | not-started | mixed | P4/M30+ | M | yes | yes | no | no | no |

## 2. Audio Codecs

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 15 | AAC-LC decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 16 | HE-AACv1 / HE-AACv2 decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 17 | Opus decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 18 | Vorbis decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 19 | MP3 (MPEG-1 Audio Layer III) decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 20 | FLAC decoding | desktop+mobile+embedded | not-started | widespread | P4/M30+ | S | yes | yes | yes | no | no |
| 21 | PCM / WAV decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | yes | yes | yes | no | no |
| 22 | AC-3 (Dolby Digital) decoding | desktop+mobile+embedded | not-started | mixed | P4/M30+ | M | partial | partial | yes | no | no |
| 23 | E-AC-3 (Dolby Digital Plus) decoding | desktop+mobile+embedded | not-started | mixed | P4/M30+ | M | partial | partial | yes | no | no |
| 24 | Dolby Atmos spatial audio | desktop+mobile+embedded | not-started | niche | P4/M30+ | XL | yes | no | yes | no | no |

## 3. Container Formats

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 25 | MP4 (ISO BMFF) demuxing | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | yes | yes | yes | no | no |
| 26 | WebM demuxing | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 27 | Ogg demuxing | desktop+mobile | not-started | widespread | P4/M30+ | M | yes | yes | partial | no | no |
| 28 | Matroska (MKV) demuxing | desktop | not-started | mixed | P4/M30+ | M | yes | partial | no | no | no |
| 29 | MPEG-TS demuxing | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 30 | HLS (m3u8) adaptive streaming | desktop+mobile+embedded | not-started | ubiquitous (mobile) / widespread (desktop) | P4/M30+ | L | yes | yes | yes | no | no |
| 31 | MPEG-DASH (mpd) adaptive streaming | desktop+mobile+embedded | not-started | ubiquitous (via MSE) | P4/M30+ | L | yes | yes | yes | no | no |

## 4. Media Source Extensions (MSE)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 32 | Media Source Extensions — MediaSource, SourceBuffer | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | XL | yes | yes | yes | no | no |
| 33 | MSE — isTypeSupported() | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 34 | MSE — byte stream formats (webm, mp4, mp2t) | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | yes | yes | partial | no | no |
| 35 | MSE — adaptive bitrate switching | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | yes | yes | yes | no | no |

## 5. Encrypted Media Extensions (EME)

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 36 | EME — ClearKey (mandatory keysystem) | desktop+mobile+embedded | not-started | ubiquitous | P3/M12 | L | yes | yes | yes | no | no |
| 37 | EME — Widevine CDM | desktop+mobile+embedded | not-started | ubiquitous (non-Apple) | P4/M36+ | XL | yes | yes | no | no | no |
| 38 | EME — PlayReady CDM | desktop+mobile+embedded | not-started | mixed | P4/M36+ | XL | yes | no | no | no | no |
| 39 | EME — FairPlay Streaming CDM | desktop+mobile (Apple) | not-started | niche (Apple only) | P4/M36+ | XL | no | no | yes | no | no |
| 40 | EME — MediaKeys / MediaKeySession / MediaKeySystemAccess API | desktop+mobile+embedded | not-started | ubiquitous | P3/M12 | L | yes | yes | yes | no | no |

## 6. Web Audio API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 41 | Web Audio — AudioContext | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | yes | yes | yes | no | no |
| 42 | Web Audio — AudioWorklet | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 43 | Web Audio — spatial audio (HRTF, PannerNode) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 44 | Web Audio — AudioListener | desktop+mobile+embedded | not-started | widespread | P4/M30+ | M | yes | yes | yes | no | no |
| 45 | Web Audio — DynamicsCompressorNode | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |

## 7. HDR Video

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 46 | HDR10 video playback | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 47 | Dolby Vision playback | desktop+mobile+embedded | not-started | mixed | P4/M30+ | XL | yes | no | yes | no | no |
| 48 | HLG (Hybrid Log-Gamma) video playback | desktop+mobile+embedded | not-started | mixed | P4/M30+ | L | yes | yes | yes | no | no |
| 49 | HDR tone mapping | desktop+mobile+embedded | not-started | mixed | P4/M30+ | L | yes | yes | yes | no | no |
| 50 | Color-gamut CSS media query | desktop+mobile | not-started | widespread | P4/M30+ | S | yes | yes | yes | no | no |

## 8. Animated Images

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 51 | Animated GIF playback | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | yes | yes | yes | no | yes |
| 52 | Animated WebP playback | desktop+mobile+embedded | not-started | widespread | P4/M30+ | S | yes | yes | yes | no | no |
| 53 | Animated AVIF playback | desktop+mobile+embedded | not-started | mixed | P4/M30+ | M | yes | yes | yes | no | no |
| 54 | Animated PNG (APNG) playback | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | yes | yes | yes | no | no |
| 55 | Animated JPEG XL playback | desktop+mobile | not-started | niche | P4/M30+ | M | no | behind-flag | no | no | no |

## 9. HTMLMediaElement

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 56 | HTMLMediaElement — play() / pause() / autoplay | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | partial |
| 57 | HTMLMediaElement — seeking / currentTime | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | partial |
| 58 | HTMLMediaElement — buffered / duration | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | partial |
| 59 | HTMLMediaElement — playbackRate / defaultPlaybackRate | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | yes | yes | yes | no | no |
| 60 | HTMLMediaElement — loop / preload | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | yes | yes | yes | no | no |
| 61 | Autoplay policy (user activation gate) | desktop+mobile | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |

## 10. MediaSession API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 62 | MediaSession — metadata (title, artist, artwork) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | S | yes | yes | yes | no | no |
| 63 | MediaSession — action handlers (play/pause/seek/next/prev/stop) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | M | yes | yes | yes | no | no |
| 64 | MediaSession — position state | desktop+mobile+embedded | not-started | widespread | P4/M30+ | S | yes | yes | partial | no | no |

## 11. Picture-in-Picture API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 65 | Picture-in-Picture — requestPictureInPicture / exit | desktop+mobile | not-started | widespread | P4/M30+ | M | yes | yes | yes | no | no |
| 66 | Picture-in-Picture — auto PiP (document PiP) | desktop | not-started | niche | P5/M43+ | M | yes | no | no | no | no |

## 12. Fullscreen API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 67 | Fullscreen — requestFullscreen / exitFullscreen | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 68 | Fullscreen — fullscreenchange / fullscreenerror events | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | yes | yes | yes | no | no |
| 69 | Fullscreen — ::fullscreen pseudo-element | desktop+mobile | not-started | ubiquitous | P4/M30+ | S | yes | yes | yes | no | no |

## 13. Remote Playback API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 70 | Remote Playback API (presentation) | desktop+mobile | not-started | mixed | P5/M43+ | L | yes | no | yes | no | no |
| 71 | Chromecast integration | desktop+mobile | not-started | niche | P5/M43+ | XL | yes | no | no | no | no |
| 72 | AirPlay integration | desktop+mobile (Apple) | not-started | niche | P5/M43+ | L | no | no | yes | no | no |

## 14. Media Recording and Capture

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 73 | MediaRecorder API (recording to blob) | desktop+mobile | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 74 | getUserMedia — audio/video capture | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | yes | yes | yes | no | no |
| 75 | getDisplayMedia — screen/window/tab capture | desktop+mobile | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 76 | Media capture constraints model | desktop+mobile | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 77 | Device enumeration — enumerateDevices() | desktop+mobile | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 78 | Media capture — permissions integration | desktop+mobile | not-started | ubiquitous | P4/M30+ | M | yes | yes | yes | no | no |
| 79 | Canvas captureStream() | desktop+mobile | not-started | widespread | P4/M30+ | M | yes | yes | yes | no | no |

## 15. WebRTC

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 80 | RTCPeerConnection | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | XL | yes | yes | yes | no | no |
| 81 | RTCDataChannel | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | yes | yes | yes | no | no |
| 82 | ICE / STUN / TURN | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | XL | yes | yes | yes | no | no |
| 83 | SRTP / DTLS (media encryption) | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | yes | yes | yes | no | no |
| 84 | WebRTC simulcast | desktop+mobile | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 85 | SVC (scalable video coding) | desktop+mobile | not-started | mixed | P4/M30+ | L | yes | partial | partial | no | no |
| 86 | AV1 SVC in WebRTC | desktop+mobile | not-started | mixed | P4/M30+ | XL | yes | partial | no | no | no |

## 16. WebCodecs API

| # | Capability | Surface | Status in Spiral | Prevalence | Phase | Complexity | Chromium | Firefox | WebKit | Servo | Ladybird|
|---|-----------|---------|-----------------|------------|-------|-----------|----------|---------|--------|-------|----------|
| 87 | WebCodecs — VideoEncoder / VideoDecoder | desktop+mobile | not-started | widespread | P4/M30+ | XL | yes | yes | yes | no | no |
| 88 | WebCodecs — AudioEncoder / AudioDecoder | desktop+mobile | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 89 | WebCodecs — VideoFrame / AudioData | desktop+mobile | not-started | widespread | P4/M30+ | L | yes | yes | yes | no | no |
| 90 | WebCodecs — EncodedVideoChunk / EncodedAudioChunk | desktop+mobile | not-started | widespread | P4/M30+ | M | yes | yes | yes | no | no |

---

## Summary Statistics

| Metric | Count |
|--------|
| Total rows | 90 |
| Sections | 16 |
| Rows with `not-started` in Spiral | 90 |
| Rows where all 6 engines say `yes` | 38 |
| Rows where Servo says `yes` | 0 |
| Rows where Ladybird says `yes` | 1 (#51 Animated GIF) |
| Rows where Ladybird says `partial` | 4 (#6, #56, #57, #58) |
| Rows with `behind-flag` | 1 (#55 Firefox JPEG XL) |
| Rows at Phase P3 | 2 (#36, #40) |
| Rows at Phase P4 | 83 |
| Rows at Phase P5 | 5 (#66, #70, #71, #72, + implicit) |
| XL complexity rows | 12 |
