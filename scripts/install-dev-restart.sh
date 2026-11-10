#!/usr/bin/env bash
# Advisory only: never interrupt the caller or active conversations automatically.
echo "Installed binary updated; running workers keep their old executable until restarted."
echo "No services were restarted automatically. Restart when active work can be interrupted."
if command -v systemctl >/dev/null 2>&1; then
  restart_targets=()
  chat=codetether-spotlessbinco-thinker.service
  if [ "$(systemctl --user show "$chat" -p LoadState --value 2>/dev/null)" = loaded ]; then
    restart_targets+=("CodeTether Chat backend ($chat)")
  fi
  worker_service=codetether-ubuntu-dev.service
  if [ "$(systemctl show "$worker_service" -p LoadState --value 2>/dev/null)" = loaded ]; then
    restart_targets+=("System worker ($worker_service; needs sudo)")
  fi
  if [ ${#restart_targets[@]} -gt 0 ]; then
    restart_script="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/install-dev-restart-services.sh"
    echo "Hey! Installed. Now run this script to restart them on the new binary:"
    printf '  %q\n' "$restart_script"
    printf '    - %s\n' "${restart_targets[@]}"
  fi
fi
echo "For other workers, restart their service or relaunch the process using the newly installed binary."
