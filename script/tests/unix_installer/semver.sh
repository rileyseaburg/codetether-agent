#!/usr/bin/env bash
# Native prerelease-ordering regression tests; never run the installer entry point.
set -euo pipefail
root=$(cd "$(dirname "$0")/../../.." && pwd)
evidence=${1:-"$root/artifacts/mac-setup/semver-regressions"}
mkdir -p "$evidence"
sed '$d' "$root/install.sh" > "$evidence/functions.sh"
source "$evidence/functions.sh"
assert_order() {
  local latest=$1 installed=$2 expected=$3 actual
  actual=$(semver_cmp "$latest" "$installed")
  if [[ $actual != "$expected" ]]; then
    printf 'Wrong ordering: %s vs %s: got %s, expected %s\n' \
      "$latest" "$installed" "$actual" "$expected" >&2
    return 1
  fi
}
assert_order v4.7.6-dev.14 4.7.6-dev.6 1
assert_order v4.7.6-dev.6 4.7.6-dev.14 -1
assert_order v4.7.6-dev.14 4.7.6-dev.14 0
assert_order v4.7.6 4.7.6-dev.14 1
assert_order v4.7.6-dev.14 4.7.6 -1
assert_order v4.7.6-dev.10 4.7.6-dev.9 1
assert_order v4.7.7-dev.1 4.7.6 1
assert_order v4.8.0-dev.1 4.7.6-dev.14 1
assert_order v5.0.0-dev.1 4.7.6-dev.14 1
assert_order v4.7.6-beta.1 4.7.6-alpha.9 1
assert_order v4.7.6-dev.14 4.7.6-dev 1
assert_order v4.7.6-dev.1 4.7.6-dev.beta -1
version_is_newer v4.7.6-dev.14 4.7.6-dev.6
if version_is_newer v4.7.6-dev.14 4.7.6-dev.14; then
  printf 'Equal versions must not trigger an upgrade\n' >&2; exit 1
fi
printf 'static/local: 12 semver comparisons and 2 upgrade decisions succeeded\n'