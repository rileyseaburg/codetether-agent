#!/usr/bin/env bash
set -euo pipefail
set +x
out="${1:?Evidence directory required}"
mkdir -p "$out"
token=$(vault kv get -field=token secret/codetether/endpoints/public-server)
trap 'unset token' EXIT
curl -fsS --max-time 30 -H @/dev/fd/3 https://server.codetether.run/v1/models \
  3<<<"Authorization: Bearer $token" > "$out/models.json"
curl -fsS --max-time 30 -H @/dev/fd/3 https://server.codetether.run/api/config \
  3<<<"Authorization: Bearer $token" | jq '{default_model}' > "$out/default-model.json"
if [[ -n "${2:-}" ]]; then
  jq -nc --arg model "$2" '{model:$model,messages:[{role:"user",content:"Reply exactly: CodeTether chat is ready."}],stream:false,max_tokens:128}' |
    curl --fail-with-body -sS --max-time 180 -H @/dev/fd/3 -H 'Content-Type: application/json' \
      --data-binary @- https://server.codetether.run/v1/chat/completions \
      3<<<"Authorization: Bearer $token" > "$out/chat-smoke.json"
fi
echo 'Authenticated chat contract retrieved.'
