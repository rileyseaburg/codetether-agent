#!/usr/bin/env bash
# Shared authenticated Cloudflare client. Never print credentials or enable xtrace.
set -euo pipefail
VAULT_TOKEN="$(cat "$HOME/.config/vault-agent/token")"
export VAULT_TOKEN
CF_SECRET="$(vault kv get -format=json kv/cloudflare/api-token)"
CF_TOKEN="$(jq -er '.data.data.token' <<<"$CF_SECRET")"
CF_ACCOUNT="$(jq -er '.data.data.account_id' <<<"$CF_SECRET")"
export CF_ACCOUNT
unset CF_SECRET
cf() {
  local method="$1" path="$2" body="${3:-}"
  local args=(--silent --show-error --fail-with-body --max-time 30
    --request "$method" --header @/dev/fd/3)
  if [[ -n "$body" ]]; then args+=(--data-binary "$body"); fi
  curl "${args[@]}" "https://api.cloudflare.com/client/v4/$path" \
    3<<<"$(printf 'Authorization: Bearer %s\nContent-Type: application/json' "$CF_TOKEN")"
}