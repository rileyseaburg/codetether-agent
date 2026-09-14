#!/usr/bin/env bash
# Mocked-local OIDC authentication and authorization stay separate.
set -euo pipefail
root=$(cd "$(dirname "$0")/../../.." && pwd)
work=${1:?Retained evidence directory required}
mkdir -p "$work"
source "$root/script/unix-install/oidc.sh"
ct_prompt() { CT_INPUT=; }
ct_oidc_login() { printf 'login\n' >> "$work/logins"; }
mode=missing
vault() {
    case "$1 $2" in
        'token lookup') [[ $mode == valid || ($mode == cached && -z ${VAULT_TOKEN:-}) ]] ;;
        'kv list') [[ $mode != denied ]] ;;
        *) return 1 ;;
    esac
}
export VAULT_TOKEN=fixture-stale
ct_vault_oidc
[[ $(wc -l < "$work/logins") -eq 1 ]]
mode=cached; ct_vault_oidc
mode=valid; ct_vault_oidc
[[ $(wc -l < "$work/logins") -eq 1 ]]
mode=denied
if ct_vault_oidc; then echo 'provider denial was mistaken for ready configuration' >&2; exit 1; fi
[[ $(wc -l < "$work/logins") -eq 2 ]]
grep -q -- '-no-print' "$root/script/unix-install/oidc.sh"
! grep -q 'role=admin' "$root/script/unix-install/oidc.sh"
echo 'mocked local: missing/cached/current token paths and provider-policy denial'
