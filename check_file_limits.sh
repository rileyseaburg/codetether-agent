#!/bin/bash
# Check that all NEW files (untracked or added) respect the 50-line limit.
# Also check that existing modified files didn't grow by more than 10 lines.
set -euo pipefail

MAX_NEW_FILE_LINES=50
MAX_GROWTH=10
ERRORS=0

# Check new (untracked + staged new) files
for f in $(git ls-files --others --exclude-standard -- 'src/tui/**/*.rs') \
         $(git diff --name-only --diff-filter=A HEAD -- 'src/tui/**/*.rs'); do
    [ -f "$f" ] || continue
    lines=$(grep -cve '^\s*$' "$f" | head -1)  # non-blank lines
    if [ "$lines" -gt "$MAX_NEW_FILE_LINES" ]; then
        echo "ERROR: New file $f has $lines non-blank lines (limit: $MAX_NEW_FILE_LINES)"
        ERRORS=$((ERRORS + 1))
    fi
done

# Check modified files didn't grow excessively
for f in $(git diff --name-only --diff-filter=M HEAD -- 'src/tui/**/*.rs'); do
    [ -f "$f" ] || continue
    old_lines=$(git show HEAD:"$f" 2>/dev/null | wc -l)
    new_lines=$(wc -l < "$f")
    growth=$((new_lines - old_lines))
    if [ "$growth" -gt "$MAX_GROWTH" ]; then
        echo "ERROR: $f grew by $growth lines (limit: +$MAX_GROWTH). Delegate to sub-modules."
        ERRORS=$((ERRORS + 1))
    fi
done

if [ "$ERRORS" -gt 0 ]; then
    echo ""
    echo "FAILED: $ERRORS file(s) violate coding standards."
    echo "Rules: New files <= $MAX_NEW_FILE_LINES non-blank lines. Existing files grow <= $MAX_GROWTH lines."
    exit 1
fi

echo "OK: All files pass coding standards checks."
