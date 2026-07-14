#!/bin/bash
set -euo pipefail

# This script groups changes by category and creates separate commits.
# Usage:
#   ./commit.sh              # commit + push
#   ./commit.sh --dry-run    # preview only, no commit/push
#   ./commit.sh --no-push    # commit without pushing

cd "$(dirname "$0")"

DRY_RUN=false
PUSH=true
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
        --no-push) PUSH=false ;;
    esac
done

# ── Helpers ──────────────────────────────────────────────────────

# Check if there's anything uncommitted.
has_changes() { ! git diff --cached --quiet 2>/dev/null && return 0 || git diff --quiet 2>/dev/null || git diff --cached --quiet 2>/dev/null; }

# Generate a commit message for a set of staged files using codetether.
gen_message() {
    local diff_stat diff_patch payload
    diff_stat="$(git diff --cached --stat | head -c 4000)"
    diff_patch="$(git diff --cached | head -c 36000)"
    payload="${diff_stat}
---
${diff_patch}"
    local model="${CODETETHER_COMMIT_MODEL:-openai-codex/gpt-5.6-sol:high}"
    local raw msg exit_code=0
    raw="$(RUST_LOG=error codetether run --model "$model" "Generate a conventional commit message for these git changes. Output ONLY the commit message in format 'type: description'. Types: feat|fix|refactor|docs|test|chore|perf|style|build|ci. Be specific about WHAT changed (file names, features, functions). One line only.

Git diff:
${payload}")" || exit_code=$?
    set +o pipefail
    msg="$(echo "$raw" \
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
    if [[ -z "$msg" ]]; then
        local files count
        files="$(git diff --cached --name-only | head -1)"
        count="$(git diff --cached --name-only | wc -l)"
        if [[ "$count" -eq 1 ]]; then
            msg="chore: update ${files}"
        else
            msg="chore: update ${count} files"
        fi
    fi
    echo "$msg"
}

# Stage specific files and commit them as a group.
commit_group() {
    local label="$1"; shift
    local msg
    git add "$@"
    if git diff --cached --quiet; then
        return 0  # nothing in this group
    fi
    msg="$(gen_message)"
    echo "  [$label] $msg"
    if [[ "$DRY_RUN" == "true" ]]; then return 0; fi
    git commit -m "$msg"
}

# ── Group definitions ───────────────────────────────────────────
# Each group: label | glob patterns
# Processed in order; first match wins for each file.

echo "Staging changes by category..."

# 1. Cargo / dependencies
commit_group "deps" \
    'Cargo.toml' 'Cargo.lock' ':**/Cargo.toml' ':**/Cargo.lock' 2>/dev/null || true

# 2. CI / build scripts
commit_group "ci" \
    'Dockerfile' 'Jenkinsfile' '*.yml' '*.yaml' \
    'check_file_limits.sh' 'commit.sh' 'release.sh' \
    'script/*.sh' '.github/**' 2>/dev/null || true

# 3. Documentation
commit_group "docs" \
    '*.md' ':**/*.md' 'docs/**' 'AGENTS.md' 2>/dev/null || true

# 4. TetherScript plugins
commit_group "tetherscript" \
    'examples/tetherscript/*.tether' 2>/dev/null || true

# 5. Tests
commit_group "tests" \
    ':**/tests/**' ':**/*test*' ':**/*_test*' 2>/dev/null || true

# 6. Source code (everything else under src/)
commit_group "src" 'src/**' 2>/dev/null || true

# 7. Remaining unstaged files
commit_group "misc" -A 2>/dev/null || true

# ── Summary ─────────────────────────────────────────────────────

COMMITTED=$(git log --oneline origin/$(git rev-parse --abbrev-ref HEAD)..HEAD 2>/dev/null | wc -l)
if [[ "$COMMITTED" -eq 0 ]]; then
    echo "Nothing to commit."
    exit 0
fi

echo ""
echo "Created $COMMITTED commit(s):"
git log --oneline origin/$(git rev-parse --abbrev-ref HEAD)..HEAD 2>/dev/null || \
    git log --oneline -"$COMMITTED"

if [[ "$DRY_RUN" == "true" ]]; then
    echo "(dry-run: no push)"
    exit 0
fi

if [[ "$PUSH" == "true" ]]; then
    BRANCH=$(git rev-parse --abbrev-ref HEAD)
    echo "Pushing to branch: $BRANCH"
    git push origin "$BRANCH"
fi
