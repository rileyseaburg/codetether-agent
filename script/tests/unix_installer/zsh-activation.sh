#!/bin/zsh
# Mocked-local activation in native zsh; only fixture credentials are used.
set -eu
export VAULT_TOKEN=fixture-stale CODETETHER_DEFAULT_MODEL='codetether models'
vault() {
    case "$1 $2" in
        'token lookup') [[ ${VAULT_TOKEN:-fixture-good} == fixture-good ]] ;;
        'print token') printf 'fixture-good' ;;
        *) return 1 ;;
    esac
}
. "$1"
[[ $VAULT_TOKEN == fixture-good ]]
[[ $CODETETHER_DEFAULT_MODEL == p/m ]]
printf '%s\n' 'mocked local: native zsh imports cached credentials and the valid model selector'
