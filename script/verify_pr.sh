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

for COMMIT in $COMMITS; do
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
echo "Results: $SIGNED/$TOTAL signed, $UNSIGNED unsigned"

if [ "$UNSIGNED" -gt 0 ]; then
    echo "❌ FAIL: $UNSIGNED commits missing provenance"
    exit 1
fi

echo "✅ PASS: All commits have provenance trailers"
