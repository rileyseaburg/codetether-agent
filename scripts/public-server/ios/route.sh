#!/usr/bin/env bash
# Add isolated download ingress while preserving all authenticated API routes.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
source "$HERE/../cloudflare.sh"
EVIDENCE="${1:?Evidence directory required}"
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel?is_deleted=false" > "$EVIDENCE/tunnels.json"
TUNNEL="$(jq -er '[.result[] | select(.name=="codetether-server-ubuntu-dev" and .status=="healthy")] | if length==1 then .[0].id else error("Expected one healthy public tunnel") end' "$EVIDENCE/tunnels.json")"
cf GET 'zones?name=codetether.run' > "$EVIDENCE/zone.json"
ZONE="$(jq -er '[.result[] | select(.status=="active")] | if length==1 then .[0].id else error("Expected one active zone") end' "$EVIDENCE/zone.json")"
cf GET "zones/$ZONE/dns_records?name=ios.codetether.run" > "$EVIDENCE/dns-before.json"
TARGET="$TUNNEL.cfargotunnel.com"
jq -e --arg target "$TARGET" '(.result | length)==0 or ((.result | length)==1 and .result[0].type=="CNAME" and .result[0].content==$target and .result[0].proxied==true)' "$EVIDENCE/dns-before.json" >/dev/null || { echo 'Conflicting installer DNS; refusing overwrite' >&2; exit 1; }
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel/$TUNNEL/configurations" > "$EVIDENCE/tunnel-before.json"
jq --arg host 'ios.codetether.run' '{config:.result.config} | .config.ingress=([{hostname:$host,service:"http://127.0.0.1:4098"}]+[.config.ingress[] | select(.hostname!=$host)])' "$EVIDENCE/tunnel-before.json" > "$EVIDENCE/tunnel-request.json"
cf PUT "accounts/$CF_ACCOUNT/cfd_tunnel/$TUNNEL/configurations" "$(cat "$EVIDENCE/tunnel-request.json")" > "$EVIDENCE/tunnel-update.json"
jq -e '.success==true' "$EVIDENCE/tunnel-update.json" >/dev/null
if [[ "$(jq '.result | length' "$EVIDENCE/dns-before.json")" == 0 ]]; then
  BODY="$(jq -nc --arg target "$TARGET" '{type:"CNAME",name:"ios.codetether.run",content:$target,proxied:true,ttl:1}')"
  cf POST "zones/$ZONE/dns_records" "$BODY" > "$EVIDENCE/dns-created.json"
  jq -e '.success==true' "$EVIDENCE/dns-created.json" >/dev/null
fi
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel/$TUNNEL/configurations" > "$EVIDENCE/tunnel-result.json"
cf GET "zones/$ZONE/dns_records?name=ios.codetether.run" > "$EVIDENCE/dns-result.json"
jq -e '.success==true' "$EVIDENCE/dns-result.json" >/dev/null
diff <(jq -S '[.result.config.ingress[] | select(.hostname!="ios.codetether.run")]' "$EVIDENCE/tunnel-before.json") <(jq -S '[.result.config.ingress[] | select(.hostname!="ios.codetether.run")]' "$EVIDENCE/tunnel-result.json")
systemctl --user show codetether-public-tunnel.service -p ActiveState -p SubState > "$EVIDENCE/tunnel-service.txt"
printf 'Installer DNS and tunnel routing recorded in %s\n' "$EVIDENCE"
