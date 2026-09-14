#!/usr/bin/env bash
# Mocked-local profile, selector and parent/child environment contracts.
set -euo pipefail
root=$(cd "$(dirname "$0")/../../.." && pwd)
work=${1:?Retained evidence directory required}
mkdir -p "$work/home"
export HOME="$work/home" SHELL=/bin/zsh XDG_CONFIG_HOME="$work/home/.config"
CT_HELPERS="$root/script/unix-install"
for name in input model config profile; do source "$CT_HELPERS/$name.sh"; done
! ct_valid_model 'codetether models'
! ct_valid_model '$(touch unwanted)/model'
ct_valid_model 'openai-codex/gpt-6-astra-fast:high'
fake_models() { printf '[{"provider":"p","models":[{"canonical_id":"m","selectable_id":"p/m"}]}]\n'; }
[[ $(ct_discover_model fake_models) == p/m ]]
printf '# unrelated\nexport KEEP_ME=yes\n# CodeTether core configuration\nexport VAULT_ADDR="old"\nexport VAULT_TOKEN="fixture-old"\nexport CODETETHER_DEFAULT_MODEL="codetether models"\n' > "$HOME/.zshrc"
export VAULT_ADDR='https://vault.example.invalid'
config=$(ct_save_config 'p/m')
ct_profile_hook "$config"
grep -q 'export KEEP_ME=yes' "$HOME/.zshrc"
! grep -q 'fixture-old\|codetether models' "$HOME/.zshrc"
! grep -q 'fixture-old' "$config"
[[ $(stat -c %a "$config" 2>/dev/null || stat -f %Lp "$config") == 600 ]]
[[ $(find "$HOME" -name '.zshrc.codetether-backup-*' | wc -l) -ge 1 ]]
ct_profile_hook "$config"
[[ $(grep -c '^# CodeTether Vault activation$' "$HOME/.zshrc") == 1 ]]
quoted=$(ct_quote "apostrophe' and \$(touch nope)")
eval "value=$quoted"
[[ $value == "apostrophe' and \$(touch nope)" && ! -e nope ]]
vault() {
    case "$1 $2" in
        'token lookup') [[ ${VAULT_TOKEN:-fixture-good} == fixture-good ]] ;;
        'print token') printf 'fixture-good' ;;
        *) return 1 ;;
    esac
}
export VAULT_TOKEN=fixture-old CODETETHER_DEFAULT_MODEL='codetether models'
( source "$config"; [[ $VAULT_TOKEN == fixture-good && $CODETETHER_DEFAULT_MODEL == p/m ]] )
[[ $VAULT_TOKEN == fixture-old && $CODETETHER_DEFAULT_MODEL == 'codetether models' ]]
source "$config"
[[ $VAULT_TOKEN == fixture-good && $CODETETHER_DEFAULT_MODEL == p/m ]]
for file in "$CT_HELPERS"/*.sh; do bash -n "$file"; sh -n "$file"; done
echo 'mocked local: model schema, command rejection, quoting, private files, migration, and explicit parent activation'
