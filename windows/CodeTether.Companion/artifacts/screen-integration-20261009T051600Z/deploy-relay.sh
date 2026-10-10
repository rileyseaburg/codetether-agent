#!/bin/bash
# Update only the existing companion relay; preserve credentials, API and routing.
set -euo pipefail
set +x
out="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$out/../../../.." && pwd)"
source="$repo/scripts/public-server/companion"
installed="$HOME/.local/lib/codetether-companion"
test -f "$installed/server.ts"
test ! -e "$out/relay-before.tar.gz"
(cd "$source" && npm run typecheck && npm test) >"$out/relay-checks.log" 2>&1
printf '0\n' >"$out/relay-checks.exit.txt"
tar -czf "$out/relay-before.tar.gz" -C "$installed" .
chmod 600 "$out/relay-before.tar.gz"
systemctl --user show codetether-companion.service -p Id -p ActiveState -p SubState -p MainPID \
  >"$out/service-before.txt"
sha256sum "$installed/"*.ts >"$out/deployed-before.sha256"
install -m 600 "$source/"*.ts "$installed/"
sha256sum "$installed/"*.ts >"$out/deployed-after.sha256"
sha256sum "$source/"*.ts >"$out/source.sha256"
systemctl --user restart codetether-companion.service
sleep 2
systemctl --user is-active codetether-companion.service
systemctl --user show codetether-companion.service -p Id -p ActiveState -p SubState -p MainPID \
  >"$out/service-after.txt"
cat "$out/service-after.txt"