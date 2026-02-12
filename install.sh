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

set -e

REPO="rileyseaburg/codetether-agent"
BINARY_NAME="codetether"
INSTALL_DIR="/usr/local/bin"
USE_SUDO="true"
INSTALL_FUNCTIONGEMMA="false"
FUNCTIONGEMMA_ONLY="false"

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
        MINGW*|MSYS*|CYGWIN*) os="pc-windows-msvc" ;;
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

get_latest_version() {
    # Use GitHub API to get latest release tag
    if command -v curl > /dev/null 2>&1; then
        curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
            | grep '"tag_name"' \
            | head -1 \
            | sed 's/.*"tag_name": *"//;s/".*//'
    elif command -v wget > /dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" \
            | grep '"tag_name"' \
            | head -1 \
            | sed 's/.*"tag_name": *"//;s/".*//'
    else
        error "need 'curl' or 'wget' to download"
        exit 1
    fi
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
        zsh)  echo "$HOME/.zshrc" ;;
        fish) echo "$HOME/.config/fish/config.fish" ;;
        *)    echo "$HOME/.bashrc" ;;
    esac
}

print_core_env_instructions() {
    printf "\n${BOLD}Set required environment variables:${NC}\n"
    printf "  export VAULT_ADDR=\"https://vault.example.com:8200\"\n"
    printf "  export VAULT_TOKEN=\"hvs.your-token\"\n"
    printf "  export CODETETHER_DEFAULT_MODEL=\"zai/glm-5\"\n"
    printf "\n  Add them to your shell profile (for example ${CYAN}~/.bashrc${NC}) to persist.\n\n"
}

