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

source ./scripts/install-dev-cache.sh

# Reuse intermediate artifacts from previous builds
export CARGO_BUILD_BUILD_DIR="$PWD/target"

# Capture output so we can feed errors to the agent on failure
tmp=$(mktemp)
trap 'printf "Build output retained: %s\n" "$tmp"' EXIT

# Default to installing the application workspace member.
args=("$@")
if [ ${#args[@]} -eq 0 ]; then
  package_path=crates/codetether-agent
  [[ -f "$package_path/Cargo.toml" ]] || package_path=.
  args=(install --path "$package_path" --force)
fi

version=$(./scripts/bump-dev-version.sh) || exit 1
echo "Using development version $version"

while true; do
  cargo "${args[@]}" 2>&1 | tee "$tmp"
  exit_code=${PIPESTATUS[0]}

  if [ "$exit_code" -eq 0 ]; then
    if [ "${args[0]}" = install ]; then
      source ./scripts/install-dev-post-install.sh
    fi
    echo "---"
    echo "Build succeeded."
    exit 0
  fi

  if [ "${CODETETHER_INSTALL_AUTO_FIX:-1}" != 1 ] || ! command -v codetether >/dev/null 2>&1; then
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