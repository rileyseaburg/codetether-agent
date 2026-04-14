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

WORKSPACE_PATH="${INPUT_WORKSPACE_PATH:-${GITHUB_WORKSPACE:-$PWD}}"
TASK_WAIT_SECONDS="${INPUT_TASK_WAIT_SECONDS:-1200}"
mkdir -p "${WORKSPACE_PATH}"

post_github_comment() {
  local target_number="$1"
  local body="$2"
  curl -fsSL \
    -X POST \
    -H "Authorization: token ${GITHUB_TOKEN}" \
    -H "Accept: application/vnd.github.v3+json" \
    "https://api.github.com/repos/${REPO_FULL_NAME}/issues/${target_number}/comments" \
    -d "$(jq -n --arg body "$body" '{body: $body}')" \
    > /dev/null
}

write_review_output() {
  local text="$1"
  local code="${2:-0}"
  echo "exit_code=${code}" >> "$GITHUB_OUTPUT"
  {
    echo "review<<CODETETHER_EOF"
    echo "$text"
    echo "CODETETHER_EOF"
  } >> "$GITHUB_OUTPUT"
}

github_api_get() {
  local path="$1"
  curl -fsSL \
    -H "Authorization: token ${GITHUB_TOKEN}" \
    -H "Accept: application/vnd.github.v3+json" \
    "https://api.github.com${path}"
}

fetch_pr_metadata() {
  local response
  response="$(github_api_get "/repos/${REPO_FULL_NAME}/pulls/${PR_NUMBER}")"
  PR_BASE="$(echo "$response" | jq -r '.base.ref')"
  PR_HEAD="$(echo "$response" | jq -r '.head.ref')"
  PR_HEAD_REPO="$(echo "$response" | jq -r '.head.repo.full_name')"
}

run_local_codetether() {
  local prompt="$1"
  local output_file="$2"
  local run_args=()
  if codetether run --help 2>&1 | grep -q -- '--max-steps'; then
    run_args+=(--max-steps "${INPUT_MAX_STEPS}")
  fi
  codetether run "${run_args[@]}" "$prompt" 2>&1 | tee "$output_file"
  return "${PIPESTATUS[0]:-0}"
}

COMMENT_BODY=""
COMMENT_PATH=""
COMMENT_DIFF_HUNK=""
IS_PR_COMMENT="false"
FIX_REQUEST="false"
if [ -n "${GITHUB_EVENT_PATH:-}" ] && [ -f "${GITHUB_EVENT_PATH}" ]; then
  COMMENT_BODY="$(jq -r '.comment.body // ""' "${GITHUB_EVENT_PATH}")"
  COMMENT_PATH="$(jq -r '.comment.path // ""' "${GITHUB_EVENT_PATH}")"
  COMMENT_DIFF_HUNK="$(jq -r '.comment.diff_hunk // ""' "${GITHUB_EVENT_PATH}")"
  if [ "${GITHUB_EVENT_NAME:-}" = "pull_request_review_comment" ]; then
    IS_PR_COMMENT="true"
  elif [ "${GITHUB_EVENT_NAME:-}" = "issue_comment" ] && jq -e '.issue.pull_request' "${GITHUB_EVENT_PATH}" >/dev/null 2>&1; then
    IS_PR_COMMENT="true"
  fi
fi

COMMENT_BODY_LOWER="$(printf '%s' "${COMMENT_BODY}" | tr '\r\n' '  ' | tr '[:upper:]' '[:lower:]')"
if printf '%s' "${COMMENT_BODY_LOWER}" | grep -Eq '(@codetether([^[:alnum:]_-]|$).*(fix|apply|address|implement|patch))|((fix|apply|address|implement|patch).+@codetether([^[:alnum:]_-]|$))'; then
  FIX_REQUEST="true"
fi

cd "${WORKSPACE_PATH}"

# ── Issue mode: use issue body directly as prompt ────────────────
if [ "${GITHUB_EVENT_NAME:-}" = "issues" ] || { [ "${GITHUB_EVENT_NAME:-}" = "issue_comment" ] && [ "${IS_PR_COMMENT}" != "true" ]; }; then
  echo "::group::Processing issue #${PR_NUMBER}"

  COMMENT_INSTRUCTIONS=""
  if [ "${GITHUB_EVENT_NAME:-}" = "issue_comment" ] && [ -n "${COMMENT_BODY}" ]; then
    COMMENT_INSTRUCTIONS="A new issue comment mentioned @codetether:
