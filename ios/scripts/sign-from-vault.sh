#!/usr/bin/env bash
set -euo pipefail
set +x
host="${1:-amac}"
mode="${2:-build}"
[[ "$mode" == build || "$mode" == live-test ]] || exit 2
ssh_options=()
if [[ -n "${SSH_CONFIG:-}" ]]; then ssh_options=(-F "$SSH_CONFIG"); fi
apple=$(vault kv get -format=json secret/codetether/ios-api-key)
mac=$(vault kv get -format=json secret/codetether/mac-mini)
trap 'unset apple mac' EXIT
printf '%s\n%s\n' "$apple" "$mac" |
  jq -s '{apple: .[0].data.data, password: .[1].data.data.password}' |
  ssh "${ssh_options[@]}" "$host" "ruby \"\$HOME/CodeTether-iOS/scripts/sign.rb\" '$mode'"
unset apple mac
# The API private key exists only in a private temporary directory during signing.
# mac-mini.password is used to unlock the login Keychain, never written to disk.
# App bearer credentials are provisioned separately, after installation.
