#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
manifest="$root/Cargo.toml"
current=$(awk '
  /^\[package\][[:space:]]*$/ { package = 1; next }
  package && /^\[/ { package = 0 }
  package && /^version[[:space:]]*=/ {
    if (match($0, /"[^"]*"/)) {
      print substr($0, RSTART + 1, RLENGTH - 2)
      exit
    }
  }
' "$manifest")

if [[ ! $current =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)-dev\.([0-9]+)$ ]]; then
  printf 'Expected a numeric -dev.N package version, got: %s\n' "$current" >&2
  exit 1
fi

dev_number=$((10#${BASH_REMATCH[4]} + 1))
next="${BASH_REMATCH[1]}.${BASH_REMATCH[2]}.${BASH_REMATCH[3]}-dev.${dev_number}"

"$root/script/set-dev-version.sh" "$next"
printf '%s\n' "$next"
