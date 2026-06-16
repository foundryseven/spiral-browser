# Chunk 5 — Media, Codecs, and EME

**Chunk 5 of 14.** This file inventories media infrastructure, codec
support, container formats, DRM/EME, media playback and capture APIs,
WebRTC, WebCodecs, HDR video, spatial audio, and animated image formats.

---

## Scope

**In:**
Video codecs (H.264, H.265/HEVC, VP8, VP9, AV1, profiles and hardware
decode), audio codecs (AAC, Opus, Vorbis, MP3, FLAC, PCM, AC-3, E-AC-3,
Dolby Atmos), container formats (MP4, WebM, Ogg, MKV, MPEG-TS, HLS,
DASH), MSE, EME (ClearKey, Widevine, PlayReady, FairPlay), Media
Capabilities API, Web Audio API, HTMLMediaElement, MediaSession,
Picture-in-Picture, Fullscreen, Remote Playback, autoplay policy,
MediaRecorder, MediaStream, getUserMedia, getDisplayMedia, WebRTC,
WebCodecs, HDR video, spatial audio, animated images.

**Out:**
Non-animated image decoding (chunk 1 / chunk 6), Canvas 2D/WebGL/WebGPU
rendering of media (chunk 6), media in Service Workers (chunk 2/6),
media in extensions (chunk 10), media distribution / DRM packaging
(chunk 11).

---

## Methodology for this chunk

Rows are drawn from:
- WHATWG HTML spec §4.7 (`html.spec.whatwg.org`) for HTMLMediaElement and
  media elements.
- W3C EME Recommendation (`w3.org/TR/encrypted-media/`), MSE
  (`w3.org/TR/media-source/`), Media Capabilities
  (`w3.org/TR/media-capabilities/`), WebRTC
  (`w3.org/TR/webrtc/`), WebCodecs (`w3.org/TR/webcodecs/`),
  MediaRecorder (`w3.org/TR/mediastream-recording/`),
  getUserMedia (`w3.org/TR/mediacapture-streams/`),
  Picture-in-Picture (`w3.org/TR/picture-in-picture/`),
  Fullscreen (`w3.org/TR/fullscreen/`),
  Remote Playback (`w3.org/TR/remote-playback/`),
  Web Audio (`w3.org/TR/webaudio/`).
- IETF RFC 6716 (Opus), RFC 7845 (Ogg).
- ISO 14496-3 (AAC), ISO 14496-10 (AVC), ISO 23008-2 (HEVC).
- AOM AV1 Bitstream Specification.
- MDN Baseline tables and Can I Use for prevalence.
- Engine release notes and Chrome Platform Status for per-engine
  shipping status.

---

## Inventory

