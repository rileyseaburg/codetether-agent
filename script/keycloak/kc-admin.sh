#!/usr/bin/env bash
# Keycloak admin REST helper for the spotlessbinco.com realm.
# Credentials come from Vault (kv/keycloak/spotlessbinco-agent-admin) and are
# never printed. Usage: script/keycloak/kc-admin.sh <METHOD> <path> [json-body-file]
set -euo pipefail
base=https://auth.quantum-forge.io
realm=spotlessbinco.com
creds=$(vault kv get -format=json kv/keycloak/spotlessbinco-agent-admin)
id=$(printf '%s' "$creds" | jq -er .data.data.client_id)
secret=$(printf '%s' "$creds" | jq -er .data.data.client_secret)
unset creds
token=$(curl -fsS -X POST "$base/realms/$realm/protocol/openid-connect/token" \
  --data-urlencode grant_type=client_credentials --data-urlencode "client_id=$id" \
  --data-urlencode "client_secret=$secret" | jq -er .access_token)
unset secret
method=$1 path=$2 body=${3:-}
args=(-sS -w '\nHTTP %{http_code}\n' -X "$method" -H "Authorization: Bearer $token")
[ -z "$body" ] || args+=(-H 'Content-Type: application/json' --data-binary "@$body")
curl "${args[@]}" "$base/admin/realms/$realm$path"
