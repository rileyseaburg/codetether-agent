#!/bin/bash
set -euo pipefail

MAX_LINES=50
BASE_REF=${FILE_LIMIT_BASE_REF:-origin/main}
if ! BASE=$(git merge-base HEAD "$BASE_REF" 2>/dev/null); then
    echo "ERROR: unable to resolve FILE_LIMIT_BASE_REF '$BASE_REF'"
    echo "Fetch the base ref or set FILE_LIMIT_BASE_REF to a reachable commit."
    exit 1
fi
ERRORS=0

count_lines() {
    awk '
        /^[[:space:]]*$/ { next }
        /^[[:space:]]*\/\// { next }
        /^[[:space:]]*\/\*/ { in_block = 1 }
        in_block {
            if (/\*\//) in_block = 0
            next
        }
        { count++ }
        END { print count + 0 }
    ' "$@"
}

changed_files() {
    {
        git diff --name-only --diff-filter=ACMRT "$BASE" -- ':(glob)src/**/*.rs' ':(glob)crates/codetether-agent/src/**/*.rs'
        git ls-files --others --exclude-standard -- ':(glob)src/**/*.rs' ':(glob)crates/codetether-agent/src/**/*.rs'
    } | sort -u
}

fail() {
    echo "ERROR: $*"
    ERRORS=$((ERRORS + 1))
}

while IFS= read -r f; do
    [ -f "$f" ] || continue
    new=$(count_lines "$f")
    previous=$f
    if ! git cat-file -e "$BASE:$previous" 2>/dev/null; then
        case "$f" in
            crates/codetether-agent/src/*)
                legacy="src/${f#crates/codetether-agent/src/}"
                if [ ! -e "$legacy" ]; then
                    previous=$legacy
                fi
                ;;
        esac
    fi
    if ! git cat-file -e "$BASE:$previous" 2>/dev/null; then
        [ "$new" -le "$MAX_LINES" ] ||
            fail "New file $f has $new code lines (limit: $MAX_LINES)"
        continue
    fi
    old=$(git show "$BASE:$previous" | count_lines)
    if [ "$old" -le "$MAX_LINES" ] && [ "$new" -gt "$MAX_LINES" ]; then
        fail "$f grew past $MAX_LINES code lines ($old -> $new)"
    elif [ "$old" -gt "$MAX_LINES" ] && [ "$new" -gt "$old" ]; then
        fail "oversized $f grew ($old -> $new). Split before adding code."
    fi
done < <(changed_files)

if [ "$ERRORS" -gt 0 ]; then
    echo ""
    echo "FAILED: $ERRORS file(s) violate the application source line budget."
    exit 1
fi

echo "OK: changed application source files respect the global line-budget ratchet."
