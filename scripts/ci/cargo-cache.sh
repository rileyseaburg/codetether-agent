#!/usr/bin/env bash
# Restore/save the cargo home and target caches with the cluster ci-cache CLI.
#
# Mirrors spotlessbinco's cluster-directory-cache contract: a cache miss must
# never fail the job, and a save is best-effort (a huge target dir may exceed
# the 10 GiB server archive cap).
#
# Usage: cargo-cache.sh restore|save <cargo-home|target> <key>
set -euo pipefail

mode="${1:?usage: cargo-cache.sh restore|save <scope> <key>}"
scope="${2:?scope: cargo-home|target}"
key="${3:?cache key required}"

case "$scope" in
  cargo-home) path="${CARGO_HOME:-$HOME/.cargo}"; type="generic" ;;
  target) path="${CARGO_TARGET_DIR:-target}"; type="cargo" ;;
  *) echo "::error::unknown cache scope: $scope (use cargo-home or target)"; exit 1 ;;
esac

if [ "${CI_CACHE_AVAILABLE:-false}" != "true" ] && ! command -v ci-cache >/dev/null 2>&1; then
  echo "::warning::ci-cache unavailable; skipping $mode of $scope"
  exit 0
fi

if [ "$mode" = "restore" ]; then
  if out="$(ci-cache restore --cache-type "$type" --key "$key" --paths "$path" 2>&1)"; then
    if echo "$out" | grep -Eq '^RESTORED .*files=[1-9][0-9]*'; then
      echo "::notice::restored $scope cache: $key"
      echo "$out" | grep '^RESTORED' || true
    else
      echo "::notice::cache miss for $scope: $key"
    fi
  else
    echo "::warning::restore failed for $scope ($key); continuing cold: $(echo "$out" | tail -1)"
  fi
  exit 0
fi

if [ ! -d "$path" ] || [ -z "$(ls -A "$path" 2>/dev/null)" ]; then
  echo "::warning::$path is empty; skipping save"
  exit 0
fi
if out="$(ci-cache save --cache-type "$type" --key "$key" --paths "$path" --ttl 1209600 2>&1)"; then
  echo "$out" | grep -E '^(SAVED|uploaded)' || echo "::notice::saved $scope cache: $key"
else
  echo "::warning::save failed for $scope ($key); next run stays cold: $(echo "$out" | tail -1)"
fi
