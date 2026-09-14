#!/bin/sh
# CodeTether Agent Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh
#
# Installs the latest release of codetether to /usr/local/bin (or ~/.local/bin if no sudo).
# No Rust toolchain required.
#
# Options:
#   --functiongemma      Download the FunctionGemma model for local tool-call routing (optional)
#   --functiongemma-only Only download the FunctionGemma model (skip binary install)
#   --force              Force reinstall even if latest version is already installed

set -e

REPO="github.com/rileyseaburg/codetether-agent"
BINARY_NAME="codetether"
INSTALL_DIR="/usr/local/bin"
USE_SUDO="true"
INSTALL_FUNCTIONGEMMA="false"
FUNCTIONGEMMA_ONLY="false"
FORCE_INSTALL="false"

# FunctionGemma model configuration
FUNCTIONGEMMA_MODEL_DIR="${XDG_DATA_HOME:-${HOME}/.local/share}/codetether/models/functiongemma"
FUNCTIONGEMMA_MODEL_URL="https://huggingface.co/unsloth/functiongemma-270m-it-GGUF/resolve/main/functiongemma-270m-it-Q8_0.gguf"
FUNCTIONGEMMA_MODEL_FILE="functiongemma-270m-it-Q8_0.gguf"
FUNCTIONGEMMA_TOKENIZER_URL="https://huggingface.co/google/functiongemma-270m-it/resolve/main/tokenizer.json"
FUNCTIONGEMMA_TOKENIZER_FILE="tokenizer.json"

# Colors (if terminal supports them)
if [ -t 1 ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    NC='\033[0m'
else
    RED='' GREEN='' YELLOW='' CYAN='' BOLD='' NC=''
fi

info()  { printf "${CYAN}info${NC}: %s\n" "$1"; }
warn()  { printf "${YELLOW}warn${NC}: %s\n" "$1"; }
error() { printf "${RED}error${NC}: %s\n" "$1" >&2; }
ok()    { printf "${GREEN}  ok${NC}: %s\n" "$1"; }

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1; then
        error "need '$1' (command not found)"
        exit 1
    fi
}

detect_platform() {
    local os arch

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)  os="unknown-linux-gnu" ;;
        Darwin) os="apple-darwin" ;;
        MINGW*|MSYS*|CYGWIN*) error "use the PowerShell install.ps1 installer on Windows"; return 1 ;;
        *)
            error "unsupported OS: $os"
            exit 1
            ;;
    esac

    case "$arch" in
        x86_64|amd64)  arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)
            error "unsupported architecture: $arch"
            exit 1
            ;;
    esac

    echo "${arch}-${os}"
}

semver_cmp() {
    # Returns: 1 if $1 > $2, 0 if equal, -1 if $1 < $2
    awk -v a="$1" -v b="$2" '
    function isnum(x) { return x ~ /^[0-9]+$/ }
    function cmp_ident(x, y, xn, yn) {
        if (x == y) return 0
        xn = isnum(x); yn = isnum(y)
        if (xn && yn) return ((x + 0) > (y + 0)) ? 1 : -1
        if (xn && !yn) return -1
        if (!xn && yn) return 1
        return (x > y) ? 1 : -1
    }
    function parse(v,    idx, base, pre, n, arr, i) {
        sub(/^v/, "", v)
        idx = index(v, "-")
        if (idx > 0) {
            base = substr(v, 1, idx - 1)
            pre = substr(v, idx + 1)
        } else {
            base = v
            pre = ""
        }

        split(base, arr, ".")
        maj = (arr[1] == "") ? 0 : arr[1] + 0
        min = (arr[2] == "") ? 0 : arr[2] + 0
        pat = (arr[3] == "") ? 0 : arr[3] + 0

        delete pre_parts
        pre_len = 0
        if (pre != "") {
            pre_len = split(pre, pre_parts, /[.-]/)
        }
    }
    BEGIN {
        parse(a)
        a_maj = maj; a_min = min; a_pat = pat; a_pre = pre; a_pre_len = pre_len
        delete a_parts
        for (i = 1; i <= a_pre_len; i++) a_parts[i] = pre_parts[i]

        parse(b)
        b_maj = maj; b_min = min; b_pat = pat; b_pre = pre; b_pre_len = pre_len
        delete b_parts
        for (i = 1; i <= b_pre_len; i++) b_parts[i] = pre_parts[i]

        if (a_maj != b_maj) { print (a_maj > b_maj) ? 1 : -1; exit }
        if (a_min != b_min) { print (a_min > b_min) ? 1 : -1; exit }
        if (a_pat != b_pat) { print (a_pat > b_pat) ? 1 : -1; exit }

        if (a_pre == "" && b_pre == "") { print 0; exit }
        if (a_pre == "" && b_pre != "") { print 1; exit }
        if (a_pre != "" && b_pre == "") { print -1; exit }

        max_len = (a_pre_len > b_pre_len) ? a_pre_len : b_pre_len
        for (i = 1; i <= max_len; i++) {
            if (i > a_pre_len) { print -1; exit }
            if (i > b_pre_len) { print 1; exit }
            c = cmp_ident(a_parts[i], b_parts[i])
            if (c != 0) { print c; exit }
        }

        print 0
    }'
}

