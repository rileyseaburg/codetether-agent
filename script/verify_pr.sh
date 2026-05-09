#!/usr/bin/env bash
# Verify codetether provenance signatures in a PR.
# Usage: ./verify_pr.sh <commit-range>
set -euo pipefail

RANGE="${1:?Usage: verify_pr.sh <commit-range>}"

echo "🔐 Verifying codetether provenance for: $RANGE"
echo ""

COMMITS=$(git log --format="%H" "$RANGE")
TOTAL=0
SIGNED=0
UNSIGNED=0
SKIPPED=0

is_github_pr_merge_commit() {
    local commit="$1"
    local parent_count
    parent_count=$(git show -s --format="%P" "$commit" | wc -w | tr -d ' ')
    [ "$parent_count" = "2" ] || return 1

    local subject
    subject=$(git log -1 --format="%s" "$commit")
    echo "$subject" | grep -Eq '^Merge [0-9a-f]{40} into [0-9a-f]{40}$'
}

for COMMIT in $COMMITS; do
    if is_github_pr_merge_commit "$COMMIT"; then
        SKIPPED=$((SKIPPED + 1))
        echo "  ⏭️  $COMMIT — skipping GitHub pull-request merge commit"
        continue
    fi

    TOTAL=$((TOTAL + 1))
    BODY=$(git log -1 --format="%b" "$COMMIT")
    if echo "$BODY" | grep -qi "^CodeTether-Provenance-ID:"; then
        SIGNED=$((SIGNED + 1))
        AGENT=$(echo "$BODY" | grep -i "^CodeTether-Agent-Name:" | head -n1 | sed 's/^[^:]*:[[:space:]]*//' || echo "unknown")
        echo "  ✅ $COMMIT — agent=${AGENT:-unknown}"
    else
        UNSIGNED=$((UNSIGNED + 1))
        echo "  ⚠️  $COMMIT — no provenance trailer"
    fi
done

echo ""
echo "Results: $SIGNED/$TOTAL signed, $UNSIGNED unsigned, $SKIPPED skipped"

if [ "$UNSIGNED" -gt 0 ]; then
    echo "❌ FAIL: $UNSIGNED commits missing provenance"
    exit 1
fi

echo "✅ PASS: All commits have provenance trailers"
