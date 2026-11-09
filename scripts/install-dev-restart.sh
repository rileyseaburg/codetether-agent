#!/usr/bin/env bash
# Advisory only: never interrupt the caller or active conversations automatically.
echo "Installed binary updated; running workers keep their old executable until restarted."
echo "No services were restarted automatically. Restart when active work can be interrupted."
if command -v systemctl >/dev/null 2>&1; then
  chat=codetether-spotlessbinco-thinker.service
  if [ "$(systemctl --user show "$chat" -p LoadState --value 2>/dev/null)" = loaded ]; then
    echo "CodeTether Chat backend (check ExecStart for a pinned older binary first):"
    echo "  systemctl --user show $chat -p ExecStart"
    echo "  systemctl --user restart $chat"
    echo "  systemctl --user status $chat --no-pager"
  fi
  worker_service=codetether-ubuntu-dev.service
  if [ "$(systemctl show "$worker_service" -p LoadState --value 2>/dev/null)" = loaded ]; then
    echo "System worker: update its installed copy before restarting:"
    printf '  sudo install -m755 %q /opt/codetether-worker/bin/codetether\n' "$HOME/.cargo/bin/codetether"
    echo "  sudo systemctl restart $worker_service"
    echo "  sudo systemctl status $worker_service --no-pager"
  fi
fi
echo "For other workers, restart their service or relaunch the process using the newly installed binary."