version_is_newer() {
    [ "$(semver_cmp "$1" "$2")" -gt 0 ]
}

get_latest_version() {
    # Include prereleases: GitHub /latest excludes our development releases.
    local api="https://api.github.com/repos/rileyseaburg/codetether-agent/releases?per_page=1"
    if command -v curl > /dev/null 2>&1; then
        curl -fsSL "$api"
    elif command -v wget > /dev/null 2>&1; then
        wget -qO- "$api"
    else
        error "need 'curl' or 'wget' to download"
        return 1
    fi | tr ',' '\n' | sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' | head -1
}

download() {
    local url="$1" dest="$2"
    if command -v curl > /dev/null 2>&1; then
        curl -fsSL "$url" -o "$dest"
    elif command -v wget > /dev/null 2>&1; then
        wget -qO "$dest" "$url"
    fi
}

detect_shell_profile() {
    local shell_name="$(basename "${SHELL:-/bin/bash}")"
    case "$shell_name" in
        zsh) echo "$HOME/.zshrc" ;;
        fish) echo "$HOME/.config/fish/config.fish" ;;
        *) echo "$HOME/.bashrc" ;;
    esac
}

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
install_functiongemma() {
    printf "\n${BOLD}FunctionGemma Model Setup${NC}\n\n"
    info "model directory: ${FUNCTIONGEMMA_MODEL_DIR}"

    mkdir -p "$FUNCTIONGEMMA_MODEL_DIR"

    # Download GGUF model
    local model_path="${FUNCTIONGEMMA_MODEL_DIR}/${FUNCTIONGEMMA_MODEL_FILE}"
    if [ -f "$model_path" ]; then
        ok "model already exists: ${model_path}"
    else
        info "downloading FunctionGemma GGUF model (~292 MB)..."
        download "$FUNCTIONGEMMA_MODEL_URL" "$model_path"
        if [ -f "$model_path" ]; then
            ok "model downloaded: ${model_path}"
        else
            error "failed to download FunctionGemma model"
            warn "you can retry later: $0 --functiongemma-only"
            return 1
        fi
    fi

    # Download tokenizer (gated model — requires HuggingFace authentication)
    local tokenizer_path="${FUNCTIONGEMMA_MODEL_DIR}/${FUNCTIONGEMMA_TOKENIZER_FILE}"
    if [ -f "$tokenizer_path" ]; then
        ok "tokenizer already exists: ${tokenizer_path}"
    else
        local hf_token=""

        # 1. Check environment variables
        hf_token="${HF_TOKEN:-${HUGGING_FACE_HUB_TOKEN:-}}"

        # 2. Check huggingface-cli cached token (~/.cache/huggingface/token)
        if [ -z "$hf_token" ]; then
            local hf_cache_token="${HF_HOME:-${XDG_CACHE_HOME:-${HOME}/.cache}/huggingface}/token"
            if [ -f "$hf_cache_token" ]; then
                hf_token="$(cat "$hf_cache_token" 2>/dev/null | tr -d '[:space:]')"
                if [ -n "$hf_token" ]; then
                    ok "found cached HuggingFace token (from huggingface-cli login)"
                fi
            fi
        fi

        # 3. Interactive: offer browser-based login or manual paste
        if [ -z "$hf_token" ]; then
            printf "\n${BOLD}HuggingFace Authentication Required${NC}\n"
            printf "  The FunctionGemma tokenizer is a gated model that requires\n"
            printf "  a HuggingFace account with model access granted.\n\n"
            printf "  ${BOLD}Before continuing:${NC}\n"
            printf "  Accept the model license at:\n"
            printf "  ${CYAN}https://huggingface.co/google/functiongemma-270m-it${NC}\n\n"

            printf "  Choose authentication method:\n"
            printf "  ${BOLD}[1]${NC} Open browser to create a token (recommended)\n"
            printf "  ${BOLD}[2]${NC} Paste an existing token\n"
            printf "  ${BOLD}[3]${NC} Skip tokenizer download\n\n"
            printf "  Choice [1/2/3]: "
            read -r auth_choice < /dev/tty

            case "$auth_choice" in
                1|"")
                    local token_url="https://huggingface.co/settings/tokens/new?tokenType=read&description=codetether-install"
                    info "opening browser..."

                    # Try to open browser
                    if command -v xdg-open > /dev/null 2>&1; then
                        xdg-open "$token_url" 2>/dev/null
                    elif command -v open > /dev/null 2>&1; then
                        open "$token_url" 2>/dev/null
                    elif command -v wslview > /dev/null 2>&1; then
                        wslview "$token_url" 2>/dev/null
                    else
                        warn "could not open browser automatically"
                        printf "  Open this URL manually:\n"
                        printf "  ${CYAN}${token_url}${NC}\n"
                    fi

                    printf "\n  Create a ${BOLD}read${NC} token, then paste it here.\n"
                    printf "  HuggingFace token: "
                    read -r hf_token < /dev/tty
                    ;;
                2)
                    printf "  HuggingFace token: "
                    read -r hf_token < /dev/tty
                    ;;
                3)
                    warn "skipping tokenizer download"
                    warn "re-run later: HF_TOKEN=hf_... $0 --functiongemma-only"
                    return 0
                    ;;
            esac
        fi

        # Trim whitespace
        hf_token="$(echo "$hf_token" | tr -d '[:space:]')"

        if [ -z "$hf_token" ]; then
            warn "no token provided — skipping tokenizer download"
            warn "re-run later: HF_TOKEN=hf_... $0 --functiongemma-only"
            return 0
        fi

        info "downloading tokenizer (authenticated)..."
        if command -v curl > /dev/null 2>&1; then
            curl -fsSL -H "Authorization: Bearer ${hf_token}" "$FUNCTIONGEMMA_TOKENIZER_URL" -o "$tokenizer_path"
        elif command -v wget > /dev/null 2>&1; then
            wget -qO "$tokenizer_path" --header="Authorization: Bearer ${hf_token}" "$FUNCTIONGEMMA_TOKENIZER_URL"
        fi

        if [ -f "$tokenizer_path" ] && [ -s "$tokenizer_path" ]; then
            ok "tokenizer downloaded: ${tokenizer_path}"
        else
            rm -f "$tokenizer_path"
            error "failed to download tokenizer (check token and model license access)"
            warn "1. Accept license: https://huggingface.co/google/functiongemma-270m-it"
            warn "2. Re-run: HF_TOKEN=hf_... $0 --functiongemma-only"
            return 1
        fi
    fi

    ok "FunctionGemma installed to ${FUNCTIONGEMMA_MODEL_DIR}"

    # Auto-configure shell profile
    local shell_profile=""
    shell_profile="$(detect_shell_profile)"

    # Build the config block
    local config_marker="# CodeTether FunctionGemma configuration"
    local config_block="${config_marker}
