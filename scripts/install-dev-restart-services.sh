#!/usr/bin/env bash
# Restart the services that keep running the old codetether binary after
# ./scripts/install-dev.sh. This interrupts active work on those services, so
# install-dev.sh never calls it automatically: run it when you are ready.
#
# The system worker goes first and the chat backend last, because restarting
# the chat backend can end the session that launched this script.
set -uo pipefail

installed="$HOME/.cargo/bin/codetether"
chat=codetether-spotlessbinco-thinker.service
worker_service=codetether-ubuntu-dev.service
worker_bin=/opt/codetether-worker/bin/codetether
failures=0

fail() {
  echo "FAILED: $*" >&2
  failures=$((failures + 1))
}

loaded() { [ "$(systemctl "$@" -p LoadState --value 2>/dev/null)" = loaded ]; }

if [ ! -x "$installed" ]; then
  echo "No installed binary at $installed; run ./scripts/install-dev.sh first." >&2
  exit 1
fi
if ! command -v systemctl >/dev/null 2>&1; then
  echo "systemctl unavailable; restart your workers manually." >&2
  exit 1
fi
echo "Installed binary: $("$installed" --version 2>/dev/null) ($installed)"

echo "--- System worker: $worker_service"
if loaded show "$worker_service"; then
  if sudo install -m755 "$installed" "$worker_bin"; then
    echo "Updated $worker_bin"
    sudo systemctl restart "$worker_service" || fail "restart $worker_service"
  else
    fail "install $worker_bin (worker left running on its old binary)"
  fi
  sudo systemctl status "$worker_service" --no-pager || fail "$worker_service is not active"
else
  echo "Not loaded; skipped."
fi

echo "--- CodeTether Chat backend: $chat"
if loaded --user show "$chat"; then
  exec_start=$(systemctl --user show "$chat" -p ExecStart --value 2>/dev/null)
  echo "ExecStart=$exec_start"
  if [[ "$exec_start" =~ path=([^[:space:]]+) ]] && [ "${BASH_REMATCH[1]}" != "$installed" ]; then
    echo "WARNING: $chat is pinned to ${BASH_REMATCH[1]}, not $installed;" >&2
    echo "         restarting will not pick up the new build." >&2
  fi
  systemctl --user restart "$chat" || fail "restart $chat"
  systemctl --user status "$chat" --no-pager || fail "$chat is not active"
else
  echo "Not loaded; skipped."
fi

echo "---"
if [ "$failures" -gt 0 ]; then
  echo "$failures step(s) failed; see FAILED lines above." >&2
  exit 1
fi
echo "Services restarted on the new binary."
