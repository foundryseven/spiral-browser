#!/usr/bin/env bash
#
# bin/install-bot-secrets.sh
#
# Interactive helper to set the repo-level secrets needed by the bot
# workflows. Wraps the github CLI `gh`.
#
# Secrets set:
#   - OPENCODE_API_KEY       (required for internal review & fix)
#   - FORK_REVIEWER_WEBHOOK  (required for external review gate)

set -euo pipefail

# Check for gh CLI
if ! command -v gh &> /dev/null; then
  echo "Error: GitHub CLI (gh) is not installed or not in PATH." >&2
  exit 1
fi

echo "=== Spiral Browser Bot Secrets Installer ==="
echo

# 1. Install OPENCODE_API_KEY
echo "Please enter your OpenCode Go API Key."
read -rs -p "OPENCODE_API_KEY: " opencode_key
echo
if [ -z "$opencode_key" ]; then
  echo "Error: Key cannot be empty." >&2
  exit 1
fi

# 2. Install FORK_REVIEWER_WEBHOOK
echo
echo "Please enter the webhook URL for your Cloudflare Worker."
read -r -p "FORK_REVIEWER_WEBHOOK: " webhook_url
if [ -z "$webhook_url" ]; then
  echo "Error: Webhook URL cannot be empty." >&2
  exit 1
fi

echo
echo "Uploading secrets to GitHub..."
gh secret set OPENCODE_API_KEY --body "$opencode_key"
gh secret set FORK_REVIEWER_WEBHOOK --body "$webhook_url"

echo
echo "Bot secrets successfully configured!"