export CODETETHER_TOOL_ROUTER_ENABLED=true
export CODETETHER_TOOL_ROUTER_MODEL_PATH=\"${model_path}\"
export CODETETHER_TOOL_ROUTER_TOKENIZER_PATH=\"${tokenizer_path}\""

    # Check if already configured
    if [ -f "$shell_profile" ] && grep -qF "$config_marker" "$shell_profile" 2>/dev/null; then
        # Replace existing config block (remove old lines, append new)
        local tmp_profile
        tmp_profile="$(mktemp)"
        sed "/${config_marker}/,/CODETETHER_TOOL_ROUTER_TOKENIZER_PATH/d" "$shell_profile" > "$tmp_profile"
        printf "\n%s\n" "$config_block" >> "$tmp_profile"
        mv "$tmp_profile" "$shell_profile"
        ok "updated FunctionGemma config in ${shell_profile}"
    else
        printf "\n%s\n" "$config_block" >> "$shell_profile"
        ok "added FunctionGemma config to ${shell_profile}"
    fi

    # Export for current session
    export CODETETHER_TOOL_ROUTER_ENABLED=true
    export CODETETHER_TOOL_ROUTER_MODEL_PATH="${model_path}"
    export CODETETHER_TOOL_ROUTER_TOKENIZER_PATH="${tokenizer_path}"

    ok "FunctionGemma tool-call router is enabled"
    info "config written to ${shell_profile} — active in new shells"
}

