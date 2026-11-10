#!/usr/bin/env bash
# Mocked-local metadata, platform and checksum checks; no installation or network.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
work=${1:?Provide a retained evidence directory}
mkdir -p "$work"
# shellcheck disable=SC1090 # Source installer functions without executing main.
source <(sed '$d' "$root/install.sh")
curl() {
  [[ "$2" == 'https://forgejo.quantum-forge.io/api/v1/repos/riley/codetether-agent/releases?limit=1&draft=false' ]] || return 1
  printf '[{"tag_name":"v4.7.6-dev.5","draft":false,"prerelease":true}]'
}
[[ $(get_latest_version) == v4.7.6-dev.5 ]]
# shellcheck disable=SC2317 # Called indirectly by the sourced platform detector.
sysctl() { return 1; } # Intel Macs do not expose the translation flag.
for target in Linux:x86_64:unknown-linux-gnu Darwin:x86_64:apple-darwin Darwin:arm64:apple-darwin; do
  IFS=: read -r fixture_os fixture_arch suffix <<< "$target"
  uname() { if [[ "$1" == -s ]]; then echo "$fixture_os"; else echo "$fixture_arch"; fi; }
  expected_arch=$fixture_arch; [[ $fixture_arch != arm64 ]] || expected_arch=aarch64
  [[ $(detect_platform) == "$expected_arch-$suffix" ]]
done
fixture_os=Darwin fixture_arch=x86_64
sysctl() { printf '1\n'; }
[[ $(detect_platform) == aarch64-apple-darwin ]]
printf 'archive fixture' > "$work/package.tar.gz"
hash=$(sha256sum "$work/package.tar.gz"); hash=${hash%% *}
verify_fixture() {
  # shellcheck disable=SC2034 # Used by the installer fragment evaluated below.
  local tmp_dir=$work tarball=package.tar.gz version=v4.7.6-dev.5
  # shellcheck disable=SC2317 # Called by the installer fragment evaluated below.
  download() { cp "$work/manifest-fixture" "$2"; }
  eval "$(sed -n '/download .*SHA256SUMS-/,/# Extract/p' "$root/install.sh")"
}
printf '%s  package.tar.gz\n' "$hash" > "$work/manifest-fixture"
( verify_fixture )
printf '%064d  package.tar.gz\n' 0 > "$work/manifest-fixture"
if ( verify_fixture ); then echo 'corrupt archive accepted' >&2; exit 1; fi
printf '%s  different.tar.gz\n' "$hash" > "$work/manifest-fixture"
if ( verify_fixture ); then echo 'missing checksum accepted' >&2; exit 1; fi
if grep -Eq 'github.com|githubusercontent.com' "$root/install.sh"; then
  echo 'Unix installer still depends on GitHub' >&2; exit 1
fi
bash -n "$root/install.sh"
sh -n "$root/install.sh"
echo 'mocked local: Forgejo prereleases, Linux/Intel/ARM64/Rosetta targets, good/corrupt/missing checksums'
echo 'static/local: shell syntax and Forgejo-only Unix installer endpoints'