${COMMENT_BODY}

Respond directly to that comment while considering the full issue context.
"
  fi

  PROMPT="You are responding to GitHub Issue #${PR_NUMBER}: \"${PR_TITLE}\" in ${REPO_FULL_NAME}.

Analyze the issue and provide a thorough response. If it's a bug report, suggest a fix. If it's a feature request, discuss implementation approach.

${INPUT_EXTRA_PROMPT:+Additional instructions: ${INPUT_EXTRA_PROMPT}}
${COMMENT_INSTRUCTIONS}

Issue body:
${PR_BODY:-No description provided.}"

  REVIEW_TEXT=""

  # ── Dispatch to server (issue mode) ─────────────────────────
  if [ "$INPUT_MODE" = "server" ]; then
    echo "Dispatching issue task to A2A server..."

    # Truncate description to fit server's max_length validation
    MAX_DESC_CHARS=99000
    TASK_PAYLOAD=$(jq -n \
      --arg description "${PROMPT:0:$MAX_DESC_CHARS}" \
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

    DESC_LEN=${#PROMPT}
    if [ "$DESC_LEN" -gt "$MAX_DESC_CHARS" ]; then
      echo "⚠ Description truncated from ${DESC_LEN} to ${MAX_DESC_CHARS} chars for server limit"
    fi

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
    POLL_INTERVAL=5
    MAX_POLL=$(( (TASK_WAIT_SECONDS + POLL_INTERVAL - 1) / POLL_INTERVAL ))
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
      REVIEW_TEXT="Issue task timed out after ${TASK_WAIT_SECONDS}s (status: ${TASK_STATUS}). Task ID: ${TASK_ID}"
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

    post_github_comment "${PR_NUMBER}" "${COMMENT_BODY}"

    echo "Response posted to Issue #${PR_NUMBER}"
  fi

  write_review_output "$REVIEW_TEXT"

  echo "::endgroup::"
  exit 0
fi

if [ "${IS_PR_COMMENT}" = "true" ]; then
  fetch_pr_metadata

  if [ -n "${COMMENT_BODY}" ] && [ "${FIX_REQUEST}" != "true" ]; then
    INPUT_EXTRA_PROMPT="$(printf '%s\n\nRespond to this PR comment while reviewing the current diff:\n%s' "${INPUT_EXTRA_PROMPT:-}" "${COMMENT_BODY}")"
    if [ -n "${COMMENT_PATH}" ]; then
      INPUT_EXTRA_PROMPT="$(printf '%s\n\nThe comment targets file: %s' "${INPUT_EXTRA_PROMPT}" "${COMMENT_PATH}")"
    fi
    if [ -n "${COMMENT_DIFF_HUNK}" ]; then
      INPUT_EXTRA_PROMPT="$(printf '%s\n\nRelevant diff hunk:\n%s' "${INPUT_EXTRA_PROMPT}" "${COMMENT_DIFF_HUNK}")"
    fi
  fi

  if [ "${FIX_REQUEST}" = "true" ]; then
    echo "::group::Applying requested PR fix"

    if [ "${INPUT_MODE}" != "local" ]; then
      REVIEW_TEXT="Auto-fix requests on pull request comments require local mode with repository write access."
      [ "${INPUT_AUTO_COMMENT}" = "true" ] && post_github_comment "${PR_NUMBER}" "## 🛠️ CodeTether Fix\n\n${REVIEW_TEXT}"
      write_review_output "${REVIEW_TEXT}"
      echo "::endgroup::"
      exit 0
    fi

    if [ "${PR_HEAD_REPO}" != "${REPO_FULL_NAME}" ]; then
      REVIEW_TEXT="Auto-fix is not available for forked pull requests because this workflow cannot safely push to \`${PR_HEAD_REPO}:${PR_HEAD}\`."
      [ "${INPUT_AUTO_COMMENT}" = "true" ] && post_github_comment "${PR_NUMBER}" "## 🛠️ CodeTether Fix\n\n${REVIEW_TEXT}"
      write_review_output "${REVIEW_TEXT}"
      echo "::endgroup::"
      exit 0
    fi

    git fetch origin "${PR_HEAD}" --depth=1 || true
    git checkout -B "${PR_HEAD}" "origin/${PR_HEAD}" 2>/dev/null || git checkout -B "${PR_HEAD}"

    DIFF_FILE="$(mktemp)"
    git diff "origin/${PR_BASE}...HEAD" -- '*.rs' '*.py' '*.ts' '*.js' '*.go' '*.java' '*.tsx' '*.jsx' '*.yml' '*.yaml' '*.toml' > "$DIFF_FILE" 2>/dev/null || true
    FIX_DIFF="$(head -n 3000 "$DIFF_FILE")"
    FIX_FILE="$(mktemp)"

    FIX_PROMPT="You are editing the checked-out PR branch for PR #${PR_NUMBER}: \"${PR_TITLE}\" (${PR_HEAD} → ${PR_BASE}).

Apply the requested changes directly in the working tree. Do not just describe the fix.

Triggering comment:
${COMMENT_BODY}

${COMMENT_PATH:+Commented file: ${COMMENT_PATH}}
${COMMENT_DIFF_HUNK:+
Relevant diff hunk:
${COMMENT_DIFF_HUNK}}

${INPUT_EXTRA_PROMPT:+Additional instructions: ${INPUT_EXTRA_PROMPT}}

Current diff:
\`\`\`diff
${FIX_DIFF}
\`\`\`

After editing files, run the smallest relevant validation needed to support the change. Do not commit or push; the workflow will handle git."

    if ! run_local_codetether "${FIX_PROMPT}" "${FIX_FILE}"; then
      REVIEW_TEXT="I couldn't apply the requested changes automatically. Review the workflow logs for details."
      [ "${INPUT_AUTO_COMMENT}" = "true" ] && post_github_comment "${PR_NUMBER}" "## 🛠️ CodeTether Fix\n\n${REVIEW_TEXT}"
      write_review_output "${REVIEW_TEXT}" "1"
      echo "::error::codetether failed to apply the requested PR changes"
      echo "::endgroup::"
      exit 1
    fi

    if [ -z "$(git status --short)" ]; then
      REVIEW_TEXT="I reviewed the request but did not find any file changes to apply."
      [ "${INPUT_AUTO_COMMENT}" = "true" ] && post_github_comment "${PR_NUMBER}" "## 🛠️ CodeTether Fix\n\n${REVIEW_TEXT}"
      write_review_output "${REVIEW_TEXT}"
      rm -f "${DIFF_FILE}" "${FIX_FILE}"
      echo "::endgroup::"
      exit 0
    fi

    git remote set-url origin "https://x-access-token:${GITHUB_TOKEN}@github.com/${REPO_FULL_NAME}.git"
    git add -A
    # Scope the bot identity to this workflow commit so local checkouts do not
    # retain a repo-level bot author configuration.
    GIT_AUTHOR_NAME="codetether[bot]" \
    GIT_AUTHOR_EMAIL="codetether[bot]@users.noreply.github.com" \
    GIT_COMMITTER_NAME="codetether[bot]" \
    GIT_COMMITTER_EMAIL="codetether[bot]@users.noreply.github.com" \
      git commit -m "fix: address @codetether request on PR #${PR_NUMBER}"
    git push origin "HEAD:${PR_HEAD}"

    COMMIT_SHA="$(git rev-parse --short HEAD)"
    REVIEW_TEXT="Applied the requested changes in commit \`${COMMIT_SHA}\` and pushed them to \`${PR_HEAD}\`."
    [ "${INPUT_AUTO_COMMENT}" = "true" ] && post_github_comment "${PR_NUMBER}" "## 🛠️ CodeTether Fix\n\n${REVIEW_TEXT}"
    write_review_output "${REVIEW_TEXT}"
    rm -f "${DIFF_FILE}" "${FIX_FILE}"
    echo "::endgroup::"
    exit 0
  fi
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

  # Truncate description to fit server's max_length validation
  MAX_DESC_CHARS=99000
  TASK_PAYLOAD=$(jq -n \
    --arg description "${PROMPT:0:$MAX_DESC_CHARS}" \
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

  DESC_LEN=${#PROMPT}
  if [ "$DESC_LEN" -gt "$MAX_DESC_CHARS" ]; then
    echo "⚠ Description truncated from ${DESC_LEN} to ${MAX_DESC_CHARS} chars for server limit"
  fi

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
  POLL_INTERVAL=5
  MAX_POLL=$(( (TASK_WAIT_SECONDS + POLL_INTERVAL - 1) / POLL_INTERVAL ))
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
    REVIEW_TEXT="Review task timed out after ${TASK_WAIT_SECONDS}s (status: ${TASK_STATUS}). Task ID: ${TASK_ID}"
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