main() {
    # Parse arguments
    for arg in "$@"; do
        case "$arg" in
            --functiongemma)
                INSTALL_FUNCTIONGEMMA="true"
                ;;
            --functiongemma-only)
                FUNCTIONGEMMA_ONLY="true"
                ;;
            --force)
                FORCE_INSTALL="true"
                ;;
            --help|-h)
                printf "Usage: $0 [OPTIONS]\n\n"
                printf "Options:\n"
                printf "  --functiongemma      Download the FunctionGemma model for tool-call routing\n"
                printf "  --functiongemma-only Only download the FunctionGemma model\n"
                printf "  --force              Force reinstall even if latest is already installed\n"
                printf "  --help, -h           Show this help message\n"
                exit 0
                ;;
        esac
    done

    # If --functiongemma-only, skip binary install entirely
    if [ "$FUNCTIONGEMMA_ONLY" = "true" ]; then
        install_functiongemma
        exit $?
    fi

    printf "\n${BOLD}CodeTether Agent Installer${NC}\n\n"

    # Check basic dependencies
    need_cmd uname
    need_cmd tar
    need_cmd grep
    need_cmd sed

    # Detect platform
    local platform
    platform="$(detect_platform)"
    info "detected platform: ${platform}"

    # Determine install location before checking for updates
    if [ "$(id -u)" = "0" ]; then
        USE_SUDO="false"
    elif ! command -v sudo > /dev/null 2>&1; then
        USE_SUDO="false"
        INSTALL_DIR="${HOME}/.local/bin"
    fi

    if [ "$USE_SUDO" = "true" ] && ! sudo -n true 2>/dev/null; then
        info "installing to ${INSTALL_DIR} (may require sudo password)"
    fi

    # Get latest version
    info "fetching latest release..."
    local version
    version="$(get_latest_version)"

    if [ -z "$version" ]; then
        error "could not determine latest version"
        exit 1
    fi
    info "latest version: ${version}"

    local target_path="${INSTALL_DIR}/${BINARY_NAME}"
    local installed_version=""
    local skip_binary_install="false"

    if [ -x "$target_path" ]; then
        installed_version="$("$target_path" --version 2>/dev/null | awk '{print $NF}' | head -1)"
    elif command -v "$BINARY_NAME" > /dev/null 2>&1; then
        installed_version="$("$BINARY_NAME" --version 2>/dev/null | awk '{print $NF}' | head -1)"
    fi

    if [ -n "$installed_version" ]; then
        info "installed version: ${installed_version}"
        if [ "$FORCE_INSTALL" != "true" ] && ! version_is_newer "$version" "$installed_version"; then
            ok "already up to date (${installed_version}); skipping binary install"
            skip_binary_install="true"
        fi
    fi

    if [ "$skip_binary_install" = "false" ]; then

    # Build download URL
    local artifact_name="codetether-${version}-${platform}"
    local tarball="${artifact_name}.tar.gz" expected actual
    local url="https://${REPO}/releases/download/${version}/${tarball}"

        # Create temp directory
        local tmp_dir
        tmp_dir="$(mktemp -d)"
        trap 'rm -rf "$tmp_dir"' EXIT

        # Download
        info "downloading ${tarball}..."
        download "$url" "${tmp_dir}/${tarball}"
        download "https://${REPO}/releases/download/${version}/SHA256SUMS-${version}.txt" "${tmp_dir}/SHA256SUMS"
        expected="$(awk -v name="$tarball" '$2 == name || $2 == "*"name {print $1; exit}' "${tmp_dir}/SHA256SUMS")"
        if command -v sha256sum >/dev/null 2>&1; then actual="$(sha256sum "${tmp_dir}/${tarball}")"
        else actual="$(shasum -a 256 "${tmp_dir}/${tarball}")"; fi
        [ ${#expected} -eq 64 ] && [ "$expected" = "${actual%% *}" ] || { error "release checksum mismatch or missing manifest entry"; exit 1; }

        # Extract
        info "extracting..."
        tar xzf "${tmp_dir}/${tarball}" -C "${tmp_dir}"

        local extracted_binary="${tmp_dir}/${artifact_name}"
        if [ ! -f "$extracted_binary" ]; then
            extracted_binary="${tmp_dir}/${BINARY_NAME}"
        fi
        if [ ! -f "$extracted_binary" ]; then
            error "expected binary not found in archive"
            exit 1
        fi

        chmod +x "$extracted_binary"

        # Ensure install directory exists
        if [ "$USE_SUDO" = "true" ]; then
            sudo mkdir -p "$INSTALL_DIR"
            sudo mv "$extracted_binary" "$target_path"
        else
            mkdir -p "$INSTALL_DIR"
            mv "$extracted_binary" "$target_path"
        fi

        ok "installed ${BINARY_NAME} ${version} to ${INSTALL_DIR}/${BINARY_NAME}"
    fi

    # Verify the file just installed, not a stale executable earlier on PATH.
    if [ "$skip_binary_install" = "true" ] && [ ! -x "$target_path" ]; then
        target_path="$(command -v "$BINARY_NAME")"
    fi
    installed_version="$("$target_path" --version)"
    ok "${installed_version}"
    if ! command -v "$BINARY_NAME" > /dev/null 2>&1; then
        warn "${BINARY_NAME} is not in your PATH"
        if [ "$INSTALL_DIR" = "${HOME}/.local/bin" ]; then
            warn "add this to your shell profile:"
            printf "\n  export PATH=\"\$HOME/.local/bin:\$PATH\"\n\n"
        fi
    elif [ "$(command -v "$BINARY_NAME")" != "$target_path" ]; then
        warn "PATH resolves $(command -v "$BINARY_NAME"); use $target_path or update PATH"
    fi

    configure_core_env "$target_path"

    printf "\n${BOLD}Get started:${NC}\n"
    printf "  ${CYAN}codetether tui${NC}       — interactive TUI\n"
    printf "  ${CYAN}codetether run \"...\"${NC} — single prompt\n"
    printf "  ${CYAN}codetether --help${NC}    — all commands\n\n"

    # Install FunctionGemma model (opt-in)
    if [ "$INSTALL_FUNCTIONGEMMA" = "true" ]; then
        install_functiongemma
    else
        info "skipping FunctionGemma model (use --functiongemma to install)"
    fi
}

main "$@"