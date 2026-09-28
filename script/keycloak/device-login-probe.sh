#!/usr/bin/env bash
# Run a device-code Vault login detached, recording output (tokens redacted).
# Usage: script/keycloak/device-login-probe.sh <config-dir>
set -uo pipefail
dir=${1:?config dir}
mkdir -p "$dir"
log="$dir/login.log"
: > "$log"
(
  CODETETHER_VAULT_CONFIG_DIR="$dir" "$HOME/.cargo/bin/codetether" vault login device \
    --issuer https://auth.quantum-forge.io/realms/spotlessbinco.com \
    --client-id codetether-cli --mount oidc --role codetether-device --no-browser 2>&1 |
    grep --line-buffered -v ' INFO ' |
    sed -u -E 's/(hvs\.|s\.)[A-Za-z0-9._-]{8,}/<redacted>/g' >> "$log"
  echo "EXIT=${PIPESTATUS[0]}" >> "$log"
) < /dev/null &
disown
echo "started pid $!; log $log"
