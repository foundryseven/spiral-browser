# M4 Sprint 1 — Originality, Novelty, and License Audit

**Date:** 2026-06-15
**Auditor:** implementer agent (with research agents)
**Scope:** all files added in the M4 first sprint (`spiral-context`,
`spiral-filter`, Vortex GC rewrite) and the associated design docs
(`docs/design-filter-rule-model.md`, `docs/design-capability-types.md`,
`docs/design-vortex-heap.md`).

**Related artifacts (each with one job):**
- `docs/audits/2026-06-15-baseline.md` — functional baseline audit + M4.4–M4.6 prioritised plan
- `specs/GAP_ANALYSIS.md` — live checkbox tracker, what is built / missing
- `docs/baseline-warnings.md` — `cargo check --workspace` warning drift baseline
**Methodology:** four parallel research agents compared each artifact
against canonical prior art (V8, SpiderMonkey, JSC, Boa, gc-arena,
rust-gc, slotmap, generativity, qcell, ghost-cell, cap-std,
ambient-authority, Brave's `adblock` crate, uBlock Origin, EasyList,
Coalition for Better Ads).

**User mandate:**
- No copying unless open-source libraries and licenses followed.
- Everything claimed as ours must be our own code.
- Triple-check what we create is genuinely new.
- Claims of uniqueness must be verified.

---

## 0. Headline Verdict

| Question | Answer |
|---|---|
| **Copied code detected?** | **No.** All code is re-implemented from documented prior art. |
| **License risk?** | **None.** MPL-2.0 is compatible with all patterns used. No attribution omissions. |
| **Genuinely novel contribution?** | **Yes — Vortex's per-origin arenas with origin-tagged cell headers** (no shipped engine does this combination). |
| **Factual errors in the code/docs?** | **Yes** — CBA threshold data has invented numbers; novelty claims are overclaimed. |
| **Should we research anything more before M4.4?** | **Mostly no.** Vendoring `html5ever`/`cssparser`/`selectors` is mechanical (MPL-2.0 compliance, modernise deps). Gyre block layout is a fresh implementation following the CSS spec — no research needed beyond the spec itself. |

---

## 1. Code Originality — Per-Crate Verdict

### 1.1 `spiral-context` — inspired-by-not-copied

| File | Verdict | Notes |
|------|---------|-------|
| `brand.rs` | Structural similarity to generativity, qcell, ghost-cell | `PhantomData<fn(&'brand ()) -> &'brand ()>` + `PhantomData<*const ()>` is the canonical pattern from the Rust Nomicon and used by all three prior-art crates. Field rename (`phantom` → `_invariant` + `_nosend`) is stylistic. |
| `brand.rs:70-72` `make_brand` | Structural similarity to qcell's `LCellOwner::scope` and GhostCell's `GhostToken::new` | Same HRTB trick. Spiral's takes `for<'brand> fn(Brand<'brand>) -> R` (fn pointer); qcell takes generic closure. Stylistic. |
| `caps.rs:22-69` capability ZSTs | Standard textbook idiom | Same shape as cap-std's `Dir`/`File`, ambient-authority's `AmbientAuthority`. Cited in the design doc. |
| `caps.rs:123-167` `CapabilitySet` as struct of `Option<Cap>` | **Original** | No prior Rust crate uses exactly this shape. cap-std's `Pool` is a `HashSet`. |
| `context.rs:18-35` `Context<'brand, Mode>` with empty-enum marker | Standard Rust type-level state machine | Common pattern, not unique. |
| `origin.rs:13-86` `Origin::parse` | Original implementation of RFC 6454 | Different field layout from `url::Url::origin()`, no `url` dep, manual parser. |

**Verdict:** No code copied. Techniques are well-known prior art, re-implemented from first principles in Spiral-native Rust.

### 1.2 `spiral-filter` — inspired-by-not-copied

| File | Verdict | Notes |
|------|---------|-------|
| `syntax/network.rs` (231 lines) | Parallel implementation of ABP public spec | Different data structures from Brave's `adblock` crate: nested enums vs flat bitflags, separate hostname vs embedded, `Party` enum vs `THIRD_PARTY`/`FIRST_PARTY` bits. Same domain model. |
| `syntax/cosmetic.rs` (147 lines) | Parallel implementation of ABP cosmetic grammar | Same conceptual model (selector + hostname scope + exception flag); different data structures. |
| `compile/trie.rs` (87 lines) | Original | Generic CS trie applied to hostname bucketing. No other adblock crate uses a literal trie. |
| `rule.rs` (302 lines) | Original AST | Different shape from Brave. Spiral's own taxonomy. |
| `lists/cba.rs` (114 lines) | Original, but **factual errors** (see §3) | Threshold values from CBA public spec; descriptions paraphrased. |
| `policy/default_policy.rs` (85 lines) | Original severity policy | Spiral's `WorstOffender`/`Annoying`/`Privacy`/`Spec`/`Critical` taxonomy. |

**Verdict:** No code copied. Custom ABP parser is a parallel implementation of a public grammar.

### 1.3 Vortex GC rewrite — inspired-by-not-copied

| File | Verdict | Notes |
|------|---------|-------|
| `gc/header.rs` (155 lines) | **Original** | The 4-byte cell header with packed type+mark+finalizer+origin bits is a Spiral design choice not seen in V8, SpiderMonkey, JSC, Boa, gc-arena, rust-gc, or slotmap. The on-object origin tag is the single most original structural decision in the Vortex GC. |
| `gc/key.rs` (158 lines) | Original | `GcKey { slot, version, origin_id }` is structurally inspired by slotmap's `{idx, version}` (acknowledged) but adds `origin_id` and is an independent implementation. |
| `gc/arena.rs` (290 lines) | Hand-rolled mark-sweep | Standard textbook algorithm. Per-origin capability inspired by SpiderMonkey's per-zone major GC (acknowledged). |
| `gc/heap.rs` (172 lines) | Original | A `HashMap<u16, OriginArena>` of arenas. Standard composition. The cross-origin key dispatch in `get()` is Spiral's own design. |
| `runtime/mod.rs` (122 lines) | Wires `VortexHeap` into `Vortex` | One bug noted: `gc_live_count` returns 0 because the helper returns an empty iterator. Should be fixed in M4.4. |

**Verdict:** No code copied. The genuinely novel contribution — origin tag in the cell header + cross-origin type-level check — is genuinely new.

---

## 2. Uniqueness Claims — Brutal Audit

Of ~25 "first/unique/novel/only/never" claims in the design docs:

| Category | Count | Examples |
|----------|-------|----------|
| **Genuinely novel** | ~2 | Vortex's origin-tagged cell headers; SEM's typed-context combination |
| **Partially novel** (general idea exists, Spiral's combination is new) | ~9 | Compile-time filter (uBO Firefox also does it); branded-lifetime browser context; per-origin arenas with no cross-origin pause |
| **Already exists, claim is wrong/misleading** | ~10 | "Compile-time filter avoids the cost" (uBO Firefox uses `filterResponseData`); "no browser has been memory-light AND fast" (Safari, Pale Moon); "no JIT" (Duktape, QuickJS, MuJS, LibJS, Hermes, Boa) |
| **Unverifiable** (future bets, not yet built) | ~6 | M25+ SEM runtime, M30+ mmap heap checkpoint, M36+ Widevine |
| **Aspirational/configuration, not technical novelty** | ~6 | "Worst-offenders-only default" (configuration choice, CBA is well-established); "5th browser engine" (counting is subjective; Ladybird, Flow, Servo are also independent) |

### The genuine novel contribution

**Vortex's per-origin arenas with origin-tagged cell headers in a shared heap.** No shipped engine does this. SpiderMonkey has per-zone major GC (since 2014) but zones are not origin-scoped. JSC has a shared `JSVirtualMachine` but no origin tagging. V8 is per-isolate. The combination of (a) shared heap, (b) origin tag in the cell header, (c) origin tag in the `GcKey`, (d) runtime origin check on every access, (e) per-origin pause isolation, is genuinely new.

### Specific factual errors to fix

1. **CBA threshold data has invented numbers.** See §3 for details.
2. **"uBlock Origin blocks at runtime" claim is wrong for Firefox.** uBO on Firefox uses `webRequest.filterResponseData()` to filter the HTML response body *before* the parser sees it. Only uBO on Chromium is purely runtime.
3. **"No JIT" is well-populated** (Duktape, QuickJS, MuJS, LibJS, Hermes, Boa, JerryScript, XS, Espruino, engine262). The unique bit is "from-scratch Rust + no JIT + origin-tagged GC."
4. **"Per-origin pause-free GC" exists in SpiderMonkey since 2014** (per-zone major GC). The novel part is the *implementation* (simpler, in a single shared heap, with origin tags).
5. **"5th browser engine" is aspirational.** Ladybird, Flow, Servo are all in this category. The honest framing: "independent Rust-first engine project."

---

## 3. CBA Threshold Factual Errors

The Coalition for Better Ads public standards at `betterads.org/standards/` were cross-referenced. The following inaccuracies were found in `crates/spiral-filter/src/lists/cba.rs`:

### 3.1 Desktop thresholds

| Entry | Current text | Accurate CBA position | Verdict |
|-------|--------------|------------------------|---------|
| `pop_up` | "new window the user did not request" | "interstitial that blocks main content" | Inaccurate framing |
| `prestitial_countdown` | "countdown >= 5 s" | CBA does **not** specify a 5-second threshold | **Invented** |
| `large_sticky` | "height > 30% viewport" | "30% of screen's real estate" (area, not height) | 30% correct; framing slightly off |
| `ad_density_50` | "sum(ad heights) / main content height > 50%" | Exact match | ✓ Correct |
| `autoplay_video_sound` | "audible audio, no user initiation" | Paraphrase | Acceptable |
| (missing) | — | "Ad Density > 30% + Sticky Video Ad" | 1 missing |

### 3.2 Mobile thresholds

| Entry | Current text | Accurate CBA position | Verdict |
|-------|--------------|------------------------|---------|
| `pop_up` | "interstitials or new windows" | "post-load content blocking" | Inaccurate |
| `prestitial` | "full-page ad > 30% viewport" | CBA does **not** specify 30% | **Invented** |
| `ad_density_30` | "sum(ad heights) / main content height > 30%" | Exact match | ✓ Correct |
| `flashing_animated` | "animation flashes > 3 Hz" | CBA does **not** specify Hz; 3 Hz is from WCAG 2.1 | **Wrong attribution** |
| `scrollover` | "full viewport takeover" | "more than 30% of the page" | **Inflated** (30% → 100%) |
| `large_sticky` | "sticky > 30% viewport" | "30% of screen's real estate" | Acceptable |
| (missing) | — | Postitial Countdown, Autoplay Video, Sticky Pop-out Video, Sticky Video with Large Inline Ad | 4 missing |

**Recommended fixes (in M4.4):**
- Remove invented thresholds (5s, 30% mobile prestitial, 3Hz) or cite their true source (WCAG 2.1 for 3Hz; user-defined spiral policy for the others, clearly labelled).
- Correct `scrollover` description to "more than 30% of the page."
- Fix `pop_up` descriptions to "interstitials blocking main content."
- Either add the missing CBA experiences or document why they are deferred.

---

## 4. License Risk — Clean

| Source | Verdict |
|--------|---------|
| `seahash = "4"` | **MIT**, not BSD-4-Clause. Compatible with MPL-2.0. (Brave's `adblock` crate also uses seahash ^4.1.0.) |
| CBA threshold data | **Not copyrightable.** Facts and standards are not copyrightable (US doctrine, Berne Convention equivalents). Threshold values are free to use. |
| ABP / EasyList grammar | **Not copyrightable** (Baker v. Selden, 1879; merger doctrine). Many independent parsers exist. |
| `spiral-core`, `spiral-dom` | Internal MPL-2.0 dependencies. |
| `rusty_v8` (optional) | Already in workspace behind `v8` feature flag for CI compliance. |
| Patterns used (branded lifetimes, capability tokens, type-level state) | **Textbook idioms.** Cited in design docs. No attribution omission. |

**No legal exposure.** No code was copied. All techniques are well-documented prior art. No attribution headers are missing.

---

## 5. Should We Research More Before M4.4?

**M4.4–M4.6 work items:**
- Vendor `html5ever` into `spiral-fmt`; modernise deps
- Vendor `cssparser` + `selectors` into `spiral-fmt`
- Unified facade `spiral_fmt::parse_html()`, `spiral_fmt::parse_css()`
- `spiral_net::Resolver` trait wrapping hickory-dns
- Gyre block layout first pass (box model, margin collapse)

### Per-item research assessment

| Item | Additional research needed? | Why |
|------|------------------------------|-----|
| Vendor `html5ever` | **No** | Mechanical integration. MPL-2.0 → MPL-2.0. The vendoring plan is already in `docs/plans/iteration-options.md`. We have not read upstream's `html5ever` source code to copy from — we will port it, taking the same "research then write our own" approach as the M4 first sprint. |
| Vendor `cssparser` + `selectors` | **No** | Same as above. |
| `spiral_fmt::parse_*` facade | **No** | Design follows from vendored crates. |
| `spiral_net::Resolver` trait | **No** | Standard wrapper over hickory-dns. |
| Gyre block layout first pass | **Maybe — but minimal** | CSS block layout has subtle edge cases (margin collapse, BFC, IFC, floats). The risk is *correctness*, not *copying*. We are not at risk of accidentally copying Servo's layout code because we are writing Gyre from scratch, in the same way Vortex was written from scratch. The research need is: read the CSS 2.1 box model spec, read the CSS 2.1 block formatting context spec, and start writing. No additional research into "is Gyre novel?" — Gyre's novelty comes from being a custom layout engine, which is not novel (Servo, Ladybird, Flow, Pale Moon's Goanna all do it). The novelty is in the *combination* with the SEM shared style cache, which is downstream work. |

**Net answer:** **No additional research is required before M4.4.** The vendoring work is mechanical. The Gyre work is fresh implementation. The originality verification is done.

---

## 6. Recommended Code/Doc Fixes (Pre-M4.4)

These are fixes the audit surfaced. They should land in M4.4 alongside the vendoring work.

### 6.1 Fix CBA threshold data (factual accuracy)

File: `crates/spiral-filter/src/lists/cba.rs`
- Remove invented thresholds (5s, 30% mobile prestitial, 3Hz)
- Correct `scrollover` description
- Fix `pop_up` descriptions
- Add missing CBA experiences or document deferral

### 6.2 Soften overclaimed novelty in design docs

File: `docs/design-capability-types.md:9.2` and similar
- Reword "implemented from scratch in Spiral-native code" to "re-implemented the brand and capability patterns from the prior art cited in Section 11"
- Drop the "5th browser engine" framing as a uniqueness claim
- Fix the "uBlock Origin blocks at runtime" claim (uBO on Firefox does response filtering)

File: `docs/design-vortex-heap.md`
- Acknowledge SpiderMonkey's per-zone major GC (since 2014)
- Frame the genuine novelty (origin in cell header) more precisely

### 6.3 Fix `runtime/mod.rs:84-88`

`gc_live_count` always returns 0. Either expose arenas from `VortexHeap` or remove the placeholder.

### 6.4 Add tests to `spiral-context/tests/`

The `tests/` directory is empty. The design doc promises `trybuild` compile-fail tests. Either land them or update the design doc.

---

## 7. Sources

External references consulted in this audit:

- **Rust branded-type prior art:** Rust Nomicon "Table of PhantomData patterns", `generativity` crate, `qcell` crate, `ghost-cell` crate (ICFP 2021 paper by Yanovski et al.)
- **Capability systems:** `cap-std` docs, `ambient-authority` crate, FreeBSD Capsicum, seL4 capabilities tutorial, WASI preopens
- **Browser engines:** V8 (BSD), SpiderMonkey (MPL 2.0), JSC (BSD), Boa (MIT), gc-arena (MIT), rust-gc (MIT), slotmap (Zlib)
- **Ad blocking:** Brave `adblock` crate (MPL 2.0), uBlock Origin wiki, EasyList, Coalition for Better Ads public standards
- **Browser comparison:** Wikipedia (Chromium, Firefox, Safari, Brave, Servo, Ladybird, Flow, Pale Moon, uBlock Origin, Adblock Plus, Ad blocking, List of JavaScript engines, Capability-based security, Process isolation, Site isolation)

---

## 8. SSOT Update

The findings above should be reflected in:
- `docs/active_context.md` — the engine thesis sections should be softened where overclaimed
- `docs/progress_ledger.md` — append an audit entry
- `docs/system_architecture.md` — same softening
- `crates/spiral-filter/src/lists/cba.rs` — fix factual errors
- `crates/spiral-vortex/src/runtime/mod.rs` — fix `gc_live_count` bug

All these are M4.4 work items, to be done before the next sprint kicks off.
