#!/usr/bin/env bash
# Probe the Vault codetether-device role with a real Keycloak-signed JWT from
# the automation client, printing only Vault's error text (never tokens).
set -euo pipefail
creds=$(vault kv get -format=json kv/keycloak/spotlessbinco-agent-admin)
id=$(printf '%s' "$creds" | jq -er .data.data.client_id)
secret=$(printf '%s' "$creds" | jq -er .data.data.client_secret)
unset creds
jwt=$(curl -fsS -X POST https://auth.quantum-forge.io/realms/spotlessbinco.com/protocol/openid-connect/token \
  --data-urlencode grant_type=client_credentials --data-urlencode "client_id=$id" \
  --data-urlencode "client_secret=$secret" | jq -er .access_token)
unset secret
echo "claims: $(printf '%s' "$jwt" | cut -d. -f2 | tr '_-' '/+' | base64 -d 2>/dev/null | jq -c '{iss, aud, azp, groups, preferred_username}')"
jq -n --arg jwt "$jwt" '{role:"codetether-device", jwt:$jwt}' |
  curl -sS -X POST --data-binary @- "$VAULT_ADDR/v1/auth/${VAULT_JWT_MOUNT:-jwt}/login" | jq -c '.errors // {ok: (.auth != null)}'
