#!/usr/bin/env bash
set -euo pipefail

: "${MAC_KEYCHAIN_PASS:?MAC_KEYCHAIN_PASS is required}"
SIGNING_IDENTITY="${MAC_SIGNING_IDENTITY:-Developer ID Application: RC Seaburg Ventures LLC (J9YRM3U37D)}"
KEYCHAIN="${MAC_KEYCHAIN:-$HOME/Library/Keychains/login.keychain-db}"

# shellcheck source=/dev/null
source "$HOME/.cargo/env" 2>/dev/null || true
if [ -x /opt/homebrew/bin/brew ]; then
  export PATH="/opt/homebrew/bin:/opt/homebrew/sbin:$PATH"
fi

if ! command -v protoc >/dev/null 2>&1; then
  bootstrap="$HOME/.local/protoc"
  mkdir -p "$bootstrap"
  curl -fsSL https://github.com/protocolbuffers/protobuf/releases/download/v31.1/protoc-31.1-osx-universal_binary.zip -o /tmp/protoc.zip
  unzip -oq /tmp/protoc.zip -d "$bootstrap"
  export PATH="$bootstrap/bin:$PATH"
fi
PROTOC="$(command -v protoc)"
export PROTOC

available_kb="$(df -Pk /tmp | awk 'NR == 2 { print $4 }')"
minimum_kb="$(( ${MAC_MIN_FREE_GB:-10} * 1024 * 1024 ))"
if [ "$available_kb" -lt "$minimum_kb" ]; then
  echo "Error: Mac release host has less than ${MAC_MIN_FREE_GB:-10} GiB free." >&2
  exit 1
fi

security unlock-keychain -p "$MAC_KEYCHAIN_PASS" "$KEYCHAIN"
security set-keychain-settings -lut 21600 "$KEYCHAIN"
security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$MAC_KEYCHAIN_PASS" "$KEYCHAIN" >/dev/null
security find-identity -v -p codesigning "$KEYCHAIN" | grep -Fq "$SIGNING_IDENTITY"

if [ "${MAC_RELEASE_PREFLIGHT_ONLY:-0}" = "1" ]; then
  echo "macOS release preflight passed: protoc, disk, and signing identity are ready."
  exit 0
fi

rustup target add aarch64-apple-darwin x86_64-apple-darwin
./script/cargo-sccache.sh build --release --target aarch64-apple-darwin
./script/cargo-sccache.sh build --release --target x86_64-apple-darwin

for target in aarch64-apple-darwin x86_64-apple-darwin; do
  binary="target/$target/release/codetether"
  codesign --force --options runtime --timestamp --sign "$SIGNING_IDENTITY" "$binary"
  codesign --verify --strict --verbose=2 "$binary"
done
