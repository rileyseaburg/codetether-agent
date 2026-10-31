#!/usr/bin/env bash
# Deploy diagnostics only; preserve tunnel routes and the running agent backend.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
out="${1:?Evidence directory required}"
nas=/mnt/nas/CodeTether/logs/voice
test "$(findmnt -n -t cifs -o SOURCE --mountpoint /mnt/nas)" = '//192.168.50.133/NAS'
root="$HOME/.local/lib/codetether-voice-logs"
audio="$HOME/.local/lib/codetether-audio"
mkdir -p "$out/audio-before"
cp "$audio"/*.mjs "$out/audio-before/"
install -d -m 700 "$root" "$HOME/.local/state/codetether-voice-logs" "$nas"
for file in archive lines sanitize server; do
  node --check "$here/$file.mjs"
  install -m 600 "$here/$file.mjs" "$root/"
done
for file in auth body handler server media trace; do
  node --check "$here/../audio/$file.mjs"
  install -m 600 "$here/../audio/$file.mjs" "$audio/"
done
sed -e "s|@NODE@|$(command -v node)|g" -e "s|@ROOT@|$root|g" \
  "$here/codetether-voice-logs.service" > "$HOME/.config/systemd/user/codetether-voice-logs.service"
systemctl --user daemon-reload
systemctl --user enable --now codetether-voice-logs.service
systemctl --user restart codetether-audio.service
systemctl --user show codetether-voice-logs.service codetether-audio.service \
  -p Id -p ActiveState -p SubState -p MainPID > "$out/services.txt"
sha256sum "$root"/*.mjs "$audio"/*.mjs > "$out/deployed.sha256"