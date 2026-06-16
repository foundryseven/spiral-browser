# Spiral Innovations IV — Five Outside-the-Box Provocations

**Status:** design provocations (2026-06-15)
**Author:** implementer agent
**Phase context:** M4 first sprint complete; companion to
[`innovations-stubs.md`](innovations-stubs.md) (10 ideas),
[`innovations-stubs-2.md`](innovations-stubs-2.md) (11 ideas),
[`innovations-stubs-3.md`](innovations-stubs-3.md) (12 ideas),
[`innovations-stubs-4.md`](innovations-stubs-4.md) (32 ideas).
**Purpose:** deliberately-unbuildable directions. The user
explicitly chose "completely outside the box" for this
batch.

---

## How to read this doc

These are **provocations, not roadmap items.** Each one:

1. Is described as a 1-paragraph pitch.
2. Has a "what it would look like" paragraph.
3. Has a "why it might fail" paragraph.
4. Has an "open questions" list.

The audit methodology does **not** apply here. The novelty
of these ideas is their *direction*, not their buildability.
The "what would it look like" section is a sketch, not a
spec. The "why it might fail" section is a list of reasons
the direction may be impossible.

If even one of these sparks a real, buildable idea, the
batch is worthwhile.

---

## #77 — "Browser-as-a-Compiler"

**Pitch.** The browser compiles the web. Not just JavaScript
— HTML, CSS, JSON, SVG, all of it. A site is a *program*;
the browser's job is to *run* that program. The compiler
is *always on*, even for "static" pages. The output is a
binary blob that runs in a sandboxed runtime.

