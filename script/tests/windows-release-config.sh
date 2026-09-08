#!/usr/bin/env bash
# Static regression: release setup must not write the runner's read-only Docker home.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
workflow="$root/.forgejo/workflows/release-windows.yml"
grep -Fq 'DOCKER_CONFIG: /tmp/codetether-docker-config' "$workflow"
grep -Fq 'BUILDX_CONFIG: /tmp/codetether-buildx-config' "$workflow"
grep -Fq 'run: mkdir -p "$DOCKER_CONFIG" "$BUILDX_CONFIG"' "$workflow"
grep -Fq 'run: bash script/forgejo/package-windows-release.sh' "$workflow"
bash -n "$root/script/forgejo/package-windows-release.sh"
printf 'Windows release config regression checks passed.\n'