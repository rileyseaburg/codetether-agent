#!/usr/bin/env bash
# Prove the setup overrides inherited /root paths and exports writable directories.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
workflow="$root/.forgejo/workflows/release-windows.yml"
grep -Fq 'run: bash script/forgejo/prepare-windows-buildx.sh' "$workflow"
grep -Fq 'run: bash script/forgejo/package-windows-release.sh' "$workflow"
bash -n "$root/script/forgejo/package-windows-release.sh"
mkdir -p "$root/artifacts/release-verification"
fixture=$(mktemp -d "$root/artifacts/release-verification/buildx-env-XXXXXX")
export GITHUB_ENV="$fixture/github.env"
export CODETETHER_BUILDX_STATE_ROOT="$fixture/state"
export DOCKER_CONFIG=/root/.docker BUILDX_CONFIG=/root/.docker/buildx
bash "$root/script/forgejo/prepare-windows-buildx.sh" > "$fixture/setup.log"
grep -Fx "DOCKER_CONFIG=$fixture/state/codetether-docker-config" "$GITHUB_ENV"
grep -Fx "BUILDX_CONFIG=$fixture/state/codetether-buildx-config" "$GITHUB_ENV"
test -w "$fixture/state/codetether-docker-config"
test -w "$fixture/state/codetether-buildx-config"
printf 'mocked local: inherited-path regression passed; evidence: %s\n' "$fixture"