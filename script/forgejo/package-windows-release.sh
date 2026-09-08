#!/usr/bin/env bash
# Package the CI-built GNU executable with the setup ZIP and MSI installer.
set -euo pipefail
: "${RELEASE_VERSION:?RELEASE_VERSION is required}"
if ! command -v python3 >/dev/null 2>&1 || ! command -v wixl >/dev/null 2>&1; then
  elevate=()
  if [ "$(id -u)" -ne 0 ]; then elevate=(sudo); fi
  "${elevate[@]}" sh script/forgejo/apt-https.sh
  "${elevate[@]}" apt-get update
  "${elevate[@]}" apt-get install -y --no-install-recommends python3 wixl msitools
fi
asset="codetether-v$RELEASE_VERSION-x86_64-pc-windows-gnu"
mkdir -p dist
test -s build/windows/codetether.exe
cp build/windows/codetether.exe "dist/$asset.exe"
python3 script/package-windows-bundle.py --binary "dist/$asset.exe" \
  --archive "dist/$asset.zip" --staging build/windows-bundle
cp "dist/$asset.exe" dist/codetether.exe
python3 script/build-windows-msi.py
cp dist/codetether-windows.msi "dist/$asset.msi"