discover_default_model() {
    local codetether_cmd="${1:-}"

    if [ -z "$codetether_cmd" ] || [ ! -x "$codetether_cmd" ]; then
        if command -v "$BINARY_NAME" > /dev/null 2>&1; then
            codetether_cmd="$(command -v "$BINARY_NAME")"
        else
            return 1
        fi
    fi

    local models_json=""
    models_json="$($codetether_cmd models --json 2>/dev/null || true)"
    if [ -z "$models_json" ]; then
        return 1
    fi

    local provider model_id
    provider="$(printf "%s\n" "$models_json" | sed -n 's/.*"provider":[[:space:]]*"\([^"]*\)".*/\1/p' | head -1)"
    model_id="$(printf "%s\n" "$models_json" | sed -n 's/.*"id":[[:space:]]*"\([^"]*\)".*/\1/p' | head -1)"

    if [ -n "$provider" ] && [ -n "$model_id" ]; then
        printf "%s/%s\n" "$provider" "$model_id"
        return 0
    fi

    return 1
}

configure_core_env() {
    local codetether_cmd="${1:-}"
    if [ -n "${VAULT_ADDR:-}" ] && [ -n "${VAULT_TOKEN:-}" ] && [ -n "${CODETETHER_DEFAULT_MODEL:-}" ]; then
        ok "core environment variables already set in current session"
        return 0
    fi

    printf "\n${BOLD}CodeTether Core Environment Setup${NC}\n\n"
    info "Vault-backed providers require VAULT_ADDR and VAULT_TOKEN."
    info "After Vault setup, the installer will try to auto-discover CODETETHER_DEFAULT_MODEL."

    if [ ! -r /dev/tty ]; then
        warn "non-interactive install detected; skipping prompts"
        print_core_env_instructions
        return 0
    fi

    printf "Configure these variables now and save to your shell profile? [Y/n]: "
    local configure_choice=""
    read -r configure_choice < /dev/tty
    case "$configure_choice" in
        n|N|no|NO)
            print_core_env_instructions
            return 0
            ;;
    esac

    local vault_addr="${VAULT_ADDR:-https://vault.example.com:8200}"
    local vault_token="${VAULT_TOKEN:-hvs.your-token}"
    local default_model="${CODETETHER_DEFAULT_MODEL:-zai/glm-5}"
    local input=""

    printf "VAULT_ADDR [%s]: " "$vault_addr"
    read -r input < /dev/tty
    if [ -n "$input" ]; then
        vault_addr="$input"
    fi

    local token_hint="hvs.your-token"
    if [ -n "${VAULT_TOKEN:-}" ]; then
        token_hint="current-session-token"
    fi
    printf "VAULT_TOKEN [%s]: " "$token_hint"
    read -r input < /dev/tty
    if [ -n "$input" ]; then
        vault_token="$input"
    fi

    export VAULT_ADDR="$vault_addr"
    export VAULT_TOKEN="$vault_token"

    info "discovering default model from Vault-configured providers..."
    local discovered_model=""
    discovered_model="$(discover_default_model "$codetether_cmd" || true)"
    if [ -n "$discovered_model" ]; then
        default_model="$discovered_model"
        ok "discovered default model: ${default_model}"
    else
        warn "could not auto-discover a model (no provider keys in Vault yet, or provider model listing failed)"
        printf "CODETETHER_DEFAULT_MODEL [%s]: " "$default_model"
        read -r input < /dev/tty
        if [ -n "$input" ]; then
            default_model="$input"
        fi
    fi

    local shell_profile
    shell_profile="$(detect_shell_profile)"

    local config_marker="# CodeTether core configuration"
    local config_block="${config_marker}
export VAULT_ADDR=\"${vault_addr}\"
export VAULT_TOKEN=\"${vault_token}\"
export CODETETHER_DEFAULT_MODEL=\"${default_model}\""

    if [ -f "$shell_profile" ] && grep -qF "$config_marker" "$shell_profile" 2>/dev/null; then
        local tmp_profile
        tmp_profile="$(mktemp)"
        sed "/${config_marker}/,/export CODETETHER_DEFAULT_MODEL=/d" "$shell_profile" > "$tmp_profile"
        printf "\n%s\n" "$config_block" >> "$tmp_profile"
        mv "$tmp_profile" "$shell_profile"
        ok "updated core env config in ${shell_profile}"
    else
        printf "\n%s\n" "$config_block" >> "$shell_profile"
        ok "added core env config to ${shell_profile}"
    fi

    export CODETETHER_DEFAULT_MODEL="$default_model"

    ok "core environment variables exported for current session"
    if [ "$vault_token" = "hvs.your-token" ]; then
        warn "VAULT_TOKEN is still a placeholder. Update it before running provider-backed commands."
    fi
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
            --help|-h)
                printf "Usage: $0 [OPTIONS]\n\n"
                printf "Options:\n"
                printf "  --functiongemma      Download the FunctionGemma model for tool-call routing\n"
                printf "  --functiongemma-only Only download the FunctionGemma model\n"
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

    # Get latest version
    info "fetching latest release..."
    local version
    version="$(get_latest_version)"

    if [ -z "$version" ]; then
        error "could not determine latest version"
        exit 1
    fi
    info "latest version: ${version}"

    # Build download URL
    local artifact_name="codetether-${version}-${platform}"
    local tarball="${artifact_name}.tar.gz"
    local url="https://github.com/${REPO}/releases/download/${version}/${tarball}"

    # Create temp directory
    local tmp_dir
    tmp_dir="$(mktemp -d)"
    trap 'rm -rf "$tmp_dir"' EXIT

    # Download
    info "downloading ${tarball}..."
    download "$url" "${tmp_dir}/${tarball}"

    if [ ! -f "${tmp_dir}/${tarball}" ]; then
        error "download failed — no pre-built binary for ${platform}"
        error "you can build from source: cargo install codetether-agent"
        exit 1
    fi

    # Extract
    info "extracting..."
    tar xzf "${tmp_dir}/${tarball}" -C "${tmp_dir}"

    if [ ! -f "${tmp_dir}/${artifact_name}" ]; then
        error "expected binary not found in archive"
        exit 1
    fi

    chmod +x "${tmp_dir}/${artifact_name}"

    # Determine install location
    # Try /usr/local/bin with sudo, fall back to ~/.local/bin
    if [ "$(id -u)" = "0" ]; then
        USE_SUDO="false"
    elif ! command -v sudo > /dev/null 2>&1; then
        USE_SUDO="false"
        INSTALL_DIR="${HOME}/.local/bin"
    fi

    # If we can't write to /usr/local/bin without sudo, use ~/.local/bin as fallback
    if [ "$USE_SUDO" = "true" ] && ! sudo -n true 2>/dev/null; then
        # sudo might prompt for password — try it
        info "installing to ${INSTALL_DIR} (may require sudo password)"
    fi

    # Ensure install directory exists
    if [ "$USE_SUDO" = "true" ]; then
        sudo mkdir -p "$INSTALL_DIR"
        sudo mv "${tmp_dir}/${artifact_name}" "${INSTALL_DIR}/${BINARY_NAME}"
        sudo chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    else
        mkdir -p "$INSTALL_DIR"
        mv "${tmp_dir}/${artifact_name}" "${INSTALL_DIR}/${BINARY_NAME}"
        chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    fi

    ok "installed ${BINARY_NAME} ${version} to ${INSTALL_DIR}/${BINARY_NAME}"

    # Verify
    if command -v "$BINARY_NAME" > /dev/null 2>&1; then
        local installed_version
        installed_version="$("$BINARY_NAME" --version 2>/dev/null || true)"
        ok "${installed_version}"
    else
        warn "${BINARY_NAME} is not in your PATH"
        if [ "$INSTALL_DIR" = "${HOME}/.local/bin" ]; then
            warn "add this to your shell profile:"
            printf "\n  export PATH=\"\$HOME/.local/bin:\$PATH\"\n\n"
        fi
    fi

    configure_core_env "${INSTALL_DIR}/${BINARY_NAME}"

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