**What it would look like.** Spiral ships with a new "Spiral"
compilation target. The browser parses HTML, parses CSS,
parses JS, parses JSON, parses SVG, and produces a *single
binary blob* representing the page's runtime. The blob is
signed by the site (or the user's local content hash), runs
in a sandboxed runtime with explicit capability grants, and
can be snapshotted, diffed, and replayed. The web becomes
"the world's largest distributed compiler" — every site is
already a program, and the browser is the runtime.

**Why it might fail.** The web is not a programming language.
The standards (HTML, CSS, JS) are intentionally lossy
browsers render what they get, and "compiling" loses
information. Backwards compatibility is impossible — every
existing site would have to be re-compiled. The W3C would
not adopt a new "Spiral" target. The performance gains are
unproven (browsers already JIT, and the Wasm bytecode format
is already a "compile target"). The security argument (signed
binaries) is undermined by the fact that the existing web
is "signed" by HTTPS, which is a weaker but already-shipped
authentication mechanism.

**Open questions.** Is the *output* actually a new format,
or is it just "Wasm with HTML/CSS embedded"? If Wasm, the
idea is just "compile the web to Wasm," which is what
Shopify, Figma, and others have already explored for
specific domains. What does "compile" mean when the input
is inherently dynamic (CSS `:hover`, JS `eval`)?
What is the *unit of compilation* — the document, the
site, the user session?

**Depends on.** Vortex (which is itself a compilation
target), Gyre (layout is hard to compile), spiral-css
(cascade is dynamic), spiral-network (responses are
streams).

---

## #78 — "The Web as a Single Address Space"

**Pitch.** Every URL is a *function* in a global
namespace. `https://nytimes.com/today` is a function; you
call it. Sites are libraries. The browser is the linker.
The web stops being a graph of documents and becomes a
graph of *callable* values.

**What it would look like.** The address bar is a function
call, not a navigation. `cmd+L nytimes.com/today` calls
`nytimes.today()` and renders the result. The result is
typed — a `NewsArticle` is a structured value, not a DOM
tree. You can compose: `gitHub.issue(123)` returns an
issue; you can pipe it into a different site. The cross-
site composition problem (which Facebook, Google, and
every aggregation site solves with embeds and iframes)
disappears.

**Why it might fail.** This is the metaverse pitch without
the VR. The web is not a single address space by design —
the same-origin policy is fundamental to security. Cross-
origin function calls are an XSS nightmare. The web is
also deliberately polymorphic: a "news article" can be
rendered as HTML, JSON, RSS, AMP, schema.org, or any of a
dozen formats. Type-uniformity is impossible. The
adoption barrier is enormous: every site would have to
publish its address space as a typed interface, and the
existing web has 5 billion pages of untyped documents.

**Open questions.** What's the *type system*? TypeScript's?
A custom one? How are cross-origin calls authorised?
Every existing authentication scheme is HTTP-cookie-
based; function calls would need a different model.
Is the address space a *namespace* (hierarchical) or a
*graph* (with edges)?

**Depends on.** Vortex (JS function call model), spiral-
network (cross-origin request model), spiral-context
(capability model for cross-origin calls).

---

## #79 — "Time as a First-Class Dimension"

**Pitch.** The web is a 4D thing. URLs have a time
coordinate. `nytimes.com/today/2024-09-12T08:00:00Z` is a
valid URL. Browsers can show you the page *as it was* at
that time. The URL is no longer a 2D name; it's a
spacetime coordinate.

**What it would look like.** A slider in the URL bar:
drag it, the page changes. Or, the browser is always
storing the page's current state; `ctrl+Z` rewinds to a
previous state. The "you broke the page" panic is replaced
by "I broke the page, but here's a slider to go back."
"Recent changes" is a first-class browser feature. The
Wayback Machine is built in.

**Why it might fail.** Every site has to opt in by
serving historical snapshots. We already explored this as
#16 URL Time-Travel; the "first-class dimension" is the
radical version where the URL itself encodes time. But
the URL is the *primary identifier* of the page; adding a
time coordinate changes every link, every bookmark, every
caching layer. The web's "addressability" depends on URLs
being stable; adding time breaks that. And the *value* of
"see the page as it was" is mostly novelty — most pages
*change* in place, and a snapshot of a 4D coordinate is
just a static image of a moving target.

**Open questions.** Is the time coordinate continuous or
discrete? Who serves the historical snapshots — the site
or a third-party archive? How does this interact with
authentication — is "the page as you saw it 3 days ago"
a different page than "the page as you see it now"? What
about pages that have live state (chat, video)? Does the
slider work on those?

**Depends on.** spiral-network (URL parsing), Vortex
(state management), spiral-storage (archival).

---

## #80 — "The Anti-Browser"

**Pitch.** A browser that *refuses* most of the web. By
default, a curated set of 1,000 sites are accessible;
everything else requires a request. The user is *not* a
target audience for ad-tech. The browser is for the rest
of us — the people who want to read, not be marketed to.

**What it would look like.** A browser with a small
homepage of "approved" sites: Wikipedia, Project
Gutenberg, news sites that don't run ads, search engines
that don't track, email providers that don't share data.
The user can *request* access to other sites; the
request is queued for review. Over time, the user's
allowlist grows based on their needs. The browser has
*no* third-party cookie support. *No* fingerprinting
defence (because there's nothing to fingerprint — most
sites don't load). The browser's killer feature is that
it doesn't load the page at all if the page isn't on
the allowlist.

**Why it might fail.** This is a niche product, not a
general-purpose browser. Most people *want* the whole
web. The "1000 approved sites" is a curated list, which
is editorial — who decides? The browser would be
*permanently* a year behind on emerging sites. The
business model is unclear — if Spiral is independent and
sells nothing, who pays for the curation? The product
might be a *kind* of browser (like a kiosk browser or a
school browser) rather than a general browser.

**Open questions.** Who curates the allowlist? How does
a site *get* on the allowlist? What's the request flow?
Is the browser usable as a daily driver for someone who
works in tech (where new sites appear daily)? What's the
business model? Is the "anti-browser" framing marketing
or a real positioning?

**Depends on.** All of Spiral's engines, plus an
editorial/compliance team to curate the allowlist.

---

## #81 — "The Browser That Forgets on Purpose"

**Pitch.** Every page is rendered, then *forgotten*. The
DOM, the layout, the JS heap, the storage — all destroyed
30 seconds after the user navigates away. The browser
keeps *metadata* (the URL, the title, the favicon, the
last-view timestamp) so the user can re-find the page,
but not the *content*.

**What it would look like.** A browser with no cache, no
history of *content*, no scroll position to restore.
Just navigation. "Back" goes to the previous URL, but
the previous page is *re-fetched from scratch* every
time. The "you cleared your cookies" panic is impossible
because there's nothing to clear. Privacy is the default
state, not a setting. Ad-tech is impossible because
fingerprinting requires *state*, and state doesn't persist.

**Why it might fail.** Most of the web is built on the
assumption that browsers *remember*. Login sessions,
shopping carts, the "I was reading that article" feel of
revisiting a page — all depend on browser state. Removing
state breaks most pages. The "30 second" timeout is
arbitrary; the right value is "no state" but that's
unusable. The "back button" loses most of its value.
This is a philosophical statement, not a product.

**Open questions.** Is "forgetting" a *property* of the
browser, or is it a *mode* (e.g. a "private by design"
mode that's opt-in)? If opt-in, it's just private
browsing. If default, it's a different product. What's
the user's *workflow* — how do they re-find a page they
were reading? By bookmark? By history? The metadata
alone is not enough. Is the forgetting *session-based* or
*page-based*?

**Depends on.** All of Spiral's engines (everything
would need to be designed to forget).

---

## Cross-cutting thoughts on the five

These five ideas are **directions**, not commits. They
are not novel features in the standard sense; they are
*philosophical* reorientations of what a browser is.

Three observations:

1. **All five are *conservative*.** They restrict the web
   in some way (forget it, refuse it, give it type
   structure, give it time, compile it). This is the
   opposite of the "let's add more capability" instinct
   that has driven browser engineering for 30 years.
   Spiral's "no telemetry" value might predispose us to
   this kind of thinking. Or it might be a reaction to
   the bloat of the modern web.

2. **None of them is buildable as a v1.** They all
   require either (a) a new web standard, (b) a critical
   mass of opt-in sites, or (c) a marketing story that
   gets a non-trivial user base. Spiral at M4 has
   *zero* users. The anti-browser at M0 (a year from
   the start of Phase 2) has a much smaller barrier
   than the anti-browser at M84 (v1.0).

3. **All five could be a *sidebar project* of Spiral.**
   A fork of Spiral, a separate binary, an experimental
   build. The user can explore the directions without
   polluting the main codebase. The M4 audit
   methodology (research, audit, honest verdict) still
   applies.

---

## SSOT links

- [`docs/innovations-stubs.md`](innovations-stubs.md) — batch 1 (10 ideas)
- [`docs/innovations-stubs-2.md`](innovations-stubs-2.md) — batch 2 (11 ideas)
- [`docs/innovations-stubs-3.md`](innovations-stubs-3.md) — batch 3 (12 ideas)
- [`docs/innovations-stubs-4.md`](innovations-stubs-4.md) — batch 4 (32 ideas, audited)
- [`docs/innovations-routing.md`](innovations-routing.md) — wave vs main routing
- [`docs/innovations-index.md`](innovations-index.md) — one-page index
- [`docs/innovations-top-10.md`](innovations-top-10.md) — top 10 to build first
