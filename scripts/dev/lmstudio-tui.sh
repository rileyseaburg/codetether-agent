#!/usr/bin/env bash
# Local Gemma launcher; does not change the user's normal model defaults.
set -euo pipefail

systemctl --user start codetether-lmstudio-tunnel.service
model="${CODETETHER_LMSTUDIO_MODEL:-google/gemma-4-12b-qat}"
ready=false
for attempt in {1..10}; do
  if curl -fsS --max-time 5 http://127.0.0.1:18080/api/v0/models |
    jq -e --arg model "$model" '.data[] | select(.id == $model and .state == "loaded" and .loaded_context_length > 2048)' >/dev/null; then
    ready=true
    break
  fi
  sleep 1
done
if [[ "$ready" != true ]]; then
  echo "LM Studio model $model is not loaded or the SSH tunnel is unavailable." >&2
  exit 1
fi
export CODETETHER_DEFAULT_PROVIDER=lmstudio CODETETHER_DEFAULT_MODEL="lmstudio/$model"
export CODETETHER_LMSTUDIO_MAX_OUTPUT_TOKENS="${CODETETHER_LMSTUDIO_MAX_OUTPUT_TOKENS:-4096}"
export CODETETHER_SESSION_MAX_TOKENS="${CODETETHER_SESSION_MAX_TOKENS:-$CODETETHER_LMSTUDIO_MAX_OUTPUT_TOKENS}" CODETETHER_TOOL_OUTPUT_MAX_BYTES=4096
export CODETETHER_CONTEXT_POLICY=incremental CODETETHER_TOOL_PROFILE=
exec codetether tui "$@"