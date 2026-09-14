#!/bin/sh
# Interactive bootstrap, then explicit activation in the caller's shell.
ct_configure_env() {
    ct_binary=$1
    VAULT_ADDR=${VAULT_ADDR:-https://vault.spotlessbinco.com}
    printf '\nCodeTether Vault setup (OIDC; no raw-token prompt)\n' >&2
    if [ -r /dev/tty ] && (stty -g < /dev/tty) >/dev/null 2>&1; then
        ct_prompt 'Set up Vault OIDC and a shell activation hook now? [Y/n]: ' || return 1
        case "$CT_INPUT" in n|N|no|NO) printf '%s\n' 'Skipped credential setup; no profile changes were made.' >&2; return 0 ;; esac
        ct_prompt "Vault HTTPS address [$VAULT_ADDR]: " || return 1
        [ -z "$CT_INPUT" ] || VAULT_ADDR=$CT_INPUT
    fi
    ct_address_valid "$VAULT_ADDR" || { printf '%s\n' 'Invalid Vault address; settings were not saved.' >&2; return 1; }
    export VAULT_ADDR
    if [ -r /dev/tty ] && (stty -g < /dev/tty) >/dev/null 2>&1; then
        ct_vault_oidc || { printf '%s\n' 'Authentication/provider setup is pending; no profile changes were made.' >&2; return 0; }
    else
        printf '%s\n' 'No interactive terminal: skipping login and profile writes. See the Vault OIDC setup guide.' >&2
        return 0
    fi
    . "$CT_HELPERS/activation.sh"
    ct_model=${CODETETHER_DEFAULT_MODEL:-}
    if [ -z "$ct_model" ] && [ -n "${VAULT_TOKEN:-}" ]; then
        ct_model=$(ct_discover_model "$ct_binary") || ct_model=
    fi
    ct_valid_model "$ct_model" || ct_model=
    [ -n "$ct_model" ] || printf '%s\n' 'No default model selected. Run codetether models after activation; do not enter a shell command as a model ID.' >&2
    CT_CONFIG=$(ct_save_config "$ct_model") || return 1
    case "${SHELL:-}" in
        */fish) printf '%s\n' 'Automatic profile setup is limited to Bash/zsh. Open Bash for the activation command below.' >&2 ;;
        *) ct_profile_hook "$CT_CONFIG" || return 1 ;;
    esac
    printf '\nBinary installed; your parent shell environment has NOT been changed.\n' >&2
    printf 'Paste this into the shell that launched the installer:\n  . ' >&2
    ct_quote "$CT_CONFIG" >&2
    printf '\nThen run:\n  codetether models\n  codetether tui\n' >&2
}
