#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
manifest="$root/Cargo.toml"
lockfile="$root/Cargo.lock"
next=${1:?usage: set-dev-version.sh VERSION}

rewrite() {
  local source=$1 target=$2 lock=$3
  awk -v replacement="$next" -v lock="$lock" '
    !lock && /^\[package\][[:space:]]*$/ { package = 1 }
    !lock && package && /^\[/ && !/^\[package\][[:space:]]*$/ { package = 0 }
    lock && /^\[\[package\]\][[:space:]]*$/ { package = 0 }
    lock && /^name = "codetether-agent"$/ { package = 1 }
    package && /^version[[:space:]]*=/ {
      sub(/"[^"]*"/, "\"" replacement "\"")
      updated = 1
      package = 0
    }
    { print }
    END { exit !updated }
  ' "$source" >"$target"
}

manifest_tmp=$(mktemp "$root/.Cargo.toml.dev-version.XXXXXX")
lock_tmp=
cleanup() {
  rm -f "$manifest_tmp"
  [[ -z "$lock_tmp" ]] || rm -f "$lock_tmp"
}
trap cleanup EXIT

rewrite "$manifest" "$manifest_tmp" 0
if [[ -f "$lockfile" ]]; then
  lock_tmp=$(mktemp "$root/.Cargo.lock.dev-version.XXXXXX")
  if ! rewrite "$lockfile" "$lock_tmp" 1; then
    printf 'Root package was not found in %s\n' "$lockfile" >&2
    exit 1
  fi
fi

cat "$manifest_tmp" >"$manifest"
[[ -z "$lock_tmp" ]] || cat "$lock_tmp" >"$lockfile"
