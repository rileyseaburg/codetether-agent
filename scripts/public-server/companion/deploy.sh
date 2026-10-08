#!/usr/bin/env bash
# Install only the relay and public shell; never restart the Rust API or rotate its token.
set -euo pipefail
set +x
HERE="$(cd "$(dirname "$0")" && pwd)"
out="${1:?Evidence directory required}"
mkdir -p "$out"
out="$(cd "$out" && pwd)"
(cd "$HERE" && npm run typecheck && npm test) >"$out/local-tests.log" 2>&1
export VAULT_TOKEN="${VAULT_TOKEN:-$(cat "$HOME/.config/vault-agent/token")}"
node "$HERE/../../../ios/scripts/scan-secrets.mjs" "$HERE/web" >"$out/public-secret-audit.json"
root="$HOME/.local/lib/codetether-companion"
install -d -m 700 "$root" "$root/web" "$HOME/.config/systemd/user"
if [[ -f "$HOME/.config/systemd/user/codetether-companion.service" ]]; then
  cp "$HOME/.config/systemd/user/codetether-companion.service" "$out/service-before.txt"
fi
install -m 600 "$HERE"/*.ts "$HERE/package.json" "$root/"
install -m 600 "$HERE/web/"*.js "$HERE/web/"*.html "$HERE/web/"*.css "$HERE/web/"*.svg "$HERE/web/"*.webmanifest "$root/web/"
sed -e "s|@NODE@|$(command -v node)|g" -e "s|@ROOT@|$root|g" \
  "$HERE/codetether-companion.service" >"$HOME/.config/systemd/user/codetether-companion.service"
systemctl --user daemon-reload
systemctl --user enable --now codetether-companion.service
systemctl --user restart codetether-companion.service
for attempt in {1..20}; do
  if curl -fsS http://127.0.0.1:4099/companion/ >"$out/local-index.html"; then break; fi
  sleep 1
done
curl -fsS http://127.0.0.1:4099/companion/ >"$out/local-index.html"
systemctl --user show codetether-companion.service -p Id -p ActiveState -p SubState -p MainPID >"$out/service.txt"
sha256sum "$root/"*.ts "$root/web/"* >"$out/deployed.sha256"
bash "$HERE/route.sh" "$out"