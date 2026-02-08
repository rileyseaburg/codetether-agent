#!/bin/sh
# CodeTether Agent Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh
#
# Installs the latest release of codetether to /usr/local/bin (or ~/.local/bin if no sudo).
# By default, also downloads the FunctionGemma model for local tool-call routing.
# No Rust toolchain required.
#
# Options:
#   --no-functiongemma   Skip FunctionGemma model download
#   --functiongemma-only Only download the FunctionGemma model (skip binary install)

set -e

REPO="rileyseaburg/codetether-agent"
BINARY_NAME="codetether"
INSTALL_DIR="/usr/local/bin"
USE_SUDO="true"
INSTALL_FUNCTIONGEMMA="true"
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

    # Download tokenizer (gated model — requires HuggingFace API token)
    local tokenizer_path="${FUNCTIONGEMMA_MODEL_DIR}/${FUNCTIONGEMMA_TOKENIZER_FILE}"
    if [ -f "$tokenizer_path" ]; then
        ok "tokenizer already exists: ${tokenizer_path}"
    else
        # Check for HF token in env, then prompt
        local hf_token="${HF_TOKEN:-${HUGGING_FACE_HUB_TOKEN:-}}"
        if [ -z "$hf_token" ]; then
            printf "\n${YELLOW}The FunctionGemma tokenizer requires a HuggingFace API token.${NC}\n"
            printf "  1. Go to ${CYAN}https://huggingface.co/settings/tokens${NC}\n"
            printf "  2. Create a token with ${BOLD}read${NC} access\n"
            printf "  3. Accept the model license at ${CYAN}https://huggingface.co/google/functiongemma-270m-it${NC}\n\n"
            printf "HuggingFace API token (or press Enter to skip): "
            read -r hf_token
        fi

        if [ -z "$hf_token" ]; then
            warn "skipping tokenizer download (no API token provided)"
            warn "set HF_TOKEN and re-run: HF_TOKEN=hf_... $0 --functiongemma-only"
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
            error "failed to download tokenizer (check token and model access)"
            warn "set HF_TOKEN and re-run: HF_TOKEN=hf_... $0 --functiongemma-only"
            return 1
        fi
    fi

    ok "FunctionGemma installed to ${FUNCTIONGEMMA_MODEL_DIR}"

    # Print activation instructions
    printf "\n${BOLD}To enable the FunctionGemma tool-call router:${NC}\n"
    printf "  ${CYAN}export CODETETHER_TOOL_ROUTER_ENABLED=true${NC}\n"
    printf "  ${CYAN}export CODETETHER_TOOL_ROUTER_MODEL_PATH=\"${model_path}\"${NC}\n"
    printf "  ${CYAN}export CODETETHER_TOOL_ROUTER_TOKENIZER_PATH=\"${tokenizer_path}\"${NC}\n\n"
    info "add these to your shell profile for persistent activation"
}

main() {
    # Parse arguments
    for arg in "$@"; do
        case "$arg" in
            --no-functiongemma)
                INSTALL_FUNCTIONGEMMA="false"
                ;;
            --functiongemma-only)
                FUNCTIONGEMMA_ONLY="true"
                ;;
            --help|-h)
                printf "Usage: $0 [OPTIONS]\n\n"
                printf "Options:\n"
                printf "  --no-functiongemma    Skip FunctionGemma model download\n"
                printf "  --functiongemma-only  Only download the FunctionGemma model\n"
                printf "  --help, -h            Show this help message\n"
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

    printf "\n${BOLD}Get started:${NC}\n"
    printf "  ${CYAN}codetether tui${NC}       — interactive TUI\n"
    printf "  ${CYAN}codetether run \"...\"${NC} — single prompt\n"
    printf "  ${CYAN}codetether --help${NC}    — all commands\n\n"

    # Install FunctionGemma model (default: on)
    if [ "$INSTALL_FUNCTIONGEMMA" = "true" ]; then
        install_functiongemma
    else
        info "skipping FunctionGemma model download (--no-functiongemma)"
        info "you can install it later: $0 --functiongemma-only"
    fi
}

main "$@"
