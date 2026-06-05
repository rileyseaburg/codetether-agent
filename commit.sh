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

# Build a bounded prompt payload for reliability. The stat alone can be huge
# for big refactors (200+ files), so cap it. ARG_MAX on Linux is ~2 MiB but
# leaving it uncapped is a latent bug — codetether run was silently failing
# with "Argument list too long" for large diffs.
DIFF_TMPDIR="$(mktemp -d)"
trap 'rm -rf "$DIFF_TMPDIR"' EXIT
git diff --cached --stat > "$DIFF_TMPDIR/stat"
git diff --cached              > "$DIFF_TMPDIR/patch"
# Truncate with `dd` (no SIGPIPE on the upstream git process) and read back.
DIFF_STAT="$(head -c 4000 "$DIFF_TMPDIR/stat")"
DIFF_PATCH="$(head -c 36000 "$DIFF_TMPDIR/patch")"
DIFF="${DIFF_STAT}
---
${DIFF_PATCH}"

echo "Generating commit message..."

# Use a capable model for commit message generation.
COMMIT_MODEL="${CODETETHER_COMMIT_MODEL:-zai/glm-5.1}"

RAW_OUTPUT=""
CODETETHER_EXIT=0
# Note: stderr is intentionally captured (not /dev/null'd) so the user sees
# real failures like "Argument list too long" or model errors. Stderr is
# stripped from the final COMMIT_MSG via the `grep -v` filters below.
if ! RAW_OUTPUT="$(RUST_LOG=error codetether run --model "$COMMIT_MODEL" "Generate a conventional commit message for these git changes. Output ONLY the commit message in format 'type: description'. Types: feat|fix|refactor|docs|test|chore|perf|style|build|ci. Be specific about WHAT changed (file names, features, functions). Examples:
- feat: add user authentication to login.rs
- fix: resolve null pointer in parse_config
- refactor: extract validation logic into validator.rs
- docs: update AGENTS.md with new tool guidelines

Git diff:
${DIFF}")"; then
    CODETETHER_EXIT=$?
fi

# Strip ANSI codes, markdown, session footer, and log noise.
# `head -1` causes SIGPIPE upstream under `set -o pipefail`, so disable it
# briefly for the filter pipeline. The trailing `|| true` handles empty results.
set +o pipefail
COMMIT_MSG="$(echo "$RAW_OUTPUT" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | sed 's/^```[a-z]*//;s/^```$//' \
  | grep -v '^\[Session:' \
  | grep -v '^[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}T' \
  | grep -v '^\(INFO\|WARN\|DEBUG\|ERROR\) ' \
  | grep -v '^codetether::' \
  | grep -v '^Crash reporting' \
  | grep -v '^Continue with:' \
  | sed -e 's/^[[:space:]]*//' -e 's/^["'\''`]*//' \
  | grep -E '^(feat|fix|refactor|docs|test|chore|perf|style|build|ci):' \
  | head -1 \
  | sed 's/^["'\''`]*//;s/["'\''`]*$//' \
  | xargs || true)"
set -o pipefail

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
