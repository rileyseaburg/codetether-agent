#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
versioner="$root/script/release-version.sh"

assert_version() {
  local expected=$1
  shift
  local actual
  actual=$("$versioner" "$@")
  [[ $actual == "$expected" ]] || {
    printf 'expected %s, got %s\n' "$expected" "$actual" >&2
    exit 1
  }
}

assert_version 4.7.5 4.7.5-dev.0 patch
assert_version 4.7.6 4.7.5 patch
assert_version 4.8.0 4.7.5-dev.0 minor
assert_version 5.0.0 4.7.5-dev.0 major
assert_version 9.1.2 4.7.5-dev.0 patch v9.1.2

if "$versioner" invalid patch >/dev/null 2>&1; then
  printf 'invalid version unexpectedly succeeded\n' >&2
  exit 1
fi