### Video codecs

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 1 | H.264/AVC decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes (chr-inherit). | WHATWG HTML §4.7; ISO 14496-10; MDN |
| 2 | H.265/HEVC decoding | desktop+mobile+embedded | not-started | mixed | P4/M30+ | L | Chr: partial (hw-dep, platform codec). FX: no (default). WK: yes (VideoToolbox). Servo: no. Ladybird: no. Flow: yes (chr-inherit). | ISO 23008-2; MDN; Can I Use |
| 3 | VP8 decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | RFC 6386; WHATWG HTML §4.7; MDN |
| 4 | VP9 decoding (8-bit, Profile 0) | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes (Safari 15+). Servo: no. Ladybird: no. Flow: yes. | VP9 bitstream spec; MDN; Can I Use |
| 5 | VP9 Profile 2 (10-bit, HDR) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | VP9 bitstream spec; MDN |
| 6 | AV1 decoding (Main profile) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes (Safari 17+). Servo: no. Ladybird: partial (sw, new). Flow: yes (chr-inherit). | AOM AV1 spec; MDN; Can I Use |
| 7 | AV1 High / Professional profiles | desktop+mobile | not-started | mixed | P4/M30+ | L | Chr: sw decode yes. FX: sw decode yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | AOM AV1 spec §6 |
| 8 | Hardware video decode — VAAPI | desktop (Linux) | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: n/a. Servo: no. Ladybird: no. Flow: yes. | VA-API spec (intel.github.io); MDN |
| 9 | Hardware video decode — VDPAU | desktop (Linux) | not-started | niche | P4/M30+ | M | Chr: legacy. FX: legacy. WK: n/a. Servo: no. Ladybird: no. Flow: n/a. | VDPAU spec (freedesktop.org) |
| 10 | Hardware video decode — VideoToolbox | desktop+mobile (Apple) | not-started | ubiquitous (Apple) | P4/M30+ | L | Chr: yes (macOS). FX: yes. WK: yes (primary). Servo: no. Ladybird: no. Flow: yes. | Apple Developer docs; VTDecompressionSession |
| 11 | Hardware video decode — DXVA2 / D3D11VA | desktop (Windows) | not-started | ubiquitous (Windows) | P4/M30+ | L | Chr: yes. FX: yes. WK: n/a. Servo: no. Ladybird: no. Flow: yes. | Microsoft DXVA2 spec; MDN |
| 12 | Hardware video decode — MediaCodec | mobile+embedded (Android) | not-started | ubiquitous (Android) | P4/M30+ | L | Chr: yes. FX: yes. WK: n/a. Servo: no. Ladybird: no. Flow: yes. | Android MediaCodec API; developer.android.com |
| 13 | Media Capabilities — decodingInfo() | desktop+mobile+embedded | not-started | widespread | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C media-capabilities; MDN |
| 14 | Media Capabilities — encodingInfo() | desktop+mobile | not-started | mixed | P4/M30+ | M | Chr: yes. FX: yes. WK: no. Servo: no. Ladybird: no. Flow: yes. | W3C media-capabilities §5 |

### Audio codecs

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 15 | AAC-LC decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes (platform). FX: yes (platform). WK: yes (CoreAudio). Servo: no. Ladybird: no. Flow: yes. | ISO 14496-3; MDN |
| 16 | HE-AACv1 / HE-AACv2 decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | ISO 14496-3; 3GPP TS 26.401 |
| 17 | Opus decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | RFC 6716; WHATWG HTML §4.7; MDN |
| 18 | Vorbis decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | xiph.org vorbis spec; MDN |
| 19 | MP3 (MPEG-1 Audio Layer III) decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | ISO 11172-3; MDN |
| 20 | FLAC decoding | desktop+mobile+embedded | not-started | widespread | P4/M30+ | S | Chr: yes. FX: yes. WK: yes (Safari 11+). Servo: no. Ladybird: no. Flow: yes. | xiph.org FLAC spec; MDN; Can I Use |
| 21 | PCM / WAV decoding | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | WHATWG HTML §4.7; RIFF/WAVE |
| 22 | AC-3 (Dolby Digital) decoding | desktop+mobile+embedded | not-started | mixed | P4/M30+ | M | Chr: platform-dep. FX: platform-dep. WK: yes (Apple HW). Servo: no. Ladybird: no. Flow: chr-inherit. | ATSC A/52; platform vendor docs |
| 23 | E-AC-3 (Dolby Digital Plus) decoding | desktop+mobile+embedded | not-started | mixed | P4/M30+ | M | Chr: platform-dep. FX: platform-dep. WK: yes (Apple HW). Servo: no. Ladybird: no. Flow: chr-inherit. | ATSC A/52; platform vendor docs |
| 24 | Dolby Atmos spatial audio | desktop+mobile+embedded | not-started | niche | P4/M30+ | XL | Chr: yes (Android, EME). FX: no. WK: yes (Apple HW + EME). Servo: no. Ladybird: no. Flow: chr-inherit. | Dolby spec; EME integration |

