#!/usr/bin/env bash
# Read-only preflight; emits metadata only, never account or tunnel credentials.
# shellcheck source=scripts/public-server/cloudflare.sh
source "$(dirname "$0")/cloudflare.sh"
cf GET 'zones?name=codetether.run' | jq '{success,errors,zones:[.result[]|{id,name,status}]}'
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel?is_deleted=false" |
  jq '{success,errors,tunnels:[.result[]|{id,name,status,config_src}]}'