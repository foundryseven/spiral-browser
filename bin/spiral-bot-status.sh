#!/usr/bin/env bash
#
# bin/spiral-bot-status.sh
#
# Displays the last 5 runs and status of each bot workflow.
# Wraps the github CLI `gh`.

set -euo pipefail

# Check for gh CLI
if ! command -v gh &> /dev/null; then
  echo "Error: GitHub CLI (gh) is not installed or not in PATH." >&2
  exit 1
fi

echo "=== Spiral Browser Bot Workflows Status (Last 5 Runs) ==="
echo

WORKFLOWS=(
  "spiral-review.yml"
  "spiral-fix.yml"
  "spiral-external-gate.yml"
  "spiral-external-fix.yml"
)

for wf in "${WORKFLOWS[@]}"; do
  echo "------------------------------------------------------------"
  echo "Workflow: $wf"
  echo "------------------------------------------------------------"
  # Fetch run list from GitHub Actions
  gh run list --workflow "$wf" --limit 5 || echo "No runs found or unable to fetch status."
  echo
done