### Container formats

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 25 | MP4 (ISO BMFF) demuxing | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | ISO 14496-12; WHATWG MSE byte stream format |
| 26 | WebM demuxing | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | WebM spec (webmproject.org); MSE |
| 27 | Ogg demuxing | desktop+mobile | not-started | widespread | P4/M30+ | M | Chr: yes. FX: yes. WK: partial (no MSE). Servo: no. Ladybird: no. Flow: yes. | RFC 7845; RFC 3533 |
| 28 | Matroska (MKV) demuxing | desktop | not-started | mixed | P4/M30+ | M | Chr: yes. FX: limited. WK: no. Servo: no. Ladybird: no. Flow: yes. | Matroska spec (matroska.org) |
| 29 | MPEG-TS demuxing | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | ISO 13818-1; MSE mp2t bytestream |
| 30 | HLS (m3u8) adaptive streaming | desktop+mobile+embedded | not-started | ubiquitous (mobile) / widespread (desktop) | P4/M30+ | L | Chr: native on Android; via MSE elsewhere. FX: via MSE. WK: native (Apple). Servo: no. Ladybird: no. Flow: chr-inherit. | RFC 8216; Apple HLS spec |
| 31 | MPEG-DASH (mpd) adaptive streaming | desktop+mobile+embedded | not-started | ubiquitous (via MSE) | P4/M30+ | L | Chr: via MSE. FX: via MSE. WK: via MSE. Servo: no. Ladybird: no. Flow: yes. | DASH-IF spec; ISO 23009-1 |

### Media Source Extensions (MSE)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 32 | Media Source Extensions — MediaSource, SourceBuffer | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | XL | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C media-source; MDN |
| 33 | MSE — isTypeSupported() | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C media-source §2 |
| 34 | MSE — byte stream formats (webm, mp4, mp2t) | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | Chr: mp4+webm+mp2t. FX: mp4+webm+mp2t. WK: mp4+webm. Servo: no. Ladybird: no. Flow: yes. | W3C byte-stream-format-registry |
| 35 | MSE — adaptive bitrate switching | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C media-source §3.5 |

### Encrypted Media Extensions (EME)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 36 | EME — ClearKey (mandatory keysystem) | desktop+mobile+embedded | not-started | ubiquitous | P3/M12 | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C encrypted-media §6.1 |
| 37 | EME — Widevine CDM | desktop+mobile+embedded | not-started | ubiquitous (non-Apple) | P4/M36+ | XL | Chr: yes (built-in). FX: yes (CDM binary). WK: n/a. Servo: no. Ladybird: no. Flow: yes. | Widevine docs; W3C EME §6 |
| 38 | EME — PlayReady CDM | desktop+mobile+embedded | not-started | mixed | P4/M36+ | XL | Chr: yes (Edge/Windows). FX: no. WK: n/a. Servo: no. Ladybird: no. Flow: chr-inherit. | Microsoft PlayReady docs |
| 39 | EME — FairPlay Streaming CDM | desktop+mobile (Apple) | not-started | niche (Apple only) | P4/M36+ | XL | Chr: n/a. FX: n/a. WK: yes. Servo: no. Ladybird: no. Flow: n/a. | Apple FairPlay docs; W3C EME |
| 40 | EME — MediaKeys / MediaKeySession / MediaKeySystemAccess API | desktop+mobile+embedded | not-started | ubiquitous | P3/M12 | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C encrypted-media §5 |

### Web Audio API

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 41 | Web Audio — AudioContext | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webaudio §1; MDN |
| 42 | Web Audio — AudioWorklet | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webaudio §9; MDN |
| 43 | Web Audio — spatial audio (HRTF, PannerNode) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webaudio §7 (SpatialPanner, HRTF) |
| 44 | Web Audio — AudioListener | desktop+mobile+embedded | not-started | widespread | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webaudio §7.2 |
| 45 | Web Audio — DynamicsCompressorNode | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webaudio §7.12 |

