#!/usr/bin/env bash
# wait-and-review.sh
# Waits 610 minutes then reads PR #21 for feedback and addresses it.
set -euo pipefail

PR_NUMBER=21
REPO="rileyseaburg/codetether-agent"
WAIT_MINUTES=610
WAIT_SECS=$((WAIT_MINUTES * 60))
LOG="/tmp/pr_review_$(date +%s).log"

echo "[$(date)] Starting ${WAIT_MINUTES}-minute wait before reviewing PR #${PR_NUMBER}" | tee "$LOG"
echo "[$(date)] Will review at approximately $(date -d "+${WAIT_MINUTES} minutes" 2>/dev/null || date -v+${WAIT_MINUTES}M 2>/dev/null || echo "in ${WAIT_MINUTES} minutes")" | tee -a "$LOG"

sleep "$WAIT_SECS"

echo "[$(date)] Wait complete. Reading PR #${PR_NUMBER} feedback..." | tee -a "$LOG"

# Fetch PR comments
echo "=== PR Comments ===" | tee -a "$LOG"
gh pr view "$PR_NUMBER" --comments 2>&1 | tee -a "$LOG"

echo "" | tee -a "$LOG"
echo "=== PR Review Comments ===" | tee -a "$LOG"
gh api "repos/${REPO}/pulls/${PR_NUMBER}/comments" 2>&1 | python3 -c "
import sys, json
try:
    comments = json.load(sys.stdin)
    for c in comments:
        print(f'--- {c.get(\"user\",{}).get(\"login\",\"?\")} @ {c.get(\"created_at\",\"?\")} ---')
        print(f'File: {c.get(\"path\",\"?\")}:{c.get(\"line\",\"?\")}')
        print(c.get('body',''))
        print()
except:
    print('No review comments or parse error')
" | tee -a "$LOG"

echo "" | tee -a "$LOG"
echo "=== PR Reviews ===" | tee -a "$LOG"
gh api "repos/${REPO}/pulls/${PR_NUMBER}/reviews" 2>&1 | python3 -c "
import sys, json
try:
    reviews = json.load(sys.stdin)
    for r in reviews:
        print(f'--- {r.get(\"user\",{}).get(\"login\",\"?\")} ({r.get(\"state\",\"?\")}) ---')
        print(r.get('body','(no body)'))
        print()
except:
    print('No reviews or parse error')
" | tee -a "$LOG"

echo "" | tee -a "$LOG"
echo "[$(date)] Review complete. Log saved to $LOG" | tee -a "$LOG"
