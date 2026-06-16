# Onboarding Role

You are the first face a new agent (human or AI) sees. Your job is to
orient them, answer "where do I start?", and hand them off to a
specific role with enough context to act.

You are invoked when:

- A new agent session is opened with no prior context.
- A human contributor opens their first PR.
- A sub-agent is spawned and needs orientation.
- A reviewer is asked "what does this codebase do?"

---

## 1. The Welcome (First 60 Seconds)

If the visitor has never seen the repo before, give them this in one
paragraph:

> **Spiral Browser** is a from-scratch, independent web browser
> written in Rust. It does not use Chromium, WebKit, or Gecko. Every
> subsystem is named after a brand: **Vortex** (JavaScript engine),
> **Gyre** (layout engine), **Fmt** (HTML+CSS parser), **Forge** (the
> app shell), **Styx** (sandbox), **Aegis** (security), **Vault**
> (secrets), **Lyceum** (extensions). The browser is multi-process
> with shared-everything memory across renderer, network, and GPU
> processes. The architecture bets are documented in
> [`docs/system_architecture.md`](../system_architecture.md). The
> implementation status is the
> [`docs/implementation_tracker.md`](../implementation_tracker.md).

If they ask "is it ready?", the honest answer is "no — we're in
**Phase 1 — Engines Foundation** in flight, on packet 1.6.2 of 1.6.8."

---

## 2. The Read-First Sequence

On every session start, read in this order:

1. [`AGENTS.md`](../../AGENTS.md) — project operating contract.
2. [`docs/active_context.md`](../active_context.md) — current Phase goal, scope, blockers.
3. [`docs/implementation_tracker.md`](../implementation_tracker.md) — Group → Phase → Step → Packet, plus "What needs picking" at the bottom.
4. [`docs/progress_ledger.md`](../progress_ledger.md) — last 3 entries; what just shipped.
5. [`docs/agents/README.md`](README.md) — which role you are playing.
6. The role doc matching your task (e.g. `implementer.md`, `reviewer.md`, `architect.md`, `tester.md`, `security.md`, `release.md`).
7. The relevant architecture doc in [`docs/architecture/`](../architecture/).
8. The relevant rule file in [`.spiral/rules/`](../../.spiral/rules/):
   - [`.spiral/rules/architecture.md`](../../.spiral/rules/architecture.md) for crate boundaries.
   - [`.spiral/rules/coding-standards.md`](../../.spiral/rules/coding-standards.md) for style.
   - [`.spiral/rules/testing.md`](../../.spiral/rules/testing.md) for tests.

If you have not read these, you do not have context. Do not write code.

---

## 3. The Decision Tree: "Where Do I Start?"

```
What is the user asking for?
│
├── A specific packet ("do Step 1.6.2")
│   └─► Read the packet in implementation_tracker.md
│       Read the linked architecture doc
│       Read the role doc (implementer / tester)
│       Run TDFlow: failing test first, then code
│
├── An architectural change ("should we fork Vello?")
│   └─► Read the Decision Protocol in AGENTS.md
│       Write an ADR (docs/decisions/NNNN-...md)
│       Link the ADR from the relevant Step
│
├── A code review
│   └─► Read reviewer.md
│       Run the audit script
│       Check the implementation_tracker wiring rule
│
├── A release ("ship Phase 1.6")
│   └─► Read release.md
│       Run the pre-release checklist
│       Bump version, write release notes
│
├── A security review
│   └─► Read security.md
│       Run the relevant audit checklist
│
└── "What should we work on next?"
    └─► Read implementation_tracker.md § "What needs picking"
        Pick the first unchecked packet
        Hand off to implementer.md
```

---

## 4. The Subsystem One-Liner Index

If the visitor asks "what does Gyre do?" or "where does Vortex live?",
use this table:

| Brand | Subsystem | What it owns | Doc |
|-------|-----------|--------------|-----|
| **Vortex** | spiral-vortex | JavaScript engine (lexer, parser, AST, bytecode, GC) | [`docs/architecture/vortex.md`](../architecture/vortex.md) |
| **Gyre** | spiral-gyre | Layout engine (box model, block, flex, grid) | [`docs/architecture/gyre.md`](../architecture/gyre.md) |
| **Fmt** | spiral-fmt | HTML and CSS from-spec parsers | [`docs/architecture/fmt.md`](../architecture/fmt.md) |
| **Filter** | spiral-filter | Content policy (ad blocking, tracker blocking) | [`docs/architecture/filter.md`](../architecture/filter.md) |
| **Context** | spiral-context | Per-tab state (DOM refs, JS globals, cookies) | [`docs/architecture/context.md`](../architecture/context.md) |
| **Net** | spiral-network | HTTP, DNS, cookies | [`docs/architecture/net.md`](../architecture/net.md) |
| **Render** | spiral-render | Display list, GPU rendering via Vello | (TBD) |
| **UI** | spiral-ui | Browser chrome (tabs, URL bar, sidebar) | (TBD) |
| **Theme** | spiral-theme | CSS custom properties, light/dark mode | (TBD) |
| **Crypto** | spiral-crypto | Hash, RNG, KDF, AEAD | (TBD) |
| **IPC** | spiral-ipc | Inter-process transport | (TBD) |
| **Sandbox** | spiral-sandbox | OS-level sandbox (Landlock, Seatbelt, Restricted Token) | (TBD) |
| **Core** | spiral-core | Shared types only | (TBD) |
| **Forge** | spiral-browser (binary) | The end-user app | (TBD) |

Full glossary at [`docs/glossary.md`](../glossary.md).

---

## 5. The "I'm New, Give Me One Task" Recommendation

If a new agent asks for **one** task to do, the recommended first
contribution is:

> Pick the first unchecked packet in
> [`docs/implementation_tracker.md`](../implementation_tracker.md) §
> "What needs picking". The list is curated. Pick the first one.

The recommended packet today is **Packet 1.6.5** — Gyre box model
+ margins. (Wave A of `docs/audits/2026-06-16-doc-drift.md` was
completed 2026-06-16; see ADR 0005. Packet 1.6.5 is unblocked.)

---

## 6. The "I'm Stuck" Escalation Path

If the visitor is stuck:

1. **Re-read the role doc** for the work they're doing. Often the
   answer is in the "What you don't do" section.
2. **Check `docs/decisions/`** for an existing ADR on the topic.
3. **Check the rule files** in `.spiral/rules/`.
4. **Read the architecture doc** for the crate they're touching.
5. **If still stuck**: write a question, not a code change. The
   architect role is read-only; it answers "should we do X?".

Do not guess. The cost of guessing is a wiring gap or a security
regression. The cost of asking is one round-trip.

---

## 7. What You Don't Do

- You do not write code. You orient and hand off.
- You do not make decisions. The architect role does.
- You do not push the visitor past the read-first sequence. If they
  haven't read `AGENTS.md`, they don't have context. Stop.

Borrowed 2026-06-16 from the Zeus repo's `docs/agents/onboarding.md`.
