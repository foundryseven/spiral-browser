---
paths:
  - "bin/**"
  - "justfile"
  - ".spiral/rules/**"
  - "AGENTS.md"
  - "docs/agents/**"
---

# Workflow Tooling Rules

These rules govern when the operational scripts in `bin/`
and recipes in `justfile` are invoked. They are the
single source of truth for "what tool, when" — the role
contracts in `docs/agents/` and the workflow table in
`AGENTS.md` both pull from here.

## Session Start (mandatory)

Every fresh session MUST begin with the context primer:

```bash
bin/spiral-context.sh             # no packet
bin/spiral-context.sh <packet-id> # picking up a specific packet
just context [<packet-id>]        # equivalent via justfile
```

Skipping this and reading files manually is a smell —
it means the implementer is paying the 15-20 minute
context re-load cost that the script was built to avoid.
The script also surfaces the packet's pre-expanded block
(spec, crates affected, call sites, tests expected,
end-to-end surface), which is otherwise invisible until
you scroll through the tracker.

## During Iteration (recommended)

While writing tests and code for a single packet, use the
scoped test target instead of the full workspace sweep:

```bash
just test-fast <crate> [pattern]   # in-cycle
just test-with-deps <crate>        # after a pub API change
```

The full `cargo test --workspace` is for pre-commit
verification only. Use it then, not during TDFlow.

## Pre-Commit Verification (mandatory)

Before claiming a packet complete OR before the user
asks to commit, run all six:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
./scripts/audit-orphan-exports.sh
./scripts/audit-doc-drift.sh
```

Or scoped, in one command:

```bash
just verify-packet <crate>
```

The PR script (`bin/spiral-pr.sh`) runs all six
automatically before pushing.

## Session End / PR Workflow (when a PR is wanted)

When the user wants a PR:

```bash
bin/spiral-pr.sh <packet-id>
```

The script is the entry point — do not invoke `gh pr
create` manually unless the script fails. Manually
bypassing the pre-flight checks is how orphan exports
and doc-drift regressions sneak into the trunk.

If the user does NOT want a PR, do not invoke the
script. Just commit when asked.

## Adding New Workflow Scripts

When a workflow pattern recurs (e.g. "open the tracker
in vim", "show recent test failures", "rotate the docs
SSOT"), add a script to `bin/` rather than expecting
every implementer to remember the underlying shell
incantation. See `bin/README.md` for the conventions.

When a script needs flags or has multiple modes, add
it to `justfile` as a recipe too, so `just --list`
surfaces it.

When the script's behaviour changes, update this rule
file AND `AGENTS.md` § Workflow Tools AND the relevant
role contract in `docs/agents/`. All three must agree;
otherwise the implementer will pick up the stale copy.

## Why this rule file exists

The role contracts (implementer, reviewer, architect,
etc.) describe WHAT an agent does. The `.spiral/rules/`
files describe HOW the code is written (architecture,
testing, coding-standards, unsafe-standards, performance).
This file is the bridge: it describes WHEN the
operational scripts that wrap the SSOT are invoked.

Without it, the implementer reads `implementer.md` and
sees "run cargo test" instead of "run `just test-fast`".
The session-pace improvements are real but only if they
are actually used.

Adopted 2026-06-17 per the implementer-agent ergonomics
review. See `bin/README.md` and `docs/agents/ledger-template.md`.