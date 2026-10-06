#!/usr/bin/env bash
# Live public-edge smoke tests. Artifacts contain no credentials or API payloads.
# shellcheck source=scripts/public-server/cloudflare.sh
source "$(dirname "$0")/cloudflare.sh"
out="${1:?Usage: bash scripts/public-server/verify.sh ARTIFACT_DIRECTORY}"
mkdir -p "$out"
dir="$HOME/.config/codetether-public-server"
id="$(cat "$dir/tunnel.id")"
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel/$id" |
  jq '{success,tunnel:{id:.result.id,name:.result.name,status:.result.status,connections:[.result.connections[]?|{colo_name,is_pending_reconnect}]}}' >"$out/tunnel.json"
jq -e '.success and .tunnel.status=="healthy"' "$out/tunnel.json" >/dev/null
zone="$(cf GET 'zones?name=codetether.run' | jq -er '.result|select(length==1)|.[0].id')"
cf GET "zones/$zone/dns_records?name=server.codetether.run" |
  jq '{success,records:[.result[]|{id,type,name,content,proxied}]}' >"$out/dns.json"
# shellcheck source=/dev/null
source "$dir/origin.env"
base=https://server.codetether.run
probe() {
  local path="$1" expected="$2" mode="$3" name="$4" status
  local args=(--silent --show-error --max-time 30 -o /dev/null
    --dump-header "$out/$name.headers" -w '%{http_code}')
  case "$mode" in
    authenticated) args+=(--header @/dev/fd/3) ;;
    invalid) args+=(--header 'Authorization: Bearer invalid-smoke-test') ;;
    anonymous) ;;
  esac
  status="$(curl "${args[@]}" "$base$path" 3<<<"Authorization: Bearer $CODETETHER_AUTH_TOKEN")"
  printf '%s %s %s expected=%s observed=%s\n' "$base" "$path" "$mode" "$expected" "$status" | tee -a "$out/http.txt"
  [[ "$status" == "$expected" ]]
}
date -u +'%Y-%m-%dT%H:%M:%SZ' >"$out/checked-at.txt"
probe /health 200 anonymous health
probe /api/version 401 anonymous protected
probe /api/version 401 invalid invalid-token
probe /api/version 200 authenticated version
probe /api/agent 200 authenticated agents
for unit in codetether-public-tunnel.service codetether-spotlessbinco-thinker.service; do
  systemctl --user show "$unit" -p Id -p ActiveState -p SubState -p UnitFileState -p MainPID
done >"$out/services.txt"
pid="$(systemctl --user show codetether-spotlessbinco-thinker.service -p MainPID --value)"
sha256sum "/proc/$pid/exe" >"$out/origin-binary.sha256"
readlink "/proc/$pid/exe" >>"$out/origin-binary.sha256"
curl --silent --show-error --fail --max-time 30 --header @/dev/fd/3 "$base/api/version" \
  3<<<"Authorization: Bearer $CODETETHER_AUTH_TOKEN" >"$out/version.json"
unset CODETETHER_AUTH_TOKEN
cat "$out/tunnel.json" "$out/services.txt" "$out/version.json"
echo
echo "Live public-edge checks succeeded; evidence: $out"