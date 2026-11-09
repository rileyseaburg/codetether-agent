#!/bin/sh
# Sourceable Bash/zsh activation; no token value is embedded in this file.
ct_activate_vault() {
    if command -v vault >/dev/null 2>&1; then
        if ! vault token lookup >/dev/null 2>&1; then
            if (unset VAULT_TOKEN; vault token lookup >/dev/null 2>&1); then
                ct_cached_token=$(unset VAULT_TOKEN; vault print token 2>/dev/null) || return 1
                [ -n "$ct_cached_token" ] || return 1
                export VAULT_TOKEN="$ct_cached_token"
                unset ct_cached_token
            else
                printf '%s\n' 'CodeTether: Vault login is needed. Use the documented OIDC login; this activation does not open a browser.' >&2
                return 1
            fi
        fi
        # The CLI may already use its token helper even with VAULT_TOKEN absent.
        if [ -z "${VAULT_TOKEN:-}" ]; then
            ct_cached_token=$(vault print token 2>/dev/null) || return 1
            [ -n "$ct_cached_token" ] || return 1
            export VAULT_TOKEN="$ct_cached_token"
            unset ct_cached_token
        fi
    elif [ -z "${VAULT_TOKEN:-}" ]; then
        printf '%s\n' 'CodeTether: install Vault CLI and authenticate through OIDC.' >&2
        return 1
    fi
}
if [ -n "${CODETETHER_DEFAULT_MODEL:-}" ] && ! printf '%s\n' "$CODETETHER_DEFAULT_MODEL" | grep -Eq '^[[:alnum:]_-]+/[[:alnum:]_.:/+-]+$'; then
    unset CODETETHER_DEFAULT_MODEL
    printf '%s\n' 'CodeTether: discarded an invalid model selector; choose provider/model from codetether models.' >&2
fi
ct_activate_vault || :
unset -f ct_activate_vault
