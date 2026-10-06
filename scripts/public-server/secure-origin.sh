#!/usr/bin/env bash
# Persist and rotate the origin credential without logging it or passing it in argv.
set -euo pipefail
umask 077
VAULT_TOKEN="$(cat "$HOME/.config/vault-agent/token")"
export VAULT_TOKEN
dir="$HOME/.config/codetether-public-server"
unit=codetether-spotlessbinco-thinker.service
mkdir -p "$dir" "$HOME/.config/systemd/user/$unit.d"
chmod 700 "$dir"
if [[ ! -s "$dir/origin.env" ]]; then
  token="$(openssl rand -hex 32)"
  printf '%s' "$token" | jq -Rs '{token:.,url:"https://server.codetether.run"}' |
    vault kv put secret/codetether/endpoints/public-server - >/dev/null
  printf 'CODETETHER_AUTH_TOKEN=%s\n' "$token" >"$dir/origin.env"
  unset token
fi
printf '[Service]\nEnvironmentFile=%%h/.config/codetether-public-server/origin.env\n' \
  >"$HOME/.config/systemd/user/$unit.d/public-auth.conf"
systemctl --user daemon-reload
systemctl --user restart "$unit"
for ((attempt=0; attempt<30; attempt++)); do
  if curl --silent --fail http://127.0.0.1:4096/health >/dev/null; then break; fi
  sleep 2
done
# shellcheck source=/dev/null
source "$dir/origin.env"
status="$(curl --silent --show-error --max-time 10 -o /dev/null -w '%{http_code}' \
  --header @/dev/fd/3 http://127.0.0.1:4096/api/version \
  3<<<"Authorization: Bearer $CODETETHER_AUTH_TOKEN")"
[[ "$status" == 200 ]] || { echo "Origin auth check failed: HTTP $status"; exit 1; }
echo 'Origin authenticated version: HTTP 200; credential persisted in Vault.'
unset CODETETHER_AUTH_TOKEN