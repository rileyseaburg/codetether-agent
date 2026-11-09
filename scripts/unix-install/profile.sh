#!/bin/sh
# Migrate only recognized CodeTether blocks; preserve an owner-private backup.
ct_profile_hook() (
    set -e
    umask 077
    case "${SHELL:-}" in */zsh) ct_profile="$HOME/.zshrc" ;; *) ct_profile="$HOME/.bashrc" ;; esac
    if [ -L "$ct_profile" ]; then
        printf '%s\n' 'Profile is a symlink; leaving it unchanged. Use the printed activation command.' >&2
        exit 0
    fi
    ct_pending=$(mktemp "${ct_profile}.codetether.XXXXXX")
    if [ -f "$ct_profile" ]; then
        ct_backup=$(mktemp "${ct_profile}.codetether-backup-XXXXXX")
        cp "$ct_profile" "$ct_backup"; chmod 600 "$ct_backup"
        awk '
          /^# CodeTether core configuration$/ {
            b[0]=$0; n=0
            for(i=1;i<=3;i++){if((getline)<=0)break; b[i]=$0; n=i}
            if(n==3 && b[1]~/^export VAULT_ADDR=/ && b[2]~/^export VAULT_TOKEN=/ && b[3]~/^export CODETETHER_DEFAULT_MODEL=/)next
            for(i=0;i<=n;i++)print b[i]
            next
          }
          /^# CodeTether Vault activation$/ {
            previous=$0
            if((getline)>0){if($0~/^\. .*vault-env\.sh/)next; print previous; print; next}
            print previous; next
          }
          {print}
        ' "$ct_profile" > "$ct_pending"
    fi
    {
        printf '\n# CodeTether Vault activation\n. '
        ct_quote "$1"; printf '\n'
    } >> "$ct_pending"
    chmod 600 "$ct_pending"
    mv "$ct_pending" "$ct_profile"
    printf 'Saved a profile hook; it has NOT run in the parent shell: %s\n' "$ct_profile" >&2
)