### HDR video

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 46 | HDR10 video playback | desktop+mobile+embedded | not-started | widespread | P4/M30+ | L | Chr: yes (via codec+hw). FX: yes (platform-dep). WK: yes. Servo: no. Ladybird: no. Flow: yes. | CTA-861.3; SMPTE ST 2084 |
| 47 | Dolby Vision playback | desktop+mobile+embedded | not-started | mixed | P4/M30+ | XL | Chr: yes (Android). FX: no. WK: yes (Apple HW). Servo: no. Ladybird: no. Flow: chr-inherit. | Dolby spec; platform vendor docs |
| 48 | HLG (Hybrid Log-Gamma) video playback | desktop+mobile+embedded | not-started | mixed | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | ARIB STD-B67; ITU-R BT.2100 |
| 49 | HDR tone mapping | desktop+mobile+embedded | not-started | mixed | P4/M30+ | L | Chr: yes (auto). FX: yes (auto). WK: yes (auto). Servo: no. Ladybird: no. Flow: yes. | Platform compositor; CSS color-gamut |
| 50 | Color-gamut CSS media query | desktop+mobile | not-started | widespread | P4/M30+ | S | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | CSS Media Queries 4 §11; MDN |

### Animated images (media context)

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 51 | Animated GIF playback | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: yes (basic). Flow: yes. | GIF89a spec; MDN |
| 52 | Animated WebP playback | desktop+mobile+embedded | not-started | widespread | P4/M30+ | S | Chr: yes. FX: yes. WK: yes (Safari 14+). Servo: no. Ladybird: no. Flow: yes. | WebP spec (webmproject.org); MDN |
| 53 | Animated AVIF playback | desktop+mobile+embedded | not-started | mixed | P4/M30+ | M | Chr: yes. FX: yes (Firefox 93+). WK: yes (Safari 16.4+). Servo: no. Ladybird: no. Flow: yes. | AOM AVIF spec; MDN |
| 54 | Animated PNG (APNG) playback | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | Chr: yes. FX: yes (first mover). WK: yes. Servo: no. Ladybird: no. Flow: yes. | APNG spec (wiki.mozilla.org); MDN |
| 55 | Animated JPEG XL playback | desktop+mobile | not-started | niche | P4/M30+ | M | Chr: no (removed). FX: experimental (flag). WK: no. Servo: no. Ladybird: no. Flow: no. | JPEG XL spec (ISO 18181); MDN |

### HTMLMediaElement

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 56 | HTMLMediaElement — play() / pause() / autoplay | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: partial. Flow: yes. | WHATWG HTML §4.7.10; MDN |
| 57 | HTMLMediaElement — seeking / currentTime | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: partial. Flow: yes. | WHATWG HTML §4.7.10 |
| 58 | HTMLMediaElement — buffered / duration | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: partial. Flow: yes. | WHATWG HTML §4.7.10 |
| 59 | HTMLMediaElement — playbackRate / defaultPlaybackRate | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | WHATWG HTML §4.7.10 |
| 60 | HTMLMediaElement — loop / preload | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | WHATWG HTML §4.7.10 |
| 61 | Autoplay policy (user activation gate) | desktop+mobile | not-started | ubiquitous | P4/M30+ | M | Chr: yes (strict). FX: yes (block w/o interaction). WK: yes (block w/o gesture on mobile). Servo: no. Ladybird: no. Flow: yes. | WHATWG HTML §4.7.10; Permissions Policy |

### MediaSession API

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 62 | MediaSession — metadata (title, artist, artwork) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | S | Chr: yes. FX: yes. WK: yes (Safari 15+). Servo: no. Ladybird: no. Flow: yes. | W3C mediasession §4; MDN |
| 63 | MediaSession — action handlers (play/pause/seek/next/prev/stop) | desktop+mobile+embedded | not-started | widespread | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C mediasession §5 |
| 64 | MediaSession — position state | desktop+mobile+embedded | not-started | widespread | P4/M30+ | S | Chr: yes. FX: yes. WK: partial. Servo: no. Ladybird: no. Flow: yes. | W3C mediasession §6 |

### Picture-in-Picture API

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 65 | Picture-in-Picture — requestPictureInPicture / exit | desktop+mobile | not-started | widespread | P4/M30+ | M | Chr: yes. FX: yes (Firefox 116+). WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C picture-in-picture; MDN |
| 66 | Picture-in-Picture — auto PiP (document PiP) | desktop | not-started | niche | P5/M43+ | M | Chr: yes (Document PiP). FX: no. WK: no. Servo: no. Ladybird: no. Flow: chr-inherit. | Chrome Platform Status |

