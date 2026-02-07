#!/bin/sh
# CodeTether Agent Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh
#
# Installs the latest release of codetether to /usr/local/bin (or ~/.local/bin if no sudo).
# No Rust toolchain required.

set -e

REPO="rileyseaburg/codetether-agent"
BINARY_NAME="codetether"
INSTALL_DIR="/usr/local/bin"
USE_SUDO="true"

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

main() {
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
}

main "$@"
