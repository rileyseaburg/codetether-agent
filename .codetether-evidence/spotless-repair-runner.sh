#!/usr/bin/env bash
# Keeps a CodeTether repair session alive until it commits its focused fix.
set -uo pipefail

# Recovers a session ID from a prior interrupted CodeTether log.
# Usage: recover_repair_session
# Example: recover_repair_session || echo "no prior session"
recover_repair_session() {
  local session_id session_tmp
  [[ -f "$AGENT_LOG" ]] || return 1
  session_id="$(sed -nE 's/.*Created new session: ([0-9a-f-]{36}).*/\1/p' "$AGENT_LOG" | tail -n 1)"
  [[ -n "$session_id" ]] || return 1
  session_tmp="$SESSION_FILE.tmp.$$"
  printf '%s\n' "$session_id" > "$session_tmp"
  mv -- "$session_tmp" "$SESSION_FILE"
  echo "Recovered CodeTether repair session $session_id from $AGENT_LOG."
}
# Saves the session identifier returned by a CodeTether JSON response.
# Usage: persist_repair_session '{"session_id":"session-123"}'
# Example: persist_repair_session "$CODETETHER_OUTPUT"
persist_repair_session() {
  local output="$1"
  local session_id session_tmp
  session_id="$(printf '%s\n' "$output" | jq -r '.session_id // empty' 2>/dev/null)"
  if [[ -z "$session_id" ]]; then
    if [[ -s "$SESSION_FILE" ]] || recover_repair_session; then
      echo "CodeTether returned no session ID; retaining the saved repair session."
      return 0
    fi
    echo "CodeTether returned no session ID; see $AGENT_LOG" >&2
    return 1
  fi
  session_tmp="$SESSION_FILE.tmp.$$"
  printf '%s\n' "$session_id" > "$session_tmp"
  mv -- "$session_tmp" "$SESSION_FILE"
}

# Runs one repair turn, resuming the persisted session when one exists.
# Usage: run_repair_turn
# Example: run_repair_turn || echo "repair turn failed"
run_repair_turn() {
  local output status session_id="" repair_prompt
  local -a command
  if [[ -s "$SESSION_FILE" ]]; then
    IFS= read -r session_id < "$SESSION_FILE"
    echo "Continuing CodeTether repair session $session_id."
  else
    echo "Starting a new CodeTether repair session."
  fi
  if ! write_repair_context; then
    echo "Could not write repair context attachment: $REPAIR_CONTEXT_FILE" >&2
    return 1
  fi
  repair_prompt="$(build_repair_prompt)"
  echo "Feeding ${#repair_prompt} prompt bytes to CodeTether with attachment $REPAIR_CONTEXT_FILE."
  command=(codetether run "$repair_prompt" --model "$MODEL"
    --access-mode "$ACCESS_MODE" --max-steps "$REPAIR_MAX_STEPS"
    --format json --file "$REPAIR_CONTEXT_FILE")
  if [[ -n "$session_id" ]]; then
    command+=(--session "$session_id")
  fi
  command+=(-- "$REPO_ROOT")
  output="$("${command[@]}" 2> >(tee -a "$AGENT_LOG" >&2))"
  status="$?"
  printf '%s\n' "$output" | tee -a "$AGENT_LOG"
  persist_repair_session "$output" || return 1
  if (( status != 0 )); then
    echo "CodeTether failed with exit $status; see $AGENT_LOG" >&2
    return "$status"
  fi
}

# Continues the same repair session until it advances the original branch.
# Usage: repair_until_commit
# Example: CODETETHER_REPAIR_MAX_ATTEMPTS=6 repair_until_commit
repair_until_commit() {
  local attempt current_head turn_status
  if [[ ! -s "$SESSION_FILE" ]]; then
    recover_repair_session || true
  fi
  : > "$AGENT_LOG"
  for ((attempt = 1; attempt <= REPAIR_MAX_ATTEMPTS; attempt += 1)); do
    echo "CodeTether repair turn $attempt of $REPAIR_MAX_ATTEMPTS."
    turn_status=0
    run_repair_turn || turn_status="$?"
    if [[ "$(git branch --show-current)" != "$REPAIR_BRANCH" ]]; then
      echo "CodeTether switched branches; refusing to continue." >&2
      return 1
    fi
    current_head="$(git rev-parse HEAD)"
    if [[ "$current_head" != "$REPAIR_HEAD" ]]; then
      if ! git merge-base --is-ancestor "$REPAIR_HEAD" "$current_head"; then
        echo "CodeTether replaced branch history; refusing to continue." >&2
        return 1
      fi
      echo "CodeTether created repair commit $current_head."
      return 0
    fi
    if (( turn_status != 0 )); then
      echo "Repair turn exited $turn_status without a commit; retrying the saved session." >&2
    else
      echo "No repair commit yet; continuing session from $SESSION_FILE."
    fi
  done
  echo "CodeTether created no repair commit after $REPAIR_MAX_ATTEMPTS turns." >&2
  return 1
}