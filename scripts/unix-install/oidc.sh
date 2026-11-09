ct_oidc_login() {
    (unset VAULT_TOKEN; vault login -method=oidc -path="${CODETETHER_VAULT_OIDC_MOUNT:-oidc}" \
        -no-print role="${CODETETHER_VAULT_OIDC_ROLE:-default}" \
        skip_browser="${CODETETHER_VAULT_SKIP_BROWSER:-false}") < /dev/tty
}

#!/bin/sh
# Authenticate via Vault's configured OIDC mount, never by collecting a raw token.
ct_vault_oidc() {
    if ! command -v vault >/dev/null 2>&1; then
        printf '%s\n' 'Vault CLI is required for browser login: https://developer.hashicorp.com/vault/install' >&2
        return 1
    fi
    if vault token lookup >/dev/null 2>&1; then return 0; fi
    # An expired process token must not mask a valid token in Vault's helper.
    if (unset VAULT_TOKEN; vault token lookup >/dev/null 2>&1); then return 0; fi
    printf '%s\n' 'Vault authentication is missing or rejected. Sign in through its configured OIDC provider.' >&2
    if [ -n "${SSH_CONNECTION:-}" ]; then
        printf '%s\n' 'SSH: the browser must reach this machine on localhost:8250. Use a tunnel or a browser on this machine.' >&2
    fi
    ct_prompt 'Start Vault OIDC browser login now? [Y/n]: ' || return 1
    case "$CT_INPUT" in n|N|no|NO) return 1 ;; esac
    ct_oidc_login || return 1
    if ! (unset VAULT_TOKEN; vault kv list -mount="${VAULT_MOUNT:-secret}" \
        "${VAULT_SECRETS_PATH:-codetether/providers}" >/dev/null 2>&1); then
        printf '%s\n' 'OIDC login succeeded, but provider access is not configured/authorized. Ask the Vault administrator for the approved role/group.' >&2
        return 1
    fi
}