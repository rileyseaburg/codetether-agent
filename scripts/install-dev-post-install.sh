#!/usr/bin/env bash
# Sourced only after cargo install succeeds. Preserve writable-copy synchronization.
installed="$HOME/.cargo/bin/codetether"
target=/usr/local/bin/codetether
if [ -e "$target" ]; then
  if [ -w "$target" ] && cp -f "$installed" "$target" 2>/dev/null; then
    echo "Synced $target -> $("$target" --version 2>/dev/null)"
  else
    echo "WARNING: $target is stale and needs root:" >&2
    printf '  sudo install -m755 %q %q\n' "$installed" "$target" >&2
  fi
fi

# Root-owned copies need manual installation before restarting their service.
worker=/opt/codetether-worker/bin/codetether
if [ -e "$worker" ] && [ "$("$worker" --version 2>/dev/null)" != "codetether $version" ]; then
  echo "WARNING: $worker is stale ($("$worker" --version 2>/dev/null)); needs root:" >&2
  printf '  sudo install -m755 %q %q && sudo systemctl restart codetether-ubuntu-dev.service\n' "$installed" "$worker" >&2
fi
source ./scripts/install-dev-restart.sh
