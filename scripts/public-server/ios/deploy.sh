#!/usr/bin/env bash
# Publish only audited installation assets; keep authenticated API routes intact.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
ASSETS="${1:?Asset directory required}"
EVIDENCE="${2:?Evidence directory required}"
VERSION="${3:?Version required}"
BUILD="${4:?Build required}"
SHA="${5:?Trusted IPA checksum required}"
RELEASE="$(node --input-type=module -e \
  'const {releaseConfig}=await import(process.argv[1]);const c=releaseConfig(...process.argv.slice(2));console.log(`${c.version}-${c.build}`)' \
  "$ROOT/ios/scripts/release-config.mjs" "$VERSION" "$BUILD" "$SHA")"
mkdir -p "$EVIDENCE"
export VAULT_TOKEN="${VAULT_TOKEN:-$(cat "$HOME/.config/vault-agent/token")}"
node "$ROOT/ios/scripts/scan-secrets.mjs" "$ASSETS" > "$EVIDENCE/secret-audit.txt"
(cd "$ASSETS" && sha256sum -c SHA256SUMS) > "$EVIDENCE/checksums.txt"
DEST="$HOME/.local/share/codetether-ios/releases/$RELEASE"
mkdir -p "$DEST" "$HOME/.config/systemd/user"
for file in CodeTether.ipa manifest.plist index.html release.json SHA256SUMS; do
  if [[ -f "$DEST/$file" ]] && ! cmp -s "$ASSETS/$file" "$DEST/$file"; then
    echo "Refusing to replace differing published asset: $file" >&2; exit 1
  fi
  install -m 644 "$ASSETS/$file" "$DEST/$file"
done
sed -e "s|@NODE@|$(command -v node)|g" -e "s|@ROOT@|$HERE|g" \
  -e "s|@RELEASE@|$RELEASE|g" \
  "$HERE/codetether-ios-download.service" > "$HOME/.config/systemd/user/codetether-ios-download.service"
systemctl --user daemon-reload
systemctl --user restart codetether-ios-download.service
systemctl --user enable --now codetether-ios-download.service
for attempt in {1..20}; do
  if curl -fsS http://127.0.0.1:4098/ > "$EVIDENCE/local-index.html"; then break; fi
  sleep 1
done
curl -fsS http://127.0.0.1:4098/ > "$EVIDENCE/local-index.html"
systemctl --user show codetether-ios-download.service -p ActiveState -p SubState -p MemoryCurrent > "$EVIDENCE/download-service.txt"
sha256sum "$HERE"/*.mjs "$DEST"/* > "$EVIDENCE/deployed.sha256"
bash "$HERE/route.sh" "$EVIDENCE"
printf 'Download service and installer ingress evidence: %s\n' "$EVIDENCE"