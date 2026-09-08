#!/usr/bin/env bash
# Override runner-inherited Docker paths inside the actual job process.
set -euo pipefail
: "${GITHUB_ENV:?GITHUB_ENV is required}"
state_root="${CODETETHER_BUILDX_STATE_ROOT:-/tmp}"
export DOCKER_CONFIG="$state_root/codetether-docker-config"
export BUILDX_CONFIG="$state_root/codetether-buildx-config"
mkdir -p "$DOCKER_CONFIG" "$BUILDX_CONFIG"
test -w "$DOCKER_CONFIG"
test -w "$BUILDX_CONFIG"
printf 'DOCKER_CONFIG=%s\nBUILDX_CONFIG=%s\n' \
  "$DOCKER_CONFIG" "$BUILDX_CONFIG" >> "$GITHUB_ENV"
printf 'Writable Docker config: %s\nWritable Buildx config: %s\n' \
  "$DOCKER_CONFIG" "$BUILDX_CONFIG"