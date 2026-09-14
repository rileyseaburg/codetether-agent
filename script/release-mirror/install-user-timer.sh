#!/usr/bin/env bash
# Install a user-owned copy service using existing gh auth; no tokens are copied.
set -euo pipefail
root=$(cd "$(dirname "$0")" && pwd)
node=$(command -v node)
command -v gh >/dev/null
destination="$HOME/.local/lib/codetether-release-mirror"
units="$HOME/.config/systemd/user"
mkdir -p "$destination" "$units"
cp "$root"/*.mjs "$destination/"
cat > "$units/codetether-release-mirror.service" <<EOF
[Unit]
Description=Mirror Forgejo release assets to GitHub without rebuilding
[Service]
Type=oneshot
ExecStart=$node $destination/run.mjs
Environment=PATH=/usr/local/bin:/usr/bin:/bin:/snap/bin
Environment=CODETETHER_RELEASE_MIRROR_EVIDENCE=$HOME/.local/state/codetether-release-mirror
TimeoutStartSec=30min
UMask=0077
EOF
cat > "$units/codetether-release-mirror.timer" <<'EOF'
[Unit]
Description=Keep GitHub release downloads synchronized with Forgejo
[Timer]
OnStartupSec=1min
OnUnitInactiveSec=5min
Persistent=true
[Install]
WantedBy=timers.target
EOF
systemctl --user daemon-reload
systemctl --user enable --now codetether-release-mirror.timer
systemctl --user start codetether-release-mirror.service
systemctl --user show codetether-release-mirror.service -p Result -p ExecMainStatus