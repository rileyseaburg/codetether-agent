#!/usr/bin/env bash
set -euo pipefail
set +x
here=$(cd "$(dirname "$0")" && pwd)
out="${1:?Evidence directory required}"
mkdir -p "$out"
root="$HOME/.local/lib/codetether-audio"
install -d -m 700 "$root" "$HOME/.config/systemd/user"
install -d -m 700 "$HOME/.local/share/codetether-mobile/uploads"
for file in "$here"/*.mjs; do node --check "$file"; done
install -m 600 "$here"/{auth,body,handler,server,media}.mjs "$root/"
sed -e "s|@NODE@|$(command -v node)|g" -e "s|@ROOT@|$root|g" \
  "$here/codetether-audio.service" > "$HOME/.config/systemd/user/codetether-audio.service"
systemctl --user daemon-reload
systemctl --user enable --now codetether-audio.service
systemctl --user restart codetether-audio.service
for attempt in {1..20}; do
  code=$(curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:4097/tts/health || true)
  [[ "$code" == 401 ]] && break
  sleep 1
done
[[ "$code" == 401 ]] || { echo 'Audio bridge did not start securely'; exit 1; }
source "$here/../cloudflare.sh"
id=$(cat "$HOME/.config/codetether-public-server/tunnel.id")
cf GET "accounts/$CF_ACCOUNT/cfd_tunnel/$id/configurations" | jq -e '.result.config' > "$out/tunnel-before.json"
jq '{config:(.ingress = ([{hostname:"server.codetether.run",path:"^/(tts/(speak|voices|health)|mobile/(attachments|image))$",service:"http://127.0.0.1:4097"}] + [.ingress[]|select(.hostname != "server.codetether.run" or .service != "http://127.0.0.1:4097")]))}' \
  "$out/tunnel-before.json" > "$out/tunnel-update.json"
cf PUT "accounts/$CF_ACCOUNT/cfd_tunnel/$id/configurations" "$(cat "$out/tunnel-update.json")" | jq '{success,errors}' > "$out/tunnel-result.json"
jq -e '.success' "$out/tunnel-result.json" >/dev/null
systemctl --user show codetether-audio.service -p Id -p ActiveState -p SubState -p UnitFileState > "$out/service.txt"
sha256sum "$root"/*.mjs > "$out/source.sha256"
cat "$out/service.txt" "$out/tunnel-result.json"
