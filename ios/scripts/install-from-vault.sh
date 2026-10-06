#!/usr/bin/env bash
set -euo pipefail
set +x
host="${1:-amac}"
ssh_options=()
if [[ -n "${SSH_CONFIG:-}" ]]; then ssh_options=(-F "$SSH_CONFIG"); fi
device="${2:?Usage: install-from-vault.sh HOST DEVICE_UUID}"
[[ "$device" =~ ^[A-Fa-f0-9-]+$ ]] || exit 2
vault kv get -format=json secret/codetether/endpoints/public-server |
  jq -e '{token: .data.data.token | select(type == "string" and length > 0)}' |
  ssh "${ssh_options[@]}" "$host" "ruby \"\$HOME/CodeTether-iOS/scripts/install-device.rb\" '$device'"
