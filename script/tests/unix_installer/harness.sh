#!/bin/sh
# Exercise the real installer main while isolating network, credentials and destinations.
set -e
. "$CASE_DIR/functions.sh"
INSTALL_DIR="$CASE_DIR/installed"
id() { printf '0\n'; }
detect_platform() { printf '%s\n' "$INSTALL_TEST_PLATFORM"; }
get_latest_version() { printf 'v4.7.5\n'; }
download() {
    expected="https://github.com/rileyseaburg/codetether-agent/releases/download/v4.7.5/codetether-v4.7.5-$INSTALL_TEST_PLATFORM.tar.gz"
    test "$1" = "$expected"
    cp "$INSTALL_TEST_ARCHIVE" "$2"
}
mktemp() { command mktemp -d "$CASE_DIR/stage-XXXXXX"; }
configure_core_env() { test "$1" = "$INSTALL_DIR/codetether"; }
main --force