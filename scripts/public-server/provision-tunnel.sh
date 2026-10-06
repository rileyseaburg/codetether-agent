#!/usr/bin/env bash
# Create one dedicated remotely-managed tunnel and a non-conflicting DNS record.
# shellcheck source=scripts/public-server/cloudflare.sh
source "$(dirname "$0")/cloudflare.sh"
umask 077
dir="$HOME/.config/codetether-public-server"
mkdir -p "$dir"
zone="$(cf GET 'zones?name=codetether.run' | jq -er '.result|select(length==1)|.[0].id')"
records="$(cf GET "zones/$zone/dns_records?name=server.codetether.run")"
tunnels="$(cf GET "accounts/$CF_ACCOUNT/cfd_tunnel?is_deleted=false&name=codetether-server-ubuntu-dev")"
id="$(jq -r '.result[]|select(.name=="codetether-server-ubuntu-dev")|.id' <<<"$tunnels")"
if [[ -z "$id" ]]; then
  [[ "$(jq '.result|length' <<<"$records")" == 0 ]] || { echo 'DNS already exists; refusing replacement'; exit 1; }
  id="$(cf POST "accounts/$CF_ACCOUNT/cfd_tunnel" \
    '{"name":"codetether-server-ubuntu-dev","config_src":"cloudflare"}' | jq -er '.result.id')"
fi
[[ "$id" != *$'\n'* ]] || { echo 'Ambiguous tunnel identity'; exit 1; }
jq -e --arg target "$id.cfargotunnel.com" \
  '.success and ((.result|length)==0 or ((.result|length)==1 and .result[0].type=="CNAME" and .result[0].content==$target and .result[0].proxied))' \
  <<<"$records" >/dev/null || { echo 'DNS conflict; refusing replacement'; exit 1; }
curl --silent --show-error --fail http://127.0.0.1:4096/health >/dev/null
cf PUT "accounts/$CF_ACCOUNT/cfd_tunnel/$id/configurations" \
  '{"config":{"ingress":[{"hostname":"server.codetether.run","service":"http://127.0.0.1:4096"},{"service":"http_status:404"}]}}' |
  jq -e '.success' >/dev/null
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel/$id/token" | jq -er '.result' >"$dir/tunnel.token"
chmod 600 "$dir/tunnel.token"
cat >"$HOME/.config/systemd/user/codetether-public-tunnel.service" <<'UNIT'
[Unit]
Description=Cloudflare Tunnel for server.codetether.run
After=network-online.target codetether-spotlessbinco-thinker.service
Wants=network-online.target codetether-spotlessbinco-thinker.service
[Service]
ExecStart=%h/bin/cloudflared tunnel --no-autoupdate run --token-file %h/.config/codetether-public-server/tunnel.token
Restart=always
RestartSec=5
NoNewPrivileges=true
UMask=0077
[Install]
WantedBy=default.target
UNIT
systemctl --user daemon-reload
systemctl --user enable --now codetether-public-tunnel.service
printf '%s\n' "$id" >"$dir/tunnel.id"
if [[ "$(jq '.result|length' <<<"$records")" == 0 ]]; then
  cf POST "zones/$zone/dns_records" "$(jq -nc --arg target "$id.cfargotunnel.com" '{type:"CNAME",name:"server.codetether.run",content:$target,proxied:true,ttl:1}')" | jq '{success,record:{id:.result.id,name:.result.name,content:.result.content,proxied:.result.proxied}}'
fi