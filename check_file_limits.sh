#!/bin/bash
set -euo pipefail

MAX_LINES=50
BASE_REF=${FILE_LIMIT_BASE_REF:-origin/main}
BASE=$(git merge-base HEAD "$BASE_REF" 2>/dev/null || echo HEAD)
ERRORS=0

count_lines() {
    awk '
        /^[[:space:]]*$/ { next }
        /^[[:space:]]*\/\// { next }
        /^[[:space:]]*\/\*/ { next }
        /^[[:space:]]*\*/ { next }
        { count++ }
        END { print count + 0 }
    ' "$@"
}

changed_files() {
    {
        git diff --name-only --diff-filter=AM "$BASE" -- 'src/**/*.rs'
        git ls-files --others --exclude-standard -- 'src/**/*.rs'
    } | sort -u
}

fail() {
    echo "ERROR: $*"
    ERRORS=$((ERRORS + 1))
}

while IFS= read -r f; do
    [ -f "$f" ] || continue
    new=$(count_lines "$f")
    if ! git cat-file -e "$BASE:$f" 2>/dev/null; then
        [ "$new" -le "$MAX_LINES" ] ||
            fail "New file $f has $new code lines (limit: $MAX_LINES)"
        continue
    fi
    old=$(git show "$BASE:$f" | count_lines)
    if [ "$old" -le "$MAX_LINES" ] && [ "$new" -gt "$MAX_LINES" ]; then
        fail "$f grew past $MAX_LINES code lines ($old -> $new)"
    elif [ "$old" -gt "$MAX_LINES" ] && [ "$new" -gt "$old" ]; then
        fail "oversized $f grew ($old -> $new). Split before adding code."
    fi
done < <(changed_files)

if [ "$ERRORS" -gt 0 ]; then
    echo ""
    echo "FAILED: $ERRORS file(s) violate global src/**/*.rs line budget."
    exit 1
fi

echo "OK: changed src/**/*.rs files respect the global line-budget ratchet."
