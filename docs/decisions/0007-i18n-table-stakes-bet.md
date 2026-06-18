# ADR 0007 — Table-Stakes i18n Bet (Steps 2.9–2.12)

- **Date:** 2026-06-18
- **Status:** Accepted
- **Supersedes:** none
- **Superseded by:** none
- **Author:** implementer agent
- **Scope:** Steps 2.9, 2.10, 2.11, 2.12 in `docs/implementation_tracker.md`; the i18n rows of `docs/research/09-i18n-engine.md` (rows 45-47, 64-66, 68-72, 74-75, and 71).

## Context

Per the i18n-vs-Ladybird gap analysis captured in the in-chat deep-dive (2026-06-18) and the i18n engine research at `docs/research/09-i18n-engine.md`, Spiral is **46 of 46 i18n rows "not-started"** as of the last matrix update. The big three engines (Chromium, Firefox, Safari) all have most of these; Ladybird has partial coverage. Closing every row would cost 18-24 months of work that does not advance the architectural bet (Rust + brand types + shared-everything + L1-L5 filter + GPU chrome).

The user request (2026-06-18) was: "Factor your highest-ROI sequence into the existing dev path." The deep-dive ranked the i18n work into three tiers by ROI:

1. **Cheap wins** (1-4 weeks each, configuration choices that wire existing Rust crates): `navigator.language` IDL, `<html lang>` reflection, encoding detection via `encoding_rs`, line break via `linebreak`, IDNA via `idna`, bidi via `unicode-bidi`, normalisation via `unicode-normalization`, East Asian Width via `unicode-width`.
2. **Medium wins** (1-3 months each, partial architectural surface): complex text shaping via `rustybuzz` (Servo's HarfBuzz port), script detection via `unicode-script`, locale-aware `<input>` types.
3. **Expensive wins** (6+ months each, structural rewrites): `Intl.*` family (12+ objects, ICU bindings), vertical typesetting (Gyre is single-axis), translation API, browser spellcheck, IME composition events, calendar systems beyond Gregorian, IANA tzdata, OpenType variable-font features.

The architecture rules in `.spiral/rules/architecture.md` (the "down-only" dep graph) require that any new dep edge points "down" the canonical graph. The new i18n work must respect that.

## Decision

**Adopt the table-stakes bet:** ship Steps 2.9–2.12 (the cheap and medium wins) using existing Rust crates. Defer the expensive wins to v0.2+ work in a future Phase. Wire the new deps behind a feature flag where the dep is heavy (`rustybuzz` is the only one — ~1.5 MB compiled).

The dep graph changes:

| New dep | Crate | Feature flag | Down-only check |
|---------|-------|--------------|-----------------|
| `encoding_rs` | `spiral-fmt` | (default on) | `spiral-fmt` is upstream of everything that needs it; no upward edge |
| `unicode-bidi` | `spiral-gyre` | (default on) | `spiral-gyre` is downstream of `spiral-fmt`; safe |
| `linebreak` | `spiral-gyre` | (default on) | same |
| `unicode-normalization` | `spiral-vortex` | (default on) | same |
| `unicode-width` | `spiral-gyre` | (default on) | same |
| `unicode-script` | `spiral-gyre` | (default on) | same |
| `rustybuzz` | `spiral-gyre` | `harfbuzz` (default off) | gated; not in default build |
| `idna` | `spiral-fmt` or `spiral-net` (TBD by Packet 2.7.1 ownership) | (default on) | depends on URL parser ownership |

All eight deps point "down" the canonical graph. The `rustybuzz` flag keeps the heavy dep out of CI runs that don't need it; the `harfbuzz` feature gate is testable in isolation via `cargo test -p spiral-gyre --features harfbuzz`.

## Why this is the right bet

The Spiral bet (`docs/system_architecture.md:56-189`) rests on five architectural choices: shared-everything, JIT-optional, L1-L5 filter, persistent renderer, GPU chrome. Each of those bets is *demonstrated* on the Latin-script, LTR, ASCII, network-stable web — the kind of web that Chromium, Firefox, and Safari were also built for. The i18n gaps do not threaten the bet; they threaten the daily-driver use case.

The user (James) is a no-code-agentic driver. The bet is meant to ship a *bets-proving* engine, not a *table-stakes-complete* one. The four Steps land 12 packets that close ~20 of the 46 i18n rows in 3-6 months of focused work, using crates that already have years of WPT coverage behind them. The remaining ~26 rows (mostly `Intl.*`, vertical text, translation, spellcheck, IME) are deferred to v0.2+ where the bet is already proven and the question shifts from "does this engine exist" to "is this engine daily-driver ready."

## Alternatives considered

### Option A — Defer all i18n work to v0.2

Skip Steps 2.9–2.12 entirely. Cost: 0 months. Risk: Spiral's first public demo still has the same i18n gaps as a 1995 browser. **Rejected** because the table-stakes work is cheap enough that not doing it leaves a real product gap (no Hebrew, no Bengali, no Japanese, no encoding detection, no internationalised domain names) that would not be visible from WPT pass rates on Latin-script sites.

### Option B — Ship the full `Intl.*` family in Phase 2 (12+ packets, 12-18 months)

Cost: 12-18 months. Risk: erodes the bet timeline; pushes Phase 3 Networking and Phase 4 Presentation into 2027. **Rejected** because `Intl.*` parity is a 0.1% of pages concern, not a daily-driver blocker for the audience the bet is targeting.

### Option C — Ship Steps 2.9–2.12 in a single "i18n" Phase between Phase 2 and Phase 3 (not slotted into Phase 2)

Cost: same as the chosen option, but creates a new Phase 2.5. **Rejected** because Phase 2 is "Spec Compliance" and i18n *is* spec compliance (HTML Living Standard §15.1.2, WHATWG Encoding Standard, UAX #9, #11, #14, #15, RFC 5890, UTS #46). Sloting the Steps into Phase 2 matches the Phase's stated purpose.

### Option D — Wire `rustybuzz` as a default-on dep (no `harfbuzz` feature flag)

Cost: 1.5 MB larger default binary. Risk: extends the `just verify-packet` runtime; bloats CI cache. **Rejected** for the same reason the `v8` feature flag exists on `spiral-vortex` (`docs/architecture/vortex.md`): heavy deps stay behind flags so the verification matrix stays fast.

## Novelty check (per `AGENTS.md` §"Novelty Claims")

The four Steps do **not** introduce any novel capability. Each one is a configuration choice (wire an existing Rust crate) that is present in at least one of: Chromium, Firefox, Safari, Servo, Ladybird, Flow. The novel-tending aspects of Spiral (brand types, shared Vortex heap, L1-L5 filter, GPU chrome) are unchanged. The claim survives: "table-stakes i18n" is the right framing per `docs/audit-sprint-m4.md:105,117` ("Aspirational/configuration, not technical novelty").

## Wiring & Integration

- **Crates affected:** `spiral-fmt` (encoding detection, IDNA, `<html lang>` reflection), `spiral-dom` (`lang` field), `spiral-vortex` (`navigator.language`, `TextEncoder`/`TextDecoder`, `String.prototype.normalize`), `spiral-gyre` (bidi, linebreak, EAW, normalisation, shaper, script detection, font fallback), `spiral-render` (Noto font stack).
- **Call sites:**
  - `spiral_fmt::html::tree::TreeBuilder` writes `lang` on `</html>` end-tag.
  - `spiral_fmt::parse_html(bytes: &[u8])` decodes via BOM + `<meta charset>` + `Content-Type`.
  - `spiral_gyre::LayoutEngine::text_bidi_pass`, `text::linebreak::breaks`, `text::shaper::shape`, `text::script::detect` are called by the layout iterator at `crates/spiral-gyre/src/lib.rs:36`.
  - `spiral_vortex::builtins::{navigator, encoding, string::normalize}` are registered on the `window` global.
- **Test coverage:** 11 new test files (one per packet) and one end-to-end smoke test at `crates/spiral-browser/tests/i18n_smoke.rs`.
- **End-to-end surface:** The smoke test loads a fixture page with `<html lang="ja">` containing a Hebrew+Bengali+Latin string into a 60 px box and asserts (a) `navigator.language` returns `"en-US"`, (b) `document.documentElement.lang` returns `"ja"`, (c) the layout tree contains four glyph runs tagged per-script per the fallback chain, and (d) the host parser accepts `https://bücher.de`.
- **Audit expectation:** `./scripts/audit-orphan-exports.sh` exits 0 after every packet; the `harfbuzz` feature flag keeps `rustybuzz` off the default build.

## Open questions (left to the implementer agent picking up Packet 2.9.1)

1. **Packet 2.7.1 ownership** — the URL parser is in flux. Packet 2.12.1 (IDNA) attaches to whichever crate wins the 2.7.1 decision. If `spiral-fmt`, the `idna` dep lands there; if `spiral-net`, it lands there.
2. **Feature flag name** — the ADR uses `harfbuzz` to match the C++ ecosystem's vocabulary. If the reviewer agent prefers `shaper` (Spiral-native), the rename is a one-line `Cargo.toml` edit.
3. **Smoke test fixture licensing** — the Hebrew+Bengali+Latin string in `i18n_smoke.rs` should use a public-domain fixture (e.g. a UN declaration excerpt in the public domain). Implementer must verify before commit.