### Fullscreen API

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 67 | Fullscreen — requestFullscreen / exitFullscreen | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C fullscreen; WHATWG HTML §7.5 |
| 68 | Fullscreen — fullscreenchange / fullscreenerror events | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | S | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C fullscreen §4 |
| 69 | Fullscreen — ::fullscreen pseudo-element | desktop+mobile | not-started | ubiquitous | P4/M30+ | S | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | CSS Selectors 4; MDN |

### Remote Playback API

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 70 | Remote Playback API (presentation) | desktop+mobile | not-started | mixed | P5/M43+ | L | Chr: yes (Chromecast). FX: no. WK: yes (AirPlay). Servo: no. Ladybird: no. Flow: chr-inherit. | W3C remote-playback; MDN |
| 71 | Chromecast integration | desktop+mobile | not-started | niche | P5/M43+ | XL | Chr: yes. FX: no. WK: n/a. Servo: no. Ladybird: no. Flow: chr-inherit. | Google Cast SDK |
| 72 | AirPlay integration | desktop+mobile (Apple) | not-started | niche | P5/M43+ | L | Chr: n/a. FX: n/a. WK: yes. Servo: no. Ladybird: no. Flow: n/a. | Apple AirPlay docs |

### Media recording and capture

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 73 | MediaRecorder API (recording to blob) | desktop+mobile | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes (Safari 14.1+). Servo: no. Ladybird: no. Flow: yes. | W3C mediastream-recording; MDN |
| 74 | getUserMedia — audio/video capture | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C mediacapture-streams §10; MDN |
| 75 | getDisplayMedia — screen/window/tab capture | desktop+mobile | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes (Safari 13+). Servo: no. Ladybird: no. Flow: yes. | W3C screen-capture §4; MDN |
| 76 | Media capture constraints model | desktop+mobile | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C mediacapture-streams §11 |
| 77 | Device enumeration — enumerateDevices() | desktop+mobile | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C mediacapture-streams §5 |
| 78 | Media capture — permissions integration | desktop+mobile | not-started | ubiquitous | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C Permissions API; mediacapture-streams §10.3 |
| 79 | Canvas captureStream() | desktop+mobile | not-started | widespread | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | WHATWG HTML §4.12.5; MDN |

### WebRTC

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 80 | RTCPeerConnection | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | XL | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webrtc §4; MDN |
| 81 | RTCDataChannel | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webrtc §6 |
| 82 | ICE / STUN / TURN | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | XL | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | RFC 8445 (ICE); RFC 5389 (STUN); RFC 5766 (TURN) |
| 83 | SRTP / DTLS (media encryption) | desktop+mobile+embedded | not-started | ubiquitous | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | RFC 3711 (SRTP); RFC 6347 (DTLS) |
| 84 | WebRTC simulcast | desktop+mobile | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webrtc §5.2; RFC 7656 |
| 85 | SVC (scalable video coding) | desktop+mobile | not-started | mixed | P4/M30+ | L | Chr: yes (VP9 SVC). FX: partial. WK: partial. Servo: no. Ladybird: no. Flow: chr-inherit. | RFC 6190; WebRTC-SVC spec |
| 86 | AV1 SVC in WebRTC | desktop+mobile | not-started | mixed | P4/M30+ | XL | Chr: yes. FX: partial. WK: no. Servo: no. Ladybird: no. Flow: chr-inherit. | AOM AV1 RTP spec (draft-ietf-avtcore-av1-rtp) |

### WebCodecs API

| # | Capability | Surface | Status in Spiral | Browser prevalence | Phase impact | Complexity | Engine notes | Sources |
|---|-----------|---------|-----------------|-------------------|-------------|-----------|-------------|---------|
| 87 | WebCodecs — VideoEncoder / VideoDecoder | desktop+mobile | not-started | widespread | P4/M30+ | XL | Chr: yes. FX: yes (Firefox 130+). WK: yes (Safari 16.4+). Servo: no. Ladybird: no. Flow: yes. | W3C webcodecs §3-4; MDN |
| 88 | WebCodecs — AudioEncoder / AudioDecoder | desktop+mobile | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webcodecs §5-6 |
| 89 | WebCodecs — VideoFrame / AudioData | desktop+mobile | not-started | widespread | P4/M30+ | L | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webcodecs §7-8 |
| 90 | WebCodecs — EncodedVideoChunk / EncodedAudioChunk | desktop+mobile | not-started | widespread | P4/M30+ | M | Chr: yes. FX: yes. WK: yes. Servo: no. Ladybird: no. Flow: yes. | W3C webcodecs §9-10 |

