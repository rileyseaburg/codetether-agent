#!/usr/bin/env bash
set -euo pipefail

cargo_cmd=${1:-cargo}
user_binary=${CODETETHER_USER_BIN:-"$HOME/.cargo/bin/codetether"}
system_binary=${CODETETHER_SYSTEM_BIN:-/usr/local/bin/codetether}

"$cargo_cmd" install --path . --force

if [ ! -x "$user_binary" ]; then
  echo "Error: release binary was not installed at $user_binary" >&2
  exit 1
fi

if [ "$user_binary" != "$system_binary" ]; then
  install -m755 "$user_binary" "$system_binary"
fi

echo "==> Installed release binary:"
echo "    $user_binary -> $("$user_binary" --version)"
if [ "$user_binary" != "$system_binary" ]; then
  echo "    $system_binary -> $("$system_binary" --version)"
fi