#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

cache_args=()
cache_next=.docker-cache/windows-new

buildx_driver() {
  docker buildx inspect 2>/dev/null | awk '/^Driver:/ {print $2; exit}'
}

configure_cache() {
  mkdir -p .docker-cache

  if [ "$(buildx_driver)" = "docker" ]; then
    echo "Docker buildx driver does not support local cache export; building without external cache."
    return
  fi

  cache_args=(
    --cache-from type=local,src=.docker-cache/windows
    --cache-to type=local,dest="$cache_next",mode=max
  )
}

publish_cache() {
  # A docker-driver build did not export this directory; it may be stale.
  if [ "${#cache_args[@]}" -gt 0 ] && [ -d "$cache_next" ]; then
    rm -rf .docker-cache/windows
    mv "$cache_next" .docker-cache/windows
  fi
}

copy_artifacts() {
  mkdir -p dist/windows
  cp dist/codetether.exe dist/windows/codetether.exe
  cp dist/codetether.exe dist/codetether-windows.exe
}

mkdir -p dist
configure_cache

docker buildx build \
  --platform linux/amd64 \
  --file docker/release/windows.Dockerfile \
  --target artifact \
  "${cache_args[@]}" \
  --output type=local,dest=dist \
  .

copy_artifacts
python3 script/package-windows-bundle.py
publish_cache