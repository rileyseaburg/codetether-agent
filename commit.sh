#!/bin/bash
set -euo pipefail

# This script creates an auditable commit with an AI-generated message.
# Usage:
#   ./commit.sh
#   ./commit.sh --dry-run

cd "$(dirname "$0")"

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
fi

# In dry-run mode, preserve caller's original staging state.
INDEX_BACKUP=""
if [[ "$DRY_RUN" == "true" ]]; then
    INDEX_BACKUP="$(mktemp)"
    cp .git/index "$INDEX_BACKUP"
    trap 'if [[ -n "$INDEX_BACKUP" ]]; then cp "$INDEX_BACKUP" .git/index && rm -f "$INDEX_BACKUP"; fi' EXIT
fi

# Stage all changes.
git add -A

# Check there's something to commit.
if git diff --cached --quiet; then
    echo "Nothing to commit."
    exit 0
fi

# Build a bounded prompt payload for reliability.
DIFF_STAT="$(git diff --cached --stat)"
DIFF_PATCH="$(git diff --cached)"
DIFF_PATCH_TRUNCATED="$(printf "%s" "$DIFF_PATCH" | cut -c1-40000)"
DIFF="${DIFF_STAT}
---
${DIFF_PATCH_TRUNCATED}"

echo "Generating commit message..."

RAW_OUTPUT=""
CODETETHER_EXIT=0
if ! RAW_OUTPUT="$(RUST_LOG=error codetether run "You are a commit message generator. Based on the following git diff, produce ONLY a single-line conventional commit message (type: description). No explanation, no markdown, no quotes. Just the commit message.

${DIFF}" 2>/dev/null)"; then
    CODETETHER_EXIT=$?
fi

# Strip ANSI escape codes, then keep the first likely message line.
COMMIT_MSG="$(echo "$RAW_OUTPUT" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | awk '
      /^\[Session:/ { next }
      /^[0-9]{4}-[0-9]{2}-[0-9]{2}T/ { next }
      /^(INFO|WARN|DEBUG|ERROR) / { next }
      /^codetether::/ { next }
      /^Crash reporting/ { next }
      /^$/ { next }
      { print; exit }
    ' \
  | sed 's/^["'\''`]*//;s/["'\''`]*$//' \
  | xargs || true)"

if [[ -z "$COMMIT_MSG" ]]; then
    if [[ "$CODETETHER_EXIT" -ne 0 ]]; then
        echo "Warning: codetether run exited with status $CODETETHER_EXIT."
    fi
    COMMIT_MSG="chore: update repository changes"
    echo "Warning: using fallback commit message: $COMMIT_MSG"
fi

echo "Commit message: $COMMIT_MSG"

if [[ "$DRY_RUN" == "true" ]]; then
    echo "Dry run enabled; skipping git commit and push."
    exit 0
fi

git commit -m "$COMMIT_MSG"
git push origin main
