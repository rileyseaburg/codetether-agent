#!/usr/bin/env bash
# Keep the artifact in the daemon; do not stream it through a long-lived BuildKit session.
set -euo pipefail
unset DOCKER_CONTEXT DOCKER_TLS_VERIFY DOCKER_CERT_PATH
export DOCKER_HOST=unix:///var/run/docker.sock
docker info
grep -Eq 'cargo build .*--locked' docker/release/windows.Dockerfile

# This path relies on the workflow's docker driver automatically loading tagged
# images into the local daemon (no client-side local/tar exporter or --load).
driver=$(docker buildx inspect | sed -n 's/^Driver:[[:space:]]*//p')
if [ "$driver" != docker ]; then
  printf 'Expected the docker Buildx driver, got: %s\n' "$driver" >&2
  exit 1
fi
image="codetether-windows-artifact:${GITHUB_RUN_ID:-local}-$$"
container=''
cleanup() {
  if [ -n "$container" ]; then
    docker rm "$container" >/dev/null 2>&1 || true
  fi
  docker image rm "$image" >/dev/null 2>&1 || true
}
trap cleanup EXIT
docker buildx build --progress plain --platform linux/amd64 \
  --file docker/release/windows.Dockerfile --target artifact \
  --tag "$image" .
mkdir -p build/windows
# The scratch image contains a Windows binary: create a container, never start it.
container=$(docker create "$image" /codetether.exe)
docker cp "$container:/codetether.exe" build/windows/codetether.exe
test -s build/windows/codetether.exe
printf 'Windows executable copied from daemon image: %s\n' "$image"