---

## Cross-refs

These rows in `specs/GAP_ANALYSIS.md` already track media-related gaps:

| GAP row | Section | Status | Notes |
|---------|---------|--------|-------|
| Hardware video decode (`spiral-media`) | §4.3 (line 295) | `[ ]` | M30+. |
| AV1 (dav1d), VP9, HEVC, Opus, AAC | §4.3 (line 296) | `[ ]` | |
| Widevine CDM bridge | §4.3 (line 297) | `[ ]` | M36+. |
| ClearKey EME | §4.3 (line 298) | `[ ]` | M12. |
| `spiral-media` crate existence | §4.4 (line 300–304) | `[ ]` | Not in workspace. M30+. |
| WebRTC | §2.1 (line 168) | `[ ]` | |
| Animated images (APNG, animated WebP, AVIF) | §2.4 (line 211) | `[ ]` | |
| Priority #15: `spiral-media` (MSE/EME/codecs) | §6 (line 390) | P3 | M30+. |

No rows for MediaSession, Picture-in-Picture, Fullscreen API, Remote
Playback, MediaRecorder, getUserMedia, getDisplayMedia, WebCodecs,
Web Audio API, HDR video, or the Media Capabilities API exist in
GAP_ANALYSIS today. These are new entries for chunk 13 to add.

---

## Open questions for the user

1. **Codec strategy — vendoring vs platform decoding.** Modern engines
   use a mix of bundled software codecs (dav1d for AV1, libvpx for VP9)
   and platform-provided decoders (VideoToolbox, MediaCodec, DXVA).
   Does Spiral intend to bundle its own codec libraries (like Chromium
   does with dav1d), delegate to the OS (like WebKit on Apple
   platforms), or both? This decision has massive implications for
   binary size, licensing (H.264/HEVC patent pools), and platform
   support matrix.

2. **EME / DRM priority.** GAP_ANALYSIS marks ClearKey EME at M12 and
   Widevine at M36+. Is ClearKey (which is mandatory per spec and
   unencumbered by licensing) a firm M12 deliverable, or does it slip
   if media infrastructure is not ready? Widevine integration requires
   a licensing agreement with Google and a binary CDM — is this
   something Spiral is willing to pursue?

3. **WebRTC priority.** Real-time communication is a substantial
   subsystem (ICE, STUN/TURN, SRTP, codec negotiation). Is WebRTC in
   scope for v0.1 or v1.0? Most engines treat it as essential, but it
   is a large engineering surface with limited relevance for a
   primarily reading-focused browser.

4. **HLS vs DASH.** HLS is dominant on mobile (Apple requires it for
   App Store video). DASH is the standard for desktop. Should Spiral
   prioritise one over the other, or support both via MSE? Native HLS
   parsing (without MSE) is significantly more complex.

5. **Dolby / proprietary codec licensing.** AC-3, E-AC-3, Dolby
   Atmos, Dolby Vision, and HEVC all carry patent licensing
   obligations. Does Spiral intend to ship these, or treat them as
   platform-provided (OS codec) and expose only via the platform's
   media framework?

6. **Animated JPEG XL.** Chromium removed JPEG XL support. Firefox
   has it behind a flag. This codec is in limbo — should it be scored
   as `experimental` or `legacy`? The current row scores it `niche`.

7. **Surface parity.** Media capabilities are overwhelmingly
   desktop+mobile+embedded. Should Spiral track any
   mobile-embedded-only capabilities (e.g. haptic feedback during
   media playback, background audio policy on mobile OSes) as distinct
   rows, or collapse them into the existing rows?
