#!/usr/bin/env bash
# CodeTether GitHub Action entrypoint
# Supports two modes:
#   local  — runs the agent in the GH Actions runner
#   server — dispatches a review task to an A2A server
set -euo pipefail

# ── Clear empty credential env vars to avoid panic ───────────────
# vaultrs panics on empty VAULT_ADDR; codetether should fall back to env providers
[ -z "${VAULT_ADDR:-}" ] && unset VAULT_ADDR
[ -z "${VAULT_TOKEN:-}" ] && unset VAULT_TOKEN
[ -z "${GITHUB_COPILOT_TOKEN:-}" ] && unset GITHUB_COPILOT_TOKEN

# ── Map generic API key to provider-specific env var ─────────────
if [ -n "${CODETETHER_API_KEY:-}" ]; then
  MODEL="${CODETETHER_DEFAULT_MODEL:-glm-5.1}"
  case "$MODEL" in
    glm*|zhipu*)   export ZAI_API_KEY="$CODETETHER_API_KEY" ;;
    gpt*|o1*|o3*)  export OPENAI_API_KEY="$CODETETHER_API_KEY" ;;
    claude*)       export ANTHROPIC_API_KEY="$CODETETHER_API_KEY" ;;
    copilot/*)     export GITHUB_COPILOT_TOKEN="$CODETETHER_API_KEY" ;;
    *)             export OPENAI_API_KEY="$CODETETHER_API_KEY" ;;
  esac
  unset CODETETHER_API_KEY
fi

# ── Issue mode: use issue body directly as prompt ────────────────
if [ "${GITHUB_EVENT_NAME:-}" = "issues" ]; then
  echo "::group::Processing issue #${PR_NUMBER}"

  PROMPT="You are responding to GitHub Issue #${PR_NUMBER}: \"${PR_TITLE}\" in ${REPO_FULL_NAME}.

Analyze the issue and provide a thorough response. If it's a bug report, suggest a fix. If it's a feature request, discuss implementation approach.

${INPUT_EXTRA_PROMPT:+Additional instructions: ${INPUT_EXTRA_PROMPT}}

Issue body:
${PR_BODY:-No description provided.}"

  REVIEW_TEXT=""

  # ── Dispatch to server (issue mode) ─────────────────────────
  if [ "$INPUT_MODE" = "server" ]; then
    echo "Dispatching issue task to A2A server..."

    TASK_PAYLOAD=$(jq -n \
      --arg description "$PROMPT" \
      --arg agent_type "$INPUT_AGENT_TYPE" \
      --arg title "Issue #${PR_NUMBER}: ${PR_TITLE}" \
      --arg repo "$REPO_FULL_NAME" \
      --arg pr_number "$PR_NUMBER" \
      '{
        title: $title,
        description: $description,
        agent_type: $agent_type,
        metadata: {
          source: "github-actions",
          repo: $repo,
          issue_number: ($pr_number | tonumber)
        }
      }')

    RESPONSE=$(curl -fsSL \
      -X POST "${CODETETHER_SERVER}/v1/tasks/dispatch" \
      -H "Authorization: Bearer ${CODETETHER_TOKEN}" \
      -H "Content-Type: application/json" \
      -d "$TASK_PAYLOAD")

    TASK_ID=$(echo "$RESPONSE" | jq -r '.task_id // "unknown"')
    echo "Task dispatched: ${TASK_ID}"

    if [ "$TASK_ID" = "unknown" ]; then
      echo "::error::Failed to dispatch task: ${RESPONSE}"
      echo "review=Failed to dispatch task." >> "$GITHUB_OUTPUT"
      echo "exit_code=1" >> "$GITHUB_OUTPUT"
      exit 1
    fi

    # Poll for completion
    MAX_POLL=60
    POLL_INTERVAL=5
    POLL_COUNT=0
    TASK_STATUS="pending"

    while [ "$POLL_COUNT" -lt "$MAX_POLL" ] && [ "$TASK_STATUS" != "completed" ] && [ "$TASK_STATUS" != "failed" ] && [ "$TASK_STATUS" != "canceled" ]; do
      sleep "$POLL_INTERVAL"
      POLL_COUNT=$((POLL_COUNT + 1))
      TASK_RESPONSE=$(curl -fsSL \
        -H "Authorization: Bearer ${CODETETHER_TOKEN}" \
        "${CODETETHER_SERVER}/v1/tasks/dispatch/${TASK_ID}" 2>/dev/null || echo '{}')
      TASK_STATUS=$(echo "$TASK_RESPONSE" | jq -r '.status // "unknown"')
      echo "  Poll ${POLL_COUNT}/${MAX_POLL}: status=${TASK_STATUS}"
    done

    if [ "$TASK_STATUS" = "completed" ]; then
      RESULT_RESPONSE=$(curl -fsSL \
        -H "Authorization: Bearer ${CODETETHER_TOKEN}" \
        "${CODETETHER_SERVER}/v1/tasks/dispatch/${TASK_ID}")
      RESULT_TEXT=$(echo "$RESULT_RESPONSE" | jq -r '.result // "No response returned."')
      REVIEW_TEXT=$(echo "$RESULT_TEXT" | head -c 65000)
    elif [ "$TASK_STATUS" = "failed" ]; then
      REVIEW_TEXT="Issue task failed. Check server logs for task ${TASK_ID}."
    else
      REVIEW_TEXT="Issue task timed out after $((MAX_POLL * POLL_INTERVAL))s (status: ${TASK_STATUS}). Task ID: ${TASK_ID}"
    fi
  fi

  # ── Post issue comment ──────────────────────────────────────
  if [ "${INPUT_AUTO_COMMENT}" = "true" ] && [ -n "${PR_NUMBER:-}" ] && [ -n "$REVIEW_TEXT" ]; then
    COMMENT_BODY="## 🤖 CodeTether Response

<details>
<summary>Issue #${PR_NUMBER}: ${PR_TITLE}</summary>

Task: \`${TASK_ID:-local}\`

</details>

${REVIEW_TEXT}"

    if [ ${#COMMENT_BODY} -gt 65000 ]; then
      COMMENT_BODY="${COMMENT_BODY:0:64900}

..._truncated (response exceeded comment size limit)_"
    fi

    curl -fsSL \
      -X POST \
      -H "Authorization: token ${GITHUB_TOKEN}" \
      -H "Accept: application/vnd.github.v3+json" \
      "https://api.github.com/repos/${REPO_FULL_NAME}/issues/${PR_NUMBER}/comments" \
      -d "$(jq -n --arg body "$COMMENT_BODY" '{body: $body}')" \
      > /dev/null

    echo "Response posted to Issue #${PR_NUMBER}"
  fi

  echo "exit_code=0" >> "$GITHUB_OUTPUT"
  {
    echo "review<<CODETETHER_EOF"
    echo "$REVIEW_TEXT"
    echo "CODETETHER_EOF"
  } >> "$GITHUB_OUTPUT"

  echo "::endgroup::"
  exit 0
fi

# ── Gather PR diff ───────────────────────────────────────────────
echo "::group::Fetching PR diff"
DIFF_FILE="$(mktemp)"
git diff "origin/${PR_BASE}...HEAD" -- '*.rs' '*.py' '*.ts' '*.js' '*.go' '*.java' '*.tsx' '*.jsx' '*.yml' '*.yaml' '*.toml' > "$DIFF_FILE" 2>/dev/null || true

DIFF_LINES=$(wc -l < "$DIFF_FILE")
echo "Diff: ${DIFF_LINES} lines"

if [ "$DIFF_LINES" -eq 0 ]; then
  echo "No code changes detected — skipping review."
  echo "review=No code changes to review." >> "$GITHUB_OUTPUT"
  echo "exit_code=0" >> "$GITHUB_OUTPUT"
  exit 0
fi

# Truncate massive diffs to avoid blowing context limits
MAX_DIFF_LINES=3000
if [ "$DIFF_LINES" -gt "$MAX_DIFF_LINES" ]; then
  echo "⚠ Diff truncated to ${MAX_DIFF_LINES} lines (was ${DIFF_LINES})"
  head -n "$MAX_DIFF_LINES" "$DIFF_FILE" > "${DIFF_FILE}.trunc"
  mv "${DIFF_FILE}.trunc" "$DIFF_FILE"
fi
echo "::endgroup::"

# ── Build review prompt ──────────────────────────────────────────
PROMPT="You are reviewing PR #${PR_NUMBER}: \"${PR_TITLE}\" (${PR_HEAD} → ${PR_BASE}).

Review the following diff and report:
1. **Bugs** — logic errors, off-by-one, null/unwrap safety
2. **Security** — OWASP Top 10, injection, auth bypass, secrets exposure
3. **Performance** — unnecessary allocations, O(n²) patterns, missing indexes
4. **Style** — naming, dead code, missing error handling

Be concise. Only comment on real issues, not nitpicks.
If the code looks good, say so briefly.

${INPUT_EXTRA_PROMPT:+Additional instructions: ${INPUT_EXTRA_PROMPT}}

\`\`\`diff
$(cat "$DIFF_FILE")
\`\`\`"

# ── Mode: server ─────────────────────────────────────────────────
if [ "$INPUT_MODE" = "server" ]; then
  echo "::group::Dispatching review task to A2A server"

  if [ -z "${CODETETHER_SERVER:-}" ]; then
    echo "::error::server_url is required in server mode"
    exit 1
  fi
  if [ -z "${CODETETHER_TOKEN:-}" ]; then
    echo "::error::token is required in server mode"
    exit 1
  fi

  TASK_PAYLOAD=$(jq -n \
    --arg description "$PROMPT" \
    --arg agent_type "$INPUT_AGENT_TYPE" \
    --arg title "PR Review: #${PR_NUMBER} ${PR_TITLE}" \
    --arg repo "$REPO_FULL_NAME" \
    --arg pr_number "$PR_NUMBER" \
    '{
      title: $title,
      description: $description,
      agent_type: $agent_type,
      metadata: {
        source: "github-actions",
        repo: $repo,
        pr_number: ($pr_number | tonumber)
      }
    }')

  RESPONSE=$(curl -fsSL \
    -X POST "${CODETETHER_SERVER}/v1/tasks/dispatch" \
    -H "Authorization: Bearer ${CODETETHER_TOKEN}" \
    -H "Content-Type: application/json" \
    -H "Idempotency-Key: pr-review-${REPO_FULL_NAME//\//-}-${PR_NUMBER}-${GITHUB_SHA:0:8}" \
    -d "$TASK_PAYLOAD")

  TASK_ID=$(echo "$RESPONSE" | jq -r '.task_id // "unknown"')
  echo "Task dispatched: ${TASK_ID}"

  if [ "$TASK_ID" = "unknown" ]; then
    echo "::error::Failed to dispatch task: ${RESPONSE}"
    echo "review=Failed to dispatch review task." >> "$GITHUB_OUTPUT"
    echo "exit_code=1" >> "$GITHUB_OUTPUT"
    exit 1
  fi

  # ── Poll for task completion ─────────────────────────────────
  echo "Waiting for task to complete..."
  MAX_POLL=60
  POLL_INTERVAL=5
  POLL_COUNT=0
  TASK_STATUS="pending"

  while [ "$POLL_COUNT" -lt "$MAX_POLL" ] && [ "$TASK_STATUS" != "completed" ] && [ "$TASK_STATUS" != "failed" ] && [ "$TASK_STATUS" != "canceled" ]; do
    sleep "$POLL_INTERVAL"
    POLL_COUNT=$((POLL_COUNT + 1))

    TASK_RESPONSE=$(curl -fsSL \
      -H "Authorization: Bearer ${CODETETHER_TOKEN}" \
      "${CODETETHER_SERVER}/v1/tasks/dispatch/${TASK_ID}" 2>/dev/null || echo '{}')

    TASK_STATUS=$(echo "$TASK_RESPONSE" | jq -r '.status // "unknown"')
    echo "  Poll ${POLL_COUNT}/${MAX_POLL}: status=${TASK_STATUS}"
  done

  if [ "$TASK_STATUS" = "completed" ]; then
    # Fetch completed task with result from dispatch API
    RESULT_RESPONSE=$(curl -fsSL \
      -H "Authorization: Bearer ${CODETETHER_TOKEN}" \
      "${CODETETHER_SERVER}/v1/tasks/dispatch/${TASK_ID}")
    RESULT_TEXT=$(echo "$RESULT_RESPONSE" | jq -r '.result // "No review text returned."')

    REVIEW_TEXT=$(echo "$RESULT_TEXT" | head -c 65000)
  elif [ "$TASK_STATUS" = "failed" ]; then
    REVIEW_TEXT="Review task failed. Check server logs for task ${TASK_ID}."
  else
    REVIEW_TEXT="Review task timed out after $((MAX_POLL * POLL_INTERVAL))s (status: ${TASK_STATUS}). Task ID: ${TASK_ID}"
  fi

  echo "exit_code=0" >> "$GITHUB_OUTPUT"
  {
    echo "review<<CODETETHER_EOF"
    echo "$REVIEW_TEXT"
    echo "CODETETHER_EOF"
  } >> "$GITHUB_OUTPUT"

  # ── Post PR comment (server mode) ─────────────────────────────
  if [ "${INPUT_AUTO_COMMENT}" = "true" ] && [ -n "${PR_NUMBER:-}" ] && [ "$TASK_STATUS" = "completed" ]; then
    COMMENT_BODY="## 🔍 CodeTether Review

<details>
<summary>PR #${PR_NUMBER}: ${PR_TITLE}</summary>

Mode: server · Task: \`${TASK_ID}\`

</details>

${REVIEW_TEXT}"

    if [ ${#COMMENT_BODY} -gt 65000 ]; then
      COMMENT_BODY="${COMMENT_BODY:0:64900}

..._truncated (review exceeded comment size limit)_"
    fi

    curl -fsSL \
      -X POST \
      -H "Authorization: token ${GITHUB_TOKEN}" \
      -H "Accept: application/vnd.github.v3+json" \
      "https://api.github.com/repos/${REPO_FULL_NAME}/issues/${PR_NUMBER}/comments" \
      -d "$(jq -n --arg body "$COMMENT_BODY" '{body: $body}')" \
      > /dev/null

    echo "Review posted to PR #${PR_NUMBER}"
  fi

  echo "::endgroup::"
  exit 0
fi

# ── Mode: local ──────────────────────────────────────────────────
echo "::group::Running CodeTether review"

REVIEW_FILE="$(mktemp)"

# Build args — --max-steps only available in >= 4.5.0
RUN_ARGS=()
if codetether run --help 2>&1 | grep -q -- '--max-steps'; then
  RUN_ARGS+=(--max-steps "${INPUT_MAX_STEPS}")
fi

codetether run \
  "${RUN_ARGS[@]}" \
  "$PROMPT" \
  2>&1 | tee "$REVIEW_FILE"

EXIT_CODE=${PIPESTATUS[0]:-0}
echo "exit_code=${EXIT_CODE}" >> "$GITHUB_OUTPUT"
echo "::endgroup::"

# ── Capture output ───────────────────────────────────────────────
# GitHub outputs have a 1MB limit; truncate if needed
REVIEW_TEXT=$(head -c 65000 "$REVIEW_FILE")

# Use heredoc delimiter for multi-line output
{
  echo "review<<CODETETHER_EOF"
  echo "$REVIEW_TEXT"
  echo "CODETETHER_EOF"
} >> "$GITHUB_OUTPUT"

# ── Post PR comment ──────────────────────────────────────────────
if [ "${INPUT_AUTO_COMMENT}" = "true" ] && [ -n "${PR_NUMBER:-}" ]; then
  echo "::group::Posting review comment"

  COMMENT_BODY="## 🔍 CodeTether Review

<details>
<summary>PR #${PR_NUMBER}: ${PR_TITLE}</summary>

Model: \`${CODETETHER_DEFAULT_MODEL:-default}\` · Steps: ${INPUT_MAX_STEPS}

</details>

${REVIEW_TEXT}"

  # Truncate to GitHub's 65536-char comment limit
  if [ ${#COMMENT_BODY} -gt 65000 ]; then
    COMMENT_BODY="${COMMENT_BODY:0:64900}

..._truncated (review exceeded comment size limit)_"
  fi

  curl -fsSL \
    -X POST \
    -H "Authorization: token ${GITHUB_TOKEN}" \
    -H "Accept: application/vnd.github.v3+json" \
    "https://api.github.com/repos/${REPO_FULL_NAME}/issues/${PR_NUMBER}/comments" \
    -d "$(jq -n --arg body "$COMMENT_BODY" '{body: $body}')" \
    > /dev/null

  echo "Review posted to PR #${PR_NUMBER}"
  echo "::endgroup::"
fi

# Clean up
rm -f "$DIFF_FILE" "$REVIEW_FILE"
