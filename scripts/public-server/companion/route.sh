#!/usr/bin/env bash
# Add the narrow companion prefix, preserving every existing API and installer route.
set -euo pipefail
set +x
HERE="$(cd "$(dirname "$0")" && pwd)"
source "$HERE/../cloudflare.sh"
out="${1:?Evidence directory required}"
id="$(cat "$HOME/.config/codetether-public-server/tunnel.id")"
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel/$id" | jq '{success,id:.result.id,name:.result.name,status:.result.status}' >"$out/tunnel.json"
jq -e '.success and .name=="codetether-server-ubuntu-dev" and .status=="healthy"' "$out/tunnel.json" >/dev/null
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel/$id/configurations" >"$out/ingress-before.json"
pattern='^/companion(?:/.*)?$'
jq -e --arg path "$pattern" '[.result.config.ingress[] | select(.hostname=="server.codetether.run" and .path==$path and .service!="http://127.0.0.1:4099")] | length==0' "$out/ingress-before.json" >/dev/null
jq --arg path "$pattern" '{config:.result.config} | .config.ingress=([{hostname:"server.codetether.run",path:$path,service:"http://127.0.0.1:4099"}]+[.config.ingress[] | select(.hostname!="server.codetether.run" or .path!=$path)])' "$out/ingress-before.json" >"$out/ingress-request.json"
cf PUT "accounts/$CF_ACCOUNT/cfd_tunnel/$id/configurations" "$(cat "$out/ingress-request.json")" >"$out/ingress-update.json"
jq -e '.success' "$out/ingress-update.json" >/dev/null
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel/$id/configurations" >"$out/ingress-after.json"
diff <(jq -S --arg path "$pattern" '[.result.config.ingress[] | select(.hostname!="server.codetether.run" or .path!=$path)]' "$out/ingress-before.json") \
     <(jq -S --arg path "$pattern" '[.result.config.ingress[] | select(.hostname!="server.codetether.run" or .path!=$path)]' "$out/ingress-after.json")
printf 'Companion route applied; all other ingress entries unchanged. Evidence: %s\n' "$out"