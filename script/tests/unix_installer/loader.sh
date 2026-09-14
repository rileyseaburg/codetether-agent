#!/usr/bin/env bash
# Mocked-local helper integrity and noninteractive safety checks.
set -euo pipefail
root=$(cd "$(dirname "$0")/../../.." && pwd)
work=${1:?Retained evidence directory required}
mkdir -p "$work/home" "$work/tmp"
export HOME="$work/home" TMPDIR="$work/tmp"
sed '$d' "$root/install.sh" > "$work/functions.sh"
source "$work/functions.sh"
mode=good
stty() { return 1; }
download() {
    [[ $1 == https://forgejo.quantum-forge.io/riley/codetether-agent/raw/commit/53bf14390e5e206808687c27542f9c64bd263cac/script/unix-install/* ]]
    cp "$root/script/unix-install/${1##*/}" "$2"
    if [[ $mode == corrupt && $1 == */profile.sh ]]; then printf 'echo unsafe\n' >> "$2"; fi
}
configure_core_env /nonexistent > "$work/good.log" 2>&1
grep -q 'skipping login and profile writes' "$work/good.log"
[[ ! -e $HOME/.bashrc && ! -e $HOME/.zshrc ]]
mode=corrupt
if configure_core_env /nonexistent > "$work/corrupt.log" 2>&1; then
    echo 'corrupt helper was accepted' >&2; exit 1
fi
grep -q 'integrity check failed' "$work/corrupt.log"
! grep -q 'exported for current session\|VAULT_TOKEN \[' "$root/install.sh"
echo 'mocked local: immutable helper loading, fail-closed hashes and noninteractive no-write behavior'
