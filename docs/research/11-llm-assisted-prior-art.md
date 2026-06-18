# Chunk 11 — LLM-Assisted Authorship Prior Art

> **Scope.** This file documents the prior art for the framing Spiral uses
> to describe how it is built. It is the evidence base for the
> novelty-gate check required by `AGENTS.md` § Novelty Claims, and the
> supporting citation for [`docs/methodology.md`](../methodology.md) §8
> ("What this is not").
>
> **Companion to:** `09-i18n-engine.md` (i18n), `00-methodology.md`
> (research methodology), `docs/methodology.md` (project methodology).

---

## §1 The question

Spiral's marketing posture is "LLM-assisted, human-directed,
adversarially reviewed, test-verified." A hostile reviewer will
immediately ask: is this novel? Is the AI assistance a first? Is the
methodology document a first? Is the public failure log a first?

The honest answer is **no, not as a combination** — at least two prior
projects (Ladybird, FastRender) have shipped substantive prior art on
the individual axes. The narrow novelty claim that *does* survive
audit-script enforcement is discussed in §3. This file documents the
prior art so the claim in [`docs/methodology.md`](../methodology.md) §8
is evidence-backed, not asserted.

---

## §2 Prior art by axis

### §2.1 Browsers and browser engines

**Ladybird (Kling, 2026-02-23).** Andreas Kling — a designer and
project manager, not an engineer — published a long-form post titled
"Ladybird adopts Rust, with help from AI"
([`https://ladybird.org/posts/adopting-rust/`](https://ladybird.org/posts/adopting-rust/)).
Kling used Claude Code and OpenAI Codex to translate 25,000 lines of
C++ to Rust in two weeks, on a from-scratch browser engine, with
explicit adversarial review passes and a quantified test/regression
writeup. The role analogue to the Spiral maintainer is exact: Kling
directs the work, Kling is not the engineer, and Kling is open about
the AI assistance in the project.

Ladybird ships:
- a written methodology (the Feb 2026 post);
- named failures and their resolution (documented inline in the post);
- adversarial review (documented as part of the porting process).

Ladybird does **not** ship:
- a CI-enforced audit script that gates the methodology on every commit;
- a self-published, ongoing, LLM-mistake-specific failure log (the
  Feb 2026 post is a one-off writeup, not a maintained collection);
- a `pub` symbol audit (Ladybird is C++/Rust at the engine layer with
  different surface-area concerns; the audit category is less
  applicable).

**FastRender (Lin / Cursor, 2026-01).** Wilson Lin, in collaboration
with Cursor, published a Rust browser engine with a 300-line
`AGENTS.md` methodology document. The project was audited by the
Software Improvement Group (SIG) in February 2026, producing a
public quality report
([`https://www.softwareimprovementgroup.com/blog/quality-of-fastrender/`](https://www.softwareimprovementgroup.com/blog/quality-of-fastrender/))
which scored the project 1.3/5 on maintainability (bottom 5% of all
projects SIG has audited). The audit, the methodology document, and
the public scoring make FastRender the most direct prior art for
"browser + AI assistance + public methodology + external audit."

FastRender ships:
- a 300-line methodology document;
- a public third-party audit.

FastRender does **not** ship:
- a CI-enforced self-audit (the SIG audit was one-off, not gated);
- a public failure log of LLM-specific mistakes (the SIG report is a
  quality report, not a mistake log);
- a designer-led model (Lin is an engineer).

**Other AI-assisted browser side-projects.** The 2024-2026 "vibe
coding" wave produced several short-lived browser side-projects. None
of them has the combination of (a) sustained development, (b) public
methodology, and (c) public failure log. Most are single-weekend
demonstrations that do not survive the first serious review. The
prior art on the LLM-assisted browser axis is essentially Ladybird
and FastRender.

### §2.2 The broader "vibe coding" / AI-assisted development wave

**Mod (rumoured, unverifiable).** A widely-cited "Mod" project (an
RSS reader, "software for one, by a designer") was referenced as a
prior-art analogue during Spiral's planning. **The project does not
appear in any verifiable form** in the Wikipedia, GitHub, or
DuckDuckGo indexes as of 2026-06-18. The reference is dropped from
Spiral's methodology doc. The closest verified analogue is Kling
himself (Ladybird).

**Mod by Ioseb Gomarteli.** A search for "Mod" + "vibe coding" +
"designer" returns this person as the most common referent. The
project is undocumented at the URL level. **Not cited.**

**The Cursor / FastRender episode.** FastRender is the only widely
publicised project in this wave to publish a methodology document of
the size and specificity that approaches Spiral's. The SIG audit
response was widely covered; the SIG report explicitly framed the
project as "the work of an AI model with a human in the loop." This
is the strongest piece of external validation that the model works
in the browser space — and the strongest piece of external
skepticism that the model produces maintainable code without
enforcement.

**Other LLM-assisted systems software.** LLM-assisted operating
system, kernel, and systems-software projects exist in 2025-2026 but
none has reached the same scale or visibility as a browser. The
closest analogue is the "Rewrite it in Rust" wave (LSD.UU, ripgrep
successors) where AI assistance is occasionally used but not the
primary authorship model. The methodology on the *systems* axis is
underdeveloped; Spiral's contribution here is minor.

### §2.3 Named-failure logs in any open source project

**Mozilla Security Blog.** A continuous record of named
vulnerabilities and their resolutions, written by the project
itself. This is the closest cultural analogue to the Spiral failure
log, but it is security-specific and is not an LLM-mistake log.

**Google Project Zero.** A third-party vulnerability disclosure
log. Strongest published example of "named failures" as a cultural
practice in open source. Not LLM-specific.

**Monocypher (Loup Vaillant, ongoing).** A small cryptography
library with a public QA log and named failures. The strongest
non-browser analogue for "QA + audit + named-failure log" as an
ongoing, self-published practice. URL:
[`https://monocypher.org/quality-assurance/`](https://monocypher.org/quality-assurance/).

**Other.** Multiple crypto projects (libsodium, age, BearSSL) publish
audit logs. Few publish named-failure logs at the same depth.

### §2.4 Adversarial-review process in any open source project

**Monocypher.** Loup Vaillant publishes his review process as a
public doc. The QA log is part of the review.

**Cryptography projects generally.** Two-person mandatory review is
the norm in mature cryptography projects (OpenSSL, BoringSSL, age,
libsodium). The practice is well-established outside of LLM-assisted
work.

**Browsers with mandatory two-person code review.** Chromium,
Firefox, and WebKit all require two-person review on commits to
sensitive areas. The practice is not publicly documented at the
process level — it is enforced in the contribution guidelines, not in
a public methodology document.

**Spiral's contribution.** Spiral's adversarial review is the same
in spirit as the cryptography norm but adds a **mechanical adversary**
layer: the `audit-orphan-exports.sh` and `audit-doc-drift.sh`
scripts are automated reviewers that exit non-zero on a class of
regressions humans do not catch reliably. This is the audit-script
enforcement axis on which Spiral's novelty claim is grounded.

---

## §3 The narrow novelty claim

Combining the prior art, the narrowest defensible novelty claim is:

> **Spiral is the first browser project to make the LLM-assisted
> methodology operative rather than aspirational: the audit
> scripts run on every commit, and an audit failure is a build
> break.**

This claim survives the prior-art evidence. Ladybird documents the
methodology but enforces it by convention. FastRender documents the
methodology but the enforcement is by the SIG audit, not by CI on
every commit. The closed-loop "CI audit grep is a build break" loop
is the contribution.

The wider claims — "first LLM-assisted browser", "first methodology
document", "first public failure log" — do **not** survive the
prior-art evidence. [`docs/methodology.md`](../methodology.md) §8
cites the prior art honestly and narrows the framing accordingly.

---

## §4 Implications for the Spiral project

1. **The marketing line is "audit-enforced methodology", not "AI
   assistance."** The former is defensible. The latter is
   over-claiming relative to the prior art.

2. **The failure log must be LLM-mistake-specific to be
   defensible.** A generic "things that broke" log is not a
   differentiator (Mozilla Security Blog, Project Zero, Monocypher
   all do this). The LLM-mistake-specific framing is the
   differentiator.

3. **The methodology document must concede the prior art.** §8 of
   [`docs/methodology.md`](../methodology.md) is the concession.
   Without it, the project is over-claiming relative to Ladybird and
   FastRender.

4. **The audit scripts are the load-bearing artifact.** The novelty
   claim is structurally dependent on `scripts/audit-orphan-exports.sh`
   and `scripts/audit-doc-drift.sh` continuing to run on every commit
   and exit non-zero on the categories of regression they catch. A
   regression in the audit scripts is a regression in the project's
   core claim.

---

## §5 Citations

- **Ladybird / Kling (2026-02-23)** — "Ladybird adopts Rust, with
  help from AI", [`https://ladybird.org/posts/adopting-rust/`](https://ladybird.org/posts/adopting-rust/).
  Methodology writeup, named failures, adversarial review. Does not
  have CI-enforced audit scripts or a maintained LLM-mistake log.

- **FastRender / Lin (2026-01)** — Rust browser engine with
  300-line `AGENTS.md`, [`https://github.com/nicebyte/fastrender`](https://github.com/nicebyte/fastrender).
  SIG quality audit (2026-02), [`https://www.softwareimprovementgroup.com/blog/quality-of-fastrender/`](https://www.softwareimprovementgroup.com/blog/quality-of-fastrender/).
  1.3/5 maintainability, bottom 5%. Public methodology, public
  audit, no CI-enforced self-audit, no maintained LLM-mistake log.

- **Monocypher / Vaillant (ongoing)** — Quality assurance log,
  [`https://monocypher.org/quality-assurance/`](https://monocypher.org/quality-assurance/).
  Strongest non-browser analogue for "QA + audit + named-failure
  log" as an ongoing, self-published practice.

- **Mozilla Security Blog** — Continuous vulnerability and
  resolution log, [`https://blog.mozilla.org/security/`](https://blog.mozilla.org/security/).
  Cultural reference for "named-failure log as project practice."

- **Google Project Zero** — Third-party vulnerability disclosure,
  [`https://googleprojectzero.blogspot.com/`](https://googleprojectzero.blogspot.com/).
  Reference for "named failures as published practice."

- **AGENTS.md (this repository) § Novelty Claims** — Gate on
  research-verified evidence before any "first", "novel", "unique",
  or "no prior art" claim is committed.

---

## §6 Conclusion

The narrow claim — "first browser to enforce the LLM-assisted
methodology by audit-script CI on every commit" — survives the
prior-art evidence. The wider claims do not. The Spiral project
documents the prior art in [`docs/methodology.md`](../methodology.md)
§8, narrows the framing to the audit-enforcement axis, and grounds
the failure log in LLM-mistake specificity to remain defensible.
This file is the evidence base; the methodology file is the
commitment.
