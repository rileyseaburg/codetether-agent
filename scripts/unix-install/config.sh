#!/bin/sh
# Store activation code and non-secret settings atomically with private permissions.
ct_save_config() (
    set -e
    umask 077
    ct_directory="${XDG_CONFIG_HOME:-$HOME/.config}/codetether"
    mkdir -p "$ct_directory"
    ct_file="$ct_directory/vault-env.sh"
    if [ -e "$ct_file" ]; then
        grep -q '^# CodeTether managed Vault activation$' "$ct_file" || {
            printf '%s\n' 'Existing vault-env.sh is not installer-managed; refusing to overwrite it.' >&2; exit 1;
        }
        ct_backup=$(mktemp "$ct_file.backup-XXXXXX")
        cp "$ct_file" "$ct_backup"; chmod 600 "$ct_backup"
    fi
    ct_pending=$(mktemp "$ct_directory/.vault-env.XXXXXX")
    {
        printf '# CodeTether managed Vault activation\nexport VAULT_ADDR='
        ct_quote "$VAULT_ADDR"; printf '\n'
        if [ -n "$1" ]; then printf 'export CODETETHER_DEFAULT_MODEL='; ct_quote "$1"; printf '\n'; fi
        cat "$CT_HELPERS/activation.sh"
    } > "$ct_pending"
    chmod 600 "$ct_pending"
    mv "$ct_pending" "$ct_file"
    printf '%s\n' "$ct_file"
)