#!/usr/bin/env bash
set -uo pipefail

cd "$(dirname "$0")/.." || exit 1

# ---------------------------------------------------------------------------
# Self-detach.
#
# A release build takes ~19 minutes. Run from an agent or SSH session it dies
# with the caller, leaving the version bumped but the binary stale -- observed
# exactly that way at dev.20. Re-exec under setsid so the build is reparented to
# init and survives the session that started it.
#
# Use --foreground to opt out (CI, or when you want to watch the build).
# ---------------------------------------------------------------------------
LOG_FILE="${CODETETHER_INSTALL_LOG:-/tmp/codetether-install-dev.log}"

if [ "${1:-}" = "--foreground" ]; then
  shift
elif [ -z "${CODETETHER_INSTALL_DETACHED:-}" ]; then
  if ! command -v setsid >/dev/null 2>&1; then
    echo "setsid unavailable; running in the foreground" >&2
  else
    export CODETETHER_INSTALL_DETACHED=1
    setsid nohup "$0" "$@" > "$LOG_FILE" 2>&1 < /dev/null &
    detached_pid=$!
    sleep 1
    cat <<EOF
Detached install started (survives this session).
  pid: $detached_pid
  log: $LOG_FILE

Follow:   tail -f $LOG_FILE
Check:    grep -E 'Build succeeded|INSTALL_COMPLETE|^error' $LOG_FILE
Running?  pgrep -f 'cargo install --path'
EOF
    exit 0
  fi
fi

# Keep cargo reachable when re-exec'd with a minimal environment.
case ":$PATH:" in
  *":$HOME/.cargo/bin:"*) ;;
  *) PATH="$HOME/.cargo/bin:$PATH"; export PATH ;;
esac

if [ -f "$HOME/.bashrc" ]; then
  set +u
  # shellcheck disable=SC1091
  source "$HOME/.bashrc"
  set -u
fi

if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER="${RUSTC_WRAPPER:-sccache}"
fi

# Reuse intermediate artifacts from previous builds
export CARGO_BUILD_BUILD_DIR="$PWD/target"

# Capture output so we can feed errors to the agent on failure
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT

# Default to install --path . if no args given
args=("$@")
if [ ${#args[@]} -eq 0 ]; then
  args=(install --path . --force)
fi

version=$(./script/bump-dev-version.sh) || exit 1
echo "Using development version $version"

while true; do
  cargo "${args[@]}" 2>&1 | tee "$tmp"
  exit_code=${PIPESTATUS[0]}

  if [ "$exit_code" -eq 0 ]; then
    # Keep every writable install location on the same build. /usr/local/bin
    # silently sat 13 versions behind because only ~/.cargo/bin was refreshed.
    installed="$HOME/.cargo/bin/codetether"
    target=/usr/local/bin/codetether
    if [ -e "$target" ]; then
      if [ -w "$target" ] && cp -f "$installed" "$target" 2>/dev/null; then
        echo "Synced $target -> $("$target" --version 2>/dev/null)"
      else
        echo "WARNING: $target is stale and needs root:" >&2
        echo "  sudo install -m755 $installed $target" >&2
      fi
    fi

    # Root-owned copies cannot be updated here; report them explicitly rather
    # than leaving a stale binary to be discovered later.
    worker=/opt/codetether-worker/bin/codetether
    if [ -e "$worker" ] && [ "$("$worker" --version 2>/dev/null)" != "codetether $version" ]; then
      echo "WARNING: $worker is stale ($("$worker" --version 2>/dev/null)); needs root:" >&2
      echo "  sudo install -m755 $installed $worker && sudo systemctl restart codetether-ubuntu-dev.service" >&2
    fi

    echo "---"
    echo "Build succeeded."
    exit 0
  fi

  if ! command -v codetether >/dev/null 2>&1; then
    exit "$exit_code"
  fi

  errors=$(grep -E '^error' "$tmp" | head -80)
  if [ -z "$errors" ]; then
    exit "$exit_code"
  fi

  echo "---"
  echo "Build failed. Running codetether to fix errors..."
  (cd .. && codetether run -c --model openai-codex/gpt-5.6-sol --access-mode full "fix these build errors:

${errors}")

  echo "---"
  echo "Retrying build..."
done