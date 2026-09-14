# Embedded in install.sh: all setup helpers are immutable and SHA256 checked.
configure_core_env() {
    local ct_target="$1" ct_hash ct_name ct_actual
    CT_HELPERS=$(mktemp -d "${TMPDIR:-/tmp}/codetether-vault.XXXXXX") || return 1
    while read -r ct_hash ct_name; do
        download "https://raw.githubusercontent.com/${REPO#*/}/53bf14390e5e206808687c27542f9c64bd263cac/script/unix-install/$ct_name" "$CT_HELPERS/$ct_name" || return 1
        if command -v sha256sum >/dev/null 2>&1; then ct_actual=$(sha256sum "$CT_HELPERS/$ct_name")
        else ct_actual=$(shasum -a 256 "$CT_HELPERS/$ct_name"); fi
        if [ "$ct_hash" != "${ct_actual%% *}" ]; then
            error 'Vault setup helper integrity check failed; no helper was executed.'
            return 1
        fi
    done <<'CODETETHER_HELPERS'
afa582af19718b1dab0c21661df35f2b81aaa99808ad64d67b9f3f239449df4b activation.sh
276d8bab0757bfe92ad6b6b6d3d915e577bc8bc80f0d60db9252c30283a822b7 config.sh
61657a0ea400ee03edd581ea5d94d839afe60a2cde6cf00556e54dee0a5b2b37 configure.sh
748200d49e93d757b808c4ed349c17ac5c196fd2f099c88bc7ecdae8bf0cb104 input.sh
6ba05df251aa812f70ce91f78a97884ce99edaf18b054b5ebb2adde77acceb83 model.sh
504908bb0ae53cb8271dec0f48569c6cc88de003a0a6e4cd94a300c49b89282e oidc.sh
97c3d953ba4e6f332db017f6a3a698e698a9be4fef922969186709ce40c9ab9f profile.sh
CODETETHER_HELPERS
    for ct_name in input model oidc config profile configure; do
        . "$CT_HELPERS/$ct_name.sh" || return 1
    done
    ct_configure_env "$ct_target"
}