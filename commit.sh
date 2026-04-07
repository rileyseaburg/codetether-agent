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

# Use a capable model for commit message generation.
COMMIT_MODEL="${CODETETHER_COMMIT_MODEL:-zai/glm-5.1}"

RAW_OUTPUT=""
CODETETHER_EXIT=0
if ! RAW_OUTPUT="$(RUST_LOG=error codetether run --model "$COMMIT_MODEL" "Generate a conventional commit message for these git changes. Output ONLY the commit message in format 'type: description'. Types: feat|fix|refactor|docs|test|chore|perf|style|build|ci. Be specific about WHAT changed (file names, features, functions). Examples:
- feat: add user authentication to login.rs
- fix: resolve null pointer in parse_config
- refactor: extract validation logic into validator.rs
- docs: update AGENTS.md with new tool guidelines

Git diff:
${DIFF}" 2>/dev/null)"; then
    CODETETHER_EXIT=$?
fi

# Strip ANSI codes, markdown, session footer, and log noise
COMMIT_MSG="$(echo "$RAW_OUTPUT" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | sed 's/^```[a-z]*//;s/^```$//' \
  | grep -v '^\[Session:' \
  | grep -v '^[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}T' \
  | grep -v '^\(INFO\|WARN\|DEBUG\|ERROR\) ' \
  | grep -v '^codetether::' \
  | grep -v '^Crash reporting' \
  | grep -v '^Continue with:' \
  | sed 's/^[[:space:]]*//' \
  | grep -E '^(feat|fix|refactor|docs|test|chore|perf|style|build|ci):' \
  | head -1 \
  | sed 's/^["'\''`]*//;s/["'\''`]*$//' \
  | xargs || true)"

if [[ -z "$COMMIT_MSG" ]]; then
    if [[ "$CODETETHER_EXIT" -ne 0 ]]; then
        echo "Warning: codetether run exited with status $CODETETHER_EXIT."
        echo "Raw output was: $RAW_OUTPUT"
    fi
    
    # Generate context-aware message based on file changes
    CHANGED_FILES="$(git diff --cached --name-only | head -5)"
    FILE_COUNT="$(git diff --cached --name-only | wc -l)"
    
    if echo "$CHANGED_FILES" | grep -qE '\.md$'; then
        COMMIT_MSG="docs: update documentation"
    elif echo "$CHANGED_FILES" | grep -qE 'sessions/.*\.json$'; then
        COMMIT_MSG="chore: update session data"
    elif echo "$CHANGED_FILES" | grep -qE 'test'; then
        COMMIT_MSG="test: update test files"
    elif echo "$CHANGED_FILES" | grep -qE 'Cargo\.(toml|lock)'; then
        COMMIT_MSG="chore: update dependencies"
    elif [[ "$FILE_COUNT" -eq 1 ]]; then
        COMMIT_MSG="chore: update $(echo "$CHANGED_FILES" | head -1)"
    else
        COMMIT_MSG="chore: update $FILE_COUNT files"
    fi
    echo "Warning: using generated commit message: $COMMIT_MSG"
fi

echo "Commit message: $COMMIT_MSG"

if [[ "$DRY_RUN" == "true" ]]; then
    echo "Dry run enabled; skipping git commit and push."
    exit 0
fi

git commit -m "$COMMIT_MSG"
# Get current branch name and push to it
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
echo "Pushing to branch: $CURRENT_BRANCH"
git push origin "$CURRENT_BRANCH"
