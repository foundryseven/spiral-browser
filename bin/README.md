# Spiral Browser — `bin/` Scripts

Operational scripts for Spiral Browser. These wrap common
tasks so an implementer (human or LLM) does not have to
remember flag combinations.

## Scripts

| Script | Purpose | Run-time |
|--------|---------|----------|
| `spiral-context.sh` | Print the 5-10 files most relevant to a given packet (or session start). The biggest session-pace win — kills the 15-20 minute context re-load that every fresh LLM session pays. | <1s |
| `spiral-pr.sh` | Pre-flight checks + push + open PR for a packet. Wraps `cargo fmt`, `cargo clippy`, `cargo test --workspace`, the audit scripts, and `gh pr create` so PRs always go out clean. | 1-3 min |

## Conventions

- Scripts live under `bin/`, not `scripts/`. `scripts/` is
  for build-time tooling (audits, codegen); `bin/` is for
  human-time tooling (workflow, navigation).
- Scripts are POSIX-ish bash, not pure POSIX. We rely on
  `[[ ]]`, `${var,,}` (lowercase), and Bash 3.2+ features.
  Tested on macOS (Bash 3.2 via Homebrew coreutils) and Linux
  (Bash 5+).
- Scripts accept `--help` and print their own usage.
- Scripts exit non-zero on real failures; warnings print to
  stderr but don't fail the script.
- All scripts `cd` to the repo root on entry so they can be
  called from anywhere: `bin/spiral-context.sh 2.1.2`.

## Adding a new script

1. Put it under `bin/`, name it `spiral-<verb>.sh`.
2. `chmod +x` it. The first line must be `#!/usr/bin/env bash`.
3. Add a header comment block (5-15 lines) explaining what,
   why, and how. Adopt the style of the existing scripts.
4. Update the table above.
5. If it's complex enough to need its own `--help`, write it
   as a heredoc in the script body (`usage() { sed -n ... }`).
6. If it shells out to `cargo` or `gh`, gate with `--dry-run`
   so callers can preview the side-effects.

## Why not a single tool?

`just` already exists for build-time recipes (see `justfile`).
The `bin/` scripts target a different audience: the
human/agent who is navigating the codebase, not the human who
is building it. They print context, not build artefacts.
Keeping them separate avoids `just` becoming a dumping ground
for every workflow idea.

Adopted 2026-06-17 per the implementer-agent ergonomics review.