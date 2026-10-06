#!/usr/bin/env bash
# Export with Vault-managed signing credentials; never print or persist secrets.
set -euo pipefail
set +x
host="${1:-amac}"
ssh_args=()
if [[ -n "${SSH_CONFIG:-}" ]]; then ssh_args=(-F "$SSH_CONFIG"); fi
vault kv get -field=password secret/codetether/mac-mini |
  ssh "${ssh_args[@]}" "$host" \
    'export PATH="/opt/homebrew/bin:$PATH"; cd "$HOME/CodeTether-iOS"; xcodegen generate >&2 && exec ruby scripts/export-adhoc